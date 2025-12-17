fn main() {
    // Pass the API token from environment to compile-time
    // This allows GitHub Actions secrets and local shell env to work
    println!("cargo:rerun-if-env-changed=REPLICATE_API_TOKEN");

    tauri_build::build()
}
