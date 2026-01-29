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
            affected_blocks: result
                .affected_blocks
                .iter()
                .map(|id| id.to_string())
                .collect(),
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
            AddArgs {
                input,
                output,
                parent,
                content_type,
                content,
                language,
                label,
                role,
                tags,
            },
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
        } => move_block(
            MoveBlockArgs {
                input,
                output,
                id,
                to_parent,
                before,
                after,
                index,
            },
            format,
        ),
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
            UpdateArgs {
                input,
                output,
                id,
                content,
                label,
                role,
                summary,
                add_tag,
                remove_tag,
            },
            format,
        ),
    }
}

#[derive(clap::Args)]
struct AddArgs {
    /// Input document file
    #[arg(short, long)]
    input: Option<String>,

    /// Output document file
    #[arg(short, long)]
    output: Option<String>,

    /// Parent block ID
    #[arg(short, long)]
    parent: Option<String>,

    /// Content type
    #[arg(short = 't', long, default_value = "text")]
    content_type: String,

    /// Block content (reads from stdin if not provided)
    #[arg(short, long)]
    content: Option<String>,

    /// Language for code blocks
    #[arg(short, long)]
    language: Option<String>,

    /// Block label
    #[arg(short, long)]
    label: Option<String>,

    /// Semantic role
    #[arg(short, long)]
    role: Option<String>,

    /// Comma-separated tags
    #[arg(short, long)]
    tags: Option<String>,
}

fn add(args: AddArgs, format: OutputFormat) -> Result<()> {
    let mut doc = if args.input.is_some() {
        read_document(args.input.as_deref().map(|s| s.to_string()))?
    } else {
        ucm_core::Document::create()
    };

    let content = match args.content {
        Some(c) => c,
        None => {
            use std::io::Read;
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            buffer.trim().to_string()
        }
    };

    let content = match args.content_type.to_lowercase().as_str() {
        "text" => Content::text(&content),
        "code" => {
            let lang = args.language.unwrap_or_else(|| "plaintext".to_string());
            Content::code(&lang, &content)
        }
        "markdown" => Content::markdown(&content),
        "json" => Content::json(serde_json::from_str(&content)?),
        "math" => Content::Math(ucm_core::Math {
            expression: content,
            format: ucm_core::MathFormat::LaTeX,
            display_mode: false,
        }),
        "table" => {
            // Expect JSON table definition
            let table: ucm_core::Table = serde_json::from_str(&content)?;
            Content::Table(table)
        }
        "media" => {
            // Expect JSON media definition
            let media: ucm_core::Media = serde_json::from_str(&content)?;
            Content::Media(media)
        }
        _ => return Err(anyhow!("Unknown content type: {}", args.content_type)),
    };

    let parent_id = args
        .parent
        .map(|p| {
            p.parse()
                .map_err(|e| anyhow::anyhow!("Invalid parent ID: {}", e))
        })
        .transpose()?
        .unwrap_or_else(|| doc.root);

    let block = Block {
        id: BlockId::generate(),
        content,
        metadata: BlockMetadata {
            label: args.label,
            role: args.role,
            summary: None,
            tags: args
                .tags
                .as_deref()
                .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default(),
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
        },
        children: Vec::new(),
        edges: Vec::new(),
    };

    let block_id = doc.add_block(block, &parent_id)?;

    write_document(&doc, args.output)?;

    match format {
        OutputFormat::Text => {
            print_success("Block added successfully");
            println!("Block ID: {}", block_id);
        }
        OutputFormat::Json => {
            let result = serde_json::json!({
                "block_id": block_id,
                "status": "success"
            });
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    }

    Ok(())
}

fn get(input: Option<String>, id: String, metadata_only: bool, format: OutputFormat) -> Result<()> {
    let doc = read_document(input)?;
    let block_id = BlockId::from_str(&id).map_err(|_| anyhow!("Invalid block ID: {}", id))?;

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
    let block_id = BlockId::from_str(&id).map_err(|_| anyhow!("Invalid block ID: {}", id))?;

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

#[derive(clap::Args)]
struct MoveBlockArgs {
    /// Input document file
    #[arg(short, long)]
    input: Option<String>,

    /// Output document file
    #[arg(short, long)]
    output: Option<String>,

    /// Block ID to move
    #[arg(short, long)]
    id: String,

    /// Target parent block ID
    #[arg(long)]
    to_parent: Option<String>,

    /// Insert before this block ID
    #[arg(long)]
    before: Option<String>,

    /// Insert after this block ID
    #[arg(long)]
    after: Option<String>,

    /// Index at which to insert
    #[arg(long)]
    index: Option<usize>,
}

#[derive(Debug)]
struct MoveBlockTarget {
    to_parent: Option<BlockId>,
    before: Option<BlockId>,
    after: Option<BlockId>,
    index: Option<usize>,
}

fn move_block(args: MoveBlockArgs, format: OutputFormat) -> Result<()> {
    let mut doc = read_document(args.input)?;
    let block_id =
        BlockId::from_str(&args.id).map_err(|_| anyhow!("Invalid block ID: {}", args.id))?;

    let target = MoveBlockTarget {
        to_parent: args
            .to_parent
            .map(|p| BlockId::from_str(&p).map_err(|_| anyhow!("Invalid parent block ID: {}", p)))
            .transpose(),
        before: args
            .before
            .map(|b| BlockId::from_str(&b).map_err(|_| anyhow!("Invalid before block ID: {}", b)))
            .transpose(),
        after: args
            .after
            .map(|a| BlockId::from_str(&a).map_err(|_| anyhow!("Invalid after block ID: {}", a)))
            .transpose(),
        index: args.index,
    };

    let target = match (target.to_parent, target.before, target.after) {
        (Some(parent), _, _) => MoveTarget::ToParent {
            parent_id: parent,
            index: target.index,
        },
        (_, Some(before), _) => MoveTarget::Before { sibling_id: before },
        (_, _, Some(after)) => MoveTarget::After { sibling_id: after },
        _ => return Err(anyhow!("Must specify --to-parent, --before, or --after")),
    };

    let engine = Engine::new();
    let op = Operation::MoveToTarget { block_id, target };

    let result = engine.execute(&mut doc, op)?;

    match format {
        OutputFormat::Json => {
            let json_result = OperationResultJson::from(&result);
            println!("{}", serde_json::to_string_pretty(&json_result)?);
        }
        OutputFormat::Text => {
            if result.success {
                print_success(&format!("Moved block {}", args.id));
            } else {
                print_error(&format!(
                    "Failed to move block: {}",
                    result.error.unwrap_or_default()
                ));
            }
        }
    }

    write_document(&doc, args.output)?;
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

#[derive(clap::Args)]
struct UpdateArgs {
    /// Input document file
    #[arg(short, long)]
    input: Option<String>,

    /// Output document file
    #[arg(short, long)]
    output: Option<String>,

    /// Block ID to update
    #[arg(short, long)]
    id: String,

    /// New block content
    #[arg(short, long)]
    content: Option<String>,

    /// Block label
    #[arg(short, long)]
    label: Option<String>,

    /// Semantic role
    #[arg(short, long)]
    role: Option<String>,

    /// Block summary
    #[arg(long)]
    summary: Option<String>,

    /// Tags to add
    #[arg(long)]
    add_tag: Option<String>,

    /// Tags to remove
    #[arg(long)]
    remove_tag: Option<String>,
}

fn update(args: UpdateArgs, format: OutputFormat) -> Result<()> {
    let mut doc = read_document(args.input)?;
    let block_id =
        BlockId::from_str(&args.id).map_err(|_| anyhow!("Invalid block ID: {}", args.id))?;

    let engine = Engine::new();
    let mut results = Vec::new();

    // Update content if provided
    if let Some(new_content) = args.content {
        let op = Operation::Edit {
            block_id,
            path: "content.text".to_string(),
            value: serde_json::Value::String(new_content),
            operator: EditOperator::Set,
        };
        results.push(engine.execute(&mut doc, op)?);
    }

    // Update label if provided
    if let Some(new_label) = args.label {
        let op = Operation::Edit {
            block_id,
            path: "metadata.label".to_string(),
            value: serde_json::Value::String(new_label),
            operator: EditOperator::Set,
        };
        results.push(engine.execute(&mut doc, op)?);
    }

    // Update role if provided
    if let Some(new_role) = args.role {
        let op = Operation::Edit {
            block_id,
            path: "metadata.semantic_role".to_string(),
            value: serde_json::Value::String(new_role),
            operator: EditOperator::Set,
        };
        results.push(engine.execute(&mut doc, op)?);
    }

    // Update summary if provided
    if let Some(new_summary) = args.summary {
        let op = Operation::Edit {
            block_id,
            path: "metadata.summary".to_string(),
            value: serde_json::Value::String(new_summary),
            operator: EditOperator::Set,
        };
        results.push(engine.execute(&mut doc, op)?);
    }

    // Add tags if provided
    if let Some(tags) = args.add_tag {
        let op = Operation::Edit {
            block_id,
            path: "metadata.tags".to_string(),
            value: serde_json::Value::String(tags),
            operator: EditOperator::Append,
        };
        results.push(engine.execute(&mut doc, op)?);
    }

    // Remove tags if provided
    if let Some(tags) = args.remove_tag {
        let op = Operation::Edit {
            block_id,
            path: "metadata.tags".to_string(),
            value: serde_json::Value::String(tags),
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
                    args.id,
                    results.len()
                ));
            } else {
                print_error("Some updates failed");
            }
        }
    }

    write_document(&doc, args.output)?;
    Ok(())
}
