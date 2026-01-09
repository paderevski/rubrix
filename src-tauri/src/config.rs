//! Configuration and credential management for dev vs release builds

/// Check if running in development mode (debug build)
pub fn is_dev_mode() -> bool {
    cfg!(debug_assertions)
}

/// Secure credential storage using OS keychain
pub struct CredentialStore;

impl CredentialStore {
    /// Save AWS Bedrock token to keychain (dev mode only)
    pub fn save_token(token: &str) -> Result<(), String> {
        if !is_dev_mode() {
            return Err("Token persistence disabled in release mode".into());
        }

        let entry = keyring::Entry::new("rubrix", "aws_bedrock_token")
            .map_err(|e| format!("Keychain error: {}", e))?;
        
        entry
            .set_password(token)
            .map_err(|e| format!("Failed to save token to keychain: {}", e))?;
        
        eprintln!("INFO: Saved token to keychain");
        Ok(())
    }

    /// Load AWS Bedrock token from keychain (dev mode only)
    pub fn load_token() -> Option<String> {
        if !is_dev_mode() {
            return None;
        }

        match keyring::Entry::new("rubrix", "aws_bedrock_token") {
            Ok(entry) => match entry.get_password() {
                Ok(token) => {
                    eprintln!("INFO: Loaded token from keychain (length={})", token.len());
                    Some(token)
                }
                Err(e) => {
                    eprintln!("INFO: No token in keychain: {}", e);
                    None
                }
            },
            Err(e) => {
                eprintln!("WARN: Keychain access error: {}", e);
                None
            }
        }
    }

    /// Clear AWS Bedrock token from keychain
    pub fn clear_token() -> Result<(), String> {
        let entry = keyring::Entry::new("rubrix", "aws_bedrock_token")
            .map_err(|e| format!("Keychain error: {}", e))?;
        
        match entry.delete_password() {
            Ok(_) => {
                eprintln!("INFO: Cleared token from keychain");
                Ok(())
            }
            Err(keyring::Error::NoEntry) => {
                eprintln!("INFO: No token to clear");
                Ok(())
            }
            Err(e) => Err(format!("Failed to clear token: {}", e)),
        }
    }
}
