//! Document - a collection of blocks with hierarchical structure.

use crate::block::{Block, BlockState};
use crate::edge::EdgeIndex;
use crate::error::{Error, ErrorCode, Result, ValidationIssue};
use crate::id::BlockId;
use crate::metadata::TokenModel;
use crate::version::DocumentVersion;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Document identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentId(pub String);

impl DocumentId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn generate() -> Self {
        // Use chrono for WASM compatibility (chrono supports wasmbind feature)
        let ts = Utc::now().timestamp_nanos_opt().unwrap_or(0);
        Self(format!("doc_{:x}", ts))
    }
}

impl std::fmt::Display for DocumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Document metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DocumentMetadata {
    /// Document title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Document description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Authors
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<String>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,

    /// Language (ISO 639-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Custom metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub custom: HashMap<String, serde_json::Value>,
}

impl DocumentMetadata {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            modified_at: now,
            ..Default::default()
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn touch(&mut self) {
        self.modified_at = Utc::now();
    }
}

/// Secondary indices for fast lookup
#[derive(Debug, Clone, Default)]
pub struct DocumentIndices {
    /// Blocks by tag
    pub by_tag: HashMap<String, HashSet<BlockId>>,
    /// Blocks by semantic role category
    pub by_role: HashMap<String, HashSet<BlockId>>,
    /// Blocks by content type
    pub by_content_type: HashMap<String, HashSet<BlockId>>,
    /// Blocks by label
    pub by_label: HashMap<String, BlockId>,
}

impl DocumentIndices {
    pub fn new() -> Self {
        Self::default()
    }

    /// Index a block
    pub fn index_block(&mut self, block: &Block) {
        let id = &block.id;

        // Index by tags
        for tag in &block.metadata.tags {
            self.by_tag.entry(tag.clone()).or_default().insert(*id);
        }

        // Index by semantic role
        if let Some(role) = &block.metadata.semantic_role {
            self.by_role
                .entry(role.category.as_str().to_string())
                .or_default()
                .insert(*id);
        }

        // Index by content type
        self.by_content_type
            .entry(block.content_type().to_string())
            .or_default()
            .insert(*id);

        // Index by label
        if let Some(label) = &block.metadata.label {
            self.by_label.insert(label.clone(), *id);
        }
    }

    /// Remove a block from indices
    pub fn remove_block(&mut self, block: &Block) {
        let id = &block.id;

        for tag in &block.metadata.tags {
            if let Some(set) = self.by_tag.get_mut(tag) {
                set.remove(id);
            }
        }

        if let Some(role) = &block.metadata.semantic_role {
            if let Some(set) = self.by_role.get_mut(role.category.as_str()) {
                set.remove(id);
            }
        }

        if let Some(set) = self.by_content_type.get_mut(block.content_type()) {
            set.remove(id);
        }

        if let Some(label) = &block.metadata.label {
            self.by_label.remove(label);
        }
    }

    /// Rebuild all indices from blocks
    pub fn rebuild(&mut self, blocks: &HashMap<BlockId, Block>) {
        self.by_tag.clear();
        self.by_role.clear();
        self.by_content_type.clear();
        self.by_label.clear();

        for block in blocks.values() {
            self.index_block(block);
        }
    }

    /// Find blocks by tag
    pub fn find_by_tag(&self, tag: &str) -> HashSet<BlockId> {
        self.by_tag.get(tag).cloned().unwrap_or_default()
    }

    /// Find blocks by content type
    pub fn find_by_type(&self, content_type: &str) -> HashSet<BlockId> {
        self.by_content_type
            .get(content_type)
            .cloned()
            .unwrap_or_default()
    }

    /// Find block by label
    pub fn find_by_label(&self, label: &str) -> Option<BlockId> {
        self.by_label.get(label).cloned()
    }
}

/// A UCM document is a collection of blocks with hierarchical structure.
#[derive(Debug, Clone)]
pub struct Document {
    /// Document identifier
    pub id: DocumentId,

    /// Root block ID
    pub root: BlockId,

    /// Adjacency map: parent -> ordered children
    pub structure: HashMap<BlockId, Vec<BlockId>>,

    /// All blocks in the document
    pub blocks: HashMap<BlockId, Block>,

    /// Document-level metadata
    pub metadata: DocumentMetadata,

    /// Secondary indices for fast lookup
    pub indices: DocumentIndices,

    /// Edge index for relationship traversal
    pub edge_index: EdgeIndex,

    /// Document version for concurrency control
    pub version: DocumentVersion,
}

impl Document {
    /// Create a new empty document
    pub fn new(id: DocumentId) -> Self {
        let root = Block::root();
        let root_id = root.id;

        let mut blocks = HashMap::new();
        blocks.insert(root_id, root);

        Self {
            id,
            root: root_id,
            structure: HashMap::new(),
            blocks,
            metadata: DocumentMetadata::new(),
            indices: DocumentIndices::new(),
            edge_index: EdgeIndex::new(),
            version: DocumentVersion::initial(),
        }
    }

    /// Create with a generated ID
    pub fn create() -> Self {
        Self::new(DocumentId::generate())
    }

    /// Set document metadata
    pub fn with_metadata(mut self, metadata: DocumentMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Get a block by ID
    pub fn get_block(&self, id: &BlockId) -> Option<&Block> {
        self.blocks.get(id)
    }

    /// Get a mutable block by ID
    pub fn get_block_mut(&mut self, id: &BlockId) -> Option<&mut Block> {
        self.blocks.get_mut(id)
    }

    /// Get children of a block
    pub fn children(&self, parent: &BlockId) -> &[BlockId] {
        self.structure
            .get(parent)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get parent of a block
    pub fn parent(&self, child: &BlockId) -> Option<&BlockId> {
        for (parent, children) in &self.structure {
            if children.contains(child) {
                return Some(parent);
            }
        }
        None
    }

    /// Get the parent block (convenience method)
    pub fn parent_of(&self, child: &BlockId) -> Option<&Block> {
        self.parent(child).and_then(|id| self.blocks.get(id))
    }

    /// Add a block to the document
    pub fn add_block(&mut self, block: Block, parent: &BlockId) -> Result<BlockId> {
        if !self.blocks.contains_key(parent) {
            return Err(Error::BlockNotFound(parent.to_string()));
        }

        let id = block.id;

        // Index edges
        for edge in &block.edges {
            self.edge_index.add_edge(&id, edge);
        }

        // Index block
        self.indices.index_block(&block);

        // Add to blocks
        self.blocks.insert(id, block);

        // Add to structure
        self.structure.entry(*parent).or_default().push(id);

        self.touch();
        Ok(id)
    }

    /// Add a block at a specific position
    pub fn add_block_at(
        &mut self,
        block: Block,
        parent: &BlockId,
        index: usize,
    ) -> Result<BlockId> {
        if !self.blocks.contains_key(parent) {
            return Err(Error::BlockNotFound(parent.to_string()));
        }

        let id = block.id;

        for edge in &block.edges {
            self.edge_index.add_edge(&id, edge);
        }

        self.indices.index_block(&block);
        self.blocks.insert(id, block);

        let children = self.structure.entry(*parent).or_default();
        let insert_idx = index.min(children.len());
        children.insert(insert_idx, id);

        self.touch();
        Ok(id)
    }

    /// Add an edge between two blocks (wrapper for edge_index)
    pub fn add_edge(
        &mut self,
        source: &BlockId,
        edge_type: crate::edge::EdgeType,
        target: BlockId,
    ) {
        let edge = crate::edge::Edge::new(edge_type, target);
        self.edge_index.add_edge(source, &edge);
    }

    /// Remove a block from the structure (makes it orphaned)
    pub fn remove_from_structure(&mut self, id: &BlockId) -> bool {
        let mut removed = false;
        for children in self.structure.values_mut() {
            let len_before = children.len();
            children.retain(|c| c != id);
            if children.len() < len_before {
                removed = true;
            }
        }
        if removed {
            self.touch();
        }
        removed
    }

    /// Delete a block completely
    pub fn delete_block(&mut self, id: &BlockId) -> Result<Block> {
        // Remove from structure
        self.remove_from_structure(id);

        // Remove children structure
        self.structure.remove(id);

        // Remove from edge index
        self.edge_index.remove_block(id);

        // Remove and return block
        let block = self
            .blocks
            .remove(id)
            .ok_or_else(|| Error::BlockNotFound(id.to_string()))?;

        // Remove from indices
        self.indices.remove_block(&block);

        self.touch();
        Ok(block)
    }

    /// Delete a block and all its descendants
    pub fn delete_cascade(&mut self, id: &BlockId) -> Result<Vec<Block>> {
        let descendants = self.descendants(id);
        let mut deleted = Vec::new();

        // Delete in reverse order (children first)
        for desc_id in descendants.into_iter().rev() {
            if let Ok(block) = self.delete_block(&desc_id) {
                deleted.push(block);
            }
        }

        if let Ok(block) = self.delete_block(id) {
            deleted.push(block);
        }

        Ok(deleted)
    }

    /// Move a block to a new parent
    pub fn move_block(&mut self, id: &BlockId, new_parent: &BlockId) -> Result<()> {
        if !self.blocks.contains_key(id) {
            return Err(Error::BlockNotFound(id.to_string()));
        }
        if !self.blocks.contains_key(new_parent) {
            return Err(Error::BlockNotFound(new_parent.to_string()));
        }

        // Check for cycle
        if self.is_ancestor(id, new_parent) {
            return Err(Error::CycleDetected(id.to_string()));
        }

        self.remove_from_structure(id);
        self.structure.entry(*new_parent).or_default().push(*id);

        self.touch();
        Ok(())
    }

    /// Move a block to a specific position under a parent
    pub fn move_block_at(
        &mut self,
        id: &BlockId,
        new_parent: &BlockId,
        index: usize,
    ) -> Result<()> {
        if !self.blocks.contains_key(id) {
            return Err(Error::BlockNotFound(id.to_string()));
        }
        if !self.blocks.contains_key(new_parent) {
            return Err(Error::BlockNotFound(new_parent.to_string()));
        }

        if self.is_ancestor(id, new_parent) {
            return Err(Error::CycleDetected(id.to_string()));
        }

        self.remove_from_structure(id);
        let children = self.structure.entry(*new_parent).or_default();
        let insert_idx = index.min(children.len());
        children.insert(insert_idx, *id);

        self.touch();
        Ok(())
    }

    /// Move a block before another block (sibling ordering)
    pub fn move_block_before(&mut self, id: &BlockId, before: &BlockId) -> Result<()> {
        if !self.blocks.contains_key(id) {
            return Err(Error::BlockNotFound(id.to_string()));
        }
        if !self.blocks.contains_key(before) {
            return Err(Error::BlockNotFound(before.to_string()));
        }

        let parent = *self
            .parent(before)
            .ok_or_else(|| Error::BlockNotFound(format!("parent of {}", before)))?;

        if self.is_ancestor(id, &parent) {
            return Err(Error::CycleDetected(id.to_string()));
        }

        self.remove_from_structure(id);
        let children = self.structure.entry(parent).or_default();

        if let Some(pos) = children.iter().position(|child| child == before) {
            children.insert(pos, *id);
        } else {
            children.push(*id);
        }

        self.touch();
        Ok(())
    }

    /// Move a block after another block (sibling ordering)
    pub fn move_block_after(&mut self, id: &BlockId, after: &BlockId) -> Result<()> {
        if !self.blocks.contains_key(id) {
            return Err(Error::BlockNotFound(id.to_string()));
        }
        if !self.blocks.contains_key(after) {
            return Err(Error::BlockNotFound(after.to_string()));
        }

        let parent = *self
            .parent(after)
            .ok_or_else(|| Error::BlockNotFound(format!("parent of {}", after)))?;

        if self.is_ancestor(id, &parent) {
            return Err(Error::CycleDetected(id.to_string()));
        }

        self.remove_from_structure(id);
        let children = self.structure.entry(parent).or_default();

        if let Some(pos) = children.iter().position(|child| child == after) {
            children.insert(pos + 1, *id);
        } else {
            children.push(*id);
        }

        self.touch();
        Ok(())
    }

    /// Check if a block is an ancestor of another
    pub fn is_ancestor(&self, potential_ancestor: &BlockId, block: &BlockId) -> bool {
        let mut current = Some(*block);
        while let Some(id) = current {
            if &id == potential_ancestor {
                return true;
            }
            current = self.parent(&id).cloned();
        }
        false
    }

    /// Get all descendants of a block
    pub fn descendants(&self, id: &BlockId) -> Vec<BlockId> {
        let mut result = Vec::new();
        let mut stack = vec![*id];

        while let Some(current) = stack.pop() {
            if let Some(children) = self.structure.get(&current) {
                for child in children {
                    result.push(*child);
                    stack.push(*child);
                }
            }
        }

        result
    }

    /// Check if a block is reachable from root
    pub fn is_reachable(&self, id: &BlockId) -> bool {
        if id == &self.root {
            return true;
        }

        let mut visited = HashSet::new();
        let mut stack = vec![self.root];

        while let Some(current) = stack.pop() {
            if &current == id {
                return true;
            }
            if visited.insert(current) {
                if let Some(children) = self.structure.get(&current) {
                    stack.extend(children.iter().cloned());
                }
            }
        }

        false
    }

    /// Find all orphaned blocks
    pub fn find_orphans(&self) -> Vec<BlockId> {
        let mut reachable = HashSet::new();
        let mut stack = vec![self.root];

        while let Some(current) = stack.pop() {
            if reachable.insert(current) {
                if let Some(children) = self.structure.get(&current) {
                    stack.extend(children.iter().cloned());
                }
            }
        }

        self.blocks
            .keys()
            .filter(|id| !reachable.contains(*id))
            .cloned()
            .collect()
    }

    /// Get block state
    pub fn block_state(&self, id: &BlockId) -> BlockState {
        if !self.blocks.contains_key(id) {
            BlockState::Deleted
        } else if self.is_reachable(id) {
            BlockState::Live
        } else {
            BlockState::Orphaned
        }
    }

    /// Prune unreachable blocks
    pub fn prune_unreachable(&mut self) -> Vec<Block> {
        let orphans = self.find_orphans();
        let mut pruned = Vec::new();

        for id in orphans {
            if let Ok(block) = self.delete_block(&id) {
                pruned.push(block);
            }
        }

        pruned
    }

    /// Get total block count
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Get total token estimate
    pub fn total_tokens(&self, model: TokenModel) -> u32 {
        self.blocks
            .values()
            .map(|b| b.token_estimate().for_model(model))
            .sum()
    }

    /// Validate document structure
    pub fn validate(&self) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Check for orphans
        let orphans = self.find_orphans();
        for orphan in orphans {
            issues.push(ValidationIssue::warning(
                ErrorCode::E203OrphanedBlock,
                format!("Block {} is unreachable from root", orphan),
            ));
        }

        // Check for cycles
        if self.has_cycles() {
            issues.push(ValidationIssue::error(
                ErrorCode::E201CycleDetected,
                "Document structure contains a cycle",
            ));
        }

        // Check for dangling references
        for block in self.blocks.values() {
            for edge in &block.edges {
                if !self.blocks.contains_key(&edge.target) {
                    issues.push(ValidationIssue::error(
                        ErrorCode::E001BlockNotFound,
                        format!(
                            "Block {} references non-existent block {}",
                            block.id, edge.target
                        ),
                    ));
                }
            }
        }

        issues
    }

    /// Check for cycles in structure
    fn has_cycles(&self) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        fn dfs(
            node: &BlockId,
            structure: &HashMap<BlockId, Vec<BlockId>>,
            visited: &mut HashSet<BlockId>,
            rec_stack: &mut HashSet<BlockId>,
        ) -> bool {
            visited.insert(*node);
            rec_stack.insert(*node);

            if let Some(children) = structure.get(node) {
                for child in children {
                    if !visited.contains(child) {
                        if dfs(child, structure, visited, rec_stack) {
                            return true;
                        }
                    } else if rec_stack.contains(child) {
                        return true;
                    }
                }
            }

            rec_stack.remove(node);
            false
        }

        dfs(&self.root, &self.structure, &mut visited, &mut rec_stack)
    }

    /// Touch document (update modified timestamp and version)
    fn touch(&mut self) {
        self.metadata.touch();
        self.version.increment([0u8; 8]); // TODO: compute actual state hash
    }

    /// Rebuild all indices
    pub fn rebuild_indices(&mut self) {
        self.indices.rebuild(&self.blocks);

        self.edge_index.clear();
        for block in self.blocks.values() {
            for edge in &block.edges {
                self.edge_index.add_edge(&block.id, edge);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::Content;

    #[test]
    fn test_document_creation() {
        let doc = Document::create();
        assert_eq!(doc.block_count(), 1); // Just root
        assert!(doc.blocks.contains_key(&doc.root));
    }

    #[test]
    fn test_add_block() {
        let mut doc = Document::create();
        let block = Block::new(Content::text("Hello"), Some("intro"));
        let root = doc.root;

        let id = doc.add_block(block, &root).unwrap();
        assert_eq!(doc.block_count(), 2);
        assert!(doc.is_reachable(&id));
    }

    #[test]
    fn test_move_block() {
        let mut doc = Document::create();
        let root = doc.root;

        let parent1 = doc
            .add_block(Block::new(Content::text("Parent 1"), None), &root)
            .unwrap();
        let parent2 = doc
            .add_block(Block::new(Content::text("Parent 2"), None), &root)
            .unwrap();
        let child = doc
            .add_block(Block::new(Content::text("Child"), None), &parent1)
            .unwrap();

        assert!(doc.children(&parent1).contains(&child));

        doc.move_block(&child, &parent2).unwrap();

        assert!(!doc.children(&parent1).contains(&child));
        assert!(doc.children(&parent2).contains(&child));
    }

    #[test]
    fn test_cycle_detection() {
        let mut doc = Document::create();
        let root = doc.root;

        let a = doc
            .add_block(Block::new(Content::text("A"), None), &root)
            .unwrap();
        let b = doc
            .add_block(Block::new(Content::text("B"), None), &a)
            .unwrap();

        // Try to move A under B (would create cycle)
        let result = doc.move_block(&a, &b);
        assert!(result.is_err());
    }

    #[test]
    fn test_orphan_detection() {
        let mut doc = Document::create();
        let root = doc.root;

        let block = Block::new(Content::text("Test"), None);
        let id = doc.add_block(block, &root).unwrap();

        assert!(doc.find_orphans().is_empty());

        doc.remove_from_structure(&id);
        assert_eq!(doc.find_orphans(), vec![id]);
    }

    #[test]
    fn test_cascade_delete() {
        let mut doc = Document::create();
        let root = doc.root;

        let parent = doc
            .add_block(Block::new(Content::text("Parent"), None), &root)
            .unwrap();
        let _child1 = doc
            .add_block(Block::new(Content::text("Child 1"), None), &parent)
            .unwrap();
        let _child2 = doc
            .add_block(Block::new(Content::text("Child 2"), None), &parent)
            .unwrap();

        assert_eq!(doc.block_count(), 4);

        let deleted = doc.delete_cascade(&parent).unwrap();
        assert_eq!(deleted.len(), 3); // parent + 2 children
        assert_eq!(doc.block_count(), 1); // just root
    }

    #[test]
    fn test_indices() {
        let mut doc = Document::create();
        let root = doc.root;

        let block = Block::new(Content::text("Test"), None)
            .with_tag("important")
            .with_label("My Block");
        let id = doc.add_block(block, &root).unwrap();

        assert!(doc.indices.find_by_tag("important").contains(&id));
        assert_eq!(doc.indices.find_by_label("My Block"), Some(id));
    }
}
