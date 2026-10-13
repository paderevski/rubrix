# Bedrock Gateway Contract

## Overview
This document defines the request and response contract for the Bedrock gateway.
The gateway accepts user credentials and a prompt, calls Bedrock server-side, and
streams chunks back using Server-Sent Events (SSE). The response payload shape
must match the existing `llm-stream` event format.

## Endpoint
- Method: POST
- Path: the configured `BEDROCK_GATEWAY_URL` (for example `/generate`); the client POSTs to that URL as-is
- Content-Type: application/json
- Accept: text/event-stream

## Request JSON
```json
{
  "user": "string",
  "password_hash": "string",
  "prompt": "string"
}
```

### Fields
- `user`: Username.
- `password_hash`: SHA256 hex digest of the user's password.
- `prompt`: Fully rendered prompt text sent to Bedrock.
- The client currently sends only these three fields. The gateway should ignore unknown keys, so optional metadata can be added later.
- `user` is normalized by the gateway (characters outside `A-Za-z0-9._-` become `_`) before the password lookup, and the same normalization is used for usage tracking.

## Response (SSE)
- Content-Type: text/event-stream
- Each chunk uses the `data:` prefix and ends with a blank line.
- Payload matches the `llm-stream` shape: `{ "text": "...", "done": false }`.
- The gateway may add `remaining_tokens` (number) when usage tracking is enabled; the client forwards it to the UI.

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
- 401: Invalid credentials (password hash mismatch).
- 404: User not found.
- 429: Usage budget exceeded (the user's token budget is used up).
- 500: Internal error, including usage control not configured.

Errors are returned as JSON (not SSE) with the status repeated in the body:
```json
{ "error": "message", "status": 401 }
```

The client treats any `application/json` response as an error, even if the transport status is `200` (streaming Lambda responses cannot always set the status code).

## Usage control
The gateway tracks a per-user token budget in DynamoDB (`catieBedrockUsage`, default budget 1,000,000 tokens). Each streamed event may carry `remaining_tokens`, which the app displays after generation.
