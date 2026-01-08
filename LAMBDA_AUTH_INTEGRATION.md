# Lambda Authentication Integration

## Overview
Rubrix now retrieves AWS Bedrock API keys from a Lambda secret store, requiring user authentication before question generation.

## Flow
1. User attempts to generate questions
2. Backend checks for cached API key in memory
3. If no key found, login modal appears
4. User enters credentials → Lambda validates and returns Bedrock API key
5. Key cached in app state for entire session
6. Generate questions with authenticated API key

## Changes Made

### Backend (`src-tauri/`)

#### New Files
- **`src/auth.rs`**: Lambda authentication client
  - `get_bedrock_api_key(username, password)` - Calls Lambda Function URL
  - SHA256 password hashing client-side
  - Returns Bedrock API key on success

#### Modified Files
- **`src/main.rs`**:
  - Added `mod auth;`
  - Extended `AppState` with `api_key: Mutex<Option<String>>`
  - New commands:
    - `authenticate(username, password)` - Login and cache API key
    - `check_auth()` - Returns if key is cached
    - `clear_auth()` - Logout (clears cache)
  - Updated `generate_questions` and `regenerate_question` to pass cached key to LLM

- **`src/llm.rs`**:
  - `generate()` now accepts `api_token: Option<String>` parameter
  - Uses provided token if available, falls back to `AWS_BEARER_TOKEN_BEDROCK` env var
  - Maintains mock mode when no token present

- **`Cargo.toml`**:
  - Added `sha2 = "0.10"` dependency

### Frontend (`src/`)

#### New Files
- **`components/LoginModal.tsx`**: Authentication UI
  - Username/password form
  - Error display
  - Loading states
  - Security note about client-side hashing

#### Modified Files
- **`App.tsx`**:
  - Added `loginModalOpen`, `authError`, `isAuthenticated` state
  - `checkAuthentication()` on mount
  - `handleLogin()` - Calls `authenticate` command
  - `handleLogout()` - Calls `clear_auth` command
  - `handleGenerate()` checks auth before generating:
    - If not authenticated → shows login modal
    - Catches auth-related errors and prompts login
  - Added auth indicator badge in header (🔐 Authenticated + Logout button)
  - Renders `<LoginModal>` at bottom

## Environment Setup

Create `.env` in `src-tauri/` (or workspace root):
```env
LAMBDA_URL=https://your-lambda-function-url.lambda-url.us-west-2.on.aws/
```

**Important**: The app will still run without `LAMBDA_URL` using mock responses for development, but real API key retrieval requires the Lambda URL to be configured.

## User Workflow

### First Use
1. Launch app
2. Select subject/topics, click "Generate Questions"
3. Login modal appears
4. Enter credentials → key retrieved and cached
5. Questions generate normally

### Subsequent Generations
- Key remains cached in memory
- No re-authentication needed during session

### Logout
- Click "Logout" in header badge
- Clears cached key
- Next generation will require re-authentication

## Security Notes
- Passwords are SHA256-hashed **before** transmission
- API key stored in memory only (never persisted to disk)
- Key cleared on app restart
- Lambda validates credentials and returns secret

## Testing

### Mock Mode (No Lambda)
- Omit `LAMBDA_URL` env var
- App uses deterministic mock streaming
- Good for UI testing without backend

### With Lambda
1. Set `LAMBDA_URL` in `.env`
2. Run: `npm run tauri dev`
3. Try generating → login prompt should appear
4. Use valid Lambda credentials
5. Verify key caching (no re-prompt on subsequent generations)

## Lambda Requirements

Your Lambda function should:
- Accept POST with JSON: `{"user": string, "password_hash": string}`
- Return 200 with: `{"secret": "<BEDROCK_API_KEY>"}`
- Return 401 for invalid password
- Return 404 for unknown user
- Password hash is SHA256 hex string

Example response:
```json
{
  "secret": "Bearer_YOUR_BEDROCK_TOKEN_HERE"
}
```

## Build
```bash
# Development
npm run tauri dev

# Release
npm run tauri build
```
