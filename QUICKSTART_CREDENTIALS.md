# Quick Start: Dev Mode Credentials

## TL;DR

**Development**: Log in once, credentials cached in system keychain automatically. ✨

**Production**: Must log in each session (secure, no persistent storage). 🔒

## First Time Setup

1. **Run the app in dev mode:**
   ```bash
   npm run tauri dev
   ```

2. **You'll see a "🔧 DEV MODE" badge** in the top header

3. **Click "Generate" and authenticate when prompted**

4. **That's it!** Your credentials are now cached in your system keychain

5. **Next time you start the app**, it will use the cached credentials automatically

## Alternative: Skip Authentication in Dev

Want to work without AWS credentials? Just set an environment variable:

```bash
# Create .env file
echo "DEV_AWS_TOKEN=your-token-here" > src-tauri/.env

# Run app
npm run tauri dev
```

Or use mock mode (no credentials needed):
```bash
# Just run without any tokens
npm run tauri dev
# Will auto-fall back to mock data
```

## Logout

Click the **Logout** button in the header to clear cached credentials.

In dev mode, this clears both the session AND the keychain, so you'll need to re-authenticate next time.

## Building for Release

```bash
npm run tauri build
```

Release builds **never** use keychain storage. Users must authenticate each session for security.

## More Info

- Full guide: [docs/CREDENTIALS.md](../docs/CREDENTIALS.md)
- Implementation details: [DEV_RELEASE_STRATEGY.md](../DEV_RELEASE_STRATEGY.md)
