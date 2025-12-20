# Refactor Plan: Simplify Question Format from stem+code to text

## Current State
Questions use two fields:
- `stem`: Question text
- `code`: Optional code block

Plus a deprecated `content` field for backward compatibility.

## Proposed State
Questions use ONE field:
- `text`: Complete question text (markdown with code blocks)

## Files to Change

### 1. src-tauri/src/main.rs
- Question struct: Remove `stem`, `code`, `content` → Add `text`
- QuestionBankEntry struct: Remove `stem`, `code` → Add `text`

### 2. src-tauri/src/knowledge.rs
- QuestionContent struct: Remove `stem`, `code` → Add `text`
- Mapping logic: Combine stem+code into text when loading

### 3. src-tauri/src/prompts.rs
- Prompt template: Change JSON schema from stem+code to text
- Example formatting: Combine stem+code into text
- Parsing logic: Remove content field population

### 4. src-tauri/src/qti.rs
- Export logic: Parse markdown code blocks from text field
- Remove stem/code specific handling

### 5. question-bank.json files
- Restructure all questions to use single "text" field
- Combine existing stem + code into markdown format

## Migration Strategy

### Question Bank Format
```json
// OLD
{
  "content": {
    "stem": "What is the output?",
    "code": "System.out.println(5);"
  }
}

// NEW
{
  "content": {
    "text": "What is the output?\n\n```java\nSystem.out.println(5);\n```"
  }
}
```

### Benefits
1. Simpler data model
2. LLMs naturally generate markdown
3. Fewer fields to maintain
4. More flexible (can have multiple code blocks, mixed text/code)
5. Removes backward compatibility code
