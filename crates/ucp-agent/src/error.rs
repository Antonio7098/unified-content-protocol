//! Error types for the agent traversal system.

use std::time::Duration;
use thiserror::Error;
use ucm_core::BlockId;

/// Unique identifier for an agent session.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AgentSessionId(pub uuid::Uuid);

impl AgentSessionId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for AgentSessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for AgentSessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Errors that can occur during agent traversal operations.
#[derive(Debug, Error)]
pub enum AgentError {
    // Session errors
    #[error("Session not found: {0}")]
    SessionNotFound(AgentSessionId),

    #[error("Session expired: {0}")]
    SessionExpired(AgentSessionId),

    #[error("Maximum sessions reached: {max}")]
    MaxSessionsReached { max: usize },

    #[error("Session already closed: {0}")]
    SessionClosed(AgentSessionId),

    // Navigation errors
    #[error("Block not found: {0}")]
    BlockNotFound(BlockId),

    #[error("Navigation blocked: {reason}")]
    NavigationBlocked { reason: String },

    #[error("Invalid edge type: {edge_type}")]
    InvalidEdgeType { edge_type: String },

    #[error("No path exists from {from} to {to}")]
    NoPathExists { from: BlockId, to: BlockId },

    #[error("Cannot navigate: history is empty")]
    EmptyHistory,

    // Limit errors
    #[error("Depth limit exceeded: {current} > {max}")]
    DepthLimitExceeded { current: usize, max: usize },

    #[error("Context size limit exceeded: {current} tokens > {max} tokens")]
    ContextLimitExceeded { current: usize, max: usize },

    #[error("Block limit exceeded: {current} blocks > {max} blocks")]
    BlockLimitExceeded { current: usize, max: usize },

    #[error("Operation budget exhausted: {operation_type}")]
    BudgetExhausted { operation_type: String },

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Operation timed out after {0:?}")]
    OperationTimeout(Duration),

    // Circuit breaker errors
    #[error("Circuit breaker open: {reason}")]
    CircuitOpen { reason: String },

    // RAG errors
    #[error("RAG provider not configured")]
    RagNotConfigured,

    #[error("RAG search failed: {0}")]
    RagSearchFailed(String),

    // Coordination errors
    #[error("Coordination lock timeout")]
    CoordinationLockTimeout,

    #[error("Agent coordination failed: {0}")]
    CoordinationFailed(String),

    #[error("Block already claimed by another agent: {block_id}")]
    BlockAlreadyClaimed { block_id: BlockId },

    // Capability errors
    #[error("Operation not permitted: {operation}")]
    OperationNotPermitted { operation: String },

    // Context window errors
    #[error("No results available from last search/find")]
    NoResultsAvailable,

    #[error("Focus block not set")]
    NoFocusBlock,

    // Underlying errors
    #[error("Document error: {0}")]
    DocumentError(String),

    #[error("Engine error: {0}")]
    EngineError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<ucm_core::Error> for AgentError {
    fn from(e: ucm_core::Error) -> Self {
        AgentError::DocumentError(e.to_string())
    }
}

impl From<ucm_engine::error::Error> for AgentError {
    fn from(e: ucm_engine::error::Error) -> Self {
        AgentError::EngineError(e.to_string())
    }
}

impl From<ucl_parser::ParseError> for AgentError {
    fn from(e: ucl_parser::ParseError) -> Self {
        AgentError::ParseError(e.to_string())
    }
}

/// Result type for agent operations.
pub type Result<T> = std::result::Result<T, AgentError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_id_display() {
        let id = AgentSessionId::new();
        let display = format!("{}", id);
        assert!(!display.is_empty());
    }

    #[test]
    fn test_error_display() {
        let err = AgentError::SessionNotFound(AgentSessionId::new());
        let msg = format!("{}", err);
        assert!(msg.contains("Session not found"));
    }
}
