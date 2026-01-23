//! Observability bindings for Python.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::sync::Arc;
use ucp_observe::{AuditEntry, EventBus, MetricsRecorder, UcpEvent};

/// UCP event wrapper for Python.
#[pyclass(name = "UcpEvent")]
#[derive(Clone)]
pub struct PyUcpEvent {
    event_type: String,
    document_id: Option<String>,
    timestamp: String,
    details: String,
}

#[pymethods]
impl PyUcpEvent {
    /// Create a document created event.
    #[staticmethod]
    fn document_created(document_id: &str) -> Self {
        let event = UcpEvent::DocumentCreated {
            document_id: document_id.to_string(),
            timestamp: chrono::Utc::now(),
        };
        Self::from_event(&event)
    }

    /// Create a block added event.
    #[staticmethod]
    fn block_added(document_id: &str, block_id: &str, parent_id: &str, content_type: &str) -> Self {
        let event = UcpEvent::BlockAdded {
            document_id: document_id.to_string(),
            block_id: block_id.to_string(),
            parent_id: parent_id.to_string(),
            content_type: content_type.to_string(),
            timestamp: chrono::Utc::now(),
        };
        Self::from_event(&event)
    }

    /// Create a block deleted event.
    #[staticmethod]
    fn block_deleted(document_id: &str, block_id: &str, cascade: bool) -> Self {
        let event = UcpEvent::BlockDeleted {
            document_id: document_id.to_string(),
            block_id: block_id.to_string(),
            cascade,
            timestamp: chrono::Utc::now(),
        };
        Self::from_event(&event)
    }

    /// Create a snapshot created event.
    #[staticmethod]
    fn snapshot_created(document_id: &str, snapshot_name: &str) -> Self {
        let event = UcpEvent::SnapshotCreated {
            document_id: document_id.to_string(),
            snapshot_name: snapshot_name.to_string(),
            timestamp: chrono::Utc::now(),
        };
        Self::from_event(&event)
    }

    /// Get the event type.
    #[getter]
    fn event_type(&self) -> &str {
        &self.event_type
    }

    /// Get the document ID if present.
    #[getter]
    fn document_id(&self) -> Option<&str> {
        self.document_id.as_deref()
    }

    /// Get the timestamp as ISO 8601 string.
    #[getter]
    fn timestamp(&self) -> &str {
        &self.timestamp
    }

    /// Get event details as JSON string.
    #[getter]
    fn details(&self) -> &str {
        &self.details
    }

    fn __repr__(&self) -> String {
        format!(
            "UcpEvent(type={}, doc={})",
            self.event_type,
            self.document_id.as_deref().unwrap_or("None")
        )
    }
}

impl PyUcpEvent {
    fn from_event(event: &UcpEvent) -> Self {
        Self {
            event_type: event.event_type().to_string(),
            document_id: event.document_id().map(|s| s.to_string()),
            timestamp: event.timestamp().to_rfc3339(),
            details: serde_json::to_string(event).unwrap_or_default(),
        }
    }
}

/// Event bus for subscribing to UCP events.
#[pyclass(name = "EventBus")]
pub struct PyEventBus {
    inner: Arc<EventBus>,
}

#[pymethods]
impl PyEventBus {
    /// Create a new event bus.
    #[new]
    fn new() -> Self {
        Self {
            inner: Arc::new(EventBus::new()),
        }
    }

    /// Get the number of subscribers.
    #[getter]
    fn subscriber_count(&self) -> usize {
        self.inner.subscriber_count()
    }

    fn __repr__(&self) -> String {
        format!("EventBus(subscribers={})", self.subscriber_count())
    }
}

/// Audit log entry.
#[pyclass(name = "AuditEntry")]
#[derive(Clone)]
pub struct PyAuditEntry {
    inner: AuditEntry,
}

#[pymethods]
impl PyAuditEntry {
    /// Create a new audit entry.
    #[new]
    fn new(operation: &str, document_id: &str) -> Self {
        Self {
            inner: AuditEntry::new(operation, document_id),
        }
    }

    /// Set the user ID.
    fn with_user(&mut self, user_id: &str) -> Self {
        Self {
            inner: self.inner.clone().with_user(user_id),
        }
    }

    /// Set the duration in milliseconds.
    fn with_duration(&mut self, duration_ms: u64) -> Self {
        Self {
            inner: self.inner.clone().with_duration(duration_ms),
        }
    }

    /// Mark as failed.
    fn failed(&mut self) -> Self {
        Self {
            inner: self.inner.clone().failed(),
        }
    }

    /// Get the operation name.
    #[getter]
    fn operation(&self) -> &str {
        &self.inner.operation
    }

    /// Get the document ID.
    #[getter]
    fn document_id(&self) -> &str {
        &self.inner.document_id
    }

    /// Get the user ID if present.
    #[getter]
    fn user_id(&self) -> Option<&str> {
        self.inner.user_id.as_deref()
    }

    /// Check if the operation was successful.
    #[getter]
    fn success(&self) -> bool {
        self.inner.success
    }

    /// Get the duration in milliseconds.
    #[getter]
    fn duration_ms(&self) -> u64 {
        self.inner.duration_ms
    }

    /// Get the timestamp as ISO 8601 string.
    #[getter]
    fn timestamp(&self) -> String {
        self.inner.timestamp.to_rfc3339()
    }

    /// Convert to dict.
    fn to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        dict.set_item("operation", &self.inner.operation)?;
        dict.set_item("document_id", &self.inner.document_id)?;
        dict.set_item("user_id", &self.inner.user_id)?;
        dict.set_item("success", self.inner.success)?;
        dict.set_item("duration_ms", self.inner.duration_ms)?;
        dict.set_item("timestamp", self.inner.timestamp.to_rfc3339())?;
        Ok(dict.into())
    }

    fn __repr__(&self) -> String {
        format!(
            "AuditEntry(op={}, doc={}, success={})",
            self.inner.operation, self.inner.document_id, self.inner.success
        )
    }
}

/// Simple metrics recorder.
#[pyclass(name = "MetricsRecorder")]
pub struct PyMetricsRecorder {
    inner: MetricsRecorder,
}

#[pymethods]
impl PyMetricsRecorder {
    /// Create a new metrics recorder.
    #[new]
    fn new() -> Self {
        Self {
            inner: MetricsRecorder::new(),
        }
    }

    /// Record an operation.
    fn record_operation(&mut self, success: bool) {
        self.inner.record_operation(success);
    }

    /// Record a block creation.
    fn record_block_created(&mut self) {
        self.inner.record_block_created();
    }

    /// Record a block deletion.
    fn record_block_deleted(&mut self) {
        self.inner.record_block_deleted();
    }

    /// Record a snapshot creation.
    fn record_snapshot(&mut self) {
        self.inner.record_snapshot();
    }

    /// Get total operations count.
    #[getter]
    fn operations_total(&self) -> u64 {
        self.inner.operations_total
    }

    /// Get failed operations count.
    #[getter]
    fn operations_failed(&self) -> u64 {
        self.inner.operations_failed
    }

    /// Get blocks created count.
    #[getter]
    fn blocks_created(&self) -> u64 {
        self.inner.blocks_created
    }

    /// Get blocks deleted count.
    #[getter]
    fn blocks_deleted(&self) -> u64 {
        self.inner.blocks_deleted
    }

    /// Get snapshots created count.
    #[getter]
    fn snapshots_created(&self) -> u64 {
        self.inner.snapshots_created
    }

    /// Convert to dict.
    fn to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        dict.set_item("operations_total", self.inner.operations_total)?;
        dict.set_item("operations_failed", self.inner.operations_failed)?;
        dict.set_item("blocks_created", self.inner.blocks_created)?;
        dict.set_item("blocks_deleted", self.inner.blocks_deleted)?;
        dict.set_item("snapshots_created", self.inner.snapshots_created)?;
        Ok(dict.into())
    }

    fn __repr__(&self) -> String {
        format!(
            "MetricsRecorder(ops={}, failed={}, blocks_created={})",
            self.inner.operations_total, self.inner.operations_failed, self.inner.blocks_created
        )
    }
}
