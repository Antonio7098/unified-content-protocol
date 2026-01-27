//! Prune command

use anyhow::Result;
use serde::Serialize;
use ucm_engine::{Engine, Operation, PruneCondition};

use crate::cli::OutputFormat;
use crate::output::{print_error, print_success, read_document, write_document};

/// Prune orphaned or tagged blocks
pub fn prune(
    input: Option<String>,
    output: Option<String>,
    tag: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let mut doc = read_document(input)?;

    let before_count = doc.block_count();

    let engine = Engine::new();
    let condition = if let Some(t) = tag {
        Some(PruneCondition::WithTag(t))
    } else {
        Some(PruneCondition::Unreachable)
    };

    let op = Operation::Prune { condition };
    let result = engine.execute(&mut doc, op)?;

    let after_count = doc.block_count();
    let removed = before_count - after_count;

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct PruneResult {
                success: bool,
                blocks_before: usize,
                blocks_after: usize,
                blocks_removed: usize,
            }
            let result = PruneResult {
                success: result.success,
                blocks_before: before_count,
                blocks_after: after_count,
                blocks_removed: removed,
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            if result.success {
                print_success(&format!("Pruned {} blocks", removed));
            } else {
                print_error(&format!(
                    "Prune failed: {}",
                    result.error.unwrap_or_default()
                ));
            }
        }
    }

    write_document(&doc, output)?;
    Ok(())
}
