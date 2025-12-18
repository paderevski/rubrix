//! Knowledge base management - loads example questions for few-shot prompting

use crate::{
    Answer, CommonMistake, DistractorInfo, Question, QuestionBankEntry, QuestionBankOption,
    SubjectInfo, TopicInfo,
};
use regex::Regex;
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "knowledge/"]
struct KnowledgeAssets;

/// Schema file structure for topics
#[derive(Debug, Deserialize)]
struct QuestionSchema {
    topics: TopicsSection,
}

#[derive(Debug, Deserialize)]
struct TopicsSection {
    items: Vec<TopicSchemaItem>,
}

#[derive(Debug, Deserialize)]
struct TopicSchemaItem {
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    display: String,
    #[serde(default)]
    comment: String,
}

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
    /// Topics organized by subject
    pub subjects: HashMap<String, Vec<TopicInfo>>,
    /// Legacy text-based questions by subject/topic path
    pub text_questions: HashMap<String, Vec<Question>>,
    /// Rich JSON questions from question-bank.json (organized by subject)
    pub bank_entries: HashMap<String, Vec<QuestionBankEntry>>,
}

impl KnowledgeBase {
    /// Load knowledge base from embedded files, organized by subject folders
    pub fn load() -> Self {
        let mut subjects: HashMap<String, Vec<TopicInfo>> = HashMap::new();
        let mut text_questions: HashMap<String, Vec<Question>> = HashMap::new();
        let mut bank_entries: HashMap<String, Vec<QuestionBankEntry>> = HashMap::new();

        // List of subjects to scan (can be expanded)
        let subject_names = vec!["Computer Science", "Calculus", "English 7"];

        for subject_name in subject_names {
            // Load the schema file to get topic definitions
            let schema_filename = format!("{}/question-schema.json", subject_name);
            let topic_definitions = if let Some(file) = KnowledgeAssets::get(&schema_filename) {
                let content = String::from_utf8_lossy(&file.data);
                match serde_json::from_str::<QuestionSchema>(&content) {
                    Ok(schema) => {
                        // Filter to only T0XX codes and build topic map
                        schema
                            .topics
                            .items
                            .into_iter()
                            .filter(|item| item.id.starts_with('T') && !item.id.is_empty())
                            .map(|item| {
                                (
                                    item.name.clone(),
                                    item.display.clone(),
                                    format!("Topic: {}", item.display),
                                    vec![item.id.clone()],
                                )
                            })
                            .collect::<Vec<_>>()
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to parse {} question-schema.json: {}",
                            subject_name, e
                        );
                        vec![]
                    }
                }
            } else {
                eprintln!(
                    "Warning: No question-schema.json found for {}",
                    subject_name
                );
                vec![]
            };

            if topic_definitions.is_empty() {
                // No topics for this subject, skip it
                continue;
            }

            let mut subject_topics = Vec::new();

            // Load JSON question bank once per subject (outside topic loop)
            let bank_filename = format!("{}/question-bank.json", subject_name);
            let subject_bank_entries = if let Some(file) = KnowledgeAssets::get(&bank_filename) {
                let content = String::from_utf8_lossy(&file.data);
                match serde_json::from_str::<QuestionBankFile>(&content) {
                    Ok(bank) => bank
                        .questions
                        .into_iter()
                        .map(|q| QuestionBankEntry {
                            id: q.id,
                            stem: q.content.stem,
                            code: q.content.code,
                            options: q
                                .content
                                .options
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
                                common_mistakes: q
                                    .distractors
                                    .common_mistakes
                                    .into_iter()
                                    .map(|m| CommonMistake {
                                        option_id: m.option_id,
                                        misconception: m.misconception,
                                    })
                                    .collect(),
                                common_errors: q.distractors.common_errors,
                            },
                        })
                        .collect(),
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to parse {} question-bank.json: {}",
                            subject_name, e
                        );
                        Vec::new()
                    }
                }
            } else {
                Vec::new()
            };

            // Store bank entries for this subject
            bank_entries.insert(subject_name.to_string(), subject_bank_entries.clone());

            for (name, display, _desc, topic_codes) in topic_definitions {
                let question_key = format!("{}/{}", subject_name, name);

                // Note: We no longer load .txt files - only using question-bank.json
                text_questions.insert(question_key.clone(), Vec::new());

                // Count examples for this topic from question bank
                let json_count = subject_bank_entries
                    .iter()
                    .filter(|e| {
                        topic_codes
                            .iter()
                            .any(|code| e.topics.contains(&code.to_string()))
                    })
                    .count();

                // Only include topics that have questions
                if json_count > 0 {
                    subject_topics.push(TopicInfo {
                        id: name,
                        name: display,
                        description: format!("{} questions available", json_count),
                        example_count: json_count,
                    });
                }
            }

            if !subject_topics.is_empty() {
                subjects.insert(subject_name.to_string(), subject_topics);
            }
        }

        println!("Loaded {} subjects", subjects.len());
        for (subject, topics) in &subjects {
            println!("  - {}: {} topics", subject, topics.len());
        }

        KnowledgeBase {
            subjects,
            text_questions,
            bank_entries,
        }
    }

    /// Get all available subjects
    pub fn get_subjects(&self) -> Vec<SubjectInfo> {
        self.subjects
            .iter()
            .map(|(id, topics)| SubjectInfo {
                id: id.clone(),
                name: id.clone(),
                topic_count: topics.len(),
            })
            .collect()
    }

    /// Get all available topics for a specific subject
    pub fn get_topics(&self, subject: &str) -> Vec<TopicInfo> {
        self.subjects.get(subject).cloned().unwrap_or_else(Vec::new)
    }

    /// Get example questions for specified topics (legacy format for compatibility)
    pub fn get_examples(
        &self,
        subject: &str,
        topic_ids: &[String],
        max_per_topic: usize,
    ) -> Vec<Question> {
        let mut examples = Vec::new();

        for topic_id in topic_ids {
            let question_key = format!("{}/{}", subject, topic_id);
            if let Some(topic_questions) = self.text_questions.get(&question_key) {
                let take_count = max_per_topic.min(topic_questions.len());
                examples.extend(topic_questions.iter().take(take_count).cloned());
            }
        }

        examples
    }

    /// Get rich question bank entries for specified topics
    pub fn get_bank_examples(
        &self,
        subject: &str,
        topic_ids: &[String],
        difficulty: Option<&str>,
        max_total: usize,
    ) -> Vec<QuestionBankEntry> {
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
        ]
        .into_iter()
        .collect();

        let difficulty_code = match difficulty {
            Some("easy") => Some("D1"),
            Some("medium") => Some("D2"),
            Some("hard") => Some("D3"),
            _ => None,
        };

        // Get bank entries for this subject
        let subject_entries = self.bank_entries.get(subject).cloned().unwrap_or_default();

        let mut results: Vec<QuestionBankEntry> = subject_entries
            .iter()
            .filter(|entry| {
                // Check topic match
                let topic_match = topic_ids.iter().any(|tid| {
                    if let Some(codes) = topic_code_map.get(tid.as_str()) {
                        codes
                            .iter()
                            .any(|code| entry.topics.contains(&code.to_string()))
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
            let additional: Vec<QuestionBankEntry> = subject_entries
                .iter()
                .filter(|entry| {
                    let topic_match = topic_ids.iter().any(|tid| {
                        if let Some(codes) = topic_code_map.get(tid.as_str()) {
                            codes
                                .iter()
                                .any(|code| entry.topics.contains(&code.to_string()))
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

/// Parse a legacy text-format question bank file
fn parse_text_question_bank(content: &str) -> Vec<Question> {
    let mut questions = Vec::new();

    // Skip title line if present
    let content = if content.trim_start().starts_with("Title:") {
        let lines: Vec<&str> = content.lines().collect();
        let start = lines
            .iter()
            .position(|l| l.trim().starts_with("Title:"))
            .unwrap_or(0);
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
        content: content.clone(),
        stem: content,
        code: None,
        explanation: None,
        distractors: None,
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
                    explanation: None,
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
            explanation: None,
        });
    }

    answers
}
