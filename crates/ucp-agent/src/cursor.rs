//! Traversal cursor for tracking position in the graph.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use ucm_core::{BlockId, EdgeType};

/// Represents an agent's current position and view in the graph.
#[derive(Debug, Clone)]
pub struct TraversalCursor {
    /// Current focus block.
    pub position: BlockId,
    /// Visible neighborhood (cached expansion).
    pub neighborhood: CursorNeighborhood,
    /// Breadcrumb trail for navigation.
    pub breadcrumbs: VecDeque<BlockId>,
    /// Current view mode.
    pub view_mode: ViewMode,
    /// Maximum breadcrumbs to retain.
    max_breadcrumbs: usize,
}

impl TraversalCursor {
    pub fn new(position: BlockId, max_breadcrumbs: usize) -> Self {
        Self {
            position,
            neighborhood: CursorNeighborhood::default(),
            breadcrumbs: VecDeque::new(),
            view_mode: ViewMode::default(),
            max_breadcrumbs,
        }
    }

    /// Move cursor to a new position.
    pub fn move_to(&mut self, new_position: BlockId) {
        // Add current position to breadcrumbs
        self.breadcrumbs.push_back(self.position);
        if self.breadcrumbs.len() > self.max_breadcrumbs {
            self.breadcrumbs.pop_front();
        }

        // Update position and mark neighborhood as stale
        self.position = new_position;
        self.neighborhood.stale = true;
    }

    /// Go back in navigation history.
    pub fn go_back(&mut self, steps: usize) -> Option<BlockId> {
        for _ in 0..steps {
            if let Some(prev) = self.breadcrumbs.pop_back() {
                self.position = prev;
                self.neighborhood.stale = true;
            } else {
                return None;
            }
        }
        Some(self.position)
    }

    /// Check if we can go back.
    pub fn can_go_back(&self) -> bool {
        !self.breadcrumbs.is_empty()
    }

    /// Get the number of steps we can go back.
    pub fn history_depth(&self) -> usize {
        self.breadcrumbs.len()
    }

    /// Clear navigation history.
    pub fn clear_history(&mut self) {
        self.breadcrumbs.clear();
    }

    /// Update the neighborhood cache.
    pub fn update_neighborhood(&mut self, neighborhood: CursorNeighborhood) {
        self.neighborhood = neighborhood;
        self.neighborhood.stale = false;
        self.neighborhood.computed_at = Utc::now();
    }

    /// Check if neighborhood needs refresh.
    pub fn needs_refresh(&self) -> bool {
        self.neighborhood.stale
    }
}

/// The cached visible neighborhood around cursor position.
#[derive(Debug, Clone)]
pub struct CursorNeighborhood {
    /// Parent chain to root (limited depth).
    pub ancestors: Vec<BlockId>,
    /// Immediate children.
    pub children: Vec<BlockId>,
    /// Siblings at same level.
    pub siblings: Vec<BlockId>,
    /// Semantic connections (via edges).
    pub connections: Vec<(BlockId, EdgeType)>,
    /// Timestamp when neighborhood was computed.
    pub computed_at: DateTime<Utc>,
    /// Whether neighborhood needs refresh.
    pub stale: bool,
}

impl Default for CursorNeighborhood {
    fn default() -> Self {
        Self::new()
    }
}

impl CursorNeighborhood {
    pub fn new() -> Self {
        Self {
            ancestors: Vec::new(),
            children: Vec::new(),
            siblings: Vec::new(),
            connections: Vec::new(),
            computed_at: Utc::now(),
            stale: true,
        }
    }

    /// Total blocks in neighborhood.
    pub fn total_blocks(&self) -> usize {
        self.ancestors.len() + self.children.len() + self.siblings.len() + self.connections.len()
    }
}

/// How much detail to show in traversal results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ViewMode {
    /// Just block IDs and structure.
    IdsOnly,
    /// Structure with short previews (first N characters).
    Preview { length: usize },
    /// Full content for blocks in view.
    #[default]
    Full,
    /// Metadata only (semantic role, tags, edge counts).
    Metadata,
    /// Adaptive - auto-select based on relevance.
    Adaptive { interest_threshold: f32 },
}

impl ViewMode {
    pub fn preview(length: usize) -> Self {
        Self::Preview { length }
    }

    pub fn adaptive(threshold: f32) -> Self {
        Self::Adaptive {
            interest_threshold: threshold,
        }
    }
}

/// Convert from UCL AST ViewMode to our ViewMode.
impl From<ucl_parser::ast::ViewMode> for ViewMode {
    fn from(mode: ucl_parser::ast::ViewMode) -> Self {
        match mode {
            ucl_parser::ast::ViewMode::Full => ViewMode::Full,
            ucl_parser::ast::ViewMode::Preview { length } => ViewMode::Preview { length },
            ucl_parser::ast::ViewMode::Metadata => ViewMode::Metadata,
            ucl_parser::ast::ViewMode::IdsOnly => ViewMode::IdsOnly,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block_id(s: &str) -> BlockId {
        s.parse().unwrap_or_else(|_| {
            // Create a deterministic ID from the input string for testing
            let mut bytes = [0u8; 12];
            let s_bytes = s.as_bytes();
            for (i, b) in s_bytes.iter().enumerate() {
                bytes[i % 12] ^= *b;
            }
            BlockId::from_bytes(bytes)
        })
    }

    #[test]
    fn test_cursor_movement() {
        let mut cursor = TraversalCursor::new(block_id("blk_000000000001"), 10);

        cursor.move_to(block_id("blk_000000000002"));
        assert_eq!(cursor.position, block_id("blk_000000000002"));
        assert_eq!(cursor.breadcrumbs.len(), 1);

        cursor.move_to(block_id("blk_000000000003"));
        assert_eq!(cursor.position, block_id("blk_000000000003"));
        assert_eq!(cursor.breadcrumbs.len(), 2);

        // Go back
        cursor.go_back(1);
        assert_eq!(cursor.position, block_id("blk_000000000002"));
        assert_eq!(cursor.breadcrumbs.len(), 1);
    }

    #[test]
    fn test_cursor_history_limit() {
        let mut cursor = TraversalCursor::new(block_id("blk_000000000001"), 3);

        // Move more times than the limit
        for i in 2..=5 {
            cursor.move_to(block_id(&format!("blk_00000000000{}", i)));
        }

        // Should only have 3 breadcrumbs
        assert_eq!(cursor.breadcrumbs.len(), 3);
    }

    #[test]
    fn test_neighborhood_staleness() {
        let mut cursor = TraversalCursor::new(block_id("blk_000000000001"), 10);

        // Initially stale
        assert!(cursor.needs_refresh());

        // Update neighborhood
        cursor.update_neighborhood(CursorNeighborhood::new());
        assert!(!cursor.needs_refresh());

        // Move makes it stale again
        cursor.move_to(block_id("blk_000000000002"));
        assert!(cursor.needs_refresh());
    }
}
