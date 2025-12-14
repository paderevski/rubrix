//! Prompt templates and response parsing for LLM interactions

use crate::{Answer, GenerationRequest, Question, QuestionBankEntry};
use regex::Regex;

/// Build the prompt for generating multiple questions using JSON examples
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

    // Format JSON examples with pedagogically useful fields
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
            format!("\n\n**Additional Instructions:**\n{}", notes)
        }
        _ => String::new(),
    };

    format!(
        r#"You are an expert AP Computer Science A question writer. Generate NEW multiple choice questions based on the style and quality of these examples.

**Target Topic(s):** {topics}
**Target Difficulty:** {difficulty}
**Number of Questions:** {count}

---

## Reference Examples (JSON format)

Study these examples carefully. Pay special attention to:
- How the `distractors` field shows WHY each wrong answer is tempting
- The `common_errors` that students make
- The relationship between `difficulty` and `cognitive_level`

{examples}

---

## Your Task

Generate {count} NEW question(s) that:
1. Test the specified topic(s) at the target difficulty
2. Use DIFFERENT code and scenarios than the examples
3. Each wrong answer must exploit a specific student misconception
4. Match the quality and style shown in the examples

**Output Format (Markdown):**

For each question, use this exact format:

## Question

[Question stem - what are you asking?]

```java
// Include code if appropriate for the question type
// Keep code concise (under 15 lines)
```

## Choices

a. [First answer option]
b. [Second answer option]
c. [Third answer option]
d. [Fourth answer option]
e. [Fifth answer option if present]

---
**Correct Answer:** [letter]
**Explanation:** [Step-by-step explanation of why the correct answer is right]
**Distractor Analysis:**
- [wrong letter]: [What misconception or error leads a student to this answer]
- [wrong letter]: [What misconception or error leads a student to this answer]
- [wrong letter]: [What misconception or error leads a student to this answer]
- [wrong letter]: [What misconception or error leads a student to this answer, if 5 choices present]

---

(Then question 2, 3, etc. if count > 1)

**Important Rules:**
- Use standard answer labels: a. b. c. d. e.
- Indicate the correct answer in the "Correct Answer" line
- Each distractor must come from a real student error pattern (off-by-one, pass-by-value confusion, wrong loop bounds, etc.)
- Code must be syntactically correct Java
- Use backticks for inline code in answers like `42` or `"hello"`
{notes}

Generate {count} question(s) now:"#,
        topics = topics_str,
        difficulty = difficulty_desc,
        count = request.count,
        examples = examples_str,
        notes = notes_str,
    )
}

/// Format a question bank entry as JSON with only pedagogically useful fields
fn format_example_as_json(q: &QuestionBankEntry) -> String {
    // Build a clean JSON representation with the useful fields
    let options_json: Vec<String> = q
        .options
        .iter()
        .map(|opt| {
            format!(
                r#"    {{"id": "{}", "text": "{}", "is_correct": {}}}"#,
                opt.id,
                escape_json_string(&opt.text),
                opt.is_correct
            )
        })
        .collect();

    let mistakes_json: Vec<String> = q
        .distractors
        .common_mistakes
        .iter()
        .map(|m| {
            format!(
                r#"      {{"option_id": "{}", "misconception": "{}"}}"#,
                m.option_id,
                escape_json_string(&m.misconception)
            )
        })
        .collect();

    let errors_json: Vec<String> = q
        .distractors
        .common_errors
        .iter()
        .map(|e| format!(r#""{}""#, e))
        .collect();

    let skills_json: Vec<String> = q.skills.iter().map(|s| format!(r#""{}""#, s)).collect();

    let code_field = match &q.code {
        Some(code) => format!(r#"  "code": "{}","#, escape_json_string(code)),
        None => String::new(),
    };

    format!(
        r#"{{
  "stem": "{stem}",
{code}
  "options": [
{options}
  ],
  "explanation": "{explanation}",
  "difficulty": "{difficulty}",
  "cognitive_level": "{cognitive_level}",
  "skills": [{skills}],
  "distractors": {{
    "common_mistakes": [
{mistakes}
    ],
    "common_errors": [{errors}]
  }}
}}"#,
        stem = escape_json_string(&q.stem),
        code = if code_field.is_empty() {
            String::new()
        } else {
            format!("{}\n", code_field)
        },
        options = options_json.join(",\n"),
        explanation = escape_json_string(&q.explanation),
        difficulty = q.difficulty,
        cognitive_level = q.cognitive_level,
        skills = skills_json.join(", "),
        mistakes = mistakes_json.join(",\n"),
        errors = errors_json.join(", "),
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
) -> String {
    let context_str: Vec<String> = context
        .iter()
        .filter(|q| q.id != current.id)
        .take(3)
        .map(|q| format!("- {}", truncate(&q.content, 50)))
        .collect();

    let context_section = if context_str.is_empty() {
        String::new()
    } else {
        format!(
            "\n\n**Other questions in this set (avoid duplicating):**\n{}",
            context_str.join("\n")
        )
    };

    // Include one example if available
    let example_section = if let Some(ex) = examples.first() {
        format!(
            "\n\n**Reference example for quality/style:**\n```json\n{}\n```",
            format_example_as_json(ex)
        )
    } else {
        String::new()
    };

    format!(
        r#"You are an expert AP Computer Science A teacher.

Generate a NEW multiple choice question to replace this one:

**Current question to replace:**
{current}
{context}{example}

**Requirements:**
- Keep similar topic and difficulty but make it DIFFERENT
- Exactly 4 answer choices (a, b, c, d)
- Each wrong answer should exploit a specific student misconception
- Include code if appropriate

**Output format:**

## Question

[New question stem]

```java
// code if needed
```

## Choices

a. [First answer]
b. [Second answer]
c. [Third answer]
d. [Fourth answer]

---
**Correct Answer:** [letter]
**Explanation:** [Why correct]
**Distractor Analysis:**
- [wrong letter]: [What error leads here]
- [wrong letter]: [What error leads here]
- [wrong letter]: [What error leads here]

Generate the replacement question:"#,
        current = current.content,
        context = context_section,
        example = example_section,
    )
}

/// Parse LLM response into Question objects
pub fn parse_llm_response(response: &str) -> Result<Vec<Question>, String> {
    let mut questions = Vec::new();

    // Prepend newline to handle first question uniformly
    let content = format!("\n{}", response.trim());

    // Split by question markers - handle both "## Question" and legacy "1." formats
    let question_start_re = Regex::new(r"\n(?:##\s*Question|\d+\.)\s*").unwrap();
    let mut blocks: Vec<String> = Vec::new();
    let mut last_end = 0;

    for mat in question_start_re.find_iter(&content) {
        if last_end > 0 && last_end < mat.start() {
            let block = &content[last_end..mat.start()];
            if !block.trim().is_empty() {
                blocks.push(block.trim().to_string());
            }
        }
        last_end = mat.end();
    }

    // Don't forget the last block
    if last_end < content.len() {
        let block = &content[last_end..];
        if !block.trim().is_empty() {
            blocks.push(block.trim().to_string());
        }
    }

    for block in &blocks {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        if let Some(q) = parse_single_question(block) {
            questions.push(q);
        }
    }

    if questions.is_empty() {
        // Try parsing as a single question if no split found
        if let Some(q) = parse_single_question(response.trim()) {
            questions.push(q);
        }
    }

    if questions.is_empty() {
        return Err("Failed to parse any questions from LLM response".to_string());
    }

    // Assign IDs
    for (i, q) in questions.iter_mut().enumerate() {
        q.id = format!("q{}", i + 1);
    }

    Ok(questions)
}

/// Parse a single question block
fn parse_single_question(text: &str) -> Option<Question> {
    // Remove any leading question marker
    let num_re = Regex::new(r"^(?:##\s*Question|\d+\.)\s*").unwrap();
    let text = num_re.replace(text, "").to_string();

    // Find where answers start - look for "## Choices" or fallback to "a. "
    let choices_marker = Regex::new(r"\n##\s*Choices\s*\n").unwrap();
    let answer_re = Regex::new(r"\n\s*a\.\s+").unwrap();

    let answer_start = if let Some(m) = choices_marker.find(&text) {
        // If we have "## Choices", answers start after it
        m.end()
    } else if let Some(m) = answer_re.find(&text) {
        // Fallback to old format
        m.start()
    } else {
        return None;
    };

    // Content is everything before answers (and before "## Choices" if present)
    let content_end = if let Some(m) = choices_marker.find(&text) {
        m.start()
    } else {
        answer_start
    };
    let content = text[..content_end].trim().to_string();

    // Find where distractor analysis starts (to exclude from answers)
    let analysis_re = Regex::new(r"\n---\s*\n\*?\*?Correct Answer").unwrap();
    let answers_end = analysis_re
        .find(&text)
        .map(|m| m.start())
        .unwrap_or(text.len());

    // Extract the correct answer letter from metadata
    let correct_re = Regex::new(r"\*?\*?Correct Answer:?\*?\*?\s*([a-eA-E])").unwrap();
    let correct_letter = correct_re
        .captures(&text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_lowercase())
        .unwrap_or_else(|| "a".to_string());

    // Extract answers section (between answer_start and analysis)
    let answers_section = &text[answer_start..answers_end];
    let answers = parse_answers(answers_section, &correct_letter);

    if answers.is_empty() {
        return None;
    }

    Some(Question {
        id: String::new(),
        content,
        answers,
    })
}

/// Parse answer choices from text
fn parse_answers(text: &str, correct_letter: &str) -> Vec<Answer> {
    let mut answers = Vec::new();
    let mut current_letter: Option<String> = None;
    let mut current_text: Option<String> = None;

    // Match a. b. c. d. e. or A. B. C. D. E.
    let answer_marker = Regex::new(r"(?m)^([a-eA-E])\.\s+(.*)$").unwrap();

    for cap in answer_marker.captures_iter(text) {
        // Save previous answer if exists
        if let (Some(letter), Some(txt)) = (&current_letter, &current_text) {
            answers.push(Answer {
                text: txt.trim().to_string(),
                is_correct: letter.to_lowercase() == correct_letter,
            });
        }

        // Start new answer
        current_letter = Some(cap[1].to_string());
        current_text = Some(cap[2].to_string());
    }

    // Don't forget the last answer
    if let (Some(letter), Some(txt)) = (current_letter, current_text) {
        answers.push(Answer {
            text: txt.trim().to_string(),
            is_correct: letter.to_lowercase() == correct_letter,
        });
    }

    answers
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
    fn test_parse_new_format() {
        let input = r#"## Question

What is returned by this code?

```java
return 5 + 3;
```

## Choices

a. `8`
b. `53`
c. `"53"`
d. Error

---
**Correct Answer:** a
**Explanation:** Simple addition returns 8."#;

        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 1);
        assert_eq!(questions[0].answers.len(), 4);
        assert!(questions[0].answers[0].is_correct);
    }

    #[test]
    fn test_parse_legacy_format() {
        let input = r#"1. What is returned by this code?

```java
return 5 + 3;
```

a. `8`
b. `53`
c. `"53"`
d. Error

---
**Correct Answer:** a
**Explanation:** Simple addition returns 8."#;

        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 1);
        assert!(questions[0].content.contains("```java"));
    }

    #[test]
    fn test_parse_correct_answer_not_a() {
        let input = r#"## Question

Which is a prime number?

## Choices

a. 4
b. 6
c. 7
d. 8

---
**Correct Answer:** c"#;

        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 1);
        assert!(!questions[0].answers[0].is_correct); // a is not correct
        assert!(!questions[0].answers[1].is_correct); // b is not correct
        assert!(questions[0].answers[2].is_correct); // c IS correct
        assert!(!questions[0].answers[3].is_correct); // d is not correct
    }

    #[test]
    fn test_parse_five_choices() {
        let input = r#"## Question

Select the correct statement.

## Choices

a. Wrong 1
b. Wrong 2
c. Wrong 3
d. Correct answer
e. Wrong 4

---
**Correct Answer:** d"#;

        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 1);
        assert_eq!(questions[0].answers.len(), 5);
        assert!(questions[0].answers[3].is_correct); // d is correct
    }
}
