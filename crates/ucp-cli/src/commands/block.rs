//! Block operations commands

use anyhow::{anyhow, Result};
use serde::Serialize;
use std::str::FromStr;
use ucm_core::{Block, BlockId, Content};
use ucm_engine::{EditOperator, Engine, MoveTarget, Operation};

use crate::cli::{BlockCommands, OutputFormat};
use crate::output::{
    print_block, print_block_table, print_error, print_success, read_document, write_document,
    BlockSummary,
};

/// Serializable version of OperationResult for JSON output
#[derive(Serialize)]
struct OperationResultJson {
    success: bool,
    affected_blocks: Vec<String>,
    warnings: Vec<String>,
    error: Option<String>,
}

impl From<&ucm_engine::OperationResult> for OperationResultJson {
    fn from(result: &ucm_engine::OperationResult) -> Self {
        Self {
            success: result.success,
            affected_blocks: result.affected_blocks.iter().map(|id| id.to_string()).collect(),
            warnings: result.warnings.clone(),
            error: result.error.clone(),
        }
    }
}

pub fn handle(cmd: BlockCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        BlockCommands::Add {
            input,
            output,
            parent,
            content_type,
            content,
            language,
            label,
            role,
            tags,
        } => add(
            input,
            output,
            parent,
            content_type,
            content,
            language,
            label,
            role,
            tags,
            format,
        ),
        BlockCommands::Get {
            input,
            id,
            metadata,
        } => get(input, id, metadata, format),
        BlockCommands::Delete {
            input,
            output,
            id,
            cascade,
            preserve_children,
        } => delete(input, output, id, cascade, preserve_children, format),
        BlockCommands::Move {
            input,
            output,
            id,
            to_parent,
            before,
            after,
            index,
        } => move_block(input, output, id, to_parent, before, after, index, format),
        BlockCommands::List { input, ids_only } => list(input, ids_only, format),
        BlockCommands::Update {
            input,
            output,
            id,
            content,
            label,
            role,
            summary,
            add_tag,
            remove_tag,
        } => update(
            input, output, id, content, label, role, summary, add_tag, remove_tag, format,
        ),
    }
}

fn add(
    input: Option<String>,
    output: Option<String>,
    parent: Option<String>,
    content_type: String,
    content: Option<String>,
    language: Option<String>,
    label: Option<String>,
    role: Option<String>,
    tags: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let mut doc = if input.is_some() {
        read_document(input)?
    } else {
        ucm_core::Document::create()
    };

    let parent_id = if let Some(p) = parent {
        BlockId::from_str(&p).map_err(|_| anyhow!("Invalid parent block ID: {}", p))?
    } else {
        doc.root
    };

    // Get content from argument or stdin
    let content_str = if let Some(c) = content {
        c
    } else {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        buffer.trim().to_string()
    };

    // Build content based on type
    let block_content = match content_type.to_lowercase().as_str() {
        "text" => Content::text(&content_str),
        "markdown" => Content::markdown(&content_str),
        "code" => {
            let lang = language.unwrap_or_else(|| "plaintext".to_string());
            Content::code(&lang, &content_str)
        }
        "json" => Content::json(serde_json::from_str(&content_str)?),
        "math" => Content::Math(ucm_core::Math {
            expression: content_str,
            format: ucm_core::MathFormat::LaTeX,
            display_mode: false,
        }),
        "table" => {
            // Expect JSON table definition
            let table: ucm_core::Table = serde_json::from_str(&content_str)?;
            Content::Table(table)
        }
        "media" => {
            // Expect JSON media definition
            let media: ucm_core::Media = serde_json::from_str(&content_str)?;
            Content::Media(media)
        }
        _ => return Err(anyhow!("Unknown content type: {}", content_type)),
    };

    // Build block with metadata
    let mut block = Block::new(block_content, role.as_deref());

    if let Some(l) = label {
        block.metadata.label = Some(l);
    }

    if let Some(t) = tags {
        block.metadata.tags = t.split(',').map(|s| s.trim().to_string()).collect();
    }

    let block_id = doc.add_block(block, &parent_id)?;

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct AddResult {
                success: bool,
                block_id: String,
            }
            let result = AddResult {
                success: true,
                block_id: block_id.to_string(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            print_success(&format!("Added block: {}", block_id));
        }
    }

    write_document(&doc, output)?;
    Ok(())
}

fn get(input: Option<String>, id: String, metadata_only: bool, format: OutputFormat) -> Result<()> {
    let doc = read_document(input)?;
    let block_id =
        BlockId::from_str(&id).map_err(|_| anyhow!("Invalid block ID: {}", id))?;

    let block = doc
        .get_block(&block_id)
        .ok_or_else(|| anyhow!("Block not found: {}", id))?;

    match format {
        OutputFormat::Json => {
            if metadata_only {
                println!("{}", serde_json::to_string_pretty(&block.metadata)?);
            } else {
                println!("{}", serde_json::to_string_pretty(block)?);
            }
        }
        OutputFormat::Text => {
            print_block(block, !metadata_only);
        }
    }

    Ok(())
}

fn delete(
    input: Option<String>,
    output: Option<String>,
    id: String,
    cascade: bool,
    preserve_children: bool,
    format: OutputFormat,
) -> Result<()> {
    let mut doc = read_document(input)?;
    let block_id =
        BlockId::from_str(&id).map_err(|_| anyhow!("Invalid block ID: {}", id))?;

    let engine = Engine::new();
    let op = Operation::Delete {
        block_id,
        cascade,
        preserve_children,
    };

    let result = engine.execute(&mut doc, op)?;

    match format {
        OutputFormat::Json => {
            let json_result = OperationResultJson::from(&result);
            println!("{}", serde_json::to_string_pretty(&json_result)?);
        }
        OutputFormat::Text => {
            if result.success {
                print_success(&format!(
                    "Deleted block {} ({} affected)",
                    id,
                    result.affected_blocks.len()
                ));
            } else {
                print_error(&format!(
                    "Failed to delete block: {}",
                    result.error.unwrap_or_default()
                ));
            }
        }
    }

    write_document(&doc, output)?;
    Ok(())
}

fn move_block(
    input: Option<String>,
    output: Option<String>,
    id: String,
    to_parent: Option<String>,
    before: Option<String>,
    after: Option<String>,
    index: Option<usize>,
    format: OutputFormat,
) -> Result<()> {
    let mut doc = read_document(input)?;
    let block_id =
        BlockId::from_str(&id).map_err(|_| anyhow!("Invalid block ID: {}", id))?;

    let target = if let Some(parent) = to_parent {
        let parent_id = BlockId::from_str(&parent)
            .map_err(|_| anyhow!("Invalid parent block ID: {}", parent))?;
        MoveTarget::ToParent {
            parent_id,
            index,
        }
    } else if let Some(sibling) = before {
        let sibling_id = BlockId::from_str(&sibling)
            .map_err(|_| anyhow!("Invalid sibling block ID: {}", sibling))?;
        MoveTarget::Before { sibling_id }
    } else if let Some(sibling) = after {
        let sibling_id = BlockId::from_str(&sibling)
            .map_err(|_| anyhow!("Invalid sibling block ID: {}", sibling))?;
        MoveTarget::After { sibling_id }
    } else {
        return Err(anyhow!(
            "Must specify --to-parent, --before, or --after"
        ));
    };

    let engine = Engine::new();
    let op = Operation::MoveToTarget {
        block_id,
        target,
    };

    let result = engine.execute(&mut doc, op)?;

    match format {
        OutputFormat::Json => {
            let json_result = OperationResultJson::from(&result);
            println!("{}", serde_json::to_string_pretty(&json_result)?);
        }
        OutputFormat::Text => {
            if result.success {
                print_success(&format!("Moved block {}", id));
            } else {
                print_error(&format!(
                    "Failed to move block: {}",
                    result.error.unwrap_or_default()
                ));
            }
        }
    }

    write_document(&doc, output)?;
    Ok(())
}

fn list(input: Option<String>, ids_only: bool, format: OutputFormat) -> Result<()> {
    let doc = read_document(input)?;

    let blocks: Vec<&Block> = doc.blocks.values().collect();

    match format {
        OutputFormat::Json => {
            if ids_only {
                let ids: Vec<String> = blocks.iter().map(|b| b.id.to_string()).collect();
                println!("{}", serde_json::to_string_pretty(&ids)?);
            } else {
                let summaries: Vec<BlockSummary> =
                    blocks.iter().map(|b| BlockSummary::from_block(b)).collect();
                println!("{}", serde_json::to_string_pretty(&summaries)?);
            }
        }
        OutputFormat::Text => {
            if ids_only {
                for block in &blocks {
                    println!("{}", block.id);
                }
            } else {
                print_block_table(&blocks);
            }
        }
    }

    Ok(())
}

fn update(
    input: Option<String>,
    output: Option<String>,
    id: String,
    content: Option<String>,
    label: Option<String>,
    role: Option<String>,
    summary: Option<String>,
    add_tag: Option<String>,
    remove_tag: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let mut doc = read_document(input)?;
    let block_id =
        BlockId::from_str(&id).map_err(|_| anyhow!("Invalid block ID: {}", id))?;

    let engine = Engine::new();
    let mut results = Vec::new();

    // Update content if provided
    if let Some(new_content) = content {
        let op = Operation::Edit {
            block_id,
            path: "content.text".to_string(),
            value: serde_json::Value::String(new_content),
            operator: EditOperator::Set,
        };
        results.push(engine.execute(&mut doc, op)?);
    }

    // Update label
    if let Some(new_label) = label {
        let op = Operation::Edit {
            block_id,
            path: "metadata.label".to_string(),
            value: serde_json::Value::String(new_label),
            operator: EditOperator::Set,
        };
        results.push(engine.execute(&mut doc, op)?);
    }

    // Update role
    if let Some(new_role) = role {
        let op = Operation::Edit {
            block_id,
            path: "metadata.semantic_role".to_string(),
            value: serde_json::Value::String(new_role),
            operator: EditOperator::Set,
        };
        results.push(engine.execute(&mut doc, op)?);
    }

    // Update summary
    if let Some(new_summary) = summary {
        let op = Operation::Edit {
            block_id,
            path: "metadata.summary".to_string(),
            value: serde_json::Value::String(new_summary),
            operator: EditOperator::Set,
        };
        results.push(engine.execute(&mut doc, op)?);
    }

    // Add tag
    if let Some(tag) = add_tag {
        let op = Operation::Edit {
            block_id,
            path: "metadata.tags".to_string(),
            value: serde_json::Value::String(tag),
            operator: EditOperator::Append,
        };
        results.push(engine.execute(&mut doc, op)?);
    }

    // Remove tag
    if let Some(tag) = remove_tag {
        let op = Operation::Edit {
            block_id,
            path: "metadata.tags".to_string(),
            value: serde_json::Value::String(tag),
            operator: EditOperator::Remove,
        };
        results.push(engine.execute(&mut doc, op)?);
    }

    let success = results.iter().all(|r| r.success);

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct UpdateResult {
                success: bool,
                operations: usize,
            }
            let result = UpdateResult {
                success,
                operations: results.len(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            if success {
                print_success(&format!(
                    "Updated block {} ({} operations)",
                    id,
                    results.len()
                ));
            } else {
                print_error("Some updates failed");
            }
        }
    }

    write_document(&doc, output)?;
    Ok(())
}
