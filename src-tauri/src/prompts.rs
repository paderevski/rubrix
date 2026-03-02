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
    let topics_label = if current.topics.is_empty() {
        "(not specified)".to_string()
    } else {
        current.topics.join(", ")
    };

    let subject_label = if current.subject.trim().is_empty() {
        "(not specified)".to_string()
    } else {
        current.subject.clone()
    };

    let inferred_difficulty = examples
        .first()
        .map(|e| e.difficulty.clone())
        .unwrap_or_else(|| "same as original".to_string());

    let inferred_style = examples
        .first()
        .map(|e| e.cognitive_level.clone())
        .unwrap_or_else(|| "same as original".to_string());

    let current_json = serde_json::to_string_pretty(current)
        .unwrap_or_else(|_| "{\"error\":\"failed to serialize current question\"}".to_string());

    // Build short context of other questions to reduce duplication
    let other_questions: Vec<String> = context
        .iter()
        .filter(|q| q.id != current.id)
        .take(3)
        .map(|q| truncate(&q.text, 50))
        .collect();

    let other_questions_block = if other_questions.is_empty() {
        "(none)".to_string()
    } else {
        other_questions
            .iter()
            .map(|q| format!("- {}", q))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let user_instructions_block = match user_instructions {
        Some(text) if !text.trim().is_empty() => text.trim().to_string(),
        _ => "(none)".to_string(),
    };

    if let Some(template) = prompt_template {
        return template
            .replace("{current_question_json}", &current_json)
            .replace("{difficulty}", &inferred_difficulty)
            .replace("{topics}", &topics_label)
            .replace("{subject}", &subject_label)
            .replace("{style}", &inferred_style)
            .replace("{other_questions}", &other_questions_block)
            .replace("{user_instructions}", &user_instructions_block);
    }

    format!(
        r#"You are rewriting ONE multiple-choice question.

Here is the current question JSON (include stem, choices, and metadata):
```json
{current_json}
```

This is a level {difficulty} question about {topics}.
Metadata:
- subject: {subject}
- cognitive/style target: {style}
- answer choice count target: match the current question unless the prompt requires a fix
Avoid duplicating these other questions in the current set:
{other_questions}

Additional human instructions (must follow):
{user_instructions}

Please craft a NEW question that is similar in topic, complexity, and style, but not a paraphrase.

Required guards:
1. Return ONLY a JSON array with exactly 1 question object.
2. Preserve schema exactly:
   - text (string)
   - explanation (string)
   - distractors (string)
   - answers (array of objects with text, is_correct, explanation)
3. Include exactly one correct answer (`is_correct: true`).
4. Keep explanations internally consistent with the marked correct answer.
5. Do not mention these instructions or include markdown fences in output.

Output format (strict):
[
  {{
    "text": "...",
    "explanation": "...",
    "distractors": "...",
    "answers": [
      {{"text": "...", "is_correct": false, "explanation": "..."}},
      {{"text": "...", "is_correct": true,  "explanation": "..."}}
    ]
  }}
]"#,
        current_json = current_json,
        difficulty = inferred_difficulty,
        topics = topics_label,
        subject = subject_label,
        style = inferred_style,
        other_questions = other_questions_block,
        user_instructions = user_instructions_block,
    )
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

    let mut questions: Vec<Question> = serde_json::from_str(&sanitized_json).map_err(|e| {
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

    for question in &mut questions {
        normalize_question_text(question);
    }

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

const DOUBLE_BACKSLASH_N_EXCEPTIONS: [&str; 5] = ["eq", "abla", "u", "ewline", "ewcommand"];

fn normalize_question_text(question: &mut Question) {
    question.text = normalize_escaped_math_and_newlines(&question.text);

    if let Some(explanation) = question.explanation.as_ref() {
        question.explanation = Some(normalize_escaped_math_and_newlines(explanation));
    }

    if let Some(distractors) = question.distractors.as_ref() {
        question.distractors = Some(normalize_escaped_math_and_newlines(distractors));
    }

    for answer in &mut question.answers {
        answer.text = normalize_escaped_math_and_newlines(&answer.text);
        if let Some(explanation) = answer.explanation.as_ref() {
            answer.explanation = Some(normalize_escaped_math_and_newlines(explanation));
        }
    }
}

fn normalize_escaped_math_and_newlines(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut output = String::with_capacity(input.len());
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '\\' && i + 2 < chars.len() && chars[i + 1] == '\\' && chars[i + 2] == 'n' {
            let mut is_exception = false;
            for exception in DOUBLE_BACKSLASH_N_EXCEPTIONS.iter() {
                let end = i + 3 + exception.len();
                if end <= chars.len() && chars[i + 3..end].iter().copied().eq(exception.chars()) {
                    is_exception = true;
                    break;
                }
            }

            if is_exception {
                output.push(chars[i]);
                output.push(chars[i + 1]);
                output.push(chars[i + 2]);
            } else {
                output.push('\n');
            }

            i += 3;
            continue;
        }

        if chars[i] == '\\' && i + 1 < chars.len() && (i == 0 || chars[i - 1] != '\\') {
            let next = chars[i + 1];
            if next == '(' {
                output.push('$');
                i += 2;
                while i < chars.len() && (chars[i] == ' ' || chars[i] == '\t') {
                    i += 1;
                }
                continue;
            }

            if next == ')' {
                while output.ends_with(' ') || output.ends_with('\t') {
                    output.pop();
                }
                output.push('$');
                i += 2;
                continue;
            }

            if next == '[' {
                output.push_str("$$");
                i += 2;
                while i < chars.len() && (chars[i] == ' ' || chars[i] == '\t') {
                    i += 1;
                }
                continue;
            }

            if next == ']' {
                while output.ends_with(' ') || output.ends_with('\t') {
                    output.pop();
                }
                output.push_str("$$");
                i += 2;
                continue;
            }
        }

        output.push(chars[i]);
        i += 1;
    }

    output
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
                    '\n' => output.push('n'),
                    '\r' => output.push('r'),
                    '\t' => output.push('t'),
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
    fn test_backslash_newline_sanitization() {
        let input = "[\n  {\n    \"text\": \"Line 1\\\nLine 2\",\n    \"answers\": [\n      {\"text\": \"A\", \"is_correct\": true}\n    ]\n  }\n]";

        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 1);
        assert_eq!(questions[0].text, "Line 1\nLine 2");
    }

    #[test]
    fn test_preserve_double_backslash_spacing_command() {
        let input = r#"[
    {
        "text": "Consider the piecewise function\n\n$$\nf(x)=\\begin{cases}\n\\dfrac{3x^{2}-kx-12}{x-3}, & x\\neq 3,\\\\[6pt]\n c, & x=3.\n\\end{cases}\n$$",
        "answers": [
            {"text": "A", "is_correct": true}
        ]
    }
]"#;

        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 1);
        assert!(questions[0].text.contains("\\\\[6pt]"));
        assert!(!questions[0].text.contains("$$6pt]"));
    }

    #[test]
    fn test_preserve_double_backslash_spacing_command_4pt() {
        let input = r#"[
    {
        "text": "$$\\begin{aligned}a&=b,\\\\[4pt]c&=d\\end{aligned}$$",
        "answers": [
            {"text": "A", "is_correct": true}
        ]
    }
]"#;

        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 1);
        assert!(questions[0].text.contains("\\\\[4pt]"));
        assert!(!questions[0].text.contains("$$4pt]"));
    }

    #[test]
    fn test_convert_single_backslash_display_delimiters_to_dollars() {
        let input = r#"[
    {
        "text": "Before \\[ x^2 + 1 \\] after",
        "answers": [
            {"text": "A", "is_correct": true}
        ]
    }
]"#;

        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 1);
        assert!(questions[0].text.contains("Before $$x^2 + 1$$ after"));
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
        // LLMs often generate invalid JSON escapes like \(, \), \[, \] for LaTeX.
        // sanitize_json_string fixes these, then normalize_escaped_math_and_newlines
        // converts \(...\) → $...$ and \[...\] → $$...$$
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
        // \(...\) delimiters are normalised to $...$ by the pipeline
        assert!(questions[0].text.contains("$f(x)=x^2$"));
        // \[...\] delimiters are normalised to $$...$$
        assert!(questions[0].text.contains("$$0,2$$"));
    }

    // -------------------------------------------------------------------------
    // escape_json_string
    // -------------------------------------------------------------------------

    #[test]
    fn test_escape_json_string_backslash() {
        assert_eq!(escape_json_string(r"a\b"), r"a\\b");
    }

    #[test]
    fn test_escape_json_string_double_quote() {
        assert_eq!(escape_json_string(r#"say "hello""#), r#"say \"hello\""#);
    }

    #[test]
    fn test_escape_json_string_newline() {
        assert_eq!(escape_json_string("line1\nline2"), r"line1\nline2");
    }

    #[test]
    fn test_escape_json_string_tab() {
        assert_eq!(escape_json_string("col1\tcol2"), r"col1\tcol2");
    }

    #[test]
    fn test_escape_json_string_carriage_return() {
        assert_eq!(escape_json_string("a\rb"), r"a\rb");
    }

    #[test]
    fn test_escape_json_string_no_special_chars() {
        assert_eq!(escape_json_string("hello world"), "hello world");
    }

    // -------------------------------------------------------------------------
    // truncate
    // -------------------------------------------------------------------------

    #[test]
    fn test_truncate_short_string() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_exact_length() {
        assert_eq!(truncate("hello", 5), "hello");
    }

    #[test]
    fn test_truncate_long_string() {
        assert_eq!(truncate("hello world", 5), "hello...");
    }

    #[test]
    fn test_truncate_empty_string() {
        assert_eq!(truncate("", 5), "");
    }

    // -------------------------------------------------------------------------
    // parse_llm_response edge cases
    // -------------------------------------------------------------------------

    #[test]
    fn test_parse_returns_error_on_empty_input() {
        let result = parse_llm_response("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_returns_error_on_prose_only() {
        let result = parse_llm_response("Sorry, I cannot generate that.");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_returns_error_on_empty_array() {
        // A JSON array that contains no question objects should fail
        let result = parse_llm_response("[]");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_assigns_sequential_ids() {
        let input = r#"[
            {"text":"Q1","answers":[{"text":"A","is_correct":true}]},
            {"text":"Q2","answers":[{"text":"B","is_correct":true}]},
            {"text":"Q3","answers":[{"text":"C","is_correct":true}]}
        ]"#;
        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions[0].id, "q1");
        assert_eq!(questions[1].id, "q2");
        assert_eq!(questions[2].id, "q3");
    }

    // -------------------------------------------------------------------------
    // normalize_escaped_math_and_newlines
    // -------------------------------------------------------------------------

    #[test]
    fn test_normalize_inline_math_delimiters() {
        // \( ... \) → $ ... $
        let input = r"\(x^2\)";
        let output = normalize_escaped_math_and_newlines(input);
        assert_eq!(output, "$x^2$");
    }

    #[test]
    fn test_normalize_display_math_delimiters() {
        // \[ ... \] → $$ ... $$
        let input = r"\[E=mc^2\]";
        let output = normalize_escaped_math_and_newlines(input);
        assert_eq!(output, "$$E=mc^2$$");
    }

    #[test]
    fn test_normalize_double_backslash_n_is_newline() {
        // \\n (two backslashes then n) → newline, unless it starts a LaTeX command
        let chars: Vec<char> = vec!['\\', '\\', 'n'];
        let input: String = chars.into_iter().collect();
        let output = normalize_escaped_math_and_newlines(&input);
        assert_eq!(output, "\n");
    }

    #[test]
    fn test_normalize_double_backslash_newline_exception_newline_cmd() {
        // \\newline is in the exception list, so the \\n must NOT be collapsed to a newline.
        // Input string: two backslashes followed by "newline" (9 chars total)
        let input = "\\\\newline";
        let output = normalize_escaped_math_and_newlines(input);
        // The sequence is preserved as-is: \\newline
        assert_eq!(output, "\\\\newline");
    }

    #[test]
    fn test_normalize_passthrough_regular_text() {
        let input = "no special chars here";
        assert_eq!(normalize_escaped_math_and_newlines(input), input);
    }
}
