# Gateway Credential Management

## Overview

Rubrix uses a single authentication model in both development and release builds:

- The desktop app authenticates users against the Bedrock gateway.
- The app stores username/password locally for automatic login on restart.
- Generation requests are sent to the gateway with `user` and `password_hash`.

Direct Bedrock token modes are not used by the app runtime.

## Required Configuration

Set `BEDROCK_GATEWAY_URL` in `src-tauri/.env` (or bake it at build time):

```bash
BEDROCK_GATEWAY_URL=https://your-api.example.com/generate
```

If this value is missing, authentication and generation will fail with a configuration error.

## Authentication Flow

1. User opens login modal and enters username/password.
2. Backend hashes password with SHA256.
3. Backend validates credentials against gateway.
4. On success, credentials are written to app-local storage.
5. On app restart, `auto_authenticate` re-validates stored credentials.

## Storage Behavior

Credentials are stored in app-local data at:

- `auth/credentials.json` under Tauri's app local data directory.

Logout clears:

- In-memory credentials.
- Saved `credentials.json` file.

## Error Mapping

Frontend auth messaging maps common backend/gateway errors to user-friendly text:

- `401` or "invalid password" -> password incorrect.
- `404` or "user not found" -> unknown user.
- Network/connectivity failures -> auth server unreachable.
- Missing `BEDROCK_GATEWAY_URL` -> auth service not configured.

## Security Notes

- Passwords are hashed client-side before transmission for gateway requests.
- Keep gateway transport over HTTPS.
- Restrict local machine access because saved credentials are local to the user profile.
- Rotate gateway-side credentials regularly.

## Environment Variables

| Variable | Required | Purpose |
|---|---|---|
| `BEDROCK_GATEWAY_URL` | Yes | Gateway endpoint for auth and streaming generation |
| `BEDROCK_MODEL_ID` | No | Default Bedrock model used by gateway for non-FRQ requests |
| `BEDROCK_MODEL_ID_FRQ` | No | FRQ Bedrock model override (used when `question_type=frq`) |
| `BEDROCK_FRQ_MODEL_ID` | No | Legacy alias for `BEDROCK_MODEL_ID_FRQ` |
| `RUBRIX_KNOWLEDGE_DIR` | No | Override knowledge base directory |
| `BUG_REPORT_URL` | No | Bug report intake endpoint |
| `BUG_REPORT_API_KEY` | No | Optional API key header for bug reports |

## Related Docs

- [Gateway Contract](BEDROCK_GATEWAY_CONTRACT.md)
- [Architecture Diagram](ARCHITECTURE_DIAGRAM.md)
- [Quickstart Credentials](../QUICKSTART_CREDENTIALS.md)
