const streamifyResponse =
  globalThis.awslambda &&
  typeof globalThis.awslambda.streamifyResponse === "function"
    ? globalThis.awslambda.streamifyResponse
    : null;
const {
  BedrockRuntimeClient,
  ConverseStreamCommand,
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
const BEDROCK_REGION_US_WEST_2 = "us-west-2";
const ssm = new SSMClient({ region: AWS_REGION });
const bedrock = new BedrockRuntimeClient({ region: AWS_REGION });
const bedrockUsWest2 = new BedrockRuntimeClient({
  region: BEDROCK_REGION_US_WEST_2,
});
const usageRegion = "us-east-1";
const usageDb = new DynamoDBClient({ region: usageRegion });

const MODEL_ID = process.env.BEDROCK_MODEL_ID || "openai.gpt-oss-120b-1:0";
const FRQ_MODEL_ID =
  process.env.BEDROCK_MODEL_ID_FRQ ||
  process.env.BEDROCK_FRQ_MODEL_ID ||
  "us.deepseek.r1-v1:0";
const REASONING_EFFORT = process.env.BEDROCK_REASONING_EFFORT || "medium";
const MAX_TOKENS = process.env.BEDROCK_MAX_TOKENS
  ? Number(process.env.BEDROCK_MAX_TOKENS)
  : 20480;
const TEMPERATURE = process.env.BEDROCK_TEMPERATURE
  ? Number(process.env.BEDROCK_TEMPERATURE)
  : undefined;
const TOP_P = process.env.BEDROCK_TOP_P
  ? Number(process.env.BEDROCK_TOP_P)
  : undefined;
const STOP_SEQUENCES = process.env.BEDROCK_STOP_SEQUENCES
  ? process.env.BEDROCK_STOP_SEQUENCES.split(",")
      .map((item) => item.trim())
      .filter(Boolean)
  : [];
const USAGE_TABLE_NAME = "catieBedrockUsage";
const USAGE_DEFAULT_BUDGET = 1000000;
const USAGE_MAX_TEXT_BYTES = 300000;

function logGatewayError(code, message, details = null) {
  const entry = {
    level: "error",
    source: "bedrock_gateway",
    timestamp: new Date().toISOString(),
    code,
    message,
    ...(details && typeof details === "object" ? { details } : {}),
  };

  const serialized = JSON.stringify(entry);

  // CloudWatch captures console.error as stderr and console.log as stdout.
  console.error(serialized);
  console.log(serialized);

  if (typeof process?.stderr?.write === "function") {
    process.stderr.write(`${serialized}\n`);
  }
  if (typeof process?.stdout?.write === "function") {
    process.stdout.write(`${serialized}\n`);
  }
}

function buildErrorResponse(
  responseStream,
  statusCode,
  message,
  details = null,
) {
  logGatewayError("gateway_http_error", message, {
    statusCode,
    ...(details && typeof details === "object" ? details : {}),
  });
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

function sseEvent(text, done, remainingTokens = null, meta = null) {
  const payload = { text, done };
  if (Number.isFinite(remainingTokens)) {
    payload.remaining_tokens = remainingTokens;
  }
  if (meta && typeof meta === "object") {
    Object.assign(payload, meta);
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

function detectBlockTypeFromStart(start) {
  if (!start || typeof start !== "object") {
    return null;
  }

  if (
    typeof start.reasoningText === "string" ||
    typeof start.reasoningContent?.text === "string" ||
    typeof start.reasoningContent?.reasoningText?.text === "string" ||
    start.reasoningContent
  ) {
    return "reasoning";
  }

  if (typeof start.text === "string") {
    return "response";
  }

  return null;
}

function extractTextFromConverseEvent(eventChunk, usageState) {
  if (!eventChunk || typeof eventChunk !== "object") {
    return null;
  }

  const deltaEvent = eventChunk.contentBlockDelta;
  const delta = deltaEvent?.delta;
  if (delta && typeof delta === "object") {
    if (typeof delta.text === "string" && delta.text.length > 0) {
      return {
        text: delta.text,
        blockType: "response",
        contentBlockIndex: Number.isInteger(deltaEvent?.contentBlockIndex)
          ? deltaEvent.contentBlockIndex
          : null,
      };
    }

    // Some reasoning-capable models can emit reasoning text blocks.
    const reasoningText = delta.reasoningContent?.reasoningText?.text;
    if (typeof reasoningText === "string" && reasoningText.length > 0) {
      return {
        text: reasoningText,
        blockType: "reasoning",
        contentBlockIndex: Number.isInteger(deltaEvent?.contentBlockIndex)
          ? deltaEvent.contentBlockIndex
          : null,
      };
    }

    // Some providers emit reasoning text directly on reasoningContent.text.
    const reasoningContentText = delta.reasoningContent?.text;
    if (
      typeof reasoningContentText === "string" &&
      reasoningContentText.length > 0
    ) {
      return {
        text: reasoningContentText,
        blockType: "reasoning",
        contentBlockIndex: Number.isInteger(deltaEvent?.contentBlockIndex)
          ? deltaEvent.contentBlockIndex
          : null,
      };
    }

    if (
      typeof delta.reasoningText === "string" &&
      delta.reasoningText.length > 0
    ) {
      return {
        text: delta.reasoningText,
        blockType: "reasoning",
        contentBlockIndex: Number.isInteger(deltaEvent?.contentBlockIndex)
          ? deltaEvent.contentBlockIndex
          : null,
      };
    }
  }

  const startEvent = eventChunk.contentBlockStart;
  const start = startEvent?.start;
  if (start && typeof start === "object" && typeof start.text === "string") {
    return {
      text: start.text,
      blockType: "response",
      contentBlockIndex: Number.isInteger(startEvent?.contentBlockIndex)
        ? startEvent.contentBlockIndex
        : null,
    };
  }

  const startReasoningText = start?.reasoningContent?.reasoningText?.text;
  if (typeof startReasoningText === "string" && startReasoningText.length > 0) {
    return {
      text: startReasoningText,
      blockType: "reasoning",
      contentBlockIndex: Number.isInteger(startEvent?.contentBlockIndex)
        ? startEvent.contentBlockIndex
        : null,
    };
  }

  const startReasoningContentText = start?.reasoningContent?.text;
  if (
    typeof startReasoningContentText === "string" &&
    startReasoningContentText.length > 0
  ) {
    return {
      text: startReasoningContentText,
      blockType: "reasoning",
      contentBlockIndex: Number.isInteger(startEvent?.contentBlockIndex)
        ? startEvent.contentBlockIndex
        : null,
    };
  }

  const outputMessage = eventChunk.messageStart?.message;
  if (outputMessage && Array.isArray(outputMessage.content)) {
    for (const part of outputMessage.content) {
      if (!part || typeof part !== "object") {
        continue;
      }
      if (typeof part.text === "string" && part.text.length > 0) {
        return {
          text: part.text,
          blockType: "response",
          contentBlockIndex: null,
        };
      }
      if (
        typeof part.reasoningText === "string" &&
        part.reasoningText.length > 0
      ) {
        return {
          text: part.reasoningText,
          blockType: "reasoning",
          contentBlockIndex: null,
        };
      }
      if (
        typeof part.reasoningContent?.text === "string" &&
        part.reasoningContent.text.length > 0
      ) {
        return {
          text: part.reasoningContent.text,
          blockType: "reasoning",
          contentBlockIndex: null,
        };
      }
      if (
        typeof part.reasoningContent?.reasoningText?.text === "string" &&
        part.reasoningContent.reasoningText.text.length > 0
      ) {
        return {
          text: part.reasoningContent.reasoningText.text,
          blockType: "reasoning",
          contentBlockIndex: null,
        };
      }
    }
  }

  const deltaIndex = Number.isInteger(deltaEvent?.contentBlockIndex)
    ? deltaEvent.contentBlockIndex
    : null;
  if (
    deltaIndex !== null &&
    usageState &&
    usageState.blockTypesByIndex &&
    usageState.blockTypesByIndex[deltaIndex]
  ) {
    return {
      text: null,
      blockType: usageState.blockTypesByIndex[deltaIndex],
      contentBlockIndex: deltaIndex,
    };
  }

  return null;
}

function extractUsageFromConverseEvent(eventChunk) {
  if (!eventChunk || typeof eventChunk !== "object") {
    return null;
  }

  const usage = eventChunk.metadata?.usage || eventChunk.usage;
  if (usage && typeof usage === "object") {
    const promptTokens = Number.isFinite(usage.inputTokens)
      ? usage.inputTokens
      : null;
    const completionTokens = Number.isFinite(usage.outputTokens)
      ? usage.outputTokens
      : null;
    if (Number.isFinite(usage.totalTokens)) {
      return {
        totalTokens: usage.totalTokens,
        promptTokens,
        completionTokens,
        cachedTokens: null,
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
        cachedTokens: null,
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

function handleConverseEvent(eventChunk, responseStream, usageState) {
  const startEvent = eventChunk?.contentBlockStart;
  if (startEvent && Number.isInteger(startEvent.contentBlockIndex)) {
    const blockType = detectBlockTypeFromStart(startEvent.start);
    if (blockType) {
      usageState.blockTypesByIndex[startEvent.contentBlockIndex] = blockType;
      if (blockType === "reasoning") {
        usageState.reasoningSeen = true;
        usageState.activeReasoningBlocks += 1;
      }
    }
  }

  const stopEvent = eventChunk?.contentBlockStop;
  if (stopEvent && Number.isInteger(stopEvent.contentBlockIndex)) {
    const blockType = usageState.blockTypesByIndex[stopEvent.contentBlockIndex];
    if (blockType === "reasoning") {
      usageState.activeReasoningBlocks = Math.max(
        usageState.activeReasoningBlocks - 1,
        0,
      );
      if (
        usageState.reasoningSeen &&
        usageState.activeReasoningBlocks === 0 &&
        !usageState.reasoningEndSent
      ) {
        usageState.reasoningEndSent = true;
        responseStream.write(
          sseEvent("", false, null, {
            block_type: "reasoning_end",
            reasoning_done: true,
          }),
        );
      }
    }
    delete usageState.blockTypesByIndex[stopEvent.contentBlockIndex];
  }

  const usageTokens = extractUsageFromConverseEvent(eventChunk);
  if (usageTokens && Number.isFinite(usageTokens.totalTokens)) {
    usageState.tokens = usageTokens;
  }

  const extracted = extractTextFromConverseEvent(eventChunk, usageState);
  if (extracted && extracted.text) {
    const blockType = extracted.blockType || "response";

    if (
      blockType === "response" &&
      usageState.reasoningSeen &&
      !usageState.reasoningEndSent
    ) {
      usageState.reasoningEndSent = true;
      responseStream.write(
        sseEvent("", false, null, {
          block_type: "reasoning_end",
          reasoning_done: true,
        }),
      );
    }

    responseStream.write(
      sseEvent(extracted.text, false, null, {
        block_type: blockType,
        reasoning_done: usageState.reasoningEndSent,
      }),
    );
    appendChunkWithLimit(usageState, extracted.text);
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
    return buildErrorResponse(responseStream, 400, "Invalid JSON", {
      error_name: error?.name,
      error_message: error?.message,
    });
  }

  const user = body.user;
  const safeUser = normalizeUser(user);
  const passwordHash = body.password_hash;
  const prompt = body.prompt;
  const questionType =
    typeof body.question_type === "string"
      ? body.question_type.trim().toLowerCase()
      : "";
  const selectedModelId = questionType === "frq" ? FRQ_MODEL_ID : MODEL_ID;

  if (!user || !safeUser || !passwordHash || !prompt) {
    return buildErrorResponse(
      responseStream,
      400,
      "Missing user, password_hash, or prompt",
      {
        has_user: Boolean(user),
        has_password_hash: Boolean(passwordHash),
        has_prompt: Boolean(prompt),
      },
    );
  }

  if (!USAGE_TABLE_NAME || !Number.isFinite(USAGE_DEFAULT_BUDGET)) {
    return buildErrorResponse(
      responseStream,
      500,
      "Usage control is not configured",
      {
        usage_table_name: USAGE_TABLE_NAME,
        default_budget_valid: Number.isFinite(USAGE_DEFAULT_BUDGET),
      },
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
      return buildErrorResponse(responseStream, 401, "Invalid credentials", {
        safe_user: safeUser,
      });
    }
  } catch (error) {
    if (error.name === "ParameterNotFound") {
      return buildErrorResponse(responseStream, 404, "User not found", {
        safe_user: safeUser,
      });
    }
    logGatewayError("auth_error", "Authentication lookup failed", {
      safe_user: safeUser,
      error_name: error?.name,
      error_message: error?.message,
      stack: error?.stack,
    });
    return buildErrorResponse(responseStream, 500, "Internal server error", {
      safe_user: safeUser,
    });
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
        logGatewayError(
          "usage_lookup_retry_error",
          "Usage lookup retry failed",
          {
            safe_user: safeUser,
            error_name: retryError?.name,
            error_message: retryError?.message,
            stack: retryError?.stack,
          },
        );
        return buildErrorResponse(
          responseStream,
          500,
          "Internal server error",
          {
            safe_user: safeUser,
          },
        );
      }
    } else {
      logGatewayError("usage_lookup_error", "Usage lookup failed", {
        safe_user: safeUser,
        error_name: error?.name,
        error_message: error?.message,
        stack: error?.stack,
      });
      return buildErrorResponse(responseStream, 500, "Internal server error", {
        safe_user: safeUser,
      });
    }
  }

  if (usageRecord) {
    const remaining = Number.isFinite(usageRecord.remaining_tokens)
      ? usageRecord.remaining_tokens
      : USAGE_DEFAULT_BUDGET;
    if (remaining <= 0) {
      return buildErrorResponse(responseStream, 429, "Usage budget exceeded", {
        safe_user: safeUser,
        remaining,
      });
    }
  }

  const inferenceConfig = {};
  if (Number.isFinite(MAX_TOKENS)) {
    inferenceConfig.maxTokens = MAX_TOKENS;
  }
  if (Number.isFinite(TEMPERATURE)) {
    inferenceConfig.temperature = TEMPERATURE;
  }
  if (Number.isFinite(TOP_P)) {
    inferenceConfig.topP = TOP_P;
  }
  if (STOP_SEQUENCES.length > 0) {
    inferenceConfig.stopSequences = STOP_SEQUENCES;
  }

  const additionalModelRequestFields = {};
  const supportsReasoningEffort =
    typeof selectedModelId === "string" &&
    (selectedModelId.startsWith("openai.gpt-oss") ||
      selectedModelId.startsWith("deepseek.v3"));
  if (
    supportsReasoningEffort &&
    typeof REASONING_EFFORT === "string" &&
    REASONING_EFFORT.trim()
  ) {
    additionalModelRequestFields.reasoning_effort = REASONING_EFFORT.trim();
  }

  const selectedBedrockRegion =
    selectedModelId === "deepseek.v3-v1:0"
      ? BEDROCK_REGION_US_WEST_2
      : AWS_REGION;
  console.log(
    JSON.stringify({
      level: "info",
      source: "bedrock_gateway",
      code: "request_model_selection",
      timestamp: new Date().toISOString(),
      safe_user: safeUser,
      question_type: questionType || null,
      selected_model_id: selectedModelId,
      selected_bedrock_region: selectedBedrockRegion,
      reasoning_effort: additionalModelRequestFields.reasoning_effort || null,
    }),
  );

  const requestBody = {
    modelId: selectedModelId,
    messages: [
      {
        role: "user",
        content: [{ text: prompt }],
      },
    ],
    ...(Object.keys(inferenceConfig).length > 0 ? { inferenceConfig } : {}),
    ...(Object.keys(additionalModelRequestFields).length > 0
      ? { additionalModelRequestFields }
      : {}),
  };

  const bedrockClient =
    selectedModelId === "deepseek.v3-v1:0" ? bedrockUsWest2 : bedrock;

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
    blockTypesByIndex: {},
    activeReasoningBlocks: 0,
    reasoningSeen: false,
    reasoningEndSent: false,
  };

  try {
    const command = new ConverseStreamCommand(requestBody);
    const response = await bedrockClient.send(command);

    for await (const eventChunk of response.stream || []) {
      handleConverseEvent(eventChunk, responseStream, usageState);
    }

    const projectedRemaining = computeProjectedRemainingTokens(
      usageRecord,
      usageState.tokens,
    );
    if (usageState.responseParts.length === 0) {
      logGatewayError(
        "empty_model_response",
        "Model returned an empty response",
        {
          safe_user: safeUser,
          selected_model_id: selectedModelId,
          question_type: questionType || null,
          projected_remaining_tokens: projectedRemaining,
        },
      );
      responseStream.write(
        sseEvent("", true, projectedRemaining, {
          error: "Model returned an empty response",
          error_type: "EmptyModelResponse",
        }),
      );
    } else {
      responseStream.write(sseEvent("", true, projectedRemaining));
    }
    responseStream.end();
  } catch (error) {
    const projectedRemaining = computeProjectedRemainingTokens(
      usageRecord,
      usageState.tokens,
    );
    const message =
      (error && typeof error.message === "string" && error.message.trim()) ||
      "Bedrock invocation failed";
    const name =
      (error && typeof error.name === "string" && error.name.trim()) ||
      "BedrockInvokeError";
    logGatewayError("bedrock_invoke_error", message, {
      safe_user: safeUser,
      selected_model_id: selectedModelId,
      question_type: questionType || null,
      error_name: name,
      stack: error?.stack,
      projected_remaining_tokens: projectedRemaining,
    });
    responseStream.write(
      sseEvent("", true, projectedRemaining, {
        error: message,
        error_type: name,
      }),
    );
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
        logGatewayError(
          "usage_update_error",
          "Failed to update usage summary",
          {
            safe_user: safeUser,
            error_name: error?.name,
            error_message: error?.message,
            stack: error?.stack,
          },
        );
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
      logGatewayError(
        "usage_detail_insert_error",
        "Failed to insert usage detail record",
        {
          safe_user: safeUser,
          request_id: requestId,
          error_name: error?.name,
          error_message: error?.message,
          stack: error?.stack,
        },
      );
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
