const streamifyResponse =
  globalThis.awslambda &&
  typeof globalThis.awslambda.streamifyResponse === "function"
    ? globalThis.awslambda.streamifyResponse
    : null;
const crypto = require("node:crypto");
const { promisify } = require("node:util");
const Anthropic = require("@anthropic-ai/sdk");
const { SSMClient, GetParameterCommand } = require("@aws-sdk/client-ssm");
const {
  DynamoDBClient,
  GetItemCommand,
  PutItemCommand,
  UpdateItemCommand,
} = require("@aws-sdk/client-dynamodb");
const { marshall, unmarshall } = require("@aws-sdk/util-dynamodb");

const scryptAsync = promisify(crypto.scrypt);

const AWS_REGION = "us-east-1";
const ssm = new SSMClient({ region: AWS_REGION });
const usageRegion = "us-east-1";
const usageDb = new DynamoDBClient({ region: usageRegion });

// Model configuration (Anthropic API, called directly).
const MODEL_ID = process.env.ANTHROPIC_MODEL || "claude-haiku-5-5";
const FRQ_MODEL_ID = process.env.ANTHROPIC_MODEL_FRQ || MODEL_ID;
// Effort controls how much the model thinks. Claude Haiku 5.5 defaults to "medium".
const EFFORT = process.env.ANTHROPIC_EFFORT || "medium";
const FRQ_EFFORT = process.env.ANTHROPIC_EFFORT_FRQ || "high";
// Thinking counts toward max_tokens, so leave room for it on top of the answer.
const MAX_TOKENS = process.env.ANTHROPIC_MAX_TOKENS
  ? Number(process.env.ANTHROPIC_MAX_TOKENS)
  : 32000;
// SSM SecureString holding the Anthropic API key (kept out of the /secrets/ per-user namespace).
const ANTHROPIC_API_KEY_PARAM =
  process.env.ANTHROPIC_API_KEY_PARAM || "/catie/anthropic_api_key";

// Password verification. The client sends the password over HTTPS; stored values are
// `scrypt$N$r$p$<salt b64>$<hash b64>` of it (see tools/password_hashing.py).
const SCRYPT_MAXMEM = 256 * 1024 * 1024;

const USAGE_TABLE_NAME = "catieBedrockUsage";
const USAGE_DEFAULT_BUDGET = 1000000;
const USAGE_MAX_TEXT_BYTES = 300000;

// Dollar budget per user. Spending is tracked in nano-dollars (1e-9 USD) so per-token prices
// are whole numbers and running totals stay exact.
const NANOUSD_PER_USD = 1e9;
const USAGE_DEFAULT_BUDGET_USD = process.env.USAGE_DEFAULT_BUDGET_USD
  ? Number(process.env.USAGE_DEFAULT_BUDGET_USD)
  : 5;
const USAGE_DEFAULT_BUDGET_NANOUSD = Math.round(
  USAGE_DEFAULT_BUDGET_USD * NANOUSD_PER_USD,
);

// Anthropic API prices in nano-dollars per token ($ per million tokens x 1000).
// Source: https://platform.claude.com/docs/en/about-claude/pricing (checked 2026-11-12).
// Update this table when prices change or a new model is configured; requests on a model
// missing here are logged as unpriced and do not reduce the dollar budget.
const MODEL_PRICES_NANOUSD = {
  "claude-haiku-5-5": {
    // A request whose prompt (input + cache reads + cache writes) exceeds this pays the
    // long-context prices for the whole request.
    longContextThreshold: 100000,
    standard: { input: 100, cacheWrite: 125, cacheRead: 10, output: 500 },
    longContext: { input: 500, cacheWrite: 625, cacheRead: 50, output: 2500 },
  },
};

/** Cost of one request in nano-dollars, or null if the model has no price entry. */
function computeCostNanoUsd(modelId, apiUsage) {
  const prices = MODEL_PRICES_NANOUSD[modelId];
  if (!prices || !apiUsage) {
    return null;
  }
  const input = apiUsage.input_tokens || 0;
  // Cache writes are priced at the 5-minute rate; the gateway does not request 1-hour caching.
  const cacheWrite = apiUsage.cache_creation_input_tokens || 0;
  const cacheRead = apiUsage.cache_read_input_tokens || 0;
  const output = apiUsage.output_tokens || 0;
  const promptTokens = input + cacheWrite + cacheRead;
  const rate =
    prices.longContextThreshold && promptTokens > prices.longContextThreshold
      ? prices.longContext
      : prices.standard;
  return (
    input * rate.input +
    cacheWrite * rate.cacheWrite +
    cacheRead * rate.cacheRead +
    output * rate.output
  );
}

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

/** `remaining` is `{ tokens, usd }` from `computeProjectedRemaining`, or null. */
function sseEvent(text, done, remaining = null, meta = null) {
  const payload = { text, done };
  if (remaining && Number.isFinite(remaining.tokens)) {
    payload.remaining_tokens = remaining.tokens;
  }
  if (remaining && Number.isFinite(remaining.usd)) {
    payload.remaining_usd = remaining.usd;
  }
  if (meta && typeof meta === "object") {
    Object.assign(payload, meta);
  }
  return `data: ${JSON.stringify(payload)}\n\n`;
}

/** Dollar budget left (nano-dollars) according to a summary record, before this request. */
function remainingNanoUsd(usageRecord) {
  const budget =
    usageRecord && Number.isFinite(usageRecord.budget_nanousd)
      ? usageRecord.budget_nanousd
      : USAGE_DEFAULT_BUDGET_NANOUSD;
  const spent =
    usageRecord && Number.isFinite(usageRecord.spent_nanousd)
      ? usageRecord.spent_nanousd
      : 0;
  return budget - spent;
}

/**
 * Remaining budget to report to the app after this request: legacy tokens (still shown by
 * older app versions) and dollars. `usageState` may be null (no model call made).
 */
function computeProjectedRemaining(usageRecord, usageState) {
  const currentTokens =
    usageRecord && Number.isFinite(usageRecord.remaining_tokens)
      ? usageRecord.remaining_tokens
      : USAGE_DEFAULT_BUDGET;
  const usedTokens =
    usageState?.tokens && Number.isFinite(usageState.tokens.totalTokens)
      ? usageState.tokens.totalTokens
      : 0;
  const cost = usageState?.tokens ? usageState.tokens.costNanoUsd : 0;
  const usedNano = Number.isFinite(cost) ? cost : 0;
  const remainingNano = Math.max(remainingNanoUsd(usageRecord) - usedNano, 0);
  return {
    tokens: Math.max(currentTokens - usedTokens, 0),
    // Rounded to a millionth of a dollar; plenty for display.
    usd: Math.round(remainingNano / 1000) / 1e6,
  };
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

function writeReasoningEndOnce(responseStream, usageState) {
  if (!usageState.reasoningSeen || usageState.reasoningEndSent) {
    return;
  }
  usageState.reasoningEndSent = true;
  responseStream.write(
    sseEvent("", false, null, {
      block_type: "reasoning_end",
      reasoning_done: true,
    }),
  );
}

function writeStreamText(responseStream, usageState, text, blockType) {
  if (!text) {
    return;
  }
  if (blockType === "response") {
    writeReasoningEndOnce(responseStream, usageState);
  }
  responseStream.write(
    sseEvent(text, false, null, {
      block_type: blockType,
      reasoning_done: usageState.reasoningEndSent,
    }),
  );
  appendChunkWithLimit(usageState, text);
}

// Usage arrives on message_start (input side) and message_delta (cumulative output),
// so partial usage is still recorded if the stream fails midway.
function recordUsage(usageState, usage) {
  if (!usage || typeof usage !== "object") {
    return;
  }
  const u = usageState.apiUsage;
  for (const key of [
    "input_tokens",
    "output_tokens",
    "cache_creation_input_tokens",
    "cache_read_input_tokens",
  ]) {
    if (Number.isFinite(usage[key])) {
      u[key] = usage[key];
    }
  }
  const input =
    (u.input_tokens || 0) +
    (u.cache_creation_input_tokens || 0) +
    (u.cache_read_input_tokens || 0);
  const output = u.output_tokens || 0;
  usageState.tokens = {
    totalTokens: input + output,
    promptTokens: input,
    completionTokens: output,
    cachedTokens: u.cache_read_input_tokens || 0,
    cacheWriteTokens: u.cache_creation_input_tokens || 0,
    costNanoUsd: computeCostNanoUsd(usageState.modelId, u),
  };
}

function handleAnthropicEvent(event, responseStream, usageState) {
  switch (event.type) {
    case "message_start":
      recordUsage(usageState, event.message?.usage);
      break;
    case "message_delta":
      recordUsage(usageState, event.usage);
      break;
    case "content_block_start":
      if (event.content_block?.type === "thinking") {
        usageState.blockTypesByIndex[event.index] = "reasoning";
        usageState.reasoningSeen = true;
        usageState.activeReasoningBlocks += 1;
      } else if (event.content_block?.type === "text") {
        usageState.blockTypesByIndex[event.index] = "response";
      }
      break;
    case "content_block_delta":
      if (event.delta?.type === "thinking_delta") {
        writeStreamText(
          responseStream,
          usageState,
          event.delta.thinking,
          "reasoning",
        );
      } else if (event.delta?.type === "text_delta") {
        writeStreamText(
          responseStream,
          usageState,
          event.delta.text,
          "response",
        );
      }
      break;
    case "content_block_stop":
      if (usageState.blockTypesByIndex[event.index] === "reasoning") {
        usageState.activeReasoningBlocks = Math.max(
          usageState.activeReasoningBlocks - 1,
          0,
        );
        if (usageState.activeReasoningBlocks === 0) {
          writeReasoningEndOnce(responseStream, usageState);
        }
      }
      delete usageState.blockTypesByIndex[event.index];
      break;
    default:
      break;
  }
}

// --- Anthropic client (API key read from SSM once per container) ---

let anthropicClientPromise = null;

function getAnthropicClient() {
  if (!anthropicClientPromise) {
    anthropicClientPromise = ssm
      .send(
        new GetParameterCommand({
          Name: ANTHROPIC_API_KEY_PARAM,
          WithDecryption: true,
        }),
      )
      .then((result) => new Anthropic({ apiKey: result.Parameter.Value }))
      .catch((error) => {
        anthropicClientPromise = null; // retry on the next request
        throw error;
      });
  }
  return anthropicClientPromise;
}

// --- Password verification ---

async function verifyPassword(password, stored) {
  if (typeof password !== "string" || typeof stored !== "string") {
    return { ok: false, reason: "invalid_input" };
  }
  const parts = stored.split("$");
  if (parts.length !== 6 || parts[0] !== "scrypt") {
    return { ok: false, reason: "malformed_record" };
  }
  const [, nStr, rStr, pStr, saltB64, hashB64] = parts;
  const N = Number(nStr);
  const r = Number(rStr);
  const p = Number(pStr);
  const salt = Buffer.from(saltB64, "base64");
  const expected = Buffer.from(hashB64, "base64");
  if (
    ![N, r, p].every(Number.isInteger) ||
    salt.length === 0 ||
    expected.length === 0
  ) {
    return { ok: false, reason: "malformed_record" };
  }
  const actual = await scryptAsync(password, salt, expected.length, {
    N,
    r,
    p,
    maxmem: SCRYPT_MAXMEM,
  });
  return { ok: crypto.timingSafeEqual(actual, expected), reason: "scrypt" };
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
  const password = body.password;
  const prompt = body.prompt;
  const questionType =
    typeof body.question_type === "string"
      ? body.question_type.trim().toLowerCase()
      : "";
  const selectedModelId = questionType === "frq" ? FRQ_MODEL_ID : MODEL_ID;

  if (!user || !safeUser || !password || !prompt) {
    return buildErrorResponse(
      responseStream,
      400,
      "Missing user, password, or prompt",
      {
        has_user: Boolean(user),
        has_password: Boolean(password),
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
    const storedRecord = storedHashParam.Parameter.Value;
    const verification = await verifyPassword(password, storedRecord);
    if (verification.reason === "malformed_record") {
      logGatewayError(
        "auth_record_malformed",
        "Password record is not a scrypt record",
        { safe_user: safeUser },
      );
    }
    if (!verification.ok) {
      return buildErrorResponse(responseStream, 401, "Invalid credentials", {
        safe_user: safeUser,
        reason: verification.reason,
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
        budget_nanousd: USAGE_DEFAULT_BUDGET_NANOUSD,
        spent_nanousd: 0,
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

  // The dollar budget is what is enforced; the legacy token count is reported only.
  if (usageRecord) {
    const remainingNano = remainingNanoUsd(usageRecord);
    if (remainingNano <= 0) {
      return buildErrorResponse(responseStream, 429, "Usage budget exceeded", {
        safe_user: safeUser,
        remaining_usd: remainingNano / NANOUSD_PER_USD,
      });
    }
  }

  // The desktop app validates logins with this fixed prompt; answer it without a model call.
  if (prompt === "auth_check") {
    if (typeof responseStream.setContentType === "function") {
      responseStream.setContentType("text/event-stream");
    }
    responseStream.write(
      sseEvent("", true, computeProjectedRemaining(usageRecord, null)),
    );
    responseStream.end();
    return;
  }

  const selectedEffort = questionType === "frq" ? FRQ_EFFORT : EFFORT;
  console.log(
    JSON.stringify({
      level: "info",
      source: "bedrock_gateway",
      code: "request_model_selection",
      timestamp: new Date().toISOString(),
      safe_user: safeUser,
      question_type: questionType || null,
      selected_model_id: selectedModelId,
      effort: selectedEffort,
    }),
  );

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
    modelId: selectedModelId,
    tokens: null,
    apiUsage: {},
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
    const client = await getAnthropicClient();
    const stream = client.messages.stream({
      model: selectedModelId,
      max_tokens: MAX_TOKENS,
      // Summarized thinking is streamed to the app as "reasoning" blocks, as before.
      thinking: { type: "adaptive", display: "summarized" },
      output_config: { effort: selectedEffort },
      messages: [{ role: "user", content: promptText }],
    });

    for await (const streamEvent of stream) {
      handleAnthropicEvent(streamEvent, responseStream, usageState);
    }
    const finalMessage = await stream.finalMessage();
    recordUsage(usageState, finalMessage.usage);

    if (finalMessage.stop_reason === "refusal") {
      const category = finalMessage.stop_details?.category || null;
      const projectedRemaining = computeProjectedRemaining(
        usageRecord,
        usageState,
      );
      logGatewayError("model_refusal", "Model declined the request", {
        safe_user: safeUser,
        selected_model_id: selectedModelId,
        category,
      });
      responseStream.write(
        sseEvent("", true, projectedRemaining, {
          error: "The model declined this request",
          error_type: "Refusal",
          refusal_category: category,
        }),
      );
      responseStream.end();
      return;
    }
    if (finalMessage.stop_reason === "max_tokens") {
      logGatewayError(
        "max_tokens_reached",
        "Response hit max_tokens and may be truncated",
        {
          safe_user: safeUser,
          selected_model_id: selectedModelId,
          max_tokens: MAX_TOKENS,
        },
      );
    }

    const projectedRemaining = computeProjectedRemaining(
      usageRecord,
      usageState,
    );
    if (usageState.responseParts.length === 0) {
      logGatewayError(
        "empty_model_response",
        "Model returned an empty response",
        {
          safe_user: safeUser,
          selected_model_id: selectedModelId,
          question_type: questionType || null,
          projected_remaining: projectedRemaining,
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
    const projectedRemaining = computeProjectedRemaining(
      usageRecord,
      usageState,
    );
    const message =
      (error && typeof error.message === "string" && error.message.trim()) ||
      "Model invocation failed";
    const name =
      (error && typeof error.name === "string" && error.name.trim()) ||
      "ModelInvokeError";
    logGatewayError("model_invoke_error", message, {
      safe_user: safeUser,
      selected_model_id: selectedModelId,
      question_type: questionType || null,
      error_name: name,
      api_status: error instanceof Anthropic.APIError ? error.status : null,
      stack: error?.stack,
      projected_remaining: projectedRemaining,
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
    const costNanoUsd = Number.isFinite(tokens.costNanoUsd)
      ? tokens.costNanoUsd
      : null;
    if (Number.isFinite(totalTokens) && totalTokens > 0) {
      if (costNanoUsd === null) {
        logGatewayError(
          "usage_unpriced_model",
          "No price entry for model; dollar budget not charged",
          { safe_user: safeUser, selected_model_id: selectedModelId },
        );
      }
      const now = new Date().toISOString();
      try {
        // ADD is atomic, so overlapping requests from one user cannot overwrite each
        // other's deductions (the old read-then-SET of remaining_tokens could).
        await usageDb.send(
          new UpdateItemCommand({
            TableName: USAGE_TABLE_NAME,
            Key: marshall({ user_id: safeUser, record_id: "summary" }),
            UpdateExpression:
              "SET budget_tokens = if_not_exists(budget_tokens, :budget), " +
              "budget_nanousd = if_not_exists(budget_nanousd, :budgetNano), " +
              "updated_at = :now " +
              "ADD total_tokens :used, remaining_tokens :negUsed, " +
              "spent_nanousd :cost",
            ExpressionAttributeValues: marshall({
              ":used": totalTokens,
              ":negUsed": -totalTokens,
              ":cost": costNanoUsd ?? 0,
              ":budget": USAGE_DEFAULT_BUDGET,
              ":budgetNano": USAGE_DEFAULT_BUDGET_NANOUSD,
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
        tokens_cache_write: Number.isFinite(tokens.cacheWriteTokens)
          ? tokens.cacheWriteTokens
          : null,
        model_id: selectedModelId,
        cost_nanousd: costNanoUsd,
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
