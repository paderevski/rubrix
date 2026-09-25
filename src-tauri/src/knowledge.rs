//! Knowledge base management - loads example questions for few-shot prompting

use crate::{QuestionBankDocument, QuestionBankEntry, SubjectInfo, SubtopicInfo, TopicInfo};
use rand::seq::SliceRandom;
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[derive(RustEmbed)]
#[folder = "../imports/knowledge/"]
struct KnowledgeAssets;

fn configured_knowledge_roots() -> Vec<PathBuf> {
    let Ok(base) = env::var("RUBRIX_KNOWLEDGE_DIR") else {
        return Vec::new();
    };
    let base_path = PathBuf::from(base);
    if base_path.is_absolute() {
        return vec![base_path];
    }

    let Ok(cwd) = env::current_dir() else {
        return vec![base_path];
    };
    let mut roots = vec![cwd.join(&base_path)];
    if let Some(parent) = cwd.parent() {
        roots.push(parent.join(base_path));
    }
    roots
}

fn resolve_knowledge_path(file: &str) -> Option<PathBuf> {
    configured_knowledge_roots()
        .into_iter()
        .map(|root| root.join(file))
        .find(|candidate| candidate.exists())
}

fn load_knowledge_file(file: &str) -> Option<String> {
    if let Some(path) = resolve_knowledge_path(file) {
        if let Ok(content) = fs::read_to_string(&path) {
            return Some(content);
        }
    }

    KnowledgeAssets::get(file).map(|embedded: rust_embed::EmbeddedFile| {
        String::from_utf8_lossy(&embedded.data).to_string()
    })
}

fn validate_bank_subject(subject: &str) -> Result<(), String> {
    let subject_path = Path::new(subject);
    if subject_path.components().count() != 1
        || !matches!(
            subject_path.components().next(),
            Some(std::path::Component::Normal(_))
        )
    {
        return Err(format!("Invalid question bank subject: {}", subject));
    }
    Ok(())
}

pub fn validate_bank_location(subject: &str, file_name: &str) -> Result<(), String> {
    validate_bank_subject(subject)?;
    let path = Path::new(file_name);
    if path.file_name().and_then(|name| name.to_str()) != Some(file_name)
        || path.extension().and_then(|extension| extension.to_str()) != Some("json")
    {
        return Err(format!("Invalid question bank filename: {}", file_name));
    }
    Ok(())
}

/// List JSON bank files from writable knowledge roots and bundled assets.
pub fn list_question_bank_files(
    subject: &str,
    writable_root: Option<&Path>,
) -> Result<Vec<String>, String> {
    validate_bank_subject(subject)?;
    let mut files = BTreeSet::new();
    let relative_dir = Path::new(subject).join("banks");
    let mut roots = configured_knowledge_roots();
    if let Some(root) = writable_root {
        roots.insert(0, root.to_path_buf());
    }

    for root in roots {
        let directory = root.join(&relative_dir);
        if !directory.exists() {
            continue;
        }
        let entries = fs::read_dir(&directory)
            .map_err(|error| format!("Failed to read {}: {}", directory.display(), error))?;
        for entry in entries {
            let entry = entry.map_err(|error| {
                format!("Failed to read entry in {}: {}", directory.display(), error)
            })?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                    files.insert(file_name.to_string());
                }
            }
        }
    }

    let embedded_prefix = format!("{}/banks/", subject);
    for path in KnowledgeAssets::iter() {
        let path = path.as_ref();
        if let Some(file_name) = path.strip_prefix(&embedded_prefix) {
            if !file_name.contains('/') && file_name.ends_with(".json") {
                files.insert(file_name.to_string());
            }
        }
    }

    Ok(files.into_iter().collect())
}

/// Read one question bank, preferring writable knowledge files over bundled assets.
pub fn read_question_bank_document(
    subject: &str,
    file_name: &str,
    writable_root: Option<&Path>,
) -> Result<QuestionBankDocument, String> {
    validate_bank_location(subject, file_name)?;
    let relative_path = Path::new(subject).join("banks").join(file_name);
    let mut roots = configured_knowledge_roots();
    if let Some(root) = writable_root {
        roots.insert(0, root.to_path_buf());
    }

    let data = roots
        .into_iter()
        .map(|root| root.join(&relative_path))
        .find(|path| path.is_file())
        .map(|path| {
            fs::read_to_string(&path)
                .map_err(|error| format!("Failed to read {}: {}", path.display(), error))
        })
        .transpose()?
        .or_else(|| {
            let embedded_path = format!("{}/banks/{}", subject, file_name);
            KnowledgeAssets::get(&embedded_path)
                .map(|asset| String::from_utf8_lossy(&asset.data).to_string())
        })
        .ok_or_else(|| format!("Question bank file not found: {}", relative_path.display()))?;

    serde_json::from_str(&data)
        .map_err(|error| format!("Failed to parse question bank {}: {}", file_name, error))
}

/// Load and combine valid V2 question entries from every bank JSON for a subject.
pub fn load_question_bank_entries(
    subject: &str,
    writable_root: Option<&Path>,
) -> Vec<QuestionBankEntry> {
    let file_names = match list_question_bank_files(subject, writable_root) {
        Ok(file_names) => file_names,
        Err(error) => {
            eprintln!(
                "Warning: Failed to list question banks for {}: {}",
                subject, error
            );
            return Vec::new();
        }
    };

    let mut entries = Vec::new();
    for file_name in file_names {
        match read_question_bank_document(subject, &file_name, writable_root) {
            Ok(document) => entries.extend(document.questions),
            Err(error) => eprintln!("Warning: {}", error),
        }
    }
    entries
}

/// Schema file structure for topics
#[derive(Debug, Deserialize)]
struct QuestionSchema {
    topics: TopicsSection,
    #[serde(default)]
    subtopics: SubtopicsSection,
}

#[derive(Debug, Deserialize)]
struct TopicsSection {
    items: Vec<TopicSchemaItem>,
}

#[derive(Debug, Deserialize, Default)]
struct SubtopicsSection {
    items: Vec<SubtopicSchemaItem>,
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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct SubtopicSchemaItem {
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    display: String,
    #[serde(default)]
    parent_topic: String,
}

pub struct KnowledgeBase {
    /// Topics organized by subject
    pub subjects: HashMap<String, Vec<TopicInfo>>,
    /// Rich JSON questions from question-bank.json (organized by subject)
    pub bank_entries: HashMap<String, Vec<QuestionBankEntry>>,
    /// Mapping of topic_id -> topic_codes for each subject
    pub topic_code_mappings: HashMap<String, HashMap<String, Vec<String>>>,
    /// Prompt templates for each subject
    pub prompts: HashMap<String, String>,
    /// Regeneration prompt templates for each subject
    pub regeneration_prompts: HashMap<String, String>,
    /// FRQ generation prompt templates for each subject
    pub frq_prompts: HashMap<String, String>,
}

fn append_random_matches<F>(
    results: &mut Vec<QuestionBankEntry>,
    seen_ids: &mut HashSet<String>,
    entries: &[QuestionBankEntry],
    max_total: usize,
    matches: F,
) where
    F: Fn(&QuestionBankEntry) -> bool,
{
    if results.len() >= max_total {
        return;
    }

    let mut candidates: Vec<QuestionBankEntry> = entries
        .iter()
        .filter(|entry| entry.status == "active")
        .filter(|entry| !seen_ids.contains(&entry.id))
        .filter(|entry| matches(entry))
        .cloned()
        .collect();
    candidates.shuffle(&mut rand::thread_rng());

    for entry in candidates.into_iter().take(max_total - results.len()) {
        seen_ids.insert(entry.id.clone());
        results.push(entry);
    }
}

impl KnowledgeBase {
    /// Load knowledge base from embedded files, organized by subject folders
    pub fn load(writable_root: Option<&Path>) -> Self {
        let mut subjects: HashMap<String, Vec<TopicInfo>> = HashMap::new();
        let mut prompts: HashMap<String, String> = HashMap::new();
        let mut regeneration_prompts: HashMap<String, String> = HashMap::new();
        let mut frq_prompts: HashMap<String, String> = HashMap::new();
        let mut bank_entries: HashMap<String, Vec<QuestionBankEntry>> = HashMap::new();
        let mut topic_code_mappings: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

        // List of subjects to scan (can be expanded)
        let subject_names = vec!["Computer Science", "Calculus", "English 7"];

        for subject_name in subject_names {
            // Load the schema file to get topic definitions
            let schema_filename = format!("{}/question-schema.json", subject_name);
            let topic_definitions = if let Some(content) = load_knowledge_file(&schema_filename) {
                match serde_json::from_str::<QuestionSchema>(&content) {
                    Ok(schema) => {
                        // Filter to only T0XX codes and build topic map
                        schema
                            .topics
                            .items
                            .into_iter()
                            .filter(|item| !item.id.is_empty())
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

            // Load subtopics definitions (if present)
            let mut subtopic_map: HashMap<String, Vec<SubtopicInfo>> = HashMap::new();
            if let Some(content) = load_knowledge_file(&schema_filename) {
                if let Ok(schema) = serde_json::from_str::<QuestionSchema>(&content) {
                    for sub in schema
                        .subtopics
                        .items
                        .into_iter()
                        .filter(|s| !s.id.is_empty())
                    {
                        subtopic_map
                            .entry(sub.parent_topic.clone())
                            .or_default()
                            .push(SubtopicInfo {
                                id: sub.id.clone(),
                                name: sub.display.clone(),
                                description: format!("Subtopic of {}", sub.parent_topic),
                                example_count: 0,
                                parent_topic: Some(sub.parent_topic.clone()),
                            });
                    }
                }
            }

            let mut subject_topics = Vec::new();

            // Load JSON question bank once per subject (outside topic loop)
            let subject_bank_entries = load_question_bank_entries(subject_name, writable_root);

            // Store bank entries for this subject
            bank_entries.insert(subject_name.to_string(), subject_bank_entries.clone());

            // Build topic code mapping for this subject
            let mut subject_topic_codes: HashMap<String, Vec<String>> = HashMap::new();

            for (name, display, _desc, topic_codes) in topic_definitions {
                // Store mappings for both the canonical name and each code -> codes
                subject_topic_codes.insert(name.clone(), topic_codes.clone());
                for code in &topic_codes {
                    subject_topic_codes.insert(code.clone(), topic_codes.clone());
                }

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
                    let primary_code = topic_codes.get(0).cloned().unwrap_or_else(|| name.clone());

                    // Attach subtopics (if any) and map them to parent codes
                    let children = subtopic_map
                        .remove(&primary_code)
                        .unwrap_or_default()
                        .into_iter()
                        .map(|mut child| {
                            // Child count should reflect only this specific subtopic.
                            child.example_count = subject_bank_entries
                                .iter()
                                .filter(|entry| {
                                    entry.topics.iter().any(|t| t == &child.id)
                                        || entry.subtopics.as_ref().map_or(false, |subs| {
                                            subs.iter().any(|s| s == &child.id)
                                        })
                                })
                                .count();
                            subject_topic_codes.insert(child.id.clone(), topic_codes.clone());
                            child
                        })
                        .collect::<Vec<_>>();

                    subject_topics.push(TopicInfo {
                        id: primary_code,
                        name: display,
                        description: format!("{} questions available", json_count),
                        example_count: json_count,
                        children,
                    });
                }
            }

            if !subject_topics.is_empty() {
                subjects.insert(subject_name.to_string(), subject_topics);
                topic_code_mappings.insert(subject_name.to_string(), subject_topic_codes);
            }

            // Load prompt template for this subject
            let prompt_filename = format!("{}/prompt.txt", subject_name);
            if let Some(content) = load_knowledge_file(&prompt_filename) {
                prompts.insert(subject_name.to_string(), content);
            } else {
                eprintln!(
                    "Warning: No prompt.txt found for {}, will use default",
                    subject_name
                );
            }

            // Load regeneration prompt template for this subject
            let regen_prompt_filename = format!("{}/regeneration-prompt.txt", subject_name);
            if let Some(content) = load_knowledge_file(&regen_prompt_filename) {
                regeneration_prompts.insert(subject_name.to_string(), content);
            } else {
                eprintln!(
                    "Warning: No regeneration-prompt.txt found for {}, will use built-in regeneration prompt",
                    subject_name
                );
            }

            // Load optional FRQ prompt template for this subject
            let frq_prompt_filename = format!("{}/frq-test-prompt.txt", subject_name);
            if let Some(content) = load_knowledge_file(&frq_prompt_filename) {
                frq_prompts.insert(subject_name.to_string(), content);
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
            prompts,
            regeneration_prompts,
            frq_prompts,
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

        let subject_entries = self
            .bank_entries
            .get(subject)
            .map(Vec::as_slice)
            .unwrap_or_default();
        let selected_id_match = |entry: &QuestionBankEntry| {
            topic_ids.iter().any(|id| {
                entry.topics.iter().any(|topic| topic == id)
                    || entry.subtopics.as_ref().map_or(false, |subtopics| {
                        subtopics.iter().any(|subtopic| subtopic == id)
                    })
            })
        };
        let parent_codes: BTreeSet<&str> = topic_code_map
            .into_iter()
            .flat_map(|map| topic_ids.iter().filter_map(|id| map.get(id)))
            .flat_map(|codes| codes.iter().map(String::as_str))
            .collect();

        let mut results = Vec::new();
        let mut seen_ids = HashSet::new();

        append_random_matches(
            &mut results,
            &mut seen_ids,
            subject_entries,
            max_total,
            |entry| {
                selected_id_match(entry)
                    && difficulty_code.map_or(true, |code| entry.difficulty == code)
            },
        );

        if difficulty_code.is_some() {
            append_random_matches(
                &mut results,
                &mut seen_ids,
                subject_entries,
                max_total,
                selected_id_match,
            );
        }

        append_random_matches(
            &mut results,
            &mut seen_ids,
            subject_entries,
            max_total,
            |entry| {
                entry
                    .topics
                    .iter()
                    .any(|topic| parent_codes.contains(topic.as_str()))
            },
        );

        results
    }

    /// Get prompt template for a subject, or return default
    pub fn get_prompt(&self, subject: &str) -> Option<&str> {
        self.prompts.get(subject).map(|s| s.as_str())
    }

    /// Get regeneration prompt template for a subject
    pub fn get_regeneration_prompt(&self, subject: &str) -> Option<&str> {
        self.regeneration_prompts.get(subject).map(|s| s.as_str())
    }

    /// Get FRQ prompt template for a subject
    pub fn get_frq_prompt(&self, subject: &str) -> Option<&str> {
        self.frq_prompts.get(subject).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::QuestionBankAnswer;

    fn test_document(id: &str) -> QuestionBankDocument {
        QuestionBankDocument {
            schema_version: "2.0.0".to_string(),
            source: id.to_string(),
            questions: vec![QuestionBankEntry {
                id: id.to_string(),
                status: "active".to_string(),
                migration_note: None,
                text: format!("Question {}", id),
                answers: vec![QuestionBankAnswer {
                    id: "a".to_string(),
                    text: "Answer".to_string(),
                    is_correct: true,
                    explanation: "Correct".to_string(),
                }],
                explanation: "Explanation".to_string(),
                difficulty: "D1".to_string(),
                cognitive_level: "B2".to_string(),
                topics: vec!["T001".to_string()],
                subtopics: None,
            }],
        }
    }

    #[test]
    fn loads_and_combines_every_json_bank_in_subject_folder() {
        let root = env::temp_dir().join(format!("rubrix-bank-load-test-{}", std::process::id()));
        let subject = format!("BankAggregateTest_{}", std::process::id());
        let bank_dir = root.join(&subject).join("banks");
        fs::create_dir_all(&bank_dir).unwrap();

        for (file_name, id) in [("first.json", "q1"), ("second.json", "q2")] {
            let data = serde_json::to_string(&test_document(id)).unwrap();
            fs::write(bank_dir.join(file_name), data).unwrap();
        }
        fs::write(bank_dir.join("ignore.txt"), "not a bank").unwrap();

        let entries = load_question_bank_entries(&subject, Some(&root));

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].id, "q1");
        assert_eq!(entries[1].id, "q2");

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn bank_examples_prioritize_active_subtopic_matches_before_parent_fallback() {
        let mut exact = test_document("exact").questions.remove(0);
        exact.subtopics = Some(vec!["S001".to_string()]);

        let mut other_difficulty = exact.clone();
        other_difficulty.id = "other-difficulty".to_string();
        other_difficulty.difficulty = "D2".to_string();

        let mut parent_fallback = exact.clone();
        parent_fallback.id = "parent-fallback".to_string();
        parent_fallback.subtopics = Some(vec!["S002".to_string()]);

        let mut inactive = exact.clone();
        inactive.id = "inactive".to_string();
        inactive.status = "inactive".to_string();

        let knowledge = KnowledgeBase {
            subjects: HashMap::new(),
            bank_entries: HashMap::from([(
                "Test".to_string(),
                vec![exact, other_difficulty, parent_fallback, inactive],
            )]),
            topic_code_mappings: HashMap::from([(
                "Test".to_string(),
                HashMap::from([("S001".to_string(), vec!["T001".to_string()])]),
            )]),
            prompts: HashMap::new(),
            regeneration_prompts: HashMap::new(),
            frq_prompts: HashMap::new(),
        };

        let examples = knowledge.get_bank_examples("Test", &["S001".to_string()], Some("easy"), 3);
        let ids: HashSet<&str> = examples.iter().map(|entry| entry.id.as_str()).collect();

        assert_eq!(examples.len(), 3);
        assert!(ids.contains("exact"));
        assert!(ids.contains("other-difficulty"));
        assert!(ids.contains("parent-fallback"));
        assert!(!ids.contains("inactive"));
    }
}
