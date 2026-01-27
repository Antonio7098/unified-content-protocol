//! Core operations for agent graph traversal.

use crate::cursor::{CursorNeighborhood, ViewMode};
use crate::error::{AgentError, AgentSessionId, Result};
use crate::rag::{RagProvider, RagSearchOptions, RagSearchResults};
use crate::safety::{CircuitBreaker, DepthGuard, GlobalLimits};
use crate::session::{AgentSession, SessionConfig};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Instant;
use ucm_core::{BlockId, Document, EdgeType};
use ucm_engine::traversal::{NavigateDirection, TraversalEngine, TraversalFilter, TraversalOutput};

/// Result of a navigation operation.
#[derive(Debug, Clone)]
pub struct NavigationResult {
    /// New position after navigation.
    pub position: BlockId,
    /// Whether the neighborhood was refreshed.
    pub refreshed: bool,
    /// Current neighborhood.
    pub neighborhood: CursorNeighborhood,
}

/// Options for expansion operations.
#[derive(Debug, Clone, Default)]
pub struct ExpandOptions {
    /// Maximum depth to expand.
    pub depth: usize,
    /// View mode for results.
    pub view_mode: ViewMode,
    /// Filter by semantic roles.
    pub roles: Option<Vec<String>>,
    /// Filter by tags.
    pub tags: Option<Vec<String>>,
}

impl ExpandOptions {
    pub fn new() -> Self {
        Self {
            depth: 3,
            ..Default::default()
        }
    }

    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    pub fn with_view_mode(mut self, mode: ViewMode) -> Self {
        self.view_mode = mode;
        self
    }

    pub fn with_roles(mut self, roles: Vec<String>) -> Self {
        self.roles = Some(roles);
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }
}

/// Result of an expansion operation.
#[derive(Debug, Clone)]
pub struct ExpansionResult {
    /// Root of the expansion.
    pub root: BlockId,
    /// Expanded blocks by depth level.
    pub levels: Vec<Vec<BlockId>>,
    /// Total blocks expanded.
    pub total_blocks: usize,
}

/// Direction for expansion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpandDirection {
    /// Expand to children (descendants).
    Down,
    /// Expand to parents (ancestors).
    Up,
    /// Expand in both directions.
    Both,
    /// Expand via semantic edges only.
    Semantic,
}

impl From<ucl_parser::ast::ExpandDirection> for ExpandDirection {
    fn from(d: ucl_parser::ast::ExpandDirection) -> Self {
        match d {
            ucl_parser::ast::ExpandDirection::Down => ExpandDirection::Down,
            ucl_parser::ast::ExpandDirection::Up => ExpandDirection::Up,
            ucl_parser::ast::ExpandDirection::Both => ExpandDirection::Both,
            ucl_parser::ast::ExpandDirection::Semantic => ExpandDirection::Semantic,
        }
    }
}

/// Options for search operations.
#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
    /// Maximum results to return.
    pub limit: usize,
    /// Minimum similarity threshold.
    pub min_similarity: f32,
    /// Filter by semantic roles.
    pub roles: Option<Vec<String>>,
    /// Filter by tags.
    pub tags: Option<Vec<String>>,
}

impl SearchOptions {
    pub fn new() -> Self {
        Self {
            limit: 10,
            min_similarity: 0.0,
            roles: None,
            tags: None,
        }
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn with_min_similarity(mut self, threshold: f32) -> Self {
        self.min_similarity = threshold;
        self
    }
}

/// Result of a find operation.
#[derive(Debug, Clone)]
pub struct FindResult {
    /// Matching block IDs.
    pub matches: Vec<BlockId>,
    /// Total blocks searched.
    pub total_searched: usize,
}

/// View of a block's content.
#[derive(Debug, Clone)]
pub struct BlockView {
    /// Block ID.
    pub block_id: BlockId,
    /// Content (based on view mode).
    pub content: Option<String>,
    /// Semantic role.
    pub role: Option<String>,
    /// Tags.
    pub tags: Vec<String>,
    /// Children count.
    pub children_count: usize,
    /// Incoming edges count.
    pub incoming_edges: usize,
    /// Outgoing edges count.
    pub outgoing_edges: usize,
}

/// View of the cursor neighborhood.
#[derive(Debug, Clone)]
pub struct NeighborhoodView {
    /// Current position.
    pub position: BlockId,
    /// Parent blocks.
    pub ancestors: Vec<BlockView>,
    /// Child blocks.
    pub children: Vec<BlockView>,
    /// Sibling blocks.
    pub siblings: Vec<BlockView>,
    /// Connected blocks via semantic edges.
    pub connections: Vec<(BlockView, EdgeType)>,
}

/// Main interface for agent graph traversal operations.
pub struct AgentTraversal {
    /// Active sessions.
    sessions: RwLock<HashMap<AgentSessionId, AgentSession>>,
    /// The document being traversed.
    document: Arc<RwLock<Document>>,
    /// Optional RAG provider for semantic search.
    rag_provider: Option<Arc<dyn RagProvider>>,
    /// Global limits for all sessions.
    global_limits: GlobalLimits,
    /// Circuit breaker for fault tolerance.
    circuit_breaker: CircuitBreaker,
    /// Depth guard for recursion protection.
    depth_guard: DepthGuard,
}

impl AgentTraversal {
    /// Create a new agent traversal system.
    pub fn new(document: Document) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            document: Arc::new(RwLock::new(document)),
            rag_provider: None,
            global_limits: GlobalLimits::default(),
            circuit_breaker: CircuitBreaker::new(5, std::time::Duration::from_secs(30)),
            depth_guard: DepthGuard::new(100),
        }
    }

    /// Create with a RAG provider.
    pub fn with_rag_provider(mut self, provider: Arc<dyn RagProvider>) -> Self {
        self.rag_provider = Some(provider);
        self
    }

    /// Create with custom global limits.
    pub fn with_global_limits(mut self, limits: GlobalLimits) -> Self {
        self.global_limits = limits;
        self
    }

    /// Update the internal document with a new copy.
    ///
    /// Use this when you've added blocks to the original document
    /// after creating the AgentTraversal.
    pub fn update_document(&self, document: Document) -> Result<()> {
        let mut doc = self
            .document
            .write()
            .map_err(|_| AgentError::Internal("Failed to acquire document lock".to_string()))?;
        *doc = document;
        Ok(())
    }

    /// Get a clone of the internal document.
    pub fn get_document(&self) -> Result<Document> {
        let doc = self
            .document
            .read()
            .map_err(|_| AgentError::Internal("Failed to acquire document lock".to_string()))?;
        Ok(doc.clone())
    }

    // ==================== Session Management ====================

    /// Create a new agent session.
    pub fn create_session(&self, config: SessionConfig) -> Result<AgentSessionId> {
        self.circuit_breaker.can_proceed()?;

        let mut sessions = self
            .sessions
            .write()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        if sessions.len() >= self.global_limits.max_sessions {
            return Err(AgentError::MaxSessionsReached {
                max: self.global_limits.max_sessions,
            });
        }

        let doc = self
            .document
            .read()
            .map_err(|_| AgentError::Internal("Failed to acquire document lock".to_string()))?;

        // Get start block or default to document root
        let start_block = config.start_block.unwrap_or(doc.root);

        let session = AgentSession::new(start_block, config);
        let session_id = session.id.clone();
        sessions.insert(session_id.clone(), session);

        Ok(session_id)
    }

    /// Get a reference to a session.
    pub fn get_session(
        &self,
        id: &AgentSessionId,
    ) -> Result<std::sync::RwLockReadGuard<'_, HashMap<AgentSessionId, AgentSession>>> {
        let sessions = self
            .sessions
            .read()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;
        if !sessions.contains_key(id) {
            return Err(AgentError::SessionNotFound(id.clone()));
        }
        Ok(sessions)
    }

    /// Close a session.
    pub fn close_session(&self, id: &AgentSessionId) -> Result<()> {
        let mut sessions = self
            .sessions
            .write()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        let session = sessions
            .get_mut(id)
            .ok_or_else(|| AgentError::SessionNotFound(id.clone()))?;

        session.complete();
        sessions.remove(id);
        Ok(())
    }

    // ==================== Navigation ====================

    /// Navigate to a specific block.
    pub fn navigate_to(
        &self,
        session_id: &AgentSessionId,
        target: BlockId,
    ) -> Result<NavigationResult> {
        self.circuit_breaker.can_proceed()?;
        let _guard = self.depth_guard.try_enter()?;
        let start = Instant::now();

        let mut sessions = self
            .sessions
            .write()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| AgentError::SessionNotFound(session_id.clone()))?;

        session.check_can_traverse()?;

        // Verify block exists
        let doc = self
            .document
            .read()
            .map_err(|_| AgentError::Internal("Failed to acquire document lock".to_string()))?;

        if doc.get_block(&target).is_none() {
            return Err(AgentError::BlockNotFound(target));
        }

        // Move cursor
        session.cursor.move_to(target);
        session.touch();
        session.metrics.record_navigation();
        session.budget.record_traversal();

        // Refresh neighborhood
        let neighborhood = self.compute_neighborhood(&doc, &target)?;
        session.cursor.update_neighborhood(neighborhood.clone());

        session.metrics.record_execution_time(start.elapsed());

        Ok(NavigationResult {
            position: target,
            refreshed: true,
            neighborhood,
        })
    }

    /// Go back in navigation history.
    pub fn go_back(&self, session_id: &AgentSessionId, steps: usize) -> Result<NavigationResult> {
        self.circuit_breaker.can_proceed()?;
        let start = Instant::now();

        let mut sessions = self
            .sessions
            .write()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| AgentError::SessionNotFound(session_id.clone()))?;

        session.check_can_traverse()?;

        let position = session
            .cursor
            .go_back(steps)
            .ok_or(AgentError::EmptyHistory)?;

        session.touch();
        session.metrics.record_navigation();

        // Refresh neighborhood
        let doc = self
            .document
            .read()
            .map_err(|_| AgentError::Internal("Failed to acquire document lock".to_string()))?;

        let neighborhood = self.compute_neighborhood(&doc, &position)?;
        session.cursor.update_neighborhood(neighborhood.clone());

        session.metrics.record_execution_time(start.elapsed());

        Ok(NavigationResult {
            position,
            refreshed: true,
            neighborhood,
        })
    }

    // ==================== Expansion ====================

    /// Expand from a block in a given direction.
    pub fn expand(
        &self,
        session_id: &AgentSessionId,
        block_id: BlockId,
        direction: ExpandDirection,
        options: ExpandOptions,
    ) -> Result<ExpansionResult> {
        self.circuit_breaker.can_proceed()?;
        let _guard = self.depth_guard.try_enter()?;
        let start = Instant::now();

        let sessions = self
            .sessions
            .read()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        let session = sessions
            .get(session_id)
            .ok_or_else(|| AgentError::SessionNotFound(session_id.clone()))?;

        session.check_can_traverse()?;

        // Check depth limit
        if options.depth > session.limits.max_expand_depth {
            return Err(AgentError::DepthLimitExceeded {
                current: options.depth,
                max: session.limits.max_expand_depth,
            });
        }

        let doc = self
            .document
            .read()
            .map_err(|_| AgentError::Internal("Failed to acquire document lock".to_string()))?;

        // Build traversal filter
        let filter = self.build_traversal_filter(&options);

        let levels = match direction {
            ExpandDirection::Down => self.expand_down(&doc, &block_id, options.depth, &filter)?,
            ExpandDirection::Up => self.expand_up(&doc, &block_id, options.depth)?,
            ExpandDirection::Both => {
                let mut down = self.expand_down(&doc, &block_id, options.depth, &filter)?;
                let up = self.expand_up(&doc, &block_id, options.depth)?;
                down.extend(up);
                down
            }
            ExpandDirection::Semantic => self.expand_semantic(&doc, &block_id, options.depth)?,
        };

        let total_blocks: usize = levels.iter().map(|l| l.len()).sum();

        // Update metrics
        drop(sessions);
        let mut sessions_mut = self
            .sessions
            .write()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        if let Some(session) = sessions_mut.get_mut(session_id) {
            session.metrics.record_expansion(total_blocks);
            session.budget.record_traversal();
            session.metrics.record_execution_time(start.elapsed());
            session.touch();
        }

        Ok(ExpansionResult {
            root: block_id,
            levels,
            total_blocks,
        })
    }

    // ==================== Search ====================

    /// Perform semantic search (requires RAG provider).
    pub async fn search(
        &self,
        session_id: &AgentSessionId,
        query: &str,
        options: SearchOptions,
    ) -> Result<RagSearchResults> {
        self.circuit_breaker.can_proceed()?;
        let start = Instant::now();

        let rag = self
            .rag_provider
            .as_ref()
            .ok_or(AgentError::RagNotConfigured)?;

        {
            let sessions = self
                .sessions
                .read()
                .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

            let session = sessions
                .get(session_id)
                .ok_or_else(|| AgentError::SessionNotFound(session_id.clone()))?;

            session.check_can_search()?;
        }

        let rag_options = RagSearchOptions::new()
            .with_limit(options.limit)
            .with_min_similarity(options.min_similarity);

        let results = rag.search(query, rag_options).await?;

        // Store results for CTX ADD RESULTS
        let mut sessions = self
            .sessions
            .write()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        if let Some(session) = sessions.get_mut(session_id) {
            session.store_results(results.block_ids());
            session.metrics.record_search();
            session.metrics.record_execution_time(start.elapsed());
            session.touch();
        }

        Ok(results)
    }

    /// Find blocks by pattern (no RAG required).
    pub fn find_by_pattern(
        &self,
        session_id: &AgentSessionId,
        role: Option<&str>,
        tag: Option<&str>,
        label: Option<&str>,
        pattern: Option<&str>,
    ) -> Result<FindResult> {
        self.circuit_breaker.can_proceed()?;
        let start = Instant::now();

        let sessions = self
            .sessions
            .read()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        let session = sessions
            .get(session_id)
            .ok_or_else(|| AgentError::SessionNotFound(session_id.clone()))?;

        session.check_can_search()?;

        let doc = self
            .document
            .read()
            .map_err(|_| AgentError::Internal("Failed to acquire document lock".to_string()))?;

        let mut matches = Vec::new();
        let mut total_searched = 0;

        // Build regex if pattern provided
        let regex = pattern
            .map(regex::Regex::new)
            .transpose()
            .map_err(|e| AgentError::Internal(format!("Invalid regex pattern: {}", e)))?;

        for block in doc.blocks.values() {
            total_searched += 1;

            // Filter by role
            if let Some(r) = role {
                let block_role = block
                    .metadata
                    .semantic_role
                    .as_ref()
                    .map(|sr| sr.category.as_str())
                    .unwrap_or("");
                if block_role != r {
                    continue;
                }
            }

            // Filter by tag
            if let Some(t) = tag {
                if !block.metadata.tags.contains(&t.to_string()) {
                    continue;
                }
            }

            // Filter by label
            if let Some(l) = label {
                if block.metadata.label.as_deref() != Some(l) {
                    continue;
                }
            }

            // Filter by content pattern
            if let Some(ref re) = regex {
                let content = self.extract_content_text(&block.content);
                if !re.is_match(&content) {
                    continue;
                }
            }

            matches.push(block.id);
        }

        // Store results for CTX ADD RESULTS
        drop(sessions);
        let mut sessions_mut = self
            .sessions
            .write()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        if let Some(session) = sessions_mut.get_mut(session_id) {
            session.store_results(matches.clone());
            session.metrics.record_search();
            session.metrics.record_execution_time(start.elapsed());
            session.touch();
        }

        Ok(FindResult {
            matches,
            total_searched,
        })
    }

    // ==================== View ====================

    /// View a specific block.
    pub fn view_block(
        &self,
        session_id: &AgentSessionId,
        block_id: BlockId,
        mode: ViewMode,
    ) -> Result<BlockView> {
        self.circuit_breaker.can_proceed()?;

        let sessions = self
            .sessions
            .read()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        let session = sessions
            .get(session_id)
            .ok_or_else(|| AgentError::SessionNotFound(session_id.clone()))?;

        session.check_active()?;

        let doc = self
            .document
            .read()
            .map_err(|_| AgentError::Internal("Failed to acquire document lock".to_string()))?;

        self.view_block_internal(&doc, &block_id, &mode)
    }

    /// View the neighborhood around the current cursor position.
    pub fn view_neighborhood(&self, session_id: &AgentSessionId) -> Result<NeighborhoodView> {
        self.circuit_breaker.can_proceed()?;

        let sessions = self
            .sessions
            .read()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        let session = sessions
            .get(session_id)
            .ok_or_else(|| AgentError::SessionNotFound(session_id.clone()))?;

        session.check_active()?;

        let position = session.cursor.position;
        let view_mode = session.cursor.view_mode.clone();

        // Release session lock before calling view_block
        drop(sessions);

        let doc = self
            .document
            .read()
            .map_err(|_| AgentError::Internal("Failed to acquire document lock".to_string()))?;

        let neighborhood = self.compute_neighborhood(&doc, &position)?;

        // Build views for each block in neighborhood
        let ancestors: Vec<BlockView> = neighborhood
            .ancestors
            .iter()
            .filter_map(|id| self.view_block_internal(&doc, id, &view_mode).ok())
            .collect();

        let children: Vec<BlockView> = neighborhood
            .children
            .iter()
            .filter_map(|id| self.view_block_internal(&doc, id, &view_mode).ok())
            .collect();

        let siblings: Vec<BlockView> = neighborhood
            .siblings
            .iter()
            .filter_map(|id| self.view_block_internal(&doc, id, &view_mode).ok())
            .collect();

        let connections: Vec<(BlockView, EdgeType)> = neighborhood
            .connections
            .iter()
            .filter_map(|(id, edge_type)| {
                self.view_block_internal(&doc, id, &view_mode)
                    .ok()
                    .map(|view| (view, edge_type.clone()))
            })
            .collect();

        Ok(NeighborhoodView {
            position,
            ancestors,
            children,
            siblings,
            connections,
        })
    }

    // ==================== Path Finding ====================

    /// Find a path between two blocks.
    pub fn find_path(
        &self,
        session_id: &AgentSessionId,
        from: BlockId,
        to: BlockId,
        max_length: Option<usize>,
    ) -> Result<Vec<BlockId>> {
        self.circuit_breaker.can_proceed()?;
        let _guard = self.depth_guard.try_enter()?;
        let start = Instant::now();

        let sessions = self
            .sessions
            .read()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        let session = sessions
            .get(session_id)
            .ok_or_else(|| AgentError::SessionNotFound(session_id.clone()))?;

        session.check_can_traverse()?;

        let doc = self
            .document
            .read()
            .map_err(|_| AgentError::Internal("Failed to acquire document lock".to_string()))?;

        // Simple BFS path finding
        let max_depth = max_length.unwrap_or(10);
        let path = self.bfs_path(&doc, &from, &to, max_depth)?;

        // Update metrics
        drop(sessions);
        let mut sessions_mut = self
            .sessions
            .write()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        if let Some(session) = sessions_mut.get_mut(session_id) {
            session.metrics.record_traversal();
            session.budget.record_traversal();
            session.metrics.record_execution_time(start.elapsed());
            session.touch();
        }

        Ok(path)
    }

    // ==================== Context Operations ====================

    /// Add a block to the context window.
    pub fn context_add(
        &self,
        session_id: &AgentSessionId,
        block_id: BlockId,
        _reason: Option<String>,
        _relevance: Option<f32>,
    ) -> Result<()> {
        let mut sessions = self
            .sessions
            .write()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| AgentError::SessionNotFound(session_id.clone()))?;

        session.check_can_modify_context()?;

        // For now, we just verify the block exists
        let doc = self
            .document
            .read()
            .map_err(|_| AgentError::Internal("Failed to acquire document lock".to_string()))?;

        if doc.get_block(&block_id).is_none() {
            return Err(AgentError::BlockNotFound(block_id));
        }

        session.metrics.record_context_add(1);
        session.touch();

        // Context management is handled by the external context manager
        // This is a placeholder for integration
        Ok(())
    }

    /// Add all last results to context.
    pub fn context_add_results(&self, session_id: &AgentSessionId) -> Result<Vec<BlockId>> {
        let mut sessions = self
            .sessions
            .write()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| AgentError::SessionNotFound(session_id.clone()))?;

        session.check_can_modify_context()?;

        let results = session.get_last_results()?.to_vec();
        session.metrics.record_context_add(results.len());
        session.touch();

        Ok(results)
    }

    /// Remove a block from context.
    pub fn context_remove(&self, session_id: &AgentSessionId, _block_id: BlockId) -> Result<()> {
        let mut sessions = self
            .sessions
            .write()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| AgentError::SessionNotFound(session_id.clone()))?;

        session.check_can_modify_context()?;
        session.metrics.record_context_remove();
        session.touch();

        Ok(())
    }

    /// Clear the context window.
    pub fn context_clear(&self, session_id: &AgentSessionId) -> Result<()> {
        let mut sessions = self
            .sessions
            .write()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| AgentError::SessionNotFound(session_id.clone()))?;

        session.check_can_modify_context()?;
        session.touch();

        Ok(())
    }

    /// Set focus block.
    pub fn context_focus(
        &self,
        session_id: &AgentSessionId,
        block_id: Option<BlockId>,
    ) -> Result<()> {
        let mut sessions = self
            .sessions
            .write()
            .map_err(|_| AgentError::Internal("Failed to acquire sessions lock".to_string()))?;

        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| AgentError::SessionNotFound(session_id.clone()))?;

        session.set_focus(block_id);
        session.touch();

        Ok(())
    }

    // ==================== Internal Helpers ====================

    fn compute_neighborhood(
        &self,
        doc: &Document,
        position: &BlockId,
    ) -> Result<CursorNeighborhood> {
        let mut neighborhood = CursorNeighborhood::new();

        // Get ancestors
        let mut current = *position;
        for _ in 0..5 {
            if let Some(parent) = doc.parent(&current) {
                neighborhood.ancestors.push(*parent);
                current = *parent;
            } else {
                break;
            }
        }

        // Get children
        neighborhood.children = doc.children(position).to_vec();

        // Get siblings
        if let Some(parent) = doc.parent(position) {
            neighborhood.siblings = doc
                .children(parent)
                .iter()
                .filter(|id| *id != position)
                .copied()
                .collect();
        }

        // Get semantic connections via edge index
        for (edge_type, target) in doc.edge_index.outgoing_from(position) {
            neighborhood.connections.push((*target, edge_type.clone()));
        }

        neighborhood.stale = false;
        Ok(neighborhood)
    }

    fn view_block_internal(
        &self,
        doc: &Document,
        block_id: &BlockId,
        mode: &ViewMode,
    ) -> Result<BlockView> {
        let block = doc
            .get_block(block_id)
            .ok_or(AgentError::BlockNotFound(*block_id))?;

        let content = match mode {
            ViewMode::IdsOnly => None,
            ViewMode::Preview { length } => {
                let text = self.extract_content_text(&block.content);
                Some(text.chars().take(*length).collect())
            }
            ViewMode::Full => Some(self.extract_content_text(&block.content)),
            ViewMode::Metadata => None,
            ViewMode::Adaptive { .. } => Some(self.extract_content_text(&block.content)),
        };

        let outgoing_edges = doc.edge_index.outgoing_from(block_id).len();
        let incoming_edges = doc.edge_index.incoming_to(block_id).len();

        Ok(BlockView {
            block_id: *block_id,
            content,
            role: block
                .metadata
                .semantic_role
                .as_ref()
                .map(|r| r.category.as_str().to_string()),
            tags: block.metadata.tags.clone(),
            children_count: doc.children(block_id).len(),
            incoming_edges,
            outgoing_edges,
        })
    }

    fn extract_content_text(&self, content: &ucm_core::Content) -> String {
        match content {
            ucm_core::Content::Text(t) => t.text.clone(),
            ucm_core::Content::Code(c) => c.source.clone(),
            ucm_core::Content::Table(t) => format!("Table: {} rows", t.rows.len()),
            ucm_core::Content::Math(m) => m.expression.clone(),
            ucm_core::Content::Media(m) => {
                m.alt_text.clone().unwrap_or_else(|| "Media".to_string())
            }
            ucm_core::Content::Json { .. } => "JSON data".to_string(),
            ucm_core::Content::Binary { .. } => "Binary data".to_string(),
            ucm_core::Content::Composite { children, .. } => {
                format!("Composite: {} children", children.len())
            }
        }
    }

    fn build_traversal_filter(&self, options: &ExpandOptions) -> TraversalFilter {
        let mut filter = TraversalFilter::default();

        if let Some(ref roles) = options.roles {
            filter.include_roles = roles.clone();
        }

        if let Some(ref tags) = options.tags {
            filter.include_tags = tags.clone();
        }

        filter
    }

    fn expand_down(
        &self,
        doc: &Document,
        block_id: &BlockId,
        depth: usize,
        filter: &TraversalFilter,
    ) -> Result<Vec<Vec<BlockId>>> {
        let engine = TraversalEngine::new();
        let result = engine
            .navigate(
                doc,
                Some(*block_id),
                NavigateDirection::BreadthFirst,
                Some(depth),
                Some(filter.clone()),
                TraversalOutput::StructureOnly,
            )
            .map_err(|e| AgentError::EngineError(e.to_string()))?;

        // Group by depth level
        let mut levels: Vec<Vec<BlockId>> = vec![Vec::new(); depth + 1];
        for node in result.nodes {
            if node.depth <= depth {
                levels[node.depth].push(node.id);
            }
        }

        // Remove empty trailing levels
        while levels.last().map(|l| l.is_empty()).unwrap_or(false) {
            levels.pop();
        }

        Ok(levels)
    }

    fn expand_up(
        &self,
        doc: &Document,
        block_id: &BlockId,
        depth: usize,
    ) -> Result<Vec<Vec<BlockId>>> {
        let mut levels = Vec::new();
        let mut current = *block_id;

        for _ in 0..depth {
            if let Some(parent) = doc.parent(&current) {
                levels.push(vec![*parent]);
                current = *parent;
            } else {
                break;
            }
        }

        Ok(levels)
    }

    fn expand_semantic(
        &self,
        doc: &Document,
        block_id: &BlockId,
        depth: usize,
    ) -> Result<Vec<Vec<BlockId>>> {
        let mut levels = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut current_level = vec![*block_id];
        visited.insert(*block_id);

        for _ in 0..depth {
            let mut next_level = Vec::new();

            for id in &current_level {
                for (_, target) in doc.edge_index.outgoing_from(id) {
                    if !visited.contains(target) {
                        visited.insert(*target);
                        next_level.push(*target);
                    }
                }
            }

            if next_level.is_empty() {
                break;
            }

            levels.push(next_level.clone());
            current_level = next_level;
        }

        Ok(levels)
    }

    fn bfs_path(
        &self,
        doc: &Document,
        from: &BlockId,
        to: &BlockId,
        max_depth: usize,
    ) -> Result<Vec<BlockId>> {
        use std::collections::{HashSet, VecDeque};

        if from == to {
            return Ok(vec![*from]);
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent_map: HashMap<BlockId, BlockId> = HashMap::new();

        queue.push_back((*from, 0));
        visited.insert(*from);

        while let Some((current, depth)) = queue.pop_front() {
            if depth >= max_depth {
                continue;
            }

            // Check children
            for child in doc.children(&current) {
                if !visited.contains(child) {
                    visited.insert(*child);
                    parent_map.insert(*child, current);

                    if child == to {
                        // Reconstruct path
                        let mut path = vec![*to];
                        let mut c = *to;
                        while let Some(p) = parent_map.get(&c) {
                            path.push(*p);
                            c = *p;
                        }
                        path.reverse();
                        return Ok(path);
                    }

                    queue.push_back((*child, depth + 1));
                }
            }

            // Check parent
            if let Some(parent) = doc.parent(&current) {
                if !visited.contains(parent) {
                    visited.insert(*parent);
                    parent_map.insert(*parent, current);

                    if parent == to {
                        let mut path = vec![*to];
                        let mut c = *to;
                        while let Some(p) = parent_map.get(&c) {
                            path.push(*p);
                            c = *p;
                        }
                        path.reverse();
                        return Ok(path);
                    }

                    queue.push_back((*parent, depth + 1));
                }
            }

            // Check semantic edges
            for (_, target) in doc.edge_index.outgoing_from(&current) {
                if !visited.contains(target) {
                    visited.insert(*target);
                    parent_map.insert(*target, current);

                    if target == to {
                        let mut path = vec![*to];
                        let mut c = *to;
                        while let Some(p) = parent_map.get(&c) {
                            path.push(*p);
                            c = *p;
                        }
                        path.reverse();
                        return Ok(path);
                    }

                    queue.push_back((*target, depth + 1));
                }
            }
        }

        Err(AgentError::NoPathExists {
            from: *from,
            to: *to,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_document() -> Document {
        Document::create()
    }

    #[test]
    fn test_create_session() {
        let doc = create_test_document();
        let traversal = AgentTraversal::new(doc);

        let session_id = traversal.create_session(SessionConfig::default()).unwrap();
        assert!(!session_id.0.is_nil());
    }

    #[test]
    fn test_close_session() {
        let doc = create_test_document();
        let traversal = AgentTraversal::new(doc);

        let session_id = traversal.create_session(SessionConfig::default()).unwrap();
        assert!(traversal.close_session(&session_id).is_ok());

        // Session should be removed
        assert!(traversal.close_session(&session_id).is_err());
    }

    #[test]
    fn test_max_sessions_limit() {
        let doc = create_test_document();
        let traversal = AgentTraversal::new(doc).with_global_limits(GlobalLimits {
            max_sessions: 2,
            ..Default::default()
        });

        // Create 2 sessions
        let _ = traversal.create_session(SessionConfig::default()).unwrap();
        let _ = traversal.create_session(SessionConfig::default()).unwrap();

        // Third should fail
        let result = traversal.create_session(SessionConfig::default());
        assert!(matches!(
            result,
            Err(AgentError::MaxSessionsReached { max: 2 })
        ));
    }
}
