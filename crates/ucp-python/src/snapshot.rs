//! Snapshot management wrapper for Python.

use pyo3::prelude::*;
use ucm_engine::{Snapshot, SnapshotManager};

use crate::document::PyDocument;

/// Snapshot information.
#[pyclass(name = "SnapshotInfo")]
#[derive(Clone)]
pub struct PySnapshotInfo {
    /// Snapshot name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Creation timestamp as ISO 8601 string
    pub created_at: String,
    /// Document version at snapshot time
    pub version: u64,
}

#[pymethods]
impl PySnapshotInfo {
    #[getter]
    fn name(&self) -> &str {
        &self.name
    }

    #[getter]
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    #[getter]
    fn created_at(&self) -> &str {
        &self.created_at
    }

    #[getter]
    fn version(&self) -> u64 {
        self.version
    }

    fn __repr__(&self) -> String {
        format!(
            "SnapshotInfo(name={:?}, created_at={:?})",
            self.name, self.created_at
        )
    }
}

impl From<&Snapshot> for PySnapshotInfo {
    fn from(s: &Snapshot) -> Self {
        Self {
            name: s.id.0.clone(),
            description: s.description.clone(),
            created_at: s.created_at.to_rfc3339(),
            version: s.document_version.counter,
        }
    }
}

/// Manages document snapshots for versioning.
#[pyclass(name = "SnapshotManager")]
pub struct PySnapshotManager {
    inner: SnapshotManager,
}

#[pymethods]
impl PySnapshotManager {
    /// Create a new snapshot manager.
    #[new]
    #[pyo3(signature = (max_snapshots=None))]
    fn new(max_snapshots: Option<usize>) -> Self {
        let inner = if let Some(max) = max_snapshots {
            SnapshotManager::with_max_snapshots(max)
        } else {
            SnapshotManager::new()
        };
        Self { inner }
    }

    /// Create a snapshot of a document.
    #[pyo3(signature = (name, doc, description=None))]
    fn create(
        &mut self,
        name: &str,
        doc: &PyDocument,
        description: Option<&str>,
    ) -> PyResult<String> {
        self.inner
            .create(name, doc.inner(), description.map(|s| s.to_string()))
            .map(|id| id.0)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    /// Restore a document from a snapshot.
    fn restore(&self, name: &str) -> PyResult<PyDocument> {
        self.inner
            .restore(name)
            .map(PyDocument::new)
            .map_err(|e| pyo3::exceptions::PyKeyError::new_err(e.to_string()))
    }

    /// Get information about a snapshot.
    fn get(&self, name: &str) -> Option<PySnapshotInfo> {
        self.inner.get(name).map(PySnapshotInfo::from)
    }

    /// List all snapshots (most recent first).
    fn list(&self) -> Vec<PySnapshotInfo> {
        self.inner
            .list()
            .iter()
            .map(|s| PySnapshotInfo::from(*s))
            .collect()
    }

    /// Delete a snapshot.
    fn delete(&mut self, name: &str) -> bool {
        self.inner.delete(name)
    }

    /// Check if a snapshot exists.
    fn exists(&self, name: &str) -> bool {
        self.inner.exists(name)
    }

    /// Get snapshot count.
    fn __len__(&self) -> usize {
        self.inner.count()
    }

    fn __repr__(&self) -> String {
        format!("SnapshotManager(snapshots={})", self.inner.count())
    }
}
