use serde::{Deserialize, Serialize};
use ucm_core::BlockId;

use crate::{
    CodeGraphCoderef, CodeGraphContextUpdate, CodeGraphDetailLevel, CodeGraphExportOmissionDetail,
    CodeGraphOperationBudget, CodeGraphRecommendation, CodeGraphSelectionOrigin,
    CodeGraphSessionEvent,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeGraphExpandMode {
    File,
    Dependencies,
    Dependents,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeGraphFindQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_regex: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_regex: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_key_regex: Option<String>,
    #[serde(default)]
    pub case_sensitive: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exported: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphNodeSummary {
    pub block_id: BlockId,
    pub node_class: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub exported: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coderef: Option<CodeGraphCoderef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphPathHop {
    pub from: BlockId,
    pub to: BlockId,
    pub relation: String,
    pub direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphPathResult {
    pub start: CodeGraphNodeSummary,
    pub end: CodeGraphNodeSummary,
    #[serde(default)]
    pub hops: Vec<CodeGraphPathHop>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphSelectionExplanation {
    pub selector: String,
    pub block_id: BlockId,
    pub selected: bool,
    pub focus: bool,
    pub pinned: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail_level: Option<CodeGraphDetailLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<CodeGraphSelectionOrigin>,
    pub explanation: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<CodeGraphNodeSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<CodeGraphNodeSummary>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provenance_chain: Vec<CodeGraphProvenanceStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphProvenanceStep {
    pub block_id: BlockId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<CodeGraphNodeSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<CodeGraphSelectionOrigin>,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphSessionDiff {
    #[serde(default)]
    pub added: Vec<CodeGraphNodeSummary>,
    #[serde(default)]
    pub removed: Vec<CodeGraphNodeSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus_before: Option<BlockId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus_after: Option<BlockId>,
    pub changed_focus: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphRecommendedActionsResult {
    #[serde(default)]
    pub applied_actions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommendations: Vec<CodeGraphRecommendation>,
    pub update: CodeGraphContextUpdate,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<CodeGraphSessionEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphSelectorResolutionExplanation {
    pub selector: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_block_id: Option<BlockId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_kind: Option<String>,
    pub ambiguous: bool,
    pub explanation: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub candidates: Vec<CodeGraphNodeSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphExportOmissionExplanation {
    pub selector: String,
    pub omitted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<BlockId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<CodeGraphExportOmissionDetail>,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphPruneExplanation {
    pub selector: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<BlockId>,
    pub pruned: bool,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphMutationEstimate {
    pub operation: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_block_id: Option<BlockId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resolved_block_ids: Vec<BlockId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub budget: Option<CodeGraphOperationBudget>,
    pub estimated_nodes_added: usize,
    pub estimated_nodes_changed: usize,
    pub estimated_nodes_visited: usize,
    pub estimated_frontier_width: usize,
    pub estimated_rendered_bytes: usize,
    pub estimated_rendered_tokens: u32,
    pub estimated_export_growth: isize,
    pub explanation: String,
}
