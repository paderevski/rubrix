# Bedrock Gateway Contract

## Overview
This document defines the request and response contract for the Bedrock gateway.
The gateway accepts user credentials and a prompt, calls Bedrock server-side, and
streams chunks back using Server-Sent Events (SSE). The response payload shape
must match the existing `llm-stream` event format.

## Endpoint
- Method: POST
- Path: /generate
- Content-Type: application/json
- Accept: text/event-stream

## Request JSON
```json
{
  "user": "string",
  "password_hash": "string",
  "prompt": "string",
  "question_type": "multiple_choice | frq",
  "meta": {
    "subject": "string",
    "topics": ["string"],
    "difficulty": "string",
    "count": 0,
    "regenerate": false,
    "user_instructions": "string"
  }
}
```

### Fields
- `user`: Username.
- `password_hash`: SHA256 hex digest of the user's password.
- `prompt`: Fully rendered prompt text sent to Bedrock.
- `question_type`: Optional generation type hint. Supported values currently include `multiple_choice` and `frq`.
  - When omitted, gateway defaults to standard model routing.
- `meta`: Optional request metadata for logging or usage tracking.
  - The gateway should accept unknown keys and ignore them.

## Model Routing

The gateway can route by `question_type`:

- `frq` requests: FRQ model
- all other values: default model

Environment variables:

- `BEDROCK_MODEL_ID` (default model, fallback `openai.gpt-oss-120b-1:0`)
- `BEDROCK_MODEL_ID_FRQ` (preferred FRQ model override)
- `BEDROCK_FRQ_MODEL_ID` (legacy FRQ model override alias)

Current FRQ fallback model ID: `deepseek.v3.2`.

## Response (SSE)
- Content-Type: text/event-stream
- Each chunk uses the `data:` prefix and ends with a blank line.
- Payload matches the `llm-stream` shape: `{ "text": "...", "done": false }`.

Example stream:
```
HTTP/1.1 200 OK
Content-Type: text/event-stream
Cache-Control: no-cache

data: {"text":"Hello ","done":false}

data: {"text":"world","done":false}

data: {"text":"","done":true}

```

### Notes
- `text` is the incremental content chunk.
- The final event must set `done` to `true`.

## Error Responses
- 400: Invalid JSON or missing required fields.
- 401: Invalid password hash.
- 404: Unknown user.
- 429: Rate limit exceeded.
- 500: Internal error.

Errors are returned as JSON (not SSE):
```json
{ "error": "message" }
```
