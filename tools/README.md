# User Management Tools

Scripts for managing Catie user accounts in AWS SSM Parameter Store. The Bedrock gateway Lambda ([lambda_functions/bedrock_gateway](../lambda_functions/bedrock_gateway/lambda_handler.js)) validates logins against `/secrets/{user}/password_hash`.

## Parameters per user

- `/secrets/{user}/password_hash` - SHA256 hex digest of the password (used by the gateway for authentication)
- `/secrets/{user}/secret` - legacy Bedrock API key slot. The gateway calls Bedrock with its own server-side credentials, so this is not used at runtime; pass `NONE`.

Usernames are normalized (sanitized) the same way in the app, the scripts, and the gateway.

## Scripts

| Script | Purpose |
|---|---|
| `add_user.py <username> <password> <bedrock_api_key>` | Create or update a user (use `NONE` for the key) |
| `add_users_from_csv.sh [-n\|--dry-run]` | Bulk-add users from `catie-users.csv` (columns `Email Address`, `Password`); `-n` previews only |
| `verify_user.py <username> [password]` | Check a user exists; optionally verify a password matches |
| `list_users.py` | List users that have stored credentials |
| `old/` | Archived one-off debugging/migration scripts; not maintained |

`catie-users.csv` is git-ignored because it contains passwords. Never commit it.

## Example

```bash
python add_user.py alice "s3cret-pass" NONE
python verify_user.py alice "s3cret-pass"
python list_users.py
```

Then launch Catie, log in as `alice`, and generate a question to confirm end to end.

## Requirements

```bash
pip install boto3
aws configure   # or AWS_ACCESS_KEY_ID / AWS_SECRET_ACCESS_KEY / AWS_DEFAULT_REGION
```

The AWS identity needs SSM `PutParameter`, `GetParameter` and `GetParametersByPath` on `/secrets/*` (parameters are `SecureString`).

## Security notes

- Passwords are SHA256-hashed before storage, matching the client hashing in [auth.rs](../src-tauri/src/auth.rs).
- Never commit passwords, hashes, or keys.
- Restrict SSM access with IAM.
