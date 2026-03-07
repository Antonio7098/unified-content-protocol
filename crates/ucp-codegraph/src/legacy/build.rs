use anyhow::{anyhow, Context, Result};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::Path;
use ucm_core::{Block, BlockId, Content, Document, DocumentId, Edge, EdgeType};

use crate::model::*;

use super::languages::ts_js::extend_unique_block_ids;
use super::{
    alias_scope_key, analyze_file, ancestor_directories, canonical_fingerprint,
    collect_repository_files, compare_extracted_symbols, compute_stats, format_coderef,
    format_line_range, normalize_path, normalize_temporal_fields, parent_directory_id,
    parent_id_for_file, resolve_alias_target_ids, resolve_import, resolve_relationship_target_ids,
    resolve_usage_target_ids, sanitize_identifier, sort_edges,
    sort_structure_children_by_logical_key, unique_symbol_logical_key, validate_code_graph_profile,
    GitignoreMatcher,
};

pub fn build_code_graph(input: &CodeGraphBuildInput) -> Result<CodeGraphBuildResult> {
    let repo_root = input
        .repository_path
        .canonicalize()
        .with_context(|| format!("failed to resolve repo path {:?}", input.repository_path))?;

    if !repo_root.is_dir() {
        return Err(anyhow!(
            "repository path is not a directory: {}",
            repo_root.display()
        ));
    }

    let mut diagnostics = Vec::new();
    let matcher = GitignoreMatcher::from_repository(&repo_root)?;
    let files = collect_repository_files(&repo_root, &input.config, &matcher, &mut diagnostics)?;

    let repo_name = repo_root
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "repository".to_string());

    let mut doc = Document::new(DocumentId::new(format!(
        "codegraph:{}:{}",
        sanitize_identifier(&repo_name),
        sanitize_identifier(&input.commit_hash)
    )));

    initialize_document_metadata(&mut doc, &repo_root, &repo_name, &input.commit_hash);

    let repo_block = make_repository_block(&repo_name, &input.commit_hash);
    let root_id = doc.root;
    let repo_block_id = doc.add_block(repo_block, &root_id)?;

    let mut directories = BTreeSet::new();
    for file in &files {
        for dir in ancestor_directories(&file.relative_path) {
            directories.insert(dir);
        }
    }

    let mut directory_ids: BTreeMap<String, BlockId> = BTreeMap::new();
    for dir in directories {
        let parent_id = parent_directory_id(&dir, &directory_ids).unwrap_or(repo_block_id);
        let block = make_directory_block(&dir);
        let block_id = doc.add_block(block, &parent_id)?;
        directory_ids.insert(dir, block_id);
    }

    let mut file_ids: BTreeMap<String, BlockId> = BTreeMap::new();
    let mut symbol_ids_by_file_identity: BTreeMap<(String, String), BlockId> = BTreeMap::new();
    let mut top_level_symbol_ids: BTreeMap<(String, String), Vec<BlockId>> = BTreeMap::new();
    let mut exported_top_level_symbol_ids: BTreeMap<String, Vec<(String, BlockId)>> =
        BTreeMap::new();
    let mut default_exported_top_level_symbol_ids: BTreeMap<String, Vec<BlockId>> = BTreeMap::new();
    let mut file_analyses = Vec::new();
    let mut used_symbol_keys: HashSet<String> = HashSet::new();

    for file in files {
        let parent_id = parent_id_for_file(&file.relative_path, repo_block_id, &directory_ids);

        let source = match fs::read_to_string(&file.absolute_path) {
            Ok(s) => s,
            Err(err) => {
                let diag = CodeGraphDiagnostic::error(
                    "CG2003",
                    format!("failed to read source file: {}", err),
                )
                .with_path(file.relative_path.clone());
                diagnostics.push(diag);
                if input.config.continue_on_parse_error {
                    continue;
                }
                return Err(anyhow!(
                    "failed to read source file {}: {}",
                    file.relative_path,
                    err
                ));
            }
        };

        if source.len() > input.config.max_file_bytes {
            diagnostics.push(
                CodeGraphDiagnostic::warning(
                    "CG2008",
                    format!(
                        "file skipped due to size limit ({} bytes > {} bytes)",
                        source.len(),
                        input.config.max_file_bytes
                    ),
                )
                .with_path(file.relative_path.clone()),
            );
            continue;
        }

        let FileAnalysis {
            file_description,
            mut symbols,
            imports,
            relationships,
            usages,
            aliases,
            export_bindings,
            default_exported_symbol_names,
            diagnostics: analysis_diagnostics,
            ..
        } = analyze_file(&file.relative_path, &source, file.language);

        let file_block = make_file_block(
            &file.relative_path,
            file.language.as_str(),
            file_description.as_deref(),
        );
        let file_block_id = doc.add_block(file_block, &parent_id)?;
        file_ids.insert(file.relative_path.clone(), file_block_id);

        for diag in &analysis_diagnostics {
            diagnostics.push(diag.clone().with_path(file.relative_path.clone()));
        }

        symbols.sort_by(compare_extracted_symbols);
        let mut symbol_ids_by_identity: BTreeMap<String, BlockId> = BTreeMap::new();

        for symbol in &symbols {
            let parent_block_id = symbol
                .parent_identity
                .as_ref()
                .and_then(|identity| symbol_ids_by_identity.get(identity).copied())
                .unwrap_or(file_block_id);
            let logical_key = unique_symbol_logical_key(
                &file.relative_path,
                &symbol.qualified_name,
                symbol.start_line,
                &mut used_symbol_keys,
            );
            let symbol_block = make_symbol_block(
                &logical_key,
                &file.relative_path,
                file.language.as_str(),
                symbol,
            );
            let symbol_id = doc.add_block(symbol_block, &parent_block_id)?;
            symbol_ids_by_identity.insert(symbol.identity.clone(), symbol_id);
            symbol_ids_by_file_identity.insert(
                (file.relative_path.clone(), symbol.identity.clone()),
                symbol_id,
            );

            if symbol.parent_identity.is_none() {
                top_level_symbol_ids
                    .entry((file.relative_path.clone(), symbol.name.clone()))
                    .or_default()
                    .push(symbol_id);
                if symbol.exported {
                    exported_top_level_symbol_ids
                        .entry(file.relative_path.clone())
                        .or_default()
                        .push((symbol.name.clone(), symbol_id));
                    if default_exported_symbol_names.contains(&symbol.name) {
                        default_exported_top_level_symbol_ids
                            .entry(file.relative_path.clone())
                            .or_default()
                            .push(symbol_id);
                    }
                }
            }

            if symbol.exported && input.config.emit_export_edges {
                let mut edge = Edge::new(EdgeType::Custom("exports".to_string()), symbol_id);
                edge.metadata
                    .custom
                    .insert("relation".to_string(), json!("exports"));
                edge.metadata
                    .custom
                    .insert("symbol".to_string(), json!(symbol.name.clone()));
                if let Some(source_block) = doc.get_block_mut(&file_block_id) {
                    source_block.edges.push(edge);
                }
            }
        }

        file_analyses.push(FileAnalysisRecord {
            file: file.relative_path,
            language: file.language,
            imports,
            relationships,
            usages,
            aliases,
            export_bindings,
        });
    }

    let known_files: BTreeSet<String> = file_ids.keys().cloned().collect();
    let mut exported_symbol_targets_by_file: BTreeMap<String, BTreeMap<String, Vec<BlockId>>> =
        BTreeMap::new();
    let mut imported_symbol_targets_by_file: BTreeMap<String, BTreeMap<String, Vec<BlockId>>> =
        BTreeMap::new();
    let mut imported_module_targets_by_file: BTreeMap<String, BTreeMap<String, Vec<String>>> =
        BTreeMap::new();
    let mut imported_module_paths_by_file: BTreeMap<String, BTreeMap<String, Vec<String>>> =
        BTreeMap::new();
    let mut alias_names_by_scope: BTreeMap<(String, String), BTreeSet<String>> = BTreeMap::new();
    let mut alias_records_by_scope: BTreeMap<
        (String, String),
        BTreeMap<String, Vec<ExtractedAlias>>,
    > = BTreeMap::new();
    let mut aliased_symbol_targets_by_scope: BTreeMap<
        (String, String),
        BTreeMap<String, Vec<BlockId>>,
    > = BTreeMap::new();
    let mut pending_reference_edges: BTreeSet<(String, String, String)> = BTreeSet::new();
    let mut pending_symbol_reference_edges: BTreeSet<(String, String, String, String)> =
        BTreeSet::new();
    let mut pending_wildcard_symbol_reference_edges: BTreeSet<(String, String, String)> =
        BTreeSet::new();
    let mut pending_reexport_edges: BTreeSet<(String, String, String, String)> = BTreeSet::new();
    let mut pending_wildcard_reexport_edges: BTreeSet<(String, String, String, Vec<String>)> =
        BTreeSet::new();
    let mut pending_relationship_edges: Vec<(BlockId, BlockId, String, String)> = Vec::new();
    let mut pending_usage_edges: Vec<(BlockId, BlockId, String)> = Vec::new();

    for (file, exports) in &exported_top_level_symbol_ids {
        let entry = exported_symbol_targets_by_file
            .entry(file.clone())
            .or_default();
        for (name, symbol_id) in exports {
            entry.entry(name.clone()).or_default().push(*symbol_id);
        }
    }
    for (file, ids) in &default_exported_top_level_symbol_ids {
        exported_symbol_targets_by_file
            .entry(file.clone())
            .or_default()
            .entry("default".to_string())
            .or_default()
            .extend(ids.iter().copied());
    }
    for record in &file_analyses {
        let entry = exported_symbol_targets_by_file
            .entry(record.file.clone())
            .or_default();
        for binding in &record.export_bindings {
            if let Some(ids) =
                top_level_symbol_ids.get(&(record.file.clone(), binding.local_name.clone()))
            {
                extend_unique_block_ids(
                    entry.entry(binding.source_name.clone()).or_default(),
                    ids.iter().copied(),
                );
            }
        }
    }

    for targets in exported_symbol_targets_by_file.values_mut() {
        for ids in targets.values_mut() {
            let existing = std::mem::take(ids);
            extend_unique_block_ids(ids, existing);
        }
    }

    for _ in 0..=file_analyses.len() {
        let mut progress = false;

        for record in &file_analyses {
            for import in &record.imports {
                if !import.reexported {
                    continue;
                }

                let ImportResolution::Resolved(target) =
                    resolve_import(&record.file, &record.language, &import.module, &known_files)
                else {
                    continue;
                };

                let target_exports = exported_symbol_targets_by_file
                    .get(&target)
                    .cloned()
                    .unwrap_or_default();
                let entry = exported_symbol_targets_by_file
                    .entry(record.file.clone())
                    .or_default();

                if import.wildcard {
                    for (export_name, ids) in target_exports.clone() {
                        if export_name == "default" {
                            continue;
                        }
                        let targets = entry.entry(export_name).or_default();
                        progress |= extend_unique_block_ids(targets, ids.iter().copied());
                    }
                }

                for binding in &import.bindings {
                    if let Some(ids) = target_exports.get(&binding.source_name) {
                        let targets = entry.entry(binding.local_name.clone()).or_default();
                        progress |= extend_unique_block_ids(targets, ids.iter().copied());
                    }
                }
            }
        }

        if !progress {
            break;
        }
    }

    for record in &file_analyses {
        for import in &record.imports {
            match resolve_import(&record.file, &record.language, &import.module, &known_files) {
                ImportResolution::Resolved(target) if target != record.file => {
                    pending_reference_edges.insert((
                        record.file.clone(),
                        target.clone(),
                        import.module.clone(),
                    ));

                    for symbol_name in &import.symbols {
                        pending_symbol_reference_edges.insert((
                            record.file.clone(),
                            target.clone(),
                            symbol_name.clone(),
                            import.module.clone(),
                        ));
                        if import.reexported {
                            pending_reexport_edges.insert((
                                record.file.clone(),
                                target.clone(),
                                symbol_name.clone(),
                                import.module.clone(),
                            ));
                        }
                    }

                    if matches!(record.language, CodeLanguage::Rust | CodeLanguage::Python)
                        && import.wildcard
                    {
                        if let Some(exports) = exported_symbol_targets_by_file.get(&target) {
                            let entry = imported_symbol_targets_by_file
                                .entry(record.file.clone())
                                .or_default();
                            for (export_name, target_symbol_ids) in exports {
                                if export_name == "default" {
                                    continue;
                                }
                                entry
                                    .entry(export_name.clone())
                                    .or_default()
                                    .extend(target_symbol_ids.iter().copied());
                            }
                        }
                    }

                    if !import.bindings.is_empty() {
                        let entry = imported_symbol_targets_by_file
                            .entry(record.file.clone())
                            .or_default();
                        for binding in &import.bindings {
                            if let Some(target_symbol_ids) = exported_symbol_targets_by_file
                                .get(&target)
                                .and_then(|exports| exports.get(&binding.source_name))
                            {
                                entry
                                    .entry(binding.local_name.clone())
                                    .or_default()
                                    .extend(target_symbol_ids.iter().copied());
                            }
                        }
                    }

                    if !import.module_aliases.is_empty() {
                        let path_entry = imported_module_paths_by_file
                            .entry(record.file.clone())
                            .or_default();
                        for alias in &import.module_aliases {
                            let paths = path_entry.entry(alias.clone()).or_default();
                            if !paths.contains(&import.module) {
                                paths.push(import.module.clone());
                            }
                        }

                        let entry = imported_module_targets_by_file
                            .entry(record.file.clone())
                            .or_default();
                        for alias in &import.module_aliases {
                            let targets = entry.entry(alias.clone()).or_default();
                            if !targets.contains(&target) {
                                targets.push(target.clone());
                            }
                        }
                    }

                    if import.reexported && import.wildcard && import.symbols.is_empty() {
                        pending_wildcard_reexport_edges.insert((
                            record.file.clone(),
                            target.clone(),
                            import.module.clone(),
                            import.symbols.clone(),
                        ));
                    }

                    if import.wildcard && import.symbols.is_empty() {
                        pending_wildcard_symbol_reference_edges.insert((
                            record.file.clone(),
                            target,
                            import.module.clone(),
                        ));
                    }
                }
                ImportResolution::Resolved(_) | ImportResolution::External => {}
                ImportResolution::Unresolved => {
                    diagnostics.push(
                        CodeGraphDiagnostic::warning(
                            "CG2006",
                            format!("unresolved import '{}'", import.module),
                        )
                        .with_path(record.file.clone()),
                    );
                }
            }
        }
    }

    for targets in imported_symbol_targets_by_file.values_mut() {
        for symbol_ids in targets.values_mut() {
            let mut unique_ids = Vec::new();
            for symbol_id in symbol_ids.drain(..) {
                if !unique_ids.contains(&symbol_id) {
                    unique_ids.push(symbol_id);
                }
            }
            *symbol_ids = unique_ids;
        }
    }

    for targets in imported_module_targets_by_file.values_mut() {
        for file_paths in targets.values_mut() {
            file_paths.sort();
            file_paths.dedup();
        }
    }

    for targets in imported_module_paths_by_file.values_mut() {
        for module_paths in targets.values_mut() {
            module_paths.sort();
            module_paths.dedup();
        }
    }

    for record in &file_analyses {
        for alias in &record.aliases {
            let scope_key = alias_scope_key(alias.owner_identity.as_deref());
            alias_names_by_scope
                .entry((record.file.clone(), scope_key.clone()))
                .or_default()
                .insert(alias.name.clone());
            alias_records_by_scope
                .entry((record.file.clone(), scope_key))
                .or_default()
                .entry(alias.name.clone())
                .or_default()
                .push(alias.clone());
        }
    }

    let mut unresolved_aliases = file_analyses
        .iter()
        .flat_map(|record| {
            record
                .aliases
                .iter()
                .cloned()
                .map(|alias| (record.file.clone(), record.language, alias))
        })
        .collect::<Vec<_>>();

    while !unresolved_aliases.is_empty() {
        let mut next_unresolved = Vec::new();
        let mut made_progress = false;

        for (file, language, alias) in unresolved_aliases {
            let target_ids = resolve_alias_target_ids(
                &file,
                language,
                &alias,
                &top_level_symbol_ids,
                &exported_symbol_targets_by_file,
                &imported_symbol_targets_by_file,
                &imported_module_targets_by_file,
                &imported_module_paths_by_file,
                &alias_names_by_scope,
                &aliased_symbol_targets_by_scope,
                &known_files,
            );
            if target_ids.is_empty() {
                next_unresolved.push((file, language, alias));
                continue;
            }

            aliased_symbol_targets_by_scope
                .entry((file, alias_scope_key(alias.owner_identity.as_deref())))
                .or_default()
                .entry(alias.name)
                .or_default()
                .extend(target_ids);
            made_progress = true;
        }

        if !made_progress {
            break;
        }
        unresolved_aliases = next_unresolved;
    }

    for targets in aliased_symbol_targets_by_scope.values_mut() {
        for symbol_ids in targets.values_mut() {
            let mut unique_ids = Vec::new();
            for symbol_id in symbol_ids.drain(..) {
                if !unique_ids.contains(&symbol_id) {
                    unique_ids.push(symbol_id);
                }
            }
            *symbol_ids = unique_ids;
        }
    }

    for record in &file_analyses {
        for relationship in &record.relationships {
            let Some(source_id) = symbol_ids_by_file_identity
                .get(&(record.file.clone(), relationship.source_identity.clone()))
            else {
                continue;
            };

            for target_id in resolve_relationship_target_ids(
                &record.file,
                record.language,
                relationship,
                &top_level_symbol_ids,
                &imported_symbol_targets_by_file,
                &known_files,
            ) {
                if target_id == *source_id {
                    continue;
                }
                let edge = (
                    *source_id,
                    target_id,
                    relationship.relation.clone(),
                    relationship.target_expr.clone(),
                );
                if !pending_relationship_edges.contains(&edge) {
                    pending_relationship_edges.push(edge);
                }
            }
        }
    }

    for record in &file_analyses {
        for usage in &record.usages {
            let Some(source_id) = symbol_ids_by_file_identity
                .get(&(record.file.clone(), usage.source_identity.clone()))
            else {
                continue;
            };

            for target_id in resolve_usage_target_ids(
                &record.file,
                record.language,
                usage,
                &top_level_symbol_ids,
                &exported_symbol_targets_by_file,
                &imported_symbol_targets_by_file,
                &imported_module_targets_by_file,
                &imported_module_paths_by_file,
                &alias_names_by_scope,
                &alias_records_by_scope,
                &aliased_symbol_targets_by_scope,
                &known_files,
            ) {
                let edge = (*source_id, target_id, usage.target_expr.clone());
                if !pending_usage_edges.contains(&edge) {
                    pending_usage_edges.push(edge);
                }
            }
        }
    }

    for (source_path, target_path, raw_import) in pending_reference_edges {
        let (Some(source_id), Some(target_id)) =
            (file_ids.get(&source_path), file_ids.get(&target_path))
        else {
            continue;
        };
        let mut edge = Edge::new(EdgeType::References, *target_id);
        edge.metadata
            .custom
            .insert("relation".to_string(), json!("imports"));
        edge.metadata
            .custom
            .insert("raw_import".to_string(), json!(raw_import));
        if let Some(source_block) = doc.get_block_mut(source_id) {
            source_block.edges.push(edge);
        }
    }

    for (source_path, target_path, symbol_name, raw_import) in pending_symbol_reference_edges {
        let Some(source_id) = file_ids.get(&source_path) else {
            continue;
        };
        let Some(target_symbol_ids) =
            top_level_symbol_ids.get(&(target_path.clone(), symbol_name.clone()))
        else {
            continue;
        };

        for target_symbol_id in target_symbol_ids {
            let mut edge = Edge::new(
                EdgeType::Custom("imports_symbol".to_string()),
                *target_symbol_id,
            );
            edge.metadata
                .custom
                .insert("relation".to_string(), json!("imports_symbol"));
            edge.metadata
                .custom
                .insert("raw_import".to_string(), json!(raw_import.clone()));
            edge.metadata
                .custom
                .insert("symbol".to_string(), json!(symbol_name.clone()));
            if let Some(source_block) = doc.get_block_mut(source_id) {
                source_block.edges.push(edge);
            }
        }
    }

    for (source_path, target_path, raw_import) in pending_wildcard_symbol_reference_edges {
        let Some(source_id) = file_ids.get(&source_path) else {
            continue;
        };
        let Some(target_symbols) = exported_top_level_symbol_ids.get(&target_path) else {
            continue;
        };

        for (symbol_name, target_symbol_id) in target_symbols {
            let mut edge = Edge::new(
                EdgeType::Custom("imports_symbol".to_string()),
                *target_symbol_id,
            );
            edge.metadata
                .custom
                .insert("relation".to_string(), json!("imports_symbol"));
            edge.metadata
                .custom
                .insert("raw_import".to_string(), json!(raw_import.clone()));
            edge.metadata
                .custom
                .insert("symbol".to_string(), json!(symbol_name.clone()));
            if let Some(source_block) = doc.get_block_mut(source_id) {
                source_block.edges.push(edge);
            }
        }
    }

    if input.config.emit_export_edges {
        for (source_path, target_path, symbol_name, raw_import) in pending_reexport_edges {
            let Some(source_id) = file_ids.get(&source_path) else {
                continue;
            };
            let Some(target_symbol_ids) =
                top_level_symbol_ids.get(&(target_path.clone(), symbol_name.clone()))
            else {
                continue;
            };

            for target_symbol_id in target_symbol_ids {
                let mut edge =
                    Edge::new(EdgeType::Custom("exports".to_string()), *target_symbol_id);
                edge.metadata
                    .custom
                    .insert("relation".to_string(), json!("reexports"));
                edge.metadata
                    .custom
                    .insert("raw_import".to_string(), json!(raw_import.clone()));
                edge.metadata
                    .custom
                    .insert("symbol".to_string(), json!(symbol_name.clone()));
                if let Some(source_block) = doc.get_block_mut(source_id) {
                    source_block.edges.push(edge);
                }
            }
        }

        for (source_path, target_path, raw_import, filter_names) in pending_wildcard_reexport_edges
        {
            let Some(source_id) = file_ids.get(&source_path) else {
                continue;
            };
            let Some(target_symbols) = exported_top_level_symbol_ids.get(&target_path) else {
                continue;
            };

            for (symbol_name, target_symbol_id) in target_symbols {
                if !filter_names.is_empty() && !filter_names.contains(symbol_name) {
                    continue;
                }
                let mut edge =
                    Edge::new(EdgeType::Custom("exports".to_string()), *target_symbol_id);
                edge.metadata
                    .custom
                    .insert("relation".to_string(), json!("reexports"));
                edge.metadata
                    .custom
                    .insert("raw_import".to_string(), json!(raw_import.clone()));
                edge.metadata
                    .custom
                    .insert("symbol".to_string(), json!(symbol_name.clone()));
                if let Some(source_block) = doc.get_block_mut(source_id) {
                    source_block.edges.push(edge);
                }
            }
        }
    }

    for (source_id, target_id, relation, raw_target) in pending_relationship_edges {
        let mut edge = Edge::new(EdgeType::Custom(relation.clone()), target_id);
        edge.metadata
            .custom
            .insert("relation".to_string(), json!(relation));
        edge.metadata
            .custom
            .insert("raw_target".to_string(), json!(raw_target));
        if let Some(source_block) = doc.get_block_mut(&source_id) {
            source_block.edges.push(edge);
        }
    }

    for (source_id, target_id, raw_target) in pending_usage_edges {
        let mut edge = Edge::new(EdgeType::Custom("uses_symbol".to_string()), target_id);
        edge.metadata
            .custom
            .insert("relation".to_string(), json!("uses_symbol"));
        edge.metadata
            .custom
            .insert("raw_target".to_string(), json!(raw_target));
        if let Some(source_block) = doc.get_block_mut(&source_id) {
            source_block.edges.push(edge);
        }
    }

    sort_structure_children_by_logical_key(&mut doc);
    sort_edges(&mut doc);
    normalize_temporal_fields(&mut doc);
    doc.rebuild_indices();

    let mut validation = validate_code_graph_profile(&doc);
    diagnostics.append(&mut validation.diagnostics);

    let fingerprint = canonical_fingerprint(&doc)?;
    let stats = compute_stats(&doc);

    let has_profile_errors = diagnostics
        .iter()
        .any(|d| d.severity == CodeGraphSeverity::Error && d.code.starts_with("CG100"));
    let has_non_info = diagnostics
        .iter()
        .any(|d| d.severity != CodeGraphSeverity::Info);

    let status = if has_profile_errors {
        CodeGraphBuildStatus::FailedValidation
    } else if has_non_info {
        CodeGraphBuildStatus::PartialSuccess
    } else {
        CodeGraphBuildStatus::Success
    };

    Ok(CodeGraphBuildResult {
        document: doc,
        diagnostics,
        stats,
        profile_version: CODEGRAPH_PROFILE_MARKER.to_string(),
        canonical_fingerprint: fingerprint,
        status,
        incremental: None,
    })
}

#[derive(Debug, Clone)]
pub(super) struct LoadedRepoFile {
    pub repo_file: RepoFile,
    pub content_hash: Option<String>,
    pub source: Option<String>,
    pub diagnostics: Vec<CodeGraphDiagnostic>,
}

#[derive(Debug, Clone)]
pub(super) struct AnalyzedRepoFile {
    pub relative_path: String,
    pub language: CodeLanguage,
    pub content_hash: Option<String>,
    pub analysis: Option<FileAnalysis>,
    pub diagnostics: Vec<CodeGraphDiagnostic>,
}

#[derive(Debug, Clone)]
pub(super) struct AssembledCodeGraph {
    pub result: CodeGraphBuildResult,
    pub dependencies_by_file: BTreeMap<String, Vec<String>>,
}

pub(super) fn hash_source(source: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(source.as_bytes());
    hex::encode(hasher.finalize())
}

pub(super) fn load_repo_file(
    repo_file: &RepoFile,
    config: &CodeGraphExtractorConfig,
) -> Result<LoadedRepoFile> {
    let source = match fs::read_to_string(&repo_file.absolute_path) {
        Ok(source) => source,
        Err(err) => {
            let diag = CodeGraphDiagnostic::error(
                "CG2003",
                format!("failed to read source file: {}", err),
            )
            .with_path(repo_file.relative_path.clone());
            if config.continue_on_parse_error {
                return Ok(LoadedRepoFile {
                    repo_file: repo_file.clone(),
                    content_hash: None,
                    source: None,
                    diagnostics: vec![diag],
                });
            }
            return Err(anyhow!(
                "failed to read source file {}: {}",
                repo_file.relative_path,
                err
            ));
        }
    };

    let content_hash = hash_source(&source);
    if source.len() > config.max_file_bytes {
        let diag = CodeGraphDiagnostic::warning(
            "CG2008",
            format!(
                "file skipped due to size limit ({} bytes > {} bytes)",
                source.len(),
                config.max_file_bytes
            ),
        )
        .with_path(repo_file.relative_path.clone());
        return Ok(LoadedRepoFile {
            repo_file: repo_file.clone(),
            content_hash: Some(content_hash),
            source: None,
            diagnostics: vec![diag],
        });
    }

    Ok(LoadedRepoFile {
        repo_file: repo_file.clone(),
        content_hash: Some(content_hash),
        source: Some(source),
        diagnostics: Vec::new(),
    })
}

pub(super) fn analyze_loaded_repo_file(loaded: LoadedRepoFile) -> AnalyzedRepoFile {
    let mut diagnostics = loaded.diagnostics;
    let analysis = loaded.source.as_ref().map(|source| {
        let analysis = analyze_file(
            &loaded.repo_file.relative_path,
            source,
            loaded.repo_file.language,
        );
        for diag in &analysis.diagnostics {
            diagnostics.push(
                diag.clone()
                    .with_path(loaded.repo_file.relative_path.clone()),
            );
        }
        analysis
    });

    AnalyzedRepoFile {
        relative_path: loaded.repo_file.relative_path,
        language: loaded.repo_file.language,
        content_hash: loaded.content_hash,
        analysis,
        diagnostics,
    }
}

pub(super) fn assemble_code_graph_from_analyzed_files(
    repo_root: &Path,
    repo_name: &str,
    commit_hash: &str,
    config: &CodeGraphExtractorConfig,
    analyzed_files: &[AnalyzedRepoFile],
    mut diagnostics: Vec<CodeGraphDiagnostic>,
) -> Result<AssembledCodeGraph> {
    let mut doc = Document::new(DocumentId::new(format!(
        "codegraph:{}:{}",
        sanitize_identifier(repo_name),
        sanitize_identifier(commit_hash)
    )));
    initialize_document_metadata(&mut doc, repo_root, repo_name, commit_hash);

    let repo_block = make_repository_block(repo_name, commit_hash);
    let root_id = doc.root;
    let repo_block_id = doc.add_block(repo_block, &root_id)?;

    let mut directories = BTreeSet::new();
    for file in analyzed_files {
        for dir in ancestor_directories(&file.relative_path) {
            directories.insert(dir);
        }
    }

    let mut directory_ids: BTreeMap<String, BlockId> = BTreeMap::new();
    for dir in directories {
        let parent_id = parent_directory_id(&dir, &directory_ids).unwrap_or(repo_block_id);
        let block = make_directory_block(&dir);
        let block_id = doc.add_block(block, &parent_id)?;
        directory_ids.insert(dir, block_id);
    }

    let mut file_ids: BTreeMap<String, BlockId> = BTreeMap::new();
    let mut symbol_ids_by_file_identity: BTreeMap<(String, String), BlockId> = BTreeMap::new();
    let mut symbol_file_by_id: HashMap<BlockId, String> = HashMap::new();
    let mut top_level_symbol_ids: BTreeMap<(String, String), Vec<BlockId>> = BTreeMap::new();
    let mut exported_top_level_symbol_ids: BTreeMap<String, Vec<(String, BlockId)>> =
        BTreeMap::new();
    let mut default_exported_top_level_symbol_ids: BTreeMap<String, Vec<BlockId>> = BTreeMap::new();
    let mut file_analyses = Vec::new();
    let mut used_symbol_keys: HashSet<String> = HashSet::new();

    for analyzed_file in analyzed_files {
        let parent_id =
            parent_id_for_file(&analyzed_file.relative_path, repo_block_id, &directory_ids);
        diagnostics.extend(analyzed_file.diagnostics.clone());

        let Some(analysis) = analyzed_file.analysis.as_ref() else {
            continue;
        };

        let file_block = make_file_block(
            &analyzed_file.relative_path,
            analyzed_file.language.as_str(),
            analysis.file_description.as_deref(),
        );
        let file_block_id = doc.add_block(file_block, &parent_id)?;
        file_ids.insert(analyzed_file.relative_path.clone(), file_block_id);

        let mut symbols = analysis.symbols.clone();
        symbols.sort_by(compare_extracted_symbols);
        let mut symbol_ids_by_identity: BTreeMap<String, BlockId> = BTreeMap::new();

        for symbol in &symbols {
            let parent_block_id = symbol
                .parent_identity
                .as_ref()
                .and_then(|identity| symbol_ids_by_identity.get(identity).copied())
                .unwrap_or(file_block_id);
            let logical_key = unique_symbol_logical_key(
                &analyzed_file.relative_path,
                &symbol.qualified_name,
                symbol.start_line,
                &mut used_symbol_keys,
            );
            let symbol_block = make_symbol_block(
                &logical_key,
                &analyzed_file.relative_path,
                analyzed_file.language.as_str(),
                symbol,
            );
            let symbol_id = doc.add_block(symbol_block, &parent_block_id)?;
            symbol_ids_by_identity.insert(symbol.identity.clone(), symbol_id);
            symbol_file_by_id.insert(symbol_id, analyzed_file.relative_path.clone());
            symbol_ids_by_file_identity.insert(
                (analyzed_file.relative_path.clone(), symbol.identity.clone()),
                symbol_id,
            );

            if symbol.parent_identity.is_none() {
                top_level_symbol_ids
                    .entry((analyzed_file.relative_path.clone(), symbol.name.clone()))
                    .or_default()
                    .push(symbol_id);
                if symbol.exported {
                    exported_top_level_symbol_ids
                        .entry(analyzed_file.relative_path.clone())
                        .or_default()
                        .push((symbol.name.clone(), symbol_id));
                    if analysis
                        .default_exported_symbol_names
                        .contains(&symbol.name)
                    {
                        default_exported_top_level_symbol_ids
                            .entry(analyzed_file.relative_path.clone())
                            .or_default()
                            .push(symbol_id);
                    }
                }
            }

            if symbol.exported && config.emit_export_edges {
                let mut edge = Edge::new(EdgeType::Custom("exports".to_string()), symbol_id);
                edge.metadata
                    .custom
                    .insert("relation".to_string(), json!("exports"));
                edge.metadata
                    .custom
                    .insert("symbol".to_string(), json!(symbol.name.clone()));
                if let Some(source_block) = doc.get_block_mut(&file_block_id) {
                    source_block.edges.push(edge);
                }
            }
        }

        file_analyses.push(FileAnalysisRecord {
            file: analyzed_file.relative_path.clone(),
            language: analyzed_file.language,
            imports: analysis.imports.clone(),
            relationships: analysis.relationships.clone(),
            usages: analysis.usages.clone(),
            aliases: analysis.aliases.clone(),
            export_bindings: analysis.export_bindings.clone(),
        });
    }

    let known_files: BTreeSet<String> = file_ids.keys().cloned().collect();
    let mut dependencies_by_file: BTreeMap<String, BTreeSet<String>> = analyzed_files
        .iter()
        .map(|file| (file.relative_path.clone(), BTreeSet::new()))
        .collect();
    let mut exported_symbol_targets_by_file: BTreeMap<String, BTreeMap<String, Vec<BlockId>>> =
        BTreeMap::new();
    let mut imported_symbol_targets_by_file: BTreeMap<String, BTreeMap<String, Vec<BlockId>>> =
        BTreeMap::new();
    let mut imported_module_targets_by_file: BTreeMap<String, BTreeMap<String, Vec<String>>> =
        BTreeMap::new();
    let mut imported_module_paths_by_file: BTreeMap<String, BTreeMap<String, Vec<String>>> =
        BTreeMap::new();
    let mut alias_names_by_scope: BTreeMap<(String, String), BTreeSet<String>> = BTreeMap::new();
    let mut alias_records_by_scope: BTreeMap<
        (String, String),
        BTreeMap<String, Vec<ExtractedAlias>>,
    > = BTreeMap::new();
    let mut aliased_symbol_targets_by_scope: BTreeMap<
        (String, String),
        BTreeMap<String, Vec<BlockId>>,
    > = BTreeMap::new();
    let mut pending_reference_edges: BTreeSet<(String, String, String)> = BTreeSet::new();
    let mut pending_symbol_reference_edges: BTreeSet<(String, String, String, String)> =
        BTreeSet::new();
    let mut pending_wildcard_symbol_reference_edges: BTreeSet<(String, String, String)> =
        BTreeSet::new();
    let mut pending_reexport_edges: BTreeSet<(String, String, String, String)> = BTreeSet::new();
    let mut pending_wildcard_reexport_edges: BTreeSet<(String, String, String, Vec<String>)> =
        BTreeSet::new();
    let mut pending_relationship_edges: Vec<(BlockId, BlockId, String, String)> = Vec::new();
    let mut pending_usage_edges: Vec<(BlockId, BlockId, String)> = Vec::new();

    for (file, exports) in &exported_top_level_symbol_ids {
        let entry = exported_symbol_targets_by_file
            .entry(file.clone())
            .or_default();
        for (name, symbol_id) in exports {
            entry.entry(name.clone()).or_default().push(*symbol_id);
        }
    }
    for (file, ids) in &default_exported_top_level_symbol_ids {
        exported_symbol_targets_by_file
            .entry(file.clone())
            .or_default()
            .entry("default".to_string())
            .or_default()
            .extend(ids.iter().copied());
    }
    for record in &file_analyses {
        let entry = exported_symbol_targets_by_file
            .entry(record.file.clone())
            .or_default();
        for binding in &record.export_bindings {
            if let Some(ids) =
                top_level_symbol_ids.get(&(record.file.clone(), binding.local_name.clone()))
            {
                extend_unique_block_ids(
                    entry.entry(binding.source_name.clone()).or_default(),
                    ids.iter().copied(),
                );
            }
        }
    }

    for targets in exported_symbol_targets_by_file.values_mut() {
        for ids in targets.values_mut() {
            let existing = std::mem::take(ids);
            extend_unique_block_ids(ids, existing);
        }
    }

    for _ in 0..=file_analyses.len() {
        let mut progress = false;

        for record in &file_analyses {
            for import in &record.imports {
                if !import.reexported {
                    continue;
                }

                let ImportResolution::Resolved(target) =
                    resolve_import(&record.file, &record.language, &import.module, &known_files)
                else {
                    continue;
                };

                let target_exports = exported_symbol_targets_by_file
                    .get(&target)
                    .cloned()
                    .unwrap_or_default();
                let entry = exported_symbol_targets_by_file
                    .entry(record.file.clone())
                    .or_default();

                if import.wildcard {
                    for (export_name, ids) in target_exports.clone() {
                        if export_name == "default" {
                            continue;
                        }
                        let targets = entry.entry(export_name).or_default();
                        progress |= extend_unique_block_ids(targets, ids.iter().copied());
                    }
                }

                for binding in &import.bindings {
                    if let Some(ids) = target_exports.get(&binding.source_name) {
                        let targets = entry.entry(binding.local_name.clone()).or_default();
                        progress |= extend_unique_block_ids(targets, ids.iter().copied());
                    }
                }
            }
        }

        if !progress {
            break;
        }
    }

    for record in &file_analyses {
        for import in &record.imports {
            match resolve_import(&record.file, &record.language, &import.module, &known_files) {
                ImportResolution::Resolved(target) if target != record.file => {
                    dependencies_by_file
                        .entry(record.file.clone())
                        .or_default()
                        .insert(target.clone());
                    pending_reference_edges.insert((
                        record.file.clone(),
                        target.clone(),
                        import.module.clone(),
                    ));

                    for symbol_name in &import.symbols {
                        pending_symbol_reference_edges.insert((
                            record.file.clone(),
                            target.clone(),
                            symbol_name.clone(),
                            import.module.clone(),
                        ));
                        if import.reexported {
                            pending_reexport_edges.insert((
                                record.file.clone(),
                                target.clone(),
                                symbol_name.clone(),
                                import.module.clone(),
                            ));
                        }
                    }

                    if matches!(record.language, CodeLanguage::Rust | CodeLanguage::Python)
                        && import.wildcard
                    {
                        if let Some(exports) = exported_symbol_targets_by_file.get(&target) {
                            let entry = imported_symbol_targets_by_file
                                .entry(record.file.clone())
                                .or_default();
                            for (export_name, target_symbol_ids) in exports {
                                if export_name == "default" {
                                    continue;
                                }
                                entry
                                    .entry(export_name.clone())
                                    .or_default()
                                    .extend(target_symbol_ids.iter().copied());
                            }
                        }
                    }

                    if !import.bindings.is_empty() {
                        let entry = imported_symbol_targets_by_file
                            .entry(record.file.clone())
                            .or_default();
                        for binding in &import.bindings {
                            if let Some(target_symbol_ids) = exported_symbol_targets_by_file
                                .get(&target)
                                .and_then(|exports| exports.get(&binding.source_name))
                            {
                                entry
                                    .entry(binding.local_name.clone())
                                    .or_default()
                                    .extend(target_symbol_ids.iter().copied());
                            }
                        }
                    }

                    if !import.module_aliases.is_empty() {
                        let path_entry = imported_module_paths_by_file
                            .entry(record.file.clone())
                            .or_default();
                        for alias in &import.module_aliases {
                            let paths = path_entry.entry(alias.clone()).or_default();
                            if !paths.contains(&import.module) {
                                paths.push(import.module.clone());
                            }
                        }

                        let entry = imported_module_targets_by_file
                            .entry(record.file.clone())
                            .or_default();
                        for alias in &import.module_aliases {
                            let targets = entry.entry(alias.clone()).or_default();
                            if !targets.contains(&target) {
                                targets.push(target.clone());
                            }
                        }
                    }

                    if import.reexported && import.wildcard && import.symbols.is_empty() {
                        pending_wildcard_reexport_edges.insert((
                            record.file.clone(),
                            target.clone(),
                            import.module.clone(),
                            import.symbols.clone(),
                        ));
                    }

                    if import.wildcard && import.symbols.is_empty() {
                        pending_wildcard_symbol_reference_edges.insert((
                            record.file.clone(),
                            target,
                            import.module.clone(),
                        ));
                    }
                }
                ImportResolution::Resolved(_) | ImportResolution::External => {}
                ImportResolution::Unresolved => {
                    diagnostics.push(
                        CodeGraphDiagnostic::warning(
                            "CG2006",
                            format!("unresolved import '{}'", import.module),
                        )
                        .with_path(record.file.clone()),
                    );
                }
            }
        }
    }

    for targets in imported_symbol_targets_by_file.values_mut() {
        for symbol_ids in targets.values_mut() {
            let mut unique_ids = Vec::new();
            for symbol_id in symbol_ids.drain(..) {
                if !unique_ids.contains(&symbol_id) {
                    unique_ids.push(symbol_id);
                }
            }
            *symbol_ids = unique_ids;
        }
    }

    for targets in imported_module_targets_by_file.values_mut() {
        for file_paths in targets.values_mut() {
            file_paths.sort();
            file_paths.dedup();
        }
    }

    for targets in imported_module_paths_by_file.values_mut() {
        for module_paths in targets.values_mut() {
            module_paths.sort();
            module_paths.dedup();
        }
    }

    for record in &file_analyses {
        for alias in &record.aliases {
            let scope_key = alias_scope_key(alias.owner_identity.as_deref());
            alias_names_by_scope
                .entry((record.file.clone(), scope_key.clone()))
                .or_default()
                .insert(alias.name.clone());
            alias_records_by_scope
                .entry((record.file.clone(), scope_key))
                .or_default()
                .entry(alias.name.clone())
                .or_default()
                .push(alias.clone());
        }
    }

    let mut unresolved_aliases = file_analyses
        .iter()
        .flat_map(|record| {
            record
                .aliases
                .iter()
                .cloned()
                .map(|alias| (record.file.clone(), record.language, alias))
        })
        .collect::<Vec<_>>();

    while !unresolved_aliases.is_empty() {
        let mut next_unresolved = Vec::new();
        let mut made_progress = false;

        for (file, language, alias) in unresolved_aliases {
            let target_ids = resolve_alias_target_ids(
                &file,
                language,
                &alias,
                &top_level_symbol_ids,
                &exported_symbol_targets_by_file,
                &imported_symbol_targets_by_file,
                &imported_module_targets_by_file,
                &imported_module_paths_by_file,
                &alias_names_by_scope,
                &aliased_symbol_targets_by_scope,
                &known_files,
            );
            if target_ids.is_empty() {
                next_unresolved.push((file, language, alias));
                continue;
            }

            aliased_symbol_targets_by_scope
                .entry((file, alias_scope_key(alias.owner_identity.as_deref())))
                .or_default()
                .entry(alias.name)
                .or_default()
                .extend(target_ids);
            made_progress = true;
        }

        if !made_progress {
            break;
        }
        unresolved_aliases = next_unresolved;
    }

    for targets in aliased_symbol_targets_by_scope.values_mut() {
        for symbol_ids in targets.values_mut() {
            let mut unique_ids = Vec::new();
            for symbol_id in symbol_ids.drain(..) {
                if !unique_ids.contains(&symbol_id) {
                    unique_ids.push(symbol_id);
                }
            }
            *symbol_ids = unique_ids;
        }
    }

    for record in &file_analyses {
        for relationship in &record.relationships {
            let Some(source_id) = symbol_ids_by_file_identity
                .get(&(record.file.clone(), relationship.source_identity.clone()))
            else {
                continue;
            };

            for target_id in resolve_relationship_target_ids(
                &record.file,
                record.language,
                relationship,
                &top_level_symbol_ids,
                &imported_symbol_targets_by_file,
                &known_files,
            ) {
                if target_id == *source_id {
                    continue;
                }
                if let Some(target_file) = symbol_file_by_id.get(&target_id) {
                    if target_file != &record.file {
                        dependencies_by_file
                            .entry(record.file.clone())
                            .or_default()
                            .insert(target_file.clone());
                    }
                }
                let edge = (
                    *source_id,
                    target_id,
                    relationship.relation.clone(),
                    relationship.target_expr.clone(),
                );
                if !pending_relationship_edges.contains(&edge) {
                    pending_relationship_edges.push(edge);
                }
            }
        }
    }

    for record in &file_analyses {
        for usage in &record.usages {
            let Some(source_id) = symbol_ids_by_file_identity
                .get(&(record.file.clone(), usage.source_identity.clone()))
            else {
                continue;
            };

            for target_id in resolve_usage_target_ids(
                &record.file,
                record.language,
                usage,
                &top_level_symbol_ids,
                &exported_symbol_targets_by_file,
                &imported_symbol_targets_by_file,
                &imported_module_targets_by_file,
                &imported_module_paths_by_file,
                &alias_names_by_scope,
                &alias_records_by_scope,
                &aliased_symbol_targets_by_scope,
                &known_files,
            ) {
                if let Some(target_file) = symbol_file_by_id.get(&target_id) {
                    if target_file != &record.file {
                        dependencies_by_file
                            .entry(record.file.clone())
                            .or_default()
                            .insert(target_file.clone());
                    }
                }
                let edge = (*source_id, target_id, usage.target_expr.clone());
                if !pending_usage_edges.contains(&edge) {
                    pending_usage_edges.push(edge);
                }
            }
        }
    }

    for (source_path, target_path, raw_import) in pending_reference_edges {
        let (Some(source_id), Some(target_id)) =
            (file_ids.get(&source_path), file_ids.get(&target_path))
        else {
            continue;
        };
        let mut edge = Edge::new(EdgeType::References, *target_id);
        edge.metadata
            .custom
            .insert("relation".to_string(), json!("imports"));
        edge.metadata
            .custom
            .insert("raw_import".to_string(), json!(raw_import));
        if let Some(source_block) = doc.get_block_mut(source_id) {
            source_block.edges.push(edge);
        }
    }

    for (source_path, target_path, symbol_name, raw_import) in pending_symbol_reference_edges {
        let Some(source_id) = file_ids.get(&source_path) else {
            continue;
        };
        let Some(target_symbol_ids) =
            top_level_symbol_ids.get(&(target_path.clone(), symbol_name.clone()))
        else {
            continue;
        };

        for target_symbol_id in target_symbol_ids {
            let mut edge = Edge::new(
                EdgeType::Custom("imports_symbol".to_string()),
                *target_symbol_id,
            );
            edge.metadata
                .custom
                .insert("relation".to_string(), json!("imports_symbol"));
            edge.metadata
                .custom
                .insert("raw_import".to_string(), json!(raw_import.clone()));
            edge.metadata
                .custom
                .insert("symbol".to_string(), json!(symbol_name.clone()));
            if let Some(source_block) = doc.get_block_mut(source_id) {
                source_block.edges.push(edge);
            }
        }
    }

    for (source_path, target_path, raw_import) in pending_wildcard_symbol_reference_edges {
        let Some(source_id) = file_ids.get(&source_path) else {
            continue;
        };
        let Some(target_symbols) = exported_top_level_symbol_ids.get(&target_path) else {
            continue;
        };

        for (symbol_name, target_symbol_id) in target_symbols {
            let mut edge = Edge::new(
                EdgeType::Custom("imports_symbol".to_string()),
                *target_symbol_id,
            );
            edge.metadata
                .custom
                .insert("relation".to_string(), json!("imports_symbol"));
            edge.metadata
                .custom
                .insert("raw_import".to_string(), json!(raw_import.clone()));
            edge.metadata
                .custom
                .insert("symbol".to_string(), json!(symbol_name.clone()));
            if let Some(source_block) = doc.get_block_mut(source_id) {
                source_block.edges.push(edge);
            }
        }
    }

    if config.emit_export_edges {
        for (source_path, target_path, symbol_name, raw_import) in pending_reexport_edges {
            let Some(source_id) = file_ids.get(&source_path) else {
                continue;
            };
            let Some(target_symbol_ids) =
                top_level_symbol_ids.get(&(target_path.clone(), symbol_name.clone()))
            else {
                continue;
            };

            for target_symbol_id in target_symbol_ids {
                let mut edge =
                    Edge::new(EdgeType::Custom("exports".to_string()), *target_symbol_id);
                edge.metadata
                    .custom
                    .insert("relation".to_string(), json!("reexports"));
                edge.metadata
                    .custom
                    .insert("raw_import".to_string(), json!(raw_import.clone()));
                edge.metadata
                    .custom
                    .insert("symbol".to_string(), json!(symbol_name.clone()));
                if let Some(source_block) = doc.get_block_mut(source_id) {
                    source_block.edges.push(edge);
                }
            }
        }

        for (source_path, target_path, raw_import, filter_names) in pending_wildcard_reexport_edges
        {
            let Some(source_id) = file_ids.get(&source_path) else {
                continue;
            };
            let Some(target_symbols) = exported_top_level_symbol_ids.get(&target_path) else {
                continue;
            };

            for (symbol_name, target_symbol_id) in target_symbols {
                if !filter_names.is_empty() && !filter_names.contains(symbol_name) {
                    continue;
                }
                let mut edge =
                    Edge::new(EdgeType::Custom("exports".to_string()), *target_symbol_id);
                edge.metadata
                    .custom
                    .insert("relation".to_string(), json!("reexports"));
                edge.metadata
                    .custom
                    .insert("raw_import".to_string(), json!(raw_import.clone()));
                edge.metadata
                    .custom
                    .insert("symbol".to_string(), json!(symbol_name.clone()));
                if let Some(source_block) = doc.get_block_mut(source_id) {
                    source_block.edges.push(edge);
                }
            }
        }
    }

    for (source_id, target_id, relation, raw_target) in pending_relationship_edges {
        let mut edge = Edge::new(EdgeType::Custom(relation.clone()), target_id);
        edge.metadata
            .custom
            .insert("relation".to_string(), json!(relation));
        edge.metadata
            .custom
            .insert("raw_target".to_string(), json!(raw_target));
        if let Some(source_block) = doc.get_block_mut(&source_id) {
            source_block.edges.push(edge);
        }
    }

    for (source_id, target_id, raw_target) in pending_usage_edges {
        let mut edge = Edge::new(EdgeType::Custom("uses_symbol".to_string()), target_id);
        edge.metadata
            .custom
            .insert("relation".to_string(), json!("uses_symbol"));
        edge.metadata
            .custom
            .insert("raw_target".to_string(), json!(raw_target));
        if let Some(source_block) = doc.get_block_mut(&source_id) {
            source_block.edges.push(edge);
        }
    }

    sort_structure_children_by_logical_key(&mut doc);
    sort_edges(&mut doc);
    normalize_temporal_fields(&mut doc);
    doc.rebuild_indices();

    let mut validation = validate_code_graph_profile(&doc);
    diagnostics.append(&mut validation.diagnostics);

    let fingerprint = canonical_fingerprint(&doc)?;
    let stats = compute_stats(&doc);
    let has_profile_errors = diagnostics
        .iter()
        .any(|d| d.severity == CodeGraphSeverity::Error && d.code.starts_with("CG100"));
    let has_non_info = diagnostics
        .iter()
        .any(|d| d.severity != CodeGraphSeverity::Info);
    let status = if has_profile_errors {
        CodeGraphBuildStatus::FailedValidation
    } else if has_non_info {
        CodeGraphBuildStatus::PartialSuccess
    } else {
        CodeGraphBuildStatus::Success
    };

    Ok(AssembledCodeGraph {
        result: CodeGraphBuildResult {
            document: doc,
            diagnostics,
            stats,
            profile_version: CODEGRAPH_PROFILE_MARKER.to_string(),
            canonical_fingerprint: fingerprint,
            status,
            incremental: None,
        },
        dependencies_by_file: dependencies_by_file
            .into_iter()
            .map(|(file, deps)| (file, deps.into_iter().collect()))
            .collect(),
    })
}

pub(super) fn initialize_document_metadata(
    doc: &mut Document,
    repo_root: &Path,
    repo_name: &str,
    commit: &str,
) {
    doc.metadata.title = Some(format!("CodeGraph: {}", repo_name));
    doc.metadata.description = Some("CodeGraphProfile v1 document".to_string());
    doc.metadata.language = Some("multi".to_string());
    doc.metadata
        .custom
        .insert("profile".to_string(), json!(CODEGRAPH_PROFILE));
    doc.metadata.custom.insert(
        "profile_version".to_string(),
        json!(CODEGRAPH_PROFILE_VERSION),
    );
    doc.metadata.custom.insert(
        "profile_marker".to_string(),
        json!(CODEGRAPH_PROFILE_MARKER),
    );
    doc.metadata.custom.insert(
        "extractor_version".to_string(),
        json!(CODEGRAPH_EXTRACTOR_VERSION),
    );
    doc.metadata
        .custom
        .insert("commit_hash".to_string(), json!(commit));
    doc.metadata.custom.insert(
        "repository_path".to_string(),
        json!(normalize_path(repo_root)),
    );
}

pub(super) fn make_repository_block(repo_name: &str, commit_hash: &str) -> Block {
    let coderef = json!({
        "path": ".",
        "display": repo_name,
    });
    let mut block = Block::new(
        Content::json(json!({
            "coderef": coderef.clone(),
            "name": repo_name,
            "commit": commit_hash,
        })),
        Some("custom.repository"),
    );
    block.metadata.label = Some(repo_name.to_string());
    block
        .metadata
        .custom
        .insert(META_NODE_CLASS.to_string(), json!("repository"));
    block.metadata.custom.insert(
        META_LOGICAL_KEY.to_string(),
        json!(format!("repository:{}", repo_name)),
    );
    block
        .metadata
        .custom
        .insert(META_CODEREF.to_string(), coderef);
    block
}

pub(super) fn make_directory_block(path: &str) -> Block {
    let coderef = json!({
        "path": path,
        "display": path,
    });
    let mut block = Block::new(
        Content::json(json!({
            "coderef": coderef.clone(),
        })),
        Some("custom.directory"),
    );
    block.metadata.label = Some(path.to_string());
    block
        .metadata
        .custom
        .insert(META_NODE_CLASS.to_string(), json!("directory"));
    block
        .metadata
        .custom
        .insert(META_CODEREF.to_string(), coderef);
    block.metadata.custom.insert(
        META_LOGICAL_KEY.to_string(),
        json!(format!("directory:{}", path)),
    );
    block
}

pub(super) fn make_file_block(path: &str, language: &str, description: Option<&str>) -> Block {
    let coderef = json!({
        "path": path,
        "display": path,
    });
    let mut content = serde_json::Map::new();
    content.insert("coderef".to_string(), coderef.clone());
    content.insert("language".to_string(), json!(language));
    if let Some(description) = description {
        content.insert("description".to_string(), json!(description));
    }

    let mut block = Block::new(
        Content::json(serde_json::Value::Object(content)),
        Some("custom.file"),
    );
    block.metadata.label = Some(path.to_string());
    block.metadata.summary = description.map(|value| value.to_string());
    block
        .metadata
        .custom
        .insert(META_NODE_CLASS.to_string(), json!("file"));
    block
        .metadata
        .custom
        .insert(META_CODEREF.to_string(), coderef);
    block
        .metadata
        .custom
        .insert(META_LANGUAGE.to_string(), json!(language));
    block.metadata.custom.insert(
        META_LOGICAL_KEY.to_string(),
        json!(format!("file:{}", path)),
    );
    block
}

pub(super) fn make_symbol_block(
    logical_key: &str,
    path: &str,
    language: &str,
    symbol: &ExtractedSymbol,
) -> Block {
    let line_range = format_line_range(symbol.start_line, symbol.end_line);
    let coderef = json!({
        "path": path,
        "start_line": symbol.start_line,
        "start_col": symbol.start_col,
        "end_line": symbol.end_line,
        "end_col": symbol.end_col,
        "display": format_coderef(path, &line_range),
    });

    let mut content = serde_json::Map::new();
    content.insert("name".to_string(), json!(symbol.name));
    content.insert("kind".to_string(), json!(symbol.kind));
    content.insert("coderef".to_string(), coderef.clone());
    content.insert("exported".to_string(), json!(symbol.exported));
    if let Some(description) = &symbol.description {
        content.insert("description".to_string(), json!(description));
    }
    if !symbol.modifiers.is_empty() {
        content.insert("modifiers".to_string(), json!(symbol.modifiers));
    }
    if !symbol.inputs.is_empty() {
        content.insert("inputs".to_string(), json!(symbol.inputs));
    }
    if let Some(output) = &symbol.output {
        content.insert("output".to_string(), json!(output));
    }
    if let Some(type_info) = &symbol.type_info {
        content.insert("type".to_string(), json!(type_info));
    }

    let mut block = Block::new(
        Content::json(serde_json::Value::Object(content)),
        Some("custom.symbol"),
    );

    block.metadata.label = Some(symbol.name.clone());
    block.metadata.summary = symbol.description.clone();
    block
        .metadata
        .custom
        .insert(META_NODE_CLASS.to_string(), json!("symbol"));
    block
        .metadata
        .custom
        .insert(META_LOGICAL_KEY.to_string(), json!(logical_key));
    block
        .metadata
        .custom
        .insert(META_CODEREF.to_string(), coderef);
    block
        .metadata
        .custom
        .insert(META_LANGUAGE.to_string(), json!(language));
    block
        .metadata
        .custom
        .insert(META_SYMBOL_KIND.to_string(), json!(symbol.kind));
    block
        .metadata
        .custom
        .insert(META_SYMBOL_NAME.to_string(), json!(symbol.name));
    block
        .metadata
        .custom
        .insert(META_EXPORTED.to_string(), json!(symbol.exported));
    block
}
