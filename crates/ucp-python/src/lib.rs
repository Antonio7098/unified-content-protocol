//! Python bindings for UCP (Unified Content Protocol).
//!
//! This crate provides PyO3 bindings exposing the Rust UCP implementation to Python.

#![allow(clippy::useless_conversion)]
#![allow(unexpected_cfgs)]

use pyo3::prelude::*;

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

use agent::{
    PyAgentCapabilities, PyAgentSessionId, PyAgentTraversal, PyBlockView, PyConnection,
    PyExpansionResult, PyFindResult, PyNavigationResult, PyNeighborhoodView, PySearchResult,
    PySessionConfig, PyViewMode,
};
use block::PyBlock;
use content::PyContent;
use document::PyDocument;
use edge::{PyEdge, PyEdgeType};
use engine::{
    PyEngine, PyEngineConfig, PyResourceLimits, PyTransactionId, PyTraversalConfig,
    PyTraversalDirection, PyTraversalEngine, PyTraversalFilter, PyTraversalNode, PyTraversalResult,
    PyValidationIssue, PyValidationPipeline, PyValidationResult,
};
use errors::{
    PyBlockNotFoundError, PyCycleDetectedError, PyInvalidBlockIdError, PyParseError, PyUcpError,
    PyValidationError,
};
use llm::{PyIdMapper, PyPromptBuilder, PyPromptPresets, PyUclCapability};
use observe::{PyAuditEntry, PyEventBus, PyMetricsRecorder, PyUcpEvent};
use section::{write_section, PyClearResult, PyDeletedContent, PyWriteSectionResult};
use snapshot::{PySnapshotInfo, PySnapshotManager};
use types::PyBlockId;

/// Parse markdown into a Document.
#[pyfunction]
#[pyo3(name = "parse")]
fn parse_markdown(markdown: &str) -> PyResult<PyDocument> {
    let doc = ucp_translator_markdown::parse_markdown(markdown)
        .map_err(|e| PyUcpError::new_err(e.to_string()))?;
    Ok(PyDocument::new(doc))
}

/// Render a Document to markdown.
#[pyfunction]
#[pyo3(name = "render")]
fn render_markdown(doc: &PyDocument) -> PyResult<String> {
    ucp_translator_markdown::render_markdown(doc.inner())
        .map_err(|e| PyUcpError::new_err(e.to_string()))
}

/// Parse HTML into a Document.
#[pyfunction]
fn parse_html(html: &str) -> PyResult<PyDocument> {
    let doc =
        ucp_translator_html::parse_html(html).map_err(|e| PyUcpError::new_err(e.to_string()))?;
    Ok(PyDocument::new(doc))
}

/// Clear a section's content with undo support.
#[pyfunction]
fn clear_section_with_undo(
    doc: &mut PyDocument,
    section_id: &PyBlockId,
) -> PyResult<PyClearResult> {
    let result =
        ucm_engine::section::clear_section_content_with_undo(doc.inner_mut(), section_id.inner())
            .map_err(|e| PyUcpError::new_err(e.to_string()))?;
    Ok(PyClearResult::from(result))
}

/// Restore previously deleted section content.
#[pyfunction]
fn restore_deleted_section(
    doc: &mut PyDocument,
    deleted: &PyDeletedContent,
) -> PyResult<Vec<PyBlockId>> {
    let restored = ucm_engine::section::restore_deleted_content(doc.inner_mut(), deleted.inner())
        .map_err(|e| PyUcpError::new_err(e.to_string()))?;
    Ok(restored.into_iter().map(PyBlockId::from).collect())
}

/// Find a section by path (e.g., "Introduction > Getting Started").
#[pyfunction]
fn find_section_by_path(doc: &PyDocument, path: &str) -> Option<PyBlockId> {
    ucm_engine::section::find_section_by_path(doc.inner(), path).map(PyBlockId::from)
}

/// Get all sections (heading blocks) in the document.
#[pyfunction]
fn get_all_sections(doc: &PyDocument) -> Vec<(PyBlockId, usize)> {
    ucm_engine::section::get_all_sections(doc.inner())
        .into_iter()
        .map(|(id, level)| (PyBlockId::from(id), level))
        .collect()
}

/// Get the depth of a section in the document hierarchy.
#[pyfunction]
fn get_section_depth(doc: &PyDocument, section_id: &PyBlockId) -> Option<usize> {
    ucm_engine::section::get_section_depth(doc.inner(), section_id.inner())
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
    m.add(
        "ValidationError",
        m.py().get_type_bound::<PyValidationError>(),
    )?;
    m.add("ParseError", m.py().get_type_bound::<PyParseError>())?;

    // Register classes
    m.add_class::<PyBlockId>()?;
    m.add_class::<PyContent>()?;
    m.add_class::<PyBlock>()?;
    m.add_class::<PyDocument>()?;
    m.add_class::<PyEdge>()?;
    m.add_class::<PyEdgeType>()?;
    m.add_class::<PyAgentTraversal>()?;
    m.add_class::<PyAgentSessionId>()?;
    m.add_class::<PySessionConfig>()?;
    m.add_class::<PyAgentCapabilities>()?;
    m.add_class::<PyViewMode>()?;
    m.add_class::<PyNavigationResult>()?;
    m.add_class::<PyExpansionResult>()?;
    m.add_class::<PyBlockView>()?;
    m.add_class::<PySearchResult>()?;
    m.add_class::<PyFindResult>()?;
    m.add_class::<PyNeighborhoodView>()?;
    m.add_class::<PyConnection>()?;

    // LLM utilities
    m.add_class::<PyIdMapper>()?;
    m.add_class::<PyPromptBuilder>()?;
    m.add_class::<PyPromptPresets>()?;
    m.add_class::<PyUclCapability>()?;

    // Snapshot management
    m.add_class::<PySnapshotManager>()?;
    m.add_class::<PySnapshotInfo>()?;

    // Observability classes
    m.add_class::<PyUcpEvent>()?;
    m.add_class::<PyEventBus>()?;
    m.add_class::<PyAuditEntry>()?;
    m.add_class::<PyMetricsRecorder>()?;

    // Section utilities
    m.add_class::<PyClearResult>()?;
    m.add_class::<PyDeletedContent>()?;
    m.add_class::<PyWriteSectionResult>()?;

    // Engine and validation classes
    m.add_class::<PyEngine>()?;
    m.add_class::<PyEngineConfig>()?;
    m.add_class::<PyTransactionId>()?;
    m.add_class::<PyResourceLimits>()?;
    m.add_class::<PyValidationPipeline>()?;
    m.add_class::<PyValidationResult>()?;
    m.add_class::<PyValidationIssue>()?;

    // Traversal classes
    m.add_class::<PyTraversalEngine>()?;
    m.add_class::<PyTraversalConfig>()?;
    m.add_class::<PyTraversalFilter>()?;
    m.add_class::<PyTraversalDirection>()?;
    m.add_class::<PyTraversalResult>()?;
    m.add_class::<PyTraversalNode>()?;

    // Register functions
    m.add_function(wrap_pyfunction!(parse_markdown, m)?)?;
    m.add_function(wrap_pyfunction!(render_markdown, m)?)?;
    m.add_function(wrap_pyfunction!(parse_html, m)?)?;
    m.add_function(wrap_pyfunction!(execute_ucl, m)?)?;
    m.add_function(wrap_pyfunction!(create, m)?)?;

    // Section functions
    m.add_function(wrap_pyfunction!(clear_section_with_undo, m)?)?;
    m.add_function(wrap_pyfunction!(restore_deleted_section, m)?)?;
    m.add_function(wrap_pyfunction!(write_section, m)?)?;
    m.add_function(wrap_pyfunction!(find_section_by_path, m)?)?;
    m.add_function(wrap_pyfunction!(get_all_sections, m)?)?;
    m.add_function(wrap_pyfunction!(get_section_depth, m)?)?;

    Ok(())
}
