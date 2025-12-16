/// Build the prompt for generating multiple questions using JSON output
pub fn build_generation_prompt(
    request: &GenerationRequest,
    examples: &[QuestionBankEntry],
) -> String {
    let topics_str = request.topics.join(", ");

    let difficulty_desc = match request.difficulty.as_str() {
        "easy" => "D1 (Easy) - Basic recall or simple application, 1-2 steps",
        "medium" => "D2 (Medium) - Requires analysis or multi-step reasoning, 3-5 steps",
        "hard" => "D3 (Hard) - Complex analysis, synthesis of multiple concepts, 5+ steps",
        _ => "D2 (Medium) - Requires analysis or multi-step reasoning",
    };

    // Format JSON examples
    let examples_str = if examples.is_empty() {
        String::from("(No examples available - generate based on AP CS A standards)")
    } else {
        examples
            .iter()
            .enumerate()
            .map(|(i, e)| {
                format!(
                    "### Example {}\n```json\n{}\n```",
                    i + 1,
                    format_example_as_json(e)
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    };

    let notes_str = match &request.notes {
        Some(notes) if !notes.trim().is_empty() => {
            format!("\n**Additional Instructions:** {}", notes)
        }
        _ => String::new(),
    };

    format!(
        r#"You are an expert AP Computer Science A question writer.

**Target:** {topics} | **Difficulty:** {difficulty} | **Count:** {count}
{notes}

## Reference Examples

{examples}

---

## Output Format

Return a JSON array with {count} question object(s). **No text outside the JSON.**

For each question, the `work` field comes FIRST. This is where you trace through the code, calculate the answer, and plan distractors. This reasoning is essential for accuracy.

```json
[
  {{
    "work": "FIRST: Trace the code step-by-step here. Calculate the correct answer. Then identify 3 common student errors and what wrong answers they produce. Example: 'mystery(5): 5 + mystery(4) → 5+4+3+2+1+0 = 15. Errors: off-by-one→14, factorial confusion→120, returns 0 early→0'",
    "stem": "Question text (markdown supported: **bold**, `inline code`)",
    "code": "// Java code goes here, plain text\n// Use \\n for newlines",
    "answers": [
      {{"text": "`15`", "is_correct": true}},
      {{"text": "`14`", "is_correct": false}},
      {{"text": "`120`", "is_correct": false}},
      {{"text": "`0`", "is_correct": false}}
    ],
    "correct_explanation": "Step-by-step: mystery(5) = 5 + mystery(4) = 5 + 4 + mystery(3) = ... = 15",
    "distractor_explanations": {{
      "14": "Off-by-one: stops at n=1 instead of n=0",
      "120": "Confuses addition with multiplication (5!)",
      "0": "Only considers base case return value"
    }}
  }}
]
```

## Field Reference

| Field | Required | Description |
|-------|----------|-------------|
| `work` | Yes | **Write this first.** Trace code, calculate answer, plan distractors. |
| `stem` | Yes | Question text. Markdown OK. |
| `code` | No | Java code snippet. Plain text, use `\n` for newlines. |
| `answers` | Yes | Array of 4-5 choices. Exactly one `is_correct: true`. |
| `correct_explanation` | Yes | Step-by-step solution walkthrough. |
| `distractor_explanations` | Yes | Map of wrong answer → misconception that causes it. |

## Rules

1. **Work first**: Always fill `work` before other fields. This is how you verify correctness.
2. **One correct answer**: Exactly one answer has `is_correct: true`.
3. **Real distractors**: Each wrong answer must come from a specific student error (off-by-one, wrong operator, loop bounds, pass-by-value confusion, etc.)
4. **Valid Java**: Code must be syntactically correct.
5. **Markdown in strings**: Use backticks for inline code in `stem` and `answers.text`.
6. **Pure JSON**: Return ONLY the JSON array. No markdown fences, no explanation outside the JSON.

Generate {count} question(s):"#,
        topics = topics_str,
        difficulty = difficulty_desc,
        count = request.count,
        examples = examples_str,
        notes = notes_str,
    )
}
