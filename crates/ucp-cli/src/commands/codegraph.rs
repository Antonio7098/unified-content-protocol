use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use serde::Serialize;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use ucm_core::{BlockId, Document};
use ucp_api::{
    build_code_graph, build_code_graph_incremental, canonical_fingerprint,
    codegraph_prompt_projection, export_codegraph_context_with_config, is_codegraph_document,
    render_codegraph_context_prompt, resolve_codegraph_selector, validate_code_graph_profile,
    CodeGraphBuildInput, CodeGraphBuildStatus, CodeGraphContextExport,
    CodeGraphContextFrontierAction, CodeGraphContextUpdate, CodeGraphDetailLevel,
    CodeGraphExportConfig, CodeGraphExtractorConfig, CodeGraphIncrementalBuildInput,
    CodeGraphPrunePolicy, CodeGraphRenderConfig, CodeGraphSeverity, CodeGraphTraversalConfig,
};

use crate::cli::{CodegraphCommands, CodegraphContextCommands, OutputFormat};
use crate::output::{
    print_error, print_success, print_warning, read_document, write_output, DocumentJson,
};
use crate::state::{
    read_stateful_document, write_stateful_document, AgentSessionState,
    CodeGraphSessionPreferences, StatefulDocument,
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
            incremental,
            state_file,
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
            incremental,
            state_file,
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
    incremental: bool,
    state_file: Option<String>,
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

    let build_input = CodeGraphBuildInput {
        repository_path,
        commit_hash,
        config,
    };
    let incremental_state_file =
        resolve_incremental_state_file(incremental, state_file.as_deref(), output.as_deref())?;
    let result = if let Some(state_file) = incremental_state_file.clone() {
        build_code_graph_incremental(&CodeGraphIncrementalBuildInput {
            build: build_input,
            state_file,
        })
    } else {
        build_code_graph(&build_input)
    }
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
                incremental: Option<ucp_api::CodeGraphIncrementalStats>,
                incremental_state_file: Option<String>,
                diagnostics: Vec<ucp_api::CodeGraphDiagnostic>,
                document: DocumentJson,
            }

            let payload = JsonBuildOutput {
                status: result.status,
                profile_version: result.profile_version,
                canonical_fingerprint: result.canonical_fingerprint,
                stats: result.stats,
                incremental: result.incremental.clone(),
                incremental_state_file: incremental_state_file
                    .as_ref()
                    .map(|path| path.display().to_string()),
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
            if let Some(incremental) = &result.incremental {
                println!(
                    "incremental: scanned={} state_entries={} direct_invalidations={} surface_changes={} rebuilt={} reused={} added={} changed={} deleted={} invalidated={}{}",
                    incremental.scanned_files,
                    incremental.state_entries,
                    incremental.direct_invalidated_files,
                    incremental.surface_changed_files,
                    incremental.rebuilt_files,
                    incremental.reused_files,
                    incremental.added_files,
                    incremental.changed_files,
                    incremental.deleted_files,
                    incremental.invalidated_files,
                    incremental
                        .full_rebuild_reason
                        .as_ref()
                        .map(|reason| format!(" (full rebuild reason: {})", reason))
                        .unwrap_or_default()
                );
                if let Some(state_file) = &incremental_state_file {
                    println!("incremental_state: {}", state_file.display());
                }
            }

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

fn resolve_incremental_state_file(
    incremental: bool,
    state_file: Option<&str>,
    output: Option<&str>,
) -> Result<Option<PathBuf>> {
    match (incremental, state_file, output) {
        (false, Some(path), _) => Ok(Some(PathBuf::from(path))),
        (false, None, _) => Ok(None),
        (true, Some(path), _) => Ok(Some(PathBuf::from(path))),
        (true, None, Some(output)) => {
            let mut path = PathBuf::from(output);
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| anyhow!("output path must include a file name for incremental mode"))?;
            path.set_file_name(format!("{}.codegraph-state.json", file_name));
            Ok(Some(path))
        }
        (true, None, None) => Err(anyhow!(
            "incremental builds require either --output or --state-file so the state can be persisted"
        )),
    }
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
            focus,
            focus_mode,
            focus_depth,
            preset,
            default_relations,
            default_compact,
            default_levels,
            default_preset,
            default_depth,
            default_only,
            default_exclude,
        } => context_init(
            input,
            name,
            max_selected,
            initial_depth,
            focus,
            focus_mode,
            focus_depth,
            preset,
            default_relations,
            default_compact,
            default_levels,
            default_preset,
            default_depth,
            default_only,
            default_exclude,
            format,
        ),
        CodegraphContextCommands::Show {
            input,
            session,
            max_tokens,
            compact,
            no_rendered,
            levels,
            only,
            exclude,
        } => context_show(
            input,
            session,
            max_tokens,
            compact,
            no_rendered,
            levels,
            only,
            exclude,
            format,
        ),
        CodegraphContextCommands::Export {
            input,
            session,
            max_tokens,
            compact,
            no_rendered,
            levels,
            only,
            exclude,
        } => context_export(
            input,
            session,
            max_tokens,
            compact,
            no_rendered,
            levels,
            only,
            exclude,
            format,
        ),
        CodegraphContextCommands::Defaults {
            input,
            session,
            compact,
            no_compact,
            levels,
            clear_levels,
            preset,
            relations,
            clear_preset,
            clear_relations,
            depth,
            clear_depth,
            only,
            exclude,
            clear_only,
            clear_exclude,
        } => context_defaults(
            input,
            session,
            compact,
            no_compact,
            levels,
            clear_levels,
            preset,
            relations,
            clear_preset,
            clear_relations,
            depth,
            clear_depth,
            only,
            exclude,
            clear_only,
            clear_exclude,
            format,
        ),
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
            preset,
            depth,
            max_add,
            priority_threshold,
        } => context_expand(
            input,
            session,
            target,
            mode,
            relation,
            relations,
            preset,
            depth,
            max_add,
            priority_threshold,
            format,
        ),
        CodegraphContextCommands::ExpandRecommended {
            input,
            session,
            top,
            padding,
            depth,
            max_add,
            priority_threshold,
        } => context_expand_recommended(
            input,
            session,
            top,
            padding,
            depth,
            max_add,
            priority_threshold,
            format,
        ),
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

#[allow(clippy::too_many_arguments)]
fn context_init(
    input: Option<String>,
    name: Option<String>,
    max_selected: usize,
    initial_depth: Option<usize>,
    focus: Option<String>,
    focus_mode: String,
    focus_depth: usize,
    preset: Option<String>,
    default_relations: Option<String>,
    default_compact: bool,
    default_levels: Option<usize>,
    default_preset: Option<String>,
    default_depth: Option<usize>,
    default_only: Option<String>,
    default_exclude: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;
    ensure_codegraph_document(&stateful.document)?;
    let focus_id = focus
        .as_deref()
        .map(|selector| resolve_selector(&stateful.document, selector))
        .transpose()?;
    let default_relation_filters = parse_csv_arg(default_relations.as_deref())?;
    let default_only_classes = parse_node_classes(default_only.as_deref())?;
    let default_exclude_classes = parse_node_classes(default_exclude.as_deref())?;
    let preferences = build_session_preferences(
        default_compact,
        default_levels,
        default_preset,
        default_relation_filters,
        default_depth,
        default_only_classes,
        default_exclude_classes,
    )?;

    let session_id = format!("cgctx_{}", uuid_short());
    let mut session = AgentSessionState::new(session_id.clone(), name.clone(), None);
    session.codegraph_preferences = preferences.clone();
    {
        let context = session.ensure_codegraph_context();
        context.set_prune_policy(CodeGraphPrunePolicy {
            max_selected: max_selected.max(1),
            ..CodeGraphPrunePolicy::default()
        });
        let update = if let Some(block_id) = focus_id {
            let mut merged = CodeGraphContextUpdate::default();
            if initial_depth.is_some() {
                merge_updates(
                    &mut merged,
                    context.seed_overview_with_depth(&stateful.document, initial_depth),
                );
            }
            let traversal = CodeGraphTraversalConfig {
                depth: focus_depth.max(1),
                relation_filters: preset
                    .as_deref()
                    .map(expand_relation_preset)
                    .transpose()?
                    .unwrap_or_default(),
                ..CodeGraphTraversalConfig::default()
            };
            merge_updates(
                &mut merged,
                init_focus_first_context(
                    context,
                    &stateful.document,
                    block_id,
                    focus_mode.as_str(),
                    &traversal,
                )?,
            );
            merged
        } else {
            context.seed_overview_with_depth(&stateful.document, initial_depth)
        };
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
                    "focus": focus,
                    "focus_mode": focus_mode,
                    "preferences": preferences,
                    "summary": session.codegraph_context.as_ref().map(|ctx| ctx.summary(&stateful.document)),
                    "rendered": rendered,
                }))?
            );
        }
        OutputFormat::Text => {
            print_success(&format!(
                "Initialized codegraph context session: {}",
                session_id
            ));
        }
    }

    write_stateful_document(&stateful, input)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn context_show(
    input: Option<String>,
    session: String,
    max_tokens: usize,
    compact: bool,
    no_rendered: bool,
    levels: Option<usize>,
    only: Option<String>,
    exclude: Option<String>,
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
    let export_config = make_export_config(
        &sess.codegraph_preferences,
        compact,
        no_rendered,
        levels,
        only.as_deref(),
        exclude.as_deref(),
    )?;
    let export =
        export_codegraph_context_with_config(&stateful.document, context, &config, &export_config);

    match format {
        OutputFormat::Json => {
            let mut value = serde_json::to_value(&export)?;
            if let Some(object) = value.as_object_mut() {
                object.insert("session".to_string(), serde_json::Value::String(session));
            }
            println!("{}", serde_json::to_string_pretty(&value)?);
        }
        OutputFormat::Text => println!(
            "{}",
            render_context_show_text(&stateful.document, context, &config, &export)
        ),
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn context_export(
    input: Option<String>,
    session: String,
    max_tokens: usize,
    compact: bool,
    no_rendered: bool,
    levels: Option<usize>,
    only: Option<String>,
    exclude: Option<String>,
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
    let export_config = make_export_config(
        &sess.codegraph_preferences,
        compact,
        no_rendered,
        levels,
        only.as_deref(),
        exclude.as_deref(),
    )?;
    let export =
        export_codegraph_context_with_config(&stateful.document, context, &config, &export_config);

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

#[allow(clippy::too_many_arguments)]
fn context_defaults(
    input: Option<String>,
    session: String,
    compact: bool,
    no_compact: bool,
    levels: Option<usize>,
    clear_levels: bool,
    preset: Option<String>,
    relations: Option<String>,
    clear_preset: bool,
    clear_relations: bool,
    depth: Option<usize>,
    clear_depth: bool,
    only: Option<String>,
    exclude: Option<String>,
    clear_only: bool,
    clear_exclude: bool,
    format: OutputFormat,
) -> Result<()> {
    let has_updates = compact
        || no_compact
        || levels.is_some()
        || clear_levels
        || preset.is_some()
        || relations.is_some()
        || clear_preset
        || clear_relations
        || depth.is_some()
        || clear_depth
        || only.is_some()
        || exclude.is_some()
        || clear_only
        || clear_exclude;

    let mut stateful = read_stateful_document(input.clone())?;
    ensure_codegraph_document(&stateful.document)?;

    if !has_updates {
        let sess = get_session(&stateful, &session)?;
        return print_context_defaults(
            format,
            &session,
            &sess.codegraph_preferences,
            Vec::new(),
            false,
        );
    }

    let sess = get_session_mut(&mut stateful, &session)?;
    let (preferences, changed_fields) = update_session_preferences(
        &sess.codegraph_preferences,
        compact,
        no_compact,
        levels,
        clear_levels,
        preset,
        relations,
        clear_preset,
        clear_relations,
        depth,
        clear_depth,
        only,
        exclude,
        clear_only,
        clear_exclude,
    )?;
    sess.codegraph_preferences = preferences.clone();

    print_context_defaults(format, &session, &preferences, changed_fields, true)?;
    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn make_export_config(
    preferences: &CodeGraphSessionPreferences,
    compact: bool,
    no_rendered: bool,
    levels: Option<usize>,
    only: Option<&str>,
    exclude: Option<&str>,
) -> Result<CodeGraphExportConfig> {
    let effective_compact = compact || preferences.compact;
    let mut export_config = if effective_compact {
        CodeGraphExportConfig::compact()
    } else {
        CodeGraphExportConfig::default()
    };
    if no_rendered {
        export_config.include_rendered = false;
    }
    export_config.visible_levels = levels.or(preferences.levels);
    export_config.only_node_classes = if let Some(only) = only {
        parse_node_classes(Some(only))?
    } else {
        preferences.only_node_classes.clone()
    };
    export_config.exclude_node_classes = if let Some(exclude) = exclude {
        parse_node_classes(Some(exclude))?
    } else {
        preferences.exclude_node_classes.clone()
    };
    Ok(export_config)
}

fn render_context_show_text(
    document: &Document,
    context: &ucp_api::CodeGraphContextSession,
    config: &CodeGraphRenderConfig,
    export: &CodeGraphContextExport,
) -> String {
    if export.visible_levels.is_none() && export.visible_node_count == export.summary.selected {
        return render_codegraph_context_prompt(document, context, config);
    }

    let mut lines = vec![format!(
        "visible nodes: {} of {} selected",
        export.visible_node_count, export.summary.selected
    )];
    if export.visible_node_count < export.summary.selected && export.visible_levels.is_none() {
        lines.push("node-class filters applied".to_string());
    }
    if let Some(levels) = export.visible_levels {
        lines.push(format!("focus window: +{} levels", levels));
    }
    if !export.hidden_levels.is_empty() {
        for hidden in &export.hidden_levels {
            let suffix = match (&hidden.direction, &hidden.relation) {
                (Some(direction), Some(relation)) => format!(" via {} {}", direction, relation),
                (Some(direction), None) => format!(" via {}", direction),
                (None, Some(relation)) => format!(" via {}", relation),
                (None, None) => String::new(),
            };
            lines.push(format!(
                "+ {} nodes at level {}{}",
                hidden.count, hidden.level, suffix
            ));
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
) -> Result<Vec<String>> {
    if relation.is_some() && relations.is_some() {
        return Err(anyhow!("Use either --relation or --relations, not both"));
    }
    let raw = relation.or(relations);
    parse_csv_arg(raw.as_deref())
}

fn parse_csv_arg(raw: Option<&str>) -> Result<Vec<String>> {
    let Some(raw) = raw else {
        return Ok(Vec::new());
    };
    let mut values: Vec<String> = raw
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect();
    values.sort();
    values.dedup();
    Ok(values)
}

fn parse_node_classes(raw: Option<&str>) -> Result<Vec<String>> {
    let classes = parse_csv_arg(raw)?;
    let allowed = ["repository", "directory", "file", "symbol", "unknown"];
    for class in &classes {
        if !allowed.contains(&class.as_str()) {
            return Err(anyhow!("Unsupported node class filter: {}", class));
        }
    }
    Ok(classes)
}

fn expand_relation_preset(name: &str) -> Result<Vec<String>> {
    let relations = match name {
        "semantic" => vec!["uses_symbol", "calls", "imports_symbol", "reexports_symbol"],
        "imports" => vec!["imports_symbol", "reexports_symbol"],
        "reverse-impact" => vec!["uses_symbol", "calls", "imports_symbol", "reexports_symbol"],
        "references" => vec!["references", "cited_by", "links_to"],
        other => return Err(anyhow!("Unknown relation preset: {}", other)),
    };
    Ok(relations.into_iter().map(str::to_string).collect())
}

fn build_session_preferences(
    compact: bool,
    levels: Option<usize>,
    preset: Option<String>,
    relation_filters: Vec<String>,
    expand_depth: Option<usize>,
    only_node_classes: Vec<String>,
    exclude_node_classes: Vec<String>,
) -> Result<CodeGraphSessionPreferences> {
    if preset.is_some() && !relation_filters.is_empty() {
        return Err(anyhow!(
            "Use either --default-preset or --default-relations, not both"
        ));
    }
    if let Some(name) = preset.as_deref() {
        let _ = expand_relation_preset(name)?;
    }
    Ok(CodeGraphSessionPreferences {
        compact,
        levels,
        relation_preset: preset,
        relation_filters,
        expand_depth,
        only_node_classes,
        exclude_node_classes,
    })
}

#[allow(clippy::too_many_arguments)]
fn update_session_preferences(
    current: &CodeGraphSessionPreferences,
    compact: bool,
    no_compact: bool,
    levels: Option<usize>,
    clear_levels: bool,
    preset: Option<String>,
    relations: Option<String>,
    clear_preset: bool,
    clear_relations: bool,
    depth: Option<usize>,
    clear_depth: bool,
    only: Option<String>,
    exclude: Option<String>,
    clear_only: bool,
    clear_exclude: bool,
) -> Result<(CodeGraphSessionPreferences, Vec<String>)> {
    if compact && no_compact {
        return Err(anyhow!("Use either --compact or --no-compact, not both"));
    }
    if levels.is_some() && clear_levels {
        return Err(anyhow!("Use either --levels or --clear-levels, not both"));
    }
    if preset.is_some() && relations.is_some() {
        return Err(anyhow!("Use either --preset or --relations, not both"));
    }
    if preset.is_some() && clear_preset {
        return Err(anyhow!("Use either --preset or --clear-preset, not both"));
    }
    if relations.is_some() && clear_relations {
        return Err(anyhow!(
            "Use either --relations or --clear-relations, not both"
        ));
    }
    if depth.is_some() && clear_depth {
        return Err(anyhow!("Use either --depth or --clear-depth, not both"));
    }
    if only.is_some() && clear_only {
        return Err(anyhow!("Use either --only or --clear-only, not both"));
    }
    if exclude.is_some() && clear_exclude {
        return Err(anyhow!("Use either --exclude or --clear-exclude, not both"));
    }

    let mut next = current.clone();
    let mut changed_fields = Vec::new();

    if compact {
        next.compact = true;
        changed_fields.push("compact".to_string());
    }
    if no_compact {
        next.compact = false;
        changed_fields.push("compact".to_string());
    }
    if let Some(levels) = levels {
        next.levels = Some(levels);
        changed_fields.push("levels".to_string());
    }
    if clear_levels {
        next.levels = None;
        changed_fields.push("levels".to_string());
    }
    if let Some(preset) = preset {
        let _ = expand_relation_preset(&preset)?;
        next.relation_preset = Some(preset);
        next.relation_filters.clear();
        changed_fields.push("relation_preset".to_string());
        changed_fields.push("relation_filters".to_string());
    }
    if let Some(relations) = relations {
        next.relation_filters = parse_csv_arg(Some(relations.as_str()))?;
        next.relation_preset = None;
        changed_fields.push("relation_filters".to_string());
        changed_fields.push("relation_preset".to_string());
    }
    if clear_preset {
        next.relation_preset = None;
        changed_fields.push("relation_preset".to_string());
    }
    if clear_relations {
        next.relation_filters.clear();
        changed_fields.push("relation_filters".to_string());
    }
    if let Some(depth) = depth {
        next.expand_depth = Some(depth);
        changed_fields.push("expand_depth".to_string());
    }
    if clear_depth {
        next.expand_depth = None;
        changed_fields.push("expand_depth".to_string());
    }
    if let Some(only) = only {
        next.only_node_classes = parse_node_classes(Some(only.as_str()))?;
        changed_fields.push("only_node_classes".to_string());
    }
    if clear_only {
        next.only_node_classes.clear();
        changed_fields.push("only_node_classes".to_string());
    }
    if let Some(exclude) = exclude {
        next.exclude_node_classes = parse_node_classes(Some(exclude.as_str()))?;
        changed_fields.push("exclude_node_classes".to_string());
    }
    if clear_exclude {
        next.exclude_node_classes.clear();
        changed_fields.push("exclude_node_classes".to_string());
    }

    changed_fields.sort();
    changed_fields.dedup();

    Ok((next, changed_fields))
}

fn print_context_defaults(
    format: OutputFormat,
    session: &str,
    preferences: &CodeGraphSessionPreferences,
    changed_fields: Vec<String>,
    updated: bool,
) -> Result<()> {
    match format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "success": true,
                    "session": session,
                    "updated": updated,
                    "changed_fields": changed_fields,
                    "preferences": preferences,
                }))?
            );
        }
        OutputFormat::Text => {
            if updated {
                print_success(&format!(
                    "Updated codegraph defaults for session: {}",
                    session
                ));
                if !changed_fields.is_empty() {
                    println!("Changed: {}", changed_fields.join(", "));
                }
            } else {
                print_success(&format!("Codegraph defaults for session: {}", session));
            }
            println!("- compact: {}", preferences.compact);
            println!(
                "- levels: {}",
                preferences
                    .levels
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "<unset>".to_string())
            );
            println!(
                "- relation preset: {}",
                preferences
                    .relation_preset
                    .clone()
                    .unwrap_or_else(|| "<unset>".to_string())
            );
            println!(
                "- relation filters: {}",
                if preferences.relation_filters.is_empty() {
                    "<unset>".to_string()
                } else {
                    preferences.relation_filters.join(",")
                }
            );
            println!(
                "- expand depth: {}",
                preferences
                    .expand_depth
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "<unset>".to_string())
            );
            println!(
                "- only classes: {}",
                if preferences.only_node_classes.is_empty() {
                    "<unset>".to_string()
                } else {
                    preferences.only_node_classes.join(",")
                }
            );
            println!(
                "- exclude classes: {}",
                if preferences.exclude_node_classes.is_empty() {
                    "<unset>".to_string()
                } else {
                    preferences.exclude_node_classes.join(",")
                }
            );
        }
    }
    Ok(())
}

fn build_traversal_config(
    preferences: &CodeGraphSessionPreferences,
    relation: Option<String>,
    relations: Option<String>,
    preset: Option<String>,
    depth: Option<usize>,
    max_add: Option<usize>,
    priority_threshold: Option<u16>,
) -> Result<CodeGraphTraversalConfig> {
    if preset.is_some() && (relation.is_some() || relations.is_some()) {
        return Err(anyhow!("Use either relation filters or --preset, not both"));
    }

    let mut relation_filters = if let Some(preset) = preset.as_deref() {
        expand_relation_preset(preset)?
    } else {
        parse_relation_filters(relation, relations)?
    };

    if relation_filters.is_empty() {
        if !preferences.relation_filters.is_empty() {
            relation_filters = preferences.relation_filters.clone();
        } else if let Some(preset) = preferences.relation_preset.as_deref() {
            relation_filters = expand_relation_preset(preset)?;
        }
    }

    Ok(CodeGraphTraversalConfig {
        depth: depth.or(preferences.expand_depth).unwrap_or(1),
        relation_filters,
        max_add,
        priority_threshold,
    })
}

fn init_focus_first_context(
    context: &mut ucp_api::CodeGraphContextSession,
    document: &Document,
    block_id: BlockId,
    focus_mode: &str,
    traversal: &CodeGraphTraversalConfig,
) -> Result<CodeGraphContextUpdate> {
    let block = document
        .get_block(&block_id)
        .ok_or_else(|| anyhow!("Focused block no longer exists: {}", block_id))?;
    let node_class = block
        .metadata
        .custom
        .get("node_class")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown");

    let effective_mode = match focus_mode {
        "auto" => match node_class {
            "file" => "file",
            "symbol" => "dependencies",
            _ => "focus",
        },
        "file" | "dependencies" | "dependents" | "focus" => focus_mode,
        other => return Err(anyhow!("Unsupported focus mode: {}", other)),
    };

    Ok(match effective_mode {
        "file" => context.expand_file_with_config(document, block_id, traversal),
        "dependencies" => context.expand_dependencies_with_config(document, block_id, traversal),
        "dependents" => context.expand_dependents_with_config(document, block_id, traversal),
        "focus" => {
            let detail = if node_class == "symbol" {
                CodeGraphDetailLevel::SymbolCard
            } else {
                CodeGraphDetailLevel::Neighborhood
            };
            let mut merged = context.select_block(document, block_id, detail);
            merge_updates(&mut merged, context.set_focus(document, Some(block_id)));
            merged
        }
        _ => unreachable!(),
    })
}

fn action_summary(action: &CodeGraphContextFrontierAction) -> String {
    match action.relation.as_deref() {
        Some(relation) => format!(
            "{} {} via {} (priority {}, {} candidates)",
            action.action, action.short_id, relation, action.priority, action.candidate_count
        ),
        None => format!(
            "{} {} (priority {}, {} candidates)",
            action.action, action.short_id, action.priority, action.candidate_count
        ),
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
    sess.current_block = update
        .focus
        .map(|id| id.to_string())
        .or_else(|| sess.current_block.clone());

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
    let update = sess
        .ensure_codegraph_context()
        .set_focus(&document, target_id);
    sess.sync_context_blocks_from_codegraph();
    sess.current_block = update.focus.map(|id| id.to_string());

    print_context_update(format, &session, &update, sess)?;
    write_stateful_document(&stateful, input)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn context_expand(
    input: Option<String>,
    session: String,
    target: String,
    mode: String,
    relation: Option<String>,
    relations: Option<String>,
    preset: Option<String>,
    depth: Option<usize>,
    max_add: Option<usize>,
    priority_threshold: Option<u16>,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;
    ensure_codegraph_document(&stateful.document)?;
    let document = stateful.document.clone();
    let block_id = resolve_selector(&document, &target)?;

    let sess = get_session_mut(&mut stateful, &session)?;
    let traversal = build_traversal_config(
        &sess.codegraph_preferences,
        relation,
        relations,
        preset,
        depth,
        max_add,
        priority_threshold,
    )?;
    let update = match mode.as_str() {
        "file" => sess
            .ensure_codegraph_context()
            .expand_file_with_config(&document, block_id, &traversal),
        "dependencies" => sess
            .ensure_codegraph_context()
            .expand_dependencies_with_config(&document, block_id, &traversal),
        "dependents" => sess
            .ensure_codegraph_context()
            .expand_dependents_with_config(&document, block_id, &traversal),
        other => return Err(anyhow!("Unsupported expand mode: {}", other)),
    };
    sess.sync_context_blocks_from_codegraph();
    sess.current_block = update.focus.map(|id| id.to_string());

    print_context_update(format, &session, &update, sess)?;
    write_stateful_document(&stateful, input)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn context_expand_recommended(
    input: Option<String>,
    session: String,
    top: usize,
    padding: usize,
    depth: Option<usize>,
    max_add: Option<usize>,
    priority_threshold: Option<u16>,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;
    ensure_codegraph_document(&stateful.document)?;
    let document = stateful.document.clone();
    let export_config = CodeGraphExportConfig {
        max_frontier_actions: top.max(1).max(8),
        ..Default::default()
    };

    let actions = {
        let sess = get_session(&stateful, &session)?;
        let context = sess
            .codegraph_context
            .as_ref()
            .ok_or_else(|| anyhow!("Session has no codegraph context: {}", session))?;
        let export = export_codegraph_context_with_config(
            &document,
            context,
            &CodeGraphRenderConfig::default(),
            &export_config,
        );
        export
            .frontier
            .into_iter()
            .filter(|action| action.candidate_count > 0)
            .filter(|action| {
                priority_threshold
                    .map(|threshold| action.priority >= threshold)
                    .unwrap_or(true)
            })
            .take(top.max(1))
            .collect::<Vec<_>>()
    };

    if actions.is_empty() {
        return Err(anyhow!(
            "No recommended actions available for the current focus"
        ));
    }

    let sess = get_session_mut(&mut stateful, &session)?;
    let mut merged = CodeGraphContextUpdate::default();
    let mut applied = Vec::new();
    for action in actions {
        let traversal = CodeGraphTraversalConfig {
            depth: depth
                .or(sess.codegraph_preferences.expand_depth)
                .unwrap_or(1),
            relation_filters: action.relation.clone().into_iter().collect(),
            max_add,
            priority_threshold,
        };
        let next = match action.action.as_str() {
            "hydrate_source" => {
                sess.ensure_codegraph_context()
                    .hydrate_source(&document, action.block_id, padding)
            }
            "expand_file" => sess.ensure_codegraph_context().expand_file_with_config(
                &document,
                action.block_id,
                &traversal,
            ),
            "expand_dependencies" => sess
                .ensure_codegraph_context()
                .expand_dependencies_with_config(&document, action.block_id, &traversal),
            "expand_dependents" => sess
                .ensure_codegraph_context()
                .expand_dependents_with_config(&document, action.block_id, &traversal),
            "collapse" => {
                sess.ensure_codegraph_context()
                    .collapse(&document, action.block_id, false)
            }
            _ => continue,
        };
        applied.push(action_summary(&action));
        merge_updates(&mut merged, next);
    }

    sess.sync_context_blocks_from_codegraph();
    sess.current_block = merged
        .focus
        .map(|id| id.to_string())
        .or_else(|| sess.current_block.clone());

    match format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "success": true,
                    "session": session,
                    "applied_actions": applied,
                    "added": merged.added.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
                    "removed": merged.removed.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
                    "changed": merged.changed.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
                    "focus": merged.focus.map(|id| id.to_string()),
                    "warnings": merged.warnings,
                    "total": sess.context_blocks.len(),
                }))?
            );
        }
        OutputFormat::Text => {
            print_success(&format!(
                "Applied {} recommended action(s) to codegraph context {}",
                applied.len(),
                session
            ));
            for item in &applied {
                println!("- {}", item);
            }
            for warning in &merged.warnings {
                print_warning(warning);
            }
        }
    }

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
    let update = sess
        .ensure_codegraph_context()
        .prune(&document, max_selected);
    sess.sync_context_blocks_from_codegraph();
    sess.current_block = update
        .focus
        .map(|id| id.to_string())
        .or_else(|| sess.current_block.clone());

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
