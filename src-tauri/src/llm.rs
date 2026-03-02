//! Bedrock-backed LLM client with streaming SSE

use futures_util::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use tauri::Manager;
use tokio::time::Duration;

use crate::config;

const BUILT_GATEWAY_URL: Option<&str> = option_env!("BEDROCK_GATEWAY_URL");

const BEDROCK_ENDPOINT: &str =
    "https://bedrock-runtime.us-east-1.amazonaws.com/openai/v1/chat/completions";
const MODEL_ID: &str = "openai.gpt-oss-120b-1:0";

/// Log prompt and response to a file (appends each time)
fn log_llm_interaction(prompt: &str, response: &str) {
    // Write to parent directory (rubrix/) to avoid triggering Tauri's file watcher
    let log_path = std::path::Path::new("../llm_log.txt");

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Unescape the response for readability (LLM returns escaped JSON)
    let response_unescaped = response;
    //    .replace("\\n", "\n")
    //    .replace("\\t", "\t")
    //    .replace("\\\"", "\"");

    // Format as readable text sections
    let formatted = format!(
        r#"================================================================================
TIMESTAMP: {}
================================================================================

--- PROMPT ---

{}

--- RESPONSE ---

{}

"#,
        timestamp, prompt, response_unescaped
    );

    // Append to log file
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

// Request structures
#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_effort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_options: Option<StreamOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct StreamOptions {
    include_usage: bool,
}

// Streaming response structures
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
    #[serde(default)]
    usage: Option<Usage>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct StreamChoice {
    delta: Delta,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
struct Delta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    role: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

/// Streaming event payload
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

pub fn gateway_url() -> Option<String> {
    let _ = dotenvy::dotenv();
    std::env::var("BEDROCK_GATEWAY_URL")
        .ok()
        .or_else(|| BUILT_GATEWAY_URL.map(|url| url.to_string()))
}

/// Resolve API token from multiple sources with priority order
fn get_api_token(provided_token: Option<String>) -> Result<String, String> {
    use std::env;

    // 1. Use explicitly provided token (from auth command)
    if let Some(token) = provided_token {
        eprintln!("INFO: Using provided API token (length={})", token.len());
        eprintln!("DEBUG: Token value: {}", token);
        return Ok(token);
    }

    // 2. DEV MODE: Try DEV_AWS_TOKEN env var (for local .env overrides)
    if config::is_dev_mode() {
        if let Ok(token) = env::var("DEV_AWS_TOKEN") {
            eprintln!("INFO: Using DEV_AWS_TOKEN from environment");
            eprintln!("DEBUG: Token value: {}", token);
            return Ok(token);
        }
    }

    // 3. Try production env var (both dev and release)
    if let Ok(token) = env::var("AWS_BEARER_TOKEN_BEDROCK") {
        eprintln!("INFO: Using AWS_BEARER_TOKEN_BEDROCK from environment");
        eprintln!("DEBUG: Token value: {}", token);
        return Ok(token);
    }

    // 4. No token available
    if config::is_dev_mode() {
        Err("No token available. Authenticate or set DEV_AWS_TOKEN in .env".into())
    } else {
        Err("Authentication required. No API token found.".into())
    }
}

/// Generate text using the Bedrock API with streaming updates
/// Pass optional api_token to override cached/env token (used for authenticated sessions)
pub async fn generate(
    prompt: &str,
    app_handle: Option<tauri::AppHandle>,
    api_token: Option<String>,
    gateway_auth: Option<GatewayAuth>,
) -> Result<String, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(90))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    // Load .env file (ignore error if already loaded or not present)
    let _ = dotenvy::dotenv();

    if let Some(url) = gateway_url() {
        let gateway_auth = match gateway_auth {
            Some(auth) => auth,
            None => {
                if config::is_dev_mode() {
                    eprintln!("WARN: No gateway credentials, falling back to mock mode in dev");
                    return Ok(generate_mock_response(prompt, app_handle).await);
                }
                return Err("Authentication required. No gateway credentials found.".into());
            }
        };

        emit_stream(&app_handle, "", false, None);

        let request = GatewayRequest {
            user: gateway_auth.user,
            password_hash: gateway_auth.password_hash,
            prompt: prompt.to_string(),
        };

        let response = client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to connect to gateway: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
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
                    let data = data.trim();
                    if data.is_empty() {
                        continue;
                    }

                    if let Ok(chunk) = serde_json::from_str::<GatewayStreamChunk>(data) {
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
        return Ok(accumulated);
    }

    // Resolve API token from hierarchy: provided > keychain > env vars
    let api_token = match get_api_token(api_token) {
        Ok(token) => token,
        Err(e) => {
            eprintln!("WARN: {}", e);
            // For dev/demo purposes, fall back to mock data
            if config::is_dev_mode() {
                eprintln!("INFO: Falling back to mock mode in dev");
                return Ok(generate_mock_response(prompt, app_handle).await);
            } else {
                return Err(e);
            }
        }
    };

    // Emit starting event
    emit_stream(&app_handle, "", false, None);

    // Build headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_token))
            .map_err(|e| format!("Invalid auth header: {}", e))?,
    );

    // Prepare request payload
    let request = ChatRequest {
        model: MODEL_ID.to_string(),
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
        reasoning_effort: Some("medium".to_string()),
        stream: Some(true),
        stream_options: Some(StreamOptions {
            include_usage: true,
        }),
    };

    // Send request
    eprintln!(
        "INFO: Dispatching Bedrock request ({} bytes payload)",
        prompt.len()
    );
    let mut response = None;
    let mut last_error: Option<String> = None;
    let max_retries = 3;

    for attempt in 0..=max_retries {
        let result = client
            .post(BEDROCK_ENDPOINT)
            .headers(headers.clone())
            .json(&request)
            .send()
            .await;

        match result {
            Ok(res) => {
                let status = res.status();
                eprintln!("INFO: Bedrock response status: {}", status);
                if status.is_success() {
                    response = Some(res);
                    break;
                }

                let body = res.text().await.unwrap_or_default();
                let err_msg = format!("Bedrock API error ({}): {}", status.as_u16(), body);

                if attempt < max_retries && is_retryable_status(status.as_u16()) {
                    let delay_ms = 500_u64.saturating_mul(2_u64.pow(attempt));
                    eprintln!(
                        "WARN: Bedrock transient error, retrying in {}ms (attempt {}/{})",
                        delay_ms,
                        attempt + 1,
                        max_retries + 1
                    );
                    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                    continue;
                }

                last_error = Some(err_msg);
                break;
            }
            Err(e) => {
                let err_msg = format!("Failed to send request: {}", e);
                if attempt < max_retries {
                    let delay_ms = 500_u64.saturating_mul(2_u64.pow(attempt));
                    eprintln!(
                        "WARN: Request failed, retrying in {}ms (attempt {}/{})",
                        delay_ms,
                        attempt + 1,
                        max_retries + 1
                    );
                    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                    continue;
                }
                last_error = Some(err_msg);
                break;
            }
        }
    }

    let response = response
        .ok_or_else(|| last_error.unwrap_or_else(|| "Bedrock request failed".to_string()))?;

    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    let mut accumulated = String::new();
    let mut chunk_count: usize = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Stream error: {}", e))?;
        let text = String::from_utf8_lossy(&chunk);
        buffer.push_str(&text);
        chunk_count += 1;
        if chunk_count <= 3 {
            eprintln!(
                "INFO: Received SSE chunk #{} ({} bytes)",
                chunk_count,
                chunk.len()
            );
        }

        // Process complete SSE lines
        while let Some(line_end) = buffer.find('\n') {
            let line = buffer[..line_end].trim().to_string();
            buffer = buffer[line_end + 1..].to_string();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with(':') {
                continue;
            }

            // Parse SSE data lines
            if let Some(data) = line.strip_prefix("data: ") {
                if data == "[DONE]" {
                    continue;
                }

                if let Ok(chunk) = serde_json::from_str::<StreamChunk>(data) {
                    for choice in chunk.choices {
                        if let Some(content) = choice.delta.content {
                            let cleaned = content
                                .replace("<reasoning>", "")
                                .replace("</reasoning>", "");
                            accumulated.push_str(&cleaned);
                            emit_stream(&app_handle, &accumulated, false, None);
                            if chunk_count <= 3 {
                                eprintln!(
                                    "INFO: Emitted stream update ({} chars total)",
                                    accumulated.len()
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    eprintln!(
        "INFO: Completed Bedrock stream after {} chunks ({} chars)",
        chunk_count,
        accumulated.len()
    );
    emit_stream(&app_handle, &accumulated, true, None);
    log_llm_interaction(prompt, &accumulated);
    Ok(accumulated)
}

fn is_retryable_status(status: u16) -> bool {
    status == 429 || status >= 500
}

/// Emit a streaming event to the frontend
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

/// Generate mock response for demo/testing without API key
async fn generate_mock_response(prompt: &str, app_handle: Option<tauri::AppHandle>) -> String {
    let is_single = prompt.contains("replace") || prompt.contains("single question");

    let response = if is_single {
        r#"1. Consider the following recursive method:

```java
public int mystery(int n) {
    if (n <= 0)
        return 0;
    return n + mystery(n - 2);
}
```

What value is returned by the call `mystery(7)`?

a. `16`
b. `12`
c. `7`
d. `0`

---
**Correct Answer:** a
**Explanation:** mystery(7) = 7 + mystery(5) = 7 + 5 + mystery(3) = 7 + 5 + 3 + mystery(1) = 7 + 5 + 3 + 1 + mystery(-1) = 7 + 5 + 3 + 1 + 0 = 16
**Distractor Analysis:**
- b: Off-by-one error, stops at n=2 instead of n<=0
- c: Only returns the initial value without recursion
- d: Thinks base case returns for all calls
"#
    } else {
        r#"1. What is the output of the following code segment?

```java
int[] arr = {1, 2, 3, 4, 5};
for (int i = 0; i < arr.length; i++) {
    arr[i] = arr[i] * 2;
}
System.out.println(arr[2]);
```

a. `6`
b. `3`
c. `4`
d. `2`

---
**Correct Answer:** a
**Explanation:** Array element at index 2 is 3, after doubling becomes 6.
**Distractor Analysis:**
- b: Forgets the doubling operation
- c: Off-by-one, thinks index 2 holds the value 4
- d: Confuses index with original value

2. Consider the following recursive method:

```java
public int factorial(int n) {
    if (n <= 1)
        return 1;
    return n * factorial(n - 1);
}
```

What value is returned by `factorial(5)`?

a. `120`
b. `24`
c. `5`
d. `1`

---
**Correct Answer:** a
**Explanation:** 5! = 5 × 4 × 3 × 2 × 1 = 120
**Distractor Analysis:**
- b: Computes factorial(4) instead
- c: Returns just the input value
- d: Returns the base case value

3. What is the value of `result` after the following code executes?

```java
String str = "Hello World";
int result = str.indexOf("o");
```

a. `4`
b. `5`
c. `7`
d. `-1`

---
**Correct Answer:** a
**Explanation:** indexOf returns the first occurrence of "o" which is at index 4.
**Distractor Analysis:**
- b: Off-by-one, counts from 1 instead of 0
- c: Returns index of second "o" in "World"
- d: Thinks character not found
"#
    };

    // Simulate streaming by emitting characters progressively
    let chars: Vec<char> = response.chars().collect();
    let mut accumulated = String::new();

    for chunk in chars.chunks(20) {
        accumulated.extend(chunk);
        emit_stream(&app_handle, &accumulated, false, None);

        // Small delay to simulate streaming (50ms between chunks)
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    emit_stream(&app_handle, &accumulated, true, None);
    response.to_string()
}
