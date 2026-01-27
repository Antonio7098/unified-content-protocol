//! Navigation commands

use anyhow::{anyhow, Result};
use serde::Serialize;
use std::str::FromStr;
use ucm_core::BlockId;

use crate::cli::{NavCommands, OutputFormat};
use crate::output::{print_block, print_block_table, read_document, BlockSummary};

pub fn handle(cmd: NavCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        NavCommands::Children { input, id } => children(input, id, format),
        NavCommands::Parent { input, id } => parent(input, id, format),
        NavCommands::Siblings { input, id } => siblings(input, id, format),
        NavCommands::Descendants { input, id, depth } => descendants(input, id, depth, format),
    }
}

fn children(input: Option<String>, id: Option<String>, format: OutputFormat) -> Result<()> {
    let doc = read_document(input)?;

    let block_id = if let Some(id_str) = id {
        BlockId::from_str(&id_str).map_err(|_| anyhow!("Invalid block ID: {}", id_str))?
    } else {
        doc.root
    };

    let child_ids = doc.children(&block_id);

    let children: Vec<_> = child_ids
        .iter()
        .filter_map(|id| doc.get_block(id))
        .collect();

    match format {
        OutputFormat::Json => {
            let summaries: Vec<BlockSummary> =
                children.iter().map(|b| BlockSummary::from_block(b)).collect();
            println!("{}", serde_json::to_string_pretty(&summaries)?);
        }
        OutputFormat::Text => {
            if children.is_empty() {
                println!("No children found");
            } else {
                println!("Children of {}:", block_id);
                print_block_table(&children);
            }
        }
    }

    Ok(())
}

fn parent(input: Option<String>, id: String, format: OutputFormat) -> Result<()> {
    let doc = read_document(input)?;

    let block_id =
        BlockId::from_str(&id).map_err(|_| anyhow!("Invalid block ID: {}", id))?;

    let parent_id = doc.parent(&block_id);

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct ParentResult {
                block_id: String,
                parent_id: Option<String>,
            }
            let result = ParentResult {
                block_id: id,
                parent_id: parent_id.map(|p| p.to_string()),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            if let Some(pid) = parent_id {
                println!("Parent of {}: {}", id, pid);
                if let Some(parent_block) = doc.get_block(pid) {
                    print_block(parent_block, false);
                }
            } else {
                println!("Block {} has no parent (is root)", id);
            }
        }
    }

    Ok(())
}

fn siblings(input: Option<String>, id: String, format: OutputFormat) -> Result<()> {
    let doc = read_document(input)?;

    let block_id =
        BlockId::from_str(&id).map_err(|_| anyhow!("Invalid block ID: {}", id))?;

    // Get parent, then get all children of parent
    let sibling_ids: Vec<BlockId> = if let Some(parent_id) = doc.parent(&block_id) {
        doc.children(parent_id)
            .iter()
            .filter(|bid| *bid != &block_id)
            .cloned()
            .collect()
    } else {
        Vec::new()
    };

    let siblings: Vec<_> = sibling_ids
        .iter()
        .filter_map(|id| doc.get_block(id))
        .collect();

    match format {
        OutputFormat::Json => {
            let summaries: Vec<BlockSummary> =
                siblings.iter().map(|b| BlockSummary::from_block(b)).collect();
            println!("{}", serde_json::to_string_pretty(&summaries)?);
        }
        OutputFormat::Text => {
            if siblings.is_empty() {
                println!("No siblings found");
            } else {
                println!("Siblings of {}:", id);
                print_block_table(&siblings);
            }
        }
    }

    Ok(())
}

fn descendants(
    input: Option<String>,
    id: Option<String>,
    max_depth: Option<usize>,
    format: OutputFormat,
) -> Result<()> {
    let doc = read_document(input)?;

    let block_id = if let Some(id_str) = id {
        BlockId::from_str(&id_str).map_err(|_| anyhow!("Invalid block ID: {}", id_str))?
    } else {
        doc.root
    };

    // Collect descendants with depth tracking
    fn collect_descendants(
        doc: &ucm_core::Document,
        block_id: &BlockId,
        current_depth: usize,
        max_depth: Option<usize>,
        results: &mut Vec<(BlockId, usize)>,
    ) {
        if let Some(max) = max_depth {
            if current_depth > max {
                return;
            }
        }

        for child_id in doc.children(block_id) {
            results.push((child_id.clone(), current_depth));
            collect_descendants(doc, child_id, current_depth + 1, max_depth, results);
        }
    }

    let mut descendant_ids = Vec::new();
    collect_descendants(&doc, &block_id, 1, max_depth, &mut descendant_ids);

    let descendants: Vec<_> = descendant_ids
        .iter()
        .filter_map(|(id, _)| doc.get_block(id))
        .collect();

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct DescendantInfo {
                id: String,
                depth: usize,
                content_type: String,
                role: Option<String>,
            }
            let info: Vec<DescendantInfo> = descendant_ids
                .iter()
                .filter_map(|(id, depth)| {
                    doc.get_block(id).map(|b| DescendantInfo {
                        id: id.to_string(),
                        depth: *depth,
                        content_type: b.content.type_tag().to_string(),
                        role: b.metadata.semantic_role.as_ref().map(|r| r.to_string()),
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&info)?);
        }
        OutputFormat::Text => {
            if descendants.is_empty() {
                println!("No descendants found");
            } else {
                println!(
                    "Descendants of {} ({} total):",
                    block_id,
                    descendants.len()
                );
                print_block_table(&descendants);
            }
        }
    }

    Ok(())
}
