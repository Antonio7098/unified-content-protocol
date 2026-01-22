//! Graph traversal operations for UCM documents.
//!
//! This module provides various traversal algorithms and utilities for
//! navigating the document's block structure, including BFS, DFS,
//! and semantic traversal.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use ucm_core::{Block, BlockId, Content, Document, EdgeType};

use crate::error::Result;

/// Direction for navigation operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NavigateDirection {
    /// Navigate to children only
    Down,
    /// Navigate to parent only
    Up,
    /// Navigate both up and down
    Both,
    /// Navigate to siblings
    Siblings,
    /// Breadth-first traversal
    BreadthFirst,
    /// Depth-first traversal
    DepthFirst,
}

/// Output format for traversal results
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TraversalOutput {
    /// Only return structure (block IDs and relationships)
    StructureOnly,
    /// Return structure and full block content
    #[default]
    StructureAndBlocks,
    /// Return structure with content previews
    StructureWithPreviews,
}

/// Filter criteria for traversal
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TraversalFilter {
    /// Only include blocks with these semantic roles
    pub include_roles: Vec<String>,
    /// Exclude blocks with these semantic roles
    pub exclude_roles: Vec<String>,
    /// Only include blocks with these tags
    pub include_tags: Vec<String>,
    /// Exclude blocks with these tags
    pub exclude_tags: Vec<String>,
    /// Only include blocks matching content pattern
    pub content_pattern: Option<String>,
    /// Follow edge types (for edge-based traversal)
    pub edge_types: Vec<EdgeType>,
}

/// A node in the traversal result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalNode {
    pub id: BlockId,
    pub depth: usize,
    pub parent_id: Option<BlockId>,
    pub content_preview: Option<String>,
    pub semantic_role: Option<String>,
    pub child_count: usize,
    pub edge_count: usize,
}

/// An edge in the traversal result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalEdge {
    pub source: BlockId,
    pub target: BlockId,
    pub edge_type: EdgeType,
}

/// Summary statistics for a traversal
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TraversalSummary {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub max_depth: usize,
    pub nodes_by_role: HashMap<String, usize>,
    pub truncated: bool,
    pub truncation_reason: Option<String>,
}

/// Metadata about the traversal operation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TraversalMetadata {
    pub start_id: Option<BlockId>,
    pub direction: Option<NavigateDirection>,
    pub max_depth: Option<usize>,
    pub execution_time_ms: Option<u64>,
}

/// Complete traversal result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalResult {
    pub nodes: Vec<TraversalNode>,
    pub edges: Vec<TraversalEdge>,
    pub paths: Vec<Vec<BlockId>>,
    pub summary: TraversalSummary,
    pub metadata: TraversalMetadata,
}

impl TraversalResult {
    pub fn empty() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            paths: Vec::new(),
            summary: TraversalSummary::default(),
            metadata: TraversalMetadata::default(),
        }
    }
}

/// Configuration for the traversal engine
#[derive(Debug, Clone)]
pub struct TraversalConfig {
    pub max_depth: usize,
    pub max_nodes: usize,
    pub default_preview_length: usize,
    pub include_orphans: bool,
    pub cache_enabled: bool,
}

impl Default for TraversalConfig {
    fn default() -> Self {
        Self {
            max_depth: 100,
            max_nodes: 10000,
            default_preview_length: 100,
            include_orphans: false,
            cache_enabled: true,
        }
    }
}

/// Graph traversal engine for UCM documents
pub struct TraversalEngine {
    config: TraversalConfig,
}

impl TraversalEngine {
    /// Create a new traversal engine with default configuration
    pub fn new() -> Self {
        Self {
            config: TraversalConfig::default(),
        }
    }

    /// Create a traversal engine with custom configuration
    pub fn with_config(config: TraversalConfig) -> Self {
        Self { config }
    }

    /// Navigate from a starting point in a specific direction
    pub fn navigate(
        &self,
        doc: &Document,
        start_id: Option<BlockId>,
        direction: NavigateDirection,
        depth: Option<usize>,
        filter: Option<TraversalFilter>,
        output: TraversalOutput,
    ) -> Result<TraversalResult> {
        let start = start_id.unwrap_or(doc.root);
        let max_depth = depth
            .unwrap_or(self.config.max_depth)
            .min(self.config.max_depth);
        let filter = filter.unwrap_or_default();

        #[cfg(not(target_arch = "wasm32"))]
        let start_time = std::time::Instant::now();

        let result = match direction {
            NavigateDirection::Down => self.traverse_down(doc, start, max_depth, &filter, output),
            NavigateDirection::Up => self.traverse_up(doc, start, max_depth, &filter, output),
            NavigateDirection::Both => self.traverse_both(doc, start, max_depth, &filter, output),
            NavigateDirection::Siblings => self.traverse_siblings(doc, start, &filter, output),
            NavigateDirection::BreadthFirst => {
                self.traverse_bfs(doc, start, max_depth, &filter, output)
            }
            NavigateDirection::DepthFirst => {
                self.traverse_dfs(doc, start, max_depth, &filter, output)
            }
        }?;

        let mut result = result;
        result.metadata.start_id = Some(start);
        result.metadata.direction = Some(direction);
        result.metadata.max_depth = Some(max_depth);

        #[cfg(not(target_arch = "wasm32"))]
        {
            result.metadata.execution_time_ms = Some(start_time.elapsed().as_millis() as u64);
        }

        Ok(result)
    }

    /// Expand a node to get its immediate children
    pub fn expand(
        &self,
        doc: &Document,
        node_id: &BlockId,
        output: TraversalOutput,
    ) -> Result<TraversalResult> {
        self.navigate(
            doc,
            Some(*node_id),
            NavigateDirection::Down,
            Some(1),
            None,
            output,
        )
    }

    /// Get the path from a node to the root
    pub fn path_to_root(&self, doc: &Document, node_id: &BlockId) -> Result<Vec<BlockId>> {
        let mut path = vec![*node_id];
        let mut current = *node_id;

        while let Some(parent) = doc.parent(&current) {
            path.push(*parent);
            if *parent == doc.root {
                break;
            }
            current = *parent;
        }

        path.reverse();
        Ok(path)
    }

    /// Find all paths between two nodes
    pub fn find_paths(
        &self,
        doc: &Document,
        from: &BlockId,
        to: &BlockId,
        max_paths: usize,
    ) -> Result<Vec<Vec<BlockId>>> {
        let mut paths = Vec::new();
        let mut visited = HashSet::new();
        let mut current_path = vec![*from];

        self.find_paths_recursive(
            doc,
            from,
            to,
            &mut visited,
            &mut current_path,
            &mut paths,
            max_paths,
        )?;

        Ok(paths)
    }

    #[allow(clippy::too_many_arguments)]
    fn find_paths_recursive(
        &self,
        doc: &Document,
        current: &BlockId,
        target: &BlockId,
        visited: &mut HashSet<BlockId>,
        current_path: &mut Vec<BlockId>,
        paths: &mut Vec<Vec<BlockId>>,
        max_paths: usize,
    ) -> Result<()> {
        if paths.len() >= max_paths {
            return Ok(());
        }

        if current == target {
            paths.push(current_path.clone());
            return Ok(());
        }

        visited.insert(*current);

        // Check children
        for child in doc.children(current) {
            if !visited.contains(child) {
                current_path.push(*child);
                self.find_paths_recursive(
                    doc,
                    child,
                    target,
                    visited,
                    current_path,
                    paths,
                    max_paths,
                )?;
                current_path.pop();
            }
        }

        // Check edges
        if let Some(block) = doc.get_block(current) {
            for edge in &block.edges {
                if !visited.contains(&edge.target) {
                    current_path.push(edge.target);
                    self.find_paths_recursive(
                        doc,
                        &edge.target,
                        target,
                        visited,
                        current_path,
                        paths,
                        max_paths,
                    )?;
                    current_path.pop();
                }
            }
        }

        visited.remove(current);
        Ok(())
    }

    /// Traverse downward from a starting node
    fn traverse_down(
        &self,
        doc: &Document,
        start: BlockId,
        max_depth: usize,
        filter: &TraversalFilter,
        output: TraversalOutput,
    ) -> Result<TraversalResult> {
        self.traverse_bfs(doc, start, max_depth, filter, output)
    }

    /// Traverse upward from a starting node
    fn traverse_up(
        &self,
        doc: &Document,
        start: BlockId,
        max_depth: usize,
        filter: &TraversalFilter,
        output: TraversalOutput,
    ) -> Result<TraversalResult> {
        let mut nodes = Vec::new();
        let mut current = start;
        let mut depth = 0;

        while depth <= max_depth {
            if let Some(block) = doc.get_block(&current) {
                if self.matches_filter(block, filter) {
                    nodes.push(self.create_traversal_node(doc, &current, depth, None, output));
                }
            }

            if let Some(parent) = doc.parent(&current) {
                current = *parent;
                depth += 1;
            } else {
                break;
            }
        }

        let summary = TraversalSummary {
            total_nodes: nodes.len(),
            max_depth: depth,
            ..Default::default()
        };

        Ok(TraversalResult {
            nodes,
            edges: Vec::new(),
            paths: Vec::new(),
            summary,
            metadata: TraversalMetadata::default(),
        })
    }

    /// Traverse both up and down from a starting node
    fn traverse_both(
        &self,
        doc: &Document,
        start: BlockId,
        max_depth: usize,
        filter: &TraversalFilter,
        output: TraversalOutput,
    ) -> Result<TraversalResult> {
        let up_result = self.traverse_up(doc, start, max_depth, filter, output)?;
        let down_result = self.traverse_down(doc, start, max_depth, filter, output)?;

        // Merge results, avoiding duplicates
        let mut seen = HashSet::new();
        let mut nodes = Vec::new();

        for node in up_result
            .nodes
            .into_iter()
            .chain(down_result.nodes.into_iter())
        {
            if seen.insert(node.id) {
                nodes.push(node);
            }
        }

        let max_depth = nodes.iter().map(|n| n.depth).max().unwrap_or(0);
        let summary = TraversalSummary {
            total_nodes: nodes.len(),
            max_depth,
            ..Default::default()
        };

        Ok(TraversalResult {
            nodes,
            edges: Vec::new(),
            paths: Vec::new(),
            summary,
            metadata: TraversalMetadata::default(),
        })
    }

    /// Traverse siblings of a node
    fn traverse_siblings(
        &self,
        doc: &Document,
        start: BlockId,
        filter: &TraversalFilter,
        output: TraversalOutput,
    ) -> Result<TraversalResult> {
        let mut nodes = Vec::new();

        if let Some(parent) = doc.parent(&start) {
            for sibling in doc.children(parent) {
                if let Some(block) = doc.get_block(sibling) {
                    if self.matches_filter(block, filter) {
                        nodes.push(self.create_traversal_node(
                            doc,
                            sibling,
                            0,
                            Some(*parent),
                            output,
                        ));
                    }
                }
            }
        }

        let summary = TraversalSummary {
            total_nodes: nodes.len(),
            max_depth: 0,
            ..Default::default()
        };

        Ok(TraversalResult {
            nodes,
            edges: Vec::new(),
            paths: Vec::new(),
            summary,
            metadata: TraversalMetadata::default(),
        })
    }

    /// Breadth-first traversal
    fn traverse_bfs(
        &self,
        doc: &Document,
        start: BlockId,
        max_depth: usize,
        filter: &TraversalFilter,
        output: TraversalOutput,
    ) -> Result<TraversalResult> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut nodes_by_role: HashMap<String, usize> = HashMap::new();

        queue.push_back((start, None::<BlockId>, 0usize));

        while let Some((node_id, parent_id, depth)) = queue.pop_front() {
            if depth > max_depth || nodes.len() >= self.config.max_nodes {
                break;
            }

            if visited.contains(&node_id) {
                continue;
            }
            visited.insert(node_id);

            if let Some(block) = doc.get_block(&node_id) {
                if self.matches_filter(block, filter) {
                    let node = self.create_traversal_node(doc, &node_id, depth, parent_id, output);

                    if let Some(role) = &node.semantic_role {
                        *nodes_by_role.entry(role.clone()).or_insert(0) += 1;
                    }

                    nodes.push(node);

                    // Collect edges
                    for edge in &block.edges {
                        if filter.edge_types.is_empty()
                            || filter.edge_types.contains(&edge.edge_type)
                        {
                            edges.push(TraversalEdge {
                                source: node_id,
                                target: edge.target,
                                edge_type: edge.edge_type.clone(),
                            });
                        }
                    }
                }

                // Add children to queue
                for child in doc.children(&node_id) {
                    if !visited.contains(child) {
                        queue.push_back((*child, Some(node_id), depth + 1));
                    }
                }
            }
        }

        let max_depth_found = nodes.iter().map(|n| n.depth).max().unwrap_or(0);
        let truncated = nodes.len() >= self.config.max_nodes;

        let summary = TraversalSummary {
            total_nodes: nodes.len(),
            total_edges: edges.len(),
            max_depth: max_depth_found,
            nodes_by_role,
            truncated,
            truncation_reason: if truncated {
                Some(format!(
                    "Max nodes limit ({}) reached",
                    self.config.max_nodes
                ))
            } else {
                None
            },
        };

        Ok(TraversalResult {
            nodes,
            edges,
            paths: Vec::new(),
            summary,
            metadata: TraversalMetadata::default(),
        })
    }

    /// Depth-first traversal
    fn traverse_dfs(
        &self,
        doc: &Document,
        start: BlockId,
        max_depth: usize,
        filter: &TraversalFilter,
        output: TraversalOutput,
    ) -> Result<TraversalResult> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut visited = HashSet::new();
        let mut nodes_by_role: HashMap<String, usize> = HashMap::new();

        self.dfs_recursive(
            doc,
            start,
            None,
            0,
            max_depth,
            filter,
            output,
            &mut visited,
            &mut nodes,
            &mut edges,
            &mut nodes_by_role,
        )?;

        let max_depth_found = nodes.iter().map(|n| n.depth).max().unwrap_or(0);
        let truncated = nodes.len() >= self.config.max_nodes;

        let summary = TraversalSummary {
            total_nodes: nodes.len(),
            total_edges: edges.len(),
            max_depth: max_depth_found,
            nodes_by_role,
            truncated,
            truncation_reason: if truncated {
                Some(format!(
                    "Max nodes limit ({}) reached",
                    self.config.max_nodes
                ))
            } else {
                None
            },
        };

        Ok(TraversalResult {
            nodes,
            edges,
            paths: Vec::new(),
            summary,
            metadata: TraversalMetadata::default(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn dfs_recursive(
        &self,
        doc: &Document,
        node_id: BlockId,
        parent_id: Option<BlockId>,
        depth: usize,
        max_depth: usize,
        filter: &TraversalFilter,
        output: TraversalOutput,
        visited: &mut HashSet<BlockId>,
        nodes: &mut Vec<TraversalNode>,
        edges: &mut Vec<TraversalEdge>,
        nodes_by_role: &mut HashMap<String, usize>,
    ) -> Result<()> {
        if depth > max_depth || nodes.len() >= self.config.max_nodes {
            return Ok(());
        }

        if visited.contains(&node_id) {
            return Ok(());
        }
        visited.insert(node_id);

        if let Some(block) = doc.get_block(&node_id) {
            if self.matches_filter(block, filter) {
                let node = self.create_traversal_node(doc, &node_id, depth, parent_id, output);

                if let Some(role) = &node.semantic_role {
                    *nodes_by_role.entry(role.clone()).or_insert(0) += 1;
                }

                nodes.push(node);

                // Collect edges
                for edge in &block.edges {
                    if filter.edge_types.is_empty() || filter.edge_types.contains(&edge.edge_type) {
                        edges.push(TraversalEdge {
                            source: node_id,
                            target: edge.target,
                            edge_type: edge.edge_type.clone(),
                        });
                    }
                }
            }

            // Recurse to children
            for child in doc.children(&node_id) {
                self.dfs_recursive(
                    doc,
                    *child,
                    Some(node_id),
                    depth + 1,
                    max_depth,
                    filter,
                    output,
                    visited,
                    nodes,
                    edges,
                    nodes_by_role,
                )?;
            }
        }

        Ok(())
    }

    /// Check if a block matches the filter criteria
    fn matches_filter(&self, block: &Block, filter: &TraversalFilter) -> bool {
        // Check role inclusion
        if !filter.include_roles.is_empty() {
            let role = block
                .metadata
                .semantic_role
                .as_ref()
                .map(|r| r.category.as_str().to_string())
                .unwrap_or_default();
            if !filter.include_roles.contains(&role) {
                return false;
            }
        }

        // Check role exclusion
        if !filter.exclude_roles.is_empty() {
            let role = block
                .metadata
                .semantic_role
                .as_ref()
                .map(|r| r.category.as_str().to_string())
                .unwrap_or_default();
            if filter.exclude_roles.contains(&role) {
                return false;
            }
        }

        // Check tag inclusion
        if !filter.include_tags.is_empty() {
            let has_tag = filter
                .include_tags
                .iter()
                .any(|t| block.metadata.tags.contains(t));
            if !has_tag {
                return false;
            }
        }

        // Check tag exclusion
        if !filter.exclude_tags.is_empty() {
            let has_excluded = filter
                .exclude_tags
                .iter()
                .any(|t| block.metadata.tags.contains(t));
            if has_excluded {
                return false;
            }
        }

        // Check content pattern
        if let Some(ref pattern) = filter.content_pattern {
            let content_text = self.extract_content_text(&block.content);
            if !content_text
                .to_lowercase()
                .contains(&pattern.to_lowercase())
            {
                return false;
            }
        }

        true
    }

    /// Create a traversal node from a block
    fn create_traversal_node(
        &self,
        doc: &Document,
        block_id: &BlockId,
        depth: usize,
        parent_id: Option<BlockId>,
        output: TraversalOutput,
    ) -> TraversalNode {
        let block = doc.get_block(block_id);
        let children = doc.children(block_id);

        let content_preview = match output {
            TraversalOutput::StructureOnly => None,
            TraversalOutput::StructureWithPreviews | TraversalOutput::StructureAndBlocks => block
                .map(|b| {
                    let text = self.extract_content_text(&b.content);
                    if text.len() > self.config.default_preview_length {
                        format!("{}...", &text[..self.config.default_preview_length])
                    } else {
                        text
                    }
                }),
        };

        let semantic_role = block
            .and_then(|b| b.metadata.semantic_role.as_ref())
            .map(|r| r.category.as_str().to_string());

        let edge_count = block.map(|b| b.edges.len()).unwrap_or(0);

        TraversalNode {
            id: *block_id,
            depth,
            parent_id,
            content_preview,
            semantic_role,
            child_count: children.len(),
            edge_count,
        }
    }

    /// Extract text content from a Content enum
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

impl Default for TraversalEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucm_core::DocumentId;

    fn create_test_document() -> Document {
        let mut doc = Document::new(DocumentId::new("test"));
        let root = doc.root;

        // Create a simple hierarchy
        let h1 = Block::new(Content::text("Chapter 1"), Some("heading1"));
        let h1_id = doc.add_block(h1, &root).unwrap();

        let p1 = Block::new(Content::text("Introduction paragraph"), Some("paragraph"));
        doc.add_block(p1, &h1_id).unwrap();

        let h2 = Block::new(Content::text("Section 1.1"), Some("heading2"));
        let h2_id = doc.add_block(h2, &h1_id).unwrap();

        let p2 = Block::new(Content::text("Section content"), Some("paragraph"));
        doc.add_block(p2, &h2_id).unwrap();

        doc
    }

    #[test]
    fn test_bfs_traversal() {
        let doc = create_test_document();
        let engine = TraversalEngine::new();

        let result = engine
            .navigate(
                &doc,
                None,
                NavigateDirection::BreadthFirst,
                Some(10),
                None,
                TraversalOutput::StructureAndBlocks,
            )
            .unwrap();

        assert!(!result.nodes.is_empty());
        assert!(result.summary.total_nodes >= 4);
    }

    #[test]
    fn test_dfs_traversal() {
        let doc = create_test_document();
        let engine = TraversalEngine::new();

        let result = engine
            .navigate(
                &doc,
                None,
                NavigateDirection::DepthFirst,
                Some(10),
                None,
                TraversalOutput::StructureAndBlocks,
            )
            .unwrap();

        assert!(!result.nodes.is_empty());
    }

    #[test]
    fn test_path_to_root() {
        let doc = create_test_document();
        let engine = TraversalEngine::new();

        // Find a leaf node
        let root_children = doc.children(&doc.root);
        if let Some(h1_id) = root_children.first() {
            let h1_children = doc.children(h1_id);
            if let Some(h2_id) = h1_children.iter().find(|id| {
                doc.get_block(id)
                    .and_then(|b| b.metadata.semantic_role.as_ref())
                    .map(|r| r.category.as_str() == "heading2")
                    .unwrap_or(false)
            }) {
                let path = engine.path_to_root(&doc, h2_id).unwrap();
                assert!(path.len() >= 2);
                assert_eq!(path[0], doc.root);
            }
        }
    }

    #[test]
    fn test_filter_by_role() {
        let doc = create_test_document();
        let engine = TraversalEngine::new();

        let filter = TraversalFilter {
            include_roles: vec!["heading1".to_string(), "heading2".to_string()],
            ..Default::default()
        };

        let result = engine
            .navigate(
                &doc,
                None,
                NavigateDirection::BreadthFirst,
                Some(10),
                Some(filter),
                TraversalOutput::StructureAndBlocks,
            )
            .unwrap();

        // Should only include headings
        for node in &result.nodes {
            if let Some(role) = &node.semantic_role {
                assert!(role.starts_with("heading"));
            }
        }
    }

    #[test]
    fn test_expand_node() {
        let doc = create_test_document();
        let engine = TraversalEngine::new();

        let result = engine
            .expand(&doc, &doc.root, TraversalOutput::StructureAndBlocks)
            .unwrap();

        // Should include root and immediate children
        assert!(!result.nodes.is_empty());
    }

    #[test]
    fn test_max_depth_limit() {
        let doc = create_test_document();
        let config = TraversalConfig {
            max_depth: 1,
            ..Default::default()
        };
        let engine = TraversalEngine::with_config(config);

        let result = engine
            .navigate(
                &doc,
                None,
                NavigateDirection::BreadthFirst,
                Some(1),
                None,
                TraversalOutput::StructureAndBlocks,
            )
            .unwrap();

        // All nodes should have depth <= 1
        for node in &result.nodes {
            assert!(node.depth <= 1);
        }
    }
}
