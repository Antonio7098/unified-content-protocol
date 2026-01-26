//! WebAssembly bindings for UCP (Unified Content Protocol).
//!
//! This crate provides wasm-bindgen bindings exposing the Rust UCP implementation to JavaScript.

use wasm_bindgen::prelude::*;

mod agent;
mod document;
mod engine;
mod errors;
mod llm;
mod observe;
mod section;
mod snapshot;
mod types;

pub use agent::*;
pub use document::*;
pub use engine::*;
pub use errors::*;
pub use llm::*;
pub use observe::*;
pub use section::*;
pub use snapshot::*;
pub use types::*;

/// Initialize panic hook for better error messages in WASM.
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Parse markdown into a Document.
#[wasm_bindgen(js_name = parseMarkdown)]
pub fn parse_markdown(markdown: &str) -> Result<Document, JsValue> {
    let doc = ucp_translator_markdown::parse_markdown(markdown)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(Document::new(doc))
}

/// Render a Document to markdown.
#[wasm_bindgen(js_name = renderMarkdown)]
pub fn render_markdown(doc: &Document) -> Result<String, JsValue> {
    ucp_translator_markdown::render_markdown(doc.inner())
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Parse HTML into a Document.
#[wasm_bindgen(js_name = parseHtml)]
pub fn parse_html(html: &str) -> Result<Document, JsValue> {
    let doc =
        ucp_translator_html::parse_html(html).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(Document::new(doc))
}

/// Execute UCL commands on a document.
#[wasm_bindgen(js_name = executeUcl)]
pub fn execute_ucl(doc: &mut Document, ucl: &str) -> Result<js_sys::Array, JsValue> {
    let client = ucp_api::UcpClient::new();
    let results = client
        .execute_ucl(doc.inner_mut(), ucl)
        .map_err(convert_error)?;

    let arr = js_sys::Array::new();
    for result in results {
        for block_id in result.affected_blocks {
            arr.push(&JsValue::from_str(&block_id.to_string()));
        }
    }
    Ok(arr)
}

/// Create a new empty document.
#[wasm_bindgen(js_name = createDocument)]
pub fn create_document(title: Option<String>) -> Document {
    let mut doc = ucm_core::Document::create();
    if let Some(t) = title {
        doc.metadata.title = Some(t);
    }
    Document::new(doc)
}

/// Get the library version.
#[wasm_bindgen(js_name = version)]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
