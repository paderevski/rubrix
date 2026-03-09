//! Bedrock gateway-backed LLM client with streaming SSE

use futures_util::StreamExt;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use tauri::Manager;
use tokio::time::Duration;

const BUILT_GATEWAY_URL: Option<&str> = option_env!("BEDROCK_GATEWAY_URL");

/// Log prompt and response to a file (appends each time)
fn log_llm_interaction(prompt: &str, response: &str) {
    // Write to parent directory (rubrix/) to avoid triggering Tauri's file watcher
    let log_path = std::path::Path::new("../llm_log.txt");

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let formatted = format!(
        r#"================================================================================
TIMESTAMP: {}
================================================================================

--- PROMPT ---

{}

--- RESPONSE ---

{}

"#,
        timestamp, prompt, response
    );

    match OpenOptions::new().create(true).append(true).open(log_path) {
        Ok(mut file) => {
            let _ = write!(file, "{}", formatted);
            eprintln!("DEBUG: Logged LLM interaction to llm_log.txt");
        }
        Err(e) => {
            eprintln!("DEBUG: Failed to write log: {}", e);
        }
    }
}

/// Streaming event payload consumed by the frontend.
#[derive(Clone, Serialize)]
pub struct StreamEvent {
    pub text: String,
    pub done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_tokens: Option<u64>,
}

#[derive(Clone)]
pub struct GatewayAuth {
    pub user: String,
    pub password_hash: String,
}

#[derive(Serialize)]
struct GatewayRequest {
    user: String,
    password_hash: String,
    prompt: String,
}

#[derive(Deserialize)]
struct GatewayStreamChunk {
    text: String,
    done: bool,
    #[serde(default)]
    remaining_tokens: Option<u64>,
}

#[derive(Deserialize)]
struct GatewayErrorResponse {
    error: Option<String>,
    status: Option<u16>,
}

pub fn gateway_url() -> Option<String> {
    let _ = dotenvy::dotenv();
    std::env::var("BEDROCK_GATEWAY_URL")
        .ok()
        .or_else(|| BUILT_GATEWAY_URL.map(|url| url.to_string()))
}

pub async fn validate_gateway_credentials(user: &str, password_hash: &str) -> Result<(), String> {
    let url =
        gateway_url().ok_or_else(|| "Authentication gateway is not configured".to_string())?;
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let request = GatewayRequest {
        user: user.to_string(),
        password_hash: password_hash.to_string(),
        // Minimal prompt to force auth check.
        prompt: "auth_check".to_string(),
    };

    let response = client
        .post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to authentication server: {}", e))?;

    let transport_status = response.status().as_u16();
    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let body = response.text().await.unwrap_or_default();
    let trimmed = body.trim();

    // Stream-mode Lambda can return JSON errors with HTTP 200.
    if content_type.contains("application/json") || trimmed.starts_with('{') {
        if let Ok(err) = serde_json::from_str::<GatewayErrorResponse>(trimmed) {
            let effective_status = err.status.unwrap_or(transport_status);
            if let Some(message) = err.error.filter(|msg| !msg.trim().is_empty()) {
                return match effective_status {
                    401 => Err("Invalid password".to_string()),
                    404 => Err("User not found".to_string()),
                    _ => Err(format!("Authentication failed: {}", message)),
                };
            }
            if effective_status >= 400 {
                return Err(format!(
                    "Authentication failed (status {})",
                    effective_status
                ));
            }
        }
    }

    if transport_status < 400 {
        return Ok(());
    }

    match transport_status {
        401 => Err("Invalid password".to_string()),
        404 => Err("User not found".to_string()),
        _ if !trimmed.is_empty() => Err(format!("Authentication failed: {}", trimmed)),
        _ => Err(format!(
            "Authentication failed (status {})",
            transport_status
        )),
    }
}

/// Generate text using the configured gateway with streaming updates.
pub async fn generate(
    prompt: &str,
    app_handle: Option<tauri::AppHandle>,
    gateway_auth: Option<GatewayAuth>,
) -> Result<String, String> {
    let gateway_url = gateway_url().ok_or_else(|| {
        "Gateway mode is required, but BEDROCK_GATEWAY_URL is not configured.".to_string()
    })?;
    let gateway_auth = gateway_auth
        .ok_or_else(|| "Authentication required. No gateway credentials found.".to_string())?;

    let client = Client::builder()
        .timeout(Duration::from_secs(90))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    emit_stream(&app_handle, "", false, None);

    let request = GatewayRequest {
        user: gateway_auth.user,
        password_hash: gateway_auth.password_hash,
        prompt: prompt.to_string(),
    };

    let response = client
        .post(&gateway_url)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to gateway: {}", e))?;

    let status = response.status();
    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_ascii_lowercase();

    if content_type.contains("application/json") {
        let body = response.text().await.unwrap_or_default();
        let trimmed = body.trim();
        if let Ok(err) = serde_json::from_str::<GatewayErrorResponse>(trimmed) {
            if let Some(message) = err.error.filter(|msg| !msg.trim().is_empty()) {
                return Err(format!(
                    "Gateway error ({}): {}",
                    err.status.unwrap_or(status.as_u16()),
                    message
                ));
            }
            if let Some(code) = err.status {
                return Err(format!("Gateway error ({})", code));
            }
        }
        if !status.is_success() {
            return Err(format!("Gateway error ({}): {}", status.as_u16(), body));
        }
        return Err("Gateway returned JSON instead of stream.".to_string());
    } else if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Gateway error ({}): {}", status.as_u16(), body));
    }

    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    let mut accumulated = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Gateway stream error: {}", e))?;
        let text = String::from_utf8_lossy(&chunk);
        buffer.push_str(&text);

        while let Some(line_end) = buffer.find('\n') {
            let line = buffer[..line_end].trim().to_string();
            buffer = buffer[line_end + 1..].to_string();

            if line.is_empty() || line.starts_with(':') {
                continue;
            }

            if let Some(data) = line.strip_prefix("data:") {
                let payload = data.trim();
                if payload.is_empty() {
                    continue;
                }

                if let Ok(chunk) = serde_json::from_str::<GatewayStreamChunk>(payload) {
                    if !chunk.text.is_empty() {
                        let cleaned = chunk
                            .text
                            .replace("<reasoning>", "")
                            .replace("</reasoning>", "");
                        accumulated.push_str(&cleaned);
                        emit_stream(&app_handle, &accumulated, false, None);
                    }
                    if chunk.done {
                        emit_stream(&app_handle, &accumulated, true, chunk.remaining_tokens);
                        log_llm_interaction(prompt, &accumulated);
                        return Ok(accumulated);
                    }
                }
            }
        }
    }

    emit_stream(&app_handle, &accumulated, true, None);
    log_llm_interaction(prompt, &accumulated);
    Ok(accumulated)
}

/// Emit a streaming event to the frontend.
fn emit_stream(
    app_handle: &Option<tauri::AppHandle>,
    text: &str,
    done: bool,
    remaining_tokens: Option<u64>,
) {
    if let Some(handle) = app_handle {
        let event = StreamEvent {
            text: text.to_string(),
            done,
            remaining_tokens,
        };
        let _ = handle.emit_all("llm-stream", event);
    }
}
