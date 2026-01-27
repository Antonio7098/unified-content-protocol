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
        } => add(
            input,
            output,
            source,
            edge_type,
            target,
            description,
            confidence,
            format,
        ),
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

fn add(
    input: Option<String>,
    output: Option<String>,
    source: String,
    edge_type: String,
    target: String,
    description: Option<String>,
    confidence: Option<f64>,
    format: OutputFormat,
) -> Result<()> {
    let mut doc = read_document(input)?;

    let source_id =
        BlockId::from_str(&source).map_err(|_| anyhow!("Invalid source block ID: {}", source))?;
    let target_id =
        BlockId::from_str(&target).map_err(|_| anyhow!("Invalid target block ID: {}", target))?;
    let et = EdgeType::from_str(&edge_type).unwrap_or(EdgeType::References);

    // Build metadata if provided
    let metadata: Option<serde_json::Value> = if description.is_some() || confidence.is_some() {
        let mut meta = serde_json::Map::new();
        if let Some(desc) = description {
            meta.insert("description".to_string(), serde_json::Value::String(desc));
        }
        if let Some(conf) = confidence {
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
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            if result.success {
                print_success(&format!(
                    "Created edge: {} --{}-> {}",
                    source, edge_type, target
                ));
            } else {
                print_error(&format!(
                    "Failed to create edge: {}",
                    result.error.unwrap_or_default()
                ));
            }
        }
    }

    write_document(&doc, output)?;
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
            println!("{}", serde_json::to_string_pretty(&result)?);
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

    let block_id =
        BlockId::from_str(&id).map_err(|_| anyhow!("Invalid block ID: {}", id))?;

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
        for (source_id, edge) in incoming {
            // Avoid duplicates if showing both
            if incoming_only || !edges.iter().any(|(s, e)| s == source_id && e.target == edge.target) {
                edges.push((*source_id, edge.clone()));
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
