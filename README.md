# Catie - Test Item Engine

An AI-powered multiple-choice question generator for AP Computer Science A and Calculus, built with Tauri + React. The app ships as **Catie**; this repository is named `rubrix`.

![Catie Screenshot](docs/screenshot.png)

## Features

- **Subjects and topics** - AP Computer Science A and Calculus, with topics and subtopics from editable knowledge banks
- **Generation** - difficulty (easy/medium/hard), 1-8 questions per run, optional instructions, streaming preview
- **Auth via gateway** - username/password login; generation runs through an AWS Bedrock gateway (no AWS keys on the client)
- **Editing and regeneration** - edit any question, regenerate one or all questions
- **Rendering** - Markdown, KaTeX math, syntax-highlighted code, tables
- **Documents** - tabs, save/open sessions (JSON), open recent
- **Export** - QTI (`.imscc`, Schoology/Canvas), Markdown, Word (`.docx`)
- **Bank Editor** - edit per-subject question banks used as few-shot examples
- **Bug reporting** - Help -> Submit Bug files a GitHub issue through an AWS intake Lambda

User documentation lives in [docs/](docs/index.md).

## Prerequisites

- [Node.js](https://nodejs.org/) 20+
- [Rust](https://rustup.rs/) (stable)
- [Tauri v1 prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

## Quick Start

```bash
npm install
npm run tauri dev      # full app
npm run dev            # frontend only
npm run tauri build    # release build
cd src-tauri && cargo build   # backend only
```

## Configuration

Catie is gateway-only. Set the following in `src-tauri/.env` (git-ignored) or bake them in at build time:

| Variable | Required | Purpose |
|---|---|---|
| `BEDROCK_GATEWAY_URL` | Yes | Gateway endpoint for auth and streaming generation |
| `RUBRIX_KNOWLEDGE_DIR` | No | Override the knowledge base directory (absolute or relative) |
| `BUG_REPORT_URL` | No | Bug intake endpoint (required for Submit Bug) |
| `BUG_REPORT_API_KEY` | No | Sent as `x-api-key` |
| `BUG_REPORT_BEARER_TOKEN` | No | Sent as a bearer token |

Without `BEDROCK_GATEWAY_URL`, login and generation fail with a configuration error. There is no mock mode.

On first Generate, Catie asks for a username and password. Credentials are stored in the app's local data directory (`auth/credentials.json`) for automatic login; Logout clears them. Details: [docs/CREDENTIALS.md](docs/CREDENTIALS.md).

## Documentation for developers

- [Architecture](docs/ARCHITECTURE_DIAGRAM.md)
- [Gateway contract](docs/BEDROCK_GATEWAY_CONTRACT.md)
- [Credentials and auth](docs/CREDENTIALS.md)
- Bug reporting: [workflow](docs/BUG_REPORT_WORKFLOW.md), [deployment](docs/BUG_REPORT_DEPLOYMENT.md), [schema](docs/BUG_REPORT_SCHEMA.json)
- User management: [tools/README.md](tools/README.md)
- [LaTeX in QTI export](LATEX_EXAMPLE.md)
- [Changelog](CHANGELOG.md)

## Project layout

```
rubrix/
├── src/                      # React frontend
│   ├── App.tsx               # Main controller (tabs, generation, sessions, auth)
│   ├── types.ts              # Types shared with the backend
│   └── components/           # QuestionCard, EditModal, Sidebar, BankEditor,
│                             # LoginModal, PreferencesModal, SubmitBugModal, ...
├── src-tauri/src/            # Rust backend
│   ├── main.rs               # Tauri commands, menus, exports, bug reports
│   ├── llm.rs                # Gateway streaming client (emits `llm-stream`)
│   ├── auth.rs               # SHA256 password hashing
│   ├── prompts.rs            # Prompt building and response parsing
│   ├── knowledge.rs          # Knowledge base loader (embedded + override dir)
│   └── qti.rs                # QTI export
├── imports/knowledge/        # Bundled subject knowledge (prompts, banks, topic files)
├── lambda_functions/         # bedrock_gateway, bug_intake, secret_store
├── terraform/bug_intake/     # Infrastructure for bug intake
├── tools/                    # User management scripts
└── docs/                     # GitHub Pages site and technical docs
```

### Knowledge base

Each subject has a folder in `imports/knowledge/<Subject>/` with `prompt.txt`, `regeneration-prompt.txt`, `question-schema.json`, `question-bank.json`, and optional topic text files. Prompt placeholders include `{topics}`, `{difficulty}`, `{count}`, `{examples}`, and `{user_instructions}`. Release builds embed these files; if `RUBRIX_KNOWLEDGE_DIR` is set (or a user copy exists in app data), edits there take precedence and the Bank Editor saves to that location. Keep topic IDs stable because `question-bank.json` references them.

## Exporting to Schoology

1. File -> Export QTI, save as `questions.imscc`
2. In Schoology: Resources -> Add Resources -> Import File
3. Select the `.imscc`; the questions appear as a Question Bank

## Releases and versioning

- `npm version <x>` bumps `package.json` and syncs [src-tauri/tauri.conf.json](src-tauri/tauri.conf.json).
- Pushing a `v*` tag triggers [.github/workflows/release.yml](.github/workflows/release.yml), which builds macOS/Linux/Windows installers into a draft GitHub release. Repository secrets: `BEDROCK_GATEWAY_URL`, `BUG_REPORT_URL`, `BUG_REPORT_API_KEY`.
- Builds are unsigned; see [docs/release-build-readme.md](docs/release-build-readme.md) for install steps.

## Tech stack

React 18, TypeScript, Tailwind CSS, Radix UI, ReactMarkdown + KaTeX; Rust + Tauri v1; AWS Bedrock (`openai.gpt-oss-120b-1:0`) behind a Lambda gateway; IMS QTI 1.2 export.

## License

MIT
