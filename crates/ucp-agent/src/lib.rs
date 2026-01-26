//! Agent Graph Traversal System for UCP Knowledge Graphs
//!
//! This crate provides a powerful, flexible graph traversal system enabling AI agents
//! to navigate knowledge graphs, search semantically, manage context windows, and
//! coordinate parallel exploration.
//!
//! # Features
//!
//! - **UCL Commands**: Traversal commands (GOTO, EXPAND, FOLLOW, etc.) and context
//!   commands (CTX ADD, CTX REMOVE, etc.) for agent-friendly graph navigation.
//! - **Session Management**: Track agent position, history, and state across operations.
//! - **RAG Integration**: Pluggable semantic search providers for knowledge retrieval.
//! - **Safety Mechanisms**: Limits, circuit breakers, and depth guards for robust operation.
//! - **Observability**: Comprehensive metrics and telemetry for monitoring.
//!
//! # Example
//!
//! ```ignore
//! use ucp_agent::{AgentTraversal, SessionConfig};
//! use ucm_core::Document;
//!
//! // Create traversal system
//! let doc = Document::new();
//! let traversal = AgentTraversal::new(doc);
//!
//! // Create a session
//! let session_id = traversal.create_session(SessionConfig::default())?;
//!
//! // Execute UCL commands
//! traversal.execute_ucl(&session_id, "GOTO blk_abc123")?;
//! traversal.execute_ucl(&session_id, "EXPAND blk_abc123 DOWN DEPTH=3")?;
//! traversal.execute_ucl(&session_id, "CTX ADD RESULTS")?;
//! ```

pub mod cursor;
pub mod error;
pub mod executor;
pub mod metrics;
pub mod operations;
pub mod rag;
pub mod safety;
pub mod session;

// Re-exports
pub use cursor::{CursorNeighborhood, TraversalCursor, ViewMode};
pub use error::{AgentError, AgentSessionId, Result};
pub use executor::{execute_ucl, ExecutionResult, UclExecutor};
pub use metrics::{MetricsSnapshot, OperationMetrics, SessionMetrics};
pub use operations::{
    AgentTraversal, BlockView, ExpandDirection, ExpandOptions, ExpansionResult, FindResult,
    NavigationResult, NeighborhoodView, SearchOptions,
};
pub use rag::{
    MockRagProvider, NullRagProvider, RagCapabilities, RagMatch, RagProvider, RagSearchOptions,
    RagSearchResults,
};
pub use safety::{
    BudgetTracker, CircuitBreaker, CircuitState, DepthGuard, GlobalLimits, OperationBudget,
    SessionLimits,
};
pub use session::{AgentCapabilities, AgentSession, SessionConfig, SessionInfo, SessionState};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_exports() {
        // Verify key types are exported
        let _: AgentSessionId = AgentSessionId::new();
        let _: SessionLimits = SessionLimits::default();
        let _: GlobalLimits = GlobalLimits::default();
        let _: ViewMode = ViewMode::default();
    }
}
