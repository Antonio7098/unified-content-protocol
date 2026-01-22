//! Snapshot management wrapper for WASM.

use wasm_bindgen::prelude::*;
use ucm_engine::SnapshotManager;

use crate::Document;

/// Manages document snapshots for versioning.
#[wasm_bindgen(js_name = SnapshotManager)]
pub struct WasmSnapshotManager {
    inner: SnapshotManager,
}

#[wasm_bindgen(js_class = SnapshotManager)]
impl WasmSnapshotManager {
    /// Create a new snapshot manager.
    #[wasm_bindgen(constructor)]
    pub fn new(max_snapshots: Option<usize>) -> Self {
        let inner = if let Some(max) = max_snapshots {
            SnapshotManager::with_max_snapshots(max)
        } else {
            SnapshotManager::new()
        };
        Self { inner }
    }

    /// Create a snapshot of a document.
    pub fn create(
        &mut self,
        name: &str,
        doc: &Document,
        description: Option<String>,
    ) -> Result<String, JsValue> {
        self.inner
            .create(name, doc.inner(), description)
            .map(|id| id.0)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Restore a document from a snapshot.
    pub fn restore(&self, name: &str) -> Result<Document, JsValue> {
        self.inner
            .restore(name)
            .map(Document::new)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get information about a snapshot.
    pub fn get(&self, name: &str) -> JsValue {
        match self.inner.get(name) {
            Some(s) => {
                let obj = js_sys::Object::new();
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("name"), &JsValue::from_str(&s.id.0));
                if let Some(desc) = &s.description {
                    let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("description"), &JsValue::from_str(desc));
                }
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("createdAt"), &JsValue::from_str(&s.created_at.to_rfc3339()));
                let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("version"), &JsValue::from_f64(s.document_version.counter as f64));
                obj.into()
            }
            None => JsValue::NULL,
        }
    }

    /// List all snapshots (most recent first).
    pub fn list(&self) -> JsValue {
        let snapshots: Vec<_> = self
            .inner
            .list()
            .iter()
            .map(|s| {
                serde_json::json!({
                    "name": s.id.0,
                    "description": s.description,
                    "createdAt": s.created_at.to_rfc3339(),
                    "version": s.document_version.counter,
                })
            })
            .collect();
        serde_wasm_bindgen::to_value(&snapshots).unwrap_or(JsValue::NULL)
    }

    /// Delete a snapshot.
    pub fn delete(&mut self, name: &str) -> bool {
        self.inner.delete(name)
    }

    /// Check if a snapshot exists.
    pub fn exists(&self, name: &str) -> bool {
        self.inner.exists(name)
    }

    /// Get snapshot count.
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.inner.count()
    }
}
