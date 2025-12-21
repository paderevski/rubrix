//! Replicate API client for LLM inference

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;
use tauri::Manager;

// Replicate API configuration
// const MODEL_VERSION: &str = "anthropic/claude-4.5-sonnet";
// const MODEL_VERSION: &str = "openai/gpt-5-mini";
const MODEL_VERSION: &str = "openai/gpt-oss-120b";

/// Log prompt and response to a file (appends each time)
fn log_llm_interaction(prompt: &str, response: &str) {
    // Write to parent directory (rubrix/) to avoid triggering Tauri's file watcher
    let log_path = std::path::Path::new("../llm_log.txt");

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Unescape the response for readability (LLM returns escaped JSON)
    let response_unescaped = response
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\\"", "\"");

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

#[derive(Debug, Deserialize)]
struct ReplicatePrediction {
    id: String,
    status: String,
    output: Option<serde_json::Value>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ReplicateError {
    detail: Option<String>,
    error: Option<String>,
}

/// Streaming event payload
#[derive(Clone, Serialize)]
pub struct StreamEvent {
    pub text: String,
    pub done: bool,
}

/// Generate text using the Replicate API with streaming updates
pub async fn generate(
    prompt: &str,
    app_handle: Option<tauri::AppHandle>,
) -> Result<String, String> {
    let client = Client::new();

    // Load .env file (ignore error if already loaded or not present)
    let _ = dotenvy::dotenv();
    use std::env;
    let api_token = env::var("REPLICATE_API_TOKEN")
        .unwrap_or_else(|_| "YOUR_REPLICATE_API_TOKEN_HERE".to_string());

    // DEBUG: Uncomment to see the prompt being sent
    // eprintln!("DEBUG prompt ({} chars):\n{}", prompt.len(), &prompt[..prompt.len().min(1000)]);

    // For demo purposes, if no API key is set, return mock data
    if api_token == "YOUR_REPLICATE_API_TOKEN_HERE" {
        return Ok(generate_mock_response(prompt, app_handle).await);
    }

    // Emit starting event
    emit_stream(&app_handle, "", false);

    // Create prediction
    let create_response = client
        .post("https://api.replicate.com/v1/predictions")
        .header("Authorization", format!("Token {}", api_token))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "version": MODEL_VERSION,
            "input": {
                "prompt": prompt,
                "max_tokens": 32000,
                "temperature": 0.2
            }
        }))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| format!("Failed to create prediction: {}", e))?;

    // Check HTTP status before parsing
    let status = create_response.status();
    let response_text = create_response.text().await.unwrap_or_default();

    // DEBUG: Uncomment to see raw API response
    // eprintln!(
    //     "DEBUG [{}]: {}",
    //     status.as_u16(),
    //     &response_text[..response_text.len()].replace("\\n", "\n")
    // );

    if !status.is_success() {
        // Try to parse as error response
        if let Ok(error_response) = serde_json::from_str::<ReplicateError>(&response_text) {
            let detail = error_response
                .detail
                .or(error_response.error)
                .unwrap_or_else(|| "Unknown error".to_string());
            return Err(format!(
                "Replicate API error ({}): {}",
                status.as_u16(),
                detail
            ));
        }

        return Err(format!(
            "Replicate API error ({}): {}",
            status.as_u16(),
            response_text
        ));
    }

    let prediction: ReplicatePrediction = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if let Some(error) = prediction.error {
        return Err(format!("Replicate error: {}", error));
    }

    // Poll for completion
    let prediction_url = format!("https://api.replicate.com/v1/predictions/{}", prediction.id);

    for _ in 0..120 {
        tokio::time::sleep(Duration::from_secs(1)).await;

        let status_response = client
            .get(&prediction_url)
            .header("Authorization", format!("Token {}", api_token))
            .send()
            .await
            .map_err(|e| format!("Failed to check status: {}", e))?;

        // Check HTTP status
        let http_status = status_response.status();
        let status_text = status_response.text().await.unwrap_or_default();

        // DEBUG: Uncomment to see polling response
        // eprintln!("DEBUG poll [{}]: {}", http_status.as_u16(), &status_text[..status_text.len().min(300)]);

        if !http_status.is_success() {
            return Err(format!(
                "Replicate API error ({}): {}",
                http_status.as_u16(),
                status_text
            ));
        }

        let status: ReplicatePrediction = serde_json::from_str(&status_text)
            .map_err(|e| format!("Failed to parse status: {}", e))?;

        // Emit partial output as it streams in
        if let Some(ref output) = status.output {
            let partial_text = output_to_string(output);
            if cfg!(debug_assertions) {
                eprint!("{}", partial_text);
                std::io::Write::flush(&mut std::io::stderr()).ok();
            }
            emit_stream(&app_handle, &partial_text, false);
        }

        match status.status.as_str() {
            "succeeded" => {
                if let Some(output) = status.output {
                    let final_text = output_to_string(&output);
                    if cfg!(debug_assertions) {
                        eprintln!("\n[stream-complete]\n");
                    }
                    emit_stream(&app_handle, &final_text, true);
                    // eprintln!(
                    //     "DEBUG response [{}]: {}",
                    //     http_status.as_u16(),
                    //     &status_text[..status_text.len()]
                    // );
                    // Log the interaction
                    log_llm_interaction(prompt, &final_text);
                    return Ok(final_text);
                }
                return Err("No output from model".to_string());
            }
            "failed" => {
                emit_stream(&app_handle, "", true);
                return Err(format!("Prediction failed: {:?}", status.error));
            }
            "canceled" => {
                emit_stream(&app_handle, "", true);
                return Err("Prediction was canceled".to_string());
            }
            _ => continue, // Still processing
        }
    }

    emit_stream(&app_handle, "", true);
    Err("Timeout waiting for prediction".to_string())
}

/// Convert output Value to String
fn output_to_string(output: &serde_json::Value) -> String {
    match output {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect::<Vec<_>>()
            .join(""),
        _ => output.to_string(),
    }
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
