# Architecture Diagram

## Gateway-Only Runtime

```mermaid
flowchart LR
    UI[React UI\nApp.tsx + LoginModal] --> TAURI[Tauri Commands\nmain.rs]
    TAURI --> AUTH[auth.rs\nSHA256 hash]
    TAURI --> LLM[llm.rs\nGateway streaming client]
    TAURI --> STORE[(App local data\nauth/credentials.json)]
    LLM --> GATEWAY[Bedrock Gateway API]
    GATEWAY --> BEDROCK[AWS Bedrock]
```

## Authentication Sequence

```mermaid
sequenceDiagram
    participant U as User
    participant FE as Frontend
    participant BE as Tauri Backend
    participant GW as Gateway

    U->>FE: Submit username/password
    FE->>BE: invoke("authenticate")
    BE->>BE: hash_password(password)
    BE->>GW: validate credentials
    GW-->>BE: success/failure
    alt success
        BE->>BE: save credentials.json
        BE-->>FE: ok
    else failure
        BE-->>FE: error
    end
```

## Generation Sequence

```mermaid
sequenceDiagram
    participant FE as Frontend
    participant BE as Tauri Backend
    participant GW as Gateway
    participant BR as Bedrock

    FE->>BE: invoke("generate_questions")
    BE->>BE: read cached credentials
    BE->>GW: POST { user, password_hash, prompt }
    GW->>BR: Invoke model
    BR-->>GW: Stream chunks
    GW-->>BE: SSE stream
    BE-->>FE: emit_all("llm-stream", { text, done, remaining_tokens? })
```

## Startup Behavior

1. Frontend calls `auto_authenticate`.
2. Backend loads saved credentials (if present).
3. Backend re-validates credentials against gateway.
4. App enters authenticated state only on successful validation.

## Configuration

- `BEDROCK_GATEWAY_URL` is required for runtime auth and generation.
- If missing, backend returns: "Gateway mode is required, but BEDROCK_GATEWAY_URL is not configured."

## Notes

- Direct Bedrock token modes are removed from active runtime flow.
- Gateway JSON error payloads are treated as errors even when transport status is `200` in stream mode.
- Streaming event payload remains stable as `{ text, done, remaining_tokens? }`.
