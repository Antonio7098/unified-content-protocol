//! Error types for the UCM engine.

use thiserror::Error;

/// Engine error type
#[derive(Debug, Error)]
pub enum Error {
    #[error("Block not found: {0}")]
    BlockNotFound(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Section error: {0}")]
    SectionError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),

    #[error("Snapshot error: {0}")]
    SnapshotError(String),

    #[error("Core error: {0}")]
    Core(#[from] ucm_core::Error),
}

/// Result type alias for engine operations
pub type Result<T> = std::result::Result<T, Error>;
