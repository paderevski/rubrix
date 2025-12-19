//! Knowledge base management - loads example questions for few-shot prompting

use crate::{
    CommonMistake, DistractorInfo, QuestionBankEntry, QuestionBankOption, SubjectInfo, TopicInfo,
};
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
    text: String,
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
}

pub struct KnowledgeBase {
    /// Topics organized by subject
    pub subjects: HashMap<String, Vec<TopicInfo>>,
    /// Rich JSON questions from question-bank.json (organized by subject)
    pub bank_entries: HashMap<String, Vec<QuestionBankEntry>>,
    /// Mapping of topic_id -> topic_codes for each subject
    pub topic_code_mappings: HashMap<String, HashMap<String, Vec<String>>>,
}

impl KnowledgeBase {
    /// Load knowledge base from embedded files, organized by subject folders
    pub fn load() -> Self {
        let mut subjects: HashMap<String, Vec<TopicInfo>> = HashMap::new();
        let mut bank_entries: HashMap<String, Vec<QuestionBankEntry>> = HashMap::new();
        let mut topic_code_mappings: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

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
                            text: q.content.text,
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

            // Build topic code mapping for this subject
            let mut subject_topic_codes: HashMap<String, Vec<String>> = HashMap::new();

            for (name, display, _desc, topic_codes) in topic_definitions {
                // Store the mapping from topic name to topic codes
                subject_topic_codes.insert(name.clone(), topic_codes.clone());

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
                topic_code_mappings.insert(subject_name.to_string(), subject_topic_codes);
            }
        }

        println!("Loaded {} subjects", subjects.len());
        for (subject, topics) in &subjects {
            println!("  - {}: {} topics", subject, topics.len());
        }

        KnowledgeBase {
            subjects,
            bank_entries,
            topic_code_mappings,
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

    /// Get rich question bank entries for specified topics
    pub fn get_bank_examples(
        &self,
        subject: &str,
        topic_ids: &[String],
        difficulty: Option<&str>,
        max_total: usize,
    ) -> Vec<QuestionBankEntry> {
        // Get topic code mapping for this subject
        let topic_code_map = self.topic_code_mappings.get(subject);

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
                let topic_match = if let Some(map) = topic_code_map {
                    topic_ids.iter().any(|tid| {
                        if let Some(codes) = map.get(tid) {
                            codes.iter().any(|code| entry.topics.contains(code))
                        } else {
                            false
                        }
                    })
                } else {
                    false
                };

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
                    let topic_match = if let Some(map) = topic_code_map {
                        topic_ids.iter().any(|tid| {
                            if let Some(codes) = map.get(tid) {
                                codes.iter().any(|code| entry.topics.contains(code))
                            } else {
                                false
                            }
                        })
                    } else {
                        false
                    };

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
