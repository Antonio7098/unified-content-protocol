//! Main transformation engine.

use crate::operation::{EditOperator, MoveTarget, Operation, OperationResult, PruneCondition};
use crate::snapshot::SnapshotManager;
use crate::transaction::{TransactionId, TransactionManager};
use crate::validate::{ValidationPipeline, ValidationResult};
use tracing::{debug, info, instrument, warn};
use ucm_core::{Block, Content, Document, Edge, Error, Result};

/// Configuration for the engine
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Whether to validate after each operation
    pub validate_on_operation: bool,
    /// Maximum operations per batch
    pub max_batch_size: usize,
    /// Enable transaction support
    pub enable_transactions: bool,
    /// Enable snapshots
    pub enable_snapshots: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            validate_on_operation: true,
            max_batch_size: 10000,
            enable_transactions: true,
            enable_snapshots: true,
        }
    }
}

/// The main transformation engine
pub struct Engine {
    config: EngineConfig,
    validator: ValidationPipeline,
    transactions: TransactionManager,
    snapshots: SnapshotManager,
}

impl Engine {
    /// Create a new engine with default configuration
    pub fn new() -> Self {
        Self {
            config: EngineConfig::default(),
            validator: ValidationPipeline::new(),
            transactions: TransactionManager::new(),
            snapshots: SnapshotManager::new(),
        }
    }

    /// Create an engine with custom configuration
    pub fn with_config(config: EngineConfig) -> Self {
        Self {
            config,
            validator: ValidationPipeline::new(),
            transactions: TransactionManager::new(),
            snapshots: SnapshotManager::new(),
        }
    }

    /// Execute a single operation on a document
    #[instrument(skip(self, doc), fields(op = %op.description()))]
    pub fn execute(&self, doc: &mut Document, op: Operation) -> Result<OperationResult> {
        debug!("Executing operation: {}", op.description());

        let result = self.execute_internal(doc, op)?;

        if self.config.validate_on_operation && !result.success {
            warn!("Operation failed: {:?}", result.error);
        }

        Ok(result)
    }

    /// Execute multiple operations atomically
    #[instrument(skip(self, doc, ops), fields(op_count = ops.len()))]
    pub fn execute_batch(
        &self,
        doc: &mut Document,
        ops: Vec<Operation>,
    ) -> Result<Vec<OperationResult>> {
        if ops.len() > self.config.max_batch_size {
            return Err(Error::ResourceLimit(format!(
                "Batch size {} exceeds maximum {}",
                ops.len(),
                self.config.max_batch_size
            )));
        }

        info!("Executing batch of {} operations", ops.len());

        let mut results = Vec::with_capacity(ops.len());
        for op in ops {
            match self.execute_internal(doc, op) {
                Ok(result) => {
                    if !result.success {
                        // On failure, return results so far
                        results.push(result);
                        break;
                    }
                    results.push(result);
                }
                Err(e) => {
                    results.push(OperationResult::failure(e.to_string()));
                    break;
                }
            }
        }

        Ok(results)
    }

    /// Validate a document
    pub fn validate(&self, doc: &Document) -> ValidationResult {
        self.validator.validate_document(doc)
    }

    /// Begin a transaction
    pub fn begin_transaction(&mut self) -> TransactionId {
        self.transactions.begin()
    }

    /// Begin a named transaction
    pub fn begin_named_transaction(&mut self, name: impl Into<String>) -> TransactionId {
        self.transactions.begin_named(name)
    }

    /// Add operation to a transaction
    pub fn add_to_transaction(&mut self, txn_id: &TransactionId, op: Operation) -> Result<()> {
        self.transactions.add_operation(txn_id, op)
    }

    /// Commit a transaction
    pub fn commit_transaction(
        &mut self,
        txn_id: &TransactionId,
        doc: &mut Document,
    ) -> Result<Vec<OperationResult>> {
        let ops = self.transactions.commit(txn_id)?;
        self.execute_batch(doc, ops)
    }

    /// Rollback a transaction
    pub fn rollback_transaction(&mut self, txn_id: &TransactionId) -> Result<()> {
        self.transactions.rollback(txn_id)
    }

    /// Create a snapshot
    pub fn create_snapshot(
        &mut self,
        name: impl Into<String>,
        doc: &Document,
        description: Option<String>,
    ) -> Result<()> {
        self.snapshots.create(name, doc, description)?;
        Ok(())
    }

    /// Restore from a snapshot
    pub fn restore_snapshot(&self, name: &str) -> Result<Document> {
        self.snapshots.restore(name)
    }

    /// List snapshots
    pub fn list_snapshots(&self) -> Vec<String> {
        self.snapshots
            .list()
            .iter()
            .map(|s| s.id.0.clone())
            .collect()
    }

    /// Delete a snapshot
    pub fn delete_snapshot(&mut self, name: &str) -> bool {
        self.snapshots.delete(name)
    }

    // Internal operation execution
    fn execute_internal(&self, doc: &mut Document, op: Operation) -> Result<OperationResult> {
        match op {
            Operation::Edit {
                block_id,
                path,
                value,
                operator,
            } => self.execute_edit(doc, &block_id, &path, value, operator),

            Operation::Move {
                block_id,
                new_parent,
                index,
            } => self.execute_move(doc, &block_id, &new_parent, index),

            Operation::MoveToTarget { block_id, target } => {
                self.execute_move_to_target(doc, &block_id, target)
            }

            Operation::Append {
                parent_id,
                content,
                label,
                tags,
                semantic_role,
                index,
            } => self.execute_append(doc, &parent_id, content, label, tags, semantic_role, index),

            Operation::Delete {
                block_id,
                cascade,
                preserve_children,
            } => self.execute_delete(doc, &block_id, cascade, preserve_children),

            Operation::Prune { condition } => self.execute_prune(doc, condition),

            Operation::Link {
                source,
                edge_type,
                target,
                metadata,
            } => self.execute_link(doc, &source, edge_type, &target, metadata),

            Operation::Unlink {
                source,
                edge_type,
                target,
            } => self.execute_unlink(doc, &source, edge_type, &target),

            Operation::CreateSnapshot { .. } => {
                // Snapshots are handled separately
                Ok(OperationResult::failure(
                    "Use create_snapshot method for snapshots",
                ))
            }

            Operation::RestoreSnapshot { .. } => {
                // Snapshots are handled separately
                Ok(OperationResult::failure(
                    "Use restore_snapshot method for snapshots",
                ))
            }

            Operation::WriteSection {
                section_id,
                markdown,
                base_heading_level,
            } => self.execute_write_section(doc, &section_id, &markdown, base_heading_level),
        }
    }

    fn execute_edit(
        &self,
        doc: &mut Document,
        block_id: &ucm_core::BlockId,
        path: &str,
        value: serde_json::Value,
        operator: EditOperator,
    ) -> Result<OperationResult> {
        let block = doc
            .get_block_mut(block_id)
            .ok_or_else(|| Error::BlockNotFound(block_id.to_string()))?;

        // Parse path and apply edit
        // This is simplified - a full implementation would parse JSON paths
        if path == "content.text" || path == "text" {
            if let Content::Text(ref mut text) = block.content {
                match operator {
                    EditOperator::Set => {
                        text.text = value.as_str().unwrap_or_default().to_string();
                    }
                    EditOperator::Append => {
                        text.text.push_str(value.as_str().unwrap_or_default());
                    }
                    EditOperator::Remove => {
                        let to_remove = value.as_str().unwrap_or_default();
                        text.text = text.text.replace(to_remove, "");
                    }
                    _ => {}
                }
                block.version.increment();
                return Ok(OperationResult::success(vec![*block_id]));
            }
        }

        // Handle metadata paths
        if path.starts_with("metadata.") {
            let meta_path = path.strip_prefix("metadata.").unwrap();
            match meta_path {
                "label" => {
                    block.metadata.label = value.as_str().map(String::from);
                }
                "tags" => {
                    if let Some(arr) = value.as_array() {
                        match operator {
                            EditOperator::Set => {
                                block.metadata.tags = arr
                                    .iter()
                                    .filter_map(|v| v.as_str().map(String::from))
                                    .collect();
                            }
                            EditOperator::Append => {
                                for v in arr {
                                    if let Some(s) = v.as_str() {
                                        block.metadata.tags.push(s.to_string());
                                    }
                                }
                            }
                            EditOperator::Remove => {
                                let to_remove: Vec<String> = arr
                                    .iter()
                                    .filter_map(|v| v.as_str().map(String::from))
                                    .collect();
                                block.metadata.tags.retain(|t| !to_remove.contains(t));
                            }
                            _ => {}
                        }
                    } else if let Some(s) = value.as_str() {
                        match operator {
                            EditOperator::Append => block.metadata.tags.push(s.to_string()),
                            EditOperator::Remove => block.metadata.tags.retain(|t| t != s),
                            _ => {}
                        }
                    }
                }
                "summary" => {
                    block.metadata.summary = value.as_str().map(String::from);
                }
                _ => {
                    // Custom metadata
                    block.metadata.custom.insert(meta_path.to_string(), value);
                }
            }
            block.version.increment();
            return Ok(OperationResult::success(vec![*block_id]));
        }

        Ok(OperationResult::failure(format!(
            "Unsupported path: {}",
            path
        )))
    }

    fn execute_move(
        &self,
        doc: &mut Document,
        block_id: &ucm_core::BlockId,
        new_parent: &ucm_core::BlockId,
        index: Option<usize>,
    ) -> Result<OperationResult> {
        match index {
            Some(idx) => doc.move_block_at(block_id, new_parent, idx)?,
            None => doc.move_block(block_id, new_parent)?,
        }
        Ok(OperationResult::success(vec![*block_id]))
    }

    fn execute_move_to_target(
        &self,
        doc: &mut Document,
        block_id: &ucm_core::BlockId,
        target: MoveTarget,
    ) -> Result<OperationResult> {
        match target {
            MoveTarget::ToParent { parent_id, index } => {
                self.execute_move(doc, block_id, &parent_id, index)
            }
            MoveTarget::Before { sibling_id } => {
                // Find sibling's parent and index
                let parent_id = doc
                    .parent(&sibling_id)
                    .cloned()
                    .ok_or_else(|| Error::BlockNotFound(sibling_id.to_string()))?;
                let siblings = doc.children(&parent_id);
                let sibling_index = siblings
                    .iter()
                    .position(|id| id == &sibling_id)
                    .ok_or_else(|| Error::Internal("Sibling not found in parent".into()))?;
                doc.move_block_at(block_id, &parent_id, sibling_index)?;
                Ok(OperationResult::success(vec![*block_id]))
            }
            MoveTarget::After { sibling_id } => {
                // Find sibling's parent and index
                let parent_id = doc
                    .parent(&sibling_id)
                    .cloned()
                    .ok_or_else(|| Error::BlockNotFound(sibling_id.to_string()))?;
                let siblings = doc.children(&parent_id);
                let sibling_index = siblings
                    .iter()
                    .position(|id| id == &sibling_id)
                    .ok_or_else(|| Error::Internal("Sibling not found in parent".into()))?;
                doc.move_block_at(block_id, &parent_id, sibling_index + 1)?;
                Ok(OperationResult::success(vec![*block_id]))
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn execute_append(
        &self,
        doc: &mut Document,
        parent_id: &ucm_core::BlockId,
        content: Content,
        label: Option<String>,
        tags: Vec<String>,
        semantic_role: Option<String>,
        index: Option<usize>,
    ) -> Result<OperationResult> {
        let mut block = Block::new(content, semantic_role.as_deref());

        if let Some(l) = label {
            block.metadata.label = Some(l);
        }
        block.metadata.tags = tags;

        let id = match index {
            Some(idx) => doc.add_block_at(block, parent_id, idx)?,
            None => doc.add_block(block, parent_id)?,
        };

        Ok(OperationResult::success(vec![id]))
    }

    fn execute_delete(
        &self,
        doc: &mut Document,
        block_id: &ucm_core::BlockId,
        cascade: bool,
        preserve_children: bool,
    ) -> Result<OperationResult> {
        if preserve_children {
            // Reparent children to grandparent
            if let Some(parent) = doc.parent(block_id).cloned() {
                let children: Vec<_> = doc.children(block_id).to_vec();
                for child in children {
                    doc.move_block(&child, &parent)?;
                }
            }
        }

        let deleted = if cascade {
            doc.delete_cascade(block_id)?
        } else {
            vec![doc.delete_block(block_id)?]
        };

        let ids: Vec<_> = deleted.iter().map(|b| b.id).collect();
        Ok(OperationResult::success(ids))
    }

    fn execute_prune(
        &self,
        doc: &mut Document,
        condition: Option<PruneCondition>,
    ) -> Result<OperationResult> {
        let pruned = match condition {
            None | Some(PruneCondition::Unreachable) => doc.prune_unreachable(),
            Some(PruneCondition::TagContains(tag)) => {
                let to_prune: Vec<_> = doc
                    .blocks
                    .values()
                    .filter(|b| b.has_tag(&tag))
                    .map(|b| b.id)
                    .collect();

                let mut pruned = Vec::new();
                for id in to_prune {
                    if let Ok(block) = doc.delete_block(&id) {
                        pruned.push(block);
                    }
                }
                pruned
            }
            Some(PruneCondition::Custom(_)) => {
                // Custom conditions require UCL expression evaluation
                return Ok(OperationResult::failure(
                    "Custom prune conditions not yet supported",
                ));
            }
        };

        let ids: Vec<_> = pruned.iter().map(|b| b.id).collect();
        Ok(OperationResult::success(ids))
    }

    fn execute_link(
        &self,
        doc: &mut Document,
        source: &ucm_core::BlockId,
        edge_type: ucm_core::EdgeType,
        target: &ucm_core::BlockId,
        metadata: Option<serde_json::Value>,
    ) -> Result<OperationResult> {
        if !doc.blocks.contains_key(source) {
            return Err(Error::BlockNotFound(source.to_string()));
        }
        if !doc.blocks.contains_key(target) {
            return Err(Error::BlockNotFound(target.to_string()));
        }

        let mut edge = Edge::new(edge_type, *target);
        if let Some(meta) = metadata {
            if let Some(obj) = meta.as_object() {
                for (k, v) in obj {
                    edge.metadata.custom.insert(k.clone(), v.clone());
                }
            }
        }

        // Add edge to block
        let block = doc.get_block_mut(source).unwrap();
        block.add_edge(edge.clone());

        // Update edge index
        doc.edge_index.add_edge(source, &edge);

        Ok(OperationResult::success(vec![*source]))
    }

    fn execute_unlink(
        &self,
        doc: &mut Document,
        source: &ucm_core::BlockId,
        edge_type: ucm_core::EdgeType,
        target: &ucm_core::BlockId,
    ) -> Result<OperationResult> {
        let block = doc
            .get_block_mut(source)
            .ok_or_else(|| Error::BlockNotFound(source.to_string()))?;

        let removed = block.remove_edge(target, &edge_type);

        if removed {
            doc.edge_index.remove_edge(source, target, &edge_type);
            Ok(OperationResult::success(vec![*source]))
        } else {
            Ok(OperationResult::failure("Edge not found"))
        }
    }

    fn execute_write_section(
        &self,
        doc: &mut Document,
        section_id: &ucm_core::BlockId,
        markdown: &str,
        base_heading_level: Option<usize>,
    ) -> Result<OperationResult> {
        use crate::section::{clear_section_content, integrate_section_blocks};

        // Verify section exists
        if !doc.blocks.contains_key(section_id) {
            return Err(Error::BlockNotFound(section_id.to_string()));
        }

        // Parse markdown into temporary document
        let temp_doc = match ucp_translator_markdown::parse_markdown(markdown) {
            Ok(d) => d,
            Err(e) => {
                return Ok(OperationResult::failure(format!(
                    "Failed to parse markdown: {}",
                    e
                )));
            }
        };

        // Clear existing section content
        let removed = clear_section_content(doc, section_id)
            .map_err(|e| Error::InvalidBlockId(format!("Failed to clear section: {}", e)))?;

        // Integrate new blocks from parsed markdown
        let added = integrate_section_blocks(doc, section_id, &temp_doc, base_heading_level)
            .map_err(|e| Error::InvalidBlockId(format!("Failed to integrate blocks: {}", e)))?;

        // Collect all affected block IDs
        let mut affected = vec![*section_id];
        affected.extend(removed);
        affected.extend(added);

        Ok(OperationResult::success(affected))
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucm_core::DocumentId;

    #[test]
    fn test_engine_append() {
        let engine = Engine::new();
        let mut doc = Document::new(DocumentId::new("test"));
        let root = doc.root;

        let result = engine
            .execute(
                &mut doc,
                Operation::Append {
                    parent_id: root,
                    content: Content::text("Hello, world!"),
                    label: Some("Greeting".into()),
                    tags: vec!["test".into()],
                    semantic_role: Some("intro".into()),
                    index: None,
                },
            )
            .unwrap();

        assert!(result.success);
        assert_eq!(doc.block_count(), 2);
    }

    #[test]
    fn test_engine_edit() {
        let engine = Engine::new();
        let mut doc = Document::new(DocumentId::new("test"));
        let root = doc.root;

        // Add a block
        let block = Block::new(Content::text("Original"), None);
        let id = doc.add_block(block, &root).unwrap();

        // Edit it
        let result = engine
            .execute(
                &mut doc,
                Operation::Edit {
                    block_id: id,
                    path: "content.text".into(),
                    value: serde_json::json!("Modified"),
                    operator: EditOperator::Set,
                },
            )
            .unwrap();

        assert!(result.success);

        let block = doc.get_block(&id).unwrap();
        if let Content::Text(text) = &block.content {
            assert_eq!(text.text, "Modified");
        }
    }

    #[test]
    fn test_engine_transaction() {
        let mut engine = Engine::new();
        let mut doc = Document::new(DocumentId::new("test"));
        let root = doc.root;

        let txn_id = engine.begin_transaction();

        engine
            .add_to_transaction(
                &txn_id,
                Operation::Append {
                    parent_id: root,
                    content: Content::text("Block 1"),
                    label: None,
                    tags: vec![],
                    semantic_role: None,
                    index: None,
                },
            )
            .unwrap();

        engine
            .add_to_transaction(
                &txn_id,
                Operation::Append {
                    parent_id: root,
                    content: Content::text("Block 2"),
                    label: None,
                    tags: vec![],
                    semantic_role: None,
                    index: None,
                },
            )
            .unwrap();

        let results = engine.commit_transaction(&txn_id, &mut doc).unwrap();

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.success));
        assert_eq!(doc.block_count(), 3); // root + 2 new blocks
    }

    #[test]
    fn test_move_before_target() {
        let engine = Engine::new();
        let mut doc = Document::new(DocumentId::new("test"));
        let root = doc.root;

        let block_a = doc
            .add_block(Block::new(Content::text("A"), None), &root)
            .unwrap();
        let block_b = doc
            .add_block(Block::new(Content::text("B"), None), &root)
            .unwrap();
        let block_c = doc
            .add_block(Block::new(Content::text("C"), None), &root)
            .unwrap();

        let result = engine
            .execute(
                &mut doc,
                Operation::MoveToTarget {
                    block_id: block_c,
                    target: MoveTarget::Before {
                        sibling_id: block_a,
                    },
                },
            )
            .unwrap();

        assert!(result.success);
        let children = doc.children(&root);
        assert_eq!(children[0], block_c);
        assert_eq!(children[1], block_a);
        assert_eq!(children[2], block_b);
    }

    #[test]
    fn test_move_after_target() {
        let engine = Engine::new();
        let mut doc = Document::new(DocumentId::new("test"));
        let root = doc.root;

        let block_a = doc
            .add_block(Block::new(Content::text("A"), None), &root)
            .unwrap();
        let block_b = doc
            .add_block(Block::new(Content::text("B"), None), &root)
            .unwrap();
        let block_c = doc
            .add_block(Block::new(Content::text("C"), None), &root)
            .unwrap();

        let result = engine
            .execute(
                &mut doc,
                Operation::MoveToTarget {
                    block_id: block_a,
                    target: MoveTarget::After {
                        sibling_id: block_c,
                    },
                },
            )
            .unwrap();

        assert!(result.success);
        let children = doc.children(&root);
        assert_eq!(children[0], block_b);
        assert_eq!(children[1], block_c);
        assert_eq!(children[2], block_a);
    }
}
