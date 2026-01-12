//! Observability utilities for UCP.
//!
//! This crate provides:
//! - Structured event types for document operations
//! - Event bus for subscribing to engine events
//! - Audit logging
//! - Metrics recording

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// =============================================================================
// EVENT TYPES
// =============================================================================

/// Event types emitted by the UCP engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UcpEvent {
    /// Document created
    DocumentCreated {
        document_id: String,
        timestamp: DateTime<Utc>,
    },
    /// Block added
    BlockAdded {
        document_id: String,
        block_id: String,
        parent_id: String,
        content_type: String,
        timestamp: DateTime<Utc>,
    },
    /// Block edited
    BlockEdited {
        document_id: String,
        block_id: String,
        path: String,
        timestamp: DateTime<Utc>,
    },
    /// Block moved
    BlockMoved {
        document_id: String,
        block_id: String,
        old_parent: Option<String>,
        new_parent: String,
        timestamp: DateTime<Utc>,
    },
    /// Block deleted
    BlockDeleted {
        document_id: String,
        block_id: String,
        cascade: bool,
        timestamp: DateTime<Utc>,
    },
    /// Edge created
    EdgeCreated {
        document_id: String,
        source_id: String,
        target_id: String,
        edge_type: String,
        timestamp: DateTime<Utc>,
    },
    /// Edge removed
    EdgeRemoved {
        document_id: String,
        source_id: String,
        target_id: String,
        edge_type: String,
        timestamp: DateTime<Utc>,
    },
    /// Snapshot created
    SnapshotCreated {
        document_id: String,
        snapshot_name: String,
        timestamp: DateTime<Utc>,
    },
    /// Snapshot restored
    SnapshotRestored {
        document_id: String,
        snapshot_name: String,
        timestamp: DateTime<Utc>,
    },
    /// Transaction started
    TransactionStarted {
        document_id: String,
        transaction_id: String,
        timestamp: DateTime<Utc>,
    },
    /// Transaction committed
    TransactionCommitted {
        document_id: String,
        transaction_id: String,
        operation_count: usize,
        timestamp: DateTime<Utc>,
    },
    /// Transaction rolled back
    TransactionRolledBack {
        document_id: String,
        transaction_id: String,
        reason: Option<String>,
        timestamp: DateTime<Utc>,
    },
    /// Validation completed
    ValidationCompleted {
        document_id: String,
        valid: bool,
        error_count: usize,
        warning_count: usize,
        timestamp: DateTime<Utc>,
    },
    /// Custom event
    Custom {
        event_type: String,
        document_id: Option<String>,
        payload: serde_json::Value,
        timestamp: DateTime<Utc>,
    },
}

impl UcpEvent {
    /// Get the event type name
    pub fn event_type(&self) -> &'static str {
        match self {
            UcpEvent::DocumentCreated { .. } => "document_created",
            UcpEvent::BlockAdded { .. } => "block_added",
            UcpEvent::BlockEdited { .. } => "block_edited",
            UcpEvent::BlockMoved { .. } => "block_moved",
            UcpEvent::BlockDeleted { .. } => "block_deleted",
            UcpEvent::EdgeCreated { .. } => "edge_created",
            UcpEvent::EdgeRemoved { .. } => "edge_removed",
            UcpEvent::SnapshotCreated { .. } => "snapshot_created",
            UcpEvent::SnapshotRestored { .. } => "snapshot_restored",
            UcpEvent::TransactionStarted { .. } => "transaction_started",
            UcpEvent::TransactionCommitted { .. } => "transaction_committed",
            UcpEvent::TransactionRolledBack { .. } => "transaction_rolled_back",
            UcpEvent::ValidationCompleted { .. } => "validation_completed",
            UcpEvent::Custom { .. } => "custom",
        }
    }

    /// Get the document ID if present
    pub fn document_id(&self) -> Option<&str> {
        match self {
            UcpEvent::DocumentCreated { document_id, .. } => Some(document_id),
            UcpEvent::BlockAdded { document_id, .. } => Some(document_id),
            UcpEvent::BlockEdited { document_id, .. } => Some(document_id),
            UcpEvent::BlockMoved { document_id, .. } => Some(document_id),
            UcpEvent::BlockDeleted { document_id, .. } => Some(document_id),
            UcpEvent::EdgeCreated { document_id, .. } => Some(document_id),
            UcpEvent::EdgeRemoved { document_id, .. } => Some(document_id),
            UcpEvent::SnapshotCreated { document_id, .. } => Some(document_id),
            UcpEvent::SnapshotRestored { document_id, .. } => Some(document_id),
            UcpEvent::TransactionStarted { document_id, .. } => Some(document_id),
            UcpEvent::TransactionCommitted { document_id, .. } => Some(document_id),
            UcpEvent::TransactionRolledBack { document_id, .. } => Some(document_id),
            UcpEvent::ValidationCompleted { document_id, .. } => Some(document_id),
            UcpEvent::Custom { document_id, .. } => document_id.as_deref(),
        }
    }

    /// Get the timestamp
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            UcpEvent::DocumentCreated { timestamp, .. } => *timestamp,
            UcpEvent::BlockAdded { timestamp, .. } => *timestamp,
            UcpEvent::BlockEdited { timestamp, .. } => *timestamp,
            UcpEvent::BlockMoved { timestamp, .. } => *timestamp,
            UcpEvent::BlockDeleted { timestamp, .. } => *timestamp,
            UcpEvent::EdgeCreated { timestamp, .. } => *timestamp,
            UcpEvent::EdgeRemoved { timestamp, .. } => *timestamp,
            UcpEvent::SnapshotCreated { timestamp, .. } => *timestamp,
            UcpEvent::SnapshotRestored { timestamp, .. } => *timestamp,
            UcpEvent::TransactionStarted { timestamp, .. } => *timestamp,
            UcpEvent::TransactionCommitted { timestamp, .. } => *timestamp,
            UcpEvent::TransactionRolledBack { timestamp, .. } => *timestamp,
            UcpEvent::ValidationCompleted { timestamp, .. } => *timestamp,
            UcpEvent::Custom { timestamp, .. } => *timestamp,
        }
    }
}

// =============================================================================
// EVENT BUS
// =============================================================================

/// Event handler callback type
pub type EventHandler = Arc<dyn Fn(&UcpEvent) + Send + Sync>;

/// Event bus for publishing and subscribing to UCP events
#[derive(Default)]
pub struct EventBus {
    handlers: RwLock<Vec<EventHandler>>,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(Vec::new()),
        }
    }

    /// Subscribe to events
    pub fn subscribe(&self, handler: EventHandler) {
        if let Ok(mut handlers) = self.handlers.write() {
            handlers.push(handler);
        }
    }

    /// Publish an event to all subscribers
    pub fn publish(&self, event: &UcpEvent) {
        if let Ok(handlers) = self.handlers.read() {
            for handler in handlers.iter() {
                handler(event);
            }
        }
    }

    /// Get the number of subscribers
    pub fn subscriber_count(&self) -> usize {
        self.handlers.read().map(|h| h.len()).unwrap_or(0)
    }
}

impl std::fmt::Debug for EventBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventBus")
            .field("subscriber_count", &self.subscriber_count())
            .finish()
    }
}

// =============================================================================
// TRACING
// =============================================================================

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

    #[test]
    fn test_event_bus_publish() {
        let bus = EventBus::new();
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        bus.subscribe(Arc::new(move |event| {
            if matches!(event, UcpEvent::DocumentCreated { .. }) {
                counter_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }));

        let event = UcpEvent::DocumentCreated {
            document_id: "doc_123".into(),
            timestamp: Utc::now(),
        };
        bus.publish(&event);

        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[test]
    fn test_event_bus_multiple_subscribers() {
        let bus = EventBus::new();
        let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        for _ in 0..3 {
            let calls = Arc::clone(&calls);
            bus.subscribe(Arc::new(move |_| {
                calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }));
        }

        let event = UcpEvent::BlockAdded {
            document_id: "doc".into(),
            block_id: "blk_1".into(),
            parent_id: "root".into(),
            content_type: "text".into(),
            timestamp: Utc::now(),
        };
        bus.publish(&event);

        assert_eq!(
            calls.load(std::sync::atomic::Ordering::SeqCst),
            3,
            "All subscribers should receive the event"
        );
        assert_eq!(bus.subscriber_count(), 3);
    }
}
