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

const AWS_REGION = process.env.AWS_REGION || "us-east-1";
const ssm = new SSMClient({ region: AWS_REGION });
const bedrock = new BedrockRuntimeClient({ region: AWS_REGION });

const MODEL_ID = process.env.BEDROCK_MODEL_ID || "openai.gpt-oss-120b-1:0";
const REASONING_EFFORT = process.env.BEDROCK_REASONING_EFFORT || "medium";
const MAX_TOKENS = process.env.BEDROCK_MAX_TOKENS
  ? Number(process.env.BEDROCK_MAX_TOKENS)
  : 2048;

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

function sseEvent(text, done) {
  return `data: ${JSON.stringify({ text, done })}\n\n`;
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

function handleRawChunk(rawText, responseStream) {
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
      const text = extractTextFromPayload(payload);
      if (text) {
        responseStream.write(sseEvent(text, false));
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
    const text = extractTextFromPayload(payload);
    if (text) {
      responseStream.write(sseEvent(text, false));
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
        handleRawChunk(rawText, responseStream);
      }
    }

    responseStream.write(sseEvent("", true));
    responseStream.end();
  } catch (error) {
    console.error("Bedrock invoke error", error);
    responseStream.write(sseEvent("", true));
    responseStream.end();
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
