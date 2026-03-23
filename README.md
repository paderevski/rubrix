# Rubrix - AI Test Generator

An AI-powered multiple choice question generator for AP Computer Science A and Calculus, built with Tauri + React.

![Rubrix Screenshot](docs/screenshot.png)

## Features

- 📚 **Multi-Subject Support** - AP Computer Science A and Calculus
- 🎯 **Topic Selection** - Choose from multiple topics per subject
- 🎚️ **Difficulty Control** - Easy, Medium, or Hard questions
- 🤖 **AI Generation** - Powered by AWS Bedrock through a secure gateway
- 🔐 **Gateway Authentication** - Username/password login with credential validation
- 🔢 **LaTeX Rendering** - Full mathematical notation support with KaTeX
- ✏️ **Edit Questions** - Modify generated questions before export
- 🔄 **Smart Regeneration** - Regenerate questions with context awareness
- 📤 **Export to QTI** - Direct export to Schoology-compatible format
- 💻 **Code Block Support** - Syntax highlighting for code in questions and answers
- 📊 **Markdown Tables** - Support for tables in question content

## Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) 1.70+
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

## Quick Start

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### First Run
On first launch, you'll be prompted to authenticate against the configured gateway. Credentials are stored in app-local storage for automatic login. See [QUICKSTART_CREDENTIALS.md](QUICKSTART_CREDENTIALS.md) for details.

## Configuration

### Gateway Configuration
Rubrix is gateway-only. Set the gateway URL in `src-tauri/.env` (or bake it at build time):

```bash
BEDROCK_GATEWAY_URL=https://your-api.example.com/generate

# Optional: model routing overrides in gateway Lambda
BEDROCK_MODEL_ID=openai.gpt-oss-120b-1:0
BEDROCK_MODEL_ID_FRQ=deepseek.v3.2
```

Without this setting, generation and authentication are disabled.

FRQ requests include `question_type=frq` and can be routed to the FRQ model in the gateway.

See [docs/BEDROCK_GATEWAY_CONTRACT.md](docs/BEDROCK_GATEWAY_CONTRACT.md) for request/response details.

### Production Builds
Release builds require gateway configuration and user authentication.

### Bug Reporting Endpoint
The app supports a standardized bug submission workflow (`Help -> Submit Bug`) that posts JSON to your server.

Set these in `src-tauri/.env` (or bake at build time):
- `BUG_REPORT_URL` (required) - AWS endpoint that receives bug JSON
- `BUG_REPORT_API_KEY` (optional) - sent as `x-api-key`
- `BUG_REPORT_BEARER_TOKEN` (optional) - sent as `Authorization: Bearer ...`

Canonical schema: [docs/BUG_REPORT_SCHEMA.json](docs/BUG_REPORT_SCHEMA.json)
AWS GitHub issue router sample: [lambda_functions/bug_intake/lambda_handler.py](lambda_functions/bug_intake/lambda_handler.py)
Full deployment runbook (IAM/API Gateway/services): [docs/BUG_REPORT_DEPLOYMENT.md](docs/BUG_REPORT_DEPLOYMENT.md)

```
rubrix/
├── src/                    # React frontend
│   ├── components/         # UI components
│   │   ├── QuestionCard.tsx    # Question display with LaTeX/code rendering
│   │   ├── EditModal.tsx       # Question editor
│   │   ├── Sidebar.tsx         # Topic selection
│   │   └── StreamingPreview.tsx # Real-time generation view
│   ├── App.tsx             # Main app component
│   └── types.ts            # TypeScript types
│
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Tauri commands & Question struct
│   │   ├── llm.rs          # Replicate API client
│   │   ├── qti.rs          # QTI export with LaTeX & formatting
│   │   ├── knowledge.rs    # Subject-specific knowledge management
│   │   └── prompts.rs      # Prompt templating system
│   └── knowledge/          # Subject knowledge bases
│       ├── Computer Science/
│       │   ├── prompt.txt          # CS-specific generation prompt
│       │   ├── question-bank.json  # Example CS questions
│       │   ├── question-schema.json
│       │   └── [topic files].txt   # Topic knowledge
│       └── Calculus/
│           ├── prompt.txt          # Calculus-specific prompt
│           ├── question-bank.json  # Example calc questions
│           └── question-schema.json
│
└── package.json
```

### Subject-Specific Prompts

Each subject has its own prompt template in `knowledge/[Subject]/prompt.txt`. These prompts use placeholders:
- `{topics}` - Selected topics
- `{difficulty}` - Question difficulty
- `{count}` - Number of questions
- `{examples}` - Few-shot examples from question-bank.json
- `{user_instructions}` - Optional user guidance
- `{regenerate}` - Context for regeneration

### Adding Knowledge Base Questions

Add example questions to `knowledge/[Subject]/question-bank.json`:

```json
{
  "id": "1",
  "text": "What is the derivative of $f(x) = x^2$?",
  "subject": "Calculus",
  "topics": ["Derivatives"],
  "answers": [
    {"text": "$f'(x) = 2x$", "is_correct": true, "explanation": "Power rule"},
    {"text": "$f'(x) = x$", "is_correct": false}
  ],
  "explanation": "Use the power rule"
}
```

**Formatting Support:**
- Inline LaTeX: `$f'(x)$`
- Display LaTeX: `$$\\int_0^1 x^2 dx$$`
- Code blocks: ` ```java ... ``` `
- Inline code: `` `variable` ``
- Markdown tables

Available **Computer Science** topics:
- Arrays, Recursion, Strings, Classes, Inheritance
- ArrayList, 2D Arrays, Sorting

Available **Calculus** topics:
- Derivatives, Integrals, Limits, and more (41 topics total)

## Usage

1. **Select Subject** - Choose between Computer Science or Calculus
2. **Select Topics** - Check the topics you want questions about
3. **Set Difficulty** - Choose Easy, Medium, or Hard
4. **Adjust Count** - Slide to set number of questions (1-20)
5. **Add Notes** - Optional guidance for the AI
6. **Generate** - Click to create questions with streaming preview
7. **Review & Edit** - Modify any questions as needed
8. **Regenerate Individual Questions** - Click regenerate on any question to get a replacement (preserves subject/topics)
9. **Export** - Save as .txt or .imscc (QTI format)

## Exporting to Schoology

1. Click "Export QTI"
2. Save as `questions.imscc`
3. In Schoology: Resources → Add Resources → Import File
4. Select your .imscc file
5. Questions appear as a Question Bank

## Development

### Frontend (React)
```bash
cd rubrix
npm run dev
```

### Backend (Rust)
```bash
cd rubrix/src-tauri
cargo build
```

### Full App
```bash
npm run tauri dev
```

## Tech Stack

- **Frontend**: React 18, TypeScript, Tailwind CSS, Radix UI
- **Rendering**: ReactMarkdown, KaTeX, rehype/remark plugins
- **Backend**: Rust, Tauri
- **AI**: Claude Sonnet 4.5 via Replicate
- **Export**: IMS QTI 1.2 with enhanced LaTeX and formatting support

## Recent Improvements (v0.3.0)

- ✅ Multi-subject support with subject-specific prompts
- ✅ LaTeX rendering with proper prime/apostrophe handling
- ✅ Smart paragraph grouping (inline LaTeX stays in paragraph)
- ✅ HTML escaping only on non-LaTeX text
- ✅ Markdown table support in QTI export
- ✅ Multi-line answer support with `<br />` tags
- ✅ Special character cleaning (UTF-8 artifacts, curly quotes)
- ✅ Question regeneration with context preservation
- ✅ Automatic version syncing between package.json and tauri.conf.json

## License

MIT
