//! Error types for HTML translation.

use thiserror::Error;

/// HTML translation error
#[derive(Debug, Error)]
pub enum HtmlError {
    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Invalid HTML structure: {0}")]
    InvalidStructure(String),

    #[error("Unsupported element: {0}")]
    UnsupportedElement(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Resource limit exceeded: {0}")]
    ResourceLimit(String),

    #[error("Core error: {0}")]
    Core(#[from] ucm_core::Error),
}

/// Result type for HTML translation
pub type Result<T> = std::result::Result<T, HtmlError>;
