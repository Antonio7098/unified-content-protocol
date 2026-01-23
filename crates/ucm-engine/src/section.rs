//! Section management utilities for UCM documents.
//!
//! This module provides utilities for working with document sections,
//! including clearing content, integrating blocks, and finding sections by path.
//!
//! The module supports undo operations by preserving deleted content
//! in `DeletedContent` structures that can be used for restoration.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use ucm_core::{Block, BlockId, Content, Document};

use crate::error::{Error, Result};

/// Represents deleted content that can be restored.
///
/// When blocks are deleted or sections are rewritten, this structure
/// preserves the original content and structure for potential restoration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedContent {
    /// The blocks that were deleted, keyed by their original IDs
    pub blocks: HashMap<BlockId, Block>,
    /// The structure (parent -> children) of deleted blocks
    pub structure: HashMap<BlockId, Vec<BlockId>>,
    /// The parent block ID where this content was attached
    pub parent_id: BlockId,
    /// Timestamp when the deletion occurred
    pub deleted_at: chrono::DateTime<chrono::Utc>,
}

impl DeletedContent {
    /// Create a new DeletedContent structure
    pub fn new(parent_id: BlockId) -> Self {
        Self {
            blocks: HashMap::new(),
            structure: HashMap::new(),
            parent_id,
            deleted_at: chrono::Utc::now(),
        }
    }

    /// Check if there is any deleted content
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// Get the number of deleted blocks
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Get all block IDs in the deleted content
    pub fn block_ids(&self) -> Vec<BlockId> {
        self.blocks.keys().copied().collect()
    }
}

/// Result of a section clear operation with undo support
#[derive(Debug, Clone)]
pub struct ClearResult {
    /// IDs of blocks that were removed
    pub removed_ids: Vec<BlockId>,
    /// The deleted content for potential restoration
    pub deleted_content: DeletedContent,
}

/// Clear all children of a section, preparing it for new content.
///
/// This removes all child blocks from the section, recursively deleting
/// the entire subtree rooted at the section.
///
/// # Arguments
/// * `doc` - The document to modify
/// * `section_id` - The block ID of the section to clear
///
/// # Returns
/// * `Ok(Vec<BlockId>)` - List of removed block IDs
/// * `Err(Error)` - If the section doesn't exist
pub fn clear_section_content(doc: &mut Document, section_id: &BlockId) -> Result<Vec<BlockId>> {
    let result = clear_section_content_with_undo(doc, section_id)?;
    Ok(result.removed_ids)
}

/// Clear all children of a section with undo support.
///
/// This is like `clear_section_content` but preserves the deleted content
/// for potential restoration using `restore_deleted_content`.
///
/// # Arguments
/// * `doc` - The document to modify
/// * `section_id` - The block ID of the section to clear
///
/// # Returns
/// * `Ok(ClearResult)` - Contains removed IDs and preserved content
/// * `Err(Error)` - If the section doesn't exist
pub fn clear_section_content_with_undo(
    doc: &mut Document,
    section_id: &BlockId,
) -> Result<ClearResult> {
    // Verify section exists
    if !doc.blocks.contains_key(section_id) {
        return Err(Error::BlockNotFound(section_id.to_string()));
    }

    let mut deleted = DeletedContent::new(*section_id);
    let mut to_remove = Vec::new();
    let mut queue = VecDeque::new();

    // Get immediate children and store structure
    if let Some(children) = doc.structure.get(section_id) {
        deleted.structure.insert(*section_id, children.clone());
        for child in children.clone() {
            queue.push_back(child);
        }
    }

    // BFS to collect all descendants and preserve them
    while let Some(block_id) = queue.pop_front() {
        to_remove.push(block_id);

        // Preserve block content
        if let Some(block) = doc.blocks.get(&block_id) {
            deleted.blocks.insert(block_id, block.clone());
        }

        // Preserve and traverse structure
        if let Some(children) = doc.structure.get(&block_id) {
            deleted.structure.insert(block_id, children.clone());
            for child in children.clone() {
                queue.push_back(child);
            }
        }
    }

    // Remove blocks from document
    for block_id in &to_remove {
        doc.blocks.remove(block_id);
        doc.structure.remove(block_id);
    }

    // Clear section's children list
    if let Some(children) = doc.structure.get_mut(section_id) {
        children.clear();
    }

    Ok(ClearResult {
        removed_ids: to_remove,
        deleted_content: deleted,
    })
}

/// Restore previously deleted content to a document.
///
/// This restores blocks that were deleted by `clear_section_content_with_undo`
/// back to their original parent section.
///
/// # Arguments
/// * `doc` - The document to restore to
/// * `deleted` - The deleted content to restore
///
/// # Returns
/// * `Ok(Vec<BlockId>)` - List of restored block IDs
/// * `Err(Error)` - If the parent section doesn't exist
pub fn restore_deleted_content(
    doc: &mut Document,
    deleted: &DeletedContent,
) -> Result<Vec<BlockId>> {
    // Verify parent exists
    if !doc.blocks.contains_key(&deleted.parent_id) {
        return Err(Error::BlockNotFound(deleted.parent_id.to_string()));
    }

    // Remove current content under the parent section
    if let Some(existing_children) = doc.structure.get(&deleted.parent_id).cloned() {
        for child in existing_children {
            remove_subtree(doc, &child);
        }
        if let Some(children) = doc.structure.get_mut(&deleted.parent_id) {
            children.clear();
        }
    }

    let mut restored = Vec::new();

    // Restore all blocks
    for (block_id, block) in &deleted.blocks {
        doc.blocks.insert(*block_id, block.clone());
        restored.push(*block_id);
    }

    // Restore structure for deleted blocks
    for (block_id, children) in &deleted.structure {
        if *block_id != deleted.parent_id {
            doc.structure.insert(*block_id, children.clone());
        }
    }

    // Restore children of parent section
    if let Some(parent_children) = deleted.structure.get(&deleted.parent_id) {
        if let Some(children) = doc.structure.get_mut(&deleted.parent_id) {
            children.extend(parent_children.clone());
        } else {
            doc.structure
                .insert(deleted.parent_id, parent_children.clone());
        }
    }

    Ok(restored)
}

fn remove_subtree(doc: &mut Document, block_id: &BlockId) {
    if let Some(children) = doc.structure.get(block_id).cloned() {
        for child in children {
            remove_subtree(doc, &child);
        }
    }

    if let Some(parent) = doc.parent(block_id).cloned() {
        if let Some(children) = doc.structure.get_mut(&parent) {
            children.retain(|c| c != block_id);
        }
    }

    doc.blocks.remove(block_id);
    doc.structure.remove(block_id);
}

/// Integrate blocks from a source document into a target section.
///
/// This takes all non-root blocks from the source document and adds them
/// as children of the target section, preserving their relative hierarchy.
///
/// # Arguments
/// * `doc` - The target document to modify
/// * `target_section` - The section to add blocks to
/// * `source_doc` - The source document containing blocks to integrate
/// * `base_heading_level` - Optional base level for heading adjustment
///
/// # Returns
/// * `Ok(Vec<BlockId>)` - List of added block IDs
/// * `Err(Error)` - If the target section doesn't exist
pub fn integrate_section_blocks(
    doc: &mut Document,
    target_section: &BlockId,
    source_doc: &Document,
    base_heading_level: Option<usize>,
) -> Result<Vec<BlockId>> {
    // Verify target section exists
    if !doc.blocks.contains_key(target_section) {
        return Err(Error::BlockNotFound(target_section.to_string()));
    }

    let mut added_blocks = Vec::new();

    // Get root children from source document
    let root_children = source_doc
        .structure
        .get(&source_doc.root)
        .cloned()
        .unwrap_or_default();

    // Process each root child and its subtree
    for child_id in root_children {
        let integrated = integrate_subtree(
            doc,
            target_section,
            source_doc,
            &child_id,
            base_heading_level,
            0,
        )?;
        added_blocks.extend(integrated);
    }

    Ok(added_blocks)
}

/// Recursively integrate a subtree from source to target document.
fn integrate_subtree(
    doc: &mut Document,
    parent_id: &BlockId,
    source_doc: &Document,
    source_block_id: &BlockId,
    base_heading_level: Option<usize>,
    depth: usize,
) -> Result<Vec<BlockId>> {
    let mut added_blocks = Vec::new();

    // Get the source block
    let source_block = source_doc
        .get_block(source_block_id)
        .ok_or_else(|| Error::BlockNotFound(source_block_id.to_string()))?;

    // Clone and potentially adjust heading level
    let mut new_block = source_block.clone();

    if let Some(base_level) = base_heading_level {
        adjust_heading_level(&mut new_block, base_level, depth);
    }

    // Regenerate ID for the new block to avoid conflicts
    let new_id = regenerate_block_id(&new_block);
    new_block.id = new_id;

    // Add block to target document
    doc.blocks.insert(new_id, new_block);
    added_blocks.push(new_id);

    // Add to parent's children
    let parent_children = doc.structure.entry(*parent_id).or_default();
    parent_children.push(new_id);

    // Initialize structure for new block
    doc.structure.entry(new_id).or_default();

    // Process children recursively
    if let Some(children) = source_doc.structure.get(source_block_id) {
        for child_id in children.clone() {
            let child_added = integrate_subtree(
                doc,
                &new_id,
                source_doc,
                &child_id,
                base_heading_level,
                depth + 1,
            )?;
            added_blocks.extend(child_added);
        }
    }

    Ok(added_blocks)
}

/// Adjust heading level based on base level and depth.
fn adjust_heading_level(block: &mut Block, base_level: usize, _depth: usize) {
    if let Some(ref mut role) = block.metadata.semantic_role {
        let role_str = role.category.as_str();

        // Check if this is a heading
        if let Some(level_str) = role_str.strip_prefix("heading") {
            if let Ok(current_level) = level_str.parse::<usize>() {
                // Adjust level: new_level = base_level + current_level - 1
                let new_level = (base_level + current_level - 1).clamp(1, 6);

                // Update the semantic role
                if let Some(new_role) =
                    ucm_core::metadata::SemanticRole::parse(&format!("heading{}", new_level))
                {
                    *role = new_role;
                }
            }
        }
    }
}

/// Regenerate block ID to avoid conflicts.
fn regenerate_block_id(block: &Block) -> BlockId {
    use chrono::Utc;

    // Create a unique ID based on content and timestamp
    let timestamp = Utc::now().timestamp_nanos_opt().unwrap_or(0) as u128;

    let content_hash = ucm_core::id::compute_content_hash(&block.content);

    // Combine timestamp and content hash for uniqueness
    let mut id_bytes = [0u8; 12];
    id_bytes[0..8].copy_from_slice(&timestamp.to_le_bytes()[0..8]);
    id_bytes[8..12].copy_from_slice(&content_hash.as_bytes()[0..4]);

    BlockId::from_bytes(id_bytes)
}

/// Find a section by path (e.g., "Section 1 > Subsection 2").
///
/// The path uses " > " as a separator between heading names.
///
/// # Arguments
/// * `doc` - The document to search
/// * `path` - The path to the section (e.g., "Introduction > Getting Started")
///
/// # Returns
/// * `Some(BlockId)` - The block ID of the found section
/// * `None` - If the path doesn't match any section
pub fn find_section_by_path(doc: &Document, path: &str) -> Option<BlockId> {
    let parts: Vec<&str> = path.split(" > ").map(|s| s.trim()).collect();

    if parts.is_empty() {
        return None;
    }

    let mut current_id = doc.root;

    for part in parts {
        let children = doc.structure.get(&current_id)?;

        let found = children.iter().find(|child_id| {
            if let Some(block) = doc.get_block(child_id) {
                // Check if this is a heading with matching text
                let is_heading = block
                    .metadata
                    .semantic_role
                    .as_ref()
                    .map(|r| r.category.as_str().starts_with("heading"))
                    .unwrap_or(false);

                if is_heading {
                    // Extract text content
                    let text = match &block.content {
                        Content::Text(t) => t.text.trim(),
                        _ => return false,
                    };
                    return text == part;
                }
            }
            false
        });

        current_id = *found?;
    }

    if current_id == doc.root {
        None
    } else {
        Some(current_id)
    }
}

/// Get the depth of a section in the document hierarchy.
///
/// # Arguments
/// * `doc` - The document to search
/// * `section_id` - The section to find the depth of
///
/// # Returns
/// * `Some(usize)` - The depth (0 for root children)
/// * `None` - If the section doesn't exist
pub fn get_section_depth(doc: &Document, section_id: &BlockId) -> Option<usize> {
    if *section_id == doc.root {
        return Some(0);
    }

    let mut depth = 0;
    let mut current = *section_id;

    while let Some(parent) = doc.parent(&current) {
        depth += 1;
        if *parent == doc.root {
            return Some(depth);
        }
        current = *parent;
    }

    None
}

/// Get all sections (heading blocks) in the document.
///
/// # Arguments
/// * `doc` - The document to search
///
/// # Returns
/// * `Vec<(BlockId, usize)>` - List of (section_id, heading_level) tuples
pub fn get_all_sections(doc: &Document) -> Vec<(BlockId, usize)> {
    let mut sections = Vec::new();

    for (block_id, block) in &doc.blocks {
        if let Some(ref role) = block.metadata.semantic_role {
            if let Some(level_str) = role.category.as_str().strip_prefix("heading") {
                if let Ok(level) = level_str.parse::<usize>() {
                    sections.push((*block_id, level));
                }
            }
        }
    }

    sections
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucm_core::{Block, Content, Document};

    fn create_test_document() -> Document {
        let mut doc = Document::create();
        let root = doc.root;

        // Create heading structure: H1 > H2 > paragraph
        let h1 = Block::new(Content::text("Introduction"), Some("heading1"));
        let h1_id = doc.add_block(h1, &root).unwrap();

        let h2 = Block::new(Content::text("Getting Started"), Some("heading2"));
        let h2_id = doc.add_block(h2, &h1_id).unwrap();

        let para = Block::new(Content::text("Some content here"), Some("paragraph"));
        doc.add_block(para, &h2_id).unwrap();

        doc
    }

    #[test]
    fn test_clear_section_content() {
        let mut doc = create_test_document();

        // Find the H1 section
        let h1_id = find_section_by_path(&doc, "Introduction").unwrap();

        // Clear the section
        let removed = clear_section_content(&mut doc, &h1_id).unwrap();

        // Should have removed H2 and paragraph
        assert_eq!(removed.len(), 2);

        // H1 should have no children now
        let children = doc.structure.get(&h1_id).unwrap();
        assert!(children.is_empty());
    }

    #[test]
    fn test_find_section_by_path() {
        let doc = create_test_document();

        // Find single level
        let h1_id = find_section_by_path(&doc, "Introduction");
        assert!(h1_id.is_some());

        // Find nested path
        let h2_id = find_section_by_path(&doc, "Introduction > Getting Started");
        assert!(h2_id.is_some());

        // Non-existent path
        let missing = find_section_by_path(&doc, "Missing Section");
        assert!(missing.is_none());
    }

    #[test]
    fn test_get_all_sections() {
        let doc = create_test_document();

        let sections = get_all_sections(&doc);

        // Should have H1 and H2
        assert_eq!(sections.len(), 2);

        // Check levels
        let levels: Vec<usize> = sections.iter().map(|(_, l)| *l).collect();
        assert!(levels.contains(&1));
        assert!(levels.contains(&2));
    }

    #[test]
    fn test_get_section_depth() {
        let doc = create_test_document();

        let h1_id = find_section_by_path(&doc, "Introduction").unwrap();
        let h2_id = find_section_by_path(&doc, "Introduction > Getting Started").unwrap();

        assert_eq!(get_section_depth(&doc, &h1_id), Some(1));
        assert_eq!(get_section_depth(&doc, &h2_id), Some(2));
    }

    #[test]
    fn test_clear_with_undo_and_restore() {
        let mut doc = create_test_document();
        let original_count = doc.block_count();

        // Find the H1 section
        let h1_id = find_section_by_path(&doc, "Introduction").unwrap();

        // Clear with undo support
        let result = clear_section_content_with_undo(&mut doc, &h1_id).unwrap();

        // Should have removed H2 and paragraph
        assert_eq!(result.removed_ids.len(), 2);
        assert_eq!(result.deleted_content.block_count(), 2);

        // Document should have fewer blocks
        assert!(doc.block_count() < original_count);

        // Restore the deleted content
        let restored = restore_deleted_content(&mut doc, &result.deleted_content).unwrap();

        // Should have restored all blocks
        assert_eq!(restored.len(), 2);
        assert_eq!(doc.block_count(), original_count);

        // H1 should have children again
        let children = doc.structure.get(&h1_id).unwrap();
        assert!(!children.is_empty());
    }
}
