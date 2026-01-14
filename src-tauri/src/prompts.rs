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
    // Use custom prompt template if provided
    if let Some(template) = config.prompt_template {
        return format_custom_prompt(template, config);
    }

    // Fallback: No prompt template found
    // This should not happen in normal operation - all subjects should have a prompt.txt file
    eprintln!("WARNING: No prompt template found. Please add a prompt.txt file for this subject.");

    // Return minimal fallback that will at least allow basic generation
    format!(
        r#"Generate {count} multiple choice question(s) about {topics} at {difficulty} difficulty level.

Return ONLY a JSON array with this structure:
[
  {{
    "text": "Question text",
    "explanation": "Explanation of the correct answer",
    "distractors": "Why wrong answers are tempting",
    "answers": [
      {{"text": "Answer 1", "is_correct": false, "explanation": "Why wrong"}},
      {{"text": "Answer 2", "is_correct": true, "explanation": "Why correct"}}
    ]
  }}
]"#,
        count = config.count,
        topics = config.topics,
        difficulty = config.difficulty
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
    topics_label: &str,
) -> String {
    let config = PromptConfig {
        topics: topics_label.to_string(),
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

    eprintln!("Parsing LLM response ({} chars)", trimmed.len());
    eprintln!("First 200 chars: {}", &trimmed[..trimmed.len().min(200)]);

    // Extract the first *complete* JSON array. This is robust against:
    // - leading/trailing prose
    // - ```json fences
    // - models that accidentally output multiple arrays
    fn extract_first_json_array(s: &str) -> Option<String> {
        let bytes = s.as_bytes();
        let mut array_depth: i32 = 0;
        let mut tracked_start: Option<usize> = None;
        let mut tracked_level: Option<i32> = None;
        let mut in_string = false;
        let mut escape = false;

        let mut i = 0;
        while i < bytes.len() {
            let b = bytes[i];

            if in_string {
                if escape {
                    escape = false;
                    i += 1;
                    continue;
                }
                if b == b'\\' {
                    escape = true;
                    i += 1;
                    continue;
                }
                if b == b'"' {
                    in_string = false;
                }
                i += 1;
                continue;
            }

            if b == b'"' {
                in_string = true;
                i += 1;
                continue;
            }

            if b == b'[' {
                array_depth += 1;

                if tracked_start.is_none() {
                    let mut j = i + 1;
                    while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                        j += 1;
                    }
                    if j < bytes.len() && bytes[j] == b'{' {
                        tracked_start = Some(i);
                        tracked_level = Some(array_depth);
                    }
                }

                i += 1;
                continue;
            }

            if b == b']' {
                if array_depth > 0 {
                    if let (Some(start), Some(level)) = (tracked_start, tracked_level) {
                        if array_depth == level {
                            return Some(s[start..=i].to_string());
                        }
                    }
                    array_depth -= 1;

                    if array_depth == 0 {
                        tracked_start = None;
                        tracked_level = None;
                    }
                }

                i += 1;
                continue;
            }

            i += 1;
        }

        None
    }

    let json_str = extract_first_json_array(trimmed).ok_or_else(|| {
        eprintln!("ERROR: No JSON array found in response");
        eprintln!("Response text: {}", &trimmed[..trimmed.len().min(1000)]);
        "No JSON array found in response".to_string()
    })?;

    eprintln!("Extracted JSON array ({} chars)", json_str.len());

    let sanitized_json = sanitize_json_string(&json_str);

    let questions: Vec<Question> = serde_json::from_str(&sanitized_json).map_err(|e| {
        eprintln!("ERROR: Failed to parse JSON: {}", e);
        eprintln!(
            "JSON string: {}",
            &sanitized_json[..sanitized_json.len().min(500)]
        );
        format!(
            "Failed to parse JSON response: {}. JSON: {}",
            e,
            &sanitized_json[..sanitized_json.len().min(500)]
        )
    })?;

    if questions.is_empty() {
        eprintln!("ERROR: Parsed JSON array is empty");
        eprintln!("JSON was: {}", &json_str[..json_str.len().min(500)]);
        return Err("No questions found in JSON response".to_string());
    }

    eprintln!("Successfully parsed {} question(s)", questions.len());

    // Assign IDs
    let mut result = Vec::new();
    for (i, mut q) in questions.into_iter().enumerate() {
        q.id = format!("q{}", i + 1);
        result.push(q);
    }

    Ok(result)
}

/// Ensure literal control characters inside JSON strings are escaped so serde can parse them.
/// Also fixes common invalid escape sequences that LLMs generate (e.g., \( \) \[ \] for LaTeX).
fn sanitize_json_string(input: &str) -> String {
    let mut output = String::with_capacity(input.len() + 256);
    let mut in_string = false;
    let mut escape = false;

    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        if in_string {
            if escape {
                // We're in an escape sequence - check if it's valid JSON
                match ch {
                    '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' | 'u' => {
                        // Valid JSON escape sequence
                        output.push(ch);
                    }
                    _ => {
                        // Invalid escape sequence (common for LaTeX like \(, \), \[, \], etc.)
                        // Add another backslash to make it literal
                        output.push('\\');
                        output.push(ch);
                    }
                }
                escape = false;
                i += 1;
                continue;
            }

            match ch {
                '\\' => {
                    output.push(ch);
                    escape = true;
                }
                '"' => {
                    output.push(ch);
                    in_string = false;
                }
                '\n' => output.push_str("\\n"),
                '\r' => output.push_str("\\r"),
                '\t' => output.push_str("\\t"),
                _ => output.push(ch),
            }
        } else {
            output.push(ch);
            if ch == '"' {
                in_string = true;
                escape = false;
            }
        }

        i += 1;
    }

    output
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

    #[test]
    fn test_parse_handles_nested_arrays() {
        let input = r#"[
  {
    "text": "Nested array question",
    "topics": ["arrays", "loops"],
    "answers": [
      {"text": "Option A", "is_correct": false},
      {"text": "Option B", "is_correct": true}
    ]
  }
]"#;

        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 1);
        assert_eq!(questions[0].text, "Nested array question");
        assert_eq!(questions[0].answers.len(), 2);
        assert!(questions[0].answers[1].is_correct);
    }

    #[test]
    fn test_parse_handles_multiline_strings() {
        let input = r#"[
    {
        "text": "Line 1
Line 2",
        "answers": [
            {"text": "Top", "is_correct": true},
            {"text": "Bottom", "is_correct": false}
        ]
    }
]"#;

        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 1);
        assert_eq!(questions[0].text, "Line 1\nLine 2");
    }

    #[test]
    fn test_parse_handles_latex_escape_sequences() {
        // LLMs often generate invalid JSON escapes like \(, \), \[, \] for LaTeX
        // Our sanitizer should fix these by adding an extra backslash
        let input = r#"[
    {
        "text": "Function \\(f(x)=x^2\\) on interval \\[0,2\\]",
        "explanation": "Formula: \\[f(x) = x^2\\]\nThis is valid.",
        "answers": [
            {"text": "$x^2$", "is_correct": true},
            {"text": "$2x$", "is_correct": false}
        ]
    }
]"#;

        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 1);
        // The output should preserve the LaTeX delimiters
        assert!(questions[0].text.contains("\\(f(x)=x^2\\)"));
        assert!(questions[0].text.contains("\\[0,2\\]"));
    }
}
