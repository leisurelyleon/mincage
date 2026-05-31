//! Core error type.

/// Errors produced by pure core logic (validation only; no I/O).
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("invalid container config: {0}")]
    Validation(String),
}

/// Convenience result alias.
pub type Result<T> = std::result::Result<T, CoreError>;
