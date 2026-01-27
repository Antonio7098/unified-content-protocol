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
use llm::{PyIdMapper, PyPromptBuilder, PyPromptBuilderConfig, PyPromptBuilderMode};
use observe::PyObserver;
use section::PySection;
use snapshot::PySnapshot;
use types::{PyBlockId, PyDocumentId, PyEdgeId, PyMetadata, PyTimestamp};

/// A Python module for UCP (Unified Content Protocol).
#[pymodule]
fn ucp_core(_py: Python, m: &PyModule) -> PyResult<()> {
    // Core types
    m.add_class::<PyBlockId>()?;
    m.add_class::<PyDocumentId>()?;
    m.add_class::<PyEdgeId>()?;
    m.add_class::<PyMetadata>()?;
    m.add_class::<PyTimestamp>()?;

    // Content and Document
    m.add_class::<PyContent>()?;
    m.add_class::<PyDocument>()?;
    m.add_class::<PyBlock>()?;
    m.add_class::<PyEdge>()?;
    m.add_class::<PyEdgeType>()?;
    m.add_class::<PySection>()?;
    m.add_class::<PySnapshot>()?;

    // Engine
    m.add_class::<PyEngine>()?;
    m.add_class::<PyEngineConfig>()?;
    m.add_class::<PyResourceLimits>()?;
    m.add_class::<PyTraversalEngine>()?;
    m.add_class::<PyTraversalConfig>()?;
    m.add_class::<PyTraversalDirection>()?;
    m.add_class::<PyTraversalFilter>()?;
    m.add_class::<PyTraversalNode>()?;
    m.add_class::<PyTraversalResult>()?;
    m.add_class::<PyTransactionId>()?;
    m.add_class::<PyValidationIssue>()?;
    m.add_class::<PyValidationPipeline>()?;
    m.add_class::<PyValidationResult>()?;

    // Agent
    m.add_class::<PyAgentTraversal>()?;
    m.add_class::<PyAgentSessionId>()?;
    m.add_class::<PySessionConfig>()?;
    m.add_class::<PyAgentCapabilities>()?;
    m.add_class::<PyViewMode>()?;
    m.add_class::<PyBlockView>()?;
    m.add_class::<PyNeighborhoodView>()?;
    m.add_class::<PyNavigationResult>()?;
    m.add_class::<PyExpansionResult>()?;
    m.add_class::<PySearchResult>()?;
    m.add_class::<PyFindResult>()?;
    m.add_class::<PyConnection>()?;

    // LLM utilities
    m.add_class::<PyIdMapper>()?;
    m.add_class::<PyPromptBuilder>()?;
    m.add_class::<PyPromptBuilderConfig>()?;
    m.add_class::<PyPromptBuilderMode>()?;

    // Observer
    m.add_class::<PyObserver>()?;

    // Errors
    m.add_class::<PyUcpError>()?;
    m.add_class::<PyParseError>()?;
    m.add_class::<PyInvalidBlockIdError>()?;
    m.add_class::<PyBlockNotFoundError>()?;
    m.add_class::<PyCycleDetectedError>()?;
    m.add_class::<PyValidationError>()?;

    Ok(())
}
