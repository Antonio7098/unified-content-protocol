mod document;
#[cfg(not(target_arch = "wasm32"))]
mod sqlite;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use ucm_core::{BlockId, PortableDocument};

use crate::types::GraphEdgeSummary;

pub use document::InMemoryGraphStore;
#[cfg(not(target_arch = "wasm32"))]
pub use sqlite::SqliteGraphStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNodeRecord {
    pub block_id: BlockId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub content_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_role: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<BlockId>,
    pub children: usize,
    pub outgoing_edges: usize,
    pub incoming_edges: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStoreStats {
    pub backend: String,
    pub document_id: String,
    pub root_block_id: BlockId,
    pub node_count: usize,
    pub explicit_edge_count: usize,
    pub structural_edge_count: usize,
    pub captured_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStoreObservability {
    pub stats: GraphStoreStats,
    #[serde(default)]
    pub indexed_fields: Vec<String>,
}

#[derive(Debug, Error)]
pub enum GraphStoreError {
    #[error(transparent)]
    Ucm(#[from] ucm_core::Error),
    #[error(transparent)]
    Regex(#[from] regex::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    #[cfg(not(target_arch = "wasm32"))]
    Sqlite(#[from] rusqlite::Error),
    #[error("graph not found: {0}")]
    GraphNotFound(String),
}

pub trait GraphStore {
    fn stats(&self) -> GraphStoreStats;
    fn observability(&self) -> GraphStoreObservability;
    fn root_id(&self) -> BlockId;
    fn node_ids(&self) -> Vec<BlockId>;
    fn node(&self, block_id: BlockId) -> Option<GraphNodeRecord>;
    fn children(&self, block_id: BlockId) -> Vec<BlockId>;
    fn parent(&self, block_id: BlockId) -> Option<BlockId>;
    fn outgoing_edges(&self, block_id: BlockId) -> Vec<GraphEdgeSummary>;
    fn incoming_edges(&self, block_id: BlockId) -> Vec<GraphEdgeSummary>;
    fn resolve_selector(&self, selector: &str) -> Option<BlockId>;
    fn to_portable_document(&self) -> Result<PortableDocument, GraphStoreError>;
}
