use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphNeighborMode {
    Children,
    Parents,
    Outgoing,
    Incoming,
    Neighborhood,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GraphFindQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_regex: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_role_regex: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_regex: Option<String>,
    #[serde(default)]
    pub case_sensitive: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}
