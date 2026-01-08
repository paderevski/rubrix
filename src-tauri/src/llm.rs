//! Bedrock-backed LLM client with streaming SSE

use futures_util::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use tauri::Manager;
use tokio::time::Duration;

const BEDROCK_ENDPOINT: &str =
    "https://bedrock-runtime.us-west-2.amazonaws.com/openai/v1/chat/completions";
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
}

/// Generate text using the Bedrock API with streaming updates
/// Pass optional api_token to override cached/env token (used for authenticated sessions)
pub async fn generate(
    prompt: &str,
    app_handle: Option<tauri::AppHandle>,
    api_token: Option<String>,
) -> Result<String, String> {
    let client = Client::new();

    // Load .env file (ignore error if already loaded or not present) and log resolution
    let env_path = dotenvy::dotenv().ok();
    if let Some(path) = env_path {
        eprintln!("INFO: Loaded .env from {}", path.display());
    } else {
        eprintln!("INFO: No .env found when initializing LLM client");
    }

    use std::env;

    // Use provided token, or fall back to env var
    let api_token = match api_token {
        Some(token) => {
            eprintln!("INFO: Using provided API token (length={})", token.len());
            token
        }
        None => match env::var("AWS_BEARER_TOKEN_BEDROCK") {
            Ok(token) => {
                eprintln!(
                    "INFO: AWS_BEARER_TOKEN_BEDROCK present (length={})",
                    token.len()
                );
                token
            }
            Err(err) => {
                eprintln!("WARN: AWS_BEARER_TOKEN_BEDROCK not set: {}", err);
                String::new()
            }
        },
    };

    // For dev/demo purposes, if no API key is set, return mock data
    if api_token.is_empty() {
        return Ok(generate_mock_response(prompt, app_handle).await);
    }

    // Emit starting event
    emit_stream(&app_handle, "", false);

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
    let response = client
        .post(BEDROCK_ENDPOINT)
        .headers(headers)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Bedrock API error ({}): {}", status.as_u16(), body));
    }

    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    let mut accumulated = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Stream error: {}", e))?;
        let text = String::from_utf8_lossy(&chunk);
        buffer.push_str(&text);

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
                            emit_stream(&app_handle, &accumulated, false);
                        }
                    }
                }
            }
        }
    }

    emit_stream(&app_handle, &accumulated, true);
    log_llm_interaction(prompt, &accumulated);
    Ok(accumulated)
}

/// Emit a streaming event to the frontend
fn emit_stream(app_handle: &Option<tauri::AppHandle>, text: &str, done: bool) {
    if let Some(handle) = app_handle {
        let event = StreamEvent {
            text: text.to_string(),
            done,
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
        emit_stream(&app_handle, &accumulated, false);

        // Small delay to simulate streaming (50ms between chunks)
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    emit_stream(&app_handle, &accumulated, true);
    response.to_string()
}
