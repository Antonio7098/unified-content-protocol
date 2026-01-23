//! Engine bindings for WASM.
//!
//! Exposes the UCM Engine with transaction support, validation, and traversal.

use ucm_engine::engine::{Engine, EngineConfig};
use ucm_engine::traversal::{
    NavigateDirection, TraversalConfig, TraversalEngine, TraversalFilter, TraversalOutput,
    TraversalResult,
};
use ucm_engine::validate::{ResourceLimits, ValidationPipeline, ValidationResult};
use wasm_bindgen::prelude::*;

use crate::Document;

/// Engine configuration.
#[wasm_bindgen]
pub struct WasmEngineConfig {
    inner: EngineConfig,
}

#[wasm_bindgen]
impl WasmEngineConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(
        validate_on_operation: Option<bool>,
        max_batch_size: Option<usize>,
        enable_transactions: Option<bool>,
        enable_snapshots: Option<bool>,
    ) -> WasmEngineConfig {
        WasmEngineConfig {
            inner: EngineConfig {
                validate_on_operation: validate_on_operation.unwrap_or(true),
                max_batch_size: max_batch_size.unwrap_or(10000),
                enable_transactions: enable_transactions.unwrap_or(true),
                enable_snapshots: enable_snapshots.unwrap_or(true),
            },
        }
    }

    #[wasm_bindgen(getter, js_name = validateOnOperation)]
    pub fn validate_on_operation(&self) -> bool {
        self.inner.validate_on_operation
    }

    #[wasm_bindgen(getter, js_name = maxBatchSize)]
    pub fn max_batch_size(&self) -> usize {
        self.inner.max_batch_size
    }

    #[wasm_bindgen(getter, js_name = enableTransactions)]
    pub fn enable_transactions(&self) -> bool {
        self.inner.enable_transactions
    }

    #[wasm_bindgen(getter, js_name = enableSnapshots)]
    pub fn enable_snapshots(&self) -> bool {
        self.inner.enable_snapshots
    }
}

/// The main transformation engine with transaction support.
#[wasm_bindgen]
pub struct WasmEngine {
    inner: Engine,
}

#[wasm_bindgen]
impl WasmEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(config: Option<WasmEngineConfig>) -> WasmEngine {
        let engine = match config {
            Some(c) => Engine::with_config(c.inner),
            None => Engine::new(),
        };
        WasmEngine { inner: engine }
    }

    /// Validate a document.
    pub fn validate(&self, doc: &Document) -> WasmValidationResult {
        let result = self.inner.validate(doc.inner());
        WasmValidationResult::from(result)
    }

    /// Begin a new transaction.
    #[wasm_bindgen(js_name = beginTransaction)]
    pub fn begin_transaction(&mut self) -> String {
        let id = self.inner.begin_transaction();
        id.0
    }

    /// Begin a named transaction.
    #[wasm_bindgen(js_name = beginNamedTransaction)]
    pub fn begin_named_transaction(&mut self, name: &str) -> String {
        let id = self.inner.begin_named_transaction(name);
        id.0
    }

    /// Rollback a transaction.
    #[wasm_bindgen(js_name = rollbackTransaction)]
    pub fn rollback_transaction(&mut self, txn_id: &str) -> Result<(), JsValue> {
        let id = ucm_engine::transaction::TransactionId(txn_id.to_string());
        self.inner
            .rollback_transaction(&id)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Create a snapshot.
    #[wasm_bindgen(js_name = createSnapshot)]
    pub fn create_snapshot(
        &mut self,
        name: &str,
        doc: &Document,
        description: Option<String>,
    ) -> Result<(), JsValue> {
        self.inner
            .create_snapshot(name, doc.inner(), description)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Restore from a snapshot.
    #[wasm_bindgen(js_name = restoreSnapshot)]
    pub fn restore_snapshot(&self, name: &str) -> Result<Document, JsValue> {
        let doc = self
            .inner
            .restore_snapshot(name)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(Document::new(doc))
    }

    /// List all snapshots.
    #[wasm_bindgen(js_name = listSnapshots)]
    pub fn list_snapshots(&self) -> js_sys::Array {
        let arr = js_sys::Array::new();
        for name in self.inner.list_snapshots() {
            arr.push(&JsValue::from_str(&name));
        }
        arr
    }

    /// Delete a snapshot.
    #[wasm_bindgen(js_name = deleteSnapshot)]
    pub fn delete_snapshot(&mut self, name: &str) -> bool {
        self.inner.delete_snapshot(name)
    }
}

/// Resource limits for validation.
#[wasm_bindgen]
pub struct WasmResourceLimits {
    inner: ResourceLimits,
}

#[wasm_bindgen]
impl WasmResourceLimits {
    #[wasm_bindgen(constructor)]
    pub fn new(
        max_document_size: Option<usize>,
        max_block_count: Option<usize>,
        max_block_size: Option<usize>,
        max_nesting_depth: Option<usize>,
        max_edges_per_block: Option<usize>,
    ) -> WasmResourceLimits {
        let defaults = ResourceLimits::default();
        WasmResourceLimits {
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
    #[wasm_bindgen(js_name = defaultLimits)]
    pub fn default_limits() -> WasmResourceLimits {
        WasmResourceLimits {
            inner: ResourceLimits::default(),
        }
    }

    #[wasm_bindgen(getter, js_name = maxDocumentSize)]
    pub fn max_document_size(&self) -> usize {
        self.inner.max_document_size
    }

    #[wasm_bindgen(getter, js_name = maxBlockCount)]
    pub fn max_block_count(&self) -> usize {
        self.inner.max_block_count
    }

    #[wasm_bindgen(getter, js_name = maxBlockSize)]
    pub fn max_block_size(&self) -> usize {
        self.inner.max_block_size
    }

    #[wasm_bindgen(getter, js_name = maxNestingDepth)]
    pub fn max_nesting_depth(&self) -> usize {
        self.inner.max_nesting_depth
    }

    #[wasm_bindgen(getter, js_name = maxEdgesPerBlock)]
    pub fn max_edges_per_block(&self) -> usize {
        self.inner.max_edges_per_block
    }

    /// Convert to JSON object.
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> JsValue {
        let obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("maxDocumentSize"),
            &JsValue::from_f64(self.inner.max_document_size as f64),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("maxBlockCount"),
            &JsValue::from_f64(self.inner.max_block_count as f64),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("maxBlockSize"),
            &JsValue::from_f64(self.inner.max_block_size as f64),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("maxNestingDepth"),
            &JsValue::from_f64(self.inner.max_nesting_depth as f64),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("maxEdgesPerBlock"),
            &JsValue::from_f64(self.inner.max_edges_per_block as f64),
        );
        obj.into()
    }
}

/// Validation result.
#[wasm_bindgen]
pub struct WasmValidationResult {
    valid: bool,
    issues: Vec<WasmValidationIssue>,
}

impl From<ValidationResult> for WasmValidationResult {
    fn from(result: ValidationResult) -> Self {
        Self {
            valid: result.valid,
            issues: result
                .issues
                .into_iter()
                .map(WasmValidationIssue::from)
                .collect(),
        }
    }
}

#[wasm_bindgen]
impl WasmValidationResult {
    #[wasm_bindgen(getter)]
    pub fn valid(&self) -> bool {
        self.valid
    }

    #[wasm_bindgen(getter)]
    pub fn issues(&self) -> js_sys::Array {
        let arr = js_sys::Array::new();
        for issue in &self.issues {
            arr.push(&issue.to_json());
        }
        arr
    }

    /// Get error count.
    #[wasm_bindgen(getter, js_name = errorCount)]
    pub fn error_count(&self) -> usize {
        self.issues.iter().filter(|i| i.severity == "error").count()
    }

    /// Get warning count.
    #[wasm_bindgen(getter, js_name = warningCount)]
    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == "warning")
            .count()
    }

    /// Convert to JSON object.
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> JsValue {
        let obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("valid"),
            &JsValue::from_bool(self.valid),
        );
        let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("issues"), &self.issues());
        obj.into()
    }
}

/// A single validation issue.
#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmValidationIssue {
    severity: String,
    code: String,
    message: String,
}

impl From<ucm_core::ValidationIssue> for WasmValidationIssue {
    fn from(issue: ucm_core::ValidationIssue) -> Self {
        Self {
            severity: format!("{:?}", issue.severity).to_lowercase(),
            code: format!("{:?}", issue.code),
            message: issue.message,
        }
    }
}

#[wasm_bindgen]
impl WasmValidationIssue {
    #[wasm_bindgen(getter)]
    pub fn severity(&self) -> String {
        self.severity.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn code(&self) -> String {
        self.code.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }

    /// Convert to JSON object.
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> JsValue {
        let obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("severity"),
            &JsValue::from_str(&self.severity),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("code"),
            &JsValue::from_str(&self.code),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("message"),
            &JsValue::from_str(&self.message),
        );
        obj.into()
    }
}

/// Validation pipeline with configurable resource limits.
#[wasm_bindgen]
pub struct WasmValidationPipeline {
    inner: ValidationPipeline,
}

#[wasm_bindgen]
impl WasmValidationPipeline {
    #[wasm_bindgen(constructor)]
    pub fn new(limits: Option<WasmResourceLimits>) -> WasmValidationPipeline {
        let pipeline = match limits {
            Some(l) => ValidationPipeline::with_limits(l.inner),
            None => ValidationPipeline::new(),
        };
        WasmValidationPipeline { inner: pipeline }
    }

    /// Validate a document.
    pub fn validate(&self, doc: &Document) -> WasmValidationResult {
        let result = self.inner.validate_document(doc.inner());
        WasmValidationResult::from(result)
    }
}

fn parse_direction(s: &str) -> NavigateDirection {
    match s.to_lowercase().as_str() {
        "down" => NavigateDirection::Down,
        "up" => NavigateDirection::Up,
        "both" => NavigateDirection::Both,
        "siblings" => NavigateDirection::Siblings,
        "breadth_first" | "bfs" | "breadthfirst" => NavigateDirection::BreadthFirst,
        "depth_first" | "dfs" | "depthfirst" => NavigateDirection::DepthFirst,
        _ => NavigateDirection::Down,
    }
}

/// Traversal filter for filtering blocks during traversal.
#[wasm_bindgen]
pub struct WasmTraversalFilter {
    include_roles: Vec<String>,
    exclude_roles: Vec<String>,
    include_tags: Vec<String>,
    exclude_tags: Vec<String>,
    content_pattern: Option<String>,
}

#[wasm_bindgen]
impl WasmTraversalFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(
        include_roles: Option<Vec<String>>,
        exclude_roles: Option<Vec<String>>,
        include_tags: Option<Vec<String>>,
        exclude_tags: Option<Vec<String>>,
        content_pattern: Option<String>,
    ) -> WasmTraversalFilter {
        WasmTraversalFilter {
            include_roles: include_roles.unwrap_or_default(),
            exclude_roles: exclude_roles.unwrap_or_default(),
            include_tags: include_tags.unwrap_or_default(),
            exclude_tags: exclude_tags.unwrap_or_default(),
            content_pattern,
        }
    }
}

impl From<&WasmTraversalFilter> for TraversalFilter {
    fn from(f: &WasmTraversalFilter) -> Self {
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
#[wasm_bindgen]
pub struct WasmTraversalConfig {
    inner: TraversalConfig,
}

#[wasm_bindgen]
impl WasmTraversalConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(
        max_depth: Option<usize>,
        max_nodes: Option<usize>,
        include_orphans: Option<bool>,
    ) -> WasmTraversalConfig {
        WasmTraversalConfig {
            inner: TraversalConfig {
                max_depth: max_depth.unwrap_or(100),
                max_nodes: max_nodes.unwrap_or(10000),
                default_preview_length: 100,
                include_orphans: include_orphans.unwrap_or(false),
                cache_enabled: true,
            },
        }
    }

    #[wasm_bindgen(getter, js_name = maxDepth)]
    pub fn max_depth(&self) -> usize {
        self.inner.max_depth
    }

    #[wasm_bindgen(getter, js_name = maxNodes)]
    pub fn max_nodes(&self) -> usize {
        self.inner.max_nodes
    }
}

/// Graph traversal engine for UCM documents.
#[wasm_bindgen]
pub struct WasmTraversalEngine {
    inner: TraversalEngine,
}

#[wasm_bindgen]
impl WasmTraversalEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(config: Option<WasmTraversalConfig>) -> WasmTraversalEngine {
        let engine = match config {
            Some(c) => TraversalEngine::with_config(c.inner),
            None => TraversalEngine::new(),
        };
        WasmTraversalEngine { inner: engine }
    }

    /// Navigate from a starting point in a specific direction.
    pub fn navigate(
        &self,
        doc: &Document,
        direction: &str,
        start_id: Option<String>,
        depth: Option<usize>,
        filter: Option<WasmTraversalFilter>,
    ) -> Result<JsValue, JsValue> {
        let dir = parse_direction(direction);
        let start = start_id.and_then(|s| s.parse().ok());
        let filt = filter.as_ref().map(TraversalFilter::from);

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
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(traversal_result_to_js(&result))
    }

    /// Expand a node to get its immediate children.
    pub fn expand(&self, doc: &Document, node_id: &str) -> Result<JsValue, JsValue> {
        let id: ucm_core::BlockId = node_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", node_id)))?;

        let result = self
            .inner
            .expand(doc.inner(), &id, TraversalOutput::StructureWithPreviews)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(traversal_result_to_js(&result))
    }

    /// Get the path from a node to the root.
    #[wasm_bindgen(js_name = pathToRoot)]
    pub fn path_to_root(&self, doc: &Document, node_id: &str) -> Result<js_sys::Array, JsValue> {
        let id: ucm_core::BlockId = node_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", node_id)))?;

        let path = self
            .inner
            .path_to_root(doc.inner(), &id)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let arr = js_sys::Array::new();
        for block_id in path {
            arr.push(&JsValue::from_str(&block_id.to_string()));
        }
        Ok(arr)
    }

    /// Find all paths between two nodes.
    #[wasm_bindgen(js_name = findPaths)]
    pub fn find_paths(
        &self,
        doc: &Document,
        from_id: &str,
        to_id: &str,
        max_paths: Option<usize>,
    ) -> Result<js_sys::Array, JsValue> {
        let from: ucm_core::BlockId = from_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", from_id)))?;
        let to: ucm_core::BlockId = to_id
            .parse()
            .map_err(|_| JsValue::from_str(&format!("Invalid block ID: {}", to_id)))?;

        let paths = self
            .inner
            .find_paths(doc.inner(), &from, &to, max_paths.unwrap_or(10))
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let arr = js_sys::Array::new();
        for path in paths {
            let path_arr = js_sys::Array::new();
            for block_id in path {
                path_arr.push(&JsValue::from_str(&block_id.to_string()));
            }
            arr.push(&path_arr);
        }
        Ok(arr)
    }
}

fn traversal_result_to_js(result: &TraversalResult) -> JsValue {
    let obj = js_sys::Object::new();

    // Nodes array
    let nodes_arr = js_sys::Array::new();
    for node in &result.nodes {
        let node_obj = js_sys::Object::new();
        let _ = js_sys::Reflect::set(
            &node_obj,
            &JsValue::from_str("id"),
            &JsValue::from_str(&node.id.to_string()),
        );
        let _ = js_sys::Reflect::set(
            &node_obj,
            &JsValue::from_str("depth"),
            &JsValue::from_f64(node.depth as f64),
        );
        if let Some(parent) = &node.parent_id {
            let _ = js_sys::Reflect::set(
                &node_obj,
                &JsValue::from_str("parentId"),
                &JsValue::from_str(&parent.to_string()),
            );
        }
        if let Some(preview) = &node.content_preview {
            let _ = js_sys::Reflect::set(
                &node_obj,
                &JsValue::from_str("contentPreview"),
                &JsValue::from_str(preview),
            );
        }
        if let Some(role) = &node.semantic_role {
            let _ = js_sys::Reflect::set(
                &node_obj,
                &JsValue::from_str("semanticRole"),
                &JsValue::from_str(role),
            );
        }
        let _ = js_sys::Reflect::set(
            &node_obj,
            &JsValue::from_str("childCount"),
            &JsValue::from_f64(node.child_count as f64),
        );
        let _ = js_sys::Reflect::set(
            &node_obj,
            &JsValue::from_str("edgeCount"),
            &JsValue::from_f64(node.edge_count as f64),
        );
        nodes_arr.push(&node_obj);
    }
    let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("nodes"), &nodes_arr);

    // Summary
    let summary_obj = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &summary_obj,
        &JsValue::from_str("totalNodes"),
        &JsValue::from_f64(result.summary.total_nodes as f64),
    );
    let _ = js_sys::Reflect::set(
        &summary_obj,
        &JsValue::from_str("maxDepth"),
        &JsValue::from_f64(result.summary.max_depth as f64),
    );
    let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("summary"), &summary_obj);

    // Metadata
    if let Some(time) = result.metadata.execution_time_ms {
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("executionTimeMs"),
            &JsValue::from_f64(time as f64),
        );
    }

    obj.into()
}
