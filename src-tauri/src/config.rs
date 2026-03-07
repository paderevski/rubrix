//! Configuration and credential management for dev vs release builds

/// Check if running in development mode (debug build)
pub fn is_dev_mode() -> bool {
    cfg!(debug_assertions)
}
