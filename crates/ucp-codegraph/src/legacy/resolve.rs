use std::collections::{BTreeMap, BTreeSet};

use ucm_core::BlockId;

use crate::model::*;

use super::languages::rust::rust_last_path_segment;
use super::{ascend_directory, normalize_relative_join, parent_directory};

pub(super) fn resolve_relationship_target_ids(
    source_file: &str,
    language: CodeLanguage,
    relationship: &ExtractedRelationship,
    top_level_symbol_ids: &BTreeMap<(String, String), Vec<BlockId>>,
    imported_symbol_targets_by_file: &BTreeMap<String, BTreeMap<String, Vec<BlockId>>>,
    known_files: &BTreeSet<String>,
) -> Vec<BlockId> {
    let mut target_ids = Vec::new();

    if let Some(local_ids) =
        top_level_symbol_ids.get(&(source_file.to_string(), relationship.target_name.clone()))
    {
        target_ids.extend(local_ids.iter().copied());
    }

    if let Some(imported_ids) = imported_symbol_targets_by_file
        .get(source_file)
        .and_then(|bindings| bindings.get(&relationship.target_name))
    {
        target_ids.extend(imported_ids.iter().copied());
    }

    if language == CodeLanguage::Rust && relationship.target_expr.contains("::") {
        if let ImportResolution::Resolved(target_file) = resolve_import(
            source_file,
            &language,
            &relationship.target_expr,
            known_files,
        ) {
            if let Some(name) = rust_last_path_segment(&relationship.target_expr) {
                if let Some(ids) = top_level_symbol_ids.get(&(target_file, name)) {
                    target_ids.extend(ids.iter().copied());
                }
            }
        }
    }

    let mut unique_ids = Vec::new();
    for target_id in target_ids {
        if !unique_ids.contains(&target_id) {
            unique_ids.push(target_id);
        }
    }
    unique_ids
}

pub(super) fn resolve_alias_target_ids(
    source_file: &str,
    language: CodeLanguage,
    alias: &ExtractedAlias,
    top_level_symbol_ids: &BTreeMap<(String, String), Vec<BlockId>>,
    exported_symbol_targets_by_file: &BTreeMap<String, BTreeMap<String, Vec<BlockId>>>,
    imported_symbol_targets_by_file: &BTreeMap<String, BTreeMap<String, Vec<BlockId>>>,
    imported_module_targets_by_file: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
    imported_module_paths_by_file: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
    alias_names_by_scope: &BTreeMap<(String, String), BTreeSet<String>>,
    aliased_symbol_targets_by_scope: &BTreeMap<(String, String), BTreeMap<String, Vec<BlockId>>>,
    known_files: &BTreeSet<String>,
) -> Vec<BlockId> {
    let mut target_ids = Vec::new();

    if alias.target_expr == alias.target_name {
        let local_scope_key = alias_scope_key(alias.owner_identity.as_deref());
        if let Some(alias_ids) = aliased_symbol_targets_by_scope
            .get(&(source_file.to_string(), local_scope_key.clone()))
            .and_then(|aliases| aliases.get(&alias.target_name))
        {
            return alias_ids.clone();
        }
        if !local_scope_key.is_empty()
            && alias_names_by_scope
                .get(&(source_file.to_string(), local_scope_key))
                .is_some_and(|aliases| aliases.contains(&alias.target_name))
        {
            return Vec::new();
        }

        let top_scope_key = alias_scope_key(None);
        if let Some(alias_ids) = aliased_symbol_targets_by_scope
            .get(&(source_file.to_string(), top_scope_key.clone()))
            .and_then(|aliases| aliases.get(&alias.target_name))
        {
            return alias_ids.clone();
        }
        if alias_names_by_scope
            .get(&(source_file.to_string(), top_scope_key))
            .is_some_and(|aliases| aliases.contains(&alias.target_name))
        {
            return Vec::new();
        }
    }

    if let Some(local_ids) =
        top_level_symbol_ids.get(&(source_file.to_string(), alias.target_name.clone()))
    {
        target_ids.extend(local_ids.iter().copied());
    }

    if let Some(imported_ids) = imported_symbol_targets_by_file
        .get(source_file)
        .and_then(|bindings| bindings.get(&alias.target_name))
    {
        target_ids.extend(imported_ids.iter().copied());
    }

    if let Some((module_alias, member_name)) = member_usage_parts(&alias.target_expr) {
        if let Some(target_files) = imported_module_targets_by_file
            .get(source_file)
            .and_then(|aliases| aliases.get(&module_alias))
        {
            for target_file in target_files {
                if let Some(ids) = exported_symbol_targets_by_file
                    .get(target_file)
                    .and_then(|exports| exports.get(&member_name))
                {
                    target_ids.extend(ids.iter().copied());
                }
            }
        }
    }

    if language == CodeLanguage::Rust && alias.target_expr.contains("::") {
        if let Some((module_alias, remainder)) = rust_alias_path_parts(&alias.target_expr) {
            if let Some(module_paths) = imported_module_paths_by_file
                .get(source_file)
                .and_then(|aliases| aliases.get(&module_alias))
            {
                for module_path in module_paths {
                    let expanded_expr = format!("{module_path}::{remainder}");
                    if let ImportResolution::Resolved(target_file) =
                        resolve_import(source_file, &language, &expanded_expr, known_files)
                    {
                        if let Some(name) = rust_last_path_segment(&expanded_expr) {
                            if let Some(ids) = top_level_symbol_ids.get(&(target_file, name)) {
                                target_ids.extend(ids.iter().copied());
                            }
                        }
                    }
                }
            }
        }

        if let ImportResolution::Resolved(target_file) =
            resolve_import(source_file, &language, &alias.target_expr, known_files)
        {
            if let Some(name) = rust_last_path_segment(&alias.target_expr) {
                if let Some(ids) = top_level_symbol_ids.get(&(target_file, name)) {
                    target_ids.extend(ids.iter().copied());
                }
            }
        }
    }

    let mut unique_ids = Vec::new();
    for target_id in target_ids {
        if !unique_ids.contains(&target_id) {
            unique_ids.push(target_id);
        }
    }
    unique_ids
}

pub(super) fn resolve_usage_target_ids(
    source_file: &str,
    language: CodeLanguage,
    usage: &ExtractedUsage,
    top_level_symbol_ids: &BTreeMap<(String, String), Vec<BlockId>>,
    exported_symbol_targets_by_file: &BTreeMap<String, BTreeMap<String, Vec<BlockId>>>,
    imported_symbol_targets_by_file: &BTreeMap<String, BTreeMap<String, Vec<BlockId>>>,
    imported_module_targets_by_file: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
    imported_module_paths_by_file: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
    alias_names_by_scope: &BTreeMap<(String, String), BTreeSet<String>>,
    alias_records_by_scope: &BTreeMap<(String, String), BTreeMap<String, Vec<ExtractedAlias>>>,
    aliased_symbol_targets_by_scope: &BTreeMap<(String, String), BTreeMap<String, Vec<BlockId>>>,
    known_files: &BTreeSet<String>,
) -> Vec<BlockId> {
    if usage.target_expr == usage.target_name {
        let local_scope_key = alias_scope_key(Some(&usage.source_identity));
        if let Some(alias_ids) = aliased_symbol_targets_by_scope
            .get(&(source_file.to_string(), local_scope_key.clone()))
            .and_then(|aliases| aliases.get(&usage.target_name))
        {
            return alias_ids.clone();
        }
        if alias_names_by_scope
            .get(&(source_file.to_string(), local_scope_key.clone()))
            .is_some_and(|aliases| aliases.contains(&usage.target_name))
        {
            return Vec::new();
        }

        let top_scope_key = alias_scope_key(None);
        if let Some(alias_ids) = aliased_symbol_targets_by_scope
            .get(&(source_file.to_string(), top_scope_key.clone()))
            .and_then(|aliases| aliases.get(&usage.target_name))
        {
            return alias_ids.clone();
        }
        if alias_names_by_scope
            .get(&(source_file.to_string(), top_scope_key))
            .is_some_and(|aliases| aliases.contains(&usage.target_name))
        {
            return Vec::new();
        }
    }

    let mut target_ids = Vec::new();

    if let Some(local_ids) =
        top_level_symbol_ids.get(&(source_file.to_string(), usage.target_name.clone()))
    {
        target_ids.extend(local_ids.iter().copied());
    }

    if let Some(imported_ids) = imported_symbol_targets_by_file
        .get(source_file)
        .and_then(|bindings| bindings.get(&usage.target_name))
    {
        target_ids.extend(imported_ids.iter().copied());
    }

    if let Some((module_alias, member_name)) = member_usage_parts(&usage.target_expr) {
        let target_files = resolve_module_alias_target_files(
            source_file,
            Some(&usage.source_identity),
            &module_alias,
            imported_module_targets_by_file,
            alias_names_by_scope,
            alias_records_by_scope,
        );
        for target_file in target_files {
            if let Some(ids) = exported_symbol_targets_by_file
                .get(&target_file)
                .and_then(|exports| exports.get(&member_name))
            {
                target_ids.extend(ids.iter().copied());
            }
        }
    }

    if language == CodeLanguage::Rust && usage.target_expr.contains("::") {
        if let Some((module_alias, remainder)) = rust_alias_path_parts(&usage.target_expr) {
            if let Some(module_paths) = imported_module_paths_by_file
                .get(source_file)
                .and_then(|aliases| aliases.get(&module_alias))
            {
                for module_path in module_paths {
                    let expanded_expr = format!("{module_path}::{remainder}");
                    if let ImportResolution::Resolved(target_file) =
                        resolve_import(source_file, &language, &expanded_expr, known_files)
                    {
                        if let Some(name) = rust_last_path_segment(&expanded_expr) {
                            if let Some(ids) = top_level_symbol_ids.get(&(target_file, name)) {
                                target_ids.extend(ids.iter().copied());
                            }
                        }
                    }
                }
            }
        }

        if let ImportResolution::Resolved(target_file) =
            resolve_import(source_file, &language, &usage.target_expr, known_files)
        {
            if let Some(name) = rust_last_path_segment(&usage.target_expr) {
                if let Some(ids) = top_level_symbol_ids.get(&(target_file, name)) {
                    target_ids.extend(ids.iter().copied());
                }
            }
        }
    }

    let mut unique_ids = Vec::new();
    for target_id in target_ids {
        if !unique_ids.contains(&target_id) {
            unique_ids.push(target_id);
        }
    }
    unique_ids
}

pub(super) fn member_usage_parts(text: &str) -> Option<(String, String)> {
    let (left, right) = text.split_once('.')?;
    let left = left.trim();
    let right = right.trim();
    if left.is_empty() || right.is_empty() {
        None
    } else {
        Some((left.to_string(), right.to_string()))
    }
}

pub(super) fn resolve_module_alias_target_files(
    source_file: &str,
    owner_identity: Option<&str>,
    alias_name: &str,
    imported_module_targets_by_file: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
    alias_names_by_scope: &BTreeMap<(String, String), BTreeSet<String>>,
    alias_records_by_scope: &BTreeMap<(String, String), BTreeMap<String, Vec<ExtractedAlias>>>,
) -> Vec<String> {
    let mut visited = BTreeSet::new();
    resolve_module_alias_target_files_recursive(
        source_file,
        owner_identity,
        alias_name,
        imported_module_targets_by_file,
        alias_names_by_scope,
        alias_records_by_scope,
        &mut visited,
    )
}

pub(super) fn resolve_module_alias_target_files_recursive(
    source_file: &str,
    owner_identity: Option<&str>,
    alias_name: &str,
    imported_module_targets_by_file: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
    alias_names_by_scope: &BTreeMap<(String, String), BTreeSet<String>>,
    alias_records_by_scope: &BTreeMap<(String, String), BTreeMap<String, Vec<ExtractedAlias>>>,
    visited: &mut BTreeSet<(String, String, String)>,
) -> Vec<String> {
    let visit_key = (
        source_file.to_string(),
        alias_scope_key(owner_identity),
        alias_name.to_string(),
    );
    if !visited.insert(visit_key) {
        return Vec::new();
    }

    if let Some(target_files) = imported_module_targets_by_file
        .get(source_file)
        .and_then(|aliases| aliases.get(alias_name))
    {
        return target_files.clone();
    }

    for scope_key in [alias_scope_key(owner_identity), alias_scope_key(None)] {
        let scope_identity = (source_file.to_string(), scope_key.clone());
        if let Some(alias_entries) = alias_records_by_scope
            .get(&scope_identity)
            .and_then(|aliases| aliases.get(alias_name))
        {
            if alias_entries.len() != 1 {
                return Vec::new();
            }
            let alias = &alias_entries[0];
            if alias.target_expr != alias.target_name || alias.target_expr.is_empty() {
                return Vec::new();
            }
            return resolve_module_alias_target_files_recursive(
                source_file,
                alias.owner_identity.as_deref(),
                &alias.target_name,
                imported_module_targets_by_file,
                alias_names_by_scope,
                alias_records_by_scope,
                visited,
            );
        }
        if alias_names_by_scope
            .get(&scope_identity)
            .is_some_and(|aliases| aliases.contains(alias_name))
        {
            return Vec::new();
        }
        if scope_key.is_empty() {
            break;
        }
    }

    Vec::new()
}

pub(super) fn rust_alias_path_parts(text: &str) -> Option<(String, String)> {
    let (left, right) = text.split_once("::")?;
    let left = left.trim();
    let right = right.trim();
    if left.is_empty() || right.is_empty() {
        None
    } else {
        Some((left.to_string(), right.to_string()))
    }
}

pub(super) fn alias_scope_key(owner_identity: Option<&str>) -> String {
    owner_identity.unwrap_or("").to_string()
}

pub(super) fn resolve_import(
    source_file: &str,
    language: &CodeLanguage,
    module: &str,
    known_files: &BTreeSet<String>,
) -> ImportResolution {
    match language {
        CodeLanguage::Rust => resolve_rust_import(source_file, module, known_files),
        CodeLanguage::Python => resolve_python_import(source_file, module, known_files),
        CodeLanguage::TypeScript | CodeLanguage::JavaScript => {
            resolve_ts_import(source_file, module, known_files)
        }
    }
}

pub(super) fn resolve_ts_import(
    source_file: &str,
    module: &str,
    known_files: &BTreeSet<String>,
) -> ImportResolution {
    if !module.starts_with('.') {
        return ImportResolution::External;
    }

    let source_dir = parent_directory(source_file);
    let joined = normalize_relative_join(&source_dir, module);

    find_known_candidate(ts_candidates(&joined), known_files)
        .map(ImportResolution::Resolved)
        .unwrap_or(ImportResolution::Unresolved)
}

pub(super) fn ts_candidates(base: &str) -> Vec<String> {
    let exts = ["ts", "tsx", "js", "jsx"];
    let mut out = Vec::new();

    if has_known_extension(base, &exts) {
        out.push(base.to_string());
    } else {
        for ext in exts {
            out.push(format!("{}.{}", base, ext));
        }
        for ext in exts {
            out.push(format!("{}/index.{}", base, ext));
        }
    }

    out
}

pub(super) fn resolve_python_import(
    source_file: &str,
    module: &str,
    known_files: &BTreeSet<String>,
) -> ImportResolution {
    let source_dir = parent_directory(source_file);
    let mut dots = 0usize;
    for ch in module.chars() {
        if ch == '.' {
            dots += 1;
        } else {
            break;
        }
    }

    let module_tail = module.trim_start_matches('.');

    let base_dir = if dots > 0 {
        ascend_directory(&source_dir, dots.saturating_sub(1))
    } else {
        String::new()
    };

    let module_path = module_tail.replace('.', "/");

    let joined = if base_dir.is_empty() {
        module_path
    } else if module_path.is_empty() {
        base_dir
    } else {
        format!("{}/{}", base_dir, module_path)
    };

    match find_known_candidate(py_candidates(&joined), known_files) {
        Some(candidate) => ImportResolution::Resolved(candidate),
        None if dots == 0 => ImportResolution::External,
        None => ImportResolution::Unresolved,
    }
}

pub(super) fn py_candidates(base: &str) -> Vec<String> {
    if base.is_empty() {
        return Vec::new();
    }

    if base.ends_with(".py") {
        return vec![base.to_string()];
    }

    vec![format!("{}.py", base), format!("{}/__init__.py", base)]
}

pub(super) fn resolve_rust_import(
    source_file: &str,
    module: &str,
    known_files: &BTreeSet<String>,
) -> ImportResolution {
    if module.starts_with("std::") || module.starts_with("core::") || module.starts_with("alloc::")
    {
        return ImportResolution::External;
    }

    if let Some(name) = module.strip_prefix("mod:") {
        let source_dir = parent_directory(source_file);
        let local = normalize_relative_join(&source_dir, name);
        return find_known_candidate(
            [format!("{}.rs", local), format!("{}/mod.rs", local)],
            known_files,
        )
        .map(ImportResolution::Resolved)
        .unwrap_or(ImportResolution::Unresolved);
    }

    let source_dir = parent_directory(source_file);
    let crate_root = rust_module_root(source_file);
    let explicitly_local = module.starts_with("crate::")
        || module.starts_with("self::")
        || module.starts_with("super::");

    let (base_dir, path_segments) = if let Some(rest) = module.strip_prefix("crate::") {
        (
            crate_root.clone(),
            rest.split("::").map(|s| s.to_string()).collect::<Vec<_>>(),
        )
    } else if let Some(rest) = module.strip_prefix("self::") {
        (
            source_dir.clone(),
            rest.split("::").map(|s| s.to_string()).collect::<Vec<_>>(),
        )
    } else if module.starts_with("super::") {
        let mut rest = module;
        let mut super_count = 0usize;
        while let Some(next) = rest.strip_prefix("super::") {
            super_count += 1;
            rest = next;
        }
        (
            ascend_directory(&source_dir, super_count),
            rest.split("::").map(|s| s.to_string()).collect::<Vec<_>>(),
        )
    } else {
        (
            crate_root.clone(),
            module
                .split("::")
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        )
    };

    if let Some(candidate) = find_known_candidate(
        (1..=path_segments.len()).rev().flat_map(|trimmed| {
            let joined = path_segments[..trimmed].join("/");
            if joined.is_empty() {
                return Vec::new();
            }
            let candidate_base = if base_dir.is_empty() {
                joined
            } else {
                format!("{}/{}", base_dir, joined)
            };
            vec![
                format!("{}.rs", candidate_base),
                format!("{}/mod.rs", candidate_base),
            ]
        }),
        known_files,
    ) {
        return ImportResolution::Resolved(candidate);
    }

    if module.starts_with("crate::") && path_segments.len() == 1 {
        if let Some(entry_file) = rust_crate_entry_file(&crate_root, known_files) {
            return ImportResolution::Resolved(entry_file);
        }
    }

    if explicitly_local {
        return ImportResolution::Unresolved;
    }

    let first_segment = path_segments
        .first()
        .map(|s| s.as_str())
        .unwrap_or_default();
    if rust_root_module_exists(&crate_root, first_segment, known_files) {
        ImportResolution::Unresolved
    } else {
        ImportResolution::External
    }
}

pub(super) fn find_known_candidate<I>(
    candidates: I,
    known_files: &BTreeSet<String>,
) -> Option<String>
where
    I: IntoIterator<Item = String>,
{
    candidates
        .into_iter()
        .find(|candidate| known_files.contains(candidate))
}

pub(super) fn rust_crate_entry_file(
    crate_root: &str,
    known_files: &BTreeSet<String>,
) -> Option<String> {
    find_known_candidate(
        [
            format!("{}/lib.rs", crate_root),
            format!("{}/main.rs", crate_root),
            format!("{}/mod.rs", crate_root),
        ],
        known_files,
    )
}

pub(super) fn rust_module_root(source_file: &str) -> String {
    let parts: Vec<&str> = source_file.split('/').collect();
    if let Some((index, _)) = parts.iter().enumerate().rfind(|(_, part)| **part == "src") {
        return parts[..=index].join("/");
    }

    parent_directory(source_file)
}

pub(super) fn rust_root_module_exists(
    crate_root: &str,
    segment: &str,
    known_files: &BTreeSet<String>,
) -> bool {
    if segment.is_empty() {
        return false;
    }

    [
        format!("{}/{}.rs", crate_root, segment),
        format!("{}/{}/mod.rs", crate_root, segment),
    ]
    .into_iter()
    .any(|candidate| known_files.contains(&candidate))
}

#[cfg(test)]
pub(super) fn expand_rust_use_declaration(text: &str) -> Vec<String> {
    expand_rust_use_declaration_items(text)
        .into_iter()
        .map(|(module, _, _)| module)
        .collect()
}

pub(super) fn expand_rust_use_declaration_items(text: &str) -> Vec<(String, Option<String>, bool)> {
    let trimmed = text.trim();
    let Some(use_index) = trimmed.find("use ") else {
        return Vec::new();
    };

    let expr = trimmed[use_index + 4..].trim().trim_end_matches(';').trim();
    expand_rust_use_tree_items("", expr)
}

pub(super) fn expand_rust_use_tree_items(
    prefix: &str,
    expr: &str,
) -> Vec<(String, Option<String>, bool)> {
    let trimmed = expr.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        return split_top_level_commas(&trimmed[1..trimmed.len() - 1])
            .into_iter()
            .flat_map(|part| expand_rust_use_tree_items(prefix, &part))
            .collect();
    }

    if let Some(open_idx) = find_top_level_char(trimmed, '{') {
        let prefix_part = trimmed[..open_idx].trim().trim_end_matches("::");
        let close_idx = matching_brace_index(trimmed, open_idx).unwrap_or(trimmed.len() - 1);
        let inner = &trimmed[open_idx + 1..close_idx];
        let combined_prefix = join_rust_use_prefix(prefix, prefix_part);
        return split_top_level_commas(inner)
            .into_iter()
            .flat_map(|part| expand_rust_use_tree_items(&combined_prefix, &part))
            .collect();
    }

    let wildcard = trimmed == "*" || trimmed.ends_with("::*");
    let (raw_segment, local_alias) = trimmed
        .rsplit_once(" as ")
        .map(|(left, right)| (left.trim(), Some(right.trim().to_string())))
        .unwrap_or((trimmed, None));
    let segment = raw_segment
        .trim_end_matches("::*")
        .trim_start_matches("::")
        .trim();
    if segment.is_empty() {
        return Vec::new();
    }

    if segment == "self" || segment == "*" {
        return if prefix.is_empty() {
            Vec::new()
        } else {
            vec![(prefix.to_string(), local_alias, wildcard || segment == "*")]
        };
    }

    vec![(join_rust_use_prefix(prefix, segment), local_alias, wildcard)]
}

pub(super) fn split_top_level_commas(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut start = 0usize;

    for (index, ch) in input.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                let part = input[start..index].trim();
                if !part.is_empty() {
                    out.push(part.to_string());
                }
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }

    let tail = input[start..].trim();
    if !tail.is_empty() {
        out.push(tail.to_string());
    }

    out
}

pub(super) fn find_top_level_char(input: &str, needle: char) -> Option<usize> {
    let mut depth = 0usize;
    for (index, ch) in input.char_indices() {
        match ch {
            '{' if ch == needle && depth == 0 => return Some(index),
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    None
}

pub(super) fn matching_brace_index(input: &str, open_idx: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (index, ch) in input.char_indices().skip_while(|(idx, _)| *idx < open_idx) {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

pub(super) fn join_rust_use_prefix(prefix: &str, segment: &str) -> String {
    let clean_prefix = prefix
        .trim()
        .trim_end_matches("::")
        .trim_start_matches("::");
    let clean_segment = segment
        .trim()
        .trim_end_matches("::")
        .trim_start_matches("::");

    if clean_prefix.is_empty() {
        clean_segment.to_string()
    } else if clean_segment.is_empty() {
        clean_prefix.to_string()
    } else {
        format!("{}::{}", clean_prefix, clean_segment)
    }
}

pub(super) fn has_known_extension(path: &str, exts: &[&str]) -> bool {
    exts.iter().any(|ext| path.ends_with(&format!(".{}", ext)))
}
