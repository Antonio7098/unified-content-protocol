//! WASM bindings for UCP Block.

use crate::{PyBlockId, PyContent, PyMetadata};
use ucm_core::block::Block;
use wasm_bindgen::prelude::*;

/// WASM wrapper for Block.
#[wasm_bindgen]
pub struct WasmBlock {
    inner: Block,
}

#[wasm_bindgen]
impl WasmBlock {
    #[wasm_bindgen(constructor)]
    pub fn new(id: &str, content: &JsValue) -> Result<WasmBlock, JsValue> {
        let block_id = PyBlockId::new(id).into();
        let py_content = PyContent::from_serde(content)?;
        let content = py_content.into();
        Ok(WasmBlock {
            inner: Block::new(block_id, content),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> Result<JsValue, JsValue> {
        let py_content: PyContent = self.inner.content().clone().into();
        py_content.to_serde()
    }

    #[wasm_bindgen(getter)]
    pub fn metadata(&self) -> Result<JsValue, JsValue> {
        let py_meta: PyMetadata = self.inner.metadata().clone().into();
        py_meta.to_serde()
    }

    #[wasm_bindgen(setter)]
    pub fn set_content(&mut self, content: &JsValue) -> Result<(), JsValue> {
        let py_content = PyContent::from_serde(content)?;
        self.inner.set_content(py_content.into());
        Ok(())
    }

    #[wasm_bindgen(setter)]
    pub fn set_metadata(&mut self, metadata: &JsValue) -> Result<(), JsValue> {
        let py_meta = PyMetadata::from_serde(metadata)?;
        self.inner.set_metadata(py_meta.into());
        Ok(())
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
