# Visual Architecture: Dev vs Release Credentials

## System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                      RUBRIX APPLICATION                      │
│                                                              │
│  ┌───────────────┐                     ┌─────────────────┐  │
│  │   React UI    │                     │  Tauri Backend  │  │
│  │               │                     │                 │  │
│  │  [Dev Badge]  │◄────invoke()────────│  main.rs        │  │
│  │  [Auth State] │                     │   ├─ authenticate│  │
│  │  [Logout Btn] │                     │   ├─ check_auth │  │
│  │               │                     │   ├─ clear_auth │  │
│  │               │                     │   └─ is_dev_mode│  │
│  └───────────────┘                     │                 │  │
│                                        │  llm.rs         │  │
│                                        │   └─ get_api    │  │
│                                        │      _token()   │  │
│                                        │                 │  │
│                                        │  config.rs      │  │
│                                        │   └─ Credential │  │
│                                        │      Store      │  │
│                                        └────────┬────────┘  │
└─────────────────────────────────────────────────┼───────────┘
                                                  │
                   ┌──────────────────────────────┼────────────────────────┐
                   │                              │                        │
                   ▼                              ▼                        ▼
        ┌─────────────────┐          ┌──────────────────┐      ┌─────────────────┐
        │  OS KEYCHAIN    │          │   .env FILE      │      │  ENV VARIABLES  │
        │  (Dev Only)     │          │  (Dev Only)      │      │                 │
        ├─────────────────┤          ├──────────────────┤      ├─────────────────┤
        │ macOS: Keychain │          │ DEV_AWS_TOKEN    │      │ AWS_BEARER_     │
        │ Win: CredMgr    │          │ LAMBDA_URL       │      │ TOKEN_BEDROCK   │
        │ Linux: Secret   │          │ RUBRIX_          │      │                 │
        │        Service  │          │ KNOWLEDGE_DIR    │      │ (Both modes)    │
        └─────────────────┘          └──────────────────┘      └─────────────────┘
         Encrypted, Persistent       Gitignored, Local          System-wide
```

## Token Resolution Flow

### Development Mode (cfg!(debug_assertions) = true)

```
User Clicks "Generate"
         │
         ▼
┌────────────────────────┐
│  Frontend: App.tsx     │
│  invoke("generate")    │
└───────────┬────────────┘
            │
            ▼
┌────────────────────────┐
│  Backend: main.rs      │
│  generate_questions()  │
└───────────┬────────────┘
            │
            ▼
┌────────────────────────┐
│  llm.rs                │
│  generate(api_token)   │
└───────────┬────────────┘
            │
            ▼
┌────────────────────────┐      ┌──────────────┐
│  llm.rs                │      │ 1. Provided? │──Yes──> Use & save to keychain
│  get_api_token()       │──────►├──────────────┤
│                        │      │ 2. Keychain? │──Yes──> Use cached
│  Priority order:       │      ├──────────────┤
│  1. Provided token     │      │ 3. DEV_AWS_  │──Yes──> Use & save to keychain
│  2. Keychain           │      │    TOKEN?    │
│  3. DEV_AWS_TOKEN      │      ├──────────────┤
│  4. AWS_BEARER_TOKEN   │      │ 4. AWS_BEARER│──Yes──> Use
│  5. Mock mode          │      │    _TOKEN?   │
└────────────────────────┘      ├──────────────┤
                                │ 5. All fail? │──> MOCK MODE
                                └──────────────┘
```

### Release Mode (cfg!(debug_assertions) = false)

```
User Clicks "Generate"
         │
         ▼
┌────────────────────────┐
│  Frontend: App.tsx     │
│  (No Dev Badge)        │
│  invoke("generate")    │
└───────────┬────────────┘
            │
            ▼
┌────────────────────────┐
│  Backend: main.rs      │
│  generate_questions()  │
└───────────┬────────────┘
            │
            ▼
┌────────────────────────┐
│  llm.rs                │
│  generate(api_token)   │
└───────────┬────────────┘
            │
            ▼
┌────────────────────────┐      ┌──────────────┐
│  llm.rs                │      │ 1. Provided? │──Yes──> Use (memory only)
│  get_api_token()       │──────►├──────────────┤
│                        │      │ 2. AWS_BEARER│──Yes──> Use
│  Keychain DISABLED     │      │    _TOKEN?   │
│  DEV_AWS_TOKEN ignored │      ├──────────────┤
│  Mock mode DISABLED    │      │ 3. All fail? │──> ERROR
└────────────────────────┘      └──────────────┘
                                      │
                                      ▼
                               Show login modal
```

## Authentication Flow

### First Time Login (Dev Mode)

```
User Opens App
     │
     ▼
check_auth() → No token found
     │
     ▼
Show Login Modal
     │
     ▼
User Enters Credentials
     │
     ▼
┌────────────────────────┐
│  authenticate()        │
│  1. Call Lambda        │──────> AWS Lambda Auth
│  2. Get token          │◄────── Returns token
│  3. Save to memory     │
│  4. Save to keychain   │───────> macOS Keychain
└────────────────────────┘        (Encrypted)
     │
     ▼
Close Login Modal
     │
     ▼
Generate Questions (token auto-used)
```

### Subsequent Runs (Dev Mode)

```
User Opens App
     │
     ▼
check_auth()
     │
     ├──> Check memory → Empty
     │
     └──> Check keychain → Found! ✓
          │
          ▼
     Auto-authenticated
     (No login modal)
          │
          ▼
     Ready to generate
```

### Logout (Dev Mode)

```
User Clicks "Logout"
     │
     ▼
┌────────────────────────┐
│  clear_auth()          │
│  1. Clear memory       │
│  2. Clear keychain     │───────> Delete from keychain
└────────────────────────┘
     │
     ▼
Next run requires login
```

## File Interaction Diagram

```
Development Workflow:
═══════════════════════

Developer          Git Repo         Local FS         System Keychain
    │                  │                │                    │
    │  git clone       │                │                    │
    ├─────────────────>│                │                    │
    │                  │                │                    │
    │  cp .env.example │                │                    │
    ├────────────────────────────────> │                    │
    │                  │   .env         │                    │
    │                  │  (gitignored)  │                    │
    │                  │                │                    │
    │  npm run tauri dev               │                    │
    ├────────────────────────────────> │                    │
    │                  │    reads       │                    │
    │                  │   DEV_AWS_TOKEN│                    │
    │                  │                │                    │
    │  First login     │                │    saves token    │
    ├────────────────────────────────────────────────────> │
    │                  │                │    (encrypted)    │
    │                  │                │                    │
    │  Restart app     │                │    reads token    │
    │                  │                │◄───────────────────│
    │  Auto-authenticated!              │                    │
    │                  │                │                    │
    │  git commit      │                │                    │
    │  (no .env!)      │                │                    │
    ├─────────────────>│                │                    │
    │                  │  ✓ .gitignore  │                    │
    │                  │    protects    │                    │


Release Workflow:
════════════════

Builder          Git Repo         Build Output     User's Machine
    │                │                  │                 │
    │  git clone     │                  │                 │
    ├───────────────>│                  │                 │
    │                │                  │                 │
    │  cargo build   │                  │                 │
    │    --release   │                  │                 │
    ├───────────────────────────────────>│                 │
    │                │   Binary         │                 │
    │                │   (no secrets)   │                 │
    │                │                  │                 │
    │  Distribute    │                  │                 │
    ├──────────────────────────────────────────────────> │
    │                │                  │  User installs  │
    │                │                  │  User runs app  │
    │                │                  │  Must login     │
    │                │                  │  (secure!)      │
```

## Component Responsibilities

```
┌──────────────────────────────────────────────────────────┐
│                        config.rs                          │
│  ┌──────────────────────────────────────────────────┐   │
│  │  is_dev_mode() → bool                            │   │
│  │    Returns cfg!(debug_assertions)                │   │
│  └──────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  CredentialStore                                 │   │
│  │    save_token(token)   → Result<(), String>     │   │
│  │    load_token()        → Option<String>         │   │
│  │    clear_token()       → Result<(), String>     │   │
│  │                                                  │   │
│  │  ⚠️  All methods check is_dev_mode() first      │   │
│  └──────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────┐
│                         llm.rs                            │
│  ┌──────────────────────────────────────────────────┐   │
│  │  get_api_token(provided: Option<String>)        │   │
│  │                → Result<String, String>          │   │
│  │                                                  │   │
│  │  1. If provided → use & maybe save to keychain  │   │
│  │  2. If dev → try keychain                       │   │
│  │  3. If dev → try DEV_AWS_TOKEN                  │   │
│  │  4. Try AWS_BEARER_TOKEN_BEDROCK                │   │
│  │  5. Fail or mock (dev only)                     │   │
│  └──────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  generate(prompt, app_handle, api_token)        │   │
│  │                → Result<String, String>          │   │
│  │                                                  │   │
│  │  Calls get_api_token(), then streams from AWS   │   │
│  └──────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────┐
│                        main.rs                            │
│  ┌──────────────────────────────────────────────────┐   │
│  │  authenticate(username, password)                │   │
│  │    1. Call Lambda                                │   │
│  │    2. Save to memory (AppState)                  │   │
│  │    3. If dev → save to keychain                  │   │
│  └──────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  check_auth()                                    │   │
│  │    1. Check memory                               │   │
│  │    2. If dev → check keychain                    │   │
│  └──────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  clear_auth()                                    │   │
│  │    1. Clear memory                               │   │
│  │    2. If dev → clear keychain                    │   │
│  └──────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  is_dev_mode()                                   │   │
│  │    Return config::is_dev_mode()                  │   │
│  └──────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────┐
│                        App.tsx                            │
│  ┌──────────────────────────────────────────────────┐   │
│  │  State: isDevMode, isAuthenticated               │   │
│  └──────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  checkDevMode() → setIsDevMode                   │   │
│  │  checkAuthentication() → setIsAuthenticated      │   │
│  │  handleLogin() → authenticate() → check_auth()   │   │
│  │  handleLogout() → clear_auth()                   │   │
│  └──────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  UI: Dev badge, auth indicator, logout button    │   │
│  └──────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────┘
```

## Security Boundaries

```
┌─────────────────────────────────────────────────────────┐
│                    TRUSTED ZONE                         │
│  ┌────────────────────────────────────────────────┐    │
│  │  Developer's Machine (Dev Mode)                │    │
│  │  • OS Keychain (encrypted)                     │    │
│  │  • .env file (gitignored, local only)          │    │
│  │  • Memory cache (current session)              │    │
│  └────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│                   UNTRUSTED ZONE                        │
│  ┌────────────────────────────────────────────────┐    │
│  │  Distributed Binaries (Release Mode)           │    │
│  │  • No keychain access                          │    │
│  │  • No .env bundled                             │    │
│  │  • Memory cache only (volatile)                │    │
│  │  • Authentication required                     │    │
│  └────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│                   VERSION CONTROL                       │
│  ┌────────────────────────────────────────────────┐    │
│  │  Git Repository                                │    │
│  │  ✓ Source code                                 │    │
│  │  ✓ .env.example (template)                     │    │
│  │  ✓ .gitignore (includes .env)                  │    │
│  │  ✗ .env (blocked)                              │    │
│  │  ✗ Tokens (never committed)                    │    │
│  └────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```
