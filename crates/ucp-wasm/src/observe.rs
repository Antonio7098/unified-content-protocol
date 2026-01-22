//! Observability bindings for WASM.

use wasm_bindgen::prelude::*;
use ucp_observe::{AuditEntry, MetricsRecorder, UcpEvent};

/// UCP event wrapper for WASM.
#[wasm_bindgen]
pub struct WasmUcpEvent {
    event_type: String,
    document_id: Option<String>,
    timestamp: String,
    details: String,
}

#[wasm_bindgen]
impl WasmUcpEvent {
    /// Create a document created event.
    #[wasm_bindgen(js_name = documentCreated)]
    pub fn document_created(document_id: &str) -> WasmUcpEvent {
        let event = UcpEvent::DocumentCreated {
            document_id: document_id.to_string(),
            timestamp: chrono::Utc::now(),
        };
        Self::from_event(&event)
    }

    /// Create a block added event.
    #[wasm_bindgen(js_name = blockAdded)]
    pub fn block_added(document_id: &str, block_id: &str, parent_id: &str, content_type: &str) -> WasmUcpEvent {
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
    #[wasm_bindgen(js_name = blockDeleted)]
    pub fn block_deleted(document_id: &str, block_id: &str, cascade: bool) -> WasmUcpEvent {
        let event = UcpEvent::BlockDeleted {
            document_id: document_id.to_string(),
            block_id: block_id.to_string(),
            cascade,
            timestamp: chrono::Utc::now(),
        };
        Self::from_event(&event)
    }

    /// Create a snapshot created event.
    #[wasm_bindgen(js_name = snapshotCreated)]
    pub fn snapshot_created(document_id: &str, snapshot_name: &str) -> WasmUcpEvent {
        let event = UcpEvent::SnapshotCreated {
            document_id: document_id.to_string(),
            snapshot_name: snapshot_name.to_string(),
            timestamp: chrono::Utc::now(),
        };
        Self::from_event(&event)
    }

    /// Get the event type.
    #[wasm_bindgen(getter, js_name = eventType)]
    pub fn event_type(&self) -> String {
        self.event_type.clone()
    }

    /// Get the document ID if present.
    #[wasm_bindgen(getter, js_name = documentId)]
    pub fn document_id(&self) -> Option<String> {
        self.document_id.clone()
    }

    /// Get the timestamp as ISO 8601 string.
    #[wasm_bindgen(getter)]
    pub fn timestamp(&self) -> String {
        self.timestamp.clone()
    }

    /// Get event details as JSON string.
    #[wasm_bindgen(getter)]
    pub fn details(&self) -> String {
        self.details.clone()
    }

    /// Convert to JSON object.
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> JsValue {
        let obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("eventType"), &JsValue::from_str(&self.event_type));
        if let Some(doc_id) = &self.document_id {
            let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("documentId"), &JsValue::from_str(doc_id));
        }
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("timestamp"), &JsValue::from_str(&self.timestamp));
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("details"), &JsValue::from_str(&self.details));
        obj.into()
    }
}

impl WasmUcpEvent {
    fn from_event(event: &UcpEvent) -> Self {
        Self {
            event_type: event.event_type().to_string(),
            document_id: event.document_id().map(|s| s.to_string()),
            timestamp: event.timestamp().to_rfc3339(),
            details: serde_json::to_string(event).unwrap_or_default(),
        }
    }
}

/// Audit log entry.
#[wasm_bindgen]
pub struct WasmAuditEntry {
    inner: AuditEntry,
}

#[wasm_bindgen]
impl WasmAuditEntry {
    /// Create a new audit entry.
    #[wasm_bindgen(constructor)]
    pub fn new(operation: &str, document_id: &str) -> WasmAuditEntry {
        WasmAuditEntry {
            inner: AuditEntry::new(operation, document_id),
        }
    }

    /// Set the user ID.
    #[wasm_bindgen(js_name = withUser)]
    pub fn with_user(mut self, user_id: &str) -> WasmAuditEntry {
        self.inner = self.inner.with_user(user_id);
        self
    }

    /// Set the duration in milliseconds.
    #[wasm_bindgen(js_name = withDuration)]
    pub fn with_duration(mut self, duration_ms: u64) -> WasmAuditEntry {
        self.inner = self.inner.with_duration(duration_ms);
        self
    }

    /// Mark as failed.
    pub fn failed(mut self) -> WasmAuditEntry {
        self.inner = self.inner.failed();
        self
    }

    /// Get the operation name.
    #[wasm_bindgen(getter)]
    pub fn operation(&self) -> String {
        self.inner.operation.clone()
    }

    /// Get the document ID.
    #[wasm_bindgen(getter, js_name = documentId)]
    pub fn document_id(&self) -> String {
        self.inner.document_id.clone()
    }

    /// Get the user ID if present.
    #[wasm_bindgen(getter, js_name = userId)]
    pub fn user_id(&self) -> Option<String> {
        self.inner.user_id.clone()
    }

    /// Check if the operation was successful.
    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool {
        self.inner.success
    }

    /// Get the duration in milliseconds.
    #[wasm_bindgen(getter, js_name = durationMs)]
    pub fn duration_ms(&self) -> u64 {
        self.inner.duration_ms
    }

    /// Get the timestamp as ISO 8601 string.
    #[wasm_bindgen(getter)]
    pub fn timestamp(&self) -> String {
        self.inner.timestamp.to_rfc3339()
    }

    /// Convert to JSON object.
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> JsValue {
        let obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("operation"), &JsValue::from_str(&self.inner.operation));
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("documentId"), &JsValue::from_str(&self.inner.document_id));
        if let Some(user_id) = &self.inner.user_id {
            let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("userId"), &JsValue::from_str(user_id));
        }
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("success"), &JsValue::from_bool(self.inner.success));
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("durationMs"), &JsValue::from_f64(self.inner.duration_ms as f64));
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("timestamp"), &JsValue::from_str(&self.inner.timestamp.to_rfc3339()));
        obj.into()
    }
}

/// Simple metrics recorder.
#[wasm_bindgen]
pub struct WasmMetricsRecorder {
    inner: MetricsRecorder,
}

#[wasm_bindgen]
impl WasmMetricsRecorder {
    /// Create a new metrics recorder.
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmMetricsRecorder {
        WasmMetricsRecorder {
            inner: MetricsRecorder::new(),
        }
    }

    /// Record an operation.
    #[wasm_bindgen(js_name = recordOperation)]
    pub fn record_operation(&mut self, success: bool) {
        self.inner.record_operation(success);
    }

    /// Record a block creation.
    #[wasm_bindgen(js_name = recordBlockCreated)]
    pub fn record_block_created(&mut self) {
        self.inner.record_block_created();
    }

    /// Record a block deletion.
    #[wasm_bindgen(js_name = recordBlockDeleted)]
    pub fn record_block_deleted(&mut self) {
        self.inner.record_block_deleted();
    }

    /// Record a snapshot creation.
    #[wasm_bindgen(js_name = recordSnapshot)]
    pub fn record_snapshot(&mut self) {
        self.inner.record_snapshot();
    }

    /// Get total operations count.
    #[wasm_bindgen(getter, js_name = operationsTotal)]
    pub fn operations_total(&self) -> u64 {
        self.inner.operations_total
    }

    /// Get failed operations count.
    #[wasm_bindgen(getter, js_name = operationsFailed)]
    pub fn operations_failed(&self) -> u64 {
        self.inner.operations_failed
    }

    /// Get blocks created count.
    #[wasm_bindgen(getter, js_name = blocksCreated)]
    pub fn blocks_created(&self) -> u64 {
        self.inner.blocks_created
    }

    /// Get blocks deleted count.
    #[wasm_bindgen(getter, js_name = blocksDeleted)]
    pub fn blocks_deleted(&self) -> u64 {
        self.inner.blocks_deleted
    }

    /// Get snapshots created count.
    #[wasm_bindgen(getter, js_name = snapshotsCreated)]
    pub fn snapshots_created(&self) -> u64 {
        self.inner.snapshots_created
    }

    /// Convert to JSON object.
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> JsValue {
        let obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("operationsTotal"), &JsValue::from_f64(self.inner.operations_total as f64));
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("operationsFailed"), &JsValue::from_f64(self.inner.operations_failed as f64));
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("blocksCreated"), &JsValue::from_f64(self.inner.blocks_created as f64));
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("blocksDeleted"), &JsValue::from_f64(self.inner.blocks_deleted as f64));
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("snapshotsCreated"), &JsValue::from_f64(self.inner.snapshots_created as f64));
        obj.into()
    }
}

impl Default for WasmMetricsRecorder {
    fn default() -> Self {
        Self::new()
    }
}
