//! Observability utilities for UCP.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize tracing with default configuration
pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer().with_target(true).with_thread_ids(true))
        .init();
}

/// Initialize tracing with compact output
pub fn init_tracing_compact() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer().compact())
        .init();
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub operation: String,
    pub document_id: String,
    pub user_id: Option<String>,
    pub details: serde_json::Value,
    pub success: bool,
    pub duration_ms: u64,
}

impl AuditEntry {
    pub fn new(operation: impl Into<String>, document_id: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            operation: operation.into(),
            document_id: document_id.into(),
            user_id: None,
            details: serde_json::Value::Null,
            success: true,
            duration_ms: 0,
        }
    }

    pub fn with_user(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = details;
        self
    }

    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    pub fn failed(mut self) -> Self {
        self.success = false;
        self
    }
}

/// Simple metrics recorder
#[derive(Debug, Default)]
pub struct MetricsRecorder {
    pub operations_total: u64,
    pub operations_failed: u64,
    pub blocks_created: u64,
    pub blocks_deleted: u64,
    pub snapshots_created: u64,
}

impl MetricsRecorder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_operation(&mut self, success: bool) {
        self.operations_total += 1;
        if !success {
            self.operations_failed += 1;
        }
    }

    pub fn record_block_created(&mut self) {
        self.blocks_created += 1;
    }

    pub fn record_block_deleted(&mut self) {
        self.blocks_deleted += 1;
    }

    pub fn record_snapshot(&mut self) {
        self.snapshots_created += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_entry() {
        let entry = AuditEntry::new("EDIT", "doc_123")
            .with_user("user_456")
            .with_duration(42);
        
        assert_eq!(entry.operation, "EDIT");
        assert!(entry.success);
    }

    #[test]
    fn test_metrics() {
        let mut m = MetricsRecorder::new();
        m.record_operation(true);
        m.record_operation(false);
        m.record_block_created();
        
        assert_eq!(m.operations_total, 2);
        assert_eq!(m.operations_failed, 1);
        assert_eq!(m.blocks_created, 1);
    }
}
