//! Agent traversal commands

use anyhow::{anyhow, Result};
use colored::Colorize;
use serde::Serialize;
use std::str::FromStr;
use ucm_core::BlockId;
use ucp_api::{
    is_codegraph_document, render_codegraph_context_prompt, resolve_codegraph_selector,
    CodeGraphDetailLevel, CodeGraphRenderConfig,
};

use crate::cli::{AgentCommands, AgentContextCommands, AgentSessionCommands, OutputFormat};
use crate::output::{content_preview, print_block, print_success, read_document};
use crate::state::{read_stateful_document, write_stateful_document, AgentSessionState};

pub fn handle(cmd: AgentCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        AgentCommands::Session(subcmd) => handle_session(subcmd, format),
        AgentCommands::Goto {
            input,
            session,
            target,
        } => goto(input, session, target, format),
        AgentCommands::Back {
            input,
            session,
            steps,
        } => back(input, session, steps, format),
        AgentCommands::Expand {
            input,
            session,
            id,
            direction,
            depth,
        } => expand(input, session, id, direction, depth, format),
        AgentCommands::Follow {
            input,
            session,
            edge_type,
        } => follow(input, session, edge_type, format),
        AgentCommands::Search {
            input,
            session,
            query,
            limit,
        } => search(input, session, query, limit, format),
        AgentCommands::Find {
            input,
            session,
            role,
            tag,
        } => find(input, session, role, tag, format),
        AgentCommands::Context(subcmd) => handle_context(subcmd, format),
        AgentCommands::View {
            input,
            session,
            mode,
        } => view(input, session, mode, format),
    }
}

fn handle_session(cmd: AgentSessionCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        AgentSessionCommands::Create { input, name, start } => {
            session_create(input, name, start, format)
        }
        AgentSessionCommands::List { input } => session_list(input, format),
        AgentSessionCommands::Close { session } => session_close(session, format),
    }
}

fn session_create(
    input: Option<String>,
    name: Option<String>,
    start: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;

    let start_block = if let Some(s) = start {
        Some(BlockId::from_str(&s).map_err(|_| anyhow!("Invalid start block ID: {}", s))?)
    } else {
        None
    };

    // Generate session ID
    let session_id = format!("sess_{}", uuid_short());

    let session = AgentSessionState::new(session_id.clone(), name.clone(), start_block);
    stateful
        .state_mut()
        .sessions
        .insert(session_id.clone(), session);

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct SessionResult {
                success: bool,
                session_id: String,
                name: Option<String>,
            }
            let result = SessionResult {
                success: true,
                session_id,
                name,
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            print_success(&format!("Created session: {}", session_id));
        }
    }

    // Write back to same input file or stdout
    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn session_list(input: Option<String>, format: OutputFormat) -> Result<()> {
    let stateful = read_stateful_document(input)?;
    let sessions = &stateful.state().sessions;

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct SessionInfo {
                id: String,
                name: Option<String>,
                current_block: Option<String>,
                context_blocks: usize,
                state: String,
            }
            let list: Vec<SessionInfo> = sessions
                .values()
                .map(|s| SessionInfo {
                    id: s.id.clone(),
                    name: s.name.clone(),
                    current_block: s.current_block.clone(),
                    context_blocks: s.context_blocks.len(),
                    state: s.state.clone(),
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&list)?);
        }
        OutputFormat::Text => {
            if sessions.is_empty() {
                println!("No active sessions");
            } else {
                println!("{}", "Active Sessions:".cyan().bold());
                for sess in sessions.values() {
                    let name_str = sess
                        .name
                        .as_ref()
                        .map(|n| format!(" ({})", n))
                        .unwrap_or_default();
                    println!(
                        "  {} {} - {} blocks in context",
                        sess.id.green(),
                        name_str,
                        sess.context_blocks.len()
                    );
                }
            }
        }
    }

    Ok(())
}

fn session_close(session: String, format: OutputFormat) -> Result<()> {
    // Note: Without input file, we can't actually persist this change
    // In practice, the user should provide an input file
    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct SessionResult {
                success: bool,
                session_id: String,
            }
            let result = SessionResult {
                success: true,
                session_id: session,
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            print_success(&format!("Session {} closed", session));
        }
    }
    Ok(())
}

fn goto(
    input: Option<String>,
    session: String,
    target: String,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;
    let document = stateful.document.clone();

    let target_id =
        BlockId::from_str(&target).map_err(|_| anyhow!("Invalid target block ID: {}", target))?;

    // Verify block exists
    if stateful.document.get_block(&target_id).is_none() {
        return Err(anyhow!("Block not found: {}", target));
    }

    let sess = stateful
        .state_mut()
        .sessions
        .get_mut(&session)
        .ok_or_else(|| anyhow!("Session not found: {}", session))?;

    sess.goto(&target_id);
    if let Some(context) = sess.codegraph_context.as_mut() {
        context.set_focus(&document, Some(target_id));
    }

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct GotoResult {
                success: bool,
                position: String,
            }
            let result = GotoResult {
                success: true,
                position: target,
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            print_success(&format!("Moved to {}", target));
            if let Some(block) = stateful.document.get_block(&target_id) {
                print_block(block, true);
            }
        }
    }

    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn back(input: Option<String>, session: String, steps: usize, format: OutputFormat) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;
    let document = stateful.document.clone();

    let sess = stateful
        .state_mut()
        .sessions
        .get_mut(&session)
        .ok_or_else(|| anyhow!("Session not found: {}", session))?;

    let new_pos = sess.back(steps);
    if let Some(context) = sess.codegraph_context.as_mut() {
        let focus = new_pos;
        context.set_focus(&document, focus);
    }

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct BackResult {
                success: bool,
                position: Option<String>,
            }
            let result = BackResult {
                success: new_pos.is_some(),
                position: new_pos.map(|p| p.to_string()),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            if let Some(pos) = new_pos {
                print_success(&format!("Moved back to {}", pos));
            } else {
                println!("No more history");
            }
        }
    }

    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn expand(
    input: Option<String>,
    session: String,
    id: Option<String>,
    direction: String,
    depth: usize,
    format: OutputFormat,
) -> Result<()> {
    let stateful = read_stateful_document(input)?;

    let sess = stateful
        .state()
        .sessions
        .get(&session)
        .ok_or_else(|| anyhow!("Session not found: {}", session))?;

    let block_id = if let Some(id_str) = id {
        BlockId::from_str(&id_str).map_err(|_| anyhow!("Invalid block ID: {}", id_str))?
    } else if let Some(curr) = &sess.current_block {
        BlockId::from_str(curr).map_err(|_| anyhow!("Invalid current block: {}", curr))?
    } else {
        stateful.document.root
    };

    // Collect blocks based on direction and depth
    fn collect_expansion(
        doc: &ucm_core::Document,
        block_id: &BlockId,
        direction: &str,
        depth: usize,
        current_depth: usize,
        results: &mut Vec<(BlockId, usize)>,
    ) {
        if current_depth > depth {
            return;
        }

        results.push((*block_id, current_depth));

        if direction == "down" || direction == "both" {
            for child_id in doc.children(block_id) {
                collect_expansion(doc, child_id, direction, depth, current_depth + 1, results);
            }
        }

        if direction == "up" || direction == "both" {
            if let Some(parent_id) = doc.parent(block_id) {
                if current_depth < depth {
                    collect_expansion(doc, parent_id, "up", depth, current_depth + 1, results);
                }
            }
        }
    }

    let mut results = Vec::new();
    collect_expansion(
        &stateful.document,
        &block_id,
        &direction,
        depth,
        0,
        &mut results,
    );

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct ExpandResult {
                root: String,
                direction: String,
                depth: usize,
                blocks: Vec<ExpandedBlock>,
            }
            #[derive(Serialize)]
            struct ExpandedBlock {
                id: String,
                level: usize,
                content_type: String,
                preview: String,
            }
            let blocks: Vec<ExpandedBlock> = results
                .iter()
                .filter_map(|(id, level)| {
                    stateful.document.get_block(id).map(|b| ExpandedBlock {
                        id: id.to_string(),
                        level: *level,
                        content_type: b.content.type_tag().to_string(),
                        preview: content_preview(&b.content, 100),
                    })
                })
                .collect();
            let result = ExpandResult {
                root: block_id.to_string(),
                direction,
                depth,
                blocks,
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            println!(
                "Expanded {} {} (depth {})",
                block_id,
                direction.cyan(),
                depth
            );
            for (id, level) in &results {
                if let Some(block) = stateful.document.get_block(id) {
                    let indent = "  ".repeat(*level);
                    let preview = content_preview(&block.content, 60);
                    let preview_line = preview.lines().next().unwrap_or("");
                    println!(
                        "{}[{}] {}",
                        indent,
                        id.to_string().yellow(),
                        preview_line.dimmed()
                    );
                }
            }
        }
    }

    Ok(())
}

fn follow(
    input: Option<String>,
    session: String,
    edge_type: String,
    format: OutputFormat,
) -> Result<()> {
    let stateful = read_stateful_document(input)?;

    let sess = stateful
        .state()
        .sessions
        .get(&session)
        .ok_or_else(|| anyhow!("Session not found: {}", session))?;

    let current = sess
        .current_block
        .as_ref()
        .ok_or_else(|| anyhow!("No current position"))?;
    let block_id = BlockId::from_str(current)?;

    let block = stateful
        .document
        .get_block(&block_id)
        .ok_or_else(|| anyhow!("Current block not found"))?;

    // Find edges of the specified type
    let matching_edges: Vec<_> = block
        .edges
        .iter()
        .filter(|e| format!("{:?}", e.edge_type).to_lowercase() == edge_type.to_lowercase())
        .collect();

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct FollowResult {
                edge_type: String,
                targets: Vec<String>,
            }
            let result = FollowResult {
                edge_type,
                targets: matching_edges
                    .iter()
                    .map(|e| e.target.to_string())
                    .collect(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            if matching_edges.is_empty() {
                println!("No edges of type '{}' found", edge_type);
            } else {
                println!("Following {} edges:", edge_type.cyan());
                for edge in matching_edges {
                    println!("  → {}", edge.target);
                }
            }
        }
    }

    Ok(())
}

fn search(
    input: Option<String>,
    _session: String,
    query: String,
    limit: usize,
    format: OutputFormat,
) -> Result<()> {
    let doc = read_document(input)?;

    // Simple text search (in a real implementation, this would use RAG)
    let query_lower = query.to_lowercase();
    let matches: Vec<_> = doc
        .blocks
        .values()
        .filter(|block| {
            let content_str = content_preview(&block.content, 10000).to_lowercase();
            content_str.contains(&query_lower)
        })
        .take(limit)
        .collect();

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct SearchResult {
                query: String,
                matches: Vec<SearchMatch>,
            }
            #[derive(Serialize)]
            struct SearchMatch {
                id: String,
                content_type: String,
                preview: String,
            }
            let result = SearchResult {
                query,
                matches: matches
                    .iter()
                    .map(|b| SearchMatch {
                        id: b.id.to_string(),
                        content_type: b.content.type_tag().to_string(),
                        preview: content_preview(&b.content, 100),
                    })
                    .collect(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            if matches.is_empty() {
                println!("No matches found for '{}'", query);
            } else {
                println!("Found {} matches for '{}':", matches.len(), query.green());
                for block in matches {
                    let preview = content_preview(&block.content, 80);
                    let preview_line = preview.lines().next().unwrap_or("");
                    println!("  [{}] {}", block.id.to_string().yellow(), preview_line);
                }
            }
        }
    }

    Ok(())
}

fn find(
    input: Option<String>,
    _session: String,
    role: Option<String>,
    tag: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let doc = read_document(input)?;

    let matches: Vec<_> = doc
        .blocks
        .values()
        .filter(|block| {
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
            if let Some(ref t) = tag {
                if !block.metadata.tags.iter().any(|bt| bt.contains(t)) {
                    return false;
                }
            }
            true
        })
        .collect();

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct FindResult {
                count: usize,
                blocks: Vec<String>,
            }
            let result = FindResult {
                count: matches.len(),
                blocks: matches.iter().map(|b| b.id.to_string()).collect(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            if matches.is_empty() {
                println!("No matching blocks found");
            } else {
                println!("Found {} blocks:", matches.len());
                for block in matches {
                    println!("  {}", block.id);
                }
            }
        }
    }

    Ok(())
}

fn handle_context(cmd: AgentContextCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        AgentContextCommands::Seed { input, session } => context_seed(input, session, format),
        AgentContextCommands::Add {
            input,
            session,
            ids,
        } => context_add(input, session, ids, format),
        AgentContextCommands::Remove {
            input,
            session,
            ids,
        } => context_remove(input, session, ids, format),
        AgentContextCommands::Focus {
            input,
            session,
            target,
        } => context_focus(input, session, target, format),
        AgentContextCommands::Expand {
            input,
            session,
            target,
            mode,
            relation,
        } => context_expand(input, session, target, mode, relation, format),
        AgentContextCommands::Hydrate {
            input,
            session,
            target,
            padding,
        } => context_hydrate(input, session, target, padding, format),
        AgentContextCommands::Collapse {
            input,
            session,
            target,
            descendants,
        } => context_collapse(input, session, target, descendants, format),
        AgentContextCommands::Pin {
            input,
            session,
            target,
        } => context_pin(input, session, target, true, format),
        AgentContextCommands::Unpin {
            input,
            session,
            target,
        } => context_pin(input, session, target, false, format),
        AgentContextCommands::Clear { input, session } => context_clear(input, session, format),
        AgentContextCommands::Show { input, session } => context_show(input, session, format),
    }
}

fn context_seed(input: Option<String>, session: String, format: OutputFormat) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;
    let document = stateful.document.clone();
    if !is_codegraph_document(&document) {
        return Err(anyhow!("context seed currently requires a codegraph document"));
    }

    let sess = get_session_mut(&mut stateful, &session)?;
    let update = sess.ensure_codegraph_context().seed_overview(&document);
    sess.sync_context_blocks_from_codegraph();
    sess.current_block = update.focus.map(|id| id.to_string());

    print_context_update(format, &session, &update, sess.context_blocks.len(), "Seeded overview")?;
    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn context_add(
    input: Option<String>,
    session: String,
    ids: String,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;

    let document = stateful.document.clone();
    let codegraph = is_codegraph_document(&document);
    let block_ids = resolve_selectors(&document, &ids)?;

    let sess = get_session_mut(&mut stateful, &session)?;

    for id in &block_ids {
        sess.add_to_context(id);
        if codegraph {
            sess.ensure_codegraph_context()
                .select_block(&document, *id, CodeGraphDetailLevel::SymbolCard);
        }
    }
    if codegraph {
        sess.sync_context_blocks_from_codegraph();
    }

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct ContextResult {
                success: bool,
                added: usize,
                total: usize,
            }
            let result = ContextResult {
                success: true,
                added: block_ids.len(),
                total: sess.context_blocks.len(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            print_success(&format!(
                "Added {} blocks to context ({} total)",
                block_ids.len(),
                sess.context_blocks.len()
            ));
        }
    }

    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn context_remove(
    input: Option<String>,
    session: String,
    ids: String,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;

    let document = stateful.document.clone();
    let codegraph = is_codegraph_document(&document);
    let block_ids = resolve_selectors(&document, &ids)?;

    let sess = get_session_mut(&mut stateful, &session)?;

    for id in &block_ids {
        sess.remove_from_context(id);
        if let Some(context) = sess.codegraph_context.as_mut() {
            context.remove_block(*id);
        }
    }
    if codegraph {
        sess.sync_context_blocks_from_codegraph();
    }

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct ContextResult {
                success: bool,
                removed: usize,
                total: usize,
            }
            let result = ContextResult {
                success: true,
                removed: block_ids.len(),
                total: sess.context_blocks.len(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            print_success(&format!(
                "Removed {} blocks from context ({} remaining)",
                block_ids.len(),
                sess.context_blocks.len()
            ));
        }
    }

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
    let document = stateful.document.clone();
    let codegraph = is_codegraph_document(&document);
    let target_id = target
        .as_deref()
        .map(|selector| resolve_selector(&document, selector))
        .transpose()?;

    let sess = get_session_mut(&mut stateful, &session)?;
    sess.current_block = target_id.map(|id| id.to_string());
    if codegraph {
        let update = sess.ensure_codegraph_context().set_focus(&document, target_id);
        sess.sync_context_blocks_from_codegraph();
        print_context_update(format, &session, &update, sess.context_blocks.len(), "Updated focus")?;
    } else {
        match format {
            OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                "success": true,
                "focus": target_id.map(|id| id.to_string())
            }))?),
            OutputFormat::Text => print_success("Updated focus"),
        }
    }

    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn context_expand(
    input: Option<String>,
    session: String,
    target: String,
    mode: String,
    relation: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;
    let document = stateful.document.clone();
    if !is_codegraph_document(&document) {
        return Err(anyhow!("context expand currently requires a codegraph document"));
    }
    let block_id = resolve_selector(&document, &target)?;

    let sess = get_session_mut(&mut stateful, &session)?;
    let update = match mode.as_str() {
        "file" => sess.ensure_codegraph_context().expand_file(&document, block_id),
        "dependencies" => sess
            .ensure_codegraph_context()
            .expand_dependencies(&document, block_id, relation.as_deref()),
        "dependents" => sess
            .ensure_codegraph_context()
            .expand_dependents(&document, block_id, relation.as_deref()),
        other => return Err(anyhow!("Unsupported expand mode: {}", other)),
    };
    sess.sync_context_blocks_from_codegraph();
    sess.current_block = update.focus.map(|id| id.to_string());

    print_context_update(format, &session, &update, sess.context_blocks.len(), "Expanded context")?;
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
    let document = stateful.document.clone();
    if !is_codegraph_document(&document) {
        return Err(anyhow!("context hydrate currently requires a codegraph document"));
    }
    let block_id = resolve_selector(&document, &target)?;

    let sess = get_session_mut(&mut stateful, &session)?;
    let update = sess
        .ensure_codegraph_context()
        .hydrate_source(&document, block_id, padding);
    sess.sync_context_blocks_from_codegraph();
    sess.current_block = update.focus.map(|id| id.to_string());

    print_context_update(format, &session, &update, sess.context_blocks.len(), "Hydrated source")?;
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
    let document = stateful.document.clone();
    if !is_codegraph_document(&document) {
        return Err(anyhow!("context collapse currently requires a codegraph document"));
    }
    let block_id = resolve_selector(&document, &target)?;

    let sess = get_session_mut(&mut stateful, &session)?;
    let update = sess
        .ensure_codegraph_context()
        .collapse(&document, block_id, descendants);
    sess.sync_context_blocks_from_codegraph();
    sess.current_block = update.focus.map(|id| id.to_string());

    print_context_update(format, &session, &update, sess.context_blocks.len(), "Collapsed context")?;
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
    let document = stateful.document.clone();
    if !is_codegraph_document(&document) {
        return Err(anyhow!("context pin currently requires a codegraph document"));
    }
    let block_id = resolve_selector(&document, &target)?;

    let sess = get_session_mut(&mut stateful, &session)?;
    sess.ensure_codegraph_context().pin(block_id, pinned);
    sess.sync_context_blocks_from_codegraph();

    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&serde_json::json!({
            "success": true,
            "session": session,
            "target": block_id.to_string(),
            "pinned": pinned,
            "total": sess.context_blocks.len()
        }))?),
        OutputFormat::Text => print_success(&format!(
            "{} {}",
            if pinned { "Pinned" } else { "Unpinned" },
            block_id
        )),
    }

    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn context_clear(input: Option<String>, session: String, format: OutputFormat) -> Result<()> {
    let mut stateful = read_stateful_document(input.clone())?;
    let sess = get_session_mut(&mut stateful, &session)?;
    sess.clear_context();
    sess.current_block = None;

    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&serde_json::json!({
            "success": true,
            "session": session,
            "count": 0
        }))?),
        OutputFormat::Text => print_success("Context cleared"),
    }

    write_stateful_document(&stateful, input)?;
    Ok(())
}

fn context_show(input: Option<String>, session: String, format: OutputFormat) -> Result<()> {
    let stateful = read_stateful_document(input)?;

    let sess = stateful
        .state()
        .sessions
        .get(&session)
        .ok_or_else(|| anyhow!("Session not found: {}", session))?;

    if is_codegraph_document(&stateful.document) {
        if let Some(context) = sess.codegraph_context.as_ref() {
            let rendered = render_codegraph_context_prompt(
                &stateful.document,
                context,
                &CodeGraphRenderConfig::default(),
            );
            match format {
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                        "session": session,
                        "focus": context.focus.map(|id| id.to_string()),
                        "summary": context.summary(&stateful.document),
                        "blocks": sess.context_blocks,
                        "rendered": rendered
                    }))?);
                }
                OutputFormat::Text => {
                    println!("{}", rendered);
                }
            }
            return Ok(());
        }
    }

    match format {
        OutputFormat::Json => {
            #[derive(Serialize)]
            struct ContextInfo {
                session: String,
                blocks: Vec<String>,
                count: usize,
            }
            let result = ContextInfo {
                session,
                blocks: sess.context_blocks.clone(),
                count: sess.context_blocks.len(),
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Text => {
            println!("{}", "Context Window:".cyan().bold());
            if sess.context_blocks.is_empty() {
                println!("  (empty)");
            } else {
                for id in &sess.context_blocks {
                    if let Ok(block_id) = BlockId::from_str(id) {
                        if let Some(block) = stateful.document.get_block(&block_id) {
                            let preview = content_preview(&block.content, 60);
                            let preview_line = preview.lines().next().unwrap_or("");
                            println!("  [{}] {}", id.yellow(), preview_line.dimmed());
                        } else {
                            println!("  [{}] (block not found)", id.yellow());
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn resolve_selectors(doc: &ucm_core::Document, selectors: &str) -> Result<Vec<BlockId>> {
    selectors
        .split(',')
        .map(|selector| resolve_selector(doc, selector.trim()))
        .collect()
}

fn resolve_selector(doc: &ucm_core::Document, selector: &str) -> Result<BlockId> {
    BlockId::from_str(selector)
        .ok()
        .or_else(|| {
            if is_codegraph_document(doc) {
                resolve_codegraph_selector(doc, selector)
            } else {
                None
            }
        })
        .ok_or_else(|| anyhow!("Could not resolve selector: {}", selector))
}

fn get_session_mut<'a>(
    stateful: &'a mut crate::state::StatefulDocument,
    session: &str,
) -> Result<&'a mut AgentSessionState> {
    stateful
        .state_mut()
        .sessions
        .get_mut(session)
        .ok_or_else(|| anyhow!("Session not found: {}", session))
}

fn print_context_update(
    format: OutputFormat,
    session: &str,
    update: &ucp_api::CodeGraphContextUpdate,
    total: usize,
    text_message: &str,
) -> Result<()> {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                "success": true,
                "session": session,
                "added": update.added.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
                "removed": update.removed.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
                "changed": update.changed.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
                "focus": update.focus.map(|id| id.to_string()),
                "warnings": update.warnings,
                "total": total
            }))?);
        }
        OutputFormat::Text => {
            print_success(&format!(
                "{} (added {}, removed {}, changed {}, total {})",
                text_message,
                update.added.len(),
                update.removed.len(),
                update.changed.len(),
                total
            ));
            if !update.warnings.is_empty() {
                for warning in &update.warnings {
                    eprintln!("warning: {}", warning);
                }
            }
        }
    }
    Ok(())
}

fn view(input: Option<String>, session: String, mode: String, format: OutputFormat) -> Result<()> {
    let stateful = read_stateful_document(input)?;

    let sess = stateful
        .state()
        .sessions
        .get(&session)
        .ok_or_else(|| anyhow!("Session not found: {}", session))?;

    let current = sess
        .current_block
        .as_ref()
        .ok_or_else(|| anyhow!("No current position"))?;
    let block_id = BlockId::from_str(current)?;

    let block = stateful
        .document
        .get_block(&block_id)
        .ok_or_else(|| anyhow!("Current block not found"))?;

    match format {
        OutputFormat::Json => match mode.as_str() {
            "metadata" => {
                println!("{}", serde_json::to_string_pretty(&block.metadata)?);
            }
            "ids" => {
                println!("{}", serde_json::to_string(&block.id)?);
            }
            _ => {
                println!("{}", serde_json::to_string_pretty(block)?);
            }
        },
        OutputFormat::Text => {
            print_block(block, mode != "metadata");
        }
    }

    Ok(())
}

/// Generate a short UUID-like string
fn uuid_short() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", now % 0xFFFFFFFF)
}
