# Lambda Secret Store Management Tools

Scripts for managing user/password combinations in AWS SSM Parameter Store for Lambda authentication.

## Overview

The Lambda authentication system requires TWO SSM parameters per user:
1. `/secrets/{user}/password_hash` - SHA256 hash of the password (for validation)
2. `/secrets/{user}/secret` - Bedrock API key (returned after successful auth)

## Scripts

### add_user.py
Add or update a user with password and Bedrock API key.

```bash
python add_user.py <username> <password> <bedrock_api_key>
```

**Example:**
```bash
python add_user.py alice mypassword123 "ABSKQmVkcm9ja0FQSUtleS0..."
```

### verify_user.py
Verify a user exists and optionally check if a password is correct.

```bash
python verify_user.py <username> [password]
```

**Examples:**
```bash
# Check if user exists and show stored hash
python verify_user.py alice

# Verify password matches
python verify_user.py alice "mypassword123"
```

### list_users.py
List all configured users and their status.

```bash
python list_users.py
```

### Legacy Scripts

- `store_secret.py` - Store Bedrock API key only (use `add_user.py` instead)
- `get_secret.py` - Retrieve Bedrock API key for a user

## Workflow

### 1. Add a new user
```bash
python add_user.py bob "secretpass123" "ABSKQmVkcm9ja0FQSUtleS0..."
```

### 2. Verify it worked
```bash
python verify_user.py bob "secretpass123"
```

### 3. List all users
```bash
python list_users.py
```

### 4. Test in Rubrix
1. Set `LAMBDA_URL` in `src-tauri/.env`
2. Run `npm run tauri dev`
3. Try generating questions
4. Login with username `bob` and password `secretpass123`
5. Should receive Bedrock API key and generate questions

## Requirements

```bash
pip install boto3
```

Configure AWS credentials:
```bash
aws configure
# or set AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_DEFAULT_REGION
```

## Security Notes

- Passwords are hashed with SHA256 before storage
- All parameters stored as `SecureString` type
- Never commit passwords or API keys to git
- Use IAM policies to restrict SSM access
