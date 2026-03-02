const streamifyResponse =
  globalThis.awslambda &&
  typeof globalThis.awslambda.streamifyResponse === "function"
    ? globalThis.awslambda.streamifyResponse
    : null;
const { TextDecoder } = require("util");
const {
  BedrockRuntimeClient,
  InvokeModelWithResponseStreamCommand,
} = require("@aws-sdk/client-bedrock-runtime");
const { SSMClient, GetParameterCommand } = require("@aws-sdk/client-ssm");
const {
  DynamoDBClient,
  GetItemCommand,
  PutItemCommand,
  UpdateItemCommand,
} = require("@aws-sdk/client-dynamodb");
const { marshall, unmarshall } = require("@aws-sdk/util-dynamodb");

const AWS_REGION = "us-east-1";
const ssm = new SSMClient({ region: AWS_REGION });
const bedrock = new BedrockRuntimeClient({ region: AWS_REGION });
const usageRegion = "us-east-1";
const usageDb = new DynamoDBClient({ region: usageRegion });

const MODEL_ID = process.env.BEDROCK_MODEL_ID || "openai.gpt-oss-120b-1:0";
const REASONING_EFFORT = process.env.BEDROCK_REASONING_EFFORT || "medium";
const MAX_TOKENS = process.env.BEDROCK_MAX_TOKENS
  ? Number(process.env.BEDROCK_MAX_TOKENS)
  : 20480;
const USAGE_TABLE_NAME = "catieBedrockUsage";
const USAGE_DEFAULT_BUDGET = 1000000;
const USAGE_MAX_TEXT_BYTES = 300000;

function buildErrorResponse(responseStream, statusCode, message) {
  if (responseStream && typeof responseStream.setContentType === "function") {
    responseStream.setContentType("application/json");
  }
  responseStream.write(JSON.stringify({ error: message, status: statusCode }));
  responseStream.end();
}

function normalizeUser(user) {
  const sanitized = (user || "").replace(/[^A-Za-z0-9._-]/g, "_");
  if (!sanitized) {
    return "";
  }
  if (/^ssm/i.test(sanitized)) {
    return `user_${sanitized}`;
  }
  return sanitized;
}

function sseEvent(text, done, remainingTokens = null) {
  const payload = { text, done };
  if (Number.isFinite(remainingTokens)) {
    payload.remaining_tokens = remainingTokens;
  }
  return `data: ${JSON.stringify(payload)}\n\n`;
}

function computeProjectedRemainingTokens(usageRecord, usageTokens) {
  const currentRemaining =
    usageRecord && Number.isFinite(usageRecord.remaining_tokens)
      ? usageRecord.remaining_tokens
      : USAGE_DEFAULT_BUDGET;
  const usedTokens =
    usageTokens && Number.isFinite(usageTokens.totalTokens)
      ? usageTokens.totalTokens
      : 0;
  return Math.max(currentRemaining - usedTokens, 0);
}

function extractTextFromPayload(payload) {
  if (!payload || typeof payload !== "object") {
    return null;
  }

  if (Array.isArray(payload.choices) && payload.choices.length > 0) {
    const choice = payload.choices[0];
    if (choice && typeof choice === "object") {
      const delta = choice.delta || {};
      if (delta && typeof delta === "object" && delta.content) {
        return delta.content;
      }
      const message = choice.message || {};
      if (message && typeof message === "object" && message.content) {
        return message.content;
      }
      if (choice.text) {
        return choice.text;
      }
    }
  }

  if (typeof payload.content === "string") {
    return payload.content;
  }

  if (Array.isArray(payload.content)) {
    return payload.content.filter((part) => typeof part === "string").join("");
  }

  return null;
}

function extractUsageFromPayload(payload) {
  if (!payload || typeof payload !== "object") {
    return null;
  }

  const usage = payload.usage;
  if (usage && typeof usage === "object") {
    const promptTokens = Number.isFinite(usage.prompt_tokens)
      ? usage.prompt_tokens
      : null;
    const completionTokens = Number.isFinite(usage.completion_tokens)
      ? usage.completion_tokens
      : null;
    const cachedTokens = Number.isFinite(usage.cached_tokens)
      ? usage.cached_tokens
      : null;
    if (Number.isFinite(usage.total_tokens)) {
      return {
        totalTokens: usage.total_tokens,
        promptTokens,
        completionTokens,
        cachedTokens,
      };
    }
    const promptValue = Number.isFinite(promptTokens) ? promptTokens : 0;
    const completionValue = Number.isFinite(completionTokens)
      ? completionTokens
      : 0;
    const total = promptValue + completionValue;
    if (total > 0) {
      return {
        totalTokens: total,
        promptTokens,
        completionTokens,
        cachedTokens,
      };
    }
  }

  return null;
}

function truncateTextToBytes(text, maxBytes) {
  if (!text) {
    return { text: "", bytes: 0, truncated: false };
  }
  const totalBytes = Buffer.byteLength(text, "utf-8");
  if (!Number.isFinite(maxBytes) || maxBytes <= 0 || totalBytes <= maxBytes) {
    return { text, bytes: totalBytes, truncated: false };
  }
  let low = 0;
  let high = text.length;
  while (low < high) {
    const mid = Math.ceil((low + high) / 2);
    const slice = text.slice(0, mid);
    if (Buffer.byteLength(slice, "utf-8") <= maxBytes) {
      low = mid;
    } else {
      high = mid - 1;
    }
  }
  const truncatedText = text.slice(0, low);
  return {
    text: truncatedText,
    bytes: Buffer.byteLength(truncatedText, "utf-8"),
    truncated: true,
  };
}

function appendChunkWithLimit(state, chunk) {
  if (!chunk || state.responseTruncated) {
    return;
  }
  const remainingBytes = Math.max(
    state.maxResponseBytes - state.responseBytes,
    0,
  );
  if (remainingBytes <= 0) {
    state.responseTruncated = true;
    return;
  }
  const chunkBytes = Buffer.byteLength(chunk, "utf-8");
  if (chunkBytes <= remainingBytes) {
    state.responseParts.push(chunk);
    state.responseBytes += chunkBytes;
    return;
  }
  const truncated = truncateTextToBytes(chunk, remainingBytes);
  if (truncated.text) {
    state.responseParts.push(truncated.text);
    state.responseBytes += truncated.bytes;
  }
  state.responseTruncated = true;
}

function handleRawChunk(rawText, responseStream, usageState) {
  const lines = rawText.split(/\r?\n/);
  let handled = false;

  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed.startsWith("data:")) {
      continue;
    }

    handled = true;
    const payloadText = trimmed.slice(5).trim();
    if (!payloadText || payloadText === "[DONE]") {
      continue;
    }

    try {
      const payload = JSON.parse(payloadText);
      const usageTokens = extractUsageFromPayload(payload);
      if (usageTokens && Number.isFinite(usageTokens.totalTokens)) {
        usageState.tokens = usageTokens;
      }
      const text = extractTextFromPayload(payload);
      if (text) {
        responseStream.write(sseEvent(text, false));
        appendChunkWithLimit(usageState, text);
      }
    } catch (error) {
      console.warn("Failed to parse Bedrock chunk", error);
    }
  }

  if (handled) {
    return;
  }

  try {
    const payload = JSON.parse(rawText);
    const usageTokens = extractUsageFromPayload(payload);
    if (usageTokens && Number.isFinite(usageTokens.totalTokens)) {
      usageState.tokens = usageTokens;
    }
    const text = extractTextFromPayload(payload);
    if (text) {
      responseStream.write(sseEvent(text, false));
      appendChunkWithLimit(usageState, text);
    }
  } catch (error) {
    console.warn("Failed to parse Bedrock payload", error);
  }
}

const streamingHandler = async (event, responseStream) => {
  let body;
  try {
    const rawBody = event.isBase64Encoded
      ? Buffer.from(event.body || "", "base64").toString("utf-8")
      : event.body || "{}";
    body = JSON.parse(rawBody);
  } catch (error) {
    return buildErrorResponse(responseStream, 400, "Invalid JSON");
  }

  const user = body.user;
  const safeUser = normalizeUser(user);
  const passwordHash = body.password_hash;
  const prompt = body.prompt;

  if (!user || !safeUser || !passwordHash || !prompt) {
    return buildErrorResponse(
      responseStream,
      400,
      "Missing user, password_hash, or prompt",
    );
  }

  if (!USAGE_TABLE_NAME || !Number.isFinite(USAGE_DEFAULT_BUDGET)) {
    return buildErrorResponse(
      responseStream,
      500,
      "Usage control is not configured",
    );
  }

  try {
    const storedHashParam = await ssm.send(
      new GetParameterCommand({
        Name: `/secrets/${safeUser}/password_hash`,
        WithDecryption: true,
      }),
    );
    const storedHash = storedHashParam.Parameter.Value;
    if (passwordHash !== storedHash) {
      return buildErrorResponse(responseStream, 401, "Invalid credentials");
    }
  } catch (error) {
    if (error.name === "ParameterNotFound") {
      return buildErrorResponse(responseStream, 404, "User not found");
    }
    console.error("Auth error", error);
    return buildErrorResponse(responseStream, 500, "Internal server error");
  }

  let usageRecord;
  try {
    const usageResponse = await usageDb.send(
      new GetItemCommand({
        TableName: USAGE_TABLE_NAME,
        Key: marshall({ user_id: safeUser, record_id: "summary" }),
      }),
    );
    if (usageResponse.Item) {
      usageRecord = unmarshall(usageResponse.Item);
    } else {
      const now = new Date().toISOString();
      const newRecord = {
        user_id: safeUser,
        record_id: "summary",
        budget_tokens: USAGE_DEFAULT_BUDGET,
        remaining_tokens: USAGE_DEFAULT_BUDGET,
        total_tokens: 0,
        updated_at: now,
      };
      await usageDb.send(
        new PutItemCommand({
          TableName: USAGE_TABLE_NAME,
          Item: marshall(newRecord),
          ConditionExpression: "attribute_not_exists(user_id)",
        }),
      );
      usageRecord = newRecord;
    }
  } catch (error) {
    if (error.name === "ConditionalCheckFailedException") {
      try {
        const usageResponse = await usageDb.send(
          new GetItemCommand({
            TableName: USAGE_TABLE_NAME,
            Key: marshall({ user_id: safeUser, record_id: "summary" }),
          }),
        );
        if (usageResponse.Item) {
          usageRecord = unmarshall(usageResponse.Item);
        }
      } catch (retryError) {
        console.error("Usage lookup retry error", retryError);
        return buildErrorResponse(responseStream, 500, "Internal server error");
      }
    } else {
      console.error("Usage lookup error", error);
      return buildErrorResponse(responseStream, 500, "Internal server error");
    }
  }

  if (usageRecord) {
    const remaining = Number.isFinite(usageRecord.remaining_tokens)
      ? usageRecord.remaining_tokens
      : USAGE_DEFAULT_BUDGET;
    if (remaining <= 0) {
      return buildErrorResponse(responseStream, 429, "Usage budget exceeded");
    }
  }

  const requestBody = {
    model: MODEL_ID,
    messages: [{ role: "user", content: prompt }],
    reasoning_effort: REASONING_EFFORT,
    stream: true,
    stream_options: { include_usage: true },
  };

  if (Number.isFinite(MAX_TOKENS)) {
    requestBody.max_tokens = MAX_TOKENS;
  }

  if (typeof responseStream.setContentType === "function") {
    responseStream.setContentType("text/event-stream");
  }

  const requestId = `req#${new Date().toISOString()}#${Math.random()
    .toString(36)
    .slice(2, 10)}`;
  const promptText = String(prompt);
  const promptMaxBytes = Math.max(Math.floor(USAGE_MAX_TEXT_BYTES / 2), 0);
  const promptResult = truncateTextToBytes(promptText, promptMaxBytes);
  const usageState = {
    tokens: null,
    responseParts: [],
    responseBytes: 0,
    responseTruncated: false,
    maxResponseBytes: Math.max(USAGE_MAX_TEXT_BYTES - promptResult.bytes, 0),
    prompt: promptResult.text,
    promptBytes: promptResult.bytes,
    promptTruncated: promptResult.truncated,
  };

  try {
    const command = new InvokeModelWithResponseStreamCommand({
      modelId: MODEL_ID,
      contentType: "application/json",
      accept: "application/json",
      body: JSON.stringify(requestBody),
    });
    const response = await bedrock.send(command);

    const decoder = new TextDecoder("utf-8");

    for await (const eventChunk of response.body) {
      const chunk = eventChunk.chunk;
      if (!chunk || !chunk.bytes) {
        continue;
      }
      const rawText = decoder.decode(chunk.bytes, { stream: false }).trim();
      if (rawText) {
        handleRawChunk(rawText, responseStream, usageState);
      }
    }

    const projectedRemaining = computeProjectedRemainingTokens(
      usageRecord,
      usageState.tokens,
    );
    responseStream.write(sseEvent("", true, projectedRemaining));
    responseStream.end();
  } catch (error) {
    console.error("Bedrock invoke error", error);
    const projectedRemaining = computeProjectedRemainingTokens(
      usageRecord,
      usageState.tokens,
    );
    responseStream.write(sseEvent("", true, projectedRemaining));
    responseStream.end();
  } finally {
    const responseText = usageState.responseParts.join("");
    const responseBytes = usageState.responseBytes;
    const responseTruncated = usageState.responseTruncated;
    const tokens = usageState.tokens || {};
    const totalTokens = tokens.totalTokens;
    if (Number.isFinite(totalTokens) && totalTokens > 0) {
      const remaining =
        usageRecord && Number.isFinite(usageRecord.remaining_tokens)
          ? usageRecord.remaining_tokens
          : USAGE_DEFAULT_BUDGET;
      const newRemaining = Math.max(remaining - totalTokens, 0);
      const now = new Date().toISOString();
      try {
        await usageDb.send(
          new UpdateItemCommand({
            TableName: USAGE_TABLE_NAME,
            Key: marshall({ user_id: safeUser, record_id: "summary" }),
            UpdateExpression:
              "SET remaining_tokens = :remaining, " +
              "total_tokens = if_not_exists(total_tokens, :zero) + :used, " +
              "budget_tokens = if_not_exists(budget_tokens, :budget), " +
              "updated_at = :now",
            ExpressionAttributeValues: marshall({
              ":remaining": newRemaining,
              ":used": totalTokens,
              ":zero": 0,
              ":budget": USAGE_DEFAULT_BUDGET,
              ":now": now,
            }),
          }),
        );
      } catch (error) {
        console.error("Usage update error", error);
      }
    }

    try {
      const createdAt = new Date().toISOString();
      const item = {
        user_id: safeUser,
        record_id: requestId,
        created_at: createdAt,
        prompt: usageState.prompt,
        response: responseText,
        prompt_bytes: usageState.promptBytes,
        response_bytes: responseBytes,
        prompt_truncated: usageState.promptTruncated,
        response_truncated: responseTruncated,
        tokens_total: Number.isFinite(totalTokens) ? totalTokens : null,
        tokens_input: Number.isFinite(tokens.promptTokens)
          ? tokens.promptTokens
          : null,
        tokens_output: Number.isFinite(tokens.completionTokens)
          ? tokens.completionTokens
          : null,
        tokens_cached: Number.isFinite(tokens.cachedTokens)
          ? tokens.cachedTokens
          : null,
      };
      await usageDb.send(
        new PutItemCommand({
          TableName: USAGE_TABLE_NAME,
          Item: marshall(item, { removeUndefinedValues: true }),
        }),
      );
    } catch (error) {
      console.error("Usage detail insert error", error);
    }
  }
};

exports.handler = streamifyResponse
  ? streamifyResponse(streamingHandler)
  : async () => ({
      statusCode: 500,
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        error: "Response streaming is not available in this runtime.",
      }),
    });
