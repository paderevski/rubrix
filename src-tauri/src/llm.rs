//! Replicate API client for LLM inference

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// Replicate API configuration
// NOTE: In production, use environment variables or secure storage
const REPLICATE_API_TOKEN: &str = "YOUR_REPLICATE_API_TOKEN_HERE";
const MODEL_VERSION: &str = "anthropic/claude-sonnet-4-5"; // Adjust based on Replicate's model ID

#[derive(Debug, Serialize)]
struct ReplicateRequest {
    version: String,
    input: ReplicateInput,
}

#[derive(Debug, Serialize)]
struct ReplicateInput {
    prompt: String,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
struct ReplicateResponse {
    id: String,
    status: String,
    output: Option<Vec<String>>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ReplicatePrediction {
    id: String,
    status: String,
    output: Option<serde_json::Value>,
    error: Option<String>,
}

/// Generate text using the Replicate API
pub async fn generate(prompt: &str) -> Result<String, String> {
    let client = Client::new();
    
    // For demo purposes, if no API key is set, return mock data
    if REPLICATE_API_TOKEN == "YOUR_REPLICATE_API_TOKEN_HERE" {
        return Ok(generate_mock_response(prompt));
    }
    
    // Create prediction
    let create_response = client
        .post("https://api.replicate.com/v1/predictions")
        .header("Authorization", format!("Token {}", REPLICATE_API_TOKEN))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "version": MODEL_VERSION,
            "input": {
                "prompt": prompt,
                "max_tokens": 4096,
                "temperature": 0.7
            }
        }))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| format!("Failed to create prediction: {}", e))?;

    let prediction: ReplicatePrediction = create_response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if let Some(error) = prediction.error {
        return Err(format!("Replicate error: {}", error));
    }

    // Poll for completion
    let prediction_url = format!("https://api.replicate.com/v1/predictions/{}", prediction.id);
    
    for _ in 0..60 {
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        let status_response = client
            .get(&prediction_url)
            .header("Authorization", format!("Token {}", REPLICATE_API_TOKEN))
            .send()
            .await
            .map_err(|e| format!("Failed to check status: {}", e))?;

        let status: ReplicatePrediction = status_response
            .json()
            .await
            .map_err(|e| format!("Failed to parse status: {}", e))?;

        match status.status.as_str() {
            "succeeded" => {
                if let Some(output) = status.output {
                    // Handle different output formats
                    return match output {
                        serde_json::Value::String(s) => Ok(s),
                        serde_json::Value::Array(arr) => {
                            let parts: Vec<String> = arr
                                .into_iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect();
                            Ok(parts.join(""))
                        }
                        _ => Ok(output.to_string()),
                    };
                }
                return Err("No output from model".to_string());
            }
            "failed" => {
                return Err(format!("Prediction failed: {:?}", status.error));
            }
            "canceled" => {
                return Err("Prediction was canceled".to_string());
            }
            _ => continue, // Still processing
        }
    }

    Err("Timeout waiting for prediction".to_string())
}

/// Generate mock response for demo/testing without API key
fn generate_mock_response(prompt: &str) -> String {
    // Check if it's asking for multiple questions or single
    let is_single = prompt.contains("regenerate") || prompt.contains("single question");
    
    if is_single {
        r#"
1. Consider the following recursive method:

```java
public int mystery(int n) {
    if (n <= 0)
        return 0;
    return n + mystery(n - 2);
}
```

What value is returned by the call `mystery(7)`?

a. `16`
a. `12`
a. `7`
a. `0`
"#.to_string()
    } else {
        r#"
1. What is the output of the following code segment?

```java
int[] arr = {1, 2, 3, 4, 5};
for (int i = 0; i < arr.length; i++) {
    arr[i] = arr[i] * 2;
}
System.out.println(arr[2]);
```

a. `6`
a. `3`
a. `4`
a. `2`

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
a. `24`
a. `5`
a. `1`

3. What is the value of `result` after the following code executes?

```java
String str = "Hello World";
int result = str.indexOf("o");
```

a. `4`
a. `5`
a. `7`
a. `-1`

4. Consider the following method:

```java
public static void mystery(int[] arr) {
    for (int i = 0; i < arr.length / 2; i++) {
        int temp = arr[i];
        arr[i] = arr[arr.length - 1 - i];
        arr[arr.length - 1 - i] = temp;
    }
}
```

If `arr` is `{1, 2, 3, 4, 5}`, what are the contents of `arr` after calling `mystery(arr)`?

a. `{5, 4, 3, 2, 1}`
a. `{1, 2, 3, 4, 5}`
a. `{2, 3, 4, 5, 1}`
a. `{5, 2, 3, 4, 1}`

5. What is printed by the following code segment?

```java
ArrayList<Integer> list = new ArrayList<Integer>();
list.add(1);
list.add(2);
list.add(3);
list.add(1, 4);
System.out.println(list.get(2));
```

a. `2`
a. `3`
a. `4`
a. `1`
"#.to_string()
    }
}
