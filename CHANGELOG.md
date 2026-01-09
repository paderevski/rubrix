# Changelog

All notable changes to Rubrix are documented in this file.

## [Unreleased]

### Added
- **Dev/Release credential management**: Development mode now caches AWS tokens in system keychain for seamless restarts
- Keyring integration for secure, OS-encrypted credential storage (macOS Keychain, Windows Credential Manager, Linux Secret Service)
- Dev mode indicator in UI header showing "🔧 DEV MODE" badge
- New `is_dev_mode` command to detect build profile from frontend
- `.env.example` file with documentation for development environment variables
- `DEV_AWS_TOKEN` environment variable support for local development overrides

### Changed
- Authentication flow now auto-saves tokens to keychain in development mode
- Token resolution hierarchy: provided > keychain > DEV_AWS_TOKEN > AWS_BEARER_TOKEN_BEDROCK > mock/error
- `check_auth` command now checks both memory cache and keychain in dev mode
- `clear_auth` (logout) now clears keychain in addition to memory cache in dev mode
- Mock mode now only activates in development builds when no credentials available

### Documentation
- Added comprehensive [docs/CREDENTIALS.md](docs/CREDENTIALS.md) guide for credential management
- Updated [DEV_RELEASE_STRATEGY.md](DEV_RELEASE_STRATEGY.md) with full implementation details

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
