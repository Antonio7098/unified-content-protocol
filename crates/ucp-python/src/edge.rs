//! Edge type wrappers for Python.

use pyo3::prelude::*;
use std::str::FromStr;
use ucm_core::{Edge, EdgeType};

use crate::types::PyBlockId;

/// Edge type enumeration.
#[pyclass(name = "EdgeType", eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PyEdgeType {
    DerivedFrom = 0,
    Supersedes = 1,
    TransformedFrom = 2,
    References = 3,
    CitedBy = 4,
    LinksTo = 5,
    Supports = 6,
    Contradicts = 7,
    Elaborates = 8,
    Summarizes = 9,
    ParentOf = 10,
    ChildOf = 11,
    SiblingOf = 12,
    PreviousSibling = 13,
    NextSibling = 14,
    VersionOf = 15,
    AlternativeOf = 16,
    TranslationOf = 17,
}

impl From<&EdgeType> for PyEdgeType {
    fn from(et: &EdgeType) -> Self {
        match et {
            EdgeType::DerivedFrom => PyEdgeType::DerivedFrom,
            EdgeType::Supersedes => PyEdgeType::Supersedes,
            EdgeType::TransformedFrom => PyEdgeType::TransformedFrom,
            EdgeType::References => PyEdgeType::References,
            EdgeType::CitedBy => PyEdgeType::CitedBy,
            EdgeType::LinksTo => PyEdgeType::LinksTo,
            EdgeType::Supports => PyEdgeType::Supports,
            EdgeType::Contradicts => PyEdgeType::Contradicts,
            EdgeType::Elaborates => PyEdgeType::Elaborates,
            EdgeType::Summarizes => PyEdgeType::Summarizes,
            EdgeType::ParentOf => PyEdgeType::ParentOf,
            EdgeType::ChildOf => PyEdgeType::ChildOf,
            EdgeType::SiblingOf => PyEdgeType::SiblingOf,
            EdgeType::PreviousSibling => PyEdgeType::PreviousSibling,
            EdgeType::NextSibling => PyEdgeType::NextSibling,
            EdgeType::VersionOf => PyEdgeType::VersionOf,
            EdgeType::AlternativeOf => PyEdgeType::AlternativeOf,
            EdgeType::TranslationOf => PyEdgeType::TranslationOf,
            EdgeType::Custom(_) => PyEdgeType::References, // Default fallback
        }
    }
}

impl From<PyEdgeType> for EdgeType {
    fn from(et: PyEdgeType) -> Self {
        match et {
            PyEdgeType::DerivedFrom => EdgeType::DerivedFrom,
            PyEdgeType::Supersedes => EdgeType::Supersedes,
            PyEdgeType::TransformedFrom => EdgeType::TransformedFrom,
            PyEdgeType::References => EdgeType::References,
            PyEdgeType::CitedBy => EdgeType::CitedBy,
            PyEdgeType::LinksTo => EdgeType::LinksTo,
            PyEdgeType::Supports => EdgeType::Supports,
            PyEdgeType::Contradicts => EdgeType::Contradicts,
            PyEdgeType::Elaborates => EdgeType::Elaborates,
            PyEdgeType::Summarizes => EdgeType::Summarizes,
            PyEdgeType::ParentOf => EdgeType::ParentOf,
            PyEdgeType::ChildOf => EdgeType::ChildOf,
            PyEdgeType::SiblingOf => EdgeType::SiblingOf,
            PyEdgeType::PreviousSibling => EdgeType::PreviousSibling,
            PyEdgeType::NextSibling => EdgeType::NextSibling,
            PyEdgeType::VersionOf => EdgeType::VersionOf,
            PyEdgeType::AlternativeOf => EdgeType::AlternativeOf,
            PyEdgeType::TranslationOf => EdgeType::TranslationOf,
        }
    }
}

#[pymethods]
impl PyEdgeType {
    /// Parse an edge type from string.
    #[staticmethod]
    fn from_string(s: &str) -> PyResult<Self> {
        let et = EdgeType::from_str(s)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(PyEdgeType::from(&et))
    }

    /// Convert to string representation.
    fn as_string(&self) -> String {
        let et: EdgeType = (*self).into();
        et.as_str()
    }

    /// Check if this edge type is symmetric.
    fn is_symmetric(&self) -> bool {
        let et: EdgeType = (*self).into();
        et.is_symmetric()
    }

    /// Check if this is a structural edge type (auto-maintained).
    fn is_structural(&self) -> bool {
        let et: EdgeType = (*self).into();
        et.is_structural()
    }

    fn __str__(&self) -> String {
        self.as_string()
    }

    fn __repr__(&self) -> String {
        format!("EdgeType.{}", self.as_string().to_uppercase())
    }
}

/// An edge representing a relationship between blocks.
#[pyclass(name = "Edge")]
#[derive(Clone)]
pub struct PyEdge(pub(crate) Edge);

impl From<Edge> for PyEdge {
    fn from(edge: Edge) -> Self {
        Self(edge)
    }
}

impl From<&Edge> for PyEdge {
    fn from(edge: &Edge) -> Self {
        Self(edge.clone())
    }
}

#[pymethods]
impl PyEdge {
    /// Create a new edge.
    #[new]
    fn new(edge_type: PyEdgeType, target: &PyBlockId) -> Self {
        let et: EdgeType = edge_type.into();
        PyEdge(Edge::new(et, *target.inner()))
    }

    /// Get the edge type.
    #[getter]
    fn edge_type(&self) -> PyEdgeType {
        PyEdgeType::from(&self.0.edge_type)
    }

    /// Get the target block ID.
    #[getter]
    fn target(&self) -> PyBlockId {
        PyBlockId::from(self.0.target)
    }

    /// Get the confidence score (0.0-1.0) if set.
    #[getter]
    fn confidence(&self) -> Option<f32> {
        self.0.metadata.confidence
    }

    /// Get the description if set.
    #[getter]
    fn description(&self) -> Option<String> {
        self.0.metadata.description.clone()
    }

    /// Get the creation timestamp as ISO 8601 string.
    #[getter]
    fn created_at(&self) -> String {
        self.0.created_at.to_rfc3339()
    }

    fn __repr__(&self) -> String {
        format!("Edge({} -> {})", self.0.edge_type.as_str(), self.0.target)
    }
}
