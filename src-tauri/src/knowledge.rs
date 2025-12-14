//! Knowledge base for storing and retrieving example questions

use crate::{Answer, Question, TopicInfo};
use regex::Regex;
use rust_embed::RustEmbed;
use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "knowledge/"]
struct KnowledgeAssets;

/// Knowledge base containing example questions organized by topic
pub struct KnowledgeBase {
    topics: Vec<TopicInfo>,
    questions: HashMap<String, Vec<Question>>,
}

impl KnowledgeBase {
    /// Load knowledge base from embedded files
    pub fn load() -> Self {
        let mut questions: HashMap<String, Vec<Question>> = HashMap::new();
        
        // Define available topics
        let topic_definitions = vec![
            ("arrays", "Arrays", "Array declaration, initialization, traversal, and manipulation"),
            ("recursion", "Recursion", "Recursive methods, base cases, and tracing recursive calls"),
            ("strings", "Strings", "String methods, manipulation, and comparison"),
            ("classes", "Classes & Objects", "Class design, constructors, instance variables, and methods"),
            ("inheritance", "Inheritance", "Subclasses, method overriding, and polymorphism"),
            ("arraylist", "ArrayList", "ArrayList operations, wrapper classes, and traversal"),
            ("2darrays", "2D Arrays", "Two-dimensional array declaration and traversal"),
            ("sorting", "Sorting & Searching", "Selection sort, insertion sort, and binary search"),
        ];
        
        // Load questions from embedded files
        for (id, _, _) in &topic_definitions {
            let filename = format!("{}.txt", id);
            if let Some(file) = KnowledgeAssets::get(&filename) {
                let content = String::from_utf8_lossy(&file.data);
                let parsed = parse_question_bank(&content);
                questions.insert(id.to_string(), parsed);
            } else {
                // Empty placeholder if file doesn't exist yet
                questions.insert(id.to_string(), Vec::new());
            }
        }
        
        // Build topic info with counts
        let topics: Vec<TopicInfo> = topic_definitions
            .iter()
            .map(|(id, name, desc)| {
                let count = questions.get(*id).map(|q| q.len()).unwrap_or(0);
                TopicInfo {
                    id: id.to_string(),
                    name: name.to_string(),
                    description: desc.to_string(),
                    example_count: count,
                }
            })
            .collect();
        
        KnowledgeBase { topics, questions }
    }
    
    /// Get all available topics
    pub fn get_topics(&self) -> Vec<TopicInfo> {
        self.topics.clone()
    }
    
    /// Get example questions for specified topics
    pub fn get_examples(&self, topic_ids: &[String], max_per_topic: usize) -> Vec<Question> {
        let mut examples = Vec::new();
        
        for topic_id in topic_ids {
            if let Some(topic_questions) = self.questions.get(topic_id) {
                let take_count = max_per_topic.min(topic_questions.len());
                examples.extend(topic_questions.iter().take(take_count).cloned());
            }
        }
        
        examples
    }
}

/// Parse a question bank file in our standard format
fn parse_question_bank(content: &str) -> Vec<Question> {
    let mut questions = Vec::new();
    
    // Skip title line if present
    let content = if content.trim_start().starts_with("Title:") {
        let lines: Vec<&str> = content.lines().collect();
        let start = lines.iter().position(|l| l.trim().starts_with("Title:")).unwrap_or(0);
        lines[start + 1..].join("\n")
    } else {
        content.to_string()
    };
    
    // Prepend newline to handle first question
    let content = format!("\n{}", content.trim());
    
    // Split by question numbers
    let question_re = Regex::new(r"\n(\d+\.\s+)").unwrap();
    let mut last_end = 0;
    let mut blocks = Vec::new();
    
    for mat in question_re.find_iter(&content) {
        if last_end > 0 {
            let block = &content[last_end..mat.start()];
            if !block.trim().is_empty() {
                blocks.push(block.trim().to_string());
            }
        }
        last_end = mat.start() + 1;
    }
    
    // Last block
    if last_end < content.len() {
        let block = &content[last_end..];
        if !block.trim().is_empty() {
            blocks.push(block.trim().to_string());
        }
    }
    
    let num_re = Regex::new(r"^\d+\.").unwrap();
    
    for (i, block) in blocks.iter().enumerate() {
        if !num_re.is_match(block) {
            continue;
        }
        
        if let Some(q) = parse_question(block, i + 1) {
            questions.push(q);
        }
    }
    
    questions
}

/// Parse a single question from text
fn parse_question(text: &str, index: usize) -> Option<Question> {
    // Remove question number
    let num_re = Regex::new(r"^\d+\.\s+").unwrap();
    let text = num_re.replace(text, "").to_string();
    
    // Extract code block if present
    let code_re = Regex::new(r"(?s)```(\w+)?\n(.*?)```").unwrap();
    let mut code: Option<String> = None;
    let mut text_without_code = text.clone();
    
    if let Some(caps) = code_re.captures(&text) {
        code = Some(caps.get(2)?.as_str().trim().to_string());
        text_without_code = code_re.replace(&text, "__CODE__").to_string();
    }
    
    // Find answer section
    let answer_re = Regex::new(r"\n\s*a\.\s+").unwrap();
    let answer_start = answer_re.find(&text_without_code)?;
    
    // Question text
    let question_text = text_without_code[..answer_start.start()]
        .replace("__CODE__", "")
        .trim()
        .to_string();
    
    // Parse answers
    let answers_section = &text_without_code[answer_start.start()..];
    let answers = parse_answers(answers_section);
    
    if answers.is_empty() {
        return None;
    }
    
    Some(Question {
        id: format!("ex{}", index),
        text: question_text,
        code,
        answers,
    })
}

/// Parse answer choices
fn parse_answers(text: &str) -> Vec<Answer> {
    let mut answers = Vec::new();
    let mut current: Option<String> = None;
    
    for line in text.lines() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("a.") {
            if let Some(text) = current.take() {
                answers.push(Answer {
                    text,
                    is_correct: answers.is_empty(),
                });
            }
            current = Some(trimmed[2..].trim().to_string());
        } else if let Some(ref mut text) = current {
            if !trimmed.is_empty() {
                text.push('\n');
                text.push_str(trimmed);
            }
        }
    }
    
    if let Some(text) = current {
        answers.push(Answer {
            text,
            is_correct: answers.is_empty(),
        });
    }
    
    answers
}
