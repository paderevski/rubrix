//! Authentication module for retrieving AWS Bedrock API keys from Lambda

use reqwest;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;

#[derive(Serialize)]
struct SecretRequest {
    user: String,
    password_hash: String,
}

#[derive(Deserialize)]
struct SecretResponse {
    secret: String,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

/// Authenticate with Lambda and retrieve the Bedrock API key
pub async fn get_bedrock_api_key(
    user: &str,
    password: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let lambda_url = env::var("LAMBDA_URL")
        .map_err(|_| "LAMBDA_URL environment variable not set. Configure your Lambda Function URL.")?;

    // Hash password client-side (SHA256)
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let password_hash = format!("{:x}", hasher.finalize());

    // Call Lambda
    let client = reqwest::Client::new();
    let request = SecretRequest {
        user: user.to_string(),
        password_hash,
    };

    let response = client
        .post(&lambda_url)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to authentication server: {}", e))?;

    match response.status().as_u16() {
        200 => {
            let secret_response: SecretResponse = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(secret_response.secret)
        }
        401 => Err("Invalid password".into()),
        404 => Err("User not found".into()),
        _ => {
            let error_response: ErrorResponse = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse error response: {}", e))?;
            Err(error_response.error.into())
        }
    }
}
