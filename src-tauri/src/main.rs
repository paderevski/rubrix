#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod config;
mod knowledge;
mod llm;
mod prompts;
mod qti;

use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::api::dialog::MessageDialogBuilder;
use tauri::{AppHandle, CustomMenuItem, Manager, Menu, MenuItem, State, Submenu};

const BUILT_BUG_REPORT_URL: Option<&str> = option_env!("BUG_REPORT_URL");
const BUILT_BUG_REPORT_API_KEY: Option<&str> = option_env!("BUG_REPORT_API_KEY");

fn load_env_vars() {
    // Load local env files from src-tauri
    let _ = dotenvy::dotenv();
}

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

fn knowledge_base_dir(app_handle: &AppHandle) -> Option<PathBuf> {
    // Load .env if present; ignore errors so we still fall back cleanly
    load_env_vars();

    if let Ok(dir) = env::var("RUBRIX_KNOWLEDGE_DIR") {
        let base = PathBuf::from(dir);
        if base.is_absolute() {
            return Some(base);
        }

        if let Ok(cwd) = env::current_dir() {
            let direct = cwd.join(&base);
            if direct.exists() {
                return Some(direct);
            }

            if let Some(parent) = cwd.parent() {
                return Some(parent.join(&base));
            }

            return Some(direct);
        }

        return Some(base);
    }

    app_handle
        .path_resolver()
        .app_local_data_dir()
        .map(|p| p.join("knowledge"))
}

fn topic_labels_for_prompt(
    subject: &str,
    topic_ids: &[String],
    knowledge: &knowledge::KnowledgeBase,
) -> String {
    let id_to_name: HashMap<String, String> = knowledge
        .get_topics(subject)
        .into_iter()
        .map(|t| (t.id, t.name))
        .collect();

    let mut labels: Vec<String> = topic_ids
        .iter()
        .map(|id| id_to_name.get(id).cloned().unwrap_or_else(|| id.clone()))
        .collect();

    labels.retain(|s| !s.is_empty());
    labels.dedup();
    if labels.is_empty() {
        return topic_ids.join(", ");
    }
    labels.join(", ")
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
    #[serde(default)]
    pub children: Vec<SubtopicInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtopicInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub example_count: usize,
    #[serde(default)]
    pub parent_topic: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BugClientContext {
    pub selected_subject: Option<String>,
    pub selected_topics: Vec<String>,
    pub question_count: usize,
    pub active_tab: String,
    pub status: String,
    pub is_authenticated: bool,
    pub is_dev_mode: bool,
    pub app_zoom: f64,
    pub preview_visible: bool,
    pub streaming_chars: usize,
    pub user_agent: String,
    pub captured_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BugSubmissionInput {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub steps_to_reproduce: Vec<String>,
    #[serde(default)]
    pub expected_behavior: Option<String>,
    #[serde(default)]
    pub actual_behavior: Option<String>,
    pub severity: String,
    #[serde(default)]
    pub reporter_email: Option<String>,
    pub include_diagnostics: bool,
    pub client_context: BugClientContext,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
struct BugReportEnvelope {
    schema_version: String,
    event_type: String,
    event_id: String,
    occurred_at: String,
    app: BugAppMetadata,
    reporter: BugReporter,
    bug: BugPayload,
    context: BugClientContext,
    diagnostics: Option<BugDiagnostics>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
struct BugAppMetadata {
    product_name: String,
    package_name: String,
    version: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
struct BugReporter {
    username: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
struct BugPayload {
    title: String,
    description: String,
    steps_to_reproduce: Vec<String>,
    expected_behavior: Option<String>,
    actual_behavior: Option<String>,
    severity: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
struct BugDiagnostics {
    os: String,
    arch: String,
    is_dev_mode: bool,
    in_memory_question_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_response: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    llm_log_source: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SubmitBugResult {
    event_id: String,
    upstream_status: u16,
    upstream_id: Option<String>,
    upstream_url: Option<String>,
    message: String,
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
    #[serde(default)]
    pub subtopics: Option<Vec<String>>,
    pub skills: Vec<String>,
    pub distractors: DistractorInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct QuestionBankFile {
    questions: Vec<QuestionBankJsonEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct QuestionBankJsonEntry {
    id: String,
    difficulty: String,
    cognitive_level: String,
    content: QuestionContent,
    pedagogy: Pedagogy,
    distractors: DistractorsJson,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct QuestionContent {
    text: String,
    options: Vec<QuestionBankOption>,
    explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Pedagogy {
    topics: Vec<String>,
    #[serde(default)]
    subtopics: Option<Vec<String>>,
    skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DistractorsJson {
    common_mistakes: Vec<CommonMistake>,
    common_errors: Vec<String>,
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
    pub api_key: Mutex<Option<String>>, // Cached Bedrock API key
    pub credentials: Mutex<Option<SavedCredentials>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedCredentials {
    username: String,
    password: String,
}

fn credentials_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let base = app_handle
        .path_resolver()
        .app_local_data_dir()
        .ok_or_else(|| "No app data dir available".to_string())?;
    Ok(base.join("auth").join("credentials.json"))
}

fn load_saved_credentials(app_handle: &AppHandle) -> Result<Option<SavedCredentials>, String> {
    let path = credentials_path(app_handle)?;
    if !path.exists() {
        return Ok(None);
    }
    let data = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    let creds = serde_json::from_str::<SavedCredentials>(&data)
        .map_err(|e| format!("Failed to parse {}: {}", path.display(), e))?;
    Ok(Some(creds))
}

fn bug_report_url() -> Option<String> {
    env::var("BUG_REPORT_URL")
        .ok()
        .or_else(|| BUILT_BUG_REPORT_URL.map(|s| s.to_string()))
        .map(|value| value.trim().trim_matches('"').to_string())
        .filter(|value| !value.is_empty())
}

fn bug_report_api_key() -> Option<String> {
    env::var("BUG_REPORT_API_KEY")
        .ok()
        .or_else(|| BUILT_BUG_REPORT_API_KEY.map(|s| s.to_string()))
        .map(|value| value.trim().trim_matches('"').to_string())
        .filter(|value| !value.is_empty())
}

fn generate_event_id() -> String {
    use rand::{distributions::Alphanumeric, Rng};
    let rand_part: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    format!(
        "bug_{}_{}",
        chrono::Utc::now().timestamp_millis(),
        rand_part
    )
}

fn extract_string_field(value: &serde_json::Value, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| {
        value
            .get(*key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    })
}

fn truncate_with_ellipsis(input: &str, max_chars: usize) -> String {
    if input.chars().count() <= max_chars {
        return input.to_string();
    }
    let mut out = String::with_capacity(max_chars + 1);
    for (idx, ch) in input.chars().enumerate() {
        if idx >= max_chars {
            break;
        }
        out.push(ch);
    }
    out.push('…');
    out
}

fn truncate_tail_with_ellipsis(input: &str, max_chars: usize) -> String {
    let total_chars = input.chars().count();
    if total_chars <= max_chars {
        return input.to_string();
    }

    let mut out = String::with_capacity(max_chars + 1);
    out.push('…');
    let skip = total_chars.saturating_sub(max_chars);
    for ch in input.chars().skip(skip) {
        out.push(ch);
    }
    out
}

fn latest_llm_log_snapshot() -> Option<(String, String, String)> {
    const PROMPT_MARKER: &str = "--- PROMPT ---";
    const RESPONSE_MARKER: &str = "--- RESPONSE ---";
    const SNAPSHOT_MAX_CHARS: usize = 8_000;
    const CANDIDATE_PATHS: [&str; 4] = [
        "../llm_log.txt",
        "./llm_log.txt",
        "llm_log.txt",
        "../llm_log_03012026.txt",
    ];

    let mut selected_path: Option<String> = None;
    let mut content: Option<String> = None;

    for candidate in CANDIDATE_PATHS.iter() {
        if let Ok(text) = fs::read_to_string(candidate) {
            if !text.trim().is_empty() {
                selected_path = Some((*candidate).to_string());
                content = Some(text);
                break;
            }
        }
    }

    let source = selected_path?;
    let text = content?;

    let prompt_marker_idx = text.rfind(PROMPT_MARKER)?;
    let response_marker_idx = text[prompt_marker_idx..]
        .find(RESPONSE_MARKER)
        .map(|offset| prompt_marker_idx + offset)?;

    let prompt_raw = text[prompt_marker_idx + PROMPT_MARKER.len()..response_marker_idx].trim();
    let response_raw = text[response_marker_idx + RESPONSE_MARKER.len()..].trim();

    if prompt_raw.is_empty() || response_raw.is_empty() {
        return None;
    }

    let prompt = truncate_with_ellipsis(prompt_raw, SNAPSHOT_MAX_CHARS);
    let response = truncate_tail_with_ellipsis(response_raw, SNAPSHOT_MAX_CHARS);

    Some((prompt, response, source))
}

fn save_saved_credentials(app_handle: &AppHandle, creds: &SavedCredentials) -> Result<(), String> {
    let path = credentials_path(app_handle)?;
    let parent = path
        .parent()
        .ok_or_else(|| format!("Invalid path for credentials: {}", path.display()))?;
    fs::create_dir_all(parent)
        .map_err(|e| format!("Failed to create dir {}: {}", parent.display(), e))?;

    let json = serde_json::to_string_pretty(creds)
        .map_err(|e| format!("Failed to serialize credentials: {}", e))?;

    let tmp_path = path.with_extension("json.tmp");
    {
        let mut f = fs::File::create(&tmp_path)
            .map_err(|e| format!("Failed to create temp file {}: {}", tmp_path.display(), e))?;
        f.write_all(json.as_bytes())
            .map_err(|e| format!("Failed to write temp file: {}", e))?;
        f.sync_all()
            .map_err(|e| format!("Failed to sync temp file: {}", e))?;
    }

    fs::rename(&tmp_path, &path)
        .map_err(|e| format!("Failed to replace {}: {}", path.display(), e))?;
    Ok(())
}

fn clear_saved_credentials(app_handle: &AppHandle) -> Result<(), String> {
    let path = credentials_path(app_handle)?;
    if path.exists() {
        fs::remove_file(&path)
            .map_err(|e| format!("Failed to remove {}: {}", path.display(), e))?;
    }
    Ok(())
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

    // Convert selected topic IDs to display names for the prompt while keeping IDs for retrieval
    let topics_label = topic_labels_for_prompt(&request.subject, &request.topics, &state.knowledge);

    // Build prompt with JSON examples
    let prompt =
        prompts::build_generation_prompt(&request, &bank_examples, prompt_template, &topics_label);

    // Get cached API key from state
    let api_key = state.api_key.lock().unwrap().clone();
    let gateway_auth = if llm::gateway_url().is_some() {
        state
            .credentials
            .lock()
            .unwrap()
            .clone()
            .map(|creds| llm::GatewayAuth {
                user: creds.username,
                password_hash: auth::hash_password(&creds.password),
            })
    } else {
        None
    };

    // Call LLM with streaming
    let response = llm::generate(&prompt, Some(app_handle), api_key, gateway_auth).await?;

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

    // Get regeneration prompt template for this subject
    let regeneration_prompt_template = state.knowledge.get_regeneration_prompt(subject);

    // Build prompt for single question regeneration
    let prompt = prompts::build_regenerate_prompt(
        current,
        &current_questions,
        &bank_examples,
        instructions.as_deref(),
        regeneration_prompt_template,
    );

    // Get cached API key from state
    let api_key = state.api_key.lock().unwrap().clone();
    let gateway_auth = if llm::gateway_url().is_some() {
        state
            .credentials
            .lock()
            .unwrap()
            .clone()
            .map(|creds| llm::GatewayAuth {
                user: creds.username,
                password_hash: auth::hash_password(&creds.password),
            })
    } else {
        None
    };

    let response = llm::generate(&prompt, Some(app_handle), api_key, gateway_auth).await?;

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

/// Authenticate with Lambda and cache the API key
#[tauri::command]
async fn authenticate(
    username: String,
    password: String,
    state: State<'_, AppState>,
    app_handle: AppHandle,
) -> Result<(), String> {
    if llm::gateway_url().is_some() {
        let creds = SavedCredentials { username, password };
        let mut cached_creds = state.credentials.lock().unwrap();
        *cached_creds = Some(creds.clone());
        save_saved_credentials(&app_handle, &creds)?;
        return Ok(());
    }

    // Call Lambda to get the Bedrock API key
    let api_key = auth::get_bedrock_api_key(&username, &password)
        .await
        .map_err(|e| e.to_string())?;

    // Cache the key in state (memory)
    let mut cached_key = state.api_key.lock().unwrap();
    *cached_key = Some(api_key.clone());

    save_saved_credentials(&app_handle, &SavedCredentials { username, password })?;

    Ok(())
}

/// Attempt to auto-authenticate using cached key or saved credentials
#[tauri::command]
async fn auto_authenticate(
    state: State<'_, AppState>,
    app_handle: AppHandle,
) -> Result<bool, String> {
    if llm::gateway_url().is_some() {
        if state.credentials.lock().unwrap().is_some() {
            return Ok(true);
        }

        if let Some(creds) = load_saved_credentials(&app_handle)? {
            let mut cached_creds = state.credentials.lock().unwrap();
            *cached_creds = Some(creds);
            return Ok(true);
        }

        return Ok(false);
    }

    if state.api_key.lock().unwrap().is_some() {
        return Ok(true);
    }

    if let Some(creds) = load_saved_credentials(&app_handle)? {
        match auth::get_bedrock_api_key(&creds.username, &creds.password).await {
            Ok(api_key) => {
                let mut cached_key = state.api_key.lock().unwrap();
                *cached_key = Some(api_key);
                return Ok(true);
            }
            Err(err) => {
                eprintln!("WARN: Auto-auth failed: {}", err);
                let _ = clear_saved_credentials(&app_handle);
            }
        }
    }

    Ok(false)
}

/// Check if we have a cached API key
#[tauri::command]
fn check_auth(state: State<AppState>) -> bool {
    // Check memory cache first
    if state.api_key.lock().unwrap().is_some() {
        return true;
    }

    if state.credentials.lock().unwrap().is_some() {
        return true;
    }

    false
}

/// Clear the cached API key (logout) - clears memory and saved credentials
#[tauri::command]
fn clear_auth(state: State<AppState>, app_handle: AppHandle) -> Result<(), String> {
    // Clear memory cache
    let mut cached_key = state.api_key.lock().unwrap();
    *cached_key = None;

    let mut cached_creds = state.credentials.lock().unwrap();
    *cached_creds = None;

    let _ = clear_saved_credentials(&app_handle);

    Ok(())
}

/// Check if running in development mode
#[tauri::command]
fn is_dev_mode() -> bool {
    config::is_dev_mode()
}

#[tauri::command]
async fn submit_bug_report(
    input: BugSubmissionInput,
    state: State<'_, AppState>,
    app_handle: AppHandle,
) -> Result<SubmitBugResult, String> {
    let endpoint = bug_report_url().ok_or_else(|| {
        "BUG_REPORT_URL not set. Configure BUG_REPORT_URL in environment or build-time env."
            .to_string()
    })?;

    let event_id = generate_event_id();
    let package = app_handle.package_info();
    let reporter_username = state
        .credentials
        .lock()
        .unwrap()
        .as_ref()
        .map(|c| c.username.clone());

    let diagnostics = if input.include_diagnostics {
        let (last_prompt, last_response, llm_log_source) =
            if let Some((prompt, response, source)) = latest_llm_log_snapshot() {
                (Some(prompt), Some(response), Some(source))
            } else {
                (None, None, None)
            };

        Some(BugDiagnostics {
            os: env::consts::OS.to_string(),
            arch: env::consts::ARCH.to_string(),
            is_dev_mode: config::is_dev_mode(),
            in_memory_question_count: state.questions.lock().unwrap().len(),
            last_prompt,
            last_response,
            llm_log_source,
        })
    } else {
        None
    };

    let envelope = BugReportEnvelope {
        schema_version: "catie.bug_report.v1".to_string(),
        event_type: "bug_report.submitted".to_string(),
        event_id: event_id.clone(),
        occurred_at: chrono::Utc::now().to_rfc3339(),
        app: BugAppMetadata {
            product_name: "Catie".to_string(),
            package_name: package.name.clone(),
            version: package.version.to_string(),
        },
        reporter: BugReporter {
            username: reporter_username,
            email: input.reporter_email.clone(),
        },
        bug: BugPayload {
            title: input.title,
            description: input.description,
            steps_to_reproduce: input.steps_to_reproduce,
            expected_behavior: input.expected_behavior,
            actual_behavior: input.actual_behavior,
            severity: input.severity,
        },
        context: input.client_context,
        diagnostics,
    };

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(20))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let mut request = client.post(endpoint).json(&envelope);

    if let Some(api_key) = bug_report_api_key() {
        request = request.header("x-api-key", api_key);
    }

    if let Ok(token) = env::var("BUG_REPORT_BEARER_TOKEN") {
        request = request.bearer_auth(token);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to submit bug report: {}", e))?;

    let upstream_status = response.status().as_u16();
    let body_text = response.text().await.unwrap_or_default();

    if upstream_status >= 400 {
        return Err(format!(
            "Bug endpoint error ({}): {}",
            upstream_status, body_text
        ));
    }

    let parsed_json: Option<serde_json::Value> = serde_json::from_str(&body_text).ok();
    let upstream_id = parsed_json.as_ref().and_then(|json| {
        extract_string_field(
            json,
            &["id", "issue_id", "report_id", "post_id", "ticket_id"],
        )
    });
    let upstream_url = parsed_json.as_ref().and_then(|json| {
        extract_string_field(
            json,
            &["url", "issue_url", "post_url", "ticket_url", "canny_url"],
        )
    });

    Ok(SubmitBugResult {
        event_id,
        upstream_status,
        upstream_id,
        upstream_url,
        message: "Bug report submitted".to_string(),
    })
}

#[tauri::command]
fn export_to_txt(title: String, state: State<AppState>) -> Result<String, String> {
    let questions = state.questions.lock().unwrap();
    qti::export_txt(&title, &questions)
}

#[tauri::command]
fn export_to_md(title: String, state: State<AppState>) -> Result<String, String> {
    let questions = state.questions.lock().unwrap();
    qti::export_md(&title, &questions)
}

#[tauri::command]
fn export_to_qti(title: String, state: State<AppState>) -> Result<Vec<u8>, String> {
    let questions = state.questions.lock().unwrap();
    qti::export_qti_zip(&title, &questions)
}

#[tauri::command]
async fn export_to_docx(title: String, state: State<'_, AppState>) -> Result<Vec<u8>, String> {
    // Get questions and process them for Word export using the markdown pipeline
    let questions = state.questions.lock().unwrap().clone();

    // Reuse markdown export (shuffle + answer key)
    let output = qti::export_md(&title, &questions)?;

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

/// Load question bank JSON for a subject from disk
#[tauri::command]
fn load_question_bank(
    subject: String,
    state: State<AppState>,
    app_handle: AppHandle,
) -> Result<Vec<QuestionBankEntry>, String> {
    let from_disk: Result<Vec<QuestionBankEntry>, String> = (|| {
        let base = knowledge_base_dir(&app_handle)
            .ok_or_else(|| "No knowledge base directory available".to_string())?;
        let path = base.join(&subject).join("question-bank.json");

        // Prefer user-edited bank in app data; fall back to packaged file if missing
        let data = if path.exists() {
            fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?
        } else {
            // Fallback to bundled asset path (dev/prod) if present
            let embedded_path = app_handle
                .path_resolver()
                .resolve_resource(format!("knowledge/{}/question-bank.json", subject))
                .unwrap_or_else(|| {
                    PathBuf::from("src-tauri/knowledge")
                        .join(&subject)
                        .join("question-bank.json")
                });
            fs::read_to_string(&embedded_path).map_err(|e| {
                format!(
                    "Failed to read embedded bank {}: {}",
                    embedded_path.display(),
                    e
                )
            })?
        };

        let parsed: QuestionBankFile = serde_json::from_str(&data)
            .map_err(|e| format!("Failed to parse question-bank.json: {}", e))?;

        Ok(parsed
            .questions
            .into_iter()
            .map(|q| QuestionBankEntry {
                id: q.id,
                text: q.content.text,
                options: q.content.options,
                explanation: q.content.explanation,
                difficulty: q.difficulty,
                cognitive_level: q.cognitive_level,
                topics: q.pedagogy.topics,
                subtopics: q.pedagogy.subtopics,
                skills: q.pedagogy.skills,
                distractors: DistractorInfo {
                    common_mistakes: q.distractors.common_mistakes,
                    common_errors: q.distractors.common_errors,
                },
            })
            .collect())
    })();

    if let Ok(entries) = &from_disk {
        return Ok(entries.clone());
    }

    let disk_error = from_disk
        .as_ref()
        .err()
        .cloned()
        .unwrap_or_else(|| "unknown error".to_string());

    // Fallback to the in-memory bank loaded at startup (from embedded assets) so the editor still has data
    if let Some(entries) = state.knowledge.bank_entries.get(&subject) {
        if !entries.is_empty() {
            eprintln!(
                "Warning: Using embedded question bank for {} because disk load failed: {}",
                subject, disk_error
            );
            return Ok(entries.clone());
        }
    }

    Err(format!(
        "No question bank found for {}. Set RUBRIX_KNOWLEDGE_DIR to the knowledge folder or add a packaged bank. Last error: {}",
        subject,
        disk_error
    ))
}

/// Save question bank JSON for a subject to disk (atomic write)
#[tauri::command]
fn save_question_bank(
    subject: String,
    entries: Vec<QuestionBankEntry>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let base =
        knowledge_base_dir(&app_handle).ok_or_else(|| "No app data dir available".to_string())?;
    let path = base.join(&subject).join("question-bank.json");
    let parent = path
        .parent()
        .ok_or_else(|| format!("Invalid path for subject: {}", subject))?;

    // Ensure parent exists
    fs::create_dir_all(parent)
        .map_err(|e| format!("Failed to create dir {}: {}", parent.display(), e))?;

    let file_model = QuestionBankFile {
        questions: entries
            .into_iter()
            .map(|e| QuestionBankJsonEntry {
                id: e.id,
                difficulty: e.difficulty,
                cognitive_level: e.cognitive_level,
                content: QuestionContent {
                    text: e.text,
                    options: e.options,
                    explanation: e.explanation,
                },
                pedagogy: Pedagogy {
                    topics: e.topics,
                    subtopics: e.subtopics,
                    skills: e.skills,
                },
                distractors: DistractorsJson {
                    common_mistakes: e.distractors.common_mistakes,
                    common_errors: e.distractors.common_errors,
                },
            })
            .collect(),
    };

    let json = serde_json::to_string_pretty(&file_model)
        .map_err(|e| format!("Failed to serialize question bank: {}", e))?;

    let tmp_path = path.with_extension("json.tmp");
    {
        let mut f = fs::File::create(&tmp_path)
            .map_err(|e| format!("Failed to create temp file {}: {}", tmp_path.display(), e))?;
        f.write_all(json.as_bytes())
            .map_err(|e| format!("Failed to write temp file: {}", e))?;
        f.sync_all()
            .map_err(|e| format!("Failed to sync temp file: {}", e))?;
    }

    fs::rename(&tmp_path, &path)
        .map_err(|e| format!("Failed to replace {}: {}", path.display(), e))?;

    Ok(())
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    load_env_vars();
    let knowledge = knowledge::KnowledgeBase::load();

    let state = AppState {
        questions: Mutex::new(Vec::new()),
        knowledge,
        api_key: Mutex::new(None), // Start with no cached key
        credentials: Mutex::new(None),
    };

    let zoom_in = CustomMenuItem::new("zoom_in", "Zoom In").accelerator("CmdOrCtrl+=");
    let zoom_out = CustomMenuItem::new("zoom_out", "Zoom Out").accelerator("CmdOrCtrl+-");
    let zoom_reset = CustomMenuItem::new("zoom_reset", "Actual Size").accelerator("CmdOrCtrl+0");
    let about_catie = CustomMenuItem::new("about_catie", "About Catie");
    let submit_bug = CustomMenuItem::new("submit_bug", "Submit Bug");

    let file_menu = Submenu::new("File", Menu::new().add_native_item(MenuItem::Quit));
    let edit_menu = Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll),
    );

    let view_menu = Submenu::new(
        "View",
        Menu::new()
            .add_item(zoom_in)
            .add_item(zoom_out)
            .add_item(zoom_reset),
    );
    let help_menu = Submenu::new(
        "Help",
        Menu::new().add_item(about_catie).add_item(submit_bug),
    );

    let menu = Menu::new()
        .add_submenu(file_menu)
        .add_submenu(edit_menu)
        .add_submenu(view_menu)
        .add_submenu(help_menu);

    tauri::Builder::default()
        .manage(state)
        .menu(menu)
        .on_menu_event(|event| {
            let id = event.menu_item_id();
            let payload = match id {
                "zoom_in" => Some("in"),
                "zoom_out" => Some("out"),
                "zoom_reset" => Some("reset"),
                _ => None,
            };

            if let Some(p) = payload {
                let _ = event.window().app_handle().emit_all("app-zoom", p);
                return;
            }

            if id == "about_catie" {
                let app = event.window().app_handle();
                let package = app.package_info();
                let version = package.version.to_string();
                let body = format!(
                    "{}\nVersion {}\n\nAI-powered test item engine for generating and exporting assessment questions.",
                    package.name, version
                );

                MessageDialogBuilder::new("About Catie", body)
                    .kind(tauri::api::dialog::MessageDialogKind::Info)
                    .show(|_| {});
                return;
            }

            if id == "submit_bug" {
                let _ = event
                    .window()
                    .app_handle()
                    .emit_all("app-action", "submit_bug");
            }
        })
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
            authenticate,
            auto_authenticate,
            check_auth,
            clear_auth,
            is_dev_mode,
            submit_bug_report,
            export_to_txt,
            export_to_md,
            export_to_qti,
            export_to_docx,
            load_question_bank,
            save_question_bank,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
