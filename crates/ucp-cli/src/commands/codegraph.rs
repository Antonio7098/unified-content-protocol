use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use serde::Serialize;
use std::path::PathBuf;
use std::process::Command;
use ucp_api::{
    build_code_graph, canonical_fingerprint, codegraph_prompt_projection,
    validate_code_graph_profile, CodeGraphBuildInput, CodeGraphBuildStatus,
    CodeGraphExtractorConfig, CodeGraphSeverity,
};

use crate::cli::{CodegraphCommands, OutputFormat};
use crate::output::{
    print_error, print_success, print_warning, read_document, write_output, DocumentJson,
};

pub fn handle(cmd: CodegraphCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        CodegraphCommands::Build {
            repo,
            commit,
            output,
            extensions,
            include_hidden,
            no_export_edges,
            fail_on_parse_error,
            max_file_bytes,
            allow_partial,
        } => build(
            repo,
            commit,
            output,
            extensions,
            include_hidden,
            no_export_edges,
            fail_on_parse_error,
            max_file_bytes,
            allow_partial,
            format,
        ),
        CodegraphCommands::Inspect { input } => inspect(input, format),
        CodegraphCommands::Prompt { input, output } => prompt(input, output, format),
    }
}

#[allow(clippy::too_many_arguments)]
fn build(
    repo: String,
    commit: Option<String>,
    output: Option<String>,
    extensions: Option<String>,
    include_hidden: bool,
    no_export_edges: bool,
    fail_on_parse_error: bool,
    max_file_bytes: usize,
    allow_partial: bool,
    format: OutputFormat,
) -> Result<()> {
    let repository_path = PathBuf::from(&repo);
    let commit_hash = commit
        .or_else(|| detect_commit_hash(&repository_path).ok())
        .unwrap_or_else(|| "unknown".to_string());

    let mut config = CodeGraphExtractorConfig::default();
    if let Some(exts) = extensions {
        config.include_extensions = exts
            .split(',')
            .map(|e| e.trim().trim_start_matches('.').to_ascii_lowercase())
            .filter(|e| !e.is_empty())
            .collect();
    }
    config.include_hidden = include_hidden;
    config.emit_export_edges = !no_export_edges;
    config.continue_on_parse_error = !fail_on_parse_error;
    config.max_file_bytes = max_file_bytes;

    let result = build_code_graph(&CodeGraphBuildInput {
        repository_path,
        commit_hash,
        config,
    })
    .context("failed to build code graph")?;

    let doc_json = DocumentJson::from_document(&result.document);

    if let Some(path) = &output {
        let serialized = serde_json::to_string_pretty(&doc_json)?;
        std::fs::write(path, serialized)?;
    }

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct JsonBuildOutput {
                status: CodeGraphBuildStatus,
                profile_version: String,
                canonical_fingerprint: String,
                stats: ucp_api::CodeGraphStats,
                diagnostics: Vec<ucp_api::CodeGraphDiagnostic>,
                document: DocumentJson,
            }

            let payload = JsonBuildOutput {
                status: result.status,
                profile_version: result.profile_version,
                canonical_fingerprint: result.canonical_fingerprint,
                stats: result.stats,
                diagnostics: result.diagnostics.clone(),
                document: doc_json,
            };

            println!("{}", serde_json::to_string_pretty(&payload)?);
        }
        OutputFormat::Text => {
            println!("{}", "CodeGraph Build Summary".cyan().bold());
            println!("{}", "─".repeat(60));
            println!("status: {:?}", result.status);
            println!("profile_version: {}", result.profile_version);
            println!("canonical_fingerprint: {}", result.canonical_fingerprint);
            println!(
                "nodes: total={} repo={} dir={} file={} symbol={}",
                result.stats.total_nodes,
                result.stats.repository_nodes,
                result.stats.directory_nodes,
                result.stats.file_nodes,
                result.stats.symbol_nodes
            );
            println!(
                "edges: total={} references={} exports={}",
                result.stats.total_edges, result.stats.reference_edges, result.stats.export_edges
            );

            if !result.stats.languages.is_empty() {
                let mut langs: Vec<_> = result.stats.languages.iter().collect();
                langs.sort_by(|a, b| a.0.cmp(b.0));
                let joined = langs
                    .into_iter()
                    .map(|(lang, count)| format!("{}:{}", lang, count))
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("languages: {joined}");
            }

            if !result.diagnostics.is_empty() {
                println!("{}", "diagnostics:".yellow().bold());
                for diag in &result.diagnostics {
                    let sev = match diag.severity {
                        CodeGraphSeverity::Error => "ERROR".red().bold(),
                        CodeGraphSeverity::Warning => "WARN".yellow().bold(),
                        CodeGraphSeverity::Info => "INFO".blue().bold(),
                    };
                    match (&diag.path, &diag.logical_key) {
                        (Some(path), _) => println!("  {} {} [{}]", sev, diag.message, path),
                        (None, Some(logical_key)) => {
                            println!("  {} {} [{}]", sev, diag.message, logical_key)
                        }
                        _ => println!("  {} {}", sev, diag.message),
                    }
                }
            }

            if let Some(path) = output {
                print_success(&format!("CodeGraph document written to {}", path));
            } else {
                println!("\n{}", serde_json::to_string_pretty(&doc_json)?);
            }
        }
    }

    if !allow_partial {
        let has_errors = result
            .diagnostics
            .iter()
            .any(|d| d.severity == CodeGraphSeverity::Error);
        if result.status == CodeGraphBuildStatus::FailedValidation {
            return Err(anyhow!(
                "code graph build failed profile validation; rerun with --allow-partial to keep output"
            ));
        }
        if has_errors {
            return Err(anyhow!(
                "code graph build produced errors; rerun with --allow-partial to keep output"
            ));
        }
    }

    Ok(())
}

fn inspect(input: Option<String>, format: OutputFormat) -> Result<()> {
    let doc = read_document(input)?;
    let validation = validate_code_graph_profile(&doc);
    let fingerprint = canonical_fingerprint(&doc)?;

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct InspectResult {
                valid: bool,
                canonical_fingerprint: String,
                diagnostics: Vec<ucp_api::CodeGraphDiagnostic>,
            }

            let result = InspectResult {
                valid: validation.valid,
                canonical_fingerprint: fingerprint,
                diagnostics: validation.diagnostics,
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            if validation.valid {
                print_success("CodeGraph profile validation passed");
            } else {
                print_error("CodeGraph profile validation failed");
            }
            println!("canonical_fingerprint: {}", fingerprint);

            if !validation.diagnostics.is_empty() {
                println!("{}", "diagnostics:".yellow().bold());
                for diag in validation.diagnostics {
                    let level = match diag.severity {
                        CodeGraphSeverity::Error => "ERROR".red().bold(),
                        CodeGraphSeverity::Warning => "WARN".yellow().bold(),
                        CodeGraphSeverity::Info => "INFO".blue().bold(),
                    };
                    println!("  {} {} ({})", level, diag.message, diag.code);
                }
            }
        }
    }

    Ok(())
}

fn prompt(input: Option<String>, output: Option<String>, format: OutputFormat) -> Result<()> {
    let doc = read_document(input)?;
    let projection = codegraph_prompt_projection(&doc);

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct PromptResult {
                projection: String,
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&PromptResult {
                    projection: projection.clone(),
                })?
            );
            if let Some(path) = output {
                write_output(&projection, Some(path))?;
            }
        }
        OutputFormat::Text => {
            write_output(&projection, output)?;
        }
    }

    Ok(())
}

fn detect_commit_hash(repo: &PathBuf) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .context("failed to run git rev-parse HEAD")?;

    if !output.status.success() {
        print_warning("could not detect commit hash; using 'unknown'");
        return Err(anyhow!(
            "git rev-parse failed with status {}",
            output.status
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
