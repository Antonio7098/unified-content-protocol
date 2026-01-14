//! High-level API for UCP.

use std::str::FromStr;

use ucl_parser::{parse, parse_commands, UclDocument};
use ucm_core::{Block, BlockId, Content, Document, EdgeType, Error, Result};
use ucm_engine::{Engine, Operation, OperationResult};

/// UCP client for document manipulation
pub struct UcpClient {
    engine: Engine,
}

impl UcpClient {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
        }
    }

    /// Create a new document
    pub fn create_document(&self) -> Document {
        Document::create()
    }

    /// Execute UCL commands on a document
    pub fn execute_ucl(&self, doc: &mut Document, ucl: &str) -> Result<Vec<OperationResult>> {
        let commands =
            parse_commands(ucl).map_err(|e| Error::Internal(format!("Parse error: {}", e)))?;

        let ops = self.commands_to_operations(commands)?;
        self.engine.execute_batch(doc, ops)
    }

    /// Parse a full UCL document
    pub fn parse_ucl(&self, ucl: &str) -> Result<UclDocument> {
        parse(ucl).map_err(|e| Error::Internal(format!("Parse error: {}", e)))
    }

    /// Add a text block
    pub fn add_text(
        &self,
        doc: &mut Document,
        parent: &BlockId,
        text: &str,
        role: Option<&str>,
    ) -> Result<BlockId> {
        let block = Block::new(Content::text(text), role);
        doc.add_block(block, parent)
    }

    /// Add a code block  
    pub fn add_code(
        &self,
        doc: &mut Document,
        parent: &BlockId,
        lang: &str,
        code: &str,
    ) -> Result<BlockId> {
        let block = Block::new(Content::code(lang, code), None);
        doc.add_block(block, parent)
    }

    /// Get document as JSON
    pub fn to_json(&self, doc: &Document) -> Result<String> {
        // Serialize blocks
        let blocks: Vec<_> = doc.blocks.values().collect();
        serde_json::to_string_pretty(&blocks)
            .map_err(|e| Error::Internal(format!("Serialization error: {}", e)))
    }

    fn commands_to_operations(&self, commands: Vec<ucl_parser::Command>) -> Result<Vec<Operation>> {
        let mut ops = Vec::new();
        for cmd in commands {
            match cmd {
                ucl_parser::Command::Edit(e) => {
                    let block_id: BlockId = e
                        .block_id
                        .parse()
                        .map_err(|_| Error::InvalidBlockId(e.block_id.clone()))?;
                    ops.push(Operation::Edit {
                        block_id,
                        path: e.path.to_string(),
                        value: e.value.to_json(),
                        operator: match e.operator {
                            ucl_parser::Operator::Set => ucm_engine::EditOperator::Set,
                            ucl_parser::Operator::Append => ucm_engine::EditOperator::Append,
                            ucl_parser::Operator::Remove => ucm_engine::EditOperator::Remove,
                            ucl_parser::Operator::Increment => ucm_engine::EditOperator::Increment,
                            ucl_parser::Operator::Decrement => ucm_engine::EditOperator::Decrement,
                        },
                    });
                }
                ucl_parser::Command::Append(a) => {
                    let parent_id: BlockId = a
                        .parent_id
                        .parse()
                        .map_err(|_| Error::InvalidBlockId(a.parent_id.clone()))?;
                    let content = match a.content_type {
                        ucl_parser::ContentType::Text => Content::text(&a.content),
                        ucl_parser::ContentType::Code => Content::code("", &a.content),
                        _ => Content::text(&a.content),
                    };
                    ops.push(Operation::Append {
                        parent_id,
                        content,
                        label: a.properties.get("label").and_then(|v| match v {
                            ucl_parser::Value::String(s) => Some(s.clone()),
                            _ => None,
                        }),
                        tags: Vec::new(),
                        semantic_role: a.properties.get("role").and_then(|v| match v {
                            ucl_parser::Value::String(s) => Some(s.clone()),
                            _ => None,
                        }),
                        index: a.index,
                    });
                }
                ucl_parser::Command::Delete(d) => {
                    if let Some(id) = d.block_id {
                        let block_id: BlockId =
                            id.parse().map_err(|_| Error::InvalidBlockId(id.clone()))?;
                        ops.push(Operation::Delete {
                            block_id,
                            cascade: d.cascade,
                            preserve_children: d.preserve_children,
                        });
                    }
                }
                ucl_parser::Command::Move(m) => {
                    let block_id: BlockId = m
                        .block_id
                        .parse()
                        .map_err(|_| Error::InvalidBlockId(m.block_id.clone()))?;
                    match m.target {
                        ucl_parser::MoveTarget::ToParent { parent_id, index } => {
                            let new_parent: BlockId = parent_id
                                .parse()
                                .map_err(|_| Error::InvalidBlockId(parent_id.clone()))?;
                            ops.push(Operation::MoveToTarget {
                                block_id,
                                target: ucm_engine::MoveTarget::ToParent {
                                    parent_id: new_parent,
                                    index,
                                },
                            });
                        }
                        ucl_parser::MoveTarget::Before { sibling_id } => {
                            let sibling: BlockId = sibling_id
                                .parse()
                                .map_err(|_| Error::InvalidBlockId(sibling_id.clone()))?;
                            ops.push(Operation::MoveToTarget {
                                block_id,
                                target: ucm_engine::MoveTarget::Before {
                                    sibling_id: sibling,
                                },
                            });
                        }
                        ucl_parser::MoveTarget::After { sibling_id } => {
                            let sibling: BlockId = sibling_id
                                .parse()
                                .map_err(|_| Error::InvalidBlockId(sibling_id.clone()))?;
                            ops.push(Operation::MoveToTarget {
                                block_id,
                                target: ucm_engine::MoveTarget::After {
                                    sibling_id: sibling,
                                },
                            });
                        }
                    }
                }
                ucl_parser::Command::Prune(p) => {
                    let condition = match p.target {
                        ucl_parser::PruneTarget::Unreachable => {
                            Some(ucm_engine::PruneCondition::Unreachable)
                        }
                        _ => None,
                    };
                    ops.push(Operation::Prune { condition });
                }
                ucl_parser::Command::Link(l) => {
                    let source: BlockId = l
                        .source_id
                        .parse()
                        .map_err(|_| Error::InvalidBlockId(l.source_id.clone()))?;
                    let target: BlockId = l
                        .target_id
                        .parse()
                        .map_err(|_| Error::InvalidBlockId(l.target_id.clone()))?;
                    let edge_type =
                        EdgeType::from_str(&l.edge_type).unwrap_or(EdgeType::References);
                    ops.push(Operation::Link {
                        source,
                        edge_type,
                        target,
                        metadata: None,
                    });
                }
                ucl_parser::Command::Snapshot(s) => match s {
                    ucl_parser::SnapshotCommand::Create { name, description } => {
                        ops.push(Operation::CreateSnapshot { name, description });
                    }
                    ucl_parser::SnapshotCommand::Restore { name } => {
                        ops.push(Operation::RestoreSnapshot { name });
                    }
                    _ => {}
                },
                _ => {} // Other commands
            }
        }
        Ok(ops)
    }
}

impl Default for UcpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_document() {
        let client = UcpClient::new();
        let doc = client.create_document();
        assert_eq!(doc.block_count(), 1);
    }

    #[test]
    fn test_add_text() {
        let client = UcpClient::new();
        let mut doc = client.create_document();
        let root = doc.root;

        let id = client
            .add_text(&mut doc, &root, "Hello, world!", Some("intro"))
            .unwrap();
        assert_eq!(doc.block_count(), 2);
        assert!(doc.get_block(&id).is_some());
    }
}
