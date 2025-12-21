use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

const BEDROCK_ENDPOINT: &str =
    "https://bedrock-runtime.us-west-2.amazonaws.com/openai/v1/chat/completions";
const MODEL_ID: &str = "openai.gpt-oss-120b-1:0";

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

#[derive(Serialize)]
struct StreamOptions {
    include_usage: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: String,
    content: String,
}

// Streaming response structures
#[derive(Deserialize, Debug)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
    #[serde(default)]
    usage: Option<Usage>,
}

#[derive(Deserialize, Debug)]
struct StreamChoice {
    delta: Delta,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
struct Delta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    role: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = std::env::var("AWS_BEARER_TOKEN_BEDROCK")
        .expect("Set AWS_BEARER_TOKEN_BEDROCK environment variable");

    // Build headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );

    let client = reqwest::Client::new();

    // Load problem from prompt.txt
    let problem = std::fs::read_to_string("src/prompt.txt").expect("Failed to read prompt.txt");

    println!("════════════════════════════════════════════════════════════════");
    println!("  Bedrock Streaming Demo - gpt-oss-120b");
    println!("════════════════════════════════════════════════════════════════\n");

    println!("📝 PROMPT:\n{}\n", problem);
    println!("────────────────────────────────────────────────────────────────");
    println!("⏳ Streaming response...\n");

    let request = ChatRequest {
        model: MODEL_ID.into(),
        messages: vec![
            ChatMessage {
                role: "system".into(),
                content: "You are a helpful math tutor. Show your reasoning step by step.".into(),
            },
            ChatMessage {
                role: "user".into(),
                content: problem.into(),
            },
        ],
        reasoning_effort: Some("medium".into()),
        stream: Some(true),
        stream_options: Some(StreamOptions {
            include_usage: true,
        }),
    };

    let response = client
        .post(BEDROCK_ENDPOINT)
        .headers(headers)
        .json(&request)
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await?;
        eprintln!("❌ Request failed with status {}", status);
        eprintln!("Response: {}", body);
        return Err(format!("API error: {}", status).into());
    }

    // Stream the response
    let mut stream = response.bytes_stream();

    use futures_util::StreamExt;
    use std::io::Write;

    let mut buffer = String::new();
    let mut raw_response = String::new(); // Buffer to store all raw stream data
    let mut reasoning_started = false;
    let mut response_started = false;
    let mut response_header_printed = false;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let text = String::from_utf8_lossy(&chunk);
        raw_response.push_str(&text); // Store raw data
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
                            // Print reasoning header on first reasoning tag
                            if content.contains("<reasoning>") && !reasoning_started {
                                print!("🧠 REASONING:\n");
                                reasoning_started = true;
                            }

                            if reasoning_started
                                && !response_started
                                && !content.contains("<reasoning>")
                            {
                                response_started = true;
                            }

                            // Strip all reasoning tags
                            let display = content
                                .replace("<reasoning>", "")
                                .replace("</reasoning>", "");

                            // Print response header right before the first response content
                            if response_started && !response_header_printed {
                                print!("\n\n────────────────────────────────────────────────────────────────\n");
                                print!("💬 RESPONSE:\n");
                                response_header_printed = true;
                            }

                            print!("{}", display);
                            std::io::stdout().flush()?;
                        }

                        if choice.finish_reason.is_some() {
                            println!("\n");
                        }
                    }

                    // Print usage if available (usually in final chunk)
                    if let Some(usage) = chunk.usage {
                        println!(
                            "────────────────────────────────────────────────────────────────"
                        );
                        println!("📊 TOKEN USAGE:");
                        println!("   Prompt tokens:     {}", usage.prompt_tokens);
                        println!("   Completion tokens: {}", usage.completion_tokens);
                        println!("   Total tokens:      {}", usage.total_tokens);
                    }
                }
            }
        }
    }

    println!("\n════════════════════════════════════════════════════════════════\n");

    // Write raw response to file
    std::fs::write("raw_response.txt", &raw_response)?;
    println!("✅ Raw response saved to raw_response.txt\n");

    Ok(())
}
