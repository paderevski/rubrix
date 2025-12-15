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

**Output Format (Markdown):**

For each question, use this exact format:

---

# Question [number]

[Question stem - what are you asking?]

```java
// Include code if appropriate for the question type
// Keep code concise (under 15 lines)
```

## Solution

**Correct Answer Explanation:** [Work through the problem step-by-step. Trace code execution, show calculations. Arrive at the CORRECT ANSWER VALUE at the end of this section.]

**Distractor Analysis:**
Now create 3-4 wrong answers based on these common student errors:
- [Describe misconception 1]: This leads to answer [wrong value]
- [Describe misconception 2]: This leads to answer [wrong value]
- [Describe misconception 3]: This leads to answer [wrong value]
- [Describe misconception 4, if needed]: This leads to answer [wrong value]

## Choices

Now format the correct answer and distractors as multiple choice options. Put them in random order (correct answer should NOT always be 'a'):

a. [One answer - could be correct or distractor]
b. [One answer - could be correct or distractor]
c. [One answer - could be correct or distractor]
d. [One answer - could be correct or distractor]
e. [One answer - could be correct or distractor, if 5 choices needed]

---
**Correct Answer:** [letter corresponding to the correct value you calculated above]

---

(Repeat for Question 2, Question 3, etc. if count > 1)

**Important Rules:**
- Work out the correct answer completely BEFORE writing the question
- Verify your explanation matches the answer letter you chose
- Use standard answer labels: a. b. c. d. e.
- Each distractor must come from a real student error pattern (off-by-one, pass-by-value confusion, wrong loop bounds, etc.)
- Code must be syntactically correct Java
- Use backticks for inline code in answers like `42` or `"hello"`
- Your explanation must be clear, accurate, and consistent with the correct answer
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
        r#"You are an expert AP Computer Science A question writer with strong analytical skills.

**CRITICAL: Calculate your answer completely before writing the question.**

Generate a NEW multiple choice question to replace this one:

**Current question to replace:**
{current}
{context}{example}

**Requirements:**
- Keep similar topic and difficulty but make it DIFFERENT
- Exactly 4 answer choices (a, b, c, d)
- Each wrong answer should exploit a specific student misconception
- Include code if appropriate
- VERIFY your calculations are correct before committing to an answer
- If you make an error, start over with different code

**Output format:**

# Question 1

[New question stem]

```java
// code if needed
```

## Solution

**Correct Answer Explanation:** [Work through the problem step-by-step to find the correct answer]

**Distractor Analysis:**
Create 3 wrong answers based on common errors:
- [Misconception 1]: Leads to answer [wrong value]
- [Misconception 2]: Leads to answer [wrong value]
- [Misconception 3]: Leads to answer [wrong value]

## Choices

a. [One answer]
b. [One answer]
c. [One answer]
d. [One answer]

---
**Correct Answer:** [letter]

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

    // Split by numbered question markers - "# Question 1", "# Question 2", etc. or legacy "1."
    // Don't split on "## Question" which is a subsection header
    let question_start_re = Regex::new(r"\n(?:#\s*Question\s+\d+|\d+\.)\s*").unwrap();
    let mut blocks: Vec<String> = Vec::new();
    let mut last_end = 0;

    for mat in question_start_re.find_iter(&content) {
        if last_end > 0 && last_end < mat.start() {
            let block = &content[last_end..mat.start()];
            if !block.trim().is_empty() {
                eprintln!("DEBUG: Found question block (length: {})", block.len());
                blocks.push(block.trim().to_string());
            }
        }
        last_end = mat.end();
    }

    // Don't forget the last block
    if last_end < content.len() {
        let block = &content[last_end..];
        if !block.trim().is_empty() {
            eprintln!(
                "DEBUG: Found final question block (length: {})",
                block.len()
            );
            blocks.push(block.trim().to_string());
        }
    }

    eprintln!("DEBUG: Total question blocks found: {}", blocks.len());

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
    eprintln!("DEBUG parse_single_question: Input length = {}", text.len());
    eprintln!(
        "DEBUG parse_single_question: First 100 chars = {}",
        &text.chars().take(100).collect::<String>()
    );

    // Remove any leading question marker - "# Question 1", "## Question", or "1."
    let num_re = Regex::new(r"^(?:#\s*Question\s+\d+|#{1,2}\s*Question|\d+\.)\s*").unwrap();
    let text = num_re.replace(text, "").to_string();

    // Find where answers start - look for "## Choices" or "# Choices" or fallback to "a. "
    let choices_marker = Regex::new(r"\n#{1,2}\s*Choices\s*\n").unwrap();
    let solution_marker = Regex::new(r"\n#{1,2}\s*Solution\s*\n").unwrap();
    let answer_re = Regex::new(r"\n\s*a\.\s+").unwrap();

    let answer_start = if let Some(m) = choices_marker.find(&text) {
        eprintln!("DEBUG: Found ## Choices marker at position {}", m.start());
        // If we have "## Choices", answers start after it
        m.end()
    } else if let Some(m) = answer_re.find(&text) {
        eprintln!(
            "DEBUG: Found a. marker at position {} (fallback)",
            m.start()
        );
        // Fallback to old format
        m.start()
    } else {
        eprintln!("DEBUG: No answer markers found!");
        return None;
    };

    // Content is everything before "## Solution" or "## Choices" (whichever comes first)
    let content_end = if let Some(m) = solution_marker.find(&text) {
        eprintln!("DEBUG: Found ## Solution marker at position {}", m.start());
        m.start()
    } else if let Some(m) = choices_marker.find(&text) {
        eprintln!(
            "DEBUG: Found ## Choices marker at position {} (for content_end)",
            m.start()
        );
        m.start()
    } else {
        answer_start
    };
    let content = text[..content_end].trim().to_string();
    eprintln!("DEBUG: Content extracted, length = {}", content.len());

    // Find where distractor analysis starts (to exclude from answers)
    let analysis_re = Regex::new(r"\n---\s*\n\*?\*?Correct Answer").unwrap();
    let answers_end = analysis_re
        .find(&text)
        .map(|m| {
            eprintln!("DEBUG: Found --- Correct Answer at position {}", m.start());
            m.start()
        })
        .unwrap_or(text.len());

    // Extract the correct answer letter from metadata
    let correct_re = Regex::new(r"\*?\*?Correct Answer:?\*?\*?\s*([a-eA-E])").unwrap();
    let correct_letter = correct_re
        .captures(&text)
        .and_then(|c| c.get(1))
        .map(|m| {
            let letter = m.as_str().to_lowercase();
            eprintln!("DEBUG: Correct answer letter = {}", letter);
            letter
        })
        .unwrap_or_else(|| {
            eprintln!("DEBUG: No correct answer found, defaulting to 'a'");
            "a".to_string()
        });

    // Extract answers section (between answer_start and analysis)
    let answers_section = &text[answer_start..answers_end];
    eprintln!("DEBUG: Answers section length = {}", answers_section.len());
    eprintln!(
        "DEBUG: Answers section preview: {}",
        &answers_section.chars().take(200).collect::<String>()
    );

    let answers = parse_answers(answers_section, &correct_letter);
    eprintln!("DEBUG: Parsed {} answers", answers.len());

    if answers.is_empty() {
        eprintln!("DEBUG: No answers parsed, returning None");
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

    eprintln!(
        "DEBUG parse_answers: Looking for correct_letter = '{}'",
        correct_letter
    );

    // Match a. b. c. d. e. or A. B. C. D. E.
    let answer_marker = Regex::new(r"(?m)^([a-eA-E])\.\s+(.*)$").unwrap();

    for cap in answer_marker.captures_iter(text) {
        // Save previous answer if exists
        if let (Some(letter), Some(txt)) = (&current_letter, &current_text) {
            let is_correct = letter.to_lowercase() == correct_letter;
            eprintln!(
                "DEBUG parse_answers: Letter '{}' -> is_correct={}",
                letter, is_correct
            );
            answers.push(Answer {
                text: txt.trim().to_string(),
                is_correct,
            });
        }

        // Start new answer
        current_letter = Some(cap[1].to_string());
        current_text = Some(cap[2].to_string());
    }

    // Don't forget the last answer
    if let (Some(letter), Some(txt)) = (current_letter, current_text) {
        let is_correct = letter.to_lowercase() == correct_letter;
        eprintln!(
            "DEBUG parse_answers: Letter '{}' (last) -> is_correct={}",
            letter, is_correct
        );
        answers.push(Answer {
            text: txt.trim().to_string(),
            is_correct,
        });
    }

    eprintln!(
        "DEBUG parse_answers: Total {} answers created",
        answers.len()
    );
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
        let input = r#"# Question 1

What is returned by this code?

```java
return 5 + 3;
```

## Solution

**Correct Answer Explanation:** Simple addition of 5 + 3 equals 8.

**Distractor Analysis:**
- String concatenation: Would give `"53"`
- Compilation error: Incorrect syntax assumption

## Choices

a. `8`
b. `53`
c. `"53"`
d. Error

---
**Correct Answer:** a"#;

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
        let input = r#"# Question 1

Which is a prime number?

## Solution

**Correct Answer Explanation:** 7 is the only prime number in this list.

**Distractor Analysis:**
- 4 is divisible by 2
- 6 is divisible by 2 and 3
- 8 is divisible by 2 and 4

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
        let input = r#"# Question 1

Select the correct statement.

## Solution

**Correct Answer Explanation:** Option d is the correct statement.

**Distractor Analysis:**
- Option a: Incorrect assumption
- Option b: Wrong reasoning
- Option c: Misunderstands concept
- Option e: Confused with different topic

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
