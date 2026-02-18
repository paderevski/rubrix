use std::env;
use std::path::PathBuf;

fn load_dotenv() -> Option<PathBuf> {
    let cwd = env::current_dir().ok()?;
    let candidates = [cwd.join(".env"), cwd.join("..").join(".env")];

    for path in candidates {
        if path.exists() {
            if dotenvy::from_path(&path).is_ok() {
                return Some(path);
            }
        }
    }

    None
}

fn main() {
    // Pass the API token from environment to compile-time
    // This allows GitHub Actions secrets and local shell env to work
    println!("cargo:rerun-if-env-changed=REPLICATE_API_TOKEN");
    println!("cargo:rerun-if-env-changed=LAMBDA_URL");
    println!("cargo:rerun-if-env-changed=BEDROCK_GATEWAY_URL");

    if let Some(path) = load_dotenv() {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    if let Ok(lambda_url) = env::var("LAMBDA_URL") {
        println!("cargo:rustc-env=LAMBDA_URL={}", lambda_url);
    }

    if let Ok(gateway_url) = env::var("BEDROCK_GATEWAY_URL") {
        println!("cargo:rustc-env=BEDROCK_GATEWAY_URL={}", gateway_url);
    }

    tauri_build::build()
}
