//! Snapshot commands

use anyhow::{anyhow, Result};
use colored::Colorize;
use serde::Serialize;

use crate::cli::{OutputFormat, SnapshotCommands};
use crate::output::{print_error, print_success};
use crate::state::{read_stateful_document, write_stateful_document, SnapshotInfo};

pub fn handle(cmd: SnapshotCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        SnapshotCommands::Create {
            input,
            output,
            name,
            description,
        } => create(input, output, name, description, format),
        SnapshotCommands::Restore {
            input,
            output,
            name,
        } => restore(input, output, name, format),
        SnapshotCommands::List { input } => list(input, format),
        SnapshotCommands::Delete {
            input,
            output,
            name,
        } => delete(input, output, name, format),
        SnapshotCommands::Diff { input, from, to } => diff(input, from, to, format),
    }
}

fn create(
    input: Option<String>,
    output: Option<String>,
    name: String,
    description: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input)?;

    // Check if snapshot already exists
    if stateful
        .state()
        .snapshots
        .iter()
        .any(|s| s.name == name)
    {
        return Err(anyhow!("Snapshot '{}' already exists", name));
    }

    let snapshot = SnapshotInfo::create(name.clone(), description, &stateful.document)?;
    stateful.state_mut().snapshots.push(snapshot);

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct SnapshotResult {
                success: bool,
                name: String,
                block_count: usize,
            }
            let result = SnapshotResult {
                success: true,
                name,
                block_count: stateful.document.block_count(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            print_success(&format!(
                "Snapshot '{}' created ({} blocks)",
                name,
                stateful.document.block_count()
            ));
        }
    }

    write_stateful_document(&stateful, output)?;
    Ok(())
}

fn restore(
    input: Option<String>,
    output: Option<String>,
    name: String,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input)?;

    let snapshot = stateful
        .state()
        .snapshots
        .iter()
        .find(|s| s.name == name)
        .ok_or_else(|| anyhow!("Snapshot '{}' not found", name))?
        .clone();

    stateful.document = snapshot.restore()?;

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct SnapshotResult {
                success: bool,
                name: String,
                block_count: usize,
            }
            let result = SnapshotResult {
                success: true,
                name,
                block_count: stateful.document.block_count(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            print_success(&format!(
                "Restored from snapshot '{}' ({} blocks)",
                name,
                stateful.document.block_count()
            ));
        }
    }

    write_stateful_document(&stateful, output)?;
    Ok(())
}

fn list(input: Option<String>, format: OutputFormat) -> Result<()> {
    let stateful = read_stateful_document(input)?;

    let snapshots = &stateful.state().snapshots;

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct SnapshotListItem {
                name: String,
                description: Option<String>,
                created_at: String,
                block_count: usize,
            }
            let list: Vec<SnapshotListItem> = snapshots
                .iter()
                .map(|s| SnapshotListItem {
                    name: s.name.clone(),
                    description: s.description.clone(),
                    created_at: s.created_at.clone(),
                    block_count: s.block_count,
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&list)?);
        }
        OutputFormat::Text => {
            if snapshots.is_empty() {
                println!("No snapshots found");
            } else {
                println!("{}", "Snapshots:".cyan().bold());
                println!("{}", "─".repeat(60));
                for snap in snapshots {
                    println!(
                        "  {} ({} blocks) - {}",
                        snap.name.green(),
                        snap.block_count,
                        snap.created_at.dimmed()
                    );
                    if let Some(desc) = &snap.description {
                        println!("    {}", desc.dimmed());
                    }
                }
            }
        }
    }

    Ok(())
}

fn delete(
    input: Option<String>,
    output: Option<String>,
    name: String,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input)?;

    let initial_len = stateful.state().snapshots.len();
    stateful.state_mut().snapshots.retain(|s| s.name != name);

    if stateful.state().snapshots.len() == initial_len {
        return Err(anyhow!("Snapshot '{}' not found", name));
    }

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct SnapshotResult {
                success: bool,
                name: String,
            }
            let result = SnapshotResult {
                success: true,
                name,
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            print_success(&format!("Snapshot '{}' deleted", name));
        }
    }

    write_stateful_document(&stateful, output)?;
    Ok(())
}

fn diff(input: Option<String>, from: String, to: String, format: OutputFormat) -> Result<()> {
    let stateful = read_stateful_document(input)?;

    let snap_from = stateful
        .state()
        .snapshots
        .iter()
        .find(|s| s.name == from)
        .ok_or_else(|| anyhow!("Snapshot '{}' not found", from))?;

    let snap_to = stateful
        .state()
        .snapshots
        .iter()
        .find(|s| s.name == to)
        .ok_or_else(|| anyhow!("Snapshot '{}' not found", to))?;

    let doc_from = snap_from.restore()?;
    let doc_to = snap_to.restore()?;

    // Collect block IDs
    let ids_from: std::collections::HashSet<_> = doc_from.blocks.keys().cloned().collect();
    let ids_to: std::collections::HashSet<_> = doc_to.blocks.keys().cloned().collect();

    let added: Vec<_> = ids_to.difference(&ids_from).collect();
    let removed: Vec<_> = ids_from.difference(&ids_to).collect();
    let common: Vec<_> = ids_from.intersection(&ids_to).collect();

    // Check for modified blocks (same ID but different content)
    let mut modified = Vec::new();
    for id in &common {
        let block_from = doc_from.get_block(id);
        let block_to = doc_to.get_block(id);
        if let (Some(bf), Some(bt)) = (block_from, block_to) {
            // Simple comparison - could be more sophisticated
            if serde_json::to_string(&bf.content)? != serde_json::to_string(&bt.content)? {
                modified.push(*id);
            }
        }
    }

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct DiffResult {
                from: String,
                to: String,
                added: Vec<String>,
                removed: Vec<String>,
                modified: Vec<String>,
            }
            let result = DiffResult {
                from,
                to,
                added: added.iter().map(|id| id.to_string()).collect(),
                removed: removed.iter().map(|id| id.to_string()).collect(),
                modified: modified.iter().map(|id| id.to_string()).collect(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            println!("{}", format!("Diff: {} → {}", from, to).cyan().bold());
            println!("{}", "─".repeat(40));

            if !added.is_empty() {
                println!("{} ({}):", "Added".green().bold(), added.len());
                for id in &added {
                    println!("  + {}", id);
                }
            }

            if !removed.is_empty() {
                println!("{} ({}):", "Removed".red().bold(), removed.len());
                for id in &removed {
                    println!("  - {}", id);
                }
            }

            if !modified.is_empty() {
                println!("{} ({}):", "Modified".yellow().bold(), modified.len());
                for id in &modified {
                    println!("  ~ {}", id);
                }
            }

            if added.is_empty() && removed.is_empty() && modified.is_empty() {
                println!("No differences found");
            }
        }
    }

    Ok(())
}
