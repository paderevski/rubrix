# Changelog

All notable changes to Catie (repository: rubrix) are documented in this file.

## [0.12.0] - 2026-03-18

### Added
- `<question>` tags in prompts so multi-question responses can be parsed reliably; good questions are salvaged when one fails to parse
- "Regenerate All Questions" runs regenerations in parallel

### Fixed
- LaTeX rendering of `aligned` regions

---

## [0.11.0] - 2026-03-08

### Added
- Password-based login against the gateway with username normalization and a cleaned-up login modal
- Word (.docx) export, including multiple versions and page breaks
- Export of answer explanations
- Document tabs, New/Close document flow, Open Recent, Preferences (default subject/difficulty), per-question raw output, topic/subtopic selection
- Menu bar with keyboard shortcuts for most actions

### Changed
- Removed the old direct-token password tooling

---

## [0.10.0] - 2026-03-01

### Added
- In-app **Help → Submit Bug** workflow backed by an AWS intake Lambda that files GitHub issues (see [docs/BUG_REPORT_WORKFLOW.md](docs/BUG_REPORT_WORKFLOW.md)); recent app logs are attached
- Remaining-token count shown after generation (`remaining_tokens` in the stream payload)

### Changed
- Improved regeneration logic

### Fixed
- Main window scrolling bug; stray `\\[4pt]` sequences in LaTeX

---

## [0.9.0] - 2026-02-17

### Added
- **Bedrock gateway**: the app now authenticates against a gateway Lambda that calls AWS Bedrock server-side; no AWS token is needed on the client (see [docs/BEDROCK_GATEWAY_CONTRACT.md](docs/BEDROCK_GATEWAY_CONTRACT.md))
- User-management scripts in [tools/](tools/README.md)

### Changed
- Gateway is the only generation path in dev and release builds; direct Bedrock tokens and mock mode were removed
- `BEDROCK_GATEWAY_URL` is required (runtime `.env` or build-time)

---

## [0.8.0] - 2026-02-10

### Added
- Basic keyboard shortcuts

### Changed
- Append is now the default generation mode
- Bedrock region switched to us-east-1

---

## [0.7.0] - 2026-02-10

### Added
- Rebrand to **Catie**
- Lambda-based credential store and user tools

### Fixed
- Several LaTeX escaping issues (`\(`/`\)` delimiters, stray `\n`)
- Generation no longer retries on HTTP 500

---

## [0.6.0] - 2026-01-07

### Added
- Authentication step to obtain an API token before generating

### Fixed
- Computer Science code styling

---

## [0.5.0] - 2025-12-26

### Added
- Subtopic support end-to-end: schema parsing, topic tree with children, prompt typing, and Bank Editor selection
- Configurable knowledge directory via `RUBRIX_KNOWLEDGE_DIR`; defaults to per-user app data; bundled knowledge remains embedded for release builds
- Bank Editor now reads/writes question banks from the configured knowledge path and shows subtopic dropdowns alongside topics

### Changed
- Topic IDs now map back to human-readable names in generation prompts to avoid code-like labels (e.g., `T001`)
- Knowledge file resolution falls back to embedded assets but prefers user-edited copies in the writable knowledge directory

### Fixed
- Dev save operations on the bank no longer trigger rebuilds by writing into the app data/override path instead of the source tree

---

## [0.4.0] - 2025-12-21

### Added
- AWS Bedrock streaming backend with reasoning-aware SSE parsing
- Native zoom controls via View menu (Cmd/Ctrl `+`, `-`, `0`) with persisted zoom factor
- Simplified streaming preview that shows raw reasoning tokens during generation

### Changed
- Prompt/response parser now ignores reasoning brackets before parsing the JSON array
- Root font sizing driven by `--app-zoom` CSS variable so UI scales uniformly

### Fixed
- LaTeX not-equal (`\ne`) now renders correctly instead of inserting newlines

---

## [0.3.0] - 2024-12-19

### Added
- **Multi-subject support**: Added Calculus alongside Computer Science
- **Subject-specific prompts**: Each subject has customized generation prompts
- **LaTeX rendering**: Full mathematical notation support with KaTeX
- **Enhanced QTI export**:
  - Proper LaTeX handling (primes/apostrophes no longer HTML-encoded)
  - Smart paragraph grouping (inline LaTeX stays in paragraph)
  - Markdown table support
  - Multi-line answer support with `<br />` tags
  - Special character cleaning (UTF-8 artifacts, curly quotes, dashes)
- **Question regeneration with context**: Preserves subject and topics when regenerating
- **Automatic version syncing**: package.json and tauri.conf.json stay in sync
- **GitHub Pages documentation site**: User-friendly docs at paderevski.github.io/rubrix

### Changed
- Refactored knowledge base to support multiple subjects with JSON structure
- Reorganized knowledge files into subject-specific folders
- Updated Calculus question bank with proper markdown formatting

### Fixed
- HTML escaping now only applies to non-LaTeX text
- Inline math expressions stay within paragraphs instead of creating separate ones

---

## [0.2.3] - 2024

### Changed
- Version bump for release

---

## [0.2.2] - 2024

### Changed
- Version bump for release

---

## [0.2.1] - 2024

### Changed
- Updated environment token configuration
- Improved versioning system

---

## [0.2] - 2024

### Added
- Text input for regenerate question functionality
- Ability to provide instructions when regenerating individual questions

---

## [0.1.1] - 2024

### Changed
- Version bump for release
- Minor fixes

---

## [0.1.0] - 2024

### Added
- Initial release
- GitHub Actions release workflow
- Core question generation functionality
- Streaming preview of question generation
- Edit and regenerate questions
- QTI export for Schoology
- Plain text export
- Support for 8 AP Computer Science A topics:
  - Arrays
  - ArrayList
  - 2D Arrays
  - Recursion
  - Strings
  - Classes
  - Inheritance
  - Sorting
- Difficulty selection (Easy, Medium, Hard)
- Adjustable question count (1-20)
- User instructions for AI guidance
- Code block syntax highlighting
- ReactMarkdown integration

---

## Project History

### Development Milestones

**v0.1.0 - v0.2**: Foundation
- Established core architecture (Tauri + React + Rust)
- Implemented streaming AI generation with Claude via Replicate
- Built question bank system
- Created QTI export functionality

**v0.2 - v0.3.0**: Multi-Subject & Enhanced Export
- Expanded to multiple subjects (CS + Calculus)
- Dramatically improved QTI export with proper LaTeX handling
- Subject-specific knowledge bases and prompts
- Professional documentation site

**v0.3.0+**: Quality & Features
- Focus on question quality improvements
- Enhanced explanation formatting
- Question bank save/load functionality
- Stricter AI response formatting

---

[Unreleased]: https://github.com/paderevski/rubrix/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/paderevski/rubrix/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/paderevski/rubrix/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/paderevski/rubrix/compare/v0.2.3...v0.3.0
[0.2.3]: https://github.com/paderevski/rubrix/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/paderevski/rubrix/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/paderevski/rubrix/compare/v0.2...v0.2.1
[0.2]: https://github.com/paderevski/rubrix/compare/v0.1.1...v0.2
[0.1.1]: https://github.com/paderevski/rubrix/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/paderevski/rubrix/releases/tag/v0.1.0
