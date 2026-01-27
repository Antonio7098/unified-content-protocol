//! WASM bindings for agent graph traversal.
//!
//! Exposes the UCP Agent traversal system for JavaScript/TypeScript usage.

use std::sync::Arc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::Document;
use ucp_agent::{
    AgentCapabilities, AgentError, AgentSessionId, AgentTraversal, ExpandDirection, ExpandOptions,
    GlobalLimits, SearchOptions, SessionConfig, SessionLimits, ViewMode,
};

/// WASM wrapper for AgentSessionId.
#[wasm_bindgen]
pub struct WasmAgentSessionId {
    inner: AgentSessionId,
}

#[wasm_bindgen]
impl WasmAgentSessionId {
    /// Get the session ID as a string.
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string_js(&self) -> String {
        self.inner.to_string()
    }

    /// Check if two session IDs are equal.
    pub fn equals(&self, other: &WasmAgentSessionId) -> bool {
        self.inner == other.inner
    }
}

impl From<AgentSessionId> for WasmAgentSessionId {
    fn from(id: AgentSessionId) -> Self {
        Self { inner: id }
    }
}

/// Session limits configuration.
#[wasm_bindgen]
pub struct WasmSessionLimits {
    inner: SessionLimits,
}

#[wasm_bindgen]
impl WasmSessionLimits {
    #[wasm_bindgen(constructor)]
    pub fn new(
        max_context_tokens: Option<usize>,
        max_context_blocks: Option<usize>,
        max_expand_depth: Option<usize>,
        max_results_per_op: Option<usize>,
        session_timeout_secs: Option<u64>,
    ) -> WasmSessionLimits {
        let mut limits = SessionLimits::default();

        if let Some(v) = max_context_tokens {
            limits.max_context_tokens = v;
        }
        if let Some(v) = max_context_blocks {
            limits.max_context_blocks = v;
        }
        if let Some(v) = max_expand_depth {
            limits.max_expand_depth = v;
        }
        if let Some(v) = max_results_per_op {
            limits.max_results_per_operation = v;
        }
        if let Some(v) = session_timeout_secs {
            limits.session_timeout = std::time::Duration::from_secs(v);
        }

        WasmSessionLimits { inner: limits }
    }

    /// Create default limits.
    #[wasm_bindgen(js_name = defaultLimits)]
    pub fn default_limits() -> WasmSessionLimits {
        WasmSessionLimits {
            inner: SessionLimits::default(),
        }
    }

    #[wasm_bindgen(getter, js_name = maxContextTokens)]
    pub fn max_context_tokens(&self) -> usize {
        self.inner.max_context_tokens
    }

    #[wasm_bindgen(getter, js_name = maxContextBlocks)]
    pub fn max_context_blocks(&self) -> usize {
        self.inner.max_context_blocks
    }

    #[wasm_bindgen(getter, js_name = maxExpandDepth)]
    pub fn max_expand_depth(&self) -> usize {
        self.inner.max_expand_depth
    }

    #[wasm_bindgen(getter, js_name = maxResultsPerOp)]
    pub fn max_results_per_op(&self) -> usize {
        self.inner.max_results_per_operation
    }

    #[wasm_bindgen(getter, js_name = sessionTimeoutSecs)]
    pub fn session_timeout_secs(&self) -> u64 {
        self.inner.session_timeout.as_secs()
    }
}

/// Global limits configuration.
#[wasm_bindgen]
pub struct WasmGlobalLimits {
    inner: GlobalLimits,
}

#[wasm_bindgen]
impl WasmGlobalLimits {
    #[wasm_bindgen(constructor)]
    pub fn new(
        max_sessions: Option<usize>,
        max_total_context_blocks: Option<usize>,
        max_ops_per_second: Option<f64>,
    ) -> WasmGlobalLimits {
        let mut limits = GlobalLimits::default();

        if let Some(v) = max_sessions {
            limits.max_sessions = v;
        }
        if let Some(v) = max_total_context_blocks {
            limits.max_total_context_blocks = v;
        }
        if let Some(v) = max_ops_per_second {
            limits.max_ops_per_second = v;
        }

        WasmGlobalLimits { inner: limits }
    }

    /// Create default global limits.
    #[wasm_bindgen(js_name = defaultLimits)]
    pub fn default_limits() -> WasmGlobalLimits {
        WasmGlobalLimits {
            inner: GlobalLimits::default(),
        }
    }

    #[wasm_bindgen(getter, js_name = maxSessions)]
    pub fn max_sessions(&self) -> usize {
        self.inner.max_sessions
    }

    #[wasm_bindgen(getter, js_name = maxTotalContextBlocks)]
    pub fn max_total_context_blocks(&self) -> usize {
        self.inner.max_total_context_blocks
    }

    #[wasm_bindgen(getter, js_name = maxOpsPerSecond)]
    pub fn max_ops_per_second(&self) -> f64 {
        self.inner.max_ops_per_second
    }
}

/// Agent capabilities configuration.
#[wasm_bindgen]
pub struct WasmAgentCapabilities {
    inner: AgentCapabilities,
}

#[wasm_bindgen]
impl WasmAgentCapabilities {
    #[wasm_bindgen(constructor)]
    pub fn new(
        can_traverse: Option<bool>,
        can_search: Option<bool>,
        can_modify_context: Option<bool>,
        can_coordinate: Option<bool>,
        max_expand_depth: Option<usize>,
    ) -> WasmAgentCapabilities {
        let mut caps = AgentCapabilities::default();

        if let Some(v) = can_traverse {
            caps.can_traverse = v;
        }
        if let Some(v) = can_search {
            caps.can_search = v;
        }
        if let Some(v) = can_modify_context {
            caps.can_modify_context = v;
        }
        if let Some(v) = can_coordinate {
            caps.can_coordinate = v;
        }
        if let Some(v) = max_expand_depth {
            caps.max_expand_depth = v;
        }

        WasmAgentCapabilities { inner: caps }
    }

    /// Create full capabilities (all permissions).
    pub fn full() -> WasmAgentCapabilities {
        WasmAgentCapabilities {
            inner: AgentCapabilities::full(),
        }
    }

    /// Create read-only capabilities.
    #[wasm_bindgen(js_name = readOnly)]
    pub fn read_only() -> WasmAgentCapabilities {
        WasmAgentCapabilities {
            inner: AgentCapabilities::read_only(),
        }
    }

    #[wasm_bindgen(getter, js_name = canTraverse)]
    pub fn can_traverse(&self) -> bool {
        self.inner.can_traverse
    }

    #[wasm_bindgen(getter, js_name = canSearch)]
    pub fn can_search(&self) -> bool {
        self.inner.can_search
    }

    #[wasm_bindgen(getter, js_name = canModifyContext)]
    pub fn can_modify_context(&self) -> bool {
        self.inner.can_modify_context
    }

    #[wasm_bindgen(getter, js_name = canCoordinate)]
    pub fn can_coordinate(&self) -> bool {
        self.inner.can_coordinate
    }

    #[wasm_bindgen(getter, js_name = maxExpandDepth)]
    pub fn max_expand_depth(&self) -> usize {
        self.inner.max_expand_depth
    }
}

/// Session configuration.
#[wasm_bindgen]
pub struct WasmSessionConfig {
    name: Option<String>,
    start_block: Option<String>,
    limits: Option<SessionLimits>,
    capabilities: Option<AgentCapabilities>,
    view_mode: Option<ViewMode>,
}

#[wasm_bindgen]
impl WasmSessionConfig {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmSessionConfig {
        WasmSessionConfig {
            name: None,
            start_block: None,
            limits: None,
            capabilities: None,
            view_mode: None,
        }
    }

    /// Set the session name.
    #[wasm_bindgen(js_name = withName)]
    pub fn with_name(mut self, name: &str) -> WasmSessionConfig {
        self.name = Some(name.to_string());
        self
    }

    /// Set the starting block ID.
    #[wasm_bindgen(js_name = withStartBlock)]
    pub fn with_start_block(mut self, block_id: &str) -> WasmSessionConfig {
        self.start_block = Some(block_id.to_string());
        self
    }

    /// Set session limits.
    #[wasm_bindgen(js_name = withLimits)]
    pub fn with_limits(mut self, limits: WasmSessionLimits) -> WasmSessionConfig {
        self.limits = Some(limits.inner);
        self
    }

    /// Set agent capabilities.
    #[wasm_bindgen(js_name = withCapabilities)]
    pub fn with_capabilities(mut self, capabilities: WasmAgentCapabilities) -> WasmSessionConfig {
        self.capabilities = Some(capabilities.inner);
        self
    }

    /// Set view mode to IDs only.
    #[wasm_bindgen(js_name = withViewModeIds)]
    pub fn with_view_mode_ids(mut self) -> WasmSessionConfig {
        self.view_mode = Some(ViewMode::IdsOnly);
        self
    }

    /// Set view mode to preview.
    #[wasm_bindgen(js_name = withViewModePreview)]
    pub fn with_view_mode_preview(mut self, length: usize) -> WasmSessionConfig {
        self.view_mode = Some(ViewMode::Preview { length });
        self
    }

    /// Set view mode to full.
    #[wasm_bindgen(js_name = withViewModeFull)]
    pub fn with_view_mode_full(mut self) -> WasmSessionConfig {
        self.view_mode = Some(ViewMode::Full);
        self
    }

    /// Set view mode to metadata.
    #[wasm_bindgen(js_name = withViewModeMetadata)]
    pub fn with_view_mode_metadata(mut self) -> WasmSessionConfig {
        self.view_mode = Some(ViewMode::Metadata);
        self
    }

    fn to_inner(&self) -> SessionConfig {
        let mut config = SessionConfig::new();

        if let Some(ref name) = self.name {
            config = config.with_name(name);
        }
        if let Some(ref block_id) = self.start_block {
            if let Ok(id) = block_id.parse() {
                config = config.with_start_block(id);
            }
        }
        if let Some(ref limits) = self.limits {
            config = config.with_limits(limits.clone());
        }
        if let Some(ref capabilities) = self.capabilities {
            config = config.with_capabilities(capabilities.clone());
        }
        if let Some(ref view_mode) = self.view_mode {
            config = config.with_view_mode(view_mode.clone());
        }

        config
    }
}

impl Default for WasmSessionConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Expand direction enum for JavaScript.
#[wasm_bindgen]
pub enum WasmExpandDirection {
    Down = 0,
    Up = 1,
    Both = 2,
    Semantic = 3,
}

impl From<WasmExpandDirection> for ExpandDirection {
    fn from(d: WasmExpandDirection) -> Self {
        match d {
            WasmExpandDirection::Down => ExpandDirection::Down,
            WasmExpandDirection::Up => ExpandDirection::Up,
            WasmExpandDirection::Both => ExpandDirection::Both,
            WasmExpandDirection::Semantic => ExpandDirection::Semantic,
        }
    }
}

/// Expand options configuration.
#[wasm_bindgen]
pub struct WasmExpandOptions {
    inner: ExpandOptions,
}

#[wasm_bindgen]
impl WasmExpandOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmExpandOptions {
        WasmExpandOptions {
            inner: ExpandOptions::new(),
        }
    }

    /// Set the expansion depth.
    #[wasm_bindgen(js_name = withDepth)]
    pub fn with_depth(mut self, depth: usize) -> WasmExpandOptions {
        self.inner = self.inner.with_depth(depth);
        self
    }

    /// Set view mode to IDs only.
    #[wasm_bindgen(js_name = withViewModeIds)]
    pub fn with_view_mode_ids(mut self) -> WasmExpandOptions {
        self.inner = self.inner.with_view_mode(ViewMode::IdsOnly);
        self
    }

    /// Set view mode to preview.
    #[wasm_bindgen(js_name = withViewModePreview)]
    pub fn with_view_mode_preview(mut self, length: usize) -> WasmExpandOptions {
        self.inner = self.inner.with_view_mode(ViewMode::Preview { length });
        self
    }

    /// Set view mode to full.
    #[wasm_bindgen(js_name = withViewModeFull)]
    pub fn with_view_mode_full(mut self) -> WasmExpandOptions {
        self.inner = self.inner.with_view_mode(ViewMode::Full);
        self
    }

    /// Set filter by roles (comma-separated).
    #[wasm_bindgen(js_name = withRoles)]
    pub fn with_roles(mut self, roles: &str) -> WasmExpandOptions {
        let role_vec: Vec<String> = roles.split(',').map(|s| s.trim().to_string()).collect();
        self.inner = self.inner.with_roles(role_vec);
        self
    }

    /// Set filter by tags (comma-separated).
    #[wasm_bindgen(js_name = withTags)]
    pub fn with_tags(mut self, tags: &str) -> WasmExpandOptions {
        let tag_vec: Vec<String> = tags.split(',').map(|s| s.trim().to_string()).collect();
        self.inner = self.inner.with_tags(tag_vec);
        self
    }
}

impl Default for WasmExpandOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Search options configuration.
#[wasm_bindgen]
pub struct WasmSearchOptions {
    inner: SearchOptions,
}

#[wasm_bindgen]
impl WasmSearchOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmSearchOptions {
        WasmSearchOptions {
            inner: SearchOptions::new(),
        }
    }

    /// Set the result limit.
    #[wasm_bindgen(js_name = withLimit)]
    pub fn with_limit(mut self, limit: usize) -> WasmSearchOptions {
        self.inner = self.inner.with_limit(limit);
        self
    }

    /// Set minimum similarity threshold.
    #[wasm_bindgen(js_name = withMinSimilarity)]
    pub fn with_min_similarity(mut self, threshold: f32) -> WasmSearchOptions {
        self.inner = self.inner.with_min_similarity(threshold);
        self
    }
}

impl Default for WasmSearchOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Main agent traversal interface for WASM.
#[wasm_bindgen]
pub struct WasmAgentTraversal {
    inner: Arc<AgentTraversal>,
}

#[wasm_bindgen]
impl WasmAgentTraversal {
    /// Create a new agent traversal system.
    #[wasm_bindgen(constructor)]
    pub fn new(doc: &Document) -> WasmAgentTraversal {
        let traversal = AgentTraversal::new(doc.inner().clone());
        WasmAgentTraversal {
            inner: Arc::new(traversal),
        }
    }

    /// Create with custom global limits.
    #[wasm_bindgen(js_name = withGlobalLimits)]
    pub fn with_global_limits(doc: &Document, limits: WasmGlobalLimits) -> WasmAgentTraversal {
        let traversal = AgentTraversal::new(doc.inner().clone()).with_global_limits(limits.inner);
        WasmAgentTraversal {
            inner: Arc::new(traversal),
        }
    }

    /// Update the internal document with a new copy.
    ///
    /// Use this when you've added blocks to the original document after
    /// creating the AgentTraversal. The traversal creates a snapshot of the
    /// document at creation time, so any changes made afterwards won't be
    /// visible until you call this method.
    ///
    /// @example
    /// ```javascript
    /// const doc = Document.create();
    /// const traversal = new WasmAgentTraversal(doc);
    ///
    /// // Add blocks after creating traversal
    /// const blockId = doc.addBlock(doc.rootId, "New content");
    ///
    /// // Sync changes to traversal
    /// traversal.updateDocument(doc);
    ///
    /// // Now you can navigate to the new block
    /// traversal.navigateTo(session, blockId);
    /// ```
    #[wasm_bindgen(js_name = updateDocument)]
    pub fn update_document(&self, doc: &Document) -> Result<(), JsValue> {
        self.inner
            .update_document(doc.inner().clone())
            .map_err(agent_error_to_js)
    }

    // ==================== Session Management ====================

    /// Create a new agent session.
    #[wasm_bindgen(js_name = createSession)]
    pub fn create_session(
        &self,
        config: Option<WasmSessionConfig>,
    ) -> Result<WasmAgentSessionId, JsValue> {
        let session_config = config.map(|c| c.to_inner()).unwrap_or_default();
        let session_id = self
            .inner
            .create_session(session_config)
            .map_err(agent_error_to_js)?;
        Ok(WasmAgentSessionId::from(session_id))
    }

    /// Close a session.
    #[wasm_bindgen(js_name = closeSession)]
    pub fn close_session(&self, session_id: &WasmAgentSessionId) -> Result<(), JsValue> {
        self.inner
            .close_session(&session_id.inner)
            .map_err(agent_error_to_js)
    }

    // ==================== Navigation ====================

    /// Navigate to a specific block.
    #[wasm_bindgen(js_name = navigateTo)]
    pub fn navigate_to(
        &self,
        session_id: &WasmAgentSessionId,
        block_id: &str,
    ) -> Result<JsValue, JsValue> {
        let target = block_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", block_id)))?;

        let result = self
            .inner
            .navigate_to(&session_id.inner, target)
            .map_err(agent_error_to_js)?;

        Ok(navigation_result_to_js(&result))
    }

    /// Go back in navigation history.
    #[wasm_bindgen(js_name = goBack)]
    pub fn go_back(
        &self,
        session_id: &WasmAgentSessionId,
        steps: Option<usize>,
    ) -> Result<JsValue, JsValue> {
        let result = self
            .inner
            .go_back(&session_id.inner, steps.unwrap_or(1))
            .map_err(agent_error_to_js)?;

        Ok(navigation_result_to_js(&result))
    }

    // ==================== Expansion ====================

    /// Expand from a block in a given direction.
    pub fn expand(
        &self,
        session_id: &WasmAgentSessionId,
        block_id: &str,
        direction: WasmExpandDirection,
        options: Option<WasmExpandOptions>,
    ) -> Result<JsValue, JsValue> {
        let target = block_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", block_id)))?;

        let opts = options.map(|o| o.inner).unwrap_or_default();

        let result = self
            .inner
            .expand(&session_id.inner, target, direction.into(), opts)
            .map_err(agent_error_to_js)?;

        Ok(expansion_result_to_js(&result))
    }

    // ==================== Search ====================

    /// Perform semantic search (requires RAG provider).
    pub fn search(
        &self,
        session_id: &WasmAgentSessionId,
        query: &str,
        options: Option<WasmSearchOptions>,
    ) -> js_sys::Promise {
        let inner = Arc::clone(&self.inner);
        let session_id = session_id.inner.clone();
        let query = query.to_string();
        let opts = options.map(|o| o.inner).unwrap_or_default();

        future_to_promise(async move {
            let result = inner
                .search(&session_id, &query, opts)
                .await
                .map_err(agent_error_to_js)?;

            Ok(search_result_to_js(&result, &query))
        })
    }

    /// Find blocks by pattern (no RAG required).
    ///
    /// @param session_id - The session ID
    /// @param role - Filter by role
    /// @param tag - Filter by a single tag (deprecated: use tags instead)
    /// @param tags - Filter by multiple tags (comma-separated string)
    /// @param label - Filter by label pattern
    /// @param pattern - Filter by content pattern (regex)
    #[wasm_bindgen(js_name = findByPattern)]
    pub fn find_by_pattern(
        &self,
        session_id: &WasmAgentSessionId,
        role: Option<String>,
        tag: Option<String>,
        tags: Option<String>,
        label: Option<String>,
        pattern: Option<String>,
    ) -> Result<JsValue, JsValue> {
        // Handle both singular 'tag' and plural 'tags' parameters
        let effective_tag = if let Some(ref tags_str) = tags {
            // If tags is provided, use the first tag for the API
            // (the underlying API only supports one tag at a time currently)
            tags_str.split(',').next().map(|s| s.trim().to_string())
        } else {
            tag
        };

        let result = self
            .inner
            .find_by_pattern(
                &session_id.inner,
                role.as_deref(),
                effective_tag.as_deref(),
                label.as_deref(),
                pattern.as_deref(),
            )
            .map_err(agent_error_to_js)?;

        Ok(find_result_to_js(&result))
    }

    // ==================== View ====================

    /// View a specific block.
    #[wasm_bindgen(js_name = viewBlock)]
    pub fn view_block(
        &self,
        session_id: &WasmAgentSessionId,
        block_id: &str,
        view_mode: Option<String>,
    ) -> Result<JsValue, JsValue> {
        let target = block_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", block_id)))?;

        let mode = parse_view_mode(view_mode.as_deref());

        let result = self
            .inner
            .view_block(&session_id.inner, target, mode)
            .map_err(agent_error_to_js)?;

        Ok(block_view_to_js(&result))
    }

    /// View the neighborhood around the current cursor position.
    #[wasm_bindgen(js_name = viewNeighborhood)]
    pub fn view_neighborhood(&self, session_id: &WasmAgentSessionId) -> Result<JsValue, JsValue> {
        let result = self
            .inner
            .view_neighborhood(&session_id.inner)
            .map_err(agent_error_to_js)?;

        Ok(neighborhood_view_to_js(&result))
    }

    // ==================== Path Finding ====================

    /// Find a path between two blocks.
    #[wasm_bindgen(js_name = findPath)]
    pub fn find_path(
        &self,
        session_id: &WasmAgentSessionId,
        from_id: &str,
        to_id: &str,
        max_length: Option<usize>,
    ) -> Result<js_sys::Array, JsValue> {
        let from = from_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", from_id)))?;
        let to = to_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", to_id)))?;

        let path = self
            .inner
            .find_path(&session_id.inner, from, to, max_length)
            .map_err(agent_error_to_js)?;

        let arr = js_sys::Array::new();
        for block_id in path {
            arr.push(&JsValue::from_str(&block_id.to_string()));
        }
        Ok(arr)
    }

    // ==================== Context Operations ====================

    /// Add a block to the context window.
    #[wasm_bindgen(js_name = contextAdd)]
    pub fn context_add(
        &self,
        session_id: &WasmAgentSessionId,
        block_id: &str,
        reason: Option<String>,
        relevance: Option<f32>,
    ) -> Result<(), JsValue> {
        let target = block_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", block_id)))?;

        self.inner
            .context_add(&session_id.inner, target, reason, relevance)
            .map_err(agent_error_to_js)
    }

    /// Add all last results to context.
    #[wasm_bindgen(js_name = contextAddResults)]
    pub fn context_add_results(
        &self,
        session_id: &WasmAgentSessionId,
    ) -> Result<js_sys::Array, JsValue> {
        let results = self
            .inner
            .context_add_results(&session_id.inner)
            .map_err(agent_error_to_js)?;

        let arr = js_sys::Array::new();
        for block_id in results {
            arr.push(&JsValue::from_str(&block_id.to_string()));
        }
        Ok(arr)
    }

    /// Remove a block from context.
    #[wasm_bindgen(js_name = contextRemove)]
    pub fn context_remove(
        &self,
        session_id: &WasmAgentSessionId,
        block_id: &str,
    ) -> Result<(), JsValue> {
        let target = block_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", block_id)))?;

        self.inner
            .context_remove(&session_id.inner, target)
            .map_err(agent_error_to_js)
    }

    /// Clear the context window.
    #[wasm_bindgen(js_name = contextClear)]
    pub fn context_clear(&self, session_id: &WasmAgentSessionId) -> Result<(), JsValue> {
        self.inner
            .context_clear(&session_id.inner)
            .map_err(agent_error_to_js)
    }

    /// Set focus block.
    #[wasm_bindgen(js_name = contextFocus)]
    pub fn context_focus(
        &self,
        session_id: &WasmAgentSessionId,
        block_id: Option<String>,
    ) -> Result<(), JsValue> {
        let target = block_id
            .map(|id| {
                id.parse()
                    .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", id)))
            })
            .transpose()?;

        self.inner
            .context_focus(&session_id.inner, target)
            .map_err(agent_error_to_js)
    }

    // ==================== UCL Execution ====================

    /// Execute UCL commands from a string.
    #[wasm_bindgen(js_name = executeUcl)]
    pub fn execute_ucl(&self, session_id: &WasmAgentSessionId, ucl_input: &str) -> js_sys::Promise {
        let inner = Arc::clone(&self.inner);
        let session_id = session_id.inner.clone();
        let ucl_input = ucl_input.to_string();

        future_to_promise(async move {
            let results = ucp_agent::execute_ucl(&inner, &session_id, &ucl_input)
                .await
                .map_err(agent_error_to_js)?;

            let arr = js_sys::Array::new();
            for result in results {
                let json = serde_json::to_string(&result)
                    .map_err(|e| JsValue::from_str(&e.to_string()))?;
                arr.push(&JsValue::from_str(&json));
            }
            Ok(JsValue::from(arr))
        })
    }
}

// ==================== Helper Functions ====================

fn agent_error_to_js(err: AgentError) -> JsValue {
    let msg = match &err {
        AgentError::BlockNotFound(_) => {
            format!(
                "{}\n\nHint: If you recently added this block to the document after creating \
                the AgentTraversal, call updateDocument(doc) first to sync the changes.",
                err
            )
        }
        _ => err.to_string(),
    };
    JsValue::from_str(&msg)
}

fn parse_view_mode(mode: Option<&str>) -> ViewMode {
    match mode {
        Some("ids") | Some("ids_only") => ViewMode::IdsOnly,
        Some("metadata") => ViewMode::Metadata,
        Some("preview") => ViewMode::Preview { length: 100 },
        Some("full") => ViewMode::Full,
        _ => ViewMode::default(),
    }
}

fn navigation_result_to_js(result: &ucp_agent::NavigationResult) -> JsValue {
    let obj = js_sys::Object::new();

    let _ = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("position"),
        &JsValue::from_str(&result.position.to_string()),
    );
    let _ = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("refreshed"),
        &JsValue::from_bool(result.refreshed),
    );

    // Include neighborhood summary
    let neighborhood = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &neighborhood,
        &JsValue::from_str("ancestorCount"),
        &JsValue::from_f64(result.neighborhood.ancestors.len() as f64),
    );
    let _ = js_sys::Reflect::set(
        &neighborhood,
        &JsValue::from_str("childCount"),
        &JsValue::from_f64(result.neighborhood.children.len() as f64),
    );
    let _ = js_sys::Reflect::set(
        &neighborhood,
        &JsValue::from_str("siblingCount"),
        &JsValue::from_f64(result.neighborhood.siblings.len() as f64),
    );
    let _ = js_sys::Reflect::set(
        &neighborhood,
        &JsValue::from_str("connectionCount"),
        &JsValue::from_f64(result.neighborhood.connections.len() as f64),
    );

    let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("neighborhood"), &neighborhood);

    obj.into()
}

fn expansion_result_to_js(result: &ucp_agent::ExpansionResult) -> JsValue {
    let obj = js_sys::Object::new();

    let _ = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("root"),
        &JsValue::from_str(&result.root.to_string()),
    );
    let _ = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("totalBlocks"),
        &JsValue::from_f64(result.total_blocks as f64),
    );

    let levels_arr = js_sys::Array::new();
    for level in &result.levels {
        let level_arr = js_sys::Array::new();
        for block_id in level {
            level_arr.push(&JsValue::from_str(&block_id.to_string()));
        }
        levels_arr.push(&level_arr);
    }
    let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("levels"), &levels_arr);

    obj.into()
}

fn search_result_to_js(result: &ucp_agent::RagSearchResults, query: &str) -> JsValue {
    let obj = js_sys::Object::new();

    let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("query"), &JsValue::from_str(query));
    let _ = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("totalSearched"),
        &JsValue::from_f64(result.total_searched as f64),
    );

    let matches_arr = js_sys::Array::new();
    for m in &result.matches {
        let match_obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(
            &match_obj,
            &JsValue::from_str("blockId"),
            &JsValue::from_str(&m.block_id.to_string()),
        );
        let _ = js_sys::Reflect::set(
            &match_obj,
            &JsValue::from_str("similarity"),
            &JsValue::from_f64(m.similarity as f64),
        );
        if let Some(ref preview) = m.content_preview {
            let _ = js_sys::Reflect::set(
                &match_obj,
                &JsValue::from_str("preview"),
                &JsValue::from_str(preview),
            );
        }
        matches_arr.push(&match_obj);
    }
    let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("matches"), &matches_arr);

    obj.into()
}

fn find_result_to_js(result: &ucp_agent::FindResult) -> JsValue {
    let obj = js_sys::Object::new();

    let _ = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("totalSearched"),
        &JsValue::from_f64(result.total_searched as f64),
    );

    let matches_arr = js_sys::Array::new();
    for block_id in &result.matches {
        matches_arr.push(&JsValue::from_str(&block_id.to_string()));
    }
    let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("matches"), &matches_arr);

    obj.into()
}

fn block_view_to_js(view: &ucp_agent::BlockView) -> JsValue {
    let obj = js_sys::Object::new();

    let _ = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("blockId"),
        &JsValue::from_str(&view.block_id.to_string()),
    );
    if let Some(ref content) = view.content {
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("content"),
            &JsValue::from_str(content),
        );
    }
    if let Some(ref role) = view.role {
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("role"), &JsValue::from_str(role));
    }

    let tags_arr = js_sys::Array::new();
    for tag in &view.tags {
        tags_arr.push(&JsValue::from_str(tag));
    }
    let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("tags"), &tags_arr);

    let _ = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("childrenCount"),
        &JsValue::from_f64(view.children_count as f64),
    );
    let _ = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("incomingEdges"),
        &JsValue::from_f64(view.incoming_edges as f64),
    );
    let _ = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("outgoingEdges"),
        &JsValue::from_f64(view.outgoing_edges as f64),
    );

    obj.into()
}

fn neighborhood_view_to_js(view: &ucp_agent::NeighborhoodView) -> JsValue {
    let obj = js_sys::Object::new();

    let _ = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("position"),
        &JsValue::from_str(&view.position.to_string()),
    );

    // Ancestors
    let ancestors_arr = js_sys::Array::new();
    for block_view in &view.ancestors {
        ancestors_arr.push(&block_view_to_js(block_view));
    }
    let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("ancestors"), &ancestors_arr);

    // Children
    let children_arr = js_sys::Array::new();
    for block_view in &view.children {
        children_arr.push(&block_view_to_js(block_view));
    }
    let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("children"), &children_arr);

    // Siblings
    let siblings_arr = js_sys::Array::new();
    for block_view in &view.siblings {
        siblings_arr.push(&block_view_to_js(block_view));
    }
    let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("siblings"), &siblings_arr);

    // Connections
    let connections_arr = js_sys::Array::new();
    for (block_view, edge_type) in &view.connections {
        let conn_obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(
            &conn_obj,
            &JsValue::from_str("block"),
            &block_view_to_js(block_view),
        );
        let _ = js_sys::Reflect::set(
            &conn_obj,
            &JsValue::from_str("edgeType"),
            &JsValue::from_str(&format!("{:?}", edge_type)),
        );
        connections_arr.push(&conn_obj);
    }
    let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("connections"), &connections_arr);

    obj.into()
}
