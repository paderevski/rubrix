---
layout: default
title: Getting Started
---

# Getting Started with Rubrix

## Installation

### macOS

1. Download `Rubrix.dmg` from the [latest release](https://github.com/paderevski/rubrix/releases/latest)
2. Open the DMG file
3. Drag Rubrix to your Applications folder
4. Right-click Rubrix and select "Open" (first time only, due to macOS security)

### Windows

1. Download `Rubrix-setup.exe` from the [latest release](https://github.com/paderevski/rubrix/releases/latest)
2. Run the installer
3. Follow the installation wizard
4. Launch Rubrix from your Start Menu

### Linux

1. Download the `.AppImage` or `.deb` file from the [latest release](https://github.com/paderevski/rubrix/releases/latest)
2. For AppImage:
   ```bash
   chmod +x Rubrix.AppImage
   ./Rubrix.AppImage
   ```
3. For Debian/Ubuntu:
   ```bash
   sudo dpkg -i rubrix_*.deb
   ```

---

## API Setup

Rubrix uses the Replicate API to generate questions with AI. You'll need an API key:

### Step 1: Create a Replicate Account

1. Go to [replicate.com](https://replicate.com)
2. Sign up (free tier includes $5 credit)
3. Go to your [Account Settings](https://replicate.com/account)
4. Copy your API token

### Step 2: Add Your API Key to Rubrix

**Currently**, you need to add the API key to the source code (we're working on a UI for this):

1. Open the app's configuration file:
   - **macOS**: `~/Library/Application Support/Rubrix/config.json`
   - **Windows**: `%APPDATA%\Rubrix\config.json`
   - **Linux**: `~/.config/Rubrix/config.json`

2. Add your API key:
   ```json
   {
     "replicate_api_token": "your-token-here"
   }
   ```

**OR** build from source with your key (see [README](https://github.com/paderevski/rubrix#configuration))

> **Coming Soon**: API key input in the app UI!

### Costs

- ~$0.005 per question (half a cent)
- $0.50 for 100 questions
- $5.00 for 1,000 questions
- First $5 is free with new Replicate account

---

## First Run

### 1. Launch Rubrix

Open the application - you'll see the main interface with a sidebar for subject selection.

### 2. Select a Subject

Click on either:
- **Computer Science** - for AP CS A questions
- **Calculus** - for calculus questions

### 3. Choose Topics

Check the boxes for topics you want questions about:

**Computer Science topics:**
- Arrays
- ArrayList
- 2D Arrays
- Recursion
- Strings
- Classes
- Inheritance
- Sorting

**Calculus topics:**
- Derivatives
- Integrals
- Limits
- Chain Rule
- Product Rule
- And 36 more...

### 4. Set Difficulty

Choose from:
- **Easy** - Basic concepts, straightforward questions
- **Medium** - Standard AP-level difficulty
- **Hard** - Complex scenarios, deeper understanding

### 5. Adjust Count

Use the slider to choose how many questions (1-20)

### 6. Add Optional Instructions

In the "User Instructions" field, you can add guidance like:
- "Focus on tracing code execution"
- "Include examples with negative numbers"
- "Avoid advanced topics"

### 7. Click Generate!

Watch as questions stream in real-time. This typically takes 30-60 seconds.

---

## Next Steps

- [Learn how to edit and export questions →](user-guide.md)
- [Read frequently asked questions →](faq.md)
- [Report issues or request features](https://github.com/paderevski/rubrix/issues)

---

[← Back to Home](index.md) | [User Guide →](user-guide.md)
