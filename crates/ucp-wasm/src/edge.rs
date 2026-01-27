//! WASM bindings for UCP Edge.

use crate::{PyEdgeId, PyMetadata};
use ucm_core::edge::{Edge, EdgeType};
use wasm_bindgen::prelude::*;

/// WASM wrapper for Edge.
#[wasm_bindgen]
pub struct WasmEdge {
    inner: Edge,
}

#[wasm_bindgen]
impl WasmEdge {
    #[wasm_bindgen(constructor)]
    pub fn new(id: &str, edge_type: &str, from_id: &str, to_id: &str) -> WasmEdge {
        let edge_id = PyEdgeId::new(id).into();
        let from = PyBlockId::new(from_id).into();
        let to = PyBlockId::new(to_id).into();
        let et = EdgeType::from(edge_type);
        WasmEdge {
            inner: Edge::new(edge_id, et, from, to),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn edge_type(&self) -> String {
        self.inner.edge_type().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn from_id(&self) -> String {
        self.inner.from_id().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn to_id(&self) -> String {
        self.inner.to_id().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn metadata(&self) -> Result<JsValue, JsValue> {
        let py_meta: PyMetadata = self.inner.metadata().clone().into();
        py_meta.to_serde()
    }

    #[wasm_bindgen(setter)]
    pub fn set_metadata(&mut self, metadata: &JsValue) -> Result<(), JsValue> {
        let py_meta = PyMetadata::from_serde(metadata)?;
        self.inner.set_metadata(py_meta.into());
        Ok(())
    }

    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.inner).map_err(|e| js_error(e.to_string()))
    }
}
