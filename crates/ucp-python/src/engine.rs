//! Engine bindings for Python.
//!
//! Exposes the UCM Engine with transaction support, validation, and batch operations.

use pyo3::prelude::*;
use ucm_engine::engine::{Engine, EngineConfig};
use ucm_engine::traversal::{
    NavigateDirection, TraversalConfig, TraversalEngine, TraversalFilter, TraversalOutput,
    TraversalResult,
};
use ucm_engine::validate::{ResourceLimits, ValidationPipeline, ValidationResult};

use crate::document::PyDocument;
use crate::types::PyBlockId;

/// Engine configuration.
#[pyclass(name = "EngineConfig")]
#[derive(Clone)]
pub struct PyEngineConfig {
    pub(crate) inner: EngineConfig,
}

#[pymethods]
impl PyEngineConfig {
    #[new]
    #[pyo3(signature = (validate_on_operation=true, max_batch_size=10000, enable_transactions=true, enable_snapshots=true))]
    fn new(
        validate_on_operation: bool,
        max_batch_size: usize,
        enable_transactions: bool,
        enable_snapshots: bool,
    ) -> Self {
        Self {
            inner: EngineConfig {
                validate_on_operation,
                max_batch_size,
                enable_transactions,
                enable_snapshots,
            },
        }
    }

    #[getter]
    fn validate_on_operation(&self) -> bool {
        self.inner.validate_on_operation
    }

    #[getter]
    fn max_batch_size(&self) -> usize {
        self.inner.max_batch_size
    }

    #[getter]
    fn enable_transactions(&self) -> bool {
        self.inner.enable_transactions
    }

    #[getter]
    fn enable_snapshots(&self) -> bool {
        self.inner.enable_snapshots
    }

    fn __repr__(&self) -> String {
        format!(
            "EngineConfig(validate_on_operation={}, max_batch_size={}, enable_transactions={}, enable_snapshots={})",
            self.inner.validate_on_operation,
            self.inner.max_batch_size,
            self.inner.enable_transactions,
            self.inner.enable_snapshots
        )
    }
}

/// Transaction ID wrapper.
#[pyclass(name = "TransactionId")]
#[derive(Clone)]
pub struct PyTransactionId {
    pub(crate) inner: String,
}

#[pymethods]
impl PyTransactionId {
    fn __repr__(&self) -> String {
        format!("TransactionId({})", self.inner)
    }

    fn __str__(&self) -> String {
        self.inner.clone()
    }
}

/// The main transformation engine with transaction support.
#[pyclass(name = "Engine")]
pub struct PyEngine {
    inner: Engine,
}

#[pymethods]
impl PyEngine {
    /// Create a new engine with default configuration.
    #[new]
    #[pyo3(signature = (config=None))]
    fn new(config: Option<PyEngineConfig>) -> Self {
        let engine = match config {
            Some(c) => Engine::with_config(c.inner),
            None => Engine::new(),
        };
        Self { inner: engine }
    }

    /// Validate a document.
    fn validate(&self, doc: &PyDocument) -> PyValidationResult {
        let result = self.inner.validate(doc.inner());
        PyValidationResult::from(result)
    }

    /// Begin a new transaction.
    fn begin_transaction(&mut self) -> PyTransactionId {
        let id = self.inner.begin_transaction();
        PyTransactionId { inner: id.0 }
    }

    /// Begin a named transaction.
    fn begin_named_transaction(&mut self, name: &str) -> PyTransactionId {
        let id = self.inner.begin_named_transaction(name);
        PyTransactionId { inner: id.0 }
    }

    /// Rollback a transaction.
    fn rollback_transaction(&mut self, txn_id: &PyTransactionId) -> PyResult<()> {
        let id = ucm_engine::transaction::TransactionId(txn_id.inner.clone());
        self.inner
            .rollback_transaction(&id)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    /// Create a snapshot.
    #[pyo3(signature = (name, doc, description=None))]
    fn create_snapshot(
        &mut self,
        name: &str,
        doc: &PyDocument,
        description: Option<&str>,
    ) -> PyResult<()> {
        self.inner
            .create_snapshot(name, doc.inner(), description.map(String::from))
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    /// Restore from a snapshot.
    fn restore_snapshot(&self, name: &str) -> PyResult<PyDocument> {
        let doc = self
            .inner
            .restore_snapshot(name)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(PyDocument::new(doc))
    }

    /// List all snapshots.
    fn list_snapshots(&self) -> Vec<String> {
        self.inner.list_snapshots()
    }

    /// Delete a snapshot.
    fn delete_snapshot(&mut self, name: &str) -> bool {
        self.inner.delete_snapshot(name)
    }

    fn __repr__(&self) -> String {
        "Engine()".to_string()
    }
}

/// Resource limits for validation.
#[pyclass(name = "ResourceLimits")]
#[derive(Clone)]
pub struct PyResourceLimits {
    pub(crate) inner: ResourceLimits,
}

#[pymethods]
impl PyResourceLimits {
    #[new]
    #[pyo3(signature = (
        max_document_size=None,
        max_block_count=None,
        max_block_size=None,
        max_nesting_depth=None,
        max_edges_per_block=None
    ))]
    fn new(
        max_document_size: Option<usize>,
        max_block_count: Option<usize>,
        max_block_size: Option<usize>,
        max_nesting_depth: Option<usize>,
        max_edges_per_block: Option<usize>,
    ) -> Self {
        let defaults = ResourceLimits::default();
        Self {
            inner: ResourceLimits {
                max_document_size: max_document_size.unwrap_or(defaults.max_document_size),
                max_block_count: max_block_count.unwrap_or(defaults.max_block_count),
                max_block_size: max_block_size.unwrap_or(defaults.max_block_size),
                max_nesting_depth: max_nesting_depth.unwrap_or(defaults.max_nesting_depth),
                max_edges_per_block: max_edges_per_block.unwrap_or(defaults.max_edges_per_block),
            },
        }
    }

    /// Create default resource limits.
    #[staticmethod]
    fn default_limits() -> Self {
        Self {
            inner: ResourceLimits::default(),
        }
    }

    #[getter]
    fn max_document_size(&self) -> usize {
        self.inner.max_document_size
    }

    #[getter]
    fn max_block_count(&self) -> usize {
        self.inner.max_block_count
    }

    #[getter]
    fn max_block_size(&self) -> usize {
        self.inner.max_block_size
    }

    #[getter]
    fn max_nesting_depth(&self) -> usize {
        self.inner.max_nesting_depth
    }

    #[getter]
    fn max_edges_per_block(&self) -> usize {
        self.inner.max_edges_per_block
    }

    fn __repr__(&self) -> String {
        format!(
            "ResourceLimits(max_block_count={}, max_nesting_depth={}, max_edges_per_block={})",
            self.inner.max_block_count,
            self.inner.max_nesting_depth,
            self.inner.max_edges_per_block
        )
    }
}

/// Validation result.
#[pyclass(name = "ValidationResult")]
#[derive(Clone)]
pub struct PyValidationResult {
    valid: bool,
    issues: Vec<PyValidationIssue>,
}

impl From<ValidationResult> for PyValidationResult {
    fn from(result: ValidationResult) -> Self {
        Self {
            valid: result.valid,
            issues: result
                .issues
                .into_iter()
                .map(PyValidationIssue::from)
                .collect(),
        }
    }
}

#[pymethods]
impl PyValidationResult {
    #[getter]
    fn valid(&self) -> bool {
        self.valid
    }

    #[getter]
    fn issues(&self) -> Vec<PyValidationIssue> {
        self.issues.clone()
    }

    /// Get only error issues.
    fn errors(&self) -> Vec<PyValidationIssue> {
        self.issues
            .iter()
            .filter(|i| i.severity == "error")
            .cloned()
            .collect()
    }

    /// Get only warning issues.
    fn warnings(&self) -> Vec<PyValidationIssue> {
        self.issues
            .iter()
            .filter(|i| i.severity == "warning")
            .cloned()
            .collect()
    }

    fn __repr__(&self) -> String {
        format!(
            "ValidationResult(valid={}, issues={})",
            self.valid,
            self.issues.len()
        )
    }

    fn __bool__(&self) -> bool {
        self.valid
    }
}

/// A single validation issue.
#[pyclass(name = "ValidationIssue")]
#[derive(Clone)]
pub struct PyValidationIssue {
    #[pyo3(get)]
    severity: String,
    #[pyo3(get)]
    code: String,
    #[pyo3(get)]
    message: String,
}

impl From<ucm_core::ValidationIssue> for PyValidationIssue {
    fn from(issue: ucm_core::ValidationIssue) -> Self {
        Self {
            severity: format!("{:?}", issue.severity).to_lowercase(),
            code: format!("{:?}", issue.code),
            message: issue.message,
        }
    }
}

#[pymethods]
impl PyValidationIssue {
    fn __repr__(&self) -> String {
        format!(
            "ValidationIssue(severity='{}', code='{}', message='{}')",
            self.severity, self.code, self.message
        )
    }
}

/// Validation pipeline with configurable resource limits.
#[pyclass(name = "ValidationPipeline")]
pub struct PyValidationPipeline {
    inner: ValidationPipeline,
}

#[pymethods]
impl PyValidationPipeline {
    /// Create a new validation pipeline with default limits.
    #[new]
    #[pyo3(signature = (limits=None))]
    fn new(limits: Option<PyResourceLimits>) -> Self {
        let pipeline = match limits {
            Some(l) => ValidationPipeline::with_limits(l.inner),
            None => ValidationPipeline::new(),
        };
        Self { inner: pipeline }
    }

    /// Validate a document.
    fn validate(&self, doc: &PyDocument) -> PyValidationResult {
        let result = self.inner.validate_document(doc.inner());
        PyValidationResult::from(result)
    }

    fn __repr__(&self) -> String {
        "ValidationPipeline()".to_string()
    }
}

/// Traversal direction.
#[pyclass(name = "TraversalDirection")]
#[derive(Clone)]
pub struct PyTraversalDirection;

#[pymethods]
impl PyTraversalDirection {
    #[classattr]
    const DOWN: &'static str = "down";
    #[classattr]
    const UP: &'static str = "up";
    #[classattr]
    const BOTH: &'static str = "both";
    #[classattr]
    const SIBLINGS: &'static str = "siblings";
    #[classattr]
    const BREADTH_FIRST: &'static str = "breadth_first";
    #[classattr]
    const DEPTH_FIRST: &'static str = "depth_first";
}

fn parse_direction(s: &str) -> NavigateDirection {
    match s.to_lowercase().as_str() {
        "down" => NavigateDirection::Down,
        "up" => NavigateDirection::Up,
        "both" => NavigateDirection::Both,
        "siblings" => NavigateDirection::Siblings,
        "breadth_first" | "bfs" => NavigateDirection::BreadthFirst,
        "depth_first" | "dfs" => NavigateDirection::DepthFirst,
        _ => NavigateDirection::Down,
    }
}

/// Traversal filter for filtering blocks during traversal.
#[pyclass(name = "TraversalFilter")]
#[derive(Clone, Default)]
pub struct PyTraversalFilter {
    include_roles: Vec<String>,
    exclude_roles: Vec<String>,
    include_tags: Vec<String>,
    exclude_tags: Vec<String>,
    content_pattern: Option<String>,
}

#[pymethods]
impl PyTraversalFilter {
    #[new]
    #[pyo3(signature = (
        include_roles=None,
        exclude_roles=None,
        include_tags=None,
        exclude_tags=None,
        content_pattern=None
    ))]
    fn new(
        include_roles: Option<Vec<String>>,
        exclude_roles: Option<Vec<String>>,
        include_tags: Option<Vec<String>>,
        exclude_tags: Option<Vec<String>>,
        content_pattern: Option<String>,
    ) -> Self {
        Self {
            include_roles: include_roles.unwrap_or_default(),
            exclude_roles: exclude_roles.unwrap_or_default(),
            include_tags: include_tags.unwrap_or_default(),
            exclude_tags: exclude_tags.unwrap_or_default(),
            content_pattern,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "TraversalFilter(include_roles={:?}, exclude_roles={:?}, include_tags={:?}, exclude_tags={:?})",
            self.include_roles, self.exclude_roles, self.include_tags, self.exclude_tags
        )
    }
}

impl From<&PyTraversalFilter> for TraversalFilter {
    fn from(f: &PyTraversalFilter) -> Self {
        TraversalFilter {
            include_roles: f.include_roles.clone(),
            exclude_roles: f.exclude_roles.clone(),
            include_tags: f.include_tags.clone(),
            exclude_tags: f.exclude_tags.clone(),
            content_pattern: f.content_pattern.clone(),
            edge_types: Vec::new(),
        }
    }
}

/// Traversal configuration.
#[pyclass(name = "TraversalConfig")]
#[derive(Clone)]
pub struct PyTraversalConfig {
    pub(crate) inner: TraversalConfig,
}

#[pymethods]
impl PyTraversalConfig {
    #[new]
    #[pyo3(signature = (max_depth=100, max_nodes=10000, include_orphans=false))]
    fn new(max_depth: usize, max_nodes: usize, include_orphans: bool) -> Self {
        Self {
            inner: TraversalConfig {
                max_depth,
                max_nodes,
                default_preview_length: 100,
                include_orphans,
                cache_enabled: true,
            },
        }
    }

    #[getter]
    fn max_depth(&self) -> usize {
        self.inner.max_depth
    }

    #[getter]
    fn max_nodes(&self) -> usize {
        self.inner.max_nodes
    }

    fn __repr__(&self) -> String {
        format!(
            "TraversalConfig(max_depth={}, max_nodes={})",
            self.inner.max_depth, self.inner.max_nodes
        )
    }
}

/// A node in the traversal result.
#[pyclass(name = "TraversalNode")]
#[derive(Clone)]
pub struct PyTraversalNode {
    #[pyo3(get)]
    id: String,
    #[pyo3(get)]
    depth: usize,
    #[pyo3(get)]
    parent_id: Option<String>,
    #[pyo3(get)]
    content_preview: Option<String>,
    #[pyo3(get)]
    semantic_role: Option<String>,
    #[pyo3(get)]
    child_count: usize,
    #[pyo3(get)]
    edge_count: usize,
}

#[pymethods]
impl PyTraversalNode {
    fn __repr__(&self) -> String {
        format!(
            "TraversalNode(id='{}', depth={}, role={:?})",
            self.id, self.depth, self.semantic_role
        )
    }
}

/// Traversal result containing nodes, edges, and summary.
#[pyclass(name = "TraversalResult")]
#[derive(Clone)]
pub struct PyTraversalResult {
    nodes: Vec<PyTraversalNode>,
    total_nodes: usize,
    max_depth: usize,
    execution_time_ms: Option<u64>,
}

impl From<TraversalResult> for PyTraversalResult {
    fn from(result: TraversalResult) -> Self {
        Self {
            nodes: result
                .nodes
                .into_iter()
                .map(|n| PyTraversalNode {
                    id: n.id.to_string(),
                    depth: n.depth,
                    parent_id: n.parent_id.map(|id| id.to_string()),
                    content_preview: n.content_preview,
                    semantic_role: n.semantic_role,
                    child_count: n.child_count,
                    edge_count: n.edge_count,
                })
                .collect(),
            total_nodes: result.summary.total_nodes,
            max_depth: result.summary.max_depth,
            execution_time_ms: result.metadata.execution_time_ms,
        }
    }
}

#[pymethods]
impl PyTraversalResult {
    #[getter]
    fn nodes(&self) -> Vec<PyTraversalNode> {
        self.nodes.clone()
    }

    #[getter]
    fn total_nodes(&self) -> usize {
        self.total_nodes
    }

    #[getter]
    fn max_depth(&self) -> usize {
        self.max_depth
    }

    #[getter]
    fn execution_time_ms(&self) -> Option<u64> {
        self.execution_time_ms
    }

    /// Get node IDs only.
    fn node_ids(&self) -> Vec<String> {
        self.nodes.iter().map(|n| n.id.clone()).collect()
    }

    fn __len__(&self) -> usize {
        self.nodes.len()
    }

    fn __repr__(&self) -> String {
        format!(
            "TraversalResult(nodes={}, max_depth={})",
            self.total_nodes, self.max_depth
        )
    }
}

/// Graph traversal engine for UCM documents.
#[pyclass(name = "TraversalEngine")]
pub struct PyTraversalEngine {
    inner: TraversalEngine,
}

#[pymethods]
impl PyTraversalEngine {
    /// Create a new traversal engine.
    #[new]
    #[pyo3(signature = (config=None))]
    fn new(config: Option<PyTraversalConfig>) -> Self {
        let engine = match config {
            Some(c) => TraversalEngine::with_config(c.inner),
            None => TraversalEngine::new(),
        };
        Self { inner: engine }
    }

    /// Navigate from a starting point in a specific direction.
    #[pyo3(signature = (doc, direction, start_id=None, depth=None, filter=None))]
    fn navigate(
        &self,
        doc: &PyDocument,
        direction: &str,
        start_id: Option<&PyBlockId>,
        depth: Option<usize>,
        filter: Option<&PyTraversalFilter>,
    ) -> PyResult<PyTraversalResult> {
        let dir = parse_direction(direction);
        let start = start_id.map(|id| *id.inner());
        let filt = filter.map(TraversalFilter::from);

        let result = self
            .inner
            .navigate(
                doc.inner(),
                start,
                dir,
                depth,
                filt,
                TraversalOutput::StructureWithPreviews,
            )
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        Ok(PyTraversalResult::from(result))
    }

    /// Expand a node to get its immediate children.
    fn expand(&self, doc: &PyDocument, node_id: &PyBlockId) -> PyResult<PyTraversalResult> {
        let result = self
            .inner
            .expand(
                doc.inner(),
                node_id.inner(),
                TraversalOutput::StructureWithPreviews,
            )
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        Ok(PyTraversalResult::from(result))
    }

    /// Get the path from a node to the root.
    fn path_to_root(&self, doc: &PyDocument, node_id: &PyBlockId) -> PyResult<Vec<PyBlockId>> {
        let path = self
            .inner
            .path_to_root(doc.inner(), node_id.inner())
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        Ok(path.into_iter().map(PyBlockId::from).collect())
    }

    /// Find all paths between two nodes.
    #[pyo3(signature = (doc, from_id, to_id, max_paths=10))]
    fn find_paths(
        &self,
        doc: &PyDocument,
        from_id: &PyBlockId,
        to_id: &PyBlockId,
        max_paths: usize,
    ) -> PyResult<Vec<Vec<PyBlockId>>> {
        let paths = self
            .inner
            .find_paths(doc.inner(), from_id.inner(), to_id.inner(), max_paths)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        Ok(paths
            .into_iter()
            .map(|p| p.into_iter().map(PyBlockId::from).collect())
            .collect())
    }

    fn __repr__(&self) -> String {
        "TraversalEngine()".to_string()
    }
}
