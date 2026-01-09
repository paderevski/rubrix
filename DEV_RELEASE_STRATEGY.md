# Dev vs Release Settings Strategy

## Overview
This document outlines a secure, standard strategy for managing different settings between local development and production release builds, specifically for AWS Bedrock API credentials.

## Current State
- **Authentication Flow**: Lambda-based auth fetches AWS token via username/password
- **Dev Pain Point**: Must re-authenticate every app restart during development
- **Token Storage**: In-memory only (lost on restart)
- **Mock Mode**: Fallback when `AWS_BEARER_TOKEN_BEDROCK` is absent

## Recommended Strategy

### 1. Build Profile Detection
Use Rust's built-in `cfg!(debug_assertions)` which is:
- ✅ Automatically set by Cargo for dev builds
- ✅ Automatically unset for release builds
- ✅ No extra configuration needed

### 2. Secure Credential Storage Hierarchy

#### Development Mode (`debug_assertions`)
Priority order:
1. **Keychain/Credential Store** (persistent, secure)
   - macOS: Keychain Services
   - Windows: Credential Manager
   - Linux: Secret Service API (via keyring crate)
2. **`.env` file** (local only, gitignored)
3. **Environment variable** `DEV_AWS_TOKEN`
4. **Lambda auth** (fallback to production flow)
5. **Mock mode** (if all fail)

#### Release Mode
1. **Lambda authentication** (user must login)
2. **Environment variable** `AWS_BEARER_TOKEN_BEDROCK` (for CI/testing)
3. **Mock mode disabled** (fail with clear error)

### 3. Keychain Integration (Recommended)
Use `keyring` crate for cross-platform secure storage:

```toml
[dependencies]
keyring = "2.3"
```

Features:
- Persistent across app restarts
- OS-level encryption
- Per-user storage
- No plaintext secrets

### 4. Configuration File Structure

`.env` (gitignored, optional for dev):
```bash
# Development AWS token (never commit!)
DEV_AWS_TOKEN=your-token-here

# Lambda URL (both dev and release)
LAMBDA_URL=https://your-lambda-url

# Knowledge directory override
RUBRIX_KNOWLEDGE_DIR=imports/knowledge

# Dev mode flag (optional, auto-detected)
# DEV_MODE=true
```

### 5. Implementation Plan

#### Step 1: Add Dependencies
```toml
# src-tauri/Cargo.toml
[dependencies]
keyring = "2.3"
```

#### Step 2: Create Config Module
New file: `src-tauri/src/config.rs`
```rust
pub fn is_dev_mode() -> bool {
    cfg!(debug_assertions)
}

pub struct CredentialStore;

impl CredentialStore {
    pub fn save_token(token: &str) -> Result<(), String> {
        if !is_dev_mode() {
            return Err("Token persistence disabled in release mode".into());
        }

        let entry = keyring::Entry::new("rubrix", "aws_bedrock_token")
            .map_err(|e| format!("Keychain error: {}", e))?;
        entry.set_password(token)
            .map_err(|e| format!("Failed to save: {}", e))?;
        Ok(())
    }

    pub fn load_token() -> Option<String> {
        if !is_dev_mode() {
            return None;
        }

        keyring::Entry::new("rubrix", "aws_bedrock_token")
            .ok()?
            .get_password()
            .ok()
    }

    pub fn clear_token() -> Result<(), String> {
        let entry = keyring::Entry::new("rubrix", "aws_bedrock_token")
            .map_err(|e| format!("Keychain error: {}", e))?;
        entry.delete_password()
            .map_err(|e| format!("Failed to clear: {}", e))?;
        Ok(())
    }
}
```

#### Step 3: Update Token Resolution Logic
Modify `llm.rs`:
```rust
pub async fn get_api_token(provided_token: Option<String>) -> Result<String, String> {
    // 1. Use explicitly provided token (from auth command)
    if let Some(token) = provided_token {
        if config::is_dev_mode() {
            let _ = config::CredentialStore::save_token(&token);
        }
        return Ok(token);
    }

    // 2. DEV MODE: Try keychain first
    if config::is_dev_mode() {
        if let Some(token) = config::CredentialStore::load_token() {
            eprintln!("INFO: Using cached token from keychain");
            return Ok(token);
        }

        // 3. Try DEV_AWS_TOKEN env var
        if let Ok(token) = std::env::var("DEV_AWS_TOKEN") {
            eprintln!("INFO: Using DEV_AWS_TOKEN");
            let _ = config::CredentialStore::save_token(&token);
            return Ok(token);
        }
    }

    // 4. Try production env var
    if let Ok(token) = std::env::var("AWS_BEARER_TOKEN_BEDROCK") {
        eprintln!("INFO: Using AWS_BEARER_TOKEN_BEDROCK");
        return Ok(token);
    }

    // 5. No token available
    if config::is_dev_mode() {
        Err("No token available. Use DEV_AWS_TOKEN env var or authenticate.".into())
    } else {
        Err("Authentication required. No API token found.".into())
    }
}
```

#### Step 4: Update Authentication Command
```rust
#[tauri::command]
async fn authenticate(
    username: String,
    password: String,
    auth_cache: State<'_, AuthCache>,
) -> Result<(), String> {
    let api_key = auth::get_bedrock_api_key(&username, &password).await?;

    // Cache in memory (all modes)
    let mut cache = auth_cache.0.lock().unwrap();
    *cache = Some(api_key.clone());

    // Save to keychain (dev mode only)
    if config::is_dev_mode() {
        config::CredentialStore::save_token(&api_key)?;
    }

    Ok(())
}
```

#### Step 5: Add Logout Command (clears keychain)
```rust
#[tauri::command]
async fn logout(auth_cache: State<'_, AuthCache>) -> Result<(), String> {
    // Clear memory cache
    let mut cache = auth_cache.0.lock().unwrap();
    *cache = None;

    // Clear keychain
    if config::is_dev_mode() {
        config::CredentialStore::clear_token()?;
    }

    Ok(())
}
```

### 6. Security Considerations

#### ✅ What's Safe
- Keychain storage in dev (OS-encrypted)
- `.env` in dev (gitignored, local filesystem)
- Memory-only in release
- No secrets in binary

#### ⚠️  Important Rules
1. **Never bundle `.env` in release builds**
   - Add to `.taurignore` (Tauri v1) or exclude in bundle config
2. **Never commit tokens**
   - Keep `.env` in `.gitignore`
3. **Clear messaging**
   - Show dev mode indicator in UI
   - Warn when using cached credentials
4. **Optional logout**
   - Let users clear keychain if needed

### 7. UI Enhancements

Show dev mode indicator:
```tsx
// src/App.tsx
const [isDevMode, setIsDevMode] = useState(false);

useEffect(() => {
  invoke('is_dev_mode').then(setIsDevMode);
}, []);

// In UI header:
{isDevMode && (
  <div className="bg-yellow-100 text-yellow-800 px-3 py-1 rounded text-sm">
    🔧 DEV MODE
  </div>
)}
```

Optional logout button:
```tsx
{isAuthenticated && (
  <button onClick={handleLogout}>
    Logout {isDevMode && '(clear keychain)'}
  </button>
)}
```

### 8. Testing Strategy

#### Dev Build Testing
```bash
# Test keychain flow
cargo build
./target/debug/rubrix  # Should use cached token after first auth

# Test .env override
echo "DEV_AWS_TOKEN=test-token" > src-tauri/.env
cargo build
./target/debug/rubrix  # Should use .env token

# Test mock mode
unset DEV_AWS_TOKEN
cargo build
./target/debug/rubrix  # Should fall back to mock
```

#### Release Build Testing
```bash
# Verify no dev shortcuts
cargo build --release
./target/release/rubrix  # Should require authentication, no keychain

# Verify no bundled secrets
strings target/release/rubrix | grep -i "token\|secret"  # Should be empty
```

### 9. Migration Path

For existing users:
1. First run in new dev build prompts for auth
2. Token saved to keychain automatically
3. Subsequent runs use keychain silently
4. Can opt-out by clearing keychain or using `RUBRIX_NO_CACHE=1`

### 10. Alternative: Simple Env-Only Approach

If keychain integration is too complex, use env vars only:

```rust
pub fn get_api_token_simple(provided: Option<String>) -> Result<String, String> {
    provided
        .or_else(|| std::env::var("DEV_AWS_TOKEN").ok())
        .or_else(|| std::env::var("AWS_BEARER_TOKEN_BEDROCK").ok())
        .ok_or_else(|| {
            if cfg!(debug_assertions) {
                "Set DEV_AWS_TOKEN in .env for development".into()
            } else {
                "Authentication required".into()
            }
        })
}
```

**Pros**: Simple, no dependencies
**Cons**: Must set env var every session

## Summary

**Recommended Flow**:
- **Dev**: Auto-detect via `cfg!(debug_assertions)` → keychain → `.env` → auth
- **Release**: Auth required → env var (CI only) → fail
- **Security**: OS keychain encryption, no bundled secrets
- **UX**: One-time auth in dev, persistent until cleared

This balances convenience (no repeated logins in dev) with security (no credentials in release builds).
