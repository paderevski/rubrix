# TODO

## Bedrock Gateway Migration (Plan)

### Goal
Move from client-side Bedrock calls to a server-side gateway so the app sends user/pass + prompt to a gateway, and the gateway calls Bedrock. Keep SSE streaming and existing `llm-stream` event shape.

### Phase 0: Contract
- Endpoint: `POST /generate`
- Request JSON: `{ "user": "...", "password_hash": "...", "prompt": "...", "meta": { ... } }`
- Response: `text/event-stream` SSE chunks with `data: {"text":"...","done":false}` and a final `done=true` event
- Status: Drafted in docs/BEDROCK_GATEWAY_CONTRACT.md

### Phase 1: Lambda Gateway (Node)
- Location: lambda_functions/bedrock_gateway
- Steps:
   - Validate user: read `/secrets/{user}/password_hash` from SSM with `WithDecryption=True`
   - Compare with `password_hash`
   - Call Bedrock with server-side credentials
   - Stream SSE chunks back to client
- Use Function URL or API Gateway HTTP API with response streaming enabled
- Status: Gateway running with streaming (Node handler)

### Phase 2: Client Swap (Tauri)
- Replace direct Bedrock calls in `src-tauri/src/llm.rs` with gateway calls
- Preserve `llm-stream` event payload shape
- Status: Gateway path wired, build-time `BEDROCK_GATEWAY_URL` required
- Build env: add `BEDROCK_GATEWAY_URL` to GitHub Actions secrets/env for release builds

### Phase 3: Usage Control (DynamoDB)
- Table: `rubrix_usage`
- Keys: partition `user`, sort `date` (YYYY-MM-DD)
- Fields: `requests`, `tokens_used`, `last_request_at`
- Flow per request:
   - Read usage row
   - Enforce limits (requests/day, tokens/day)
   - Atomic increment
- Tomorrow: implement quotas + rate limiting in gateway, keep logs extensible (CloudWatch -> S3/Athena later)

### Phase 4: Security/ops
- CloudWatch logging for audit/usage
- API Gateway throttling and IP rate limits
- Remove client Bedrock token usage entirely

## Git Workflow Improvement

### Current workflow:
- Using `release` branch as a deployment trigger
- Manually syncing `release` to `main` with `git reset --hard main` + force push
- Works but unconventional

### Recommended improvement: Switch to tag-based releases

**Benefits:**
- More standard Git practice
- Tags create permanent, immutable snapshots
- Cleaner history
- No force-pushing needed

**Steps to implement:**

1. **Update GitHub Actions workflow** (`.github/workflows/release.yml` or similar):
   ```yaml
   on:
     push:
       tags:
         - 'v*'  # Triggers on tags like v0.6.1, v0.7.0, etc.
   ```

2. **Create releases with tags instead of branch:**
   ```bash
   git checkout main
   git tag v0.6.2
   git push origin v0.6.2
   ```

3. **Optional: Use GitHub CLI for full releases:**
   ```bash
   gh release create v0.6.2 --generate-notes
   ```

4. **Once working, delete release branch:**
   ```bash
   git branch -d release
   git push origin --delete release
   ```

### Alternative: Keep current workflow
If you prefer to keep the current setup:
- It's fine for solo/small projects
- Just remember: `git reset --hard main` then `git push --force-with-lease origin release`
- Accept that it's a simple "build trigger branch" pattern
