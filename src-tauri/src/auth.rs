//! Authentication helpers for gateway credential hashing.

use sha2::{Digest, Sha256};

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        }
    }
}
