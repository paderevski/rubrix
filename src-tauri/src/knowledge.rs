//! Knowledge base management - loads example questions for few-shot prompting

use crate::{Question, Answer, TopicInfo, QuestionBankEntry, QuestionBankOption, DistractorInfo, CommonMistake};
use regex::Regex;
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "knowledge/"]
struct KnowledgeAssets;

/// Full question bank JSON structure
#[derive(Debug, Deserialize)]
struct QuestionBankFile {
    questions: Vec<QuestionBankJsonEntry>,
}

/// Raw JSON entry from question-bank.json
#[derive(Debug, Deserialize)]
struct QuestionBankJsonEntry {
    id: String,
    difficulty: String,
    cognitive_level: String,
    content: QuestionContent,
    pedagogy: Pedagogy,
    distractors: DistractorsJson,
}

#[derive(Debug, Deserialize)]
struct QuestionContent {
    stem: String,
    code: Option<String>,
    options: Vec<OptionJson>,
    explanation: String,
}

#[derive(Debug, Deserialize)]
struct OptionJson {
    id: String,
    text: String,
    is_correct: bool,
}

#[derive(Debug, Deserialize)]
struct Pedagogy {
    topics: Vec<String>,
    skills: Vec<String>,
    #[serde(default)]
    ap_cs_topics: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct DistractorsJson {
    common_mistakes: Vec<CommonMistakeJson>,
    common_errors: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct CommonMistakeJson {
    option_id: String,
    misconception: String,
    #[serde(default)]
    reasoning: Option<String>,
}

pub struct KnowledgeBase {
    pub topics: Vec<TopicInfo>,
    /// Legacy text-based questions by topic
    pub text_questions: HashMap<String, Vec<Question>>,
    /// Rich JSON questions from question-bank.json
    pub bank_entries: Vec<QuestionBankEntry>,
}

impl KnowledgeBase {
    /// Load knowledge base from embedded files
    pub fn load() -> Self {
        // Topic definitions with mappings
        let topic_definitions = vec![
            ("recursion", "Recursion", "Recursive methods, base cases, and tracing", vec!["T009"]),
            ("arrays", "Arrays", "Array declaration, traversal, and manipulation", vec!["T006"]),
            ("arraylist", "ArrayList", "ArrayList operations and wrapper classes", vec!["T007"]),
            ("2darrays", "2D Arrays", "2D array traversal and manipulation", vec!["T008"]),
            ("strings", "Strings", "String methods and manipulation", vec!["T013"]),
            ("classes", "Classes", "Class design, constructors, and methods", vec!["T005"]),
            ("inheritance", "Inheritance", "Extends, super, and polymorphism", vec!["T016", "T017"]),
            ("sorting", "Sorting & Searching", "Selection sort, insertion sort, binary search", vec!["T010", "T011"]),
            ("iteration", "Iteration", "For loops, while loops, nested loops", vec!["T004"]),
            ("boolean", "Boolean Logic", "Boolean expressions and conditionals", vec!["T003"]),
        ];
        
        // Load legacy text-based questions
        let mut text_questions: HashMap<String, Vec<Question>> = HashMap::new();
        
        for (id, _, _, _) in &topic_definitions {
            let filename = format!("{}.txt", id);
            if let Some(file) = KnowledgeAssets::get(&filename) {
                let content = String::from_utf8_lossy(&file.data);
                let parsed = parse_text_question_bank(&content);
                text_questions.insert(id.to_string(), parsed);
            } else {
                text_questions.insert(id.to_string(), Vec::new());
            }
        }
        
        // Load JSON question bank
        let bank_entries = load_question_bank();
        
        // Build topic info with counts (combining both sources)
        let topics: Vec<TopicInfo> = topic_definitions
            .iter()
            .map(|(id, name, desc, topic_codes)| {
                let text_count = text_questions.get(*id).map(|q| q.len()).unwrap_or(0);
                let json_count = bank_entries
                    .iter()
                    .filter(|e| topic_codes.iter().any(|code| e.topics.contains(&code.to_string())))
                    .count();
                
                TopicInfo {
                    id: id.to_string(),
                    name: name.to_string(),
                    description: desc.to_string(),
                    example_count: text_count + json_count,
                }
            })
            .collect();
        
        KnowledgeBase { topics, text_questions, bank_entries }
    }
    
    /// Get all available topics
    pub fn get_topics(&self) -> Vec<TopicInfo> {
        self.topics.clone()
    }
    
    /// Get example questions for specified topics (legacy format for compatibility)
    pub fn get_examples(&self, topic_ids: &[String], max_per_topic: usize) -> Vec<Question> {
        let mut examples = Vec::new();
        
        for topic_id in topic_ids {
            if let Some(topic_questions) = self.text_questions.get(topic_id) {
                let take_count = max_per_topic.min(topic_questions.len());
                examples.extend(topic_questions.iter().take(take_count).cloned());
            }
        }
        
        examples
    }
    
    /// Get rich question bank entries for specified topics
    pub fn get_bank_examples(&self, topic_ids: &[String], difficulty: Option<&str>, max_total: usize) -> Vec<QuestionBankEntry> {
        // Map topic IDs to topic codes
        let topic_code_map: HashMap<&str, Vec<&str>> = [
            ("recursion", vec!["T009"]),
            ("arrays", vec!["T006"]),
            ("arraylist", vec!["T007"]),
            ("2darrays", vec!["T008"]),
            ("strings", vec!["T013"]),
            ("classes", vec!["T005"]),
            ("inheritance", vec!["T016", "T017"]),
            ("sorting", vec!["T010", "T011"]),
            ("iteration", vec!["T004"]),
            ("boolean", vec!["T003"]),
        ].into_iter().collect();
        
        let difficulty_code = match difficulty {
            Some("easy") => Some("D1"),
            Some("medium") => Some("D2"),
            Some("hard") => Some("D3"),
            _ => None,
        };
        
        let mut results: Vec<QuestionBankEntry> = self.bank_entries
            .iter()
            .filter(|entry| {
                // Check topic match
                let topic_match = topic_ids.iter().any(|tid| {
                    if let Some(codes) = topic_code_map.get(tid.as_str()) {
                        codes.iter().any(|code| entry.topics.contains(&code.to_string()))
                    } else {
                        false
                    }
                });
                
                // Check difficulty match (if specified)
                let diff_match = difficulty_code
                    .map(|d| entry.difficulty == d)
                    .unwrap_or(true);
                
                topic_match && diff_match
            })
            .cloned()
            .collect();
        
        // If we don't have enough with exact difficulty, include adjacent difficulties
        if results.len() < max_total && difficulty_code.is_some() {
            let additional: Vec<QuestionBankEntry> = self.bank_entries
                .iter()
                .filter(|entry| {
                    let topic_match = topic_ids.iter().any(|tid| {
                        if let Some(codes) = topic_code_map.get(tid.as_str()) {
                            codes.iter().any(|code| entry.topics.contains(&code.to_string()))
                        } else {
                            false
                        }
                    });
                    
                    // Not already included
                    let not_included = !results.iter().any(|r| r.id == entry.id);
                    
                    topic_match && not_included
                })
                .cloned()
                .collect();
            
            results.extend(additional);
        }
        
        results.truncate(max_total);
        results
    }
}

/// Load and parse the question-bank.json file
fn load_question_bank() -> Vec<QuestionBankEntry> {
    let Some(file) = KnowledgeAssets::get("question-bank.json") else {
        eprintln!("Warning: question-bank.json not found in knowledge folder");
        return Vec::new();
    };
    
    let content = String::from_utf8_lossy(&file.data);
    
    let bank: QuestionBankFile = match serde_json::from_str(&content) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Warning: Failed to parse question-bank.json: {}", e);
            return Vec::new();
        }
    };
    
    // Convert to our internal format
    bank.questions
        .into_iter()
        .map(|q| QuestionBankEntry {
            id: q.id,
            stem: q.content.stem,
            code: q.content.code,
            options: q.content.options
                .into_iter()
                .map(|o| QuestionBankOption {
                    id: o.id,
                    text: o.text,
                    is_correct: o.is_correct,
                })
                .collect(),
            explanation: q.content.explanation,
            difficulty: q.difficulty,
            cognitive_level: q.cognitive_level,
            topics: q.pedagogy.topics,
            skills: q.pedagogy.skills,
            distractors: DistractorInfo {
                common_mistakes: q.distractors.common_mistakes
                    .into_iter()
                    .map(|m| CommonMistake {
                        option_id: m.option_id,
                        misconception: m.misconception,
                    })
                    .collect(),
                common_errors: q.distractors.common_errors,
            },
        })
        .collect()
}

/// Parse a legacy text-format question bank file
fn parse_text_question_bank(content: &str) -> Vec<Question> {
    let mut questions = Vec::new();
    
    // Skip title line if present
    let content = if content.trim_start().starts_with("Title:") {
        let lines: Vec<&str> = content.lines().collect();
        let start = lines.iter().position(|l| l.trim().starts_with("Title:")).unwrap_or(0);
        lines[start + 1..].join("\n")
    } else {
        content.to_string()
    };
    
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
    
    if last_end < content.len() {
        let block = &content[last_end..];
        if !block.trim().is_empty() {
            blocks.push(block.trim().to_string());
        }
    }
    
    let num_re = Regex::new(r"^\d+\.\s+").unwrap();
    
    for (i, block) in blocks.iter().enumerate() {
        if !num_re.is_match(block) {
            continue;
        }
        
        if let Some(q) = parse_text_question(block, i + 1) {
            questions.push(q);
        }
    }
    
    questions
}

/// Parse a single question from legacy text format
fn parse_text_question(text: &str, index: usize) -> Option<Question> {
    let num_re = Regex::new(r"^\d+\.\s+").unwrap();
    let text = num_re.replace(text, "").to_string();
    
    let answer_re = Regex::new(r"\n\s*a\.\s+").unwrap();
    let answer_start = answer_re.find(&text)?;
    
    let content = text[..answer_start.start()].trim().to_string();
    let answers_section = &text[answer_start.start()..];
    let answers = parse_text_answers(answers_section);
    
    if answers.is_empty() {
        return None;
    }
    
    Some(Question {
        id: format!("ex{}", index),
        content,
        answers,
    })
}

/// Parse answer choices from legacy text format
fn parse_text_answers(text: &str) -> Vec<Answer> {
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
