//! LLM integration commands

use anyhow::{anyhow, Result};
use colored::Colorize;
use serde::Serialize;
use std::str::FromStr;
use ucm_core::BlockId;
use ucp_llm::{IdMapper, PromptBuilder, UclCapability};

use crate::cli::{LlmCommands, OutputFormat};
use crate::output::{content_preview, print_success, read_document, read_file, write_output};

pub fn handle(cmd: LlmCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        LlmCommands::IdMap { input, output } => id_map(input, output, format),
        LlmCommands::ShortenUcl { ucl, mapping } => shorten_ucl(ucl, mapping, format),
        LlmCommands::ExpandUcl { ucl, mapping } => expand_ucl(ucl, mapping, format),
        LlmCommands::Prompt { capabilities } => prompt(capabilities, format),
        LlmCommands::Context {
            input,
            max_tokens,
            blocks,
        } => context(input, max_tokens, blocks, format),
    }
}

fn id_map(input: Option<String>, output: Option<String>, format: OutputFormat) -> Result<()> {
    let doc = read_document(input)?;

    let mapper = IdMapper::from_document(&doc);

    // Build a mapping table
    #[derive(Serialize)]
    struct IdMapping {
        block_id: String,
        short_id: u32,
    }

    let mappings: Vec<IdMapping> = doc
        .blocks
        .keys()
        .filter_map(|id| {
            mapper.to_short_id(id).map(|short| IdMapping {
                block_id: id.to_string(),
                short_id: short,
            })
        })
        .collect();

    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&mappings)?;
            write_output(&json, output)?;
        }
        OutputFormat::Text => {
            println!("{}", "ID Mapping:".cyan().bold());
            println!("{}", "─".repeat(50));
            for m in &mappings {
                println!("  {} → {}", m.short_id.to_string().green(), m.block_id);
            }
            println!("\nTotal: {} mappings", mappings.len());

            if let Some(out) = output {
                let json = serde_json::to_string_pretty(&mappings)?;
                std::fs::write(&out, json)?;
                print_success(&format!("Mapping written to {}", out));
            }
        }
    }

    Ok(())
}

fn shorten_ucl(ucl: String, mapping: String, format: OutputFormat) -> Result<()> {
    let mapping_json = read_file(&mapping)?;

    #[derive(serde::Deserialize)]
    struct IdMapping {
        block_id: String,
        #[serde(rename = "short_id")]
        _short_id: u32,
    }

    let mappings: Vec<IdMapping> = serde_json::from_str(&mapping_json)?;

    // Build mapper from loaded mappings
    let mut mapper = IdMapper::new();
    for m in &mappings {
        let block_id =
            BlockId::from_str(&m.block_id).map_err(|_| anyhow!("Invalid block ID in mapping"))?;
        mapper.register(&block_id);
    }

    let shortened = mapper.shorten_ucl(&ucl);

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct ShortenResult {
                original: String,
                shortened: String,
                savings: usize,
            }
            let result = ShortenResult {
                original: ucl.clone(),
                shortened: shortened.clone(),
                savings: ucl.len().saturating_sub(shortened.len()),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            println!("{}", shortened);
        }
    }

    Ok(())
}

fn expand_ucl(ucl: String, mapping: String, format: OutputFormat) -> Result<()> {
    let mapping_json = read_file(&mapping)?;

    #[derive(serde::Deserialize)]
    struct IdMapping {
        block_id: String,
        #[serde(rename = "short_id")]
        _short_id: u32,
    }

    let mappings: Vec<IdMapping> = serde_json::from_str(&mapping_json)?;

    // Build mapper from loaded mappings
    let mut mapper = IdMapper::new();
    for m in &mappings {
        let block_id =
            BlockId::from_str(&m.block_id).map_err(|_| anyhow!("Invalid block ID in mapping"))?;
        mapper.register(&block_id);
    }

    let expanded = mapper.expand_ucl(&ucl);

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct ExpandResult {
                original: String,
                expanded: String,
            }
            let result = ExpandResult {
                original: ucl,
                expanded: expanded.clone(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            println!("{}", expanded);
        }
    }

    Ok(())
}

fn prompt(capabilities: String, format: OutputFormat) -> Result<()> {
    let caps: Vec<UclCapability> = if capabilities.to_lowercase() == "all" {
        UclCapability::all()
    } else {
        capabilities
            .split(',')
            .filter_map(|s| match s.trim().to_lowercase().as_str() {
                "edit" => Some(UclCapability::Edit),
                "append" => Some(UclCapability::Append),
                "move" => Some(UclCapability::Move),
                "delete" => Some(UclCapability::Delete),
                "link" => Some(UclCapability::Link),
                "snapshot" => Some(UclCapability::Snapshot),
                "transaction" => Some(UclCapability::Transaction),
                _ => None,
            })
            .collect()
    };

    let builder = PromptBuilder::new().with_capabilities(caps.clone());
    let prompt_text = builder.build_system_prompt();

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct PromptResult {
                capabilities: Vec<String>,
                prompt: String,
            }
            let result = PromptResult {
                capabilities: caps.iter().map(|c| format!("{:?}", c)).collect(),
                prompt: prompt_text,
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            println!("{}", "UCL Prompt Documentation".cyan().bold());
            println!("{}", "═".repeat(60));
            println!("{}", prompt_text);
        }
    }

    Ok(())
}

fn context(
    input: Option<String>,
    max_tokens: usize,
    blocks: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let doc = read_document(input)?;

    // Collect blocks for context
    let block_ids: Vec<BlockId> = if let Some(ids) = blocks {
        ids.split(',')
            .filter_map(|s| BlockId::from_str(s.trim()).ok())
            .collect()
    } else {
        // Include all blocks by default
        doc.blocks.keys().cloned().collect()
    };

    // Build context respecting token limit
    let mut total_tokens = 0u32;
    let mut context_blocks = Vec::new();

    for id in &block_ids {
        if let Some(block) = doc.get_block(id) {
            let block_tokens = block
                .metadata
                .token_estimate
                .as_ref()
                .map(|t| t.generic)
                .unwrap_or(50); // Estimate if not available

            if total_tokens + block_tokens <= max_tokens as u32 {
                context_blocks.push(block);
                total_tokens += block_tokens;
            } else {
                break;
            }
        }
    }

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct ContextResult {
                max_tokens: usize,
                used_tokens: u32,
                blocks: Vec<ContextBlock>,
            }
            #[derive(Serialize)]
            struct ContextBlock {
                id: String,
                tokens: u32,
                content: String,
            }
            let result = ContextResult {
                max_tokens,
                used_tokens: total_tokens,
                blocks: context_blocks
                    .iter()
                    .map(|b| ContextBlock {
                        id: b.id.to_string(),
                        tokens: b
                            .metadata
                            .token_estimate
                            .as_ref()
                            .map(|t| t.generic)
                            .unwrap_or(50),
                        content: content_preview(&b.content, 500),
                    })
                    .collect(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            println!("{}", "LLM Context Window".cyan().bold());
            println!("{}", "─".repeat(60));
            println!(
                "Token budget: {} / {}",
                total_tokens.to_string().green(),
                max_tokens
            );
            println!("Blocks: {}", context_blocks.len());
            println!("{}", "─".repeat(60));

            for block in &context_blocks {
                let tokens = block
                    .metadata
                    .token_estimate
                    .as_ref()
                    .map(|t| t.generic)
                    .unwrap_or(50);
                println!("[{}] ({} tokens)", block.id.to_string().yellow(), tokens);
                let preview = content_preview(&block.content, 200);
                println!("{}\n", preview.dimmed());
            }
        }
    }

    Ok(())
}
