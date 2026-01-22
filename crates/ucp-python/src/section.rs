//! Section management bindings for Python.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use ucm_engine::section::{ClearResult, DeletedContent};

use crate::types::PyBlockId;

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
            removed_ids: result.removed_ids.into_iter().map(PyBlockId::from).collect(),
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
        self.inner.block_ids().into_iter().map(PyBlockId::from).collect()
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
        
        let block_ids: Vec<String> = self.inner.block_ids().iter().map(|id| id.to_string()).collect();
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
