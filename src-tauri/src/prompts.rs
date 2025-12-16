//! Prompt templates and response parsing for LLM interactions

use crate::{GenerationRequest, Question, QuestionBankEntry};

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

**Step 1: Plan & Solve (Do this work mentally/on scratch paper - don't write it in your response)**
- Decide what concept you'll test
- Write the code you'll use
- TRACE through it step-by-step and calculate the CORRECT answer
- Double-check your calculation - verify it's right
- Think through common student errors for this type of problem

**Step 2: Design Distractors (Still mental work)**
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
    "stem": "Question text here (you can use markdown for formatting like **bold** or `code`)",
    "code": "// Optional Java code block\npublic static void main(String[] args) {{\n    System.out.println(\"Hello\");\n}}",
    "answers": [
      {{"text": "Answer text (use markdown like `42` for code)", "is_correct": false, "explanation": "Why this is wrong"}},
      {{"text": "Another answer", "is_correct": true, "explanation": "Why this is correct"}},
      {{"text": "Third answer", "is_correct": false, "explanation": "Common misconception"}},
      {{"text": "Fourth answer", "is_correct": false, "explanation": "Off-by-one error"}}
    ],
    "explanation": "Step-by-step walkthrough of how to arrive at the correct answer",
    "distractors": "Analysis of why each wrong answer is tempting and what misconception leads to it"
  }}
]
```

**Field Guidelines:**
- `stem`: The question text (markdown supported for formatting)
- `code`: Optional Java code snippet (plain text, not wrapped in ```java```)
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
{notes}

Generate a JSON array with {count} question(s) now:"#,
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
        r#"You are an expert AP Computer Science A question writer with strong analytical skills.

**CRITICAL: Calculate your answer completely before writing the question.**

Generate a NEW multiple choice question to replace this one:

**Current question to replace:**
{current}
{context}{example}

**WORKFLOW (DO THIS IN ORDER):**

**Step 1: Solve First (Mental work - don't write this in your response)**
- Decide on the concept to test (keep similar topic/difficulty)
- Create the code or scenario
- TRACE through completely and calculate the CORRECT answer
- Double-check your math
- Identify 3-4 student misconceptions that would lead to wrong answers

**Step 2: Write the Question (Now output JSON)**
- Write stem and code
- Write ALL answer choices (correct + distractors) with explanations
- Write the step-by-step explanation
- Write the distractors analysis

**Requirements:**
- Keep similar topic and difficulty but make it DIFFERENT from the original
- Exactly 4 answer choices
- Each wrong answer must exploit a specific student misconception
- Include code if appropriate
- VERIFY your calculations are correct before committing to an answer
- If you discover an error while writing, start that question over with different code

**Output format (JSON):**

Return a JSON array with ONE question object:

```json
[
  {{
    "stem": "New question text (markdown supported)",
    "code": "// Optional Java code (plain text, no ```java``` wrapper)",
    "answers": [
      {{"text": "Answer 1", "is_correct": false, "explanation": "Why wrong"}},
      {{"text": "Answer 2", "is_correct": true, "explanation": "Why correct"}},
      {{"text": "Answer 3", "is_correct": false, "explanation": "Common error"}},
      {{"text": "Answer 4", "is_correct": false, "explanation": "Misconception"}}
    ],
    "explanation": "Step-by-step solution",
    "distractors": "Analysis of why wrong answers are tempting"
  }}
]
```

Return ONLY the JSON array, no additional text. Generate the replacement question:"#,
        current = current.content,
        context = context_section,
        example = example_section,
    )
}

/// Parse LLM response into Question objects
pub fn parse_llm_response(response: &str) -> Result<Vec<Question>, String> {
    eprintln!("DEBUG: Parsing LLM response, length = {}", response.len());

    // Try to parse as JSON first
    let trimmed = response.trim();

    // Find JSON array boundaries
    let json_start = trimmed.find('[').ok_or("No JSON array found in response")?;
    let json_end = trimmed
        .rfind(']')
        .ok_or("No closing bracket found in JSON")?;
    let json_str = &trimmed[json_start..=json_end];

    eprintln!("DEBUG: Extracted JSON string, length = {}", json_str.len());

    // Parse JSON
    let questions: Vec<Question> = serde_json::from_str(json_str).map_err(|e| {
        format!(
            "Failed to parse JSON response: {}. JSON: {}",
            e,
            &json_str[..json_str.len().min(500)]
        )
    })?;

    if questions.is_empty() {
        return Err("No questions found in JSON response".to_string());
    }

    eprintln!(
        "DEBUG: Successfully parsed {} questions from JSON",
        questions.len()
    );

    // Assign IDs and ensure backward compatibility by populating content field
    let mut result = Vec::new();
    for (i, mut q) in questions.into_iter().enumerate() {
        q.id = format!("q{}", i + 1);

        // Populate legacy content field for backward compatibility
        if q.content.is_empty() {
            let mut content = q.stem.clone();
            if let Some(code) = &q.code {
                content.push_str("\n\n```java\n");
                content.push_str(code);
                content.push_str("\n```");
            }
            q.content = content;
        }

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
    "stem": "What is returned by this code?",
    "code": "return 5 + 3;",
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
        assert_eq!(questions[0].stem, "What is returned by this code?");
        assert_eq!(questions[0].code, Some("return 5 + 3;".to_string()));
    }

    #[test]
    fn test_parse_multiple_questions() {
        let input = r#"[
  {
    "stem": "Question 1",
    "answers": [
      {"text": "A", "is_correct": true},
      {"text": "B", "is_correct": false}
    ]
  },
  {
    "stem": "Question 2",
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
}
