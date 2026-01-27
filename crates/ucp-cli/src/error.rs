//! CLI error handling

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Document error: {0}")]
    Document(#[from] ucm_core::Error),

    #[error("Invalid block ID: {0}")]
    InvalidBlockId(String),

    #[error("Block not found: {0}")]
    BlockNotFound(String),

    #[error("Session not found: {0}")]
    SessionNotFound(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Snapshot not found: {0}")]
    SnapshotNotFound(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),

    #[error("Agent error: {0}")]
    AgentError(String),

    #[error("{0}")]
    Other(String),
}

pub type CliResult<T> = Result<T, CliError>;

impl From<anyhow::Error> for CliError {
    fn from(e: anyhow::Error) -> Self {
        CliError::Other(e.to_string())
    }
}

impl From<ucp_translator_markdown::TranslatorError> for CliError {
    fn from(e: ucp_translator_markdown::TranslatorError) -> Self {
        CliError::ParseError(e.to_string())
    }
}

impl From<ucp_translator_html::HtmlError> for CliError {
    fn from(e: ucp_translator_html::HtmlError) -> Self {
        CliError::ParseError(e.to_string())
    }
}
