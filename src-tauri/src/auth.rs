//! Authentication module for retrieving AWS Bedrock API keys from Lambda

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use std::env;

const BUILT_LAMBDA_URL: Option<&str> = option_env!("LAMBDA_URL");

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
    let lambda_url = match env::var("LAMBDA_URL") {
        Ok(url) => url,
        Err(_) => BUILT_LAMBDA_URL.map(|url| url.to_string()).ok_or_else(|| {
            "LAMBDA_URL not set. Configure the environment variable or bake it at build time."
        })?,
    };

    // Hash password client-side (SHA256)
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let password_hash = format!("{:x}", hasher.finalize());

    eprintln!("DEBUG AUTH: user={}, password_len={}", user, password.len());
    eprintln!("DEBUG AUTH: password_hash={}", password_hash);

    // Call Lambda
    let client = reqwest::Client::new();
    let request = SecretRequest {
        user: user.to_string(),
        password_hash: password_hash.clone(),
    };

    eprintln!("DEBUG AUTH: sending request to {}", lambda_url);
    eprintln!(
        "DEBUG AUTH: request body: user={}, password_hash={}",
        user, password_hash
    );

    let response = client
        .post(&lambda_url)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to authentication server: {}", e))?;

    let status = response.status();
    eprintln!("DEBUG AUTH: response status: {}", status);

    match status.as_u16() {
        200 => {
            let secret_response: SecretResponse = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            eprintln!("DEBUG AUTH: authentication successful, received secret");
            Ok(secret_response.secret)
        }
        401 => {
            eprintln!("DEBUG AUTH: 401 - Invalid password");
            Err("Invalid password".into())
        }
        404 => {
            eprintln!("DEBUG AUTH: 404 - User not found");
            Err("User not found".into())
        }
        _ => {
            let error_text = response.text().await.unwrap_or_default();
            eprintln!("DEBUG AUTH: error response: {}", error_text);
            let error_response: Result<ErrorResponse, _> = serde_json::from_str(&error_text);
            match error_response {
                Ok(err) => Err(err.error.into()),
                Err(_) => Err(format!("Server error: {}", error_text).into()),
            }
        }
    }
}

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash_password(password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    #[test]
    fn test_hash_matches_python() {
        // These expected values were generated with:
        // python3 -c "import hashlib; print(hashlib.sha256(b'test').hexdigest())"
        let test_cases = vec![
            (
                "test",
                "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
            ),
            (
                "mypassword123",
                "6e659deaa85842cdabb5c6305fcc40033ba43772ec00d45c2a3c921741a5e377",
            ),
            (
                "",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            ),
            (
                "alice",
                "2bd806c97f0e00af1a1fc3328fa763a9269723c8db8fac4f93af71db186d6e90",
            ),
        ];

        for (password, expected) in test_cases {
            let result = hash_password(password);
            assert_eq!(
                result, expected,
                "Hash mismatch for password '{}': got {}, expected {}",
                password, result, expected
            );
            println!("✓ Password '{}' hashes correctly", password);
        }
    }
}
