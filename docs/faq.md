---
layout: default
title: FAQ
---

# Frequently Asked Questions

## General

### What is Catie?

Catie (Test Item Engine; the repository is named `rubrix`) is a desktop application that uses AI to generate multiple-choice questions for AP Computer Science A and Calculus.

### Is Catie free? Do I need an API key?

The app is open source (MIT). You do not need an AI or AWS key: generation runs through a hosted gateway, and you log in with a username and password from the project administrator.

### What subjects are supported?

AP Computer Science A and Calculus. More subjects are possible by adding a knowledge folder (prompt, schema and question bank).

### Can I use Catie offline?

No. Generation, login and Word export need internet access. Editing, saving and QTI/Markdown export of existing documents work without generating, but QTI math is rendered as images from an online service when imported.

---

## Using Catie

### How long does generation take?

Usually under a minute for a handful of questions; it depends on the count and current load.

### How many questions can I generate at once?

1-8 per run. Generated questions are appended to the document, so run it several times.

### Why are some questions not quite right?

AI can make factual errors, write ambiguous wording, miss the difficulty, or write weak distractors. Always review, edit or regenerate.

### Can I save my work and reopen it?

Yes. **File → Save** writes a JSON session file; **File → Open** and **Open Recent** reload it. Catie does not auto-save.

### Can I add my own example questions?

Yes, in the **Bank Editor**. Edits are saved to your local knowledge folder and override the bundled defaults. Set `RUBRIX_KNOWLEDGE_DIR` to use a different folder (see the [README](https://github.com/paderevski/rubrix#knowledge-base)).

### What does the token count mean?

If your account has a usage quota, the app shows the remaining tokens after generation.

---

## Exporting & LMS Integration

### What export formats exist?

QTI (`.imscc`), Markdown and Word (`.docx`).

### Which LMSs work with QTI?

Schoology (primary target). QTI 1.2 is also supported by Canvas, Blackboard and Moodle, but only Schoology import is routinely tested.

### Why isn't my math rendering in the LMS?

Math is exported as images from an online LaTeX service. Make sure the LMS can load external images and the formula is valid LaTeX.

### Can I import QTI back into Catie?

No. Save sessions as JSON to continue editing in Catie.

---

## Quality & Content

### Which AI model is used?

The gateway calls an OpenAI open-weight model (`gpt-oss-120b`) on AWS Bedrock, guided by subject-specific prompts and few-shot examples from the question bank.

### Are questions aligned with AP standards?

The prompts are designed around AP curricula, but there is no guarantee of official alignment. Use your professional judgment.

### Can questions be reused year after year?

Be cautious: students may share them. Prefer fresh or substantially edited questions.

---

## Technical

### What platforms are supported?

macOS (Apple Silicon and Intel), Windows 10/11, and Linux (`.deb`, `.rpm`, AppImage).

### Is my data secure?

- Prompts are sent to the gateway over HTTPS and forwarded to AWS Bedrock; documents are saved only where you save them.
- Your password is SHA256-hashed before it is sent. Your username and password hash are stored locally in the app data folder for auto-login; Logout clears them.
- Bug reports include recent app logs.

### How do I update Catie?

Download the latest installer from [GitHub Releases](https://github.com/paderevski/rubrix/releases). There is no auto-update yet.

### Can I contribute?

Yes: [issues](https://github.com/paderevski/rubrix/issues) and [pull requests](https://github.com/paderevski/rubrix/pulls) are welcome.

---

## Troubleshooting

### The app won't open (macOS / Windows)

Builds are unsigned; follow the [install guide](release-build-readme.md).

### Login fails

- "Password incorrect" / "user not found" - check your credentials with the administrator.
- "Auth server unreachable" - check your internet connection.
- "Auth service not configured" - the build is missing `BEDROCK_GATEWAY_URL` (see [Credentials](CREDENTIALS.md)).

### Questions aren't generating

- Check your connection and that you are logged in.
- Open the raw preview (`Cmd/Ctrl + \`) to see what the model returned.
- If you hit a quota, the error says so; contact the administrator.
- Use **Help → Submit Bug** if it persists.

### Math isn't displaying correctly

Use proper LaTeX: inline `$f'(x)$`, display `$$\int_0^1 x\,dx$$`. Edit the question to fix delimiters.

---

[← User Guide](user-guide.md) | [Back to Home](index.md)
