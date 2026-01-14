//! Block - the fundamental unit of content in UCM.

use crate::content::Content;
use crate::edge::Edge;
use crate::id::{compute_content_hash, generate_block_id, BlockId};
use crate::metadata::BlockMetadata;
use crate::version::Version;
use serde::{Deserialize, Serialize};

/// A block is the fundamental unit of content in UCM.
///
/// Blocks are immutable, content-addressed units identified by deterministic IDs
/// derived from their content. They contain typed content, metadata for search
/// and display, and edges representing relationships to other blocks.
///
/// # Example
/// ```
/// use ucm_core::{Block, Content};
///
/// let block = Block::new(Content::text("Hello, UCM!"), Some("intro"));
/// println!("Block ID: {}", block.id);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    /// Unique, content-derived identifier
    pub id: BlockId,

    /// The actual content
    pub content: Content,

    /// Block metadata
    pub metadata: BlockMetadata,

    /// Explicit relationships to other blocks
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edges: Vec<Edge>,

    /// Version for optimistic concurrency control
    pub version: Version,
}

impl Block {
    /// Create a new block with generated ID
    pub fn new(content: Content, semantic_role: Option<&str>) -> Self {
        let id = generate_block_id(&content, semantic_role, None);
        let content_hash = compute_content_hash(&content);
        let mut metadata = BlockMetadata::new(content_hash);

        if let Some(role) = semantic_role {
            if let Some(parsed_role) = crate::metadata::SemanticRole::parse(role) {
                metadata.semantic_role = Some(parsed_role);
            }
        }

        Self {
            id,
            content,
            metadata,
            edges: Vec::new(),
            version: Version::initial(),
        }
    }

    /// Create a new block with a specific ID (for deserialization or testing)
    pub fn with_id(id: BlockId, content: Content) -> Self {
        let content_hash = compute_content_hash(&content);
        Self {
            id,
            content,
            metadata: BlockMetadata::new(content_hash),
            edges: Vec::new(),
            version: Version::initial(),
        }
    }

    /// Create a root block
    pub fn root() -> Self {
        Self {
            id: BlockId::root(),
            content: Content::text(""),
            metadata: BlockMetadata::default(),
            edges: Vec::new(),
            version: Version::initial(),
        }
    }

    /// Set metadata
    pub fn with_metadata(mut self, metadata: BlockMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Set label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.metadata.label = Some(label.into());
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.metadata.tags.push(tag.into());
        self
    }

    /// Add an edge
    pub fn with_edge(mut self, edge: Edge) -> Self {
        self.edges.push(edge);
        self
    }

    /// Add multiple edges
    pub fn with_edges(mut self, edges: impl IntoIterator<Item = Edge>) -> Self {
        self.edges.extend(edges);
        self
    }

    /// Get the content type tag
    pub fn content_type(&self) -> &'static str {
        self.content.type_tag()
    }

    /// Check if the block is a root block
    pub fn is_root(&self) -> bool {
        self.id.is_root()
    }

    /// Get the estimated token count
    pub fn token_estimate(&self) -> crate::metadata::TokenEstimate {
        self.metadata
            .token_estimate
            .unwrap_or_else(|| crate::metadata::TokenEstimate::compute(&self.content))
    }

    /// Get content size in bytes
    pub fn size_bytes(&self) -> usize {
        self.content.size_bytes()
    }

    /// Update the content and regenerate ID
    pub fn update_content(&mut self, content: Content, semantic_role: Option<&str>) {
        self.content = content;
        self.id = generate_block_id(&self.content, semantic_role, None);
        self.metadata.content_hash = compute_content_hash(&self.content);
        self.metadata.touch();
        self.version.increment();
    }

    /// Add an edge to this block
    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
        self.version.increment();
    }

    /// Remove an edge by target and type
    pub fn remove_edge(&mut self, target: &BlockId, edge_type: &crate::edge::EdgeType) -> bool {
        let len_before = self.edges.len();
        self.edges
            .retain(|e| !(&e.target == target && &e.edge_type == edge_type));
        let removed = self.edges.len() < len_before;
        if removed {
            self.version.increment();
        }
        removed
    }

    /// Get edges of a specific type
    pub fn edges_of_type(&self, edge_type: &crate::edge::EdgeType) -> Vec<&Edge> {
        self.edges
            .iter()
            .filter(|e| &e.edge_type == edge_type)
            .collect()
    }

    /// Check if block has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.metadata.has_tag(tag)
    }
}

/// Block lifecycle state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockState {
    /// Reachable from document root
    Live,
    /// Not reachable from root but not deleted
    Orphaned,
    /// Marked for garbage collection
    Deleted,
}

impl std::fmt::Display for BlockState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockState::Live => write!(f, "live"),
            BlockState::Orphaned => write!(f, "orphaned"),
            BlockState::Deleted => write!(f, "deleted"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::edge::EdgeType;

    #[test]
    fn test_block_creation() {
        let block = Block::new(Content::text("Hello, world!"), Some("intro"));

        assert!(!block.id.is_root());
        assert_eq!(block.content_type(), "text");
        assert!(block.edges.is_empty());
    }

    #[test]
    fn test_deterministic_id() {
        let block1 = Block::new(Content::text("Hello"), Some("intro"));
        let block2 = Block::new(Content::text("Hello"), Some("intro"));

        assert_eq!(block1.id, block2.id);
    }

    #[test]
    fn test_different_role_different_id() {
        let block1 = Block::new(Content::text("Hello"), Some("intro"));
        let block2 = Block::new(Content::text("Hello"), Some("conclusion"));

        assert_ne!(block1.id, block2.id);
    }

    #[test]
    fn test_root_block() {
        let root = Block::root();
        assert!(root.is_root());
    }

    #[test]
    fn test_block_builder() {
        let block = Block::new(Content::text("Test"), None)
            .with_label("Test Block")
            .with_tag("important")
            .with_tag("draft");

        assert_eq!(block.metadata.label, Some("Test Block".to_string()));
        assert!(block.has_tag("important"));
        assert!(block.has_tag("draft"));
    }

    #[test]
    fn test_block_edges() {
        let target_id = BlockId::from_bytes([1u8; 12]);
        let edge = Edge::new(EdgeType::References, target_id);

        let mut block = Block::new(Content::text("Test"), None);
        block.add_edge(edge);

        assert_eq!(block.edges.len(), 1);
        assert_eq!(block.edges_of_type(&EdgeType::References).len(), 1);

        block.remove_edge(&target_id, &EdgeType::References);
        assert!(block.edges.is_empty());
    }

    #[test]
    fn test_update_content() {
        let mut block = Block::new(Content::text("Original"), Some("intro"));
        let original_id = block.id;
        let original_version = block.version.counter;

        block.update_content(Content::text("Updated"), Some("intro"));

        assert_ne!(block.id, original_id);
        assert!(block.version.counter > original_version);
    }
}
