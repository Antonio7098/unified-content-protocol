//! Developer tools for JSON-UCL workflows.
//!
//! Provides utilities for developers working with UCM documents:
//! - JSON to UCL conversion helpers
//! - Document inspection and diff utilities
//! - UCL command generation from JSON patches
//!
//! ## Design Principles (SOLID)
//!
//! - **Single Responsibility**: Each tool handles one specific task
//! - **Open/Closed**: New tools can be added without modifying existing ones
//! - **Interface Segregation**: Small, focused traits for different tool types

use serde_json::Value;
use std::collections::HashMap;
use ucm_core::{Block, BlockId, Content, Document};

/// Represents a change detected between two JSON values
#[derive(Debug, Clone, PartialEq)]
pub enum JsonChange {
    /// Value was added
    Added { path: String, value: Value },
    /// Value was removed
    Removed { path: String, old_value: Value },
    /// Value was modified
    Modified { path: String, old_value: Value, new_value: Value },
}

impl JsonChange {
    /// Convert this change to a UCL EDIT command
    pub fn to_ucl(&self, block_id: &str) -> String {
        match self {
            JsonChange::Added { path, value } => {
                let value_str = serde_json::to_string(value).unwrap_or_default();
                format!("EDIT {} SET {} = {}", block_id, path, value_str)
            }
            JsonChange::Removed { path, .. } => {
                format!("EDIT {} SET {} = null", block_id, path)
            }
            JsonChange::Modified { path, new_value, .. } => {
                let value_str = serde_json::to_string(new_value).unwrap_or_default();
                format!("EDIT {} SET {} = {}", block_id, path, value_str)
            }
        }
    }
}

/// Compare two JSON values and return the differences
pub fn diff_json(old: &Value, new: &Value) -> Vec<JsonChange> {
    let mut changes = Vec::new();
    diff_json_recursive(old, new, String::new(), &mut changes);
    changes
}

fn diff_json_recursive(old: &Value, new: &Value, path: String, changes: &mut Vec<JsonChange>) {
    match (old, new) {
        (Value::Object(old_map), Value::Object(new_map)) => {
            // Check for removed and modified keys
            for (key, old_val) in old_map {
                let key_path = if path.is_empty() {
                    format!("$.{}", key)
                } else {
                    format!("{}.{}", path, key)
                };

                if let Some(new_val) = new_map.get(key) {
                    if old_val != new_val {
                        diff_json_recursive(old_val, new_val, key_path, changes);
                    }
                } else {
                    changes.push(JsonChange::Removed {
                        path: key_path,
                        old_value: old_val.clone(),
                    });
                }
            }

            // Check for added keys
            for (key, new_val) in new_map {
                if !old_map.contains_key(key) {
                    let key_path = if path.is_empty() {
                        format!("$.{}", key)
                    } else {
                        format!("{}.{}", path, key)
                    };
                    changes.push(JsonChange::Added {
                        path: key_path,
                        value: new_val.clone(),
                    });
                }
            }
        }
        (Value::Array(old_arr), Value::Array(new_arr)) => {
            // For arrays, we do a simple comparison
            // More sophisticated diff could use LCS algorithm
            if old_arr != new_arr {
                changes.push(JsonChange::Modified {
                    path: if path.is_empty() { "$".to_string() } else { path },
                    old_value: Value::Array(old_arr.clone()),
                    new_value: Value::Array(new_arr.clone()),
                });
            }
        }
        _ => {
            if old != new {
                changes.push(JsonChange::Modified {
                    path: if path.is_empty() { "$".to_string() } else { path },
                    old_value: old.clone(),
                    new_value: new.clone(),
                });
            }
        }
    }
}

/// Generate UCL commands from JSON changes
pub fn json_changes_to_ucl(block_id: &str, changes: &[JsonChange]) -> Vec<String> {
    changes.iter().map(|c| c.to_ucl(block_id)).collect()
}

/// Document inspector for debugging and development
#[derive(Debug)]
pub struct DocumentInspector<'a> {
    doc: &'a Document,
}

impl<'a> DocumentInspector<'a> {
    pub fn new(doc: &'a Document) -> Self {
        Self { doc }
    }

    /// Get a summary of the document structure
    pub fn summary(&self) -> DocumentSummary {
        let mut content_types = HashMap::new();
        let mut roles = HashMap::new();

        for block in self.doc.blocks.values() {
            *content_types.entry(block.content_type().to_string()).or_insert(0) += 1;
            if let Some(ref role) = block.metadata.semantic_role {
                *roles.entry(role.category.as_str().to_string()).or_insert(0) += 1;
            }
        }

        DocumentSummary {
            block_count: self.doc.block_count(),
            max_depth: self.calculate_max_depth(),
            content_types,
            roles,
        }
    }

    /// Get block info by ID
    pub fn block_info(&self, id: &BlockId) -> Option<BlockInfo> {
        let block = self.doc.get_block(id)?;
        let parent = self.doc.parent(id).cloned();
        let children: Vec<_> = self.doc.children(id).to_vec();
        let depth = self.calculate_depth(id);

        Some(BlockInfo {
            id: id.clone(),
            content_type: block.content_type().to_string(),
            role: block.metadata.semantic_role.as_ref().map(|r| r.to_string()),
            label: block.metadata.label.clone(),
            parent,
            children,
            depth,
            content_preview: self.content_preview(&block.content, 100),
        })
    }

    /// Find blocks matching a predicate
    pub fn find_blocks<F>(&self, predicate: F) -> Vec<&Block>
    where
        F: Fn(&Block) -> bool,
    {
        self.doc.blocks.values().filter(|b| predicate(b)).collect()
    }

    /// Get blocks by content type
    pub fn blocks_by_type(&self, content_type: &str) -> Vec<&Block> {
        self.find_blocks(|b| b.content_type() == content_type)
    }

    /// Get blocks by role
    pub fn blocks_by_role(&self, role: &str) -> Vec<&Block> {
        self.find_blocks(|b| {
            b.metadata
                .semantic_role
                .as_ref()
                .map(|r| r.category.as_str() == role)
                .unwrap_or(false)
        })
    }

    /// Export document structure as JSON
    pub fn to_json(&self) -> Value {
        let structure: HashMap<String, Vec<String>> = self
            .doc
            .structure
            .iter()
            .map(|(parent, children)| {
                (
                    parent.to_string(),
                    children.iter().map(|c| c.to_string()).collect(),
                )
            })
            .collect();

        let blocks: HashMap<String, Value> = self
            .doc
            .blocks
            .iter()
            .map(|(id, block)| {
                (
                    id.to_string(),
                    serde_json::to_value(block).unwrap_or(Value::Null),
                )
            })
            .collect();

        serde_json::json!({
            "id": self.doc.id.to_string(),
            "root": self.doc.root.to_string(),
            "block_count": self.doc.block_count(),
            "structure": structure,
            "blocks": blocks,
            "metadata": self.doc.metadata,
        })
    }

    fn calculate_max_depth(&self) -> usize {
        let mut max_depth = 0;
        for id in self.doc.blocks.keys() {
            let depth = self.calculate_depth(id);
            if depth > max_depth {
                max_depth = depth;
            }
        }
        max_depth
    }

    fn calculate_depth(&self, id: &BlockId) -> usize {
        let mut depth = 0;
        let mut current = id.clone();
        while let Some(parent) = self.doc.parent(&current) {
            depth += 1;
            current = parent.clone();
        }
        depth
    }

    fn content_preview(&self, content: &Content, max_len: usize) -> String {
        let text = match content {
            Content::Text(t) => t.text.clone(),
            Content::Code(c) => format!("[code:{}] {}", c.language, &c.source[..c.source.len().min(50)]),
            Content::Table(t) => format!("[table {}x{}]", t.columns.len(), t.rows.len()),
            Content::Math(m) => format!("[math] {}", m.expression),
            Content::Json { value, .. } => format!("[json] {}", value),
            Content::Media(m) => format!("[media:{:?}]", m.media_type),
            Content::Binary { mime_type, .. } => format!("[binary:{}]", mime_type),
            Content::Composite { children, .. } => format!("[composite:{} children]", children.len()),
        };

        if text.len() > max_len {
            format!("{}...", &text[..max_len])
        } else {
            text
        }
    }
}

/// Summary of document structure
#[derive(Debug, Clone)]
pub struct DocumentSummary {
    pub block_count: usize,
    pub max_depth: usize,
    pub content_types: HashMap<String, usize>,
    pub roles: HashMap<String, usize>,
}

/// Information about a single block
#[derive(Debug, Clone)]
pub struct BlockInfo {
    pub id: BlockId,
    pub content_type: String,
    pub role: Option<String>,
    pub label: Option<String>,
    pub parent: Option<BlockId>,
    pub children: Vec<BlockId>,
    pub depth: usize,
    pub content_preview: String,
}

/// UCL command builder for common operations
pub struct UclBuilder {
    commands: Vec<String>,
}

impl UclBuilder {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    /// Add an EDIT command
    pub fn edit(mut self, block_id: &str, path: &str, value: &str) -> Self {
        self.commands.push(format!(
            "EDIT {} SET {} = \"{}\"",
            block_id, path, value
        ));
        self
    }

    /// Add an EDIT command with JSON value
    pub fn edit_json(mut self, block_id: &str, path: &str, value: &Value) -> Self {
        let value_str = serde_json::to_string(value).unwrap_or_default();
        self.commands.push(format!(
            "EDIT {} SET {} = {}",
            block_id, path, value_str
        ));
        self
    }

    /// Add an APPEND command
    pub fn append(mut self, parent_id: &str, content_type: &str, content: &str) -> Self {
        self.commands.push(format!(
            "APPEND {} {} :: {}",
            parent_id, content_type, content
        ));
        self
    }

    /// Add an APPEND command with label
    pub fn append_with_label(
        mut self,
        parent_id: &str,
        content_type: &str,
        label: &str,
        content: &str,
    ) -> Self {
        self.commands.push(format!(
            "APPEND {} {} WITH label = \"{}\" :: {}",
            parent_id, content_type, label, content
        ));
        self
    }

    /// Add a MOVE command
    pub fn move_to(mut self, block_id: &str, new_parent: &str) -> Self {
        self.commands.push(format!("MOVE {} TO {}", block_id, new_parent));
        self
    }

    /// Add a DELETE command
    pub fn delete(mut self, block_id: &str) -> Self {
        self.commands.push(format!("DELETE {}", block_id));
        self
    }

    /// Add a DELETE CASCADE command
    pub fn delete_cascade(mut self, block_id: &str) -> Self {
        self.commands.push(format!("DELETE {} CASCADE", block_id));
        self
    }

    /// Add a LINK command
    pub fn link(mut self, source: &str, edge_type: &str, target: &str) -> Self {
        self.commands.push(format!("LINK {} {} {}", source, edge_type, target));
        self
    }

    /// Wrap commands in ATOMIC block
    pub fn atomic(mut self) -> Self {
        if !self.commands.is_empty() {
            let inner = self.commands.join("\n  ");
            self.commands = vec![format!("ATOMIC {{\n  {}\n}}", inner)];
        }
        self
    }

    /// Build the final UCL string
    pub fn build(self) -> String {
        self.commands.join("\n")
    }

    /// Get commands as vector
    pub fn commands(self) -> Vec<String> {
        self.commands
    }
}

impl Default for UclBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_json_added() {
        let old = serde_json::json!({"a": 1});
        let new = serde_json::json!({"a": 1, "b": 2});

        let changes = diff_json(&old, &new);
        assert_eq!(changes.len(), 1);
        assert!(matches!(&changes[0], JsonChange::Added { path, .. } if path == "$.b"));
    }

    #[test]
    fn test_diff_json_removed() {
        let old = serde_json::json!({"a": 1, "b": 2});
        let new = serde_json::json!({"a": 1});

        let changes = diff_json(&old, &new);
        assert_eq!(changes.len(), 1);
        assert!(matches!(&changes[0], JsonChange::Removed { path, .. } if path == "$.b"));
    }

    #[test]
    fn test_diff_json_modified() {
        let old = serde_json::json!({"a": 1});
        let new = serde_json::json!({"a": 2});

        let changes = diff_json(&old, &new);
        assert_eq!(changes.len(), 1);
        assert!(matches!(&changes[0], JsonChange::Modified { path, .. } if path == "$.a"));
    }

    #[test]
    fn test_json_change_to_ucl() {
        let change = JsonChange::Modified {
            path: "$.name".to_string(),
            old_value: serde_json::json!("old"),
            new_value: serde_json::json!("new"),
        };

        let ucl = change.to_ucl("blk_123");
        assert!(ucl.contains("EDIT blk_123 SET $.name"));
        assert!(ucl.contains("\"new\""));
    }

    #[test]
    fn test_ucl_builder() {
        let ucl = UclBuilder::new()
            .edit("blk_1", "text", "hello")
            .append("blk_2", "text", "world")
            .build();

        assert!(ucl.contains("EDIT blk_1 SET text = \"hello\""));
        assert!(ucl.contains("APPEND blk_2 text :: world"));
    }

    #[test]
    fn test_ucl_builder_atomic() {
        let ucl = UclBuilder::new()
            .edit("blk_1", "text", "hello")
            .delete("blk_2")
            .atomic()
            .build();

        assert!(ucl.contains("ATOMIC {"));
        assert!(ucl.contains("EDIT blk_1"));
        assert!(ucl.contains("DELETE blk_2"));
    }

    #[test]
    fn test_document_inspector() {
        let doc = Document::create();
        let inspector = DocumentInspector::new(&doc);

        let summary = inspector.summary();
        assert_eq!(summary.block_count, 1); // Just root

        let json = inspector.to_json();
        assert!(json.get("root").is_some());
        assert!(json.get("blocks").is_some());
    }
}
