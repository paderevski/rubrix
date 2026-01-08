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

async fn get_secret(user: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lambda_url =
        env::var("LAMBDA_URL").map_err(|_| "Set LAMBDA_URL env var to your Function URL")?;

    // Hash password client-side
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let password_hash = format!("{:x}", hasher.finalize());

    // Call Lambda
    let client = reqwest::Client::new();
    let request = SecretRequest {
        user: user.to_string(),
        password_hash,
    };

    let response = client.post(&lambda_url).json(&request).send().await?;

    match response.status().as_u16() {
        200 => {
            let secret_response: SecretResponse = response.json().await?;
            Ok(secret_response.secret)
        }
        401 => Err("Invalid password".into()),
        404 => Err("User not found".into()),
        _ => {
            let error_response: ErrorResponse = response.json().await?;
            Err(error_response.error.into())
        }
    }
}

#[tokio::main]
async fn main() {
    match get_secret("user123", "test_password_123").await {
        Ok(secret) => {
            println!("Retrieved secret: {}", secret);

            // Use the secret
            // let api_response = make_api_call(&secret).await;
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
