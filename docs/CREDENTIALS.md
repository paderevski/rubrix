# Development vs Release Credential Management

## Overview

Rubrix now supports automatic credential caching in development mode for a seamless developer experience. In release builds, authentication is always required for security.

## How It Works

### Development Mode (Debug Builds)
- ✅ **Auto-detected**: Automatically enabled when running `npm run tauri dev` or `cargo build`
- 🔐 **Keychain Storage**: Credentials are securely stored in your system keychain
- 🔄 **Persistent**: No need to re-authenticate on app restart
- 🎯 **Visual Indicator**: "🔧 DEV MODE" badge appears in the UI

### Release Mode (Production Builds)
- 🔒 **Authentication Required**: Must log in via Lambda each session
- 💾 **Memory-Only Cache**: Credentials cleared on app close
- 🚫 **No Keychain**: Prevents credential leakage in distributed builds

## Token Resolution Priority

### In Development Mode
1. **Provided Token** (from login form)
2. **System Keychain** (cached from previous session)
3. **`DEV_AWS_TOKEN`** (from `.env` file)
4. **`AWS_BEARER_TOKEN_BEDROCK`** (environment variable)
5. **Mock Mode** (fallback for testing without credentials)

### In Release Mode
1. **Provided Token** (from login form - required)
2. **`AWS_BEARER_TOKEN_BEDROCK`** (for CI/testing only)
3. **Error** (no mock mode in production)

## Setup for Development

### Option 1: One-Time Authentication (Recommended)
1. Run the app: `npm run tauri dev`
2. Log in when prompted
3. Token is automatically saved to your system keychain
4. Subsequent runs use the cached token silently

### Option 2: Environment Variable
1. Copy the example file:
   ```bash
   cp src-tauri/.env.example src-tauri/.env
   ```

2. Edit `src-tauri/.env` and set your token:
   ```bash
   DEV_AWS_TOKEN=your-token-here
   ```

3. Run the app - it will use this token and cache it to keychain

### Option 3: Mock Mode (No Credentials)
Simply run without setting any tokens:
```bash
npm run tauri dev
```
The app will fall back to mock data generation for testing UI without AWS access.

## Keychain Details

### Storage Location
- **macOS**: Keychain Access (service: `rubrix`, account: `aws_bedrock_token`)
- **Windows**: Windows Credential Manager
- **Linux**: Secret Service API (GNOME Keyring, KWallet, etc.)

### Manual Management
You can view/delete the stored credential:

**macOS**:
```bash
# View
security find-generic-password -s rubrix -a aws_bedrock_token

# Delete
security delete-generic-password -s rubrix -a aws_bedrock_token
```

**Windows**:
```powershell
# Open Credential Manager
rundll32.exe keymgr.dll,KRShowKeyMgr
# Search for "rubrix"
```

**Linux**:
```bash
# Using secret-tool
secret-tool search service rubrix
secret-tool clear service rubrix
```

## Logout Behavior

Click the **Logout** button in the header to clear credentials:

### Development Mode
- Clears in-memory cache
- **Clears keychain** (must re-authenticate next time)
- Hover tooltip: "Clears keychain cache"

### Release Mode
- Clears in-memory cache only
- No keychain to clear
- Hover tooltip: "Clears session"

## Security Considerations

### ✅ What's Safe
- ✅ Keychain storage in dev (OS-encrypted, user-specific)
- ✅ `.env` files (gitignored, never bundled)
- ✅ No credentials embedded in release binaries
- ✅ Clear visual indicator of dev mode

### ⚠️ Important Guidelines
- ⚠️ Never commit `.env` files with real tokens
- ⚠️ Never use `DEV_AWS_TOKEN` in shared/team environments
- ⚠️ Clear keychain before sharing your development machine
- ⚠️ Release builds never access keychain

## Testing

### Verify Development Mode
```bash
cd src-tauri
cargo build
./target/debug/rubrix
# Should show "🔧 DEV MODE" badge
```

### Verify Release Mode
```bash
cd src-tauri
cargo build --release
./target/release/rubrix
# Should NOT show dev mode badge
# Should require authentication
```

### Test Token Hierarchy
```bash
# 1. Test keychain (after one login)
npm run tauri dev
# Should auto-authenticate silently

# 2. Test .env override
echo "DEV_AWS_TOKEN=test-token" > src-tauri/.env
npm run tauri dev
# Should use .env token and save to keychain

# 3. Test mock mode
unset DEV_AWS_TOKEN
rm src-tauri/.env
# Clear keychain manually (see commands above)
npm run tauri dev
# Should fall back to mock generation
```

## Troubleshooting

### "No token available" Error
**In Dev Mode**:
- Check if token is in keychain (see manual management)
- Check if `src-tauri/.env` exists and has `DEV_AWS_TOKEN`
- Try authenticating once via the login form

**In Release Mode**:
- This is expected - click authenticate and log in

### Keychain Access Denied
Some Linux systems may require additional setup:
```bash
# Install libsecret
sudo apt-get install libsecret-1-0 libsecret-1-dev  # Debian/Ubuntu
sudo dnf install libsecret libsecret-devel          # Fedora
```

### Token Not Persisting
- Verify you're running a debug build (not release)
- Check terminal output for keychain errors
- Try logging out and back in to reset keychain entry

## CI/CD Integration

For continuous integration, use environment variables:

```yaml
# GitHub Actions example
- name: Run tests
  env:
    AWS_BEARER_TOKEN_BEDROCK: ${{ secrets.AWS_TOKEN }}
  run: npm run tauri test
```

**Never use keychain in CI** - it's automatically disabled in release builds.

## Migration from Old System

If you were using the old system where tokens were only in memory:

1. **No action required** - the first time you authenticate in dev mode, the token will be cached automatically
2. Your existing Lambda authentication still works exactly the same
3. You'll just need to authenticate less frequently now

## Environment Variables Reference

| Variable | Mode | Purpose | Storage |
|----------|------|---------|---------|
| `DEV_AWS_TOKEN` | Dev only | Skip auth in development | `.env` file |
| `AWS_BEARER_TOKEN_BEDROCK` | Both | Production token or CI override | Environment |
| `LAMBDA_URL` | Both | Authentication endpoint | Environment/`.env` |
| `RUBRIX_KNOWLEDGE_DIR` | Both | Knowledge base location | Environment/`.env` |

## Code References

- **Token Resolution**: [src-tauri/src/llm.rs](../src-tauri/src/llm.rs) - `get_api_token()`
- **Keychain Storage**: [src-tauri/src/config.rs](../src-tauri/src/config.rs) - `CredentialStore`
- **Mode Detection**: [src-tauri/src/config.rs](../src-tauri/src/config.rs) - `is_dev_mode()`
- **UI Integration**: [src/App.tsx](../src/App.tsx) - Dev mode badge and logout

## Support

For issues or questions:
1. Check terminal output for detailed logging
2. Verify `.gitignore` includes `.env` files
3. Test with mock mode to isolate credential issues
4. Review [DEV_RELEASE_STRATEGY.md](../DEV_RELEASE_STRATEGY.md) for implementation details
