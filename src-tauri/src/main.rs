#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod knowledge;
mod llm;
mod prompts;
mod qti;

use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

fn de_opt_string_or_json<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(opt.and_then(|v| match v {
        serde_json::Value::Null => None,
        serde_json::Value::String(s) => Some(s),
        other => Some(serde_json::to_string_pretty(&other).unwrap_or_else(|_| other.to_string())),
    }))
}

// ============================================================================
// Types
// ============================================================================

/// Question as displayed/edited in the UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub text: String, // Question text in markdown format (may include code blocks)
    #[serde(alias = "options")]
    pub answers: Vec<Answer>,
    #[serde(default, deserialize_with = "de_opt_string_or_json")]
    pub explanation: Option<String>, // Correct answer explanation
    #[serde(default, deserialize_with = "de_opt_string_or_json")]
    pub distractors: Option<String>, // Why wrong answers are tempting
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub topics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub text: String,
    pub is_correct: bool,
    #[serde(default, deserialize_with = "de_opt_string_or_json")]
    pub explanation: Option<String>, // Why this answer is correct/incorrect
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub example_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectInfo {
    pub id: String,
    pub name: String,
    pub topic_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub subject: String,
    pub topics: Vec<String>,
    pub difficulty: String,
    pub count: u32,
    pub notes: Option<String>,
    #[serde(default)]
    pub append: bool, // If true, append to existing questions
}

// ============================================================================
// Question Bank Types (for few-shot examples)
// ============================================================================

/// Rich question entry from question-bank.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionBankEntry {
    pub id: String,
    pub text: String,
    pub options: Vec<QuestionBankOption>,
    pub explanation: String,
    pub difficulty: String,
    pub cognitive_level: String,
    pub topics: Vec<String>,
    pub skills: Vec<String>,
    pub distractors: DistractorInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionBankOption {
    pub id: String,
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistractorInfo {
    pub common_mistakes: Vec<CommonMistake>,
    pub common_errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonMistake {
    pub option_id: String,
    pub misconception: String,
}

// ============================================================================
// App State
// ============================================================================

pub struct AppState {
    pub questions: Mutex<Vec<Question>>,
    pub knowledge: knowledge::KnowledgeBase,
}

// ============================================================================
// Tauri Commands
// ============================================================================

#[tauri::command]
fn get_subjects(state: State<AppState>) -> Vec<SubjectInfo> {
    state.knowledge.get_subjects()
}

#[tauri::command]
fn get_topics(subject: String, state: State<AppState>) -> Vec<TopicInfo> {
    state.knowledge.get_topics(&subject)
}

#[tauri::command]
async fn generate_questions(
    request: GenerationRequest,
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<Vec<Question>, String> {
    // Get rich examples from question bank (prefer these for better distractors)
    let bank_examples = state.knowledge.get_bank_examples(
        &request.subject,
        &request.topics,
        Some(&request.difficulty),
        3, // Get up to 3 examples
    );

    // Get prompt template for this subject
    let prompt_template = state.knowledge.get_prompt(&request.subject);

    // Build prompt with JSON examples
    let prompt = prompts::build_generation_prompt(&request, &bank_examples, prompt_template);

    // Call LLM with streaming
    let response = llm::generate(&prompt, Some(app_handle)).await?;

    // Parse response into questions
    let mut new_questions = prompts::parse_llm_response(&response)?;

    // Set subject and topics on each generated question
    for question in &mut new_questions {
        question.subject = request.subject.clone();
        question.topics = request.topics.clone();
    }

    // Store in state (append or replace)
    let mut stored = state.questions.lock().unwrap();

    if request.append {
        // Renumber IDs to continue from existing
        let start_id = stored.len();
        for (i, q) in new_questions.iter_mut().enumerate() {
            q.id = format!("q{}", start_id + i + 1);
        }
        stored.extend(new_questions.clone());
    } else {
        *stored = new_questions.clone();
    }

    // Return all questions (for frontend to display)
    Ok(stored.clone())
}

#[tauri::command]
async fn regenerate_question(
    index: usize,
    instructions: Option<String>,
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<Question, String> {
    let current_questions = state.questions.lock().unwrap().clone();

    if index >= current_questions.len() {
        return Err("Invalid question index".to_string());
    }

    let current = &current_questions[index];

    // Use the question's subject and topics, or fall back to defaults
    let subject = if !current.subject.is_empty() {
        &current.subject
    } else {
        "Computer Science"
    };
    let topics: &Vec<String> = if !current.topics.is_empty() {
        &current.topics
    } else {
        // Use a temporary Vec so the type matches
        static DEFAULT_TOPICS: [&str; 1] = ["recursion"];
        // Convert static array to Vec<String> and store in a static once cell
        use once_cell::sync::Lazy;
        static DEFAULT_TOPICS_VEC: Lazy<Vec<String>> =
            Lazy::new(|| DEFAULT_TOPICS.iter().map(|s| s.to_string()).collect());
        &DEFAULT_TOPICS_VEC
    };

    // Get one example for reference
    let bank_examples = state.knowledge.get_bank_examples(subject, topics, None, 1);

    // Get prompt template for this subject
    let prompt_template = state.knowledge.get_prompt(subject);

    // Build prompt for single question regeneration
    let prompt = prompts::build_regenerate_prompt(
        current,
        &current_questions,
        &bank_examples,
        instructions.as_deref(),
        prompt_template,
    );
    let response = llm::generate(&prompt, Some(app_handle)).await?;

    // Parse the single question
    let mut new_questions = prompts::parse_llm_response(&response)?;
    if new_questions.is_empty() {
        return Err("Failed to generate replacement question".to_string());
    }

    let mut new_question = new_questions.remove(0);
    new_question.id = current.id.clone();
    new_question.subject = current.subject.clone();
    new_question.topics = current.topics.clone();

    // Update in state
    let mut stored = state.questions.lock().unwrap();
    stored[index] = new_question.clone();

    Ok(new_question)
}

#[tauri::command]
fn update_question(index: usize, question: Question, state: State<AppState>) -> Result<(), String> {
    let mut stored = state.questions.lock().unwrap();

    if index >= stored.len() {
        return Err("Invalid question index".to_string());
    }

    stored[index] = question;
    Ok(())
}

#[tauri::command]
fn add_question(state: State<AppState>) -> Question {
    let mut stored = state.questions.lock().unwrap();

    let new_question = Question {
        id: format!("q{}", stored.len() + 1),
        text: "New question".to_string(),
        explanation: None,
        distractors: None,
        subject: String::new(),
        topics: Vec::new(),
        answers: vec![
            Answer {
                text: "Correct answer".to_string(),
                is_correct: true,
                explanation: None,
            },
            Answer {
                text: "Wrong answer".to_string(),
                is_correct: false,
                explanation: None,
            },
            Answer {
                text: "Wrong answer".to_string(),
                is_correct: false,
                explanation: None,
            },
            Answer {
                text: "Wrong answer".to_string(),
                is_correct: false,
                explanation: None,
            },
        ],
    };

    stored.push(new_question.clone());
    new_question
}

#[tauri::command]
fn delete_question(index: usize, state: State<AppState>) -> Result<(), String> {
    let mut stored = state.questions.lock().unwrap();

    if index >= stored.len() {
        return Err("Invalid question index".to_string());
    }

    stored.remove(index);
    Ok(())
}

#[tauri::command]
fn set_questions(new_questions: Vec<Question>, state: State<AppState>) -> Result<(), String> {
    let mut stored = state.questions.lock().unwrap();
    *stored = new_questions;
    Ok(())
}

#[tauri::command]
fn get_questions(state: State<AppState>) -> Vec<Question> {
    state.questions.lock().unwrap().clone()
}

#[tauri::command]
fn export_to_txt(title: String, state: State<AppState>) -> Result<String, String> {
    let questions = state.questions.lock().unwrap();
    qti::export_txt(&title, &questions)
}

#[tauri::command]
fn export_to_qti(title: String, state: State<AppState>) -> Result<Vec<u8>, String> {
    let questions = state.questions.lock().unwrap();
    qti::export_qti_zip(&title, &questions)
}

#[tauri::command]
async fn export_to_docx(title: String, state: State<'_, AppState>) -> Result<Vec<u8>, String> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    // Get questions and process them for Word export
    let questions = state.questions.lock().unwrap().clone();

    // Build custom markdown for Word export
    let mut output = format!("Title: {}\n\n", title);

    for (i, q) in questions.iter().enumerate() {
        // Question number and text
        output.push_str(&format!("{}. {}\n\n", i + 1, q.text));

        // Shuffle answers
        let mut shuffled_answers: Vec<_> = q.answers.iter().collect();
        shuffled_answers.shuffle(&mut thread_rng());

        // Format answers with indent and mark correct ones with *
        for answer in shuffled_answers {
            let marker = if answer.is_correct { "*" } else { "" };
            output.push_str(&format!("    a. {}{}\n", marker, answer.text));
        }

        output.push_str("\n");
    }

    // Create JSON payload
    let payload = serde_json::json!({
        "markdown": output,
        "format": "docx"
    });

    // Call the API endpoint
    let client = reqwest::Client::new();
    let response = client
        .post("https://aminsl4ogh.execute-api.us-east-1.amazonaws.com/convert")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to call API: {}", e))?;

    // Check if the response was successful
    if !response.status().is_success() {
        return Err(format!("API returned error: {}", response.status()));
    }

    // Get the binary response
    let docx_bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    Ok(docx_bytes.to_vec())
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let knowledge = knowledge::KnowledgeBase::load();

    let state = AppState {
        questions: Mutex::new(Vec::new()),
        knowledge,
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_subjects,
            get_topics,
            generate_questions,
            regenerate_question,
            update_question,
            add_question,
            delete_question,
            set_questions,
            get_questions,
            export_to_txt,
            export_to_qti,
            export_to_docx,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
