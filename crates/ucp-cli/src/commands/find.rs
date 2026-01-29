//! Find and search commands

use anyhow::Result;
use regex::Regex;
use serde::Serialize;
use ucm_core::Block;

use crate::cli::OutputFormat;
use crate::output::{content_preview, print_block_table, read_document, BlockSummary};

/// Find blocks matching criteria
pub fn find(
    input: Option<String>,
    role: Option<String>,
    tag: Option<String>,
    pattern: Option<String>,
    limit: usize,
    format: OutputFormat,
) -> Result<()> {
    let doc = read_document(input)?;

    let pattern_regex = pattern.as_ref().map(|p| Regex::new(p)).transpose()?;

    let matches: Vec<&Block> = doc
        .blocks
        .values()
        .filter(|block| {
            // Filter by role
            if let Some(ref r) = role {
                if let Some(ref block_role) = block.metadata.semantic_role {
                    let role_str = block_role.to_string();
                    if !role_str.to_lowercase().contains(&r.to_lowercase()) {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            // Filter by tag
            if let Some(ref t) = tag {
                if !block.metadata.tags.iter().any(|bt| bt.contains(t)) {
                    return false;
                }
            }

            // Filter by content pattern
            if let Some(ref regex) = pattern_regex {
                let content_str = content_preview(&block.content, 10000);
                if !regex.is_match(&content_str) {
                    return false;
                }
            }

            true
        })
        .take(limit)
        .collect();

    match format {
        OutputFormat::Json => {
            let summaries: Vec<BlockSummary> = matches
                .iter()
                .map(|b| BlockSummary::from_block(b))
                .collect();
            println!("{}", serde_json::to_string_pretty(&summaries)?);
        }
        OutputFormat::Text => {
            if matches.is_empty() {
                println!("No matching blocks found");
            } else {
                println!("Found {} matching blocks:", matches.len());
                print_block_table(&matches);
            }
        }
    }

    Ok(())
}

/// Find orphaned blocks
pub fn orphans(input: Option<String>, format: OutputFormat) -> Result<()> {
    let doc = read_document(input)?;

    let orphan_ids = doc.find_orphans();

    let orphans: Vec<&Block> = orphan_ids
        .iter()
        .filter_map(|id| doc.get_block(id))
        .collect();

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct OrphanResult {
                count: usize,
                orphans: Vec<BlockSummary>,
            }
            let result = OrphanResult {
                count: orphans.len(),
                orphans: orphans
                    .iter()
                    .map(|b| BlockSummary::from_block(b))
                    .collect(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            if orphans.is_empty() {
                println!("No orphaned blocks found");
            } else {
                println!("Found {} orphaned blocks:", orphans.len());
                print_block_table(&orphans);
            }
        }
    }

    Ok(())
}
