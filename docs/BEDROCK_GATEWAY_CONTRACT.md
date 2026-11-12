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
  "password": "string",
  "prompt": "string",
  "question_type": "multiple_choice | frq"
}
```

### Fields
- `user`: Username.
- `password`: The user's password, sent over HTTPS. The gateway checks it against the user's scrypt record in SSM.
- `prompt`: Fully rendered prompt text sent to the model.
- `question_type`: Optional. `multiple_choice` or `frq`; FRQ requests run at a higher reasoning effort (`ANTHROPIC_EFFORT_FRQ`). When omitted, the standard settings are used.
- The gateway ignores unknown keys, so optional metadata can be added later.
- `user` is normalized by the gateway (characters outside `A-Za-z0-9._-` become `_`) before the password lookup, and the same normalization is used for usage tracking.

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
- The gateway may add `remaining_usd` (number, dollars) and the legacy `remaining_tokens` (number) when usage tracking is enabled; the client forwards both to the UI, which shows dollars when present.

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
- 429: Usage budget exceeded (the user's dollar budget is used up).
- 500: Internal error, including usage control not configured.

Errors are returned as JSON (not SSE) with the status repeated in the body:
```json
{ "error": "message", "status": 401 }
```

The client treats any `application/json` response as an error, even if the transport status is `200` (streaming Lambda responses cannot always set the status code).

## Usage control
The gateway tracks a per-user dollar budget in DynamoDB (`catieBedrockUsage`, summary item `record_id = "summary"`). Amounts are nano-dollars (1e-9 USD): `budget_nanousd` (default `USAGE_DEFAULT_BUDGET_USD`, $5.00) and `spent_nanousd`; remaining = budget - spent, and requests are refused with 429 once it reaches 0.

Each request's cost is computed from the API's reported usage (input, cache write, cache read and output tokens, thinking included) and the per-model price table `MODEL_PRICES_NANOUSD` in the gateway; Claude Haiku 5.5 switches to its higher prices when a prompt exceeds 100,000 tokens. Spending is added atomically (`ADD spent_nanousd`), so overlapping requests cannot overwrite each other. Each per-request record stores `model_id` and `cost_nanousd`. A model missing from the price table is logged as `usage_unpriced_model` and not charged, so update the table whenever the model or prices change.

The legacy token fields (`budget_tokens`, `remaining_tokens`, `total_tokens`) are still maintained for older app versions but are no longer enforced. Manage budgets with `tools/user_budget.py`.
