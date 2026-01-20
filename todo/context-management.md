tem maintains the current normalized format while adding powerful new navigation and exploration capabilities for both agents and users.

Feedback submitted


Your modified files:
graph-traversal.txt
todo
+752

now, context management. i want to have an active context management agent. it navigates knowledge graph (ucm). it can choose to keep blocks/sections, to add to the context. it can trim the current context by dropping blocks/sections from it
Excellent! This is a crucial capability for LLM agents. Let me design a comprehensive context management system that can intelligently navigate and curate the knowledge graph.

Perfect! Let me design a comprehensive Context Management Agent that can intelligently navigate and curate the knowledge graph. This will be a sophisticated system that can maintain context windows, make strategic decisions about what to keep/add/remove.

1. Context Management Architecture
Create crates/ucp-context/src/lib.rs:

rust
//! Intelligent context management for UCM documents.
//!
//! The Context Manager Agent navigates the knowledge graph to maintain
//! optimal context windows for LLM interactions.
use ucm_core::{Block, BlockId, Document};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
/// Context window with intelligent management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextWindow {
    pub id: String,
    pub blocks: HashMap<BlockId, ContextBlock>,
    pub relationships: Vec<ContextRelation>,
    pub metadata: ContextMetadata,
    pub constraints: ContextConstraints,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextBlock {
    pub block: Block,
    pub relevance_score: f32,
    pub inclusion_reason: InclusionReason,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub access_count: u32,
    pub context_position: Option<ContextPosition>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InclusionReason {
    DirectReference,      // Explicitly referenced in task
    SemanticRelevance,    // Semantically related to task
    StructuralContext,    // Provides structural context
    NavigationPath,       // Part of navigation path
    UserPreference,       // User explicitly requested
    AgentDecision,        // Agent decided to include
    SystemRequired,       // Required for system operation
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextPosition {
    pub sequence: usize,           // Order in context
    pub grouping: Option<String>,  // Logical grouping
    pub hierarchy_level: usize,    // Depth in original document
    pub proximity_score: f32,     // How close to focus
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextRelation {
    pub source: BlockId,
    pub target: BlockId,
    pub relation_type: RelationType,
    pub strength: f32,
    pub active: bool,              // Whether relation is currently active
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationType {
    DirectParent,                 // Parent-child relationship
    SemanticLink,                 // Semantic connection
    Reference,                    // Cross-reference
    Sequential,                   // Sequential flow
    Logical,                      // Logical connection
    Temporal,                     // Time-based connection
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub total_tokens: usize,
    pub focus_area: Option<BlockId>,
    pub navigation_history: Vec<NavigationStep>,
    pub compression_ratio: f32,    // Original vs compressed size
    pub quality_score: f32,        // Context coherence quality
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationStep {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub action: NavigationAction,
    pub block_id: Option<BlockId>,
    pub rationale: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NavigationAction {
    ExpandNode,
    CollapseNode,
    AddBlock,
    RemoveBlock,
    MoveFocus,
    FollowLink,
    Search,
    Filter,
}
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
/// Context Management Agent
pub struct ContextManager {
    window: ContextWindow,
    document: Document,
    analyzer: ContextAnalyzer,
    strategy: ContextStrategy,
    cache: ContextCache,
}
#[derive(Debug, Clone)]
pub struct ContextAnalyzer {
    relevance_scorer: RelevanceScorer,
    semantic_analyzer: SemanticAnalyzer,
    structure_analyzer: StructureAnalyzer,
}
#[derive(Debug, Clone)]
pub struct ContextStrategy {
    expansion_policy: ExpansionPolicy,
    pruning_policy: PruningPolicy,
    compression_policy: CompressionPolicy,
}
#[derive(Debug, Clone)]
pub enum ExpansionPolicy {
    Conservative,      // Only add highly relevant blocks
    Balanced,         // Balance relevance and diversity
    Aggressive,        // Add potentially useful blocks
    Adaptive,         // Adapt based on task complexity
}
#[derive(Debug, Clone)]
pub enum PruningPolicy {
    RelevanceFirst,    // Remove lowest relevance first
    RecencyFirst,      // Remove least recently accessed
    RedundancyFirst,  // Remove redundant content
    Strategic,        // Strategic removal based on structure
}
#[derive(Debug, Clone)]
pub enum CompressionPolicy {
    None,              // No compression
    Summarize,         // Summarize less important blocks
    Truncate,          // Truncate long content
    Hybrid,            // Mix of strategies
}
#[derive(Debug, Clone)]
pub struct ContextCache {
    relevance_cache: HashMap<BlockId, f32>,
    semantic_cache: HashMap<BlockId, Vec<BlockId>>,
    path_cache: HashMap<(BlockId, BlockId), Vec<BlockId>>,
}
impl ContextManager {
    pub fn new(document: Document, constraints: ContextConstraints) -> Self {
        let window = ContextWindow {
            id: format!("ctx_{}", chrono::Utc::now().timestamp()),
            blocks: HashMap::new(),
            relationships: Vec::new(),
            metadata: ContextMetadata {
                created_at: chrono::Utc::now(),
                last_modified: chrono::Utc::now(),
                total_tokens: 0,
                focus_area: None,
                navigation_history: Vec::new(),
                compression_ratio: 1.0,
                quality_score: 0.0,
            },
            constraints,
        };
        Self {
            window,
            document,
            analyzer: ContextAnalyzer::new(),
            strategy: ContextStrategy::default(),
            cache: ContextCache::new(),
        }
    }
    /// Initialize context with focus area
    pub fn initialize_focus(&mut self, focus_id: BlockId, task_description: &str) -> Result<()> {
        // Add focus block
        self.add_block_to_context(focus_id, InclusionReason::DirectReference, task_description)?;
        // Add immediate structural context
        self.add_structural_context(focus_id, 2)?; // 2 levels up/down
        // Add semantic neighbors
        self.add_semantic_context(focus_id, task_description)?;
        // Update metadata
        self.window.metadata.focus_area = Some(focus_id);
        self.record_navigation(NavigationAction::MoveFocus, Some(focus_id), "Initial focus");
        Ok(())
    }
    /// Navigate to a new focus area
    pub fn navigate_to(&mut self, target_id: BlockId, task_description: &str) -> Result<ContextUpdateResult> {
        let old_focus = self.window.metadata.focus_area;
        
        // Update focus
        self.window.metadata.focus_area = Some(target_id);
        
        // Add new focus block
        self.add_block_to_context(target_id, InclusionReason::NavigationPath, task_description)?;
        
        // Add context around new focus
        self.add_structural_context(target_id, 2)?;
        self.add_semantic_context(target_id, task_description)?;
        
        // Prune if needed
        let pruned = self.prune_if_needed()?;
        
        // Record navigation
        self.record_navigation(NavigationAction::MoveFocus, Some(target_id), 
                             format!("Navigated from {:?}", old_focus));
        Ok(ContextUpdateResult {
            added_blocks: 1,
            removed_blocks: pruned.len(),
            tokens_changed: self.calculate_token_change(),
            quality_impact: self.calculate_quality_impact(),
        })
    }
    /// Expand context around current focus
    pub fn expand_context(&mut self, direction: ExpandDirection, depth: usize) -> Result<ContextUpdateResult> {
        let focus_id = self.window.metadata.focus_area
            .ok_or_else(|| Error::NoFocusArea)?;
        let added_before = self.window.blocks.len();
        
        match direction {
            ExpandDirection::Down => {
                self.expand_downward(focus_id, depth)?;
            }
            ExpandDirection::Up => {
                self.expand_upward(focus_id, depth)?;
            }
            ExpandDirection::Both => {
                self.expand_downward(focus_id, depth)?;
                self.expand_upward(focus_id, depth)?;
            }
            ExpandDirection::Semantic => {
                self.expand_semantic(focus_id, depth)?;
            }
        }
        // Prune if over limits
        let pruned = self.prune_if_needed()?;
        self.record_navigation(NavigationAction::ExpandNode, Some(focus_id), 
                             format!("Expanded {:?} depth {}", direction, depth));
        Ok(ContextUpdateResult {
            added_blocks: self.window.blocks.len() - added_before,
            removed_blocks: pruned.len(),
            tokens_changed: self.calculate_token_change(),
            quality_impact: self.calculate_quality_impact(),
        })
    }
    /// Add specific block to context
    pub fn add_block(&mut self, block_id: BlockId, reason: InclusionReason, rationale: &str) -> Result<()> {
        self.add_block_to_context(block_id, reason, rationale)?;
        self.record_navigation(NavigationAction::AddBlock, Some(block_id), rationale);
        Ok(())
    }
    /// Remove block from context
    pub fn remove_block(&mut self, block_id: BlockId, rationale: &str) -> Result<()> {
        if self.window.blocks.remove(&block_id).is_some() {
            // Remove related relationships
            self.window.relationships.retain(|r| r.source != block_id && r.target != block_id);
            
            self.record_navigation(NavigationAction::RemoveBlock, Some(block_id), rationale);
        }
        Ok(())
    }
    /// Search and add relevant blocks
    pub fn search_and_add(&mut self, query: &str, max_results: usize) -> Result<Vec<BlockId>> {
        let results = self.analyzer.semantic_analyzer.search(&self.document, query, max_results)?;
        let mut added = Vec::new();
        for (block_id, relevance) in results {
            if self.can_add_block(block_id, relevance) {
                self.add_block_to_context(block_id, InclusionReason::AgentDecision, 
                                         format!("Search result for: {}", query))?;
                added.push(block_id);
            }
        }
        // Prune if needed
        self.prune_if_needed()?;
        Ok(added)
    }
    /// Compress context to fit constraints
    pub fn compress_context(&mut self) -> Result<CompressionResult> {
        if !self.window.constraints.allow_compression {
            return Ok(CompressionResult::no_change());
        }
        let original_tokens = self.window.metadata.total_tokens;
        let mut compressed_blocks = Vec::new();
        match self.strategy.compression_policy {
            CompressionPolicy::Summarize => {
                compressed_blocks = self.summarize_blocks()?;
            }
            CompressionPolicy::Truncate => {
                compressed_blocks = self.truncate_blocks()?;
            }
            CompressionPolicy::Hybrid => {
                compressed_blocks = self.hybrid_compression()?;
            }
            CompressionPolicy::None => {
                return Ok(CompressionResult::no_change());
            }
        }
        let compression_ratio = original_tokens as f32 / self.window.metadata.total_tokens as f32;
        self.window.metadata.compression_ratio = compression_ratio;
        Ok(CompressionResult {
            original_tokens,
            compressed_tokens: self.window.metadata.total_tokens,
            compression_ratio,
            compressed_blocks,
        })
    }
    /// Get current context as prompt
    pub fn build_prompt(&self) -> String {
        let mut sections = Vec::new();
        // Add context header
        sections.push("## Context Window".to_string());
        sections.push(format!("Blocks: {} | Tokens: {} | Focus: {:?}", 
                            self.window.blocks.len(), 
                            self.window.metadata.total_tokens,
                            self.window.metadata.focus_area));
        // Add navigation history if recent
        if !self.window.metadata.navigation_history.is_empty() {
            sections.push("## Recent Navigation".to_string());
            for step in self.window.metadata.navigation_history.iter().rev().take(3) {
                sections.push(format!("• {:?} {} ({})", 
                                    step.action, 
                                    step.block_id.map(|id| id.to_string()).unwrap_or_else(|| "None".to_string()),
                                    step.rationale));
            }
        }
        // Add context blocks
        sections.push("## Context Blocks".to_string());
        
        // Sort by relevance and position
        let mut sorted_blocks: Vec<_> = self.window.blocks.values().collect();
        sorted_blocks.sort_by(|a, b| {
            b.relevance_score.partial_cmp(&a.relevance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        for context_block in sorted_blocks {
            let block = &context_block.block;
            let position = context_block.context_position.as_ref();
            
            let header = format!("### {} ({})", 
                                block.id,
                                match context_block.inclusion_reason {
                                    InclusionReason::DirectReference => "direct",
                                    InclusionReason::SemanticRelevance => "semantic",
                                    InclusionReason::StructuralContext => "structure",
                                    InclusionReason::NavigationPath => "navigation",
                                    InclusionReason::UserPreference => "user",
                                    InclusionReason::AgentDecision => "agent",
                                    InclusionReason::SystemRequired => "system",
                                });
            sections.push(header);
            
            // Add content preview
            let content_preview = self.format_content_preview(&block.content);
            sections.push(format!("**Content:** {}", content_preview));
            
            // Add metadata
            if let Some(role) = &block.metadata.semantic_role {
                sections.push(format!("**Role:** {}", role.category.as_str()));
            }
            
            if !block.metadata.tags.is_empty() {
                sections.push(format!("**Tags:** {}", block.metadata.tags.join(", ")));
            }
            
            sections.push(format!("**Relevance:** {:.2}", context_block.relevance_score));
            
            if let Some(pos) = position {
                sections.push(format!("**Position:** Level {} | Sequence {}", pos.hierarchy_level, pos.sequence));
            }
            
            sections.push(String::new());
        }
        sections.join("\n")
    }
    /// Get context statistics
    pub fn get_statistics(&self) -> ContextStatistics {
        let role_counts = self.count_roles();
        let tag_counts = self.count_tags();
        let depth_distribution = self.calculate_depth_distribution();
        ContextStatistics {
            total_blocks: self.window.blocks.len(),
            total_tokens: self.window.metadata.total_tokens,
            focus_area: self.window.metadata.focus_area,
            average_relevance: self.calculate_average_relevance(),
            role_distribution: role_counts,
            tag_distribution: tag_counts,
            depth_distribution,
            compression_ratio: self.window.metadata.compression_ratio,
            quality_score: self.window.metadata.quality_score,
        }
    }
    // Private helper methods
    fn add_block_to_context(&mut self, block_id: BlockId, reason: InclusionReason, rationale: &str) -> Result<()> {
        // Check if already in context
        if self.window.blocks.contains_key(&block_id) {
            return Ok(());
        }
        // Get block from document
        let block = self.document.get_block(&block_id)
            .ok_or_else(|| Error::BlockNotFound(block_id.to_string()))?;
        // Calculate relevance
        let relevance = self.calculate_relevance(&block, &reason, rationale)?;
        // Check constraints
        if !self.can_add_block(block_id, relevance) {
            return Err(Error::ContextFull("Cannot add block: constraints violated".to_string()));
        }
        // Create context block
        let context_block = ContextBlock {
            block: block.clone(),
            relevance_score: relevance,
            inclusion_reason: reason,
            last_accessed: chrono::Utc::now(),
            access_count: 1,
            context_position: self.calculate_context_position(&block),
        };
        // Add to window
        self.window.blocks.insert(block_id, context_block);
        // Update relationships
        self.update_relationships(block_id);
        // Update metadata
        self.window.metadata.total_tokens += self.estimate_tokens(&block);
        self.window.metadata.last_modified = chrono::Utc::now();
        Ok(())
    }
    fn add_structural_context(&mut self, focus_id: BlockId, levels: usize) -> Result<()> {
        // Add parents
        let mut current = focus_id;
        for _ in 0..levels {
            if let Some(parent) = self.document.parent(&current) {
                self.add_block_to_context(*parent, InclusionReason::StructuralContext, "Parent context")?;
                current = *parent;
            } else {
                break;
            }
        }
        // Add children
        let children = self.document.children(&focus_id);
        for child in children {
            self.add_block_to_context(child, InclusionReason::StructuralContext, "Child context")?;
        }
        Ok(())
    }
    fn add_semantic_context(&mut self, focus_id: BlockId, task_description: &str) -> Result<()> {
        // Find semantically related blocks
        let related = self.analyzer.semantic_analyzer.find_related_blocks(&self.document, focus_id, task_description)?;
        for (related_id, relevance) in related {
            if relevance > 0.5 && self.can_add_block(related_id, relevance) {
                self.add_block_to_context(related_id, InclusionReason::SemanticRelevance, 
                                         format!("Semantic relevance: {:.2}", relevance))?;
            }
        }
        Ok(())
    }
    fn expand_downward(&mut self, focus_id: BlockId, depth: usize) -> Result<()> {
        let mut queue = VecDeque::new();
        queue.push_back((focus_id, 0));
        while let Some((current_id, current_depth)) = queue.pop_front() {
            if current_depth >= depth {
                continue;
            }
            let children = self.document.children(&current_id);
            for child_id in children {
                if !self.window.blocks.contains_key(&child_id) {
                    self.add_block_to_context(child_id, InclusionReason::NavigationPath, 
                                             format!("Expansion depth {}", current_depth + 1))?;
                    queue.push_back((child_id, current_depth + 1));
                }
            }
        }
        Ok(())
    }
    fn expand_upward(&mut self, focus_id: BlockId, depth: usize) -> Result<()> {
        let mut current = focus_id;
        for level in 1..=depth {
            if let Some(parent) = self.document.parent(&current) {
                if !self.window.blocks.contains_key(parent) {
                    self.add_block_to_context(*parent, InclusionReason::NavigationPath, 
                                             format!("Expansion level {}", level))?;
                }
                current = *parent;
            } else {
                break;
            }
        }
        Ok(())
    }
    fn expand_semantic(&mut self, focus_id: BlockId, depth: usize) -> Result<()> {
        let related = self.analyzer.semantic_analyzer.find_related_blocks(&self.document, focus_id, "")?;
        
        for (related_id, relevance) in related.iter().take(depth) {
            if !self.window.blocks.contains_key(related_id) {
                self.add_block_to_context(*related_id, InclusionReason::SemanticRelevance, 
                                         format!("Semantic expansion: {:.2}", relevance))?;
            }
        }
        Ok(())
    }
    fn prune_if_needed(&mut self) -> Result<Vec<BlockId>> {
        let mut pruned = Vec::new();
        // Check token constraint
        while self.window.metadata.total_tokens > self.window.constraints.max_tokens {
            if let Some(to_remove) = self.select_block_to_remove() {
                self.window.blocks.remove(&to_remove);
                pruned.push(to_remove);
            } else {
                break;
            }
        }
        // Check block constraint
        while self.window.blocks.len() > self.window.constraints.max_blocks {
            if let Some(to_remove) = self.select_block_to_remove() {
                self.window.blocks.remove(&to_remove);
                pruned.push(to_remove);
            } else {
                break;
            }
        }
        // Update token count
        self.window.metadata.total_tokens = self.window.blocks.values()
            .map(|cb| self.estimate_tokens(&cb.block))
            .sum();
        Ok(pruned)
    }
    fn select_block_to_remove(&self) -> Option<BlockId> {
        match self.strategy.pruning_policy {
            PruningPolicy::RelevanceFirst => {
                self.window.blocks.iter()
                    .min_by(|a, b| a.1.relevance_score.partial_cmp(&b.1.relevance_score).unwrap())
                    .map(|(id, _)| *id)
            }
            PruningPolicy::RecencyFirst => {
                self.window.blocks.iter()
                    .min_by(|a, b| a.1.last_accessed.cmp(&b.1.last_accessed))
                    .map(|(id, _)| *id)
            }
            PruningPolicy::RedundancyFirst => {
                // Find most redundant block
                self.find_most_redundant_block()
            }
            PruningPolicy::Strategic => {
                self.select_strategic_removal()
            }
        }
    }
    fn can_add_block(&self, block_id: BlockId, relevance: f32) -> bool {
        // Check if already present
        if self.window.blocks.contains_key(&block_id) {
            return false;
        }
        // Check relevance threshold
        if relevance < self.window.constraints.min_relevance {
            return false;
        }
        // Check constraints (with some flexibility)
        let would_exceed_tokens = self.window.metadata.total_tokens + self.estimate_block_tokens(block_id) > self.window.constraints.max_tokens;
        let would_exceed_blocks = self.window.blocks.len() >= self.window.constraints.max_blocks;
        !would_exceed_tokens && !would_exceed_blocks
    }
    fn calculate_relevance(&self, block: &Block, reason: &InclusionReason, rationale: &str) -> Result<f32> {
        let base_score = match reason {
            InclusionReason::DirectReference => 1.0,
            InclusionReason::SemanticRelevance => 0.8,
            InclusionReason::StructuralContext => 0.6,
            InclusionReason::NavigationPath => 0.7,
            InclusionReason::UserPreference => 0.9,
            InclusionReason::AgentDecision => 0.5,
            InclusionReason::SystemRequired => 1.0,
        };
        // Apply semantic analysis
        let semantic_score = self.analyzer.relevance_scorer.score_block(block, rationale)?;
        // Apply position-based scoring
        let position_score = self.calculate_position_score(block);
        Ok((base_score + semantic_score + position_score) / 3.0)
    }
    fn record_navigation(&mut self, action: NavigationAction, block_id: Option<BlockId>, rationale: String) {
        let step = NavigationStep {
            timestamp: chrono::Utc::now(),
            action,
            block_id,
            rationale,
        };
        self.window.metadata.navigation_history.push(step);
        
        // Keep only recent history
        if self.window.metadata.navigation_history.len() > 50 {
            self.window.metadata.navigation_history.remove(0);
        }
    }
    // Additional helper methods would be implemented here...
    // (format_content_preview, estimate_tokens, calculate_position_score, etc.)
}
/// Direction for context expansion
#[derive(Debug, Clone)]
pub enum ExpandDirection {
    Down,       // Expand to children
    Up,         // Expand to parents
    Both,       // Expand both directions
    Semantic,   // Expand to semantically related blocks
}
/// Result of context update operation
#[derive(Debug, Clone)]
pub struct ContextUpdateResult {
    pub added_blocks: usize,
    pub removed_blocks: usize,
    pub tokens_changed: isize,
    pub quality_impact: f32,
}
/// Result of compression operation
#[derive(Debug, Clone)]
pub struct CompressionResult {
    pub original_tokens: usize,
    pub compressed_tokens: usize,
    pub compression_ratio: f32,
    pub compressed_blocks: Vec<BlockId>,
}
impl CompressionResult {
    pub fn no_change() -> Self {
        Self {
            original_tokens: 0,
            compressed_tokens: 0,
            compression_ratio: 1.0,
            compressed_blocks: Vec::new(),
        }
    }
}
/// Context statistics
#[derive(Debug, Clone)]
pub struct ContextStatistics {
    pub total_blocks: usize,
    pub total_tokens: usize,
    pub focus_area: Option<BlockId>,
    pub average_relevance: f32,
    pub role_distribution: HashMap<String, usize>,
    pub tag_distribution: HashMap<String, usize>,
    pub depth_distribution: HashMap<usize, usize>,
    pub compression_ratio: f32,
    pub quality_score: f32,
}
/// Context analysis components
#[derive(Debug, Clone)]
pub struct ContextAnalyzer {
    pub relevance_scorer: RelevanceScorer,
    pub semantic_analyzer: SemanticAnalyzer,
    pub structure_analyzer: StructureAnalyzer,
}
impl ContextAnalyzer {
    pub fn new() -> Self {
        Self {
            relevance_scorer: RelevanceScorer::new(),
            semantic_analyzer: SemanticAnalyzer::new(),
            structure_analyzer: StructureAnalyzer::new(),
        }
    }
}
/// Relevance scoring for blocks
#[derive(Debug, Clone)]
pub struct RelevanceScorer {
    // Implementation would include TF-IDF, embeddings, etc.
}
impl RelevanceScorer {
    pub fn new() -> Self {
        Self {}
    }
    pub fn score_block(&self, block: &Block, query: &str) -> Result<f32> {
        // Implement relevance scoring logic
        Ok(0.5) // Placeholder
    }
}
/// Semantic analysis for finding related blocks
#[derive(Debug, Clone)]
pub struct SemanticAnalyzer {
    // Implementation would include semantic similarity, embeddings, etc.
}
impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
    pub fn find_related_blocks(&self, document: &Document, block_id: BlockId, query: &str) -> Result<Vec<(BlockId, f32)>> {
        // Implement semantic search logic
        Ok(Vec::new()) // Placeholder
    }
    pub fn search(&self, document: &Document, query: &str, max_results: usize) -> Result<Vec<(BlockId, f32)>> {
        // Implement search logic
        Ok(Vec::new()) // Placeholder
    }
}
/// Structure analysis for context positioning
#[derive(Debug, Clone)]
pub struct StructureAnalyzer {
    // Implementation would include hierarchy analysis, path finding, etc.
}
impl StructureAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}
/// Context caching for performance
#[derive(Debug, Clone)]
pub struct ContextCache {
    pub relevance_cache: HashMap<BlockId, f32>,
    pub semantic_cache: HashMap<BlockId, Vec<BlockId>>,
    pub path_cache: HashMap<(BlockId, BlockId), Vec<BlockId>>,
}
impl ContextCache {
    pub fn new() -> Self {
        Self {
            relevance_cache: HashMap::new(),
            semantic_cache: HashMap::new(),
            path_cache: HashMap::new(),
        }
    }
}
/// Default strategy
impl Default for ContextStrategy {
    fn default() -> Self {
        Self {
            expansion_policy: ExpansionPolicy::Balanced,
            pruning_policy: PruningPolicy::RelevanceFirst,
            compression_policy: CompressionPolicy::Hybrid,
        }
    }
}
/// Error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Block not found: {0}")]
    BlockNotFound(String),
    
    #[error("No focus area set")]
    NoFocusArea,
    
    #[error("Context constraint violation: {0}")]
    ContextFull(String),
    
    #[error("Analysis error: {0}")]
    AnalysisError(String),
}
pub type Result<T> = std::result::Result<T, Error>;
2. UCL Commands for Context Management
Add to crates/ucl-parser/src/ast.rs:

rust
/// Context management commands
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextCommand {
    pub action: ContextAction,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContextAction {
    /// Initialize context with focus
    Initialize {
        focus_id: String,
        task_description: String,
        constraints: Option<ContextConstraints>,
    },
    
    /// Navigate to new focus
    Navigate {
        target_id: String,
        task_description: String,
    },
    
    /// Expand context
    Expand {
        direction: String,  // "up", "down", "both", "semantic"
        depth: usize,
    },
    
    /// Add specific block
    Add {
        block_id: String,
        reason: String,  // "direct", "semantic", "structure", etc.
    },
    
    /// Remove block
    Remove {
        block_id: String,
        rationale: String,
    },
    
    /// Search and add
    Search {
        query: String,
        max_results: usize,
    },
    
    /// Compress context
    Compress {
        method: String,  // "summarize", "truncate", "hybrid"
    },
    
    /// Get context statistics
    Stats,
    
    /// Show current context
    Show,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextConstraints {
    pub max_tokens: Option<usize>,
    pub max_blocks: Option<usize>,
    pub max_depth: Option<usize>,
    pub min_relevance: Option<f32>,
    pub required_roles: Vec<String>,
    pub excluded_tags: Vec<String>,
}
3. Context Management Engine
Add to crates/ucm-engine/src/operation.rs:

rust
/// Context management operation
Context {
    action: ContextAction,
},
4. Usage Examples
ucl
# Initialize context with focus on introduction
CONTEXT INITIALIZE focus=blk_intro123 task="Explain machine learning concepts" constraints.max_tokens=4000
 
# Navigate to algorithms section
CONTEXT NAVIGATE target=blk_algo456 task="Focus on specific algorithms"
 
# Expand context downward 2 levels
CONTEXT EXPAND direction=down depth=2
 
# Search and add relevant blocks
CONTEXT SEARCH query="neural networks" max_results=5
 
# Add specific block
CONTEXT ADD block=blk_example789 reason=direct
 
# Remove less relevant block
CONTEXT REMOVE block=blk_background321 rationale="Not relevant to current task"
 
# Compress context to fit token limits
CONTEXT COMPRESS method=hybrid
 
# Get context statistics
CONTEXT STATS
 
# Show current context
CONTEXT SHOW
5. Integration with LLM Workflows
rust
// Example LLM workflow with context management
let mut context_manager = ContextManager::new(document, ContextConstraints {
    max_tokens: 4000,
    max_blocks: 50,
    max_depth: 5,
    min_relevance: 0.3,
    required_roles: vec!["heading1".to_string(), "paragraph".to_string()],
    excluded_tags: vec!["draft".to_string()],
    preserve_structure: true,
    allow_compression: true,
});
// Initialize with task focus
context_manager.initialize_focus(intro_block_id, "Explain deep learning concepts")?;
// Build initial prompt
let prompt = context_manager.build_prompt();
// Send to LLM, get response
let llm_response = send_to_llm(prompt)?;
// Parse LLM commands, navigate as needed
for command in parse_ucl_commands(&llm_response)? {
    match command {
        Command::Context(ctx_cmd) => {
            context_manager.execute_context_command(ctx_cmd)?;
        }
        // Handle other commands...
    }
}
// Get updated context for next iteration
let updated_prompt = context_manager.build_prompt();
Key Benefits
Intelligent Navigation: Automatically adds relevant structural and semantic context
Dynamic Pruning: Removes less relevant content when constraints are reached
Adaptive Strategies: Different expansion/pruning policies for different use cases
Semantic Understanding: Goes beyond simple structure to find semantically related content
Performance Optimization: Caching and efficient algorithms for large documents
LLM Integration: Seamless integration with existing UCL command flow
Context Awareness: Maintains navigation history and reasoning for transparency
Flexible Constraints: Configurable token, block, and relevance limits
This context management system provides LLM agents with the ability to intelligently navigate and curate knowledge graphs, maintaining optimal context windows while preserving the most relevant information for any given task.