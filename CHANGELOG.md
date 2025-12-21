# Changelog

All notable changes to Rubrix are documented in this file.

## [Unreleased]

### Added
- Save and load question banks functionality
- Display and formatting for question explanations
- Enforce JSON-only responses from AI to prevent parsing errors
- Normalize mathematical output formatting

### Fixed
- Question explanation rendering now properly formatted
- Markdown rendering for "Steps" sections in explanations

### Changed
- Tweaked AI prompts for better question quality

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

[Unreleased]: https://github.com/paderevski/rubrix/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/paderevski/rubrix/compare/v0.2.3...v0.3.0
[0.2.3]: https://github.com/paderevski/rubrix/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/paderevski/rubrix/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/paderevski/rubrix/compare/v0.2...v0.2.1
[0.2]: https://github.com/paderevski/rubrix/compare/v0.1.1...v0.2
[0.1.1]: https://github.com/paderevski/rubrix/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/paderevski/rubrix/releases/tag/v0.1.0
