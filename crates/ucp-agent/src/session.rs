//! Agent session management.

use crate::cursor::{TraversalCursor, ViewMode};
use crate::error::{AgentError, AgentSessionId, Result};
use crate::metrics::SessionMetrics;
use crate::safety::{BudgetTracker, SessionLimits};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use ucm_core::{BlockId, EdgeType};

/// Session state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum SessionState {
    /// Session is active and accepting commands.
    #[default]
    Active,
    /// Session is temporarily paused.
    Paused,
    /// Session completed successfully.
    Completed,
    /// Session timed out.
    TimedOut,
    /// Session ended with error.
    Error { reason: String },
}

/// Agent capabilities define what operations are permitted.
#[derive(Debug, Clone)]
pub struct AgentCapabilities {
    /// Can traverse the graph.
    pub can_traverse: bool,
    /// Can execute semantic search via RAG.
    pub can_search: bool,
    /// Can modify context window.
    pub can_modify_context: bool,
    /// Can coordinate with other agents.
    pub can_coordinate: bool,
    /// Allowed edge types for traversal.
    pub allowed_edge_types: HashSet<EdgeType>,
    /// Maximum expansion depth per operation.
    pub max_expand_depth: usize,
}

impl Default for AgentCapabilities {
    fn default() -> Self {
        Self {
            can_traverse: true,
            can_search: true,
            can_modify_context: true,
            can_coordinate: true,
            allowed_edge_types: HashSet::new(), // Empty means all allowed
            max_expand_depth: 10,
        }
    }
}

impl AgentCapabilities {
    /// Check if an edge type is allowed.
    pub fn is_edge_allowed(&self, edge_type: &EdgeType) -> bool {
        self.allowed_edge_types.is_empty() || self.allowed_edge_types.contains(edge_type)
    }

    /// Create capabilities with all permissions.
    pub fn full() -> Self {
        Self::default()
    }

    /// Create read-only capabilities (traverse only, no context modification).
    pub fn read_only() -> Self {
        Self {
            can_traverse: true,
            can_search: true,
            can_modify_context: false,
            can_coordinate: false,
            ..Default::default()
        }
    }
}

/// Configuration for creating a new session.
#[derive(Debug, Clone, Default)]
pub struct SessionConfig {
    /// Human-readable session name.
    pub name: Option<String>,
    /// Starting block ID (defaults to document root).
    pub start_block: Option<BlockId>,
    /// Session limits.
    pub limits: SessionLimits,
    /// Agent capabilities.
    pub capabilities: AgentCapabilities,
    /// Initial view mode.
    pub view_mode: ViewMode,
}

impl SessionConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_start_block(mut self, block: BlockId) -> Self {
        self.start_block = Some(block);
        self
    }

    pub fn with_limits(mut self, limits: SessionLimits) -> Self {
        self.limits = limits;
        self
    }

    pub fn with_capabilities(mut self, capabilities: AgentCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }

    pub fn with_view_mode(mut self, mode: ViewMode) -> Self {
        self.view_mode = mode;
        self
    }
}

/// Agent session state - tracks individual agent's position and history.
pub struct AgentSession {
    /// Unique session identifier.
    pub id: AgentSessionId,
    /// Human-readable session name.
    pub name: Option<String>,
    /// Current cursor position in the graph.
    pub cursor: TraversalCursor,
    /// Agent capabilities.
    pub capabilities: AgentCapabilities,
    /// Safety limits for this session.
    pub limits: SessionLimits,
    /// Budget tracker.
    pub budget: BudgetTracker,
    /// Metrics and telemetry.
    pub metrics: SessionMetrics,
    /// Session state.
    pub state: SessionState,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Last activity timestamp.
    pub last_active: DateTime<Utc>,
    /// Last search/find results for CTX ADD RESULTS.
    pub last_results: Vec<BlockId>,
    /// Focus block for context (protected from pruning).
    pub focus_block: Option<BlockId>,
}

impl AgentSession {
    pub fn new(start_block: BlockId, config: SessionConfig) -> Self {
        let now = Utc::now();
        Self {
            id: AgentSessionId::new(),
            name: config.name,
            cursor: TraversalCursor::new(start_block, config.limits.max_history_size),
            capabilities: config.capabilities,
            limits: config.limits,
            budget: BudgetTracker::new(),
            metrics: SessionMetrics::new(),
            state: SessionState::Active,
            created_at: now,
            last_active: now,
            last_results: Vec::new(),
            focus_block: None,
        }
    }

    /// Check if session is active.
    pub fn is_active(&self) -> bool {
        matches!(self.state, SessionState::Active)
    }

    /// Check if session has timed out.
    pub fn is_timed_out(&self) -> bool {
        let elapsed = Utc::now()
            .signed_duration_since(self.last_active)
            .to_std()
            .unwrap_or_default();
        elapsed >= self.limits.session_timeout
    }

    /// Update last activity timestamp.
    pub fn touch(&mut self) {
        self.last_active = Utc::now();
    }

    /// Mark session as completed.
    pub fn complete(&mut self) {
        self.state = SessionState::Completed;
    }

    /// Mark session as errored.
    pub fn error(&mut self, reason: String) {
        self.state = SessionState::Error { reason };
    }

    /// Pause the session.
    pub fn pause(&mut self) {
        self.state = SessionState::Paused;
    }

    /// Resume the session.
    pub fn resume(&mut self) -> Result<()> {
        match &self.state {
            SessionState::Paused => {
                self.state = SessionState::Active;
                self.touch();
                Ok(())
            }
            SessionState::Active => Ok(()),
            _ => Err(AgentError::SessionClosed(self.id.clone())),
        }
    }

    /// Check if session can perform an operation.
    pub fn check_active(&self) -> Result<()> {
        if !self.is_active() {
            return Err(AgentError::SessionClosed(self.id.clone()));
        }
        if self.is_timed_out() {
            return Err(AgentError::SessionExpired(self.id.clone()));
        }
        Ok(())
    }

    /// Check if session can traverse.
    pub fn check_can_traverse(&self) -> Result<()> {
        self.check_active()?;
        if !self.capabilities.can_traverse {
            return Err(AgentError::OperationNotPermitted {
                operation: "traverse".to_string(),
            });
        }
        Ok(())
    }

    /// Check if session can search.
    pub fn check_can_search(&self) -> Result<()> {
        self.check_active()?;
        if !self.capabilities.can_search {
            return Err(AgentError::OperationNotPermitted {
                operation: "search".to_string(),
            });
        }
        Ok(())
    }

    /// Check if session can modify context.
    pub fn check_can_modify_context(&self) -> Result<()> {
        self.check_active()?;
        if !self.capabilities.can_modify_context {
            return Err(AgentError::OperationNotPermitted {
                operation: "modify_context".to_string(),
            });
        }
        Ok(())
    }

    /// Store last search/find results.
    pub fn store_results(&mut self, results: Vec<BlockId>) {
        self.last_results = results;
    }

    /// Get last results (for CTX ADD RESULTS).
    pub fn get_last_results(&self) -> Result<&[BlockId]> {
        if self.last_results.is_empty() {
            return Err(AgentError::NoResultsAvailable);
        }
        Ok(&self.last_results)
    }

    /// Set focus block.
    pub fn set_focus(&mut self, block_id: Option<BlockId>) {
        self.focus_block = block_id;
    }

    /// Get session info as serializable struct.
    pub fn info(&self) -> SessionInfo {
        SessionInfo {
            id: self.id.to_string(),
            name: self.name.clone(),
            position: self.cursor.position.to_string(),
            state: self.state.clone(),
            created_at: self.created_at,
            last_active: self.last_active,
            history_depth: self.cursor.history_depth(),
            metrics: self.metrics.snapshot(),
        }
    }
}

/// Serializable session info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: String,
    pub name: Option<String>,
    pub position: String,
    pub state: SessionState,
    pub created_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
    pub history_depth: usize,
    pub metrics: crate::metrics::MetricsSnapshot,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block_id(s: &str) -> BlockId {
        s.parse().unwrap_or_else(|_| {
            // Create a deterministic ID from the input string for testing
            let mut bytes = [0u8; 12];
            let s_bytes = s.as_bytes();
            for (i, b) in s_bytes.iter().enumerate() {
                bytes[i % 12] ^= *b;
            }
            BlockId::from_bytes(bytes)
        })
    }

    #[test]
    fn test_session_creation() {
        let session = AgentSession::new(
            block_id("blk_000000000001"),
            SessionConfig::new().with_name("test"),
        );

        assert!(session.is_active());
        assert_eq!(session.name, Some("test".to_string()));
    }

    #[test]
    fn test_session_state_transitions() {
        let mut session = AgentSession::new(block_id("blk_000000000001"), SessionConfig::default());

        assert!(session.is_active());

        session.pause();
        assert!(!session.is_active());
        assert!(matches!(session.state, SessionState::Paused));

        session.resume().unwrap();
        assert!(session.is_active());

        session.complete();
        assert!(!session.is_active());
        assert!(session.resume().is_err());
    }

    #[test]
    fn test_capabilities_check() {
        let session = AgentSession::new(
            block_id("blk_000000000001"),
            SessionConfig::new().with_capabilities(AgentCapabilities::read_only()),
        );

        assert!(session.check_can_traverse().is_ok());
        assert!(session.check_can_search().is_ok());
        assert!(session.check_can_modify_context().is_err());
    }

    #[test]
    fn test_last_results() {
        let mut session = AgentSession::new(block_id("blk_000000000001"), SessionConfig::default());

        // Initially no results
        assert!(session.get_last_results().is_err());

        // Store results
        session.store_results(vec![
            block_id("blk_000000000002"),
            block_id("blk_000000000003"),
        ]);

        let results = session.get_last_results().unwrap();
        assert_eq!(results.len(), 2);
    }
}
