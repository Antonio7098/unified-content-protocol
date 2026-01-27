//! Common types for Python bindings.

use pyo3::prelude::*;
use ucm_core::types::{BlockId, DocumentId, EdgeId, Metadata, Timestamp};

/// Python wrapper for BlockId.
#[pyclass(name = "BlockId")]
#[derive(Clone, Debug)]
pub struct PyBlockId(pub BlockId);

impl From<BlockId> for PyBlockId {
    fn from(id: BlockId) -> Self {
        Self(id)
    }
}

impl From<PyBlockId> for BlockId {
    fn from(id: PyBlockId) -> Self {
        id.0
    }
}

#[pymethods]
impl PyBlockId {
    #[new]
    fn new(id: &str) -> Self {
        Self(BlockId::new(id))
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("BlockId({})", self.0)
    }
}

/// Python wrapper for DocumentId.
#[pyclass(name = "DocumentId")]
#[derive(Clone, Debug)]
pub struct PyDocumentId(pub DocumentId);

impl From<DocumentId> for PyDocumentId {
    fn from(id: DocumentId) -> Self {
        Self(id)
    }
}

impl From<PyDocumentId> for DocumentId {
    fn from(id: PyDocumentId) -> Self {
        id.0
    }
}

#[pymethods]
impl PyDocumentId {
    #[new]
    fn new(id: &str) -> Self {
        Self(DocumentId::new(id))
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("DocumentId({})", self.0)
    }
}

/// Python wrapper for EdgeId.
#[pyclass(name = "EdgeId")]
#[derive(Clone, Debug)]
pub struct PyEdgeId(pub EdgeId);

impl From<EdgeId> for PyEdgeId {
    fn from(id: EdgeId) -> Self {
        Self(id)
    }
}

impl From<PyEdgeId> for EdgeId {
    fn from(id: PyEdgeId) -> Self {
        id.0
    }
}

#[pymethods]
impl PyEdgeId {
    #[new]
    fn new(id: &str) -> Self {
        Self(EdgeId::new(id))
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("EdgeId({})", self.0)
    }
}

/// Python wrapper for Metadata.
#[pyclass(name = "Metadata")]
#[derive(Clone, Debug)]
pub struct PyMetadata(pub Metadata);

impl From<Metadata> for PyMetadata {
    fn from(meta: Metadata) -> Self {
        Self(meta)
    }
}

impl From<PyMetadata> for Metadata {
    fn from(meta: PyMetadata) -> Self {
        meta.0
    }
}

#[pymethods]
impl PyMetadata {
    #[new]
    fn new() -> Self {
        Self(Metadata::new())
    }

    fn get(&self, key: &str) -> Option<String> {
        self.0.get(key).cloned()
    }

    fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }

    fn remove(&mut self, key: &str) -> Option<String> {
        self.0.remove(key)
    }

    fn keys(&self) -> Vec<String> {
        self.0.keys().cloned().collect()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("Metadata({:?})", self.0)
    }
}

/// Python wrapper for Timestamp.
#[pyclass(name = "Timestamp")]
#[derive(Clone, Debug)]
pub struct PyTimestamp(pub Timestamp);

impl From<Timestamp> for PyTimestamp {
    fn from(ts: Timestamp) -> Self {
        Self(ts)
    }
}

impl From<PyTimestamp> for Timestamp {
    fn from(ts: PyTimestamp) -> Self {
        ts.0
    }
}

#[pymethods]
impl PyTimestamp {
    #[new]
    fn new() -> Self {
        Self(Timestamp::now())
    }

    fn as_unix(&self) -> u64 {
        self.0.as_unix()
    }

    fn as_iso8601(&self) -> String {
        self.0.to_iso8601()
    }

    fn __str__(&self) -> String {
        self.0.to_iso8601()
    }

    fn __repr__(&self) -> String {
        format!("Timestamp({})", self.0.to_iso8601())
    }
}
