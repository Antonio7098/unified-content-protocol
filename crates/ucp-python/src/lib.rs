//! Python bindings for UCP (Unified Content Protocol).
//!
//! This crate provides PyO3 bindings exposing the Rust UCP implementation to Python.

use pyo3::prelude::*;

mod block;
mod content;
mod document;
mod edge;
mod engine;
mod errors;
mod llm;
mod snapshot;
mod types;

use block::PyBlock;
use content::PyContent;
use document::PyDocument;
use edge::{PyEdge, PyEdgeType};
use errors::{
    PyBlockNotFoundError, PyCycleDetectedError, PyInvalidBlockIdError, PyParseError, PyUcpError,
    PyValidationError,
};
use llm::{PyIdMapper, PyPromptBuilder, PyPromptPresets, PyUclCapability};
use snapshot::{PySnapshotInfo, PySnapshotManager};
use types::PyBlockId;

/// Parse markdown into a Document.
#[pyfunction]
#[pyo3(name = "parse")]
fn parse_markdown(markdown: &str) -> PyResult<PyDocument> {
    let doc =
        ucp_translator_markdown::parse_markdown(markdown).map_err(|e| PyUcpError::new_err(e.to_string()))?;
    Ok(PyDocument::new(doc))
}

/// Render a Document to markdown.
#[pyfunction]
#[pyo3(name = "render")]
fn render_markdown(doc: &PyDocument) -> PyResult<String> {
    ucp_translator_markdown::render_markdown(doc.inner()).map_err(|e| PyUcpError::new_err(e.to_string()))
}

/// Execute UCL commands on a document.
#[pyfunction]
fn execute_ucl(doc: &mut PyDocument, ucl: &str) -> PyResult<Vec<PyBlockId>> {
    let client = ucp_api::UcpClient::new();
    let results = client
        .execute_ucl(doc.inner_mut(), ucl)
        .map_err(errors::convert_error)?;

    Ok(results
        .iter()
        .flat_map(|r| r.affected_blocks.iter().map(|id| PyBlockId::from(*id)))
        .collect())
}

/// Create a new empty document.
#[pyfunction]
#[pyo3(signature = (title=None))]
fn create(title: Option<&str>) -> PyDocument {
    let mut doc = ucm_core::Document::create();
    if let Some(t) = title {
        doc.metadata.title = Some(t.to_string());
    }
    PyDocument::new(doc)
}

/// Python module initialization.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register exception types
    m.add("UcpError", m.py().get_type_bound::<PyUcpError>())?;
    m.add(
        "BlockNotFoundError",
        m.py().get_type_bound::<PyBlockNotFoundError>(),
    )?;
    m.add(
        "InvalidBlockIdError",
        m.py().get_type_bound::<PyInvalidBlockIdError>(),
    )?;
    m.add(
        "CycleDetectedError",
        m.py().get_type_bound::<PyCycleDetectedError>(),
    )?;
    m.add("ValidationError", m.py().get_type_bound::<PyValidationError>())?;
    m.add("ParseError", m.py().get_type_bound::<PyParseError>())?;

    // Register classes
    m.add_class::<PyBlockId>()?;
    m.add_class::<PyContent>()?;
    m.add_class::<PyBlock>()?;
    m.add_class::<PyDocument>()?;
    m.add_class::<PyEdge>()?;
    m.add_class::<PyEdgeType>()?;

    // LLM utilities
    m.add_class::<PyIdMapper>()?;
    m.add_class::<PyPromptBuilder>()?;
    m.add_class::<PyPromptPresets>()?;
    m.add_class::<PyUclCapability>()?;

    // Snapshot management
    m.add_class::<PySnapshotManager>()?;
    m.add_class::<PySnapshotInfo>()?;

    // Register functions
    m.add_function(wrap_pyfunction!(parse_markdown, m)?)?;
    m.add_function(wrap_pyfunction!(render_markdown, m)?)?;
    m.add_function(wrap_pyfunction!(execute_ucl, m)?)?;
    m.add_function(wrap_pyfunction!(create, m)?)?;

    Ok(())
}
