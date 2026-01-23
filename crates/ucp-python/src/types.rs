//! Core type wrappers for Python.

use pyo3::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use ucm_core::BlockId;

/// A content-addressed block identifier.
#[pyclass(name = "BlockId", frozen)]
#[derive(Clone)]
pub struct PyBlockId(pub(crate) BlockId);

impl PyBlockId {
    pub fn inner(&self) -> &BlockId {
        &self.0
    }
}

impl From<BlockId> for PyBlockId {
    fn from(id: BlockId) -> Self {
        Self(id)
    }
}

#[pymethods]
impl PyBlockId {
    /// Create a BlockId from a hex string (e.g., "blk_0102030405060708090a0b0c").
    #[new]
    fn new(s: &str) -> PyResult<Self> {
        s.parse::<BlockId>()
            .map(PyBlockId)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create the root block ID.
    #[staticmethod]
    fn root() -> Self {
        PyBlockId(BlockId::root())
    }

    /// Check if this is the root block ID.
    fn is_root(&self) -> bool {
        self.0.is_root()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    /// Get the string representation (for internal use).
    pub fn to_string_repr(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("BlockId('{}')", self.0)
    }

    fn __hash__(&self) -> isize {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish() as isize
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
