#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! sha2 = "0.10"
//! ```

use sha2::{Digest, Sha256};

fn hash_password_rust(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn main() {
    let test_cases = vec!["mypassword123", "test", "alice", "", "secret!@#$%"];

    println!("Testing password hashing (Rust):\n");
    for password in test_cases {
        let hash = hash_password_rust(password);
        println!("Password: '{}'", password);
        println!("Hash:     {}", hash);
        println!();
    }

    // Test with command line args
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let password = &args[1];
        let hash = hash_password_rust(password);
        println!("\nYour password: '{}'", password);
        println!("Hash:          {}", hash);
    }
}
