//! Error handling for WASM bindings.

use serde::Serialize;
use wasm_bindgen::prelude::*;

/// Error information returned to JavaScript.
#[derive(Serialize)]
pub struct UcpError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

/// Convert a Rust UCM error to a JsValue.
pub fn convert_error(err: ucm_core::Error) -> JsValue {
    let ucp_err = UcpError {
        code: err
            .code()
            .map(|c| c.code().to_string())
            .unwrap_or_else(|| "E900".to_string()),
        message: err.to_string(),
        block_id: match &err {
            ucm_core::Error::BlockNotFound(id) => Some(id.clone()),
            ucm_core::Error::InvalidBlockId(id) => Some(id.clone()),
            ucm_core::Error::CycleDetected(id) => Some(id.clone()),
            _ => None,
        },
    };

    serde_wasm_bindgen::to_value(&ucp_err).unwrap_or_else(|_| JsValue::from_str(&err.to_string()))
}

/// Helper trait for ergonomic error conversion.
pub trait IntoWasmResult<T> {
    fn into_wasm_result(self) -> Result<T, JsValue>;
}

impl<T> IntoWasmResult<T> for ucm_core::Result<T> {
    fn into_wasm_result(self) -> Result<T, JsValue> {
        self.map_err(convert_error)
    }
}
