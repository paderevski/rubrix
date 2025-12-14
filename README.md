# Rubrix - AP CS Test Generator

An AI-powered multiple choice question generator for AP Computer Science A, built with Tauri + React.

![Rubrix Screenshot](docs/screenshot.png)

## Features

- 🎯 **Topic Selection** - Choose from 8 AP CS A topics
- 🎚️ **Difficulty Control** - Easy, Medium, or Hard questions
- 🤖 **AI Generation** - Powered by Claude Sonnet 4.5 via Replicate
- ✏️ **Edit Questions** - Modify generated questions before export
- 🔄 **Regenerate** - Don't like a question? Generate a new one
- 📤 **Export to QTI** - Direct export to Schoology-compatible format

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

## Project Structure

```
rubrix/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── App.tsx             # Main app component
│   └── types.ts            # TypeScript types
│
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Tauri commands
│   │   ├── llm.rs          # Replicate API client
│   │   ├── qti.rs          # QTI export
│   │   ├── knowledge.rs    # Knowledge base
│   │   └── prompts.rs      # LLM prompts
│   └── knowledge/          # Example question banks
│
└── package.json
```

## Configuration

### API Key

Edit `src-tauri/src/llm.rs` and replace:
```rust
const REPLICATE_API_TOKEN: &str = "YOUR_REPLICATE_API_TOKEN_HERE";
```

With your actual Replicate API token.

### Adding Knowledge Base Questions

Add question files to `src-tauri/knowledge/` in this format:

```
Title: Topic Name

1. Question text here?

```java
// Optional code block
public void example() { }
```

a. Correct answer (always first)
a. Wrong answer 2
a. Wrong answer 3
a. Wrong answer 4
```

Available topic files:
- `arrays.txt`
- `recursion.txt`
- `strings.txt`
- `classes.txt`
- `inheritance.txt`
- `arraylist.txt`
- `2darrays.txt`
- `sorting.txt`

## Usage

1. **Select Topics** - Check the topics you want questions about
2. **Set Difficulty** - Choose Easy, Medium, or Hard
3. **Adjust Count** - Slide to set number of questions (1-20)
4. **Add Notes** - Optional guidance for the AI
5. **Generate** - Click to create questions
6. **Review & Edit** - Modify any questions as needed
7. **Export** - Save as .txt or .imscc (QTI format)

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

- **Frontend**: React 18, TypeScript, Tailwind CSS
- **Backend**: Rust, Tauri
- **AI**: Claude Sonnet 4.5 via Replicate
- **Export**: IMS QTI 1.2

## License

MIT
