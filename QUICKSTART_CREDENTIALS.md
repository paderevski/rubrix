# Quick Start: Gateway Authentication

## TL;DR

Rubrix now uses a single authentication path in both dev and production: the Bedrock gateway.

## First Time Setup

1. Configure the gateway URL in `src-tauri/.env`:
   ```bash
   BEDROCK_GATEWAY_URL=https://your-api.example.com/generate
   ```

2. Run the app in dev mode:
   ```bash
   npm run tauri dev
   ```

3. Click **Generate** and authenticate when prompted.

4. Credentials are saved in app-local storage for automatic login on restart.

5. If you click **Logout**, saved credentials are cleared and you will re-authenticate next launch.

## Logout

Click the **Logout** button in the header to clear saved credentials.

## Building for Release

```bash
npm run tauri build
```

Release builds use the same gateway flow. Ensure `BEDROCK_GATEWAY_URL` is configured.

## More Info

- Gateway contract: [docs/BEDROCK_GATEWAY_CONTRACT.md](docs/BEDROCK_GATEWAY_CONTRACT.md)
- Implementation details: [DEV_RELEASE_STRATEGY.md](../DEV_RELEASE_STRATEGY.md)
