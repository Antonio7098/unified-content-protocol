//! Edge operations commands

use anyhow::{anyhow, Result};
use serde::Serialize;
use std::str::FromStr;
use ucm_core::{BlockId, Edge, EdgeType};
use ucm_engine::{Engine, Operation};

use crate::cli::{EdgeCommands, OutputFormat};
use crate::output::{
    print_edge_table, print_error, print_success, read_document, write_document, EdgeSummary,
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

pub fn handle(cmd: EdgeCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        EdgeCommands::Add {
            input,
            output,
            source,
            edge_type,
            target,
            description,
            confidence,
        } => add(AddEdgeArgs {
            input,
            output,
            source,
            edge_type,
            target,
            description,
            confidence,
        }, format),
        EdgeCommands::Remove {
            input,
            output,
            source,
            edge_type,
            target,
        } => remove(input, output, source, edge_type, target, format),
        EdgeCommands::List {
            input,
            id,
            outgoing,
            incoming,
        } => list(input, id, outgoing, incoming, format),
    }
}

#[derive(clap::Args)]
struct AddEdgeArgs {
    /// Input document file
    #[arg(short, long)]
    input: Option<String>,

    /// Output document file
    #[arg(short, long)]
    output: Option<String>,

    /// Source block ID
    #[arg(short, long)]
    source: String,

    /// Edge type
    #[arg(short = 't', long)]
    edge_type: String,

    /// Target block ID
    #[arg(short, long)]
    target: String,

    /// Edge description
    #[arg(long)]
    description: Option<String>,

    /// Edge confidence score
    #[arg(long)]
    confidence: Option<f64>,
}

fn add(args: AddEdgeArgs, format: OutputFormat) -> Result<()> {
    let mut doc = read_document(args.input)?;

    let source_id =
        BlockId::from_str(&args.source).map_err(|_| anyhow!("Invalid source block ID: {}", args.source))?;
    let target_id =
        BlockId::from_str(&args.target).map_err(|_| anyhow!("Invalid target block ID: {}", args.target))?;
    let et = EdgeType::from_str(&args.edge_type).unwrap_or(EdgeType::References);

    // Build metadata if provided
    let metadata: Option<serde_json::Value> = if args.description.is_some() || args.confidence.is_some() {
        let mut meta = serde_json::Map::new();
        if let Some(desc) = args.description {
            meta.insert("description".to_string(), serde_json::Value::String(desc));
        }
        if let Some(conf) = args.confidence {
            meta.insert("confidence".to_string(), serde_json::json!(conf));
        }
        Some(serde_json::Value::Object(meta))
    } else {
        None
    };

    let engine = Engine::new();
    let op = Operation::Link {
        source: source_id,
        edge_type: et,
        target: target_id,
        metadata,
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
                    "Created edge: {} --{}-> {}",
                    args.source, args.edge_type, args.target
                ));
            } else {
                print_error(&format!(
                    "Failed to create edge: {}",
                    result.error.unwrap_or_default()
                ));
            }
        }
    }

    write_document(&doc, args.output)?;
    Ok(())
}

fn remove(
    input: Option<String>,
    output: Option<String>,
    source: String,
    edge_type: String,
    target: String,
    format: OutputFormat,
) -> Result<()> {
    let mut doc = read_document(input)?;

    let source_id =
        BlockId::from_str(&source).map_err(|_| anyhow!("Invalid source block ID: {}", source))?;
    let target_id =
        BlockId::from_str(&target).map_err(|_| anyhow!("Invalid target block ID: {}", target))?;
    let et = EdgeType::from_str(&edge_type).unwrap_or(EdgeType::References);

    let engine = Engine::new();
    let op = Operation::Unlink {
        source: source_id,
        edge_type: et,
        target: target_id,
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
                    "Removed edge: {} --{}-> {}",
                    source, edge_type, target
                ));
            } else {
                print_error(&format!(
                    "Failed to remove edge: {}",
                    result.error.unwrap_or_default()
                ));
            }
        }
    }

    write_document(&doc, output)?;
    Ok(())
}

fn list(
    input: Option<String>,
    id: String,
    outgoing_only: bool,
    incoming_only: bool,
    format: OutputFormat,
) -> Result<()> {
    let doc = read_document(input)?;

    let block_id = BlockId::from_str(&id).map_err(|_| anyhow!("Invalid block ID: {}", id))?;

    let mut edges: Vec<(BlockId, Edge)> = Vec::new();

    // Get outgoing edges (from the block itself)
    if !incoming_only {
        if let Some(block) = doc.get_block(&block_id) {
            for edge in &block.edges {
                edges.push((block_id, edge.clone()));
            }
        }
    }

    // Get incoming edges (from edge index)
    if !outgoing_only {
        let incoming = doc.edge_index.incoming_to(&block_id);
        for (edge_type, source_id) in incoming {
            // Look up the source block to get the actual Edge object
            if let Some(source_block) = doc.get_block(source_id) {
                for edge in &source_block.edges {
                    if edge.target == block_id && &edge.edge_type == edge_type {
                        // Avoid duplicates if showing both
                        if incoming_only
                            || !edges
                                .iter()
                                .any(|(s, e)| s == source_id && e.target == edge.target)
                        {
                            edges.push((*source_id, edge.clone()));
                        }
                    }
                }
            }
        }
    }

    match format {
        OutputFormat::Json => {
            let summaries: Vec<EdgeSummary> = edges
                .iter()
                .map(|(src, edge)| EdgeSummary::new(src, edge))
                .collect();
            println!("{}", serde_json::to_string_pretty(&summaries)?);
        }
        OutputFormat::Text => {
            if edges.is_empty() {
                println!("No edges found for block {}", id);
            } else {
                print_edge_table(&edges);
            }
        }
    }

    Ok(())
}
