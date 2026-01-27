//! WASM bindings for UCP (Unified Content Protocol).
//!
//! This crate provides wasm-bindgen bindings exposing the Rust UCP implementation to JavaScript/TypeScript.

#![allow(clippy::useless_conversion)]
#![allow(unexpected_cfgs)]

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

// Core modules
mod agent;
mod block;
mod content;
mod document;
mod edge;
mod engine;
mod errors;
mod llm;
mod observe;
mod section;
mod snapshot;
mod types;

// Re-export key types
pub use types::*;
pub use agent::*;
pub use block::*;
pub use content::*;
pub use document::*;
pub use edge::*;
pub use engine::*;
pub use errors::*;
pub use llm::*;
pub use observe::*;
pub use section::*;
pub use snapshot::*;

/// Initialize the WASM module.
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// Utility function to convert Rust errors to JavaScript errors.
pub fn js_error(err: impl Into<String>) -> JsValue {
    js_sys::Error::new(&err.into()).into()
}
