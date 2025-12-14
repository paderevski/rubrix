#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod knowledge;
mod llm;
mod prompts;
mod qti;

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

// ============================================================================
// Types
// ============================================================================

/// Question as displayed/edited in the UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub content: String,  // Full markdown content
    pub answers: Vec<Answer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub example_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub topics: Vec<String>,
    pub difficulty: String,
    pub count: u32,
    pub notes: Option<String>,
}

// ============================================================================
// Question Bank Types (for few-shot examples)
// ============================================================================

/// Rich question entry from question-bank.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionBankEntry {
    pub id: String,
    pub stem: String,
    pub code: Option<String>,
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
fn get_topics(state: State<AppState>) -> Vec<TopicInfo> {
    state.knowledge.get_topics()
}

#[tauri::command]
async fn generate_questions(
    request: GenerationRequest,
    state: State<'_, AppState>,
) -> Result<Vec<Question>, String> {
    // Get rich examples from question bank (prefer these for better distractors)
    let bank_examples = state.knowledge.get_bank_examples(
        &request.topics,
        Some(&request.difficulty),
        3,  // Get up to 3 examples
    );
    
    // Build prompt with JSON examples
    let prompt = prompts::build_generation_prompt(&request, &bank_examples);
    
    // Call LLM
    let response = llm::generate(&prompt).await?;
    
    // Parse response into questions
    let questions = prompts::parse_llm_response(&response)?;
    
    // Store in state
    let mut stored = state.questions.lock().unwrap();
    *stored = questions.clone();
    
    Ok(questions)
}

#[tauri::command]
async fn regenerate_question(
    index: usize,
    state: State<'_, AppState>,
) -> Result<Question, String> {
    let current_questions = state.questions.lock().unwrap().clone();
    
    if index >= current_questions.len() {
        return Err("Invalid question index".to_string());
    }
    
    let current = &current_questions[index];
    
    // Get one example for reference
    let bank_examples = state.knowledge.get_bank_examples(
        &["recursion".to_string()], // Default topic
        None,
        1,
    );
    
    // Build prompt for single question regeneration
    let prompt = prompts::build_regenerate_prompt(current, &current_questions, &bank_examples);
    let response = llm::generate(&prompt).await?;
    
    // Parse the single question
    let mut new_questions = prompts::parse_llm_response(&response)?;
    if new_questions.is_empty() {
        return Err("Failed to generate replacement question".to_string());
    }
    
    let mut new_question = new_questions.remove(0);
    new_question.id = current.id.clone();
    
    // Update in state
    let mut stored = state.questions.lock().unwrap();
    stored[index] = new_question.clone();
    
    Ok(new_question)
}

#[tauri::command]
fn update_question(
    index: usize,
    question: Question,
    state: State<AppState>,
) -> Result<(), String> {
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
        content: "New question".to_string(),
        answers: vec![
            Answer { text: "Correct answer".to_string(), is_correct: true },
            Answer { text: "Wrong answer".to_string(), is_correct: false },
            Answer { text: "Wrong answer".to_string(), is_correct: false },
            Answer { text: "Wrong answer".to_string(), is_correct: false },
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

// ============================================================================
// Main
// ============================================================================

fn main() {
    let knowledge = knowledge::KnowledgeBase::load();
    
    println!("Loaded {} topics", knowledge.topics.len());
    println!("Loaded {} question bank entries", knowledge.bank_entries.len());
    
    let state = AppState {
        questions: Mutex::new(Vec::new()),
        knowledge,
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_topics,
            generate_questions,
            regenerate_question,
            update_question,
            add_question,
            delete_question,
            get_questions,
            export_to_txt,
            export_to_qti,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
