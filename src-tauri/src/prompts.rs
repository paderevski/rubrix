//! Prompt templates and response parsing for LLM interactions

use crate::{GenerationRequest, Question, QuestionBankEntry};

/// Configuration for prompt building
pub struct PromptConfig<'a> {
    pub topics: String,
    pub difficulty: &'a str,
    pub count: usize,
    pub examples: &'a [QuestionBankEntry],
    pub user_instructions: Option<&'a str>,
    pub regenerate_context: Option<RegenerateContext<'a>>,
    pub prompt_template: Option<&'a str>,
}

/// Context for regeneration requests
pub struct RegenerateContext<'a> {
    pub current_question: &'a Question,
    pub other_questions: Vec<String>,
}

/// Build the core prompt (used for both generate and regenerate)
fn build_core_prompt(config: &PromptConfig) -> String {
    // Use custom prompt template if provided, otherwise use default
    if let Some(template) = config.prompt_template {
        return format_custom_prompt(template, config);
    }

    // Default prompt (original AP CS A prompt)
    let difficulty_desc = match config.difficulty {
        "easy" => "D1 (Easy) - Basic recall or simple application, 1-2 steps",
        "medium" => "D2 (Medium) - Requires analysis or multi-step reasoning, 3-5 steps",
        "hard" => "D3 (Hard) - Complex analysis, synthesis of multiple concepts, 5+ steps",
        _ => "D2 (Medium) - Requires analysis or multi-step reasoning",
    };

    // Format JSON examples with pedagogically useful fields
    let examples_str = if config.examples.is_empty() {
        String::from("(No examples available - generate based on AP CS A standards)")
    } else {
        config
            .examples
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

    // Build optional sections
    let user_instructions_str = match config.user_instructions {
        Some(notes) if !notes.trim().is_empty() => {
            format!("\n\n**Additional Instructions from User:**\n{}", notes)
        }
        _ => String::new(),
    };

    let regenerate_section = match &config.regenerate_context {
        Some(ctx) => {
            let other_questions_str = if ctx.other_questions.is_empty() {
                String::new()
            } else {
                format!(
                    "\n\n**Other questions in this set (avoid duplicating these):**\n{}",
                    ctx.other_questions
                        .iter()
                        .map(|q| format!("- {}", q))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            };
            format!(
                r#"

---

## REGENERATION REQUEST

You are replacing an existing question. Generate a NEW question that is DIFFERENT but covers a similar topic and difficulty.

**Question to replace:**
{}
{}

Keep similar topic and difficulty but use DIFFERENT code/scenarios."#,
                ctx.current_question.text, other_questions_str
            )
        }
        None => String::new(),
    };

    format!(
        r#"You are an expert AP Computer Science A question writer with strong analytical and debugging skills.

**CRITICAL RULE: Calculate the correct answer BEFORE writing the question.**

When generating questions with code or calculations:
1. First, decide on the concept you'll test
2. Write the code mentally or on scratch paper
3. TRACE through the code step-by-step and calculate the correct answer
4. Verify your calculation is correct - double-check your math
5. Create wrong answers based on specific misconceptions
6. ONLY THEN start writing in the output format below

If you discover an error while writing your explanation, DO NOT try to fix it mid-stream. Instead, silently start that question over with different code.

**Target Topic(s):** {topics}
**Target Difficulty:** {difficulty}
**Number of Questions:** {count}
{regenerate}
---

## Reference Examples (JSON format)

Study these examples carefully. Pay special attention to:
- How the `distractors` field shows WHY each wrong answer is tempting
- The `common_errors` that students make
- The relationship between `difficulty` and `cognitive_level`
- The precision and accuracy of the explanations

{examples}

---

## Your Task

Generate {count} NEW question(s) that:
1. Test the specified topic(s) at the target difficulty
2. Use DIFFERENT code and scenarios than the examples
3. Each wrong answer must exploit a specific student misconception
4. Match the quality and style shown in the examples
5. Have internally consistent, mathematically correct answers that you've verified

**WORKFLOW (FOLLOW THIS ORDER):**

For EACH question you write:

**Step 1: Plan & Solve (Do this in the explanation section)**
- Decide what concept you'll test
- Write the code you'll use
- TRACE through it step-by-step and calculate the CORRECT answer
- Double-check your calculation - verify it's right
- Think through common student errors for this type of problem

**Step 2: Design Distractors (Also in the explanation section)**
- Identify 3-4 specific misconceptions students have
- For each misconception, calculate what wrong answer it would produce
- Make sure each distractor comes from a real error pattern (off-by-one, wrong operator, pass-by-value confusion, loop bound errors, etc.)

**Step 3: Write the Question (Now you can start writing JSON)**
- Only NOW should you write the stem and code
- Write ALL answer choices (correct + distractors) with their explanations
- Write the explanation showing the step-by-step solution
- Write the distractors analysis explaining the error patterns

**Output Format (JSON Array):**

Return your response as a JSON array containing {count} question object(s). Each question should follow this structure:

```json
[
  {{
    "text": "Question text here with markdown formatting like **bold** or `code`. Can include code blocks:\n\n```java\nSystem.out.println(\"Hello\");\n```",
    "explanation": "Step-by-step walkthrough of how to arrive at the correct answer. Verify correct answer. Design distractors.",
        "distractors": "Analysis of why each wrong answer is tempting and what misconception leads to it",
    "answers": [
      {{"text": "Answer text (use markdown like `42` for code)", "is_correct": false, "explanation": "Why this is wrong"}},
      {{"text": "Another answer", "is_correct": true, "explanation": "Why this is correct"}},
      {{"text": "Third answer", "is_correct": false, "explanation": "Common misconception"}},
      {{"text": "Fourth answer", "is_correct": false, "explanation": "Off-by-one error"}}
        ]
  }}
]
```

**Field Guidelines:**
- `text`: Complete question text in markdown format (can include code blocks with ```java```)
- `answers`: Array of 4-5 answer choices with explanations
  - `text`: Answer text (use backticks for code like `42`)
  - `is_correct`: Boolean indicating if this is the correct answer
  - `explanation`: Brief explanation of why this answer is correct/incorrect
- `explanation`: Detailed walkthrough showing how to solve the problem
- `distractors`: Analysis of common student errors that lead to wrong answers

**Quality Checklist (verify before submitting):**
- ✓ Did you work out the correct answer BEFORE writing anything?
- ✓ Does your explanation match the answer you marked as correct?
- ✓ Does each distractor represent a real student error pattern?
- ✓ Is your code syntactically correct Java?
- ✓ Are all inline code references wrapped in backticks?
- ✓ Is your explanation clear, accurate, and step-by-step?
- ✓ Did you return ONLY the JSON array with no extra text?
{user_instructions}

Generate a JSON array with {count} question(s) now:"#,
        topics = config.topics,
        difficulty = difficulty_desc,
        count = config.count,
        examples = examples_str,
        regenerate = regenerate_section,
        user_instructions = user_instructions_str,
    )
}

/// Format a custom prompt template with config values
fn format_custom_prompt(template: &str, config: &PromptConfig) -> String {
    let difficulty_desc = match config.difficulty {
        "easy" => "D1 (Easy) - Basic recall or simple application, 1-2 steps",
        "medium" => "D2 (Medium) - Requires analysis or multi-step reasoning, 3-5 steps",
        "hard" => "D3 (Hard) - Complex analysis, synthesis of multiple concepts, 5+ steps",
        _ => "D2 (Medium) - Requires analysis or multi-step reasoning",
    };

    let examples_str = if config.examples.is_empty() {
        String::from("(No examples available)")
    } else {
        config
            .examples
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

    let user_instructions_str = match config.user_instructions {
        Some(notes) if !notes.trim().is_empty() => {
            format!("\n\n**Additional Instructions from User:**\n{}", notes)
        }
        _ => String::new(),
    };

    let regenerate_section = match &config.regenerate_context {
        Some(ctx) => {
            let other_questions_str = if ctx.other_questions.is_empty() {
                String::new()
            } else {
                format!(
                    "\n\n**Other questions in this set (avoid duplicating these):**\n{}",
                    ctx.other_questions
                        .iter()
                        .map(|q| format!("- {}", q))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            };
            format!(
                "\n\n**Question to replace:**\n{}\n{}",
                ctx.current_question.text, other_questions_str
            )
        }
        None => String::new(),
    };

    // Replace placeholders in template
    template
        .replace("{topics}", &config.topics)
        .replace("{difficulty}", difficulty_desc)
        .replace("{count}", &config.count.to_string())
        .replace("{examples}", &examples_str)
        .replace("{user_instructions}", &user_instructions_str)
        .replace("{regenerate}", &regenerate_section)
}

/// Build the prompt for generating multiple questions using JSON examples
pub fn build_generation_prompt(
    request: &GenerationRequest,
    examples: &[QuestionBankEntry],
    prompt_template: Option<&str>,
) -> String {
    let config = PromptConfig {
        topics: request.topics.join(", "),
        difficulty: &request.difficulty,
        count: request.count as usize,
        examples,
        user_instructions: request.notes.as_deref(),
        regenerate_context: None,
        prompt_template,
    };
    build_core_prompt(&config)
}

/// Format a question bank entry as JSON with only pedagogically useful fields
fn format_example_as_json(q: &QuestionBankEntry) -> String {
    // Build a clean JSON representation with the useful fields
    let answers_json: Vec<String> = q
        .options
        .iter()
        .map(|opt| {
            // Use the current generation schema (`answers`) to reduce model confusion.
            // We intentionally omit `id` and per-choice `explanation` because the bank doesn't store them.
            format!(
                r#"    {{"text": "{}", "is_correct": {}}}"#,
                escape_json_string(&opt.text),
                opt.is_correct
            )
        })
        .collect();

    // Emit distractors as a string (matches the app's Question schema), but still preserve
    // the pedagogy signal from the bank examples.
    let distractors_text = {
        let mut lines: Vec<String> = Vec::new();

        if !q.distractors.common_mistakes.is_empty() {
            lines.push("Common mistakes:".to_string());
            for m in &q.distractors.common_mistakes {
                lines.push(format!("- {}: {}", m.option_id, m.misconception));
            }
        }

        if !q.distractors.common_errors.is_empty() {
            lines.push(format!(
                "Common errors: {}",
                q.distractors.common_errors.join(", ")
            ));
        }

        lines.join("\n")
    };

    let skills_json: Vec<String> = q.skills.iter().map(|s| format!(r#""{}""#, s)).collect();

    format!(
        r#"{{
  "text": "{text}",
  "answers": [
{answers}
  ],
  "explanation": "{explanation}",
  "difficulty": "{difficulty}",
  "cognitive_level": "{cognitive_level}",
  "skills": [{skills}],
    "distractors": "{distractors}"
}}"#,
        text = escape_json_string(&q.text),
        answers = answers_json.join(",\n"),
        explanation = escape_json_string(&q.explanation),
        difficulty = q.difficulty,
        cognitive_level = q.cognitive_level,
        skills = skills_json.join(", "),
        distractors = escape_json_string(&distractors_text),
    )
}

/// Escape special characters for JSON strings
fn escape_json_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Build prompt for regenerating a single question
pub fn build_regenerate_prompt(
    current: &Question,
    context: &[Question],
    examples: &[QuestionBankEntry],
    user_instructions: Option<&str>,
    prompt_template: Option<&str>,
) -> String {
    // Extract topics from current question content (simple heuristic)
    let topics = "similar topic as original".to_string();

    // Build context of other questions to avoid duplication
    let other_questions: Vec<String> = context
        .iter()
        .filter(|q| q.id != current.id)
        .take(3)
        .map(|q| truncate(&q.text, 50))
        .collect();

    let config = PromptConfig {
        topics,
        difficulty: "medium", // Could be inferred from the question in future
        count: 1,
        examples: &examples[..examples.len().min(1)], // Use at most 1 example for regeneration
        user_instructions,
        regenerate_context: Some(RegenerateContext {
            current_question: current,
            other_questions,
        }),
        prompt_template,
    };
    build_core_prompt(&config)
}

/// Parse LLM response into Question objects
pub fn parse_llm_response(response: &str) -> Result<Vec<Question>, String> {
    let trimmed = response.trim();

    // Extract the first *complete* JSON array. This is robust against:
    // - leading/trailing prose
    // - ```json fences
    // - models that accidentally output multiple arrays
    fn extract_first_json_array(s: &str) -> Option<String> {
        let bytes = s.as_bytes();

        // Find first '['
        let mut start = None;
        for i in 0..bytes.len() {
            if bytes[i] == b'[' {
                start = Some(i);
                break;
            }
        }
        let start = start?;

        let mut depth: i32 = 0;
        let mut in_string = false;
        let mut escape = false;

        for i in start..bytes.len() {
            let b = bytes[i];

            if in_string {
                if escape {
                    escape = false;
                    continue;
                }
                if b == b'\\' {
                    escape = true;
                    continue;
                }
                if b == b'"' {
                    in_string = false;
                }
                continue;
            }

            if b == b'"' {
                in_string = true;
                continue;
            }

            if b == b'[' {
                depth += 1;
            } else if b == b']' {
                depth -= 1;
                if depth == 0 {
                    return Some(s[start..=i].to_string());
                }
            }
        }

        None
    }

    let json_str = extract_first_json_array(trimmed).ok_or("No JSON array found in response")?;

    let questions: Vec<Question> = serde_json::from_str(&json_str).map_err(|e| {
        format!(
            "Failed to parse JSON response: {}. JSON: {}",
            e,
            &json_str[..json_str.len().min(500)]
        )
    })?;

    if questions.is_empty() {
        return Err("No questions found in JSON response".to_string());
    }

    // eprintln!(
    //    "DEBUG: Successfully parsed {} questions from JSON",
    //    questions.len()
    // );

    // Assign IDs
    let mut result = Vec::new();
    for (i, mut q) in questions.into_iter().enumerate() {
        q.id = format!("q{}", i + 1);
        result.push(q);
    }

    Ok(result)
}

/// Truncate string to max length
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_format() {
        let input = r#"[
  {
    "text": "What is returned by this code?\n\n```java\nreturn 5 + 3;\n```",
    "answers": [
      {"text": "`8`", "is_correct": true, "explanation": "Correct addition"},
      {"text": "`53`", "is_correct": false, "explanation": "String concatenation error"},
      {"text": "`\"53\"`", "is_correct": false, "explanation": "Wrong type"},
      {"text": "Error", "is_correct": false, "explanation": "Compiles fine"}
    ],
    "explanation": "Simple addition of 5 + 3 equals 8.",
    "distractors": "String concatenation vs addition confusion"
  }
]"#;

        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 1);
        assert_eq!(questions[0].answers.len(), 4);
        assert!(questions[0].answers[0].is_correct);
        assert_eq!(
            questions[0].text,
            "What is returned by this code?\n\n```java\nreturn 5 + 3;\n```"
        );
    }

    #[test]
    fn test_parse_multiple_questions() {
        let input = r#"[
  {
    "text": "Question 1",
    "answers": [
      {"text": "A", "is_correct": true},
      {"text": "B", "is_correct": false}
    ]
  },
  {
    "text": "Question 2",
    "answers": [
      {"text": "C", "is_correct": false},
      {"text": "D", "is_correct": true}
    ]
  }
]"#;

        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 2);
        assert_eq!(questions[0].id, "q1");
        assert_eq!(questions[1].id, "q2");
    }

    #[test]
    fn test_parse_first_json_array_when_multiple_present() {
        // Models sometimes emit multiple JSON blocks (e.g., "starting over"), and we should
        // parse the first complete JSON array only.
        let input = r#"Here you go:

```json
[
    {
        "text": "First array question",
        "answers": [
            {"text": "A", "is_correct": true}
        ]
    }
]
```

Some extra commentary.

```json
[
    {
        "text": "Second array question",
        "answers": [
            {"text": "B", "is_correct": true}
        ]
    }
]
```
"#;

        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 1);
        assert_eq!(questions[0].text, "First array question");
    }

    #[test]
    fn test_parse_accepts_legacy_options_field() {
        // Older prompts/examples (and some models) emit `options` instead of `answers`.
        // We alias that field to keep parsing resilient.
        let input = r#"[
    {
        "text": "Legacy options question",
        "options": [
            {"text": "A", "is_correct": true},
            {"text": "B", "is_correct": false}
        ]
    }
]"#;

        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 1);
        assert_eq!(questions[0].text, "Legacy options question");
        assert_eq!(questions[0].answers.len(), 2);
        assert!(questions[0].answers[0].is_correct);
    }
}
