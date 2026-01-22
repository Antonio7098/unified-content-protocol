//! Context management infrastructure for UCM documents.
//!
//! This module provides APIs for intelligent context window management,
//! allowing external orchestration layers to load documents, traverse
//! the knowledge graph, and curate context windows while preserving UCM invariants.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use ucm_core::{BlockId, Content, Document};

#[cfg(test)]
use ucm_core::Block;

/// Reason why a block was included in context
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InclusionReason {
    /// Directly referenced by task or query
    DirectReference,
    /// Part of navigation path
    NavigationPath,
    /// Structural context (parent/sibling)
    StructuralContext,
    /// Semantic relevance
    SemanticRelevance,
    /// External decision (from orchestrator)
    ExternalDecision,
    /// Required for understanding
    RequiredContext,
}

/// A block in the context window with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextBlock {
    pub block_id: BlockId,
    pub inclusion_reason: InclusionReason,
    pub relevance_score: f32,
    pub token_estimate: usize,
    pub access_count: usize,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub compressed: bool,
    pub original_content: Option<String>,
}

/// Relationship between context blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextRelation {
    pub source: BlockId,
    pub target: BlockId,
    pub relation_type: String,
}

/// Context window metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContextMetadata {
    pub focus_area: Option<BlockId>,
    pub task_description: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
}

/// Constraints for the context window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConstraints {
    pub max_tokens: usize,
    pub max_blocks: usize,
    pub max_depth: usize,
    pub min_relevance: f32,
    pub required_roles: Vec<String>,
    pub excluded_tags: Vec<String>,
    pub preserve_structure: bool,
    pub allow_compression: bool,
}

impl Default for ContextConstraints {
    fn default() -> Self {
        Self {
            max_tokens: 4000,
            max_blocks: 100,
            max_depth: 10,
            min_relevance: 0.0,
            required_roles: Vec::new(),
            excluded_tags: Vec::new(),
            preserve_structure: true,
            allow_compression: true,
        }
    }
}

/// Direction for context expansion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpandDirection {
    Up,
    Down,
    Both,
    Semantic,
}

/// Policy for context expansion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ExpansionPolicy {
    /// Only add highly relevant blocks
    Conservative,
    /// Balance relevance and diversity
    #[default]
    Balanced,
    /// Add potentially useful blocks
    Aggressive,
}

/// Policy for context pruning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PruningPolicy {
    /// Remove lowest relevance first
    #[default]
    RelevanceFirst,
    /// Remove least recently accessed
    RecencyFirst,
    /// Remove redundant content
    RedundancyFirst,
}

/// Method for content compression
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CompressionMethod {
    /// Truncate to length limit
    #[default]
    Truncate,
    /// Summarize content (requires external summarizer)
    Summarize,
    /// Show only structure, not content
    StructureOnly,
}

/// Result of a context operation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContextUpdateResult {
    pub blocks_added: Vec<BlockId>,
    pub blocks_removed: Vec<BlockId>,
    pub blocks_compressed: Vec<BlockId>,
    pub total_tokens: usize,
    pub total_blocks: usize,
    pub warnings: Vec<String>,
}

/// Statistics about the context window
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContextStatistics {
    pub total_tokens: usize,
    pub total_blocks: usize,
    pub blocks_by_reason: HashMap<String, usize>,
    pub average_relevance: f32,
    pub depth_distribution: HashMap<usize, usize>,
    pub compressed_count: usize,
}

/// Context window with intelligent management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextWindow {
    pub id: String,
    pub blocks: HashMap<BlockId, ContextBlock>,
    pub relationships: Vec<ContextRelation>,
    pub metadata: ContextMetadata,
    pub constraints: ContextConstraints,
}

impl ContextWindow {
    /// Create a new empty context window
    pub fn new(id: impl Into<String>, constraints: ContextConstraints) -> Self {
        Self {
            id: id.into(),
            blocks: HashMap::new(),
            relationships: Vec::new(),
            metadata: ContextMetadata {
                created_at: Some(chrono::Utc::now()),
                ..Default::default()
            },
            constraints,
        }
    }

    /// Get the number of blocks in the context
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Get estimated total tokens
    pub fn total_tokens(&self) -> usize {
        self.blocks.values().map(|b| b.token_estimate).sum()
    }

    /// Check if context has room for more blocks
    pub fn has_capacity(&self) -> bool {
        self.blocks.len() < self.constraints.max_blocks
            && self.total_tokens() < self.constraints.max_tokens
    }

    /// Check if a block is in the context
    pub fn contains(&self, block_id: &BlockId) -> bool {
        self.blocks.contains_key(block_id)
    }

    /// Get a block from the context
    pub fn get(&self, block_id: &BlockId) -> Option<&ContextBlock> {
        self.blocks.get(block_id)
    }

    /// Get all block IDs in the context
    pub fn block_ids(&self) -> Vec<BlockId> {
        self.blocks.keys().copied().collect()
    }
}

/// Context Management Infrastructure
///
/// Provides APIs for external orchestration layers to manage context windows.
pub struct ContextManager {
    window: ContextWindow,
    expansion_policy: ExpansionPolicy,
    pruning_policy: PruningPolicy,
}

impl ContextManager {
    /// Create a new context manager with default constraints
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            window: ContextWindow::new(id, ContextConstraints::default()),
            expansion_policy: ExpansionPolicy::default(),
            pruning_policy: PruningPolicy::default(),
        }
    }

    /// Create a context manager with custom constraints
    pub fn with_constraints(id: impl Into<String>, constraints: ContextConstraints) -> Self {
        Self {
            window: ContextWindow::new(id, constraints),
            expansion_policy: ExpansionPolicy::default(),
            pruning_policy: PruningPolicy::default(),
        }
    }

    /// Set the expansion policy
    pub fn with_expansion_policy(mut self, policy: ExpansionPolicy) -> Self {
        self.expansion_policy = policy;
        self
    }

    /// Set the pruning policy
    pub fn with_pruning_policy(mut self, policy: PruningPolicy) -> Self {
        self.pruning_policy = policy;
        self
    }

    /// Get a reference to the context window
    pub fn window(&self) -> &ContextWindow {
        &self.window
    }

    /// Initialize context with a focus block
    pub fn initialize_focus(
        &mut self,
        doc: &Document,
        focus_id: BlockId,
        task_description: &str,
    ) -> ContextUpdateResult {
        self.window.metadata.focus_area = Some(focus_id);
        self.window.metadata.task_description = Some(task_description.to_string());
        self.window.metadata.last_modified = Some(chrono::Utc::now());

        let mut result = ContextUpdateResult::default();

        // Add focus block
        if let Some(_block) = doc.get_block(&focus_id) {
            self.add_block_internal(doc, focus_id, InclusionReason::DirectReference, 1.0);
            result.blocks_added.push(focus_id);
        }

        // Add structural context (ancestors)
        let mut current = focus_id;
        let mut depth = 0;
        while let Some(parent) = doc.parent(&current) {
            if *parent == doc.root || depth >= 3 {
                break;
            }
            self.add_block_internal(
                doc,
                *parent,
                InclusionReason::StructuralContext,
                0.8 - depth as f32 * 0.1,
            );
            result.blocks_added.push(*parent);
            current = *parent;
            depth += 1;
        }

        result.total_tokens = self.window.total_tokens();
        result.total_blocks = self.window.block_count();
        result
    }

    /// Navigate to a new focus area
    pub fn navigate_to(
        &mut self,
        doc: &Document,
        target_id: BlockId,
        task_description: &str,
    ) -> ContextUpdateResult {
        self.window.metadata.focus_area = Some(target_id);
        self.window.metadata.task_description = Some(task_description.to_string());
        self.window.metadata.last_modified = Some(chrono::Utc::now());

        let mut result = ContextUpdateResult::default();

        // Add target block
        if doc.get_block(&target_id).is_some() {
            self.add_block_internal(doc, target_id, InclusionReason::NavigationPath, 1.0);
            result.blocks_added.push(target_id);
        }

        // Prune if needed
        let pruned = self.prune_if_needed();
        result.blocks_removed = pruned;

        result.total_tokens = self.window.total_tokens();
        result.total_blocks = self.window.block_count();
        result
    }

    /// Add a block to the context
    pub fn add_block(
        &mut self,
        doc: &Document,
        block_id: BlockId,
        reason: InclusionReason,
    ) -> ContextUpdateResult {
        let mut result = ContextUpdateResult::default();

        if doc.get_block(&block_id).is_some() {
            self.add_block_internal(doc, block_id, reason, 0.7);
            result.blocks_added.push(block_id);
        }

        // Prune if needed
        let pruned = self.prune_if_needed();
        result.blocks_removed = pruned;

        result.total_tokens = self.window.total_tokens();
        result.total_blocks = self.window.block_count();
        result
    }

    /// Remove a block from the context
    pub fn remove_block(&mut self, block_id: BlockId) -> ContextUpdateResult {
        let mut result = ContextUpdateResult::default();

        if self.window.blocks.remove(&block_id).is_some() {
            result.blocks_removed.push(block_id);
        }

        self.window.metadata.last_modified = Some(chrono::Utc::now());
        result.total_tokens = self.window.total_tokens();
        result.total_blocks = self.window.block_count();
        result
    }

    /// Expand context in a direction
    pub fn expand_context(
        &mut self,
        doc: &Document,
        direction: ExpandDirection,
        depth: usize,
    ) -> ContextUpdateResult {
        let mut result = ContextUpdateResult::default();

        let focus_id = match self.window.metadata.focus_area {
            Some(id) => id,
            None => return result,
        };

        match direction {
            ExpandDirection::Down => {
                let added = self.expand_downward(doc, focus_id, depth);
                result.blocks_added = added;
            }
            ExpandDirection::Up => {
                let added = self.expand_upward(doc, focus_id, depth);
                result.blocks_added = added;
            }
            ExpandDirection::Both => {
                let added_down = self.expand_downward(doc, focus_id, depth);
                let added_up = self.expand_upward(doc, focus_id, depth);
                result.blocks_added = added_down.into_iter().chain(added_up).collect();
            }
            ExpandDirection::Semantic => {
                // Semantic expansion would use edges
                let added = self.expand_semantic(doc, focus_id, depth);
                result.blocks_added = added;
            }
        }

        // Prune if needed
        let pruned = self.prune_if_needed();
        result.blocks_removed = pruned;

        self.window.metadata.last_modified = Some(chrono::Utc::now());
        result.total_tokens = self.window.total_tokens();
        result.total_blocks = self.window.block_count();
        result
    }

    /// Compress blocks to fit within constraints
    pub fn compress(&mut self, doc: &Document, method: CompressionMethod) -> ContextUpdateResult {
        let mut result = ContextUpdateResult::default();

        if !self.window.constraints.allow_compression {
            result
                .warnings
                .push("Compression not allowed by constraints".to_string());
            return result;
        }

        // Find blocks to compress (lowest relevance, not already compressed)
        let mut blocks_to_compress: Vec<(BlockId, f32)> = self
            .window
            .blocks
            .iter()
            .filter(|(_, cb)| !cb.compressed)
            .map(|(id, cb)| (*id, cb.relevance_score))
            .collect();

        blocks_to_compress
            .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        for (block_id, _) in blocks_to_compress.iter().take(10) {
            // Extract content text before mutable borrow
            let original = doc
                .get_block(block_id)
                .map(|block| self.extract_content_text(&block.content));

            if let Some(context_block) = self.window.blocks.get_mut(block_id) {
                if let Some(original_text) = original {
                    context_block.original_content = Some(original_text);

                    match method {
                        CompressionMethod::Truncate => {
                            // Reduce token estimate
                            context_block.token_estimate /= 2;
                        }
                        CompressionMethod::StructureOnly => {
                            // Minimal token estimate
                            context_block.token_estimate = 10;
                        }
                        CompressionMethod::Summarize => {
                            // Would need external summarizer
                            context_block.token_estimate /= 3;
                        }
                    }

                    context_block.compressed = true;
                    result.blocks_compressed.push(*block_id);
                }
            }

            // Check if we're within constraints
            if self.window.total_tokens() <= self.window.constraints.max_tokens {
                break;
            }
        }

        result.total_tokens = self.window.total_tokens();
        result.total_blocks = self.window.block_count();
        result
    }

    /// Get statistics about the context
    pub fn get_statistics(&self) -> ContextStatistics {
        let mut blocks_by_reason: HashMap<String, usize> = HashMap::new();
        let mut total_relevance = 0.0;
        let mut compressed_count = 0;

        for cb in self.window.blocks.values() {
            let reason = format!("{:?}", cb.inclusion_reason);
            *blocks_by_reason.entry(reason).or_insert(0) += 1;
            total_relevance += cb.relevance_score;
            if cb.compressed {
                compressed_count += 1;
            }
        }

        let average_relevance = if self.window.blocks.is_empty() {
            0.0
        } else {
            total_relevance / self.window.blocks.len() as f32
        };

        ContextStatistics {
            total_tokens: self.window.total_tokens(),
            total_blocks: self.window.block_count(),
            blocks_by_reason,
            average_relevance,
            depth_distribution: HashMap::new(), // Would need document access
            compressed_count,
        }
    }

    /// Render context to a format suitable for LLM prompts
    pub fn render_for_prompt(&self, doc: &Document) -> String {
        let mut output = String::new();

        // Sort blocks by relevance for output
        let mut blocks: Vec<(&BlockId, &ContextBlock)> = self.window.blocks.iter().collect();
        blocks.sort_by(|a, b| {
            b.1.relevance_score
                .partial_cmp(&a.1.relevance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for (block_id, context_block) in blocks {
            if let Some(block) = doc.get_block(block_id) {
                let content = if context_block.compressed {
                    if let Some(ref original) = context_block.original_content {
                        format!("[compressed] {}...", &original[..original.len().min(50)])
                    } else {
                        "[compressed]".to_string()
                    }
                } else {
                    self.extract_content_text(&block.content)
                };

                let role = block
                    .metadata
                    .semantic_role
                    .as_ref()
                    .map(|r| r.category.as_str())
                    .unwrap_or("block");

                output.push_str(&format!("[{}] {}: {}\n", block_id, role, content));
            }
        }

        output
    }

    // Internal helper methods

    fn add_block_internal(
        &mut self,
        doc: &Document,
        block_id: BlockId,
        reason: InclusionReason,
        relevance: f32,
    ) {
        if self.window.blocks.contains_key(&block_id) {
            // Update access count
            if let Some(cb) = self.window.blocks.get_mut(&block_id) {
                cb.access_count += 1;
                cb.last_accessed = chrono::Utc::now();
            }
            return;
        }

        if let Some(block) = doc.get_block(&block_id) {
            let token_estimate = self.estimate_tokens(&block.content);

            let context_block = ContextBlock {
                block_id,
                inclusion_reason: reason,
                relevance_score: relevance,
                token_estimate,
                access_count: 1,
                last_accessed: chrono::Utc::now(),
                compressed: false,
                original_content: None,
            };

            self.window.blocks.insert(block_id, context_block);
        }
    }

    fn expand_downward(
        &mut self,
        doc: &Document,
        start: BlockId,
        max_depth: usize,
    ) -> Vec<BlockId> {
        let mut added = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0usize));

        while let Some((node_id, depth)) = queue.pop_front() {
            if depth > max_depth || !self.window.has_capacity() {
                break;
            }

            for child in doc.children(&node_id) {
                if !self.window.contains(child) {
                    let relevance = 0.6 - depth as f32 * 0.1;
                    self.add_block_internal(
                        doc,
                        *child,
                        InclusionReason::StructuralContext,
                        relevance.max(0.1),
                    );
                    added.push(*child);
                    queue.push_back((*child, depth + 1));
                }
            }
        }

        added
    }

    fn expand_upward(&mut self, doc: &Document, start: BlockId, max_depth: usize) -> Vec<BlockId> {
        let mut added = Vec::new();
        let mut current = start;
        let mut depth = 0;

        while let Some(parent) = doc.parent(&current) {
            if *parent == doc.root || depth >= max_depth || !self.window.has_capacity() {
                break;
            }

            if !self.window.contains(parent) {
                let relevance = 0.7 - depth as f32 * 0.1;
                self.add_block_internal(
                    doc,
                    *parent,
                    InclusionReason::StructuralContext,
                    relevance.max(0.1),
                );
                added.push(*parent);
            }

            current = *parent;
            depth += 1;
        }

        added
    }

    fn expand_semantic(
        &mut self,
        doc: &Document,
        start: BlockId,
        max_depth: usize,
    ) -> Vec<BlockId> {
        let mut added = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0usize));

        while let Some((node_id, depth)) = queue.pop_front() {
            if depth > max_depth || !self.window.has_capacity() {
                break;
            }

            if visited.contains(&node_id) {
                continue;
            }
            visited.insert(node_id);

            if let Some(block) = doc.get_block(&node_id) {
                for edge in &block.edges {
                    if !self.window.contains(&edge.target) && !visited.contains(&edge.target) {
                        let relevance = 0.5 - depth as f32 * 0.1;
                        self.add_block_internal(
                            doc,
                            edge.target,
                            InclusionReason::SemanticRelevance,
                            relevance.max(0.1),
                        );
                        added.push(edge.target);
                        queue.push_back((edge.target, depth + 1));
                    }
                }
            }
        }

        added
    }

    fn prune_if_needed(&mut self) -> Vec<BlockId> {
        let mut removed = Vec::new();

        while self.window.block_count() > self.window.constraints.max_blocks
            || self.window.total_tokens() > self.window.constraints.max_tokens
        {
            // Find block to remove based on policy
            let to_remove = match self.pruning_policy {
                PruningPolicy::RelevanceFirst => self.find_lowest_relevance(),
                PruningPolicy::RecencyFirst => self.find_least_recent(),
                PruningPolicy::RedundancyFirst => self.find_lowest_relevance(), // Simplified
            };

            if let Some(block_id) = to_remove {
                self.window.blocks.remove(&block_id);
                removed.push(block_id);
            } else {
                break;
            }
        }

        removed
    }

    fn find_lowest_relevance(&self) -> Option<BlockId> {
        self.window
            .blocks
            .iter()
            .filter(|(id, _)| Some(**id) != self.window.metadata.focus_area)
            .min_by(|a, b| {
                a.1.relevance_score
                    .partial_cmp(&b.1.relevance_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(id, _)| *id)
    }

    fn find_least_recent(&self) -> Option<BlockId> {
        self.window
            .blocks
            .iter()
            .filter(|(id, _)| Some(**id) != self.window.metadata.focus_area)
            .min_by(|a, b| a.1.last_accessed.cmp(&b.1.last_accessed))
            .map(|(id, _)| *id)
    }

    fn estimate_tokens(&self, content: &Content) -> usize {
        let text = self.extract_content_text(content);
        // Rough estimate: ~4 characters per token
        (text.len() / 4).max(1)
    }

    fn extract_content_text(&self, content: &Content) -> String {
        match content {
            Content::Text(t) => t.text.clone(),
            Content::Code(c) => c.source.clone(),
            Content::Table(t) => format!("Table: {} rows", t.rows.len()),
            Content::Math(m) => m.expression.clone(),
            Content::Media(m) => m.alt_text.clone().unwrap_or_else(|| "Media".to_string()),
            Content::Json { .. } => "JSON data".to_string(),
            Content::Binary { .. } => "Binary data".to_string(),
            Content::Composite { children, .. } => {
                format!("Composite: {} children", children.len())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucm_core::DocumentId;

    fn create_test_document() -> Document {
        let mut doc = Document::new(DocumentId::new("test"));
        let root = doc.root;

        let h1 = Block::new(Content::text("Chapter 1"), Some("heading1"));
        let h1_id = doc.add_block(h1, &root).unwrap();

        let p1 = Block::new(
            Content::text("Introduction paragraph with some content"),
            Some("paragraph"),
        );
        doc.add_block(p1, &h1_id).unwrap();

        let h2 = Block::new(Content::text("Section 1.1"), Some("heading2"));
        let h2_id = doc.add_block(h2, &h1_id).unwrap();

        let p2 = Block::new(Content::text("Section content here"), Some("paragraph"));
        doc.add_block(p2, &h2_id).unwrap();

        doc
    }

    #[test]
    fn test_context_manager_creation() {
        let manager = ContextManager::new("test-context");
        assert_eq!(manager.window().block_count(), 0);
        assert!(manager.window().has_capacity());
    }

    #[test]
    fn test_initialize_focus() {
        let doc = create_test_document();
        let mut manager = ContextManager::new("test-context");

        let root_children = doc.children(&doc.root);
        let h1_id = root_children[0];

        let result = manager.initialize_focus(&doc, h1_id, "Test task");

        assert!(!result.blocks_added.is_empty());
        assert!(manager.window().contains(&h1_id));
    }

    #[test]
    fn test_expand_context() {
        let doc = create_test_document();
        let mut manager = ContextManager::new("test-context");

        let root_children = doc.children(&doc.root);
        let h1_id = root_children[0];

        manager.initialize_focus(&doc, h1_id, "Test task");

        let result = manager.expand_context(&doc, ExpandDirection::Down, 2);

        assert!(result.total_blocks > 1);
    }

    #[test]
    fn test_add_and_remove_block() {
        let doc = create_test_document();
        let mut manager = ContextManager::new("test-context");

        let root_children = doc.children(&doc.root);
        let h1_id = root_children[0];

        let result = manager.add_block(&doc, h1_id, InclusionReason::DirectReference);
        assert!(result.blocks_added.contains(&h1_id));
        assert!(manager.window().contains(&h1_id));

        let result = manager.remove_block(h1_id);
        assert!(result.blocks_removed.contains(&h1_id));
        assert!(!manager.window().contains(&h1_id));
    }

    #[test]
    fn test_constraints() {
        let constraints = ContextConstraints {
            max_blocks: 5,
            max_tokens: 100,
            ..Default::default()
        };

        let doc = create_test_document();
        let mut manager = ContextManager::with_constraints("test-context", constraints);

        let root_children = doc.children(&doc.root);
        let h1_id = root_children[0];

        manager.initialize_focus(&doc, h1_id, "Test task");
        manager.expand_context(&doc, ExpandDirection::Down, 10);

        // Should be limited by constraints
        assert!(manager.window().block_count() <= 5);
    }

    #[test]
    fn test_statistics() {
        let doc = create_test_document();
        let mut manager = ContextManager::new("test-context");

        let root_children = doc.children(&doc.root);
        let h1_id = root_children[0];

        manager.initialize_focus(&doc, h1_id, "Test task");

        let stats = manager.get_statistics();
        assert!(stats.total_blocks > 0);
        assert!(stats.total_tokens > 0);
    }

    #[test]
    fn test_render_for_prompt() {
        let doc = create_test_document();
        let mut manager = ContextManager::new("test-context");

        let root_children = doc.children(&doc.root);
        let h1_id = root_children[0];

        manager.initialize_focus(&doc, h1_id, "Test task");

        let prompt = manager.render_for_prompt(&doc);
        assert!(!prompt.is_empty());
        assert!(prompt.contains("Chapter 1"));
    }
}
