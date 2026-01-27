//! UCL (Unified Content Language) commands

use anyhow::Result;
use serde::Serialize;
use std::str::FromStr;
use ucm_core::{BlockId, Content, EdgeType};
use ucm_engine::{EditOperator, Engine, MoveTarget, Operation, PruneCondition};

use crate::cli::{OutputFormat, UclCommands};
use crate::output::{print_error, print_success, read_document, read_file, write_document};

pub fn handle(cmd: UclCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        UclCommands::Exec {
            input,
            output,
            commands,
            file,
        } => exec(input, output, commands, file, format),
        UclCommands::Parse { commands, file } => parse(commands, file, format),
    }
}

fn exec(
    input: Option<String>,
    output: Option<String>,
    commands: Option<String>,
    file: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let mut doc = read_document(input)?;

    // Get UCL from argument, file, or stdin
    let ucl = if let Some(cmd) = commands {
        cmd
    } else if let Some(f) = file {
        read_file(&f)?
    } else {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    // Parse and execute commands
    let parsed = ucl_parser::parse_commands(&ucl)
        .map_err(|e| anyhow::anyhow!("Parse error: {:?}", e))?;

    // Convert to operations and execute
    let mut results = Vec::new();
    let engine = Engine::new();

    for cmd in &parsed {
        match command_to_operation(cmd) {
            Ok(op) => {
                let result = engine.execute(&mut doc, op)?;
                results.push(result);
            }
            Err(e) => {
                results.push(ucm_engine::OperationResult {
                    success: false,
                    affected_blocks: vec![],
                    warnings: vec![],
                    error: Some(e.to_string()),
                });
            }
        }
    }

    let success_count = results.iter().filter(|r| r.success).count();
    let total = results.len();

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct ExecResult {
                success: bool,
                commands_executed: usize,
                commands_succeeded: usize,
            }
            let result = ExecResult {
                success: success_count == total,
                commands_executed: total,
                commands_succeeded: success_count,
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            if success_count == total {
                print_success(&format!("Executed {} UCL commands", total));
            } else {
                print_error(&format!(
                    "Executed {}/{} UCL commands successfully",
                    success_count, total
                ));
            }
        }
    }

    write_document(&doc, output)?;
    Ok(())
}

fn parse(commands: Option<String>, file: Option<String>, format: OutputFormat) -> Result<()> {
    // Get UCL from argument, file, or stdin
    let ucl = if let Some(cmd) = commands {
        cmd
    } else if let Some(f) = file {
        read_file(&f)?
    } else {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    match ucl_parser::parse_commands(&ucl) {
        Ok(parsed) => {
            match format {
                OutputFormat::Json => {
                    #[derive(Serialize)]
                    struct ParseResult {
                        valid: bool,
                        command_count: usize,
                        commands: Vec<String>,
                    }
                    let result = ParseResult {
                        valid: true,
                        command_count: parsed.len(),
                        commands: parsed.iter().map(|c| format!("{:?}", c)).collect(),
                    };
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
                OutputFormat::Text => {
                    print_success(&format!("Valid UCL ({} commands)", parsed.len()));
                    for (i, cmd) in parsed.iter().enumerate() {
                        println!("  {}: {:?}", i + 1, cmd);
                    }
                }
            }
        }
        Err(e) => {
            match format {
                OutputFormat::Json => {
                    #[derive(Serialize)]
                    struct ParseResult {
                        valid: bool,
                        error: String,
                    }
                    let result = ParseResult {
                        valid: false,
                        error: format!("{:?}", e),
                    };
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
                OutputFormat::Text => {
                    print_error(&format!("Invalid UCL: {:?}", e));
                }
            }
        }
    }

    Ok(())
}

fn command_to_operation(cmd: &ucl_parser::Command) -> Result<Operation> {
    match cmd {
        ucl_parser::Command::Edit(e) => {
            let block_id = BlockId::from_str(&e.block_id)
                .map_err(|_| anyhow::anyhow!("Invalid block ID: {}", e.block_id))?;
            Ok(Operation::Edit {
                block_id,
                path: e.path.to_string(),
                value: e.value.to_json(),
                operator: match e.operator {
                    ucl_parser::Operator::Set => EditOperator::Set,
                    ucl_parser::Operator::Append => EditOperator::Append,
                    ucl_parser::Operator::Remove => EditOperator::Remove,
                    ucl_parser::Operator::Increment => EditOperator::Increment,
                    ucl_parser::Operator::Decrement => EditOperator::Decrement,
                },
            })
        }
        ucl_parser::Command::Append(a) => {
            let parent_id = BlockId::from_str(&a.parent_id)
                .map_err(|_| anyhow::anyhow!("Invalid parent ID: {}", a.parent_id))?;
            let content = match a.content_type {
                ucl_parser::ContentType::Text => Content::text(&a.content),
                ucl_parser::ContentType::Code => Content::code("", &a.content),
                _ => Content::text(&a.content),
            };
            Ok(Operation::Append {
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
            })
        }
        ucl_parser::Command::Delete(d) => {
            if let Some(id) = &d.block_id {
                let block_id = BlockId::from_str(id)
                    .map_err(|_| anyhow::anyhow!("Invalid block ID: {}", id))?;
                Ok(Operation::Delete {
                    block_id,
                    cascade: d.cascade,
                    preserve_children: d.preserve_children,
                })
            } else {
                Err(anyhow::anyhow!("Delete command requires a block ID"))
            }
        }
        ucl_parser::Command::Move(m) => {
            let block_id = BlockId::from_str(&m.block_id)
                .map_err(|_| anyhow::anyhow!("Invalid block ID: {}", m.block_id))?;
            let target = match &m.target {
                ucl_parser::MoveTarget::ToParent { parent_id, index } => {
                    let pid = BlockId::from_str(parent_id)
                        .map_err(|_| anyhow::anyhow!("Invalid parent ID: {}", parent_id))?;
                    MoveTarget::ToParent {
                        parent_id: pid,
                        index: *index,
                    }
                }
                ucl_parser::MoveTarget::Before { sibling_id } => {
                    let sid = BlockId::from_str(sibling_id)
                        .map_err(|_| anyhow::anyhow!("Invalid sibling ID: {}", sibling_id))?;
                    MoveTarget::Before { sibling_id: sid }
                }
                ucl_parser::MoveTarget::After { sibling_id } => {
                    let sid = BlockId::from_str(sibling_id)
                        .map_err(|_| anyhow::anyhow!("Invalid sibling ID: {}", sibling_id))?;
                    MoveTarget::After { sibling_id: sid }
                }
            };
            Ok(Operation::MoveToTarget { block_id, target })
        }
        ucl_parser::Command::Prune(p) => {
            let condition = match &p.target {
                ucl_parser::PruneTarget::Unreachable => Some(PruneCondition::Unreachable),
                ucl_parser::PruneTarget::Where(_) => None,
            };
            Ok(Operation::Prune { condition })
        }
        ucl_parser::Command::Link(l) => {
            let source = BlockId::from_str(&l.source_id)
                .map_err(|_| anyhow::anyhow!("Invalid source ID: {}", l.source_id))?;
            let target = BlockId::from_str(&l.target_id)
                .map_err(|_| anyhow::anyhow!("Invalid target ID: {}", l.target_id))?;
            let edge_type = EdgeType::from_str(&l.edge_type).unwrap_or(EdgeType::References);
            Ok(Operation::Link {
                source,
                edge_type,
                target,
                metadata: None,
            })
        }
        ucl_parser::Command::Unlink(u) => {
            let source = BlockId::from_str(&u.source_id)
                .map_err(|_| anyhow::anyhow!("Invalid source ID: {}", u.source_id))?;
            let target = BlockId::from_str(&u.target_id)
                .map_err(|_| anyhow::anyhow!("Invalid target ID: {}", u.target_id))?;
            let edge_type = EdgeType::from_str(&u.edge_type).unwrap_or(EdgeType::References);
            Ok(Operation::Unlink {
                source,
                edge_type,
                target,
            })
        }
        ucl_parser::Command::Snapshot(s) => match s {
            ucl_parser::SnapshotCommand::Create { name, description } => {
                Ok(Operation::CreateSnapshot {
                    name: name.clone(),
                    description: description.clone(),
                })
            }
            ucl_parser::SnapshotCommand::Restore { name } => Ok(Operation::RestoreSnapshot {
                name: name.clone(),
            }),
            _ => Err(anyhow::anyhow!("Unsupported snapshot operation")),
        },
        ucl_parser::Command::WriteSection(ws) => {
            let section_id = BlockId::from_str(&ws.section_id)
                .map_err(|_| anyhow::anyhow!("Invalid section ID: {}", ws.section_id))?;
            Ok(Operation::WriteSection {
                section_id,
                markdown: ws.markdown.clone(),
                base_heading_level: ws.base_heading_level,
            })
        }
        _ => Err(anyhow::anyhow!("Unsupported UCL command: {:?}", cmd)),
    }
}
