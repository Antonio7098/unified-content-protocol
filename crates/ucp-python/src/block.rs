//! Block type wrapper for Python.

use pyo3::prelude::*;
use ucm_core::Block;

use crate::content::PyContent;
use crate::edge::{PyEdge, PyEdgeType};
use crate::types::PyBlockId;

/// A block is the fundamental unit of content in UCM.
#[pyclass(name = "Block")]
#[derive(Clone)]
pub struct PyBlock(pub(crate) Block);

impl PyBlock {
    pub fn new(block: Block) -> Self {
        Self(block)
    }

    pub fn inner(&self) -> &Block {
        &self.0
    }
}

impl From<Block> for PyBlock {
    fn from(block: Block) -> Self {
        Self(block)
    }
}

impl From<&Block> for PyBlock {
    fn from(block: &Block) -> Self {
        Self(block.clone())
    }
}

#[pymethods]
impl PyBlock {
    /// Get the block ID.
    #[getter]
    fn id(&self) -> PyBlockId {
        PyBlockId::from(self.0.id)
    }

    /// Get the content.
    #[getter]
    fn content(&self) -> PyContent {
        PyContent::from(self.0.content.clone())
    }

    /// Get the content type tag.
    #[getter]
    fn content_type(&self) -> &'static str {
        self.0.content_type()
    }

    /// Get the semantic role if set.
    #[getter]
    fn role(&self) -> Option<String> {
        self.0
            .metadata
            .semantic_role
            .as_ref()
            .map(|r| r.to_string())
    }

    /// Get the label if set.
    #[getter]
    fn label(&self) -> Option<String> {
        self.0.metadata.label.clone()
    }

    /// Get the tags.
    #[getter]
    fn tags(&self) -> Vec<String> {
        self.0.metadata.tags.clone()
    }

    /// Get the summary if set.
    #[getter]
    fn summary(&self) -> Option<String> {
        self.0.metadata.summary.clone()
    }

    /// Get the edges.
    #[getter]
    fn edges(&self) -> Vec<PyEdge> {
        self.0.edges.iter().map(PyEdge::from).collect()
    }

    /// Check if this is the root block.
    fn is_root(&self) -> bool {
        self.0.is_root()
    }

    /// Check if the block has a specific tag.
    fn has_tag(&self, tag: &str) -> bool {
        self.0.has_tag(tag)
    }

    /// Get edges of a specific type.
    fn edges_of_type(&self, edge_type: PyEdgeType) -> Vec<PyEdge> {
        let et: ucm_core::EdgeType = edge_type.into();
        self.0
            .edges_of_type(&et)
            .into_iter()
            .map(PyEdge::from)
            .collect()
    }

    /// Get the estimated token count.
    fn token_estimate(&self) -> u32 {
        self.0.token_estimate().generic
    }

    /// Get the content size in bytes.
    fn size_bytes(&self) -> usize {
        self.0.size_bytes()
    }

    /// Get the version counter.
    #[getter]
    fn version(&self) -> u64 {
        self.0.version.counter
    }

    /// Get the creation timestamp as ISO 8601 string.
    #[getter]
    fn created_at(&self) -> String {
        self.0.metadata.created_at.to_rfc3339()
    }

    /// Get the modification timestamp as ISO 8601 string.
    #[getter]
    fn modified_at(&self) -> String {
        self.0.metadata.modified_at.to_rfc3339()
    }

    /// Get the text content if this is a text block.
    fn get_text(&self) -> Option<String> {
        match &self.0.content {
            ucm_core::Content::Text(t) => Some(t.text.clone()),
            ucm_core::Content::Code(c) => Some(c.source.clone()),
            _ => None,
        }
    }

    fn __repr__(&self) -> String {
        let content_preview = match &self.0.content {
            ucm_core::Content::Text(t) => {
                let preview = if t.text.len() > 30 {
                    format!("{}...", &t.text[..30])
                } else {
                    t.text.clone()
                };
                format!("text={:?}", preview)
            }
            ucm_core::Content::Code(c) => format!("code={}", c.language),
            _ => format!("type={}", self.0.content_type()),
        };
        format!("Block(id={}, {})", self.0.id, content_preview)
    }
}
