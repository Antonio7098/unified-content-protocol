//! Python bindings for the agent graph traversal system.

use pyo3::prelude::*;
use ucp_agent::{AgentCapabilities, AgentSessionId, AgentTraversal, SessionConfig, ViewMode};

use crate::document::PyDocument;
use crate::types::PyBlockId;

/// Agent session ID wrapper.
#[pyclass(name = "AgentSessionId")]
#[derive(Clone)]
pub struct PyAgentSessionId {
    pub(crate) inner: AgentSessionId,
}

#[pymethods]
impl PyAgentSessionId {
    fn __repr__(&self) -> String {
        format!("AgentSessionId({})", self.inner)
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }
}

impl From<AgentSessionId> for PyAgentSessionId {
    fn from(id: AgentSessionId) -> Self {
        Self { inner: id }
    }
}

/// View mode for block content display.
#[pyclass(name = "ViewMode")]
#[derive(Clone)]
pub struct PyViewMode {
    inner: ViewMode,
}

#[pymethods]
impl PyViewMode {
    /// Create a Full view mode (shows complete content).
    #[staticmethod]
    fn full() -> Self {
        Self {
            inner: ViewMode::Full,
        }
    }

    /// Create a Preview view mode (shows first N characters).
    #[staticmethod]
    #[pyo3(signature = (length=100))]
    fn preview(length: usize) -> Self {
        Self {
            inner: ViewMode::preview(length),
        }
    }

    /// Create an IdsOnly view mode (just block IDs).
    #[staticmethod]
    fn ids_only() -> Self {
        Self {
            inner: ViewMode::IdsOnly,
        }
    }

    /// Create a Metadata view mode (role, tags, edge counts).
    #[staticmethod]
    fn metadata() -> Self {
        Self {
            inner: ViewMode::Metadata,
        }
    }

    /// Create an Adaptive view mode (auto-select based on relevance).
    #[staticmethod]
    #[pyo3(signature = (threshold=0.5))]
    fn adaptive(threshold: f32) -> Self {
        Self {
            inner: ViewMode::adaptive(threshold),
        }
    }

    fn __repr__(&self) -> String {
        match &self.inner {
            ViewMode::Full => "ViewMode.full()".to_string(),
            ViewMode::Preview { length } => format!("ViewMode.preview({})", length),
            ViewMode::IdsOnly => "ViewMode.ids_only()".to_string(),
            ViewMode::Metadata => "ViewMode.metadata()".to_string(),
            ViewMode::Adaptive { interest_threshold } => {
                format!("ViewMode.adaptive({})", interest_threshold)
            }
        }
    }
}

/// Agent capabilities configuration.
#[pyclass(name = "AgentCapabilities")]
#[derive(Clone)]
pub struct PyAgentCapabilities {
    inner: AgentCapabilities,
}

#[pymethods]
impl PyAgentCapabilities {
    /// Create capabilities with all permissions (default).
    #[new]
    fn new() -> Self {
        Self {
            inner: AgentCapabilities::default(),
        }
    }

    /// Create read-only capabilities (traverse only, no context modification).
    #[staticmethod]
    fn read_only() -> Self {
        Self {
            inner: AgentCapabilities::read_only(),
        }
    }

    #[getter]
    fn can_traverse(&self) -> bool {
        self.inner.can_traverse
    }

    #[getter]
    fn can_search(&self) -> bool {
        self.inner.can_search
    }

    #[getter]
    fn can_modify_context(&self) -> bool {
        self.inner.can_modify_context
    }

    #[getter]
    fn can_coordinate(&self) -> bool {
        self.inner.can_coordinate
    }

    fn __repr__(&self) -> String {
        format!(
            "AgentCapabilities(traverse={}, search={}, modify_context={}, coordinate={})",
            self.inner.can_traverse,
            self.inner.can_search,
            self.inner.can_modify_context,
            self.inner.can_coordinate
        )
    }
}

/// Session configuration for creating new agent sessions.
#[pyclass(name = "SessionConfig")]
#[derive(Clone)]
pub struct PySessionConfig {
    inner: SessionConfig,
}

#[pymethods]
impl PySessionConfig {
    #[new]
    #[pyo3(signature = (name=None, start_block=None))]
    fn new(name: Option<String>, start_block: Option<PyBlockId>) -> Self {
        let mut config = SessionConfig::new();
        if let Some(n) = name {
            config = config.with_name(&n);
        }
        if let Some(b) = start_block {
            config = config.with_start_block(*b.inner());
        }
        Self { inner: config }
    }

    /// Set session name.
    fn with_name(&self, name: &str) -> Self {
        Self {
            inner: self.inner.clone().with_name(name),
        }
    }

    /// Set initial view mode.
    fn with_view_mode(&self, mode: &PyViewMode) -> Self {
        Self {
            inner: self.inner.clone().with_view_mode(mode.inner.clone()),
        }
    }

    /// Set agent capabilities.
    fn with_capabilities(&self, caps: &PyAgentCapabilities) -> Self {
        Self {
            inner: self.inner.clone().with_capabilities(caps.inner.clone()),
        }
    }

    fn __repr__(&self) -> String {
        format!("SessionConfig(name={:?})", self.inner.name)
    }
}

/// Navigation result from a GOTO or BACK operation.
#[pyclass(name = "NavigationResult")]
pub struct PyNavigationResult {
    position: PyBlockId,
    refreshed: bool,
}

#[pymethods]
impl PyNavigationResult {
    #[getter]
    fn position(&self) -> PyBlockId {
        self.position.clone()
    }

    #[getter]
    fn refreshed(&self) -> bool {
        self.refreshed
    }

    fn __repr__(&self) -> String {
        format!(
            "NavigationResult(position={}, refreshed={})",
            self.position.inner(),
            self.refreshed
        )
    }
}

/// Expansion result from an EXPAND operation.
#[pyclass(name = "ExpansionResult")]
pub struct PyExpansionResult {
    root: PyBlockId,
    levels: Vec<Vec<PyBlockId>>,
    total_blocks: usize,
}

#[pymethods]
impl PyExpansionResult {
    #[getter]
    fn root(&self) -> PyBlockId {
        self.root.clone()
    }

    #[getter]
    fn levels(&self) -> Vec<Vec<PyBlockId>> {
        self.levels.clone()
    }

    #[getter]
    fn total_blocks(&self) -> usize {
        self.total_blocks
    }

    fn __repr__(&self) -> String {
        format!(
            "ExpansionResult(root={}, total_blocks={})",
            self.root.inner(),
            self.total_blocks
        )
    }
}

/// Block view result from a VIEW operation.
#[pyclass(name = "BlockView")]
pub struct PyBlockView {
    block_id: PyBlockId,
    content: Option<String>,
    role: Option<String>,
    tags: Vec<String>,
    children_count: usize,
    incoming_edges: usize,
    outgoing_edges: usize,
}

#[pymethods]
impl PyBlockView {
    #[getter]
    fn block_id(&self) -> PyBlockId {
        self.block_id.clone()
    }

    #[getter]
    fn content(&self) -> Option<String> {
        self.content.clone()
    }

    #[getter]
    fn role(&self) -> Option<String> {
        self.role.clone()
    }

    #[getter]
    fn tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    #[getter]
    fn children_count(&self) -> usize {
        self.children_count
    }

    #[getter]
    fn incoming_edges(&self) -> usize {
        self.incoming_edges
    }

    #[getter]
    fn outgoing_edges(&self) -> usize {
        self.outgoing_edges
    }

    fn __repr__(&self) -> String {
        format!(
            "BlockView(block_id={}, role={:?}, children={})",
            self.block_id.inner(),
            self.role,
            self.children_count
        )
    }
}

/// Search result from a SEARCH operation.
#[pyclass(name = "SearchResult")]
pub struct PySearchResult {
    query: String,
    matches: Vec<(PyBlockId, f32, Option<String>)>,
    total_searched: usize,
}

#[pymethods]
impl PySearchResult {
    #[getter]
    fn query(&self) -> String {
        self.query.clone()
    }

    #[getter]
    fn matches(&self) -> Vec<(PyBlockId, f32, Option<String>)> {
        self.matches.clone()
    }

    #[getter]
    fn total_searched(&self) -> usize {
        self.total_searched
    }

    fn __repr__(&self) -> String {
        format!(
            "SearchResult(query='{}', matches={}, total_searched={})",
            self.query,
            self.matches.len(),
            self.total_searched
        )
    }
}

/// Find result from a FIND operation.
#[pyclass(name = "FindResult")]
pub struct PyFindResult {
    matches: Vec<PyBlockId>,
    total_searched: usize,
}

#[pymethods]
impl PyFindResult {
    #[getter]
    fn matches(&self) -> Vec<PyBlockId> {
        self.matches.clone()
    }

    #[getter]
    fn total_searched(&self) -> usize {
        self.total_searched
    }

    fn __repr__(&self) -> String {
        format!(
            "FindResult(matches={}, total_searched={})",
            self.matches.len(),
            self.total_searched
        )
    }
}

/// Connection information (block + edge type).
#[pyclass(name = "Connection")]
pub struct PyConnection {
    block: PyBlockView,
    edge_type: String,
}

#[pymethods]
impl PyConnection {
    #[getter]
    fn block(&self) -> PyBlockView {
        PyBlockView {
            block_id: self.block.block_id.clone(),
            content: self.block.content.clone(),
            role: self.block.role.clone(),
            tags: self.block.tags.clone(),
            children_count: self.block.children_count,
            incoming_edges: self.block.incoming_edges,
            outgoing_edges: self.block.outgoing_edges,
        }
    }

    #[getter]
    fn edge_type(&self) -> String {
        self.edge_type.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "Connection(block_id={}, edge_type='{}')",
            self.block.block_id.inner(),
            self.edge_type
        )
    }
}

/// Neighborhood view result from a VIEW NEIGHBORHOOD operation.
#[pyclass(name = "NeighborhoodView")]
pub struct PyNeighborhoodView {
    position: PyBlockId,
    ancestors: Vec<PyBlockView>,
    children: Vec<PyBlockView>,
    siblings: Vec<PyBlockView>,
    connections: Vec<PyConnection>,
}

#[pymethods]
impl PyNeighborhoodView {
    #[getter]
    fn position(&self) -> PyBlockId {
        self.position.clone()
    }

    #[getter]
    fn ancestors(&self) -> Vec<PyBlockView> {
        self.ancestors
            .iter()
            .map(|b| PyBlockView {
                block_id: b.block_id.clone(),
                content: b.content.clone(),
                role: b.role.clone(),
                tags: b.tags.clone(),
                children_count: b.children_count,
                incoming_edges: b.incoming_edges,
                outgoing_edges: b.outgoing_edges,
            })
            .collect()
    }

    #[getter]
    fn children(&self) -> Vec<PyBlockView> {
        self.children
            .iter()
            .map(|b| PyBlockView {
                block_id: b.block_id.clone(),
                content: b.content.clone(),
                role: b.role.clone(),
                tags: b.tags.clone(),
                children_count: b.children_count,
                incoming_edges: b.incoming_edges,
                outgoing_edges: b.outgoing_edges,
            })
            .collect()
    }

    #[getter]
    fn siblings(&self) -> Vec<PyBlockView> {
        self.siblings
            .iter()
            .map(|b| PyBlockView {
                block_id: b.block_id.clone(),
                content: b.content.clone(),
                role: b.role.clone(),
                tags: b.tags.clone(),
                children_count: b.children_count,
                incoming_edges: b.incoming_edges,
                outgoing_edges: b.outgoing_edges,
            })
            .collect()
    }

    #[getter]
    fn connections(&self) -> Vec<PyConnection> {
        self.connections
            .iter()
            .map(|c| PyConnection {
                block: PyBlockView {
                    block_id: c.block.block_id.clone(),
                    content: c.block.content.clone(),
                    role: c.block.role.clone(),
                    tags: c.block.tags.clone(),
                    children_count: c.block.children_count,
                    incoming_edges: c.block.incoming_edges,
                    outgoing_edges: c.block.outgoing_edges,
                },
                edge_type: c.edge_type.clone(),
            })
            .collect()
    }

    fn __repr__(&self) -> String {
        format!(
            "NeighborhoodView(position={}, ancestors={}, children={}, siblings={}, connections={})",
            self.position.inner(),
            self.ancestors.len(),
            self.children.len(),
            self.siblings.len(),
            self.connections.len()
        )
    }
}

/// Main agent traversal interface.
#[pyclass(name = "AgentTraversal")]
pub struct PyAgentTraversal {
    inner: AgentTraversal,
    runtime: tokio::runtime::Runtime,
}

#[pymethods]
impl PyAgentTraversal {
    /// Create a new agent traversal system from a document.
    #[new]
    fn new(doc: &PyDocument) -> PyResult<Self> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        let traversal = AgentTraversal::new(doc.inner().clone());

        Ok(Self {
            inner: traversal,
            runtime,
        })
    }

    /// Create a new agent session.
    #[pyo3(signature = (config=None))]
    fn create_session(&self, config: Option<PySessionConfig>) -> PyResult<PyAgentSessionId> {
        let cfg = config.map(|c| c.inner).unwrap_or_default();
        self.inner
            .create_session(cfg)
            .map(PyAgentSessionId::from)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    /// Close a session.
    fn close_session(&self, session_id: &PyAgentSessionId) -> PyResult<()> {
        self.inner
            .close_session(&session_id.inner)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    /// Navigate to a specific block (GOTO command).
    ///
    /// Args:
    ///     session_id: The agent session
    ///     block_id: The block to navigate to
    ///
    /// Returns:
    ///     NavigationResult with the new position
    ///
    /// Note:
    ///     If you get "Block not found" errors after adding blocks to the document,
    ///     call update_document() to sync the changes to the traversal.
    fn navigate_to(
        &self,
        session_id: &PyAgentSessionId,
        block_id: &PyBlockId,
    ) -> PyResult<PyNavigationResult> {
        let result = self
            .inner
            .navigate_to(&session_id.inner, *block_id.inner())
            .map_err(|e| {
                let msg = e.to_string();
                if msg.contains("Block not found") {
                    pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "Block not found: {}. If you added blocks to the document after creating \
                        the AgentTraversal, call update_document(doc) to sync changes.",
                        block_id.inner()
                    ))
                } else {
                    pyo3::exceptions::PyRuntimeError::new_err(msg)
                }
            })?;

        Ok(PyNavigationResult {
            position: PyBlockId::from(result.position),
            refreshed: result.refreshed,
        })
    }

    /// Update the internal document with a new copy.
    ///
    /// Use this when you've added blocks to the original document after
    /// creating the AgentTraversal. The traversal creates a snapshot of the
    /// document at creation time, so any changes made afterwards won't be
    /// visible until you call this method.
    ///
    /// Args:
    ///     doc: The updated document
    ///
    /// Example:
    ///     doc = ucp.create("My Document")
    ///     traversal = ucp.AgentTraversal(doc)
    ///
    ///     # Add blocks after creating traversal
    ///     block_id = doc.add_block(doc.root_id, "New content")
    ///
    ///     # Sync changes to traversal
    ///     traversal.update_document(doc)
    ///
    ///     # Now you can navigate to the new block
    ///     traversal.navigate_to(session, block_id)
    fn update_document(&self, doc: &PyDocument) -> PyResult<()> {
        self.inner
            .update_document(doc.inner().clone())
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    /// Go back in navigation history (BACK command).
    #[pyo3(signature = (session_id, steps=1))]
    fn go_back(&self, session_id: &PyAgentSessionId, steps: usize) -> PyResult<PyNavigationResult> {
        let result = self
            .inner
            .go_back(&session_id.inner, steps)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        Ok(PyNavigationResult {
            position: PyBlockId::from(result.position),
            refreshed: result.refreshed,
        })
    }

    /// Expand from a block in a given direction (EXPAND command).
    ///
    /// Args:
    ///     session_id: The agent session
    ///     block_id: The block to expand from
    ///     direction: Direction to expand - "down" (children), "up" (ancestors),
    ///                "both" (bidirectional), or "semantic" (via semantic edges)
    ///     depth: Maximum expansion depth (default: 3)
    ///     view_mode: How to display block content (default: Full)
    ///
    /// Returns:
    ///     ExpansionResult with blocks organized by depth level
    ///
    /// Example:
    ///     # Expand children
    ///     result = traversal.expand(session, block_id, "down", depth=2)
    ///
    ///     # Expand via semantic edges
    ///     result = traversal.expand(session, block_id, "semantic", depth=1)
    #[pyo3(signature = (session_id, block_id, direction="down", depth=3, view_mode=None))]
    fn expand(
        &self,
        session_id: &PyAgentSessionId,
        block_id: &PyBlockId,
        direction: &str,
        depth: usize,
        view_mode: Option<PyViewMode>,
    ) -> PyResult<PyExpansionResult> {
        let dir = match direction.to_lowercase().as_str() {
            "down" => ucp_agent::ExpandDirection::Down,
            "up" => ucp_agent::ExpandDirection::Up,
            "both" => ucp_agent::ExpandDirection::Both,
            "semantic" => ucp_agent::ExpandDirection::Semantic,
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Invalid direction: '{}'. Valid options are: 'down' (expand children), \
                    'up' (expand ancestors), 'both' (bidirectional), or 'semantic' (follow semantic edges). \
                    Note: Use direction='semantic' for semantic expansion, not semantic=True.",
                    direction
                )))
            }
        };

        let options = ucp_agent::ExpandOptions::new()
            .with_depth(depth)
            .with_view_mode(view_mode.map(|m| m.inner).unwrap_or_default());

        let result = self
            .inner
            .expand(&session_id.inner, *block_id.inner(), dir, options)
            .map_err(|e| {
                let msg = e.to_string();
                if msg.contains("Block not found") {
                    pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "Block not found: {}. Note: If you added blocks to the document after creating \
                        the AgentTraversal, call update_document() to sync changes.",
                        block_id.inner()
                    ))
                } else {
                    pyo3::exceptions::PyRuntimeError::new_err(msg)
                }
            })?;

        Ok(PyExpansionResult {
            root: PyBlockId::from(result.root),
            levels: result
                .levels
                .into_iter()
                .map(|level| level.into_iter().map(PyBlockId::from).collect())
                .collect(),
            total_blocks: result.total_blocks,
        })
    }

    /// View a specific block (VIEW command).
    #[pyo3(signature = (session_id, block_id, view_mode=None))]
    fn view_block(
        &self,
        session_id: &PyAgentSessionId,
        block_id: &PyBlockId,
        view_mode: Option<PyViewMode>,
    ) -> PyResult<PyBlockView> {
        let mode = view_mode.map(|m| m.inner).unwrap_or_default();
        let result = self
            .inner
            .view_block(&session_id.inner, *block_id.inner(), mode)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        Ok(PyBlockView {
            block_id: PyBlockId::from(result.block_id),
            content: result.content,
            role: result.role,
            tags: result.tags,
            children_count: result.children_count,
            incoming_edges: result.incoming_edges,
            outgoing_edges: result.outgoing_edges,
        })
    }

    /// View the neighborhood around the current cursor position.
    ///
    /// Returns information about the current position and its surrounding
    /// context: ancestors, children, siblings, and semantic connections.
    ///
    /// Args:
    ///     session_id: The agent session
    ///
    /// Returns:
    ///     NeighborhoodView with position and surrounding blocks
    fn view_neighborhood(&self, session_id: &PyAgentSessionId) -> PyResult<PyNeighborhoodView> {
        let result = self
            .inner
            .view_neighborhood(&session_id.inner)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        fn convert_block_view(bv: &ucp_agent::BlockView) -> PyBlockView {
            PyBlockView {
                block_id: PyBlockId::from(bv.block_id),
                content: bv.content.clone(),
                role: bv.role.clone(),
                tags: bv.tags.clone(),
                children_count: bv.children_count,
                incoming_edges: bv.incoming_edges,
                outgoing_edges: bv.outgoing_edges,
            }
        }

        Ok(PyNeighborhoodView {
            position: PyBlockId::from(result.position),
            ancestors: result.ancestors.iter().map(convert_block_view).collect(),
            children: result.children.iter().map(convert_block_view).collect(),
            siblings: result.siblings.iter().map(convert_block_view).collect(),
            connections: result
                .connections
                .iter()
                .map(|(bv, edge_type)| PyConnection {
                    block: convert_block_view(bv),
                    edge_type: format!("{:?}", edge_type),
                })
                .collect(),
        })
    }

    /// Find blocks by pattern (FIND command).
    ///
    /// Args:
    ///     session_id: The agent session
    ///     role: Filter by semantic role (e.g., "paragraph", "heading1")
    ///     tag: Filter by a single tag
    ///     tags: Filter by multiple tags (alias for tag, uses first tag)
    ///     label: Filter by block label
    ///     pattern: Regex pattern to match content
    #[pyo3(signature = (session_id, role=None, tag=None, tags=None, label=None, pattern=None))]
    fn find(
        &self,
        session_id: &PyAgentSessionId,
        role: Option<&str>,
        tag: Option<&str>,
        tags: Option<Vec<String>>,
        label: Option<&str>,
        pattern: Option<&str>,
    ) -> PyResult<PyFindResult> {
        // Support both `tag` (singular) and `tags` (plural) for better DX
        let effective_tag = tag
            .map(|t| t.to_string())
            .or_else(|| tags.and_then(|t| t.first().cloned()));
        let tag_ref = effective_tag.as_deref();

        let result = self
            .inner
            .find_by_pattern(&session_id.inner, role, tag_ref, label, pattern)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        Ok(PyFindResult {
            matches: result.matches.into_iter().map(PyBlockId::from).collect(),
            total_searched: result.total_searched,
        })
    }

    /// Perform semantic search (SEARCH command).
    ///
    /// Requires a RAG provider to be configured. Returns blocks matching
    /// the semantic query with similarity scores.
    ///
    /// Args:
    ///     session_id: The agent session
    ///     query: The search query string
    ///     limit: Maximum number of results (default: 10)
    ///     min_similarity: Minimum similarity threshold 0.0-1.0 (default: 0.0)
    ///
    /// Returns:
    ///     SearchResult with matching blocks and similarity scores
    ///
    /// Raises:
    ///     RuntimeError: If RAG provider is not configured
    #[pyo3(signature = (session_id, query, limit=10, min_similarity=0.0))]
    fn search(
        &self,
        session_id: &PyAgentSessionId,
        query: &str,
        limit: usize,
        min_similarity: f32,
    ) -> PyResult<PySearchResult> {
        let options = ucp_agent::SearchOptions::new()
            .with_limit(limit)
            .with_min_similarity(min_similarity);

        let results = self
            .runtime
            .block_on(async { self.inner.search(&session_id.inner, query, options).await });

        let results = results.map_err(|e| {
            let msg = e.to_string();
            if msg.contains("RagNotConfigured") || msg.contains("RAG provider not configured") {
                pyo3::exceptions::PyRuntimeError::new_err(
                    "Semantic search requires a RAG provider. Use AgentTraversal with a configured RAG provider, or use find() for pattern-based search instead."
                )
            } else {
                pyo3::exceptions::PyRuntimeError::new_err(msg)
            }
        })?;

        Ok(PySearchResult {
            query: query.to_string(),
            matches: results
                .matches
                .into_iter()
                .map(|m| (PyBlockId::from(m.block_id), m.similarity, m.content_preview))
                .collect(),
            total_searched: results.total_searched,
        })
    }

    /// Find a path between two blocks (PATH command).
    #[pyo3(signature = (session_id, from_block, to_block, max_length=None))]
    fn find_path(
        &self,
        session_id: &PyAgentSessionId,
        from_block: &PyBlockId,
        to_block: &PyBlockId,
        max_length: Option<usize>,
    ) -> PyResult<Vec<PyBlockId>> {
        let result = self
            .inner
            .find_path(
                &session_id.inner,
                *from_block.inner(),
                *to_block.inner(),
                max_length,
            )
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        Ok(result.into_iter().map(PyBlockId::from).collect())
    }

    /// Add a block to context (CTX ADD command).
    #[pyo3(signature = (session_id, block_id, reason=None, relevance=None))]
    fn context_add(
        &self,
        session_id: &PyAgentSessionId,
        block_id: &PyBlockId,
        reason: Option<String>,
        relevance: Option<f32>,
    ) -> PyResult<()> {
        self.inner
            .context_add(&session_id.inner, *block_id.inner(), reason, relevance)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    /// Add all last results to context (CTX ADD RESULTS command).
    fn context_add_results(&self, session_id: &PyAgentSessionId) -> PyResult<Vec<PyBlockId>> {
        let result = self
            .inner
            .context_add_results(&session_id.inner)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        Ok(result.into_iter().map(PyBlockId::from).collect())
    }

    /// Remove a block from context (CTX REMOVE command).
    fn context_remove(&self, session_id: &PyAgentSessionId, block_id: &PyBlockId) -> PyResult<()> {
        self.inner
            .context_remove(&session_id.inner, *block_id.inner())
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    /// Clear the context window (CTX CLEAR command).
    fn context_clear(&self, session_id: &PyAgentSessionId) -> PyResult<()> {
        self.inner
            .context_clear(&session_id.inner)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    /// Set focus block (CTX FOCUS command).
    #[pyo3(signature = (session_id, block_id=None))]
    fn context_focus(
        &self,
        session_id: &PyAgentSessionId,
        block_id: Option<PyBlockId>,
    ) -> PyResult<()> {
        self.inner
            .context_focus(&session_id.inner, block_id.map(|b| *b.inner()))
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    /// Execute UCL commands from a string.
    fn execute_ucl(&self, session_id: &PyAgentSessionId, ucl_input: &str) -> PyResult<Vec<String>> {
        let results = self.runtime.block_on(async {
            ucp_agent::execute_ucl(&self.inner, &session_id.inner, ucl_input).await
        });

        let results =
            results.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        // Convert ExecutionResult to JSON strings for now
        Ok(results
            .into_iter()
            .map(|r| serde_json::to_string(&r).unwrap_or_else(|_| "{}".to_string()))
            .collect())
    }

    fn __repr__(&self) -> String {
        "AgentTraversal()".to_string()
    }
}
