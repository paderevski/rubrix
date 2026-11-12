# User Management Tools

Scripts for managing Catie user accounts in AWS SSM Parameter Store. The Bedrock gateway Lambda ([lambda_functions/bedrock_gateway](../lambda_functions/bedrock_gateway/lambda_handler.js)) validates logins against `/secrets/{user}/password_hash`.

## Parameters per user

- `/secrets/{user}/password_hash` - scrypt record of the password: `scrypt$N$r$p$<salt>$<hash>` (format and helpers in [password_hashing.py](password_hashing.py)). The app sends the password over HTTPS and the gateway checks it against this record, so reading the parameter is not enough to log in.
- `/secrets/{user}/secret` - legacy Bedrock API key slot. The gateway calls Bedrock with its own server-side credentials, so this is not used at runtime; pass `NONE`.

Usernames are normalized (sanitized) the same way in the app, the scripts, and the gateway.

## Scripts

| Script | Purpose |
|---|---|
| `add_user.py <username> <password> <bedrock_api_key>` | Create or update a user (use `NONE` for the key) |
| `add_users_from_csv.sh [-n\|--dry-run]` | Bulk-add users from `catie-users.csv` (columns `Email Address`, `Password`); `-n` previews only |
| `verify_user.py <username> [password]` | Check a user exists; optionally verify a password matches |
| `list_users.py` | List users that have stored credentials |
| `user_budget.py list \| get <user> \| set <user> <usd> \| add <user> <usd> \| reset <user>` | View/update dollar budgets in DynamoDB `catieBedrockUsage` |
| `delete_user.py <username> [--purge-usage] [-y]` | Delete a user's SSM credentials (blocks login); `--purge-usage` also deletes their DynamoDB budget/usage records. Prompts for confirmation unless `-y` |
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
pip install --upgrade boto3   # recent version required for `aws login` sessions
```

The AWS identity needs SSM `PutParameter`, `GetParameter` and `GetParametersByPath` on `/secrets/*` (parameters are `SecureString`), plus `ssm:DescribeParameters` for `list_users.py`. If a customer-managed KMS key encrypts the parameters, it also needs `kms:Decrypt`/`kms:Encrypt` on that key.

## AWS authentication

The scripts use the standard AWS credential chain with the region pinned to `us-east-1` (where the gateway reads parameters; see `REGION` in [password_hashing.py](password_hashing.py)).

One-time setup (browser-based login for an IAM user, no SSO start URL needed):

```bash
aws login --profile patrick
aws configure set region us-east-1 --profile patrick
aws sts get-caller-identity --profile patrick   # confirm it works
```

Each time you work in this directory:

```bash
aws login --profile patrick     # only when the session has expired
export AWS_PROFILE=patrick      # once per terminal (or put in ~/.zshrc)
python list_users.py
```

What is saved: the profile (region and `login_session` ARN) is in `~/.aws/config`, and the cached login session is under `~/.aws/login/cache/` (sensitive). `AWS_PROFILE` is not saved unless you add it to a shell rc file.

## Troubleshooting

| Error | Fix |
|---|---|
| `Unable to locate credentials` | Session expired or `AWS_PROFILE` unset: run `aws login --profile <name>` and `export AWS_PROFILE=<name>`. If the AWS CLI works but Python does not, upgrade boto3 (`pip install --upgrade boto3`) or use `env/bin/python`. |
| `NoRegionError` | `aws configure set region us-east-1 --profile <name>` (the gateway uses `us-east-1`). |
| `AccessDeniedException` | The IAM identity lacks the SSM/KMS permissions listed above. |

## Security notes

- Passwords are stored as scrypt records (see [password_hashing.py](password_hashing.py)); the app sends the password itself over HTTPS.
- Never commit passwords, hashes, or keys.
- Restrict SSM access with IAM.
