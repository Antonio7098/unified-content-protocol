use serde::{Deserialize, Serialize};
use ucm_core::BlockId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum GraphDetailLevel {
    Stub,
    #[default]
    Summary,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNodeSummary {
    pub block_id: BlockId,
    pub label: String,
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
pub struct GraphEdgeSummary {
    pub source: BlockId,
    pub target: BlockId,
    pub relation: String,
    pub direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphPathHop {
    pub from: BlockId,
    pub to: BlockId,
    pub relation: String,
    pub direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphPathResult {
    pub start: GraphNodeSummary,
    pub end: GraphNodeSummary,
    #[serde(default)]
    pub hops: Vec<GraphPathHop>,
}
