---
layout: default
title: User Guide
---

# User Guide

Complete guide to using Rubrix effectively.

## Table of Contents
- [Generating Questions](#generating-questions)
- [Editing Questions](#editing-questions)
- [Regenerating Questions](#regenerating-questions)
- [Exporting Questions](#exporting-questions)
- [Importing to Schoology](#importing-to-schoology)
- [Tips for Best Results](#tips-for-best-results)

---

## Generating Questions

### Basic Workflow

1. **Select Subject** - Choose Computer Science or Calculus from the sidebar
2. **Select Topics** - Check one or more topics
3. **Set Difficulty** - Easy, Medium, or Hard
4. **Choose Quantity** - 1-20 questions
5. **Add Instructions** (optional) - Guide the AI
6. **Click Generate** - Watch questions appear in real-time

### Understanding Difficulty Levels

**Easy**
- Basic concept application
- Simple scenarios
- Direct recall or straightforward logic
- Example: "What does `arr.length` return?"

**Medium**
- Standard AP exam difficulty
- May require multiple steps
- Understanding of how concepts combine
- Example: "What is printed after this code executes?"

**Hard**
- Complex scenarios
- Deep conceptual understanding
- May involve tricky edge cases
- Example: "Analyze this recursive method with multiple base cases"

### Using Instructions Effectively

The "User Instructions" field lets you guide the AI. Good examples:

✅ **Good Instructions**
- "Focus on array indexing errors"
- "Include questions about StringBuilder"
- "Use realistic scenarios from banking or games"
- "Avoid questions requiring memorization of syntax"

❌ **Less Effective**
- "Make them good" (too vague)
- "Question 1 should be about..." (AI generates all at once)

---

## Editing Questions

Click the **Edit** button (pencil icon) on any question to modify it.

### What You Can Edit

**Question Text**
- Full markdown support
- LaTeX math: Use `$f'(x)$` for inline, `$$...$$` for display
- Code blocks: Use triple backticks with language:
  ```java
  public void example() {
      System.out.println("test");
  }
  ```

**Answers**
- Edit answer text
- Change which answer is correct (checkbox)
- Add explanations

**Metadata**
- Difficulty level
- Topics (for regeneration context)

### Formatting Reference

**Text Formatting**
- *Italic*: `*text*`
- **Bold**: `**text**`
- Inline code: `` `variable` ``

**Math (Calculus)**
- Inline: `$f'(x) = 2x$` → $f'(x) = 2x$
- Display: `$$\int_0^1 x^2 dx$$` → $$\int_0^1 x^2 dx$$
- Common symbols: `\frac{a}{b}`, `\sqrt{x}`, `\lim_{x \to 0}`

**Code (Computer Science)**
````
```java
public class Example {
    private int value;
}
```
````

**Tables**
```
| Header 1 | Header 2 |
|----------|----------|
| Value 1  | Value 2  |
```

---

## Regenerating Questions

Don't like a question? Click the **Regenerate** button (refresh icon).

**How it works:**
- Generates a replacement question on the same topic(s)
- Maintains the same difficulty level
- Uses the same subject context
- Replaces the old question

**Tip**: If you keep getting similar questions, try:
1. Adding specific instructions
2. Changing the difficulty level
3. Selecting additional topics for more variety

---

## Exporting Questions

Rubrix supports two export formats:

### Plain Text (.txt)

Click **Export TXT** to save questions as readable text.

**Format:**
```
Title: Question Bank

1. What is the time complexity of binary search?

a. O(log n)
a. O(n)
a. O(n log n)
a. O(n²)
```

**Use cases:**
- Sharing with colleagues
- Manual entry into other systems
- Reference/study guides

### QTI Format (.imscc)

Click **Export QTI** to save in LMS-compatible format.

**Features:**
- Fully formatted questions
- Preserves LaTeX math
- Preserves code formatting
- Direct Schoology/Canvas import

---

## Importing to Schoology

### Step-by-Step

1. **Export from Rubrix**
   - Click "Export QTI"
   - Save as `questions.imscc`

2. **Open Schoology**
   - Go to your course
   - Navigate to **Resources**

3. **Import**
   - Click **Add Resources**
   - Select **Import File**
   - Choose your `.imscc` file
   - Click **Import**

4. **Verify**
   - Questions appear as a Question Bank
   - Open to check formatting
   - Math and code should display correctly

5. **Use in Assessments**
   - Create a new Test/Quiz
   - Click **Add Question**
   - Select **From Question Bank**
   - Choose your imported questions

### Troubleshooting

**Math not rendering?**
- Ensure Schoology's LaTeX support is enabled
- Some Schoology accounts may need admin settings adjusted

**Code formatting lost?**
- This is expected - Schoology may strip some formatting
- Consider including code as images for complex examples

**Wrong answer marked?**
- The first answer in the export is always correct
- Verify in Schoology's question editor if needed

---

## Tips for Best Results

### Topic Selection

**Do:**
- ✅ Select 2-4 related topics for coherent questions
- ✅ Choose topics you're currently teaching
- ✅ Mix topics for review quizzes

**Don't:**
- ❌ Select all topics (questions may be too varied)
- ❌ Mix very basic and advanced topics

### Instructions

**Effective instructions:**
- "Focus on common mistakes students make"
- "Include questions that require code tracing"
- "Use realistic variable names and scenarios"
- "Avoid questions that require knowing exact method signatures"

### Quantity

**Recommendations:**
- Start with 5-10 questions to test
- Review and refine before generating more
- Generate in batches for different difficulty levels

### Quality Control

**Always review generated questions for:**
- Factual accuracy
- Appropriate difficulty
- Clear wording
- Distinct answer choices
- Fair distractors (wrong answers should be plausible)

**Common things to fix:**
- Overly complex wording
- Answer choices that give away the correct answer
- Questions that are too easy or too hard
- Ambiguous phrasing

---

## Keyboard Shortcuts

- `Cmd/Ctrl + E` - Edit selected question
- `Cmd/Ctrl + R` - Regenerate selected question
- `Cmd/Ctrl + S` - Export (opens save dialog)
- `Cmd/Ctrl + G` - Generate new questions

---

## Need Help?

- [Check the FAQ](faq.md) for common questions
- [Open an issue](https://github.com/paderevski/rubrix/issues) on GitHub
- [Join discussions](https://github.com/paderevski/rubrix/discussions) with other users

---

[← Getting Started](getting-started.md) | [FAQ →](faq.md)
