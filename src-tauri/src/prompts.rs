//! Prompt templates and response parsing for LLM interactions

use crate::{Answer, GenerationRequest, Question};
use regex::Regex;

/// Build the prompt for generating multiple questions
pub fn build_generation_prompt(request: &GenerationRequest, examples: &[Question]) -> String {
    let topics_str = request.topics.join(", ");
    
    let difficulty_desc = match request.difficulty.as_str() {
        "easy" => "straightforward questions testing basic understanding",
        "medium" => "moderate questions requiring application of concepts",
        "hard" => "challenging questions requiring deep understanding and multi-step reasoning",
        _ => "moderate difficulty questions",
    };
    
    let examples_str = if examples.is_empty() {
        String::new()
    } else {
        let formatted: Vec<String> = examples.iter().map(format_question_for_prompt).collect();
        format!(
            "\n\nHere are some example questions in the correct format:\n\n{}",
            formatted.join("\n\n")
        )
    };
    
    let notes_str = match &request.notes {
        Some(notes) if !notes.trim().is_empty() => {
            format!("\n\nAdditional instructions from the teacher:\n{}", notes)
        }
        _ => String::new(),
    };

    format!(
        r#"You are an expert AP Computer Science A teacher creating multiple choice questions.

Generate exactly {count} multiple choice questions about the following topics: {topics}

Requirements:
- Difficulty level: {difficulty}
- Each question should have exactly 4 answer choices
- The FIRST answer choice must be the CORRECT answer
- All other answer choices should be plausible but incorrect
- Include Java code snippets where appropriate
- Questions should be suitable for AP Computer Science A students

Format each question EXACTLY like this:

1. Question text here.

```java
// Code block if needed (optional)
public void example() {{
    // code here
}}
```

a. Correct answer (always first)
a. Wrong answer 2
a. Wrong answer 3
a. Wrong answer 4

Important formatting rules:
- Number questions sequentially (1., 2., 3., etc.)
- Use ```java for code blocks
- Use backticks for inline code like `variableName`
- ALL answer choices start with "a." (the first one is correct)
- Separate questions with blank lines
{examples}
{notes}

Now generate {count} questions:"#,
        count = request.count,
        topics = topics_str,
        difficulty = difficulty_desc,
        examples = examples_str,
        notes = notes_str,
    )
}

/// Build prompt for regenerating a single question
pub fn build_regenerate_prompt(current: &Question, all_questions: &[Question]) -> String {
    let context: Vec<String> = all_questions
        .iter()
        .filter(|q| q.id != current.id)
        .take(3)
        .map(|q| format!("- {}", truncate(&q.content, 50)))
        .collect();
    
    let context_str = if context.is_empty() {
        String::new()
    } else {
        format!("\n\nOther questions in this set (for variety):\n{}", context.join("\n"))
    };

    format!(
        r#"You are an expert AP Computer Science A teacher.

Generate a NEW multiple choice question to replace this one:

Current question: {}

Requirements:
- Keep similar topic/difficulty but make it DIFFERENT
- Exactly 4 answer choices
- FIRST answer must be CORRECT
- Include code if appropriate
{}

Format EXACTLY like this:

1. Question text here.

```java
// optional code
```

a. Correct answer
a. Wrong answer
a. Wrong answer
a. Wrong answer

Generate the replacement question:"#,
        current.content,
        context_str
    )
}

/// Format a question for inclusion in a prompt as an example
fn format_question_for_prompt(q: &Question) -> String {
    let answers_str: Vec<String> = q.answers.iter().map(|a| format!("a. {}", a.text)).collect();
    
    format!(
        "{}\n\n{}",
        q.content,
        answers_str.join("\n")
    )
}

/// Parse LLM response into Question objects
pub fn parse_llm_response(response: &str) -> Result<Vec<Question>, String> {
    let mut questions = Vec::new();
    
    // Prepend newline to handle first question uniformly
    let content = format!("\n{}", response.trim());
    
    // Split by question numbers (manually, since Rust regex doesn't support lookahead)
    let question_start_re = Regex::new(r"\n(\d+\.\s+)").unwrap();
    let mut blocks: Vec<String> = Vec::new();
    let mut last_end = 0;
    
    for mat in question_start_re.find_iter(&content) {
        if last_end > 0 && last_end < mat.start() {
            let block = &content[last_end..mat.start()];
            if !block.trim().is_empty() {
                blocks.push(block.trim().to_string());
            }
        }
        // Next block starts after the \n
        last_end = mat.start() + 1;
    }
    
    // Don't forget the last block
    if last_end < content.len() {
        let block = &content[last_end..];
        if !block.trim().is_empty() {
            blocks.push(block.trim().to_string());
        }
    }
    
    let num_re = Regex::new(r"^\d+\.\s+").unwrap();
    
    for block in &blocks {
        let block = block.trim();
        if block.is_empty() || !num_re.is_match(block) {
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
    // Extract question number and remove it
    let num_re = Regex::new(r"^(\d+)\.\s+").unwrap();
    let text = num_re.replace(text, "").to_string();
    
    // Find where answers start
    let answer_re = Regex::new(r"\n\s*a\.\s+").unwrap();
    let answer_start = answer_re.find(&text)?;
    
    // Content is everything before answers (including code blocks, tables, etc.)
    let content = text[..answer_start.start()].trim().to_string();
    
    // Extract answers
    let answers_section = &text[answer_start.start()..];
    let answers = parse_answers(answers_section);
    
    if answers.is_empty() {
        return None;
    }
    
    Some(Question {
        id: String::new(), // Will be set by caller
        content,
        answers,
    })
}

/// Parse answer choices from text
fn parse_answers(text: &str) -> Vec<Answer> {
    let mut answers = Vec::new();
    let mut current_text: Option<String> = None;
    
    for line in text.lines() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("a.") {
            // Save previous answer
            if let Some(text) = current_text.take() {
                let is_correct = answers.is_empty(); // First answer is correct
                answers.push(Answer { text, is_correct });
            }
            // Start new answer
            current_text = Some(trimmed[2..].trim().to_string());
        } else if let Some(ref mut text) = current_text {
            // Continuation line
            if !trimmed.is_empty() {
                text.push(' ');
                text.push_str(trimmed);
            }
        }
    }
    
    // Don't forget last answer
    if let Some(text) = current_text {
        let is_correct = answers.is_empty();
        answers.push(Answer { text, is_correct });
    }
    
    answers
}

/// Truncate a string to a maximum length
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
    fn test_parse_simple_question() {
        let input = r#"1. What is 2 + 2?

a. 4
a. 3
a. 5
a. 22"#;
        
        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 1);
        assert_eq!(questions[0].content, "What is 2 + 2?");
        assert_eq!(questions[0].answers.len(), 4);
        assert!(questions[0].answers[0].is_correct);
        assert!(!questions[0].answers[1].is_correct);
    }

    #[test]
    fn test_parse_question_with_code() {
        let input = r#"1. What does this print?

```java
System.out.println("Hello");
```

a. Hello
a. hello
a. HELLO
a. Nothing"#;
        
        let questions = parse_llm_response(input).unwrap();
        assert_eq!(questions.len(), 1);
        assert!(questions[0].content.contains("println"));
        assert!(questions[0].content.contains("```java"));
    }
}
