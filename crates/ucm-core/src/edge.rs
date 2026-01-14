//! Edge types and relationships between blocks.
//!
//! Edges represent explicit relationships between blocks, such as
//! derivation, references, and semantic connections.

use crate::id::BlockId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;
use std::str::FromStr;

/// An edge represents an explicit relationship between blocks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    /// Type of relationship
    pub edge_type: EdgeType,
    /// Target block
    pub target: BlockId,
    /// Edge-specific metadata
    #[serde(default, skip_serializing_if = "EdgeMetadata::is_empty")]
    pub metadata: EdgeMetadata,
    /// When the edge was created
    pub created_at: DateTime<Utc>,
}

impl Edge {
    /// Create a new edge
    pub fn new(edge_type: EdgeType, target: BlockId) -> Self {
        Self {
            edge_type,
            target,
            metadata: EdgeMetadata::default(),
            created_at: Utc::now(),
        }
    }

    /// Add metadata to the edge
    pub fn with_metadata(mut self, metadata: EdgeMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Add confidence score
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.metadata.confidence = Some(confidence);
        self
    }

    /// Add description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.metadata.description = Some(description.into());
        self
    }
}

/// Types of relationships between blocks
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeType {
    // Derivation relationships
    /// This block was created from another
    DerivedFrom,
    /// This block replaces another
    Supersedes,
    /// This block is a transformation of another
    TransformedFrom,

    // Reference relationships
    /// This block references another
    References,
    /// Inverse of References (auto-maintained)
    CitedBy,
    /// Hyperlink relationship
    LinksTo,

    // Semantic relationships
    /// This block provides evidence for another
    Supports,
    /// This block contradicts another
    Contradicts,
    /// This block elaborates on another
    Elaborates,
    /// This block summarizes another
    Summarizes,

    // Structural relationships (auto-maintained from document structure)
    /// Structural parent
    ParentOf,
    /// Structural child
    ChildOf,
    /// Same parent
    SiblingOf,
    /// Immediate previous sibling
    PreviousSibling,
    /// Immediate next sibling
    NextSibling,

    // Version relationships
    /// Different version of same logical content
    VersionOf,
    /// Alternative representation
    AlternativeOf,
    /// Translation to different language
    TranslationOf,

    // Custom relationship
    Custom(String),
}

impl EdgeType {
    /// Get the inverse edge type (if applicable)
    pub fn inverse(&self) -> Option<EdgeType> {
        match self {
            EdgeType::References => Some(EdgeType::CitedBy),
            EdgeType::CitedBy => Some(EdgeType::References),
            EdgeType::DerivedFrom => None, // No automatic inverse
            EdgeType::Supersedes => None,
            EdgeType::ParentOf => Some(EdgeType::ChildOf),
            EdgeType::ChildOf => Some(EdgeType::ParentOf),
            EdgeType::PreviousSibling => Some(EdgeType::NextSibling),
            EdgeType::NextSibling => Some(EdgeType::PreviousSibling),
            EdgeType::Supports => None,
            EdgeType::Contradicts => Some(EdgeType::Contradicts), // Symmetric
            EdgeType::SiblingOf => Some(EdgeType::SiblingOf),     // Symmetric
            _ => None,
        }
    }

    /// Check if this edge type is symmetric
    pub fn is_symmetric(&self) -> bool {
        matches!(self, EdgeType::Contradicts | EdgeType::SiblingOf)
    }

    /// Check if this is a structural edge (auto-maintained)
    pub fn is_structural(&self) -> bool {
        matches!(
            self,
            EdgeType::ParentOf
                | EdgeType::ChildOf
                | EdgeType::SiblingOf
                | EdgeType::PreviousSibling
                | EdgeType::NextSibling
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeTypeParseError(pub String);

impl fmt::Display for EdgeTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown edge type '{}'", self.0)
    }
}

impl StdError for EdgeTypeParseError {}

impl FromStr for EdgeType {
    type Err = EdgeTypeParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "derived_from" => Ok(EdgeType::DerivedFrom),
            "supersedes" => Ok(EdgeType::Supersedes),
            "transformed_from" => Ok(EdgeType::TransformedFrom),
            "references" => Ok(EdgeType::References),
            "cited_by" => Ok(EdgeType::CitedBy),
            "links_to" => Ok(EdgeType::LinksTo),
            "supports" => Ok(EdgeType::Supports),
            "contradicts" => Ok(EdgeType::Contradicts),
            "elaborates" => Ok(EdgeType::Elaborates),
            "summarizes" => Ok(EdgeType::Summarizes),
            "parent_of" => Ok(EdgeType::ParentOf),
            "child_of" => Ok(EdgeType::ChildOf),
            "sibling_of" => Ok(EdgeType::SiblingOf),
            "previous_sibling" => Ok(EdgeType::PreviousSibling),
            "next_sibling" => Ok(EdgeType::NextSibling),
            "version_of" => Ok(EdgeType::VersionOf),
            "alternative_of" => Ok(EdgeType::AlternativeOf),
            "translation_of" => Ok(EdgeType::TranslationOf),
            s if s.starts_with("custom:") => Ok(EdgeType::Custom(
                s.strip_prefix("custom:").unwrap().to_string(),
            )),
            _ => Err(EdgeTypeParseError(s.to_string())),
        }
    }
}

impl EdgeType {
    /// Convert to string
    pub fn as_str(&self) -> String {
        match self {
            EdgeType::DerivedFrom => "derived_from".to_string(),
            EdgeType::Supersedes => "supersedes".to_string(),
            EdgeType::TransformedFrom => "transformed_from".to_string(),
            EdgeType::References => "references".to_string(),
            EdgeType::CitedBy => "cited_by".to_string(),
            EdgeType::LinksTo => "links_to".to_string(),
            EdgeType::Supports => "supports".to_string(),
            EdgeType::Contradicts => "contradicts".to_string(),
            EdgeType::Elaborates => "elaborates".to_string(),
            EdgeType::Summarizes => "summarizes".to_string(),
            EdgeType::ParentOf => "parent_of".to_string(),
            EdgeType::ChildOf => "child_of".to_string(),
            EdgeType::SiblingOf => "sibling_of".to_string(),
            EdgeType::PreviousSibling => "previous_sibling".to_string(),
            EdgeType::NextSibling => "next_sibling".to_string(),
            EdgeType::VersionOf => "version_of".to_string(),
            EdgeType::AlternativeOf => "alternative_of".to_string(),
            EdgeType::TranslationOf => "translation_of".to_string(),
            EdgeType::Custom(name) => format!("custom:{}", name),
        }
    }
}

/// Edge metadata
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EdgeMetadata {
    /// Confidence score (0.0 - 1.0) for inferred relationships
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// Human-readable description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Custom key-value pairs
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub custom: HashMap<String, serde_json::Value>,
}

impl EdgeMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.confidence.is_none() && self.description.is_none() && self.custom.is_empty()
    }

    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = Some(confidence.clamp(0.0, 1.0));
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_custom(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.custom.insert(key.into(), value);
        self
    }
}

/// Bidirectional edge index for efficient traversal
#[derive(Debug, Clone, Default)]
pub struct EdgeIndex {
    /// Outgoing edges: source -> [(type, target)]
    outgoing: HashMap<BlockId, Vec<(EdgeType, BlockId)>>,
    /// Incoming edges: target -> [(type, source)]
    incoming: HashMap<BlockId, Vec<(EdgeType, BlockId)>>,
}

impl EdgeIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an edge to the index
    pub fn add_edge(&mut self, source: &BlockId, edge: &Edge) {
        // Add to outgoing
        self.outgoing
            .entry(*source)
            .or_default()
            .push((edge.edge_type.clone(), edge.target));

        // Auto-maintain inverse edge in incoming index
        if let Some(inv) = edge.edge_type.inverse() {
            self.incoming
                .entry(edge.target)
                .or_default()
                .push((inv, *source));
        } else {
            // Even without inverse, track in incoming for traversal
            self.incoming
                .entry(edge.target)
                .or_default()
                .push((edge.edge_type.clone(), *source));
        }
    }

    /// Remove an edge from the index
    pub fn remove_edge(&mut self, source: &BlockId, target: &BlockId, edge_type: &EdgeType) {
        if let Some(edges) = self.outgoing.get_mut(source) {
            edges.retain(|(t, tgt)| !(t == edge_type && tgt == target));
            if edges.is_empty() {
                self.outgoing.remove(source);
            }
        }

        let incoming_type = edge_type.inverse().unwrap_or_else(|| edge_type.clone());
        if let Some(edges) = self.incoming.get_mut(target) {
            edges.retain(|(t, src)| !(t == &incoming_type && src == source));
            if edges.is_empty() {
                self.incoming.remove(target);
            }
        }
    }

    /// Remove all edges involving a block
    pub fn remove_block(&mut self, block_id: &BlockId) {
        // Remove outgoing edges
        if let Some(edges) = self.outgoing.remove(block_id) {
            for (edge_type, target) in edges {
                let _incoming_type = edge_type.inverse().unwrap_or(edge_type);
                if let Some(incoming) = self.incoming.get_mut(&target) {
                    incoming.retain(|(_, src)| src != block_id);
                }
            }
        }

        // Remove incoming edges
        if let Some(edges) = self.incoming.remove(block_id) {
            for (_, source) in edges {
                if let Some(outgoing) = self.outgoing.get_mut(&source) {
                    outgoing.retain(|(_, tgt)| tgt != block_id);
                }
            }
        }
    }

    /// Get all outgoing edges from a block
    pub fn outgoing_from(&self, source: &BlockId) -> &[(EdgeType, BlockId)] {
        self.outgoing
            .get(source)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get all incoming edges to a block
    pub fn incoming_to(&self, target: &BlockId) -> &[(EdgeType, BlockId)] {
        self.incoming
            .get(target)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get all edges of a specific type from a source
    pub fn outgoing_of_type(&self, source: &BlockId, edge_type: &EdgeType) -> Vec<BlockId> {
        self.outgoing
            .get(source)
            .map(|edges| {
                edges
                    .iter()
                    .filter(|(t, _)| t == edge_type)
                    .map(|(_, tgt)| *tgt)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all edges of a specific type to a target
    pub fn incoming_of_type(&self, target: &BlockId, edge_type: &EdgeType) -> Vec<BlockId> {
        self.incoming
            .get(target)
            .map(|edges| {
                edges
                    .iter()
                    .filter(|(t, _)| t == edge_type)
                    .map(|(_, src)| *src)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Check if an edge exists
    pub fn has_edge(&self, source: &BlockId, target: &BlockId, edge_type: &EdgeType) -> bool {
        self.outgoing
            .get(source)
            .map(|edges| edges.iter().any(|(t, tgt)| t == edge_type && tgt == target))
            .unwrap_or(false)
    }

    /// Get total edge count
    pub fn edge_count(&self) -> usize {
        self.outgoing.values().map(|v| v.len()).sum()
    }

    /// Clear all edges
    pub fn clear(&mut self) {
        self.outgoing.clear();
        self.incoming.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_id(n: u8) -> BlockId {
        BlockId::from_bytes([n; 12])
    }

    #[test]
    fn test_edge_creation() {
        let edge = Edge::new(EdgeType::References, make_id(2))
            .with_confidence(0.95)
            .with_description("Important reference");

        assert_eq!(edge.edge_type, EdgeType::References);
        assert_eq!(edge.metadata.confidence, Some(0.95));
    }

    #[test]
    fn test_edge_type_inverse() {
        assert_eq!(EdgeType::References.inverse(), Some(EdgeType::CitedBy));
        assert_eq!(EdgeType::ParentOf.inverse(), Some(EdgeType::ChildOf));
        assert_eq!(EdgeType::DerivedFrom.inverse(), None);
    }

    #[test]
    fn test_edge_type_parse() {
        assert_eq!(
            EdgeType::from_str("references").unwrap(),
            EdgeType::References
        );
        assert_eq!(
            EdgeType::from_str("custom:my_type").unwrap(),
            EdgeType::Custom("my_type".to_string())
        );
    }

    #[test]
    fn test_edge_index_add_remove() {
        let mut index = EdgeIndex::new();
        let source = make_id(1);
        let target = make_id(2);
        let edge = Edge::new(EdgeType::References, target);

        index.add_edge(&source, &edge);
        assert!(index.has_edge(&source, &target, &EdgeType::References));
        assert_eq!(index.edge_count(), 1);

        index.remove_edge(&source, &target, &EdgeType::References);
        assert!(!index.has_edge(&source, &target, &EdgeType::References));
    }

    #[test]
    fn test_edge_index_traversal() {
        let mut index = EdgeIndex::new();
        let a = make_id(1);
        let b = make_id(2);
        let c = make_id(3);

        index.add_edge(&a, &Edge::new(EdgeType::References, b));
        index.add_edge(&a, &Edge::new(EdgeType::References, c));
        index.add_edge(&b, &Edge::new(EdgeType::Supports, c));

        let refs = index.outgoing_of_type(&a, &EdgeType::References);
        assert_eq!(refs.len(), 2);

        let incoming = index.incoming_to(&c);
        assert_eq!(incoming.len(), 2);
    }

    #[test]
    fn test_edge_index_remove_block() {
        let mut index = EdgeIndex::new();
        let a = make_id(1);
        let b = make_id(2);
        let c = make_id(3);

        index.add_edge(&a, &Edge::new(EdgeType::References, b));
        index.add_edge(&b, &Edge::new(EdgeType::References, c));

        index.remove_block(&b);

        assert!(!index.has_edge(&a, &b, &EdgeType::References));
        assert!(!index.has_edge(&b, &c, &EdgeType::References));
    }
}
