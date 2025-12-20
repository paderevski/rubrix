---
layout: default
title: FAQ
---

# Frequently Asked Questions

## General

### What is Rubrix?

Rubrix is a desktop application that uses AI to generate multiple-choice questions for AP Computer Science A and Calculus courses. It's designed for teachers who want to quickly create high-quality quiz and test questions.

### Is Rubrix free?

Yes! Rubrix is free and open source. However, you'll need a Replicate API key for the AI generation, which costs approximately $0.005 per question (~$0.50 for 100 questions). New Replicate users get $5 in free credits.

### What subjects are supported?

Currently:
- **AP Computer Science A** (8 core topics)
- **Calculus** (41 topics)

More subjects are planned for future releases!

### Can I use Rubrix offline?

No, Rubrix requires an internet connection to generate questions since it uses cloud-based AI (Replicate/Claude). However, once generated, you can edit and export questions offline.

---

## Setup & Configuration

### How do I get a Replicate API key?

1. Go to [replicate.com](https://replicate.com)
2. Sign up for a free account
3. Navigate to your Account Settings
4. Copy your API token
5. Add it to Rubrix (see [Getting Started](getting-started.md#api-setup))

### Do you have plans to add API key input in the UI?

Yes! This is a priority feature for the next release. Currently you need to add it to a config file or build from source.

### Can I use my own AI model?

Not yet, but we're considering adding support for other AI providers (OpenAI, Anthropic direct, local models) in future versions.

### How much does it cost to generate questions?

Approximate costs with Replicate:
- ~$0.005 per question (half a cent)
- $0.50 for 100 questions
- $5.00 for 1,000 questions

New users get $5 free credit from Replicate.

---

## Using Rubrix

### How long does it take to generate questions?

Typically 30-60 seconds for 10 questions. Time varies based on:
- Number of questions
- Complexity of topics
- Current API load

### Can I generate more than 20 questions at once?

Currently, no. The limit is set to 20 to:
- Keep generation time reasonable
- Encourage quality review of each batch
- Prevent excessive API costs

You can generate multiple batches if needed.

### Why are some generated questions not quite right?

AI isn't perfect! Common issues:
- **Factual errors** - Rare, but possible
- **Ambiguous wording** - May need clarification
- **Too easy/hard** - Adjust difficulty setting
- **Poor distractors** - Wrong answers may be too obvious

This is why we include full editing capabilities. Always review questions before using them!

### Can I save my questions for later?

Yes! Export questions as:
- **QTI (.imscc)** - For importing to LMS
- **Plain text (.txt)** - For reference

You can then import them back into your LMS or generate new questions anytime.

### Can I add my own example questions?

Yes! Advanced users can add questions to the knowledge base:
1. Navigate to the knowledge folder in the installation
2. Edit `question-bank.json` for your subject
3. Follow the JSON format for existing questions

More details in the [technical README](https://github.com/pewhite/rubrix#adding-knowledge-base-questions).

---

## Exporting & LMS Integration

### What LMS platforms are supported?

The QTI export format works with:
- ✅ Schoology
- ✅ Canvas
- ✅ Blackboard
- ✅ Moodle
- ✅ Most LMS platforms supporting QTI 1.2

### Why isn't my math rendering in Schoology?

Some Schoology accounts may not have LaTeX rendering enabled by default. Check with your Schoology administrator about enabling math rendering support.

### Can I edit questions after importing to my LMS?

Yes! Once imported, you can edit questions directly in your LMS. The import is just the starting point.

### Do I need to export every time?

Yes, Rubrix doesn't automatically save your questions. Always export if you want to keep them.

### Can I import questions back into Rubrix?

Not currently. Once exported, questions live in your LMS or text files. This is a potential future feature.

---

## Quality & Content

### How does Rubrix ensure question quality?

- Uses Claude Sonnet 4.5, one of the most capable AI models
- Includes subject-specific prompts tuned for AP curricula
- Uses few-shot learning with example questions
- Allows full editing and regeneration

However, **always review questions** before using them with students.

### Are questions aligned with AP standards?

The prompts and examples are designed around AP curricula, but Rubrix doesn't guarantee official AP alignment. Use your professional judgment to ensure questions meet your standards.

### Can questions be reused year after year?

Be cautious! While technically possible:
- Students may share questions
- AI-generated content might be similar across generations
- Best practice: Generate fresh questions or significantly modify old ones

### Will students get the same questions if they use Rubrix?

Very unlikely. Even with the same settings, the AI generates different questions each time. However, questions on the same topic may have some similarities.

---

## Technical

### What platforms does Rubrix run on?

- macOS (Intel and Apple Silicon)
- Windows 10/11
- Linux (AppImage and Debian packages)

### Can I run Rubrix on a school computer?

If you have permission to install software, yes! However:
- You'll need internet access
- You'll need your own Replicate API key
- Check with your IT department first

### Is my data secure?

- Questions are generated via Replicate's API
- No questions are stored on external servers
- Your API key is stored locally
- See Replicate's privacy policy for their data handling

### Can I contribute to Rubrix?

Absolutely! Rubrix is open source:
- Report bugs: [GitHub Issues](https://github.com/pewhite/rubrix/issues)
- Request features: [GitHub Discussions](https://github.com/pewhite/rubrix/discussions)
- Contribute code: [Pull Requests](https://github.com/pewhite/rubrix/pulls)

### How do I update Rubrix?

Currently, download and install the latest version from [GitHub Releases](https://github.com/pewhite/rubrix/releases). Auto-update is planned for future releases.

---

## Troubleshooting

### The app won't launch on macOS

This is usually a security issue. Try:
1. Right-click the app
2. Select "Open"
3. Click "Open" again in the dialog

This adds a security exception.

### I'm getting API errors

Check:
- Is your API key correct?
- Do you have Replicate credits remaining?
- Is your internet connection working?
- Try generating fewer questions at once

### Questions aren't generating

- Wait 60 seconds - generation can take time
- Check your internet connection
- Verify API key is configured
- Look for error messages in the app

### Math isn't displaying correctly

Ensure you're using proper LaTeX syntax:
- Inline: `$f'(x)$`
- Display: `$$\int_0^1 x dx$$`
- Escape backslashes in JSON: `\\frac` not `\frac`

---

## Feature Requests

### Can you add [subject]?

Possibly! Priority subjects for future versions:
- AP Statistics
- Chemistry
- Physics
- Biology

Request subjects in [GitHub Discussions](https://github.com/pewhite/rubrix/discussions).

### Can you add [feature]?

We're actively developing! Planned features:
- UI for API key input
- Question history/library
- Auto-update
- More export formats
- Bulk editing
- Question templates

Request features via [GitHub Issues](https://github.com/pewhite/rubrix/issues).

---

## Still Have Questions?

- 📖 [Read the User Guide](user-guide.md)
- 💬 [Join GitHub Discussions](https://github.com/pewhite/rubrix/discussions)
- 🐛 [Report Issues](https://github.com/pewhite/rubrix/issues)
- 📧 Email: [Open an issue instead - we respond faster!]

---

[← User Guide](user-guide.md) | [Back to Home](index.md)
