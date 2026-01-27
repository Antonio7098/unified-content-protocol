//! WASM bindings for UCP Content.

use serde::{Deserialize, Serialize};
use ucm_core::content::Content;
use wasm_bindgen::prelude::*;

/// WASM wrapper for Content.
#[wasm_bindgen]
pub struct WasmContent {
    inner: Content,
}

#[wasm_bindgen]
impl WasmContent {
    #[wasm_bindgen(constructor)]
    pub fn new(content_type: &str, data: &str) -> WasmContent {
        WasmContent {
            inner: Content::new(content_type, data),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn content_type(&self) -> String {
        self.inner.content_type().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> String {
        self.inner.data().to_string()
    }

    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[wasm_bindgen(js_name = sizeInBytes)]
    pub fn size_in_bytes(&self) -> usize {
        self.inner.size_in_bytes()
    }

    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.inner).map_err(|e| js_error(e.to_string()))
    }
}
