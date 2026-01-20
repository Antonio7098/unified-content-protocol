//! Operations that can be applied to documents.

use serde::{Deserialize, Serialize};
use ucm_core::{BlockId, Content, EdgeType};

/// Target for move operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MoveTarget {
    /// Move to a parent at optional index
    ToParent {
        parent_id: BlockId,
        index: Option<usize>,
    },
    /// Move before a sibling
    Before { sibling_id: BlockId },
    /// Move after a sibling
    After { sibling_id: BlockId },
}

/// Operations that can be applied to a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    /// Edit content at a path
    Edit {
        block_id: BlockId,
        path: String,
        value: serde_json::Value,
        operator: EditOperator,
    },

    /// Move a block to a new parent (legacy)
    Move {
        block_id: BlockId,
        new_parent: BlockId,
        index: Option<usize>,
    },

    /// Move a block with flexible target
    MoveToTarget {
        block_id: BlockId,
        target: MoveTarget,
    },

    /// Append a new block
    Append {
        parent_id: BlockId,
        content: Content,
        label: Option<String>,
        tags: Vec<String>,
        semantic_role: Option<String>,
        index: Option<usize>,
    },

    /// Delete a block
    Delete {
        block_id: BlockId,
        cascade: bool,
        preserve_children: bool,
    },

    /// Prune unreachable blocks
    Prune { condition: Option<PruneCondition> },

    /// Add an edge
    Link {
        source: BlockId,
        edge_type: EdgeType,
        target: BlockId,
        metadata: Option<serde_json::Value>,
    },

    /// Remove an edge
    Unlink {
        source: BlockId,
        edge_type: EdgeType,
        target: BlockId,
    },

    /// Create a snapshot
    CreateSnapshot {
        name: String,
        description: Option<String>,
    },

    /// Restore a snapshot
    RestoreSnapshot { name: String },

    /// Write markdown content to a section, replacing all children
    WriteSection {
        /// Target section (heading block) to write to
        section_id: BlockId,
        /// New markdown content to parse and insert
        markdown: String,
        /// Adjust heading levels relative to this base (e.g., 2 means top-level becomes H2)
        base_heading_level: Option<usize>,
    },
}

/// Edit operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditOperator {
    /// Set value (=)
    Set,
    /// Append to string/array (+=)
    Append,
    /// Remove from string/array (-=)
    Remove,
    /// Increment number (++)
    Increment,
    /// Decrement number (--)
    Decrement,
}

/// Prune condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PruneCondition {
    Unreachable,
    TagContains(String),
    Custom(String),
}

/// Result of an operation
#[derive(Debug, Clone)]
pub struct OperationResult {
    /// Whether the operation succeeded
    pub success: bool,
    /// Affected block IDs
    pub affected_blocks: Vec<BlockId>,
    /// Any warnings generated
    pub warnings: Vec<String>,
    /// Error message if failed
    pub error: Option<String>,
}

impl OperationResult {
    pub fn success(affected: Vec<BlockId>) -> Self {
        Self {
            success: true,
            affected_blocks: affected,
            warnings: Vec::new(),
            error: None,
        }
    }

    pub fn failure(error: impl Into<String>) -> Self {
        Self {
            success: false,
            affected_blocks: Vec::new(),
            warnings: Vec::new(),
            error: Some(error.into()),
        }
    }

    pub fn with_warning(mut self, warning: impl Into<String>) -> Self {
        self.warnings.push(warning.into());
        self
    }
}

impl Operation {
    /// Get a description of the operation for logging
    pub fn description(&self) -> String {
        match self {
            Operation::Edit { block_id, path, .. } => {
                format!("EDIT {} SET {}", block_id, path)
            }
            Operation::Move {
                block_id,
                new_parent,
                ..
            } => {
                format!("MOVE {} TO {}", block_id, new_parent)
            }
            Operation::MoveToTarget { block_id, target } => match target {
                MoveTarget::ToParent { parent_id, index } => {
                    if let Some(idx) = index {
                        format!("MOVE {} TO {} AT {}", block_id, parent_id, idx)
                    } else {
                        format!("MOVE {} TO {}", block_id, parent_id)
                    }
                }
                MoveTarget::Before { sibling_id } => {
                    format!("MOVE {} BEFORE {}", block_id, sibling_id)
                }
                MoveTarget::After { sibling_id } => {
                    format!("MOVE {} AFTER {}", block_id, sibling_id)
                }
            },
            Operation::Append { parent_id, .. } => {
                format!("APPEND to {}", parent_id)
            }
            Operation::Delete {
                block_id, cascade, ..
            } => {
                if *cascade {
                    format!("DELETE {} CASCADE", block_id)
                } else {
                    format!("DELETE {}", block_id)
                }
            }
            Operation::Prune { condition } => match condition {
                Some(PruneCondition::Unreachable) | None => "PRUNE UNREACHABLE".to_string(),
                Some(PruneCondition::TagContains(tag)) => format!("PRUNE WHERE tag={}", tag),
                Some(PruneCondition::Custom(c)) => format!("PRUNE WHERE {}", c),
            },
            Operation::Link {
                source,
                edge_type,
                target,
                ..
            } => {
                format!("LINK {} {} {}", source, edge_type.as_str(), target)
            }
            Operation::Unlink {
                source,
                edge_type,
                target,
            } => {
                format!("UNLINK {} {} {}", source, edge_type.as_str(), target)
            }
            Operation::CreateSnapshot { name, .. } => {
                format!("SNAPSHOT CREATE {}", name)
            }
            Operation::RestoreSnapshot { name } => {
                format!("SNAPSHOT RESTORE {}", name)
            }
            Operation::WriteSection {
                section_id,
                base_heading_level,
                ..
            } => {
                if let Some(level) = base_heading_level {
                    format!("WRITE_SECTION {} BASE_LEVEL {}", section_id, level)
                } else {
                    format!("WRITE_SECTION {}", section_id)
                }
            }
        }
    }
}
