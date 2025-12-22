---
layout: default
title: Changelog
---

# Changelog

See what's new in each version of Rubrix.

---

## Latest (Unreleased)

**Features Coming Soon:**
- Save and load question banks
- Better explanation formatting
- Improved AI prompt quality

---

## Version 0.3.0
*December 19, 2024*

This is a major feature release bringing multi-subject support and dramatically improved exports.

### 🎉 New Features

**Multi-Subject Support**
- Added Calculus support (41 topics)
- Computer Science now has 8 topics
- Each subject has customized AI prompts

**Beautiful Math Rendering**
- Full LaTeX support for mathematical notation
- Inline math: $f'(x) = 2x$
- Display equations: $$\int_0^1 x^2 dx$$
- Proper rendering in both the app and exported files

**Enhanced QTI Export**
- Fixed LaTeX prime handling (apostrophes work correctly now!)
- Smart paragraph grouping (inline math stays with text)
- Markdown table support
- Multi-line answers with proper formatting
- Cleans special characters (curly quotes, dashes, etc.)

**Smart Regeneration**
- Regenerate button now remembers question context
- Preserves subject and topics automatically
- Better replacement questions

**Developer Improvements**
- Automatic version syncing between package.json and Tauri config
- GitHub Pages documentation site

### 🐛 Bug Fixes
- Fixed HTML escaping breaking LaTeX formulas
- Fixed inline math creating unwanted paragraph breaks

### [Download v0.3.0](https://github.com/paderevski/rubrix/releases/tag/v0.3.0)

---

## Version 0.2.3
*2024*

Minor release with version updates.

### [Download v0.2.3](https://github.com/paderevski/rubrix/releases/tag/v0.2.3)

---

## Version 0.2.2
*2024*

Minor release with version updates.

### [Download v0.2.2](https://github.com/paderevski/rubrix/releases/tag/v0.2.2)

---

## Version 0.2.1
*2024*

### Changes
- Updated API token configuration
- Improved versioning system

### [Download v0.2.1](https://github.com/paderevski/rubrix/releases/tag/v0.2.1)

---

## Version 0.2
*2024*

### New Features
- Added instruction field for question regeneration
- Provide custom guidance when regenerating individual questions

### [Download v0.2](https://github.com/paderevski/rubrix/releases/tag/v0.2)

---

## Version 0.1.1
*2024*

Minor bug fixes and improvements.

### [Download v0.1.1](https://github.com/paderevski/rubrix/releases/tag/v0.1.1)

---

## Version 0.1.0
*2024*

🎉 **Initial Release!**

### Features

**Question Generation**
- AI-powered generation using Claude Sonnet 4.5
- Real-time streaming preview
- 8 AP Computer Science A topics
- 3 difficulty levels (Easy, Medium, Hard)
- Generate 1-20 questions at once
- Custom instructions for the AI

**Question Management**
- Edit any generated question
- Modify question text and answers
- Regenerate individual questions
- Mark correct answers

**Export Options**
- QTI format for Schoology/LMS import
- Plain text format
- Preserves code formatting and structure

**Topics Included**
- Arrays
- ArrayList
- 2D Arrays
- Recursion
- Strings
- Classes
- Inheritance
- Sorting

**Technical**
- Desktop app for Mac, Windows, and Linux
- Built with Tauri + React
- Rust backend for performance
- Modern, clean interface

### [Download v0.1.0](https://github.com/paderevski/rubrix/releases/tag/v0.1.0)

---

## Future Plans

We're actively developing Rubrix! Planned features include:

- **More subjects**: Statistics, Chemistry, Physics, Biology
- **Question library**: Save and reuse questions across sessions
- **Better export**: More LMS formats, custom templates
- **API key UI**: Enter your Replicate key directly in the app
- **Auto-update**: Seamless updates to new versions
- **Question templates**: Create reusable question formats
- **Bulk editing**: Edit multiple questions at once

Have ideas? [Share them on GitHub!](https://github.com/paderevski/rubrix/discussions)

---

[← Back to Home](index.md)
