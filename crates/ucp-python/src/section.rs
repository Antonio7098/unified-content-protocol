//! Section management bindings for Python.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use ucm_engine::section::{
    clear_section_content_with_undo, integrate_section_blocks, restore_deleted_content,
    ClearResult, DeletedContent,
};

use crate::document::PyDocument;
use crate::types::PyBlockId;

/// Write markdown content into a section, replacing its children.
#[pyfunction]
#[pyo3(signature = (doc, section_id, markdown, base_heading_level=None))]
pub fn write_section(
    doc: &mut PyDocument,
    section_id: &PyBlockId,
    markdown: &str,
    base_heading_level: Option<usize>,
) -> PyResult<PyWriteSectionResult> {
    // Clear existing content with undo support
    let ClearResult {
        removed_ids,
        deleted_content,
    } = clear_section_content_with_undo(doc.inner_mut(), section_id.inner()).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!(
            "Failed to clear section {}: {}",
            section_id.inner(),
            e
        ))
    })?;
    let removed_ids_py: Vec<PyBlockId> =
        removed_ids.iter().map(|id| PyBlockId::from(*id)).collect();

    // Parse new markdown into a temporary document
    let temp_doc = ucp_translator_markdown::parse_markdown(markdown)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

    // Integrate parsed blocks into section
    let added = integrate_section_blocks(
        doc.inner_mut(),
        section_id.inner(),
        &temp_doc,
        base_heading_level,
    )
    .map_err(|e| {
        let _ = restore_deleted_content(doc.inner_mut(), &deleted_content);
        pyo3::exceptions::PyRuntimeError::new_err(format!(
            "Failed to integrate section {}: {}",
            section_id.inner(),
            e
        ))
    })?;

    Ok(PyWriteSectionResult {
        success: true,
        section_id: section_id.clone(),
        blocks_removed: removed_ids_py,
        blocks_added: added.into_iter().map(PyBlockId::from).collect(),
    })
}

/// Result of writing markdown into a section.
#[pyclass(name = "WriteSectionResult")]
#[derive(Clone)]
pub struct PyWriteSectionResult {
    #[pyo3(get)]
    success: bool,
    #[pyo3(get)]
    section_id: PyBlockId,
    #[pyo3(get)]
    blocks_removed: Vec<PyBlockId>,
    #[pyo3(get)]
    blocks_added: Vec<PyBlockId>,
}

#[pymethods]
impl PyWriteSectionResult {
    fn __repr__(&self) -> String {
        format!(
            "WriteSectionResult(success={}, removed={}, added={})",
            self.success,
            self.blocks_removed.len(),
            self.blocks_added.len()
        )
    }
}

/// Result of a section clear operation with undo support.
#[pyclass(name = "ClearResult")]
#[derive(Clone)]
pub struct PyClearResult {
    removed_ids: Vec<PyBlockId>,
    deleted_content: PyDeletedContent,
}

impl From<ClearResult> for PyClearResult {
    fn from(result: ClearResult) -> Self {
        Self {
            removed_ids: result
                .removed_ids
                .into_iter()
                .map(PyBlockId::from)
                .collect(),
            deleted_content: PyDeletedContent::from(result.deleted_content),
        }
    }
}

#[pymethods]
impl PyClearResult {
    /// Get the IDs of removed blocks.
    #[getter]
    fn removed_ids(&self) -> Vec<PyBlockId> {
        self.removed_ids.clone()
    }

    /// Get the deleted content for potential restoration.
    #[getter]
    fn deleted_content(&self) -> PyDeletedContent {
        self.deleted_content.clone()
    }

    /// Get the number of removed blocks.
    fn __len__(&self) -> usize {
        self.removed_ids.len()
    }

    fn __repr__(&self) -> String {
        format!("ClearResult(removed={})", self.removed_ids.len())
    }
}

/// Deleted content that can be restored.
#[pyclass(name = "DeletedContent")]
#[derive(Clone)]
pub struct PyDeletedContent {
    inner: DeletedContent,
}

impl PyDeletedContent {
    pub fn inner(&self) -> &DeletedContent {
        &self.inner
    }
}

impl From<DeletedContent> for PyDeletedContent {
    fn from(deleted: DeletedContent) -> Self {
        Self { inner: deleted }
    }
}

#[pymethods]
impl PyDeletedContent {
    /// Check if there is any deleted content.
    #[getter]
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Get the number of deleted blocks.
    #[getter]
    fn block_count(&self) -> usize {
        self.inner.block_count()
    }

    /// Get all block IDs in the deleted content.
    fn block_ids(&self) -> Vec<PyBlockId> {
        self.inner
            .block_ids()
            .into_iter()
            .map(PyBlockId::from)
            .collect()
    }

    /// Get the parent block ID where this content was attached.
    #[getter]
    fn parent_id(&self) -> PyBlockId {
        PyBlockId::from(self.inner.parent_id)
    }

    /// Get the deletion timestamp as ISO 8601 string.
    #[getter]
    fn deleted_at(&self) -> String {
        self.inner.deleted_at.to_rfc3339()
    }

    /// Convert to dict for serialization.
    fn to_dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        dict.set_item("parent_id", self.inner.parent_id.to_string())?;
        dict.set_item("block_count", self.inner.block_count())?;
        dict.set_item("deleted_at", self.inner.deleted_at.to_rfc3339())?;

        let block_ids: Vec<String> = self
            .inner
            .block_ids()
            .iter()
            .map(|id| id.to_string())
            .collect();
        dict.set_item("block_ids", block_ids)?;

        Ok(dict.into())
    }

    /// Serialize to JSON string for persistence.
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Deserialize from JSON string.
    #[staticmethod]
    fn from_json(json_str: &str) -> PyResult<Self> {
        let deleted: DeletedContent = serde_json::from_str(json_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(Self { inner: deleted })
    }

    fn __len__(&self) -> usize {
        self.inner.block_count()
    }

    fn __repr__(&self) -> String {
        format!(
            "DeletedContent(parent={}, blocks={})",
            self.inner.parent_id,
            self.inner.block_count()
        )
    }
}
