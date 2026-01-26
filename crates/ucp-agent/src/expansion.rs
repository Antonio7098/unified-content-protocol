use crate::error::AgentResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ucm_core::{Block, BlockId, Content, Document};

/// Expansion direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpandDirection {
    Down,
    Up,
    Both,
    Semantic,
}

impl Default for ExpandDirection {
    fn default() -> Self {
        ExpandDirection::Down
    }
}

/// Expansion options
#[derive(Debug, Clone, Default)]
pub struct ExpandOptions {
    pub depth: usize,
    pub view_mode: ViewMode,
    pub role_filter: Vec<String>,
    pub tag_filter: Vec<String>,
    pub include_metadata: bool,
}

impl ExpandOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    pub fn with_view_mode(mut self, mode: ViewMode) -> Self {
        self.view_mode = mode;
        self
    }

    pub fn with_role_filter(mut self, roles: &[&str]) -> Self {
        self.role_filter = roles.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_tag_filter(mut self, tags: &[&str]) -> Self {
        self.tag_filter = tags.iter().map(|s| s.to_string()).collect();
        self
    }
}

/// View mode for displaying blocks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ViewMode {
    #[default]
    Full,
    Preview,
    Metadata,
    IdsOnly,
    Adaptive,
}

/// Block view representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockView {
    pub block_id: BlockId,
    pub content_type: String,
    pub content: String,
    pub role: Option<String>,
    pub tags: Vec<String>,
    pub label: Option<String>,
    pub edge_count: usize,
    pub preview_length: usize,
}

impl BlockView {
    pub fn from_block(block: &Block, mode: ViewMode) -> AgentResult<Self> {
        let content = match &block.content {
            Content::Text(t) => t.text.clone(),
            Content::Code(c) => format!("```{}\n{}\n```", c.language, c.source),
            Content::Table(t) => format!(
                "|{}|",
                t.rows
                    .iter()
                    .map(|r| r.join("|"))
                    .collect::<Vec<_>>()
                    .join("|\n|")
            ),
            Content::Math(m) => m.expression.clone(),
            Content::Json { value } => value.to_string(),
            Content::Media(m) => format!("[Media: {:?}]", m.media_type),
            Content::Binary { mime_type, .. } => format!("[Binary: {}]", mime_type),
            Content::Composite { .. } => "[Composite]".to_string(),
        };

        let preview_length = match mode {
            ViewMode::Preview => 200,
            ViewMode::IdsOnly => 0,
            _ => content.len(),
        };

        let display_content = if preview_length > 0 && content.len() > preview_length {
            format!("{}...", &content[..preview_length])
        } else {
            content.clone()
        };

        Ok(Self {
            block_id: block.id.clone(),
            content_type: format!("{:?}", block.content),
            content: match mode {
                ViewMode::IdsOnly => String::new(),
                ViewMode::Metadata => String::new(),
                _ => display_content,
            },
            role: block
                .metadata
                .semantic_role
                .as_ref()
                .map(|r| r.category.as_str().to_string()),
            tags: block.metadata.tags.clone(),
            label: block.metadata.label.clone(),
            edge_count: block.edges.len(),
            preview_length,
        })
    }
}

/// Neighborhood view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeighborhoodView {
    pub center: BlockId,
    pub center_view: Option<BlockView>,
    pub ancestors: Vec<BlockView>,
    pub children: Vec<BlockView>,
    pub siblings: Vec<BlockView>,
    pub connections: Vec<ConnectionView>,
    pub depth: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionView {
    pub target: BlockId,
    pub edge_type: String,
    pub target_view: Option<BlockView>,
}

/// Expansion result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpansionResult {
    pub root_block: BlockId,
    pub total_blocks: usize,
    pub blocks_by_level: HashMap<usize, Vec<BlockView>>,
    pub directions: Vec<ExpandDirection>,
    pub depth_reached: usize,
    pub view_mode: ViewMode,
    pub timestamp: u64,
}

impl ExpansionResult {
    pub fn compute(
        doc: &Document,
        block_id: &BlockId,
        direction: ExpandDirection,
        options: ExpandOptions,
    ) -> AgentResult<Self> {
        let mut blocks_by_level = HashMap::new();
        let mut visited = std::collections::HashSet::new();
        let mut depth_reached = 0;

        // BFS expansion
        let mut current_level = vec![block_id.clone()];
        visited.insert(block_id.clone());

        for depth in 0..=options.depth {
            let mut block_views = Vec::new();

            for bid in &current_level {
                if let Some(block) = doc.get_block(bid) {
                    // Apply filters
                    if !options.role_filter.is_empty() {
                        if let Some(role) = &block.metadata.semantic_role {
                            if !options
                                .role_filter
                                .iter()
                                .any(|r| r == &role.category.as_str())
                            {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    }

                    if !options.tag_filter.is_empty() {
                        if !options
                            .tag_filter
                            .iter()
                            .any(|t| block.metadata.tags.contains(t))
                        {
                            continue;
                        }
                    }

                    if let Ok(view) = BlockView::from_block(block, options.view_mode) {
                        block_views.push(view);
                    }
                }
            }

            if !block_views.is_empty() {
                blocks_by_level.insert(depth, block_views);
                depth_reached = depth;
            }

            // Get next level blocks
            let mut next_level = Vec::new();
            for bid in &current_level {
                match direction {
                    ExpandDirection::Down | ExpandDirection::Both => {
                        let children = doc.children(bid);
                        for child in children {
                            if !visited.contains(&child) {
                                visited.insert(child.clone());
                                next_level.push(child);
                            }
                        }
                    }
                    ExpandDirection::Up => {
                        if let Some(parent) = doc.parent_of(bid) {
                            if !visited.contains(&parent) {
                                visited.insert(parent.clone());
                                next_level.push(parent);
                            }
                        }
                    }
                    ExpandDirection::Semantic => {
                        if let Some(edges) = doc.edges.get(bid) {
                            for edge in edges {
                                if !visited.contains(&edge.target) {
                                    visited.insert(edge.target.clone());
                                    next_level.push(edge.target.clone());
                                }
                            }
                        }
                    }
                }
            }

            current_level = next_level;
            if current_level.is_empty() {
                break;
            }
        }

        Ok(Self {
            root_block: block_id.clone(),
            total_blocks: visited.len(),
            blocks_by_level,
            directions: vec![direction],
            depth_reached,
            view_mode: options.view_mode,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        })
    }
}

impl NeighborhoodView {
    pub fn from_neighborhood(
        neighborhood: &super::cursor::Neighborhood,
        doc: &Document,
        view_mode: ViewMode,
    ) -> Self {
        let center_view = doc
            .get_block(&neighborhood.center)
            .and_then(|b| BlockView::from_block(b, view_mode).ok());

        let ancestors = neighborhood
            .ancestors
            .iter()
            .filter_map(|id| {
                doc.get_block(id)
                    .and_then(|b| BlockView::from_block(b, view_mode).ok())
            })
            .collect();

        let children = neighborhood
            .children
            .iter()
            .filter_map(|id| {
                doc.get_block(id)
                    .and_then(|b| BlockView::from_block(b, view_mode).ok())
            })
            .collect();

        let siblings = neighborhood
            .siblings
            .iter()
            .filter_map(|id| {
                doc.get_block(id)
                    .and_then(|b| BlockView::from_block(b, view_mode).ok())
            })
            .collect();

        let connections = neighborhood
            .connections
            .iter()
            .map(|conn| ConnectionView {
                target: conn.block_id.clone(),
                edge_type: conn.edge_type.clone(),
                target_view: doc
                    .get_block(&conn.block_id)
                    .and_then(|b| BlockView::from_block(b, view_mode).ok()),
            })
            .collect();

        Self {
            center: neighborhood.center.clone(),
            center_view,
            ancestors,
            children,
            siblings,
            connections,
            depth: neighborhood.depth,
        }
    }
}
