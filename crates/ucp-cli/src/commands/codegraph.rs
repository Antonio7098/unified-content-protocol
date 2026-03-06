use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use serde::Serialize;
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use ucp_api::{
    build_code_graph, canonical_fingerprint, codegraph_prompt_projection,
    export_codegraph_context_with_config, is_codegraph_document, render_codegraph_context_prompt,
    resolve_codegraph_selector,
    validate_code_graph_profile, CodeGraphBuildInput, CodeGraphBuildStatus,
    CodeGraphContextExport, CodeGraphContextUpdate, CodeGraphDetailLevel, CodeGraphExportConfig,
    CodeGraphExtractorConfig, CodeGraphPrunePolicy, CodeGraphRenderConfig, CodeGraphSeverity,
};
use ucm_core::{BlockId, Document};

use crate::cli::{CodegraphCommands, CodegraphContextCommands, OutputFormat};
use crate::output::{
    print_error, print_success, print_warning, read_document, write_output, DocumentJson,
};
use crate::state::{
    read_stateful_document, write_stateful_document, AgentSessionState, StatefulDocument,
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
        CodegraphCommands::Context(cmd) => context(cmd, format),
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

fn context(cmd: CodegraphContextCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        CodegraphContextCommands::Init {
            input,
            name,
            max_selected,
            initial_depth,
        } => context_init(input, name, max_selected, initial_depth, format),
        CodegraphContextCommands::Show {
            input,
            session,
            max_tokens,
            compact,
            no_rendered,
            levels,
        } => context_show(input, session, max_tokens, compact, no_rendered, levels, format),
        CodegraphContextCommands::Export {
            input,
            session,
            max_tokens,
            compact,
            no_rendered,
            levels,
        } => context_export(input, session, max_tokens, compact, no_rendered, levels, format),
        CodegraphContextCommands::Add {
            input,
            session,
            selectors,
        } => context_add(input, session, selectors, format),
        CodegraphContextCommands::Focus {
            input,
            session,
            target,
        } => context_focus(input, session, target, format),
        CodegraphContextCommands::Expand {
            input,
            session,
            target,
            mode,
            relation,
            relations,
            depth,
        } => context_expand(input, session, target, mode, relation, relations, depth, format),
        CodegraphContextCommands::Hydrate {
            input,
            session,
            target,
            padding,
        } => context_hydrate(input, session, target, padding, format),
        CodegraphContextCommands::Collapse {
            input,
            session,
            target,
            descendants,
        } => context_collapse(input, session, target, descendants, format),
        CodegraphContextCommands::Pin {
            input,
            session,
            target,
        } => context_pin(input, session, target, true, format),
        CodegraphContextCommands::Unpin {
            input,
            session,
            target,
        } => context_pin(input, session, target, false, format),
        CodegraphContextCommands::Prune {
            input,
            session,
            max_selected,
        } => context_prune(input, session, max_selected, format),
    }
}

fn context_init(
    input: Option<String>,
    name: Option<String>,
    max_selected: usize,
    initial_depth: Option<usize>,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;
    ensure_codegraph_document(&stateful.document)?;

    let session_id = format!("cgctx_{}", uuid_short());
    let mut session = AgentSessionState::new(session_id.clone(), name.clone(), None);
    {
        let context = session.ensure_codegraph_context();
        context.set_prune_policy(CodeGraphPrunePolicy {
            max_selected: max_selected.max(1),
            ..CodeGraphPrunePolicy::default()
        });
        let update = context.seed_overview_with_depth(&stateful.document, initial_depth);
        session.current_block = update.focus.map(|id| id.to_string());
        session.sync_context_blocks_from_codegraph();
    }
    stateful
        .state_mut()
        .sessions
        .insert(session_id.clone(), session.clone());

    match format {
        OutputFormat::Json => {
            let rendered = render_codegraph_context_prompt(
                &stateful.document,
                session.codegraph_context.as_ref().expect("context seeded"),
                &CodeGraphRenderConfig::default(),
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "success": true,
                    "session_id": session_id,
                    "name": name,
                    "initial_depth": initial_depth,
                    "summary": session.codegraph_context.as_ref().map(|ctx| ctx.summary(&stateful.document)),
                    "rendered": rendered,
                }))?
            );
        }
        OutputFormat::Text => {
            print_success(&format!("Initialized codegraph context session: {}", session_id));
        }
    }

    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn context_show(
    input: Option<String>,
    session: String,
    max_tokens: usize,
    compact: bool,
    no_rendered: bool,
    levels: Option<usize>,
    format: OutputFormat,
) -> Result<()> {
    let stateful = read_stateful_document(input)?;
    ensure_codegraph_document(&stateful.document)?;
    let sess = get_session(&stateful, &session)?;
    let context = sess
        .codegraph_context
        .as_ref()
        .ok_or_else(|| anyhow!("Session has no codegraph context: {}", session))?;
    let config = CodeGraphRenderConfig::for_max_tokens(max_tokens);
    let export_config = make_export_config(compact, no_rendered, levels);
    let export = export_codegraph_context_with_config(&stateful.document, context, &config, &export_config);

    match format {
        OutputFormat::Json => {
            let mut value = serde_json::to_value(&export)?;
            if let Some(object) = value.as_object_mut() {
                object.insert("session".to_string(), serde_json::Value::String(session));
            }
            println!("{}", serde_json::to_string_pretty(&value)?);
        }
        OutputFormat::Text => println!("{}", render_context_show_text(&stateful.document, context, &config, &export)),
    }

    Ok(())
}

fn context_export(
    input: Option<String>,
    session: String,
    max_tokens: usize,
    compact: bool,
    no_rendered: bool,
    levels: Option<usize>,
    format: OutputFormat,
) -> Result<()> {
    let stateful = read_stateful_document(input)?;
    ensure_codegraph_document(&stateful.document)?;
    let sess = get_session(&stateful, &session)?;
    let context = sess
        .codegraph_context
        .as_ref()
        .ok_or_else(|| anyhow!("Session has no codegraph context: {}", session))?;
    let config = CodeGraphRenderConfig::for_max_tokens(max_tokens);
    let export_config = make_export_config(compact, no_rendered, levels);
    let export = export_codegraph_context_with_config(&stateful.document, context, &config, &export_config);

    match format {
        OutputFormat::Json => {
            let mut value = serde_json::to_value(&export)?;
            if let Some(object) = value.as_object_mut() {
                object.insert("session".to_string(), serde_json::Value::String(session));
            }
            println!("{}", serde_json::to_string_pretty(&value)?);
        }
        OutputFormat::Text => println!("{}", serde_json::to_string_pretty(&export)?),
    }

    Ok(())
}

fn make_export_config(compact: bool, no_rendered: bool, levels: Option<usize>) -> CodeGraphExportConfig {
    let mut export_config = if compact {
        CodeGraphExportConfig::compact()
    } else {
        CodeGraphExportConfig::default()
    };
    if no_rendered {
        export_config.include_rendered = false;
    }
    export_config.visible_levels = levels;
    export_config
}

fn render_context_show_text(
    document: &Document,
    context: &ucp_api::CodeGraphContextSession,
    config: &CodeGraphRenderConfig,
    export: &CodeGraphContextExport,
) -> String {
    if export.visible_levels.is_none() {
        return render_codegraph_context_prompt(document, context, config);
    }

    let mut lines = vec![format!(
        "visible nodes: {} of {} selected",
        export.visible_node_count, export.summary.selected
    )];
    if let Some(levels) = export.visible_levels {
        lines.push(format!("focus window: +{} levels", levels));
    }
    if !export.hidden_levels.is_empty() {
        for hidden in &export.hidden_levels {
            lines.push(format!("+ {} nodes at level {}", hidden.count, hidden.level));
        }
    }
    if export.hidden_unreachable_count > 0 {
        lines.push(format!(
            "+ {} selected nodes disconnected from focus",
            export.hidden_unreachable_count
        ));
    }
    if let Some(action) = &export.heuristics.recommended_next_action {
        lines.push(format!(
            "next: {} {}",
            action.action,
            action.relation.as_deref().unwrap_or("*")
        ));
    }
    lines.join("\n")
}

fn parse_relation_filters(
    relation: Option<String>,
    relations: Option<String>,
) -> Result<Option<HashSet<String>>> {
    if relation.is_some() && relations.is_some() {
        return Err(anyhow!("Use either --relation or --relations, not both"));
    }
    let raw = relation.or(relations);
    let Some(raw) = raw else {
        return Ok(None);
    };
    let filters: HashSet<String> = raw
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect();
    if filters.is_empty() {
        Ok(None)
    } else {
        Ok(Some(filters))
    }
}

fn context_add(
    input: Option<String>,
    session: String,
    selectors: String,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;
    ensure_codegraph_document(&stateful.document)?;
    let document = stateful.document.clone();
    let block_ids = resolve_selectors(&document, &selectors)?;

    let sess = get_session_mut(&mut stateful, &session)?;
    let mut update = CodeGraphContextUpdate::default();
    {
        let context = sess.ensure_codegraph_context();
        for block_id in block_ids {
            merge_updates(
                &mut update,
                context.select_block(&document, block_id, CodeGraphDetailLevel::SymbolCard),
            );
        }
    }
    sess.sync_context_blocks_from_codegraph();
    sess.current_block = update.focus.map(|id| id.to_string()).or_else(|| sess.current_block.clone());

    print_context_update(format, &session, &update, sess)?;
    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn context_focus(
    input: Option<String>,
    session: String,
    target: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;
    ensure_codegraph_document(&stateful.document)?;
    let document = stateful.document.clone();
    let target_id = target
        .as_deref()
        .map(|selector| resolve_selector(&document, selector))
        .transpose()?;

    let sess = get_session_mut(&mut stateful, &session)?;
    let update = sess.ensure_codegraph_context().set_focus(&document, target_id);
    sess.sync_context_blocks_from_codegraph();
    sess.current_block = update.focus.map(|id| id.to_string());

    print_context_update(format, &session, &update, sess)?;
    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn context_expand(
    input: Option<String>,
    session: String,
    target: String,
    mode: String,
    relation: Option<String>,
    relations: Option<String>,
    depth: usize,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;
    ensure_codegraph_document(&stateful.document)?;
    let document = stateful.document.clone();
    let block_id = resolve_selector(&document, &target)?;
    let relation_filters = parse_relation_filters(relation, relations)?;

    let sess = get_session_mut(&mut stateful, &session)?;
    let update = match mode.as_str() {
        "file" => sess
            .ensure_codegraph_context()
            .expand_file_with_depth(&document, block_id, depth),
        "dependencies" => sess
            .ensure_codegraph_context()
            .expand_dependencies_with_filters(&document, block_id, relation_filters.as_ref(), depth),
        "dependents" => sess
            .ensure_codegraph_context()
            .expand_dependents_with_filters(&document, block_id, relation_filters.as_ref(), depth),
        other => return Err(anyhow!("Unsupported expand mode: {}", other)),
    };
    sess.sync_context_blocks_from_codegraph();
    sess.current_block = update.focus.map(|id| id.to_string());

    print_context_update(format, &session, &update, sess)?;
    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn context_hydrate(
    input: Option<String>,
    session: String,
    target: String,
    padding: usize,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;
    ensure_codegraph_document(&stateful.document)?;
    let document = stateful.document.clone();
    let block_id = resolve_selector(&document, &target)?;

    let sess = get_session_mut(&mut stateful, &session)?;
    let update = sess
        .ensure_codegraph_context()
        .hydrate_source(&document, block_id, padding);
    sess.sync_context_blocks_from_codegraph();
    sess.current_block = update.focus.map(|id| id.to_string());

    print_context_update(format, &session, &update, sess)?;
    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn context_collapse(
    input: Option<String>,
    session: String,
    target: String,
    descendants: bool,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;
    ensure_codegraph_document(&stateful.document)?;
    let document = stateful.document.clone();
    let block_id = resolve_selector(&document, &target)?;

    let sess = get_session_mut(&mut stateful, &session)?;
    let update = sess
        .ensure_codegraph_context()
        .collapse(&document, block_id, descendants);
    sess.sync_context_blocks_from_codegraph();
    sess.current_block = update.focus.map(|id| id.to_string());

    print_context_update(format, &session, &update, sess)?;
    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn context_pin(
    input: Option<String>,
    session: String,
    target: String,
    pinned: bool,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;
    ensure_codegraph_document(&stateful.document)?;
    let document = stateful.document.clone();
    let block_id = resolve_selector(&document, &target)?;

    let sess = get_session_mut(&mut stateful, &session)?;
    let update = sess.ensure_codegraph_context().pin(block_id, pinned);
    sess.sync_context_blocks_from_codegraph();

    print_context_update(format, &session, &update, sess)?;
    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn context_prune(
    input: Option<String>,
    session: String,
    max_selected: Option<usize>,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;
    ensure_codegraph_document(&stateful.document)?;
    let document = stateful.document.clone();

    let sess = get_session_mut(&mut stateful, &session)?;
    let update = sess.ensure_codegraph_context().prune(&document, max_selected);
    sess.sync_context_blocks_from_codegraph();
    sess.current_block = update.focus.map(|id| id.to_string()).or_else(|| sess.current_block.clone());

    print_context_update(format, &session, &update, sess)?;
    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn ensure_codegraph_document(doc: &Document) -> Result<()> {
    if is_codegraph_document(doc) {
        Ok(())
    } else {
        Err(anyhow!("document is not a codegraph"))
    }
}

fn resolve_selectors(doc: &Document, selectors: &str) -> Result<Vec<BlockId>> {
    selectors
        .split(',')
        .map(|selector| resolve_selector(doc, selector.trim()))
        .collect()
}

fn resolve_selector(doc: &Document, selector: &str) -> Result<BlockId> {
    BlockId::from_str(selector)
        .ok()
        .or_else(|| resolve_codegraph_selector(doc, selector))
        .ok_or_else(|| anyhow!("Could not resolve selector: {}", selector))
}

fn get_session<'a>(stateful: &'a StatefulDocument, session: &str) -> Result<&'a AgentSessionState> {
    stateful
        .state()
        .sessions
        .get(session)
        .ok_or_else(|| anyhow!("Session not found: {}", session))
}

fn get_session_mut<'a>(
    stateful: &'a mut StatefulDocument,
    session: &str,
) -> Result<&'a mut AgentSessionState> {
    stateful
        .state_mut()
        .sessions
        .get_mut(session)
        .ok_or_else(|| anyhow!("Session not found: {}", session))
}

fn merge_updates(into: &mut CodeGraphContextUpdate, next: CodeGraphContextUpdate) {
    into.added.extend(next.added);
    into.removed.extend(next.removed);
    into.changed.extend(next.changed);
    into.focus = next.focus.or(into.focus);
    into.warnings.extend(next.warnings);
}

fn print_context_update(
    format: OutputFormat,
    session_id: &str,
    update: &CodeGraphContextUpdate,
    session: &AgentSessionState,
) -> Result<()> {
    match format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "success": true,
                    "session": session_id,
                    "added": update.added.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
                    "removed": update.removed.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
                    "changed": update.changed.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
                    "focus": update.focus.map(|id| id.to_string()),
                    "warnings": update.warnings,
                    "total": session.context_blocks.len(),
                }))?
            );
        }
        OutputFormat::Text => {
            print_success(&format!(
                "Updated codegraph context {} (added {}, removed {}, changed {}, total {})",
                session_id,
                update.added.len(),
                update.removed.len(),
                update.changed.len(),
                session.context_blocks.len()
            ));
            for warning in &update.warnings {
                print_warning(warning);
            }
        }
    }
    Ok(())
}

fn uuid_short() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", now % 0xFFFFFFFF)
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
