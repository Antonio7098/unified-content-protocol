use anyhow::{Context, Result};
use regex::Regex;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;
use ucm_core::BlockId;

use crate::model::*;

pub(super) fn collect_repository_files(
    root: &Path,
    config: &CodeGraphExtractorConfig,
    matcher: &GitignoreMatcher,
    diagnostics: &mut Vec<CodeGraphDiagnostic>,
) -> Result<Vec<RepoFile>> {
    let include_exts: HashSet<String> = config
        .include_extensions
        .iter()
        .map(|ext| ext.trim_start_matches('.').to_ascii_lowercase())
        .collect();

    let exclude_dirs: HashSet<String> = config.exclude_dirs.iter().cloned().collect();

    let mut out = Vec::new();
    collect_repository_files_recursive(
        root,
        root,
        &include_exts,
        &exclude_dirs,
        config,
        matcher,
        diagnostics,
        &mut out,
    )?;

    out.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    Ok(out)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn collect_repository_files_recursive(
    root: &Path,
    current: &Path,
    include_exts: &HashSet<String>,
    exclude_dirs: &HashSet<String>,
    config: &CodeGraphExtractorConfig,
    matcher: &GitignoreMatcher,
    diagnostics: &mut Vec<CodeGraphDiagnostic>,
    out: &mut Vec<RepoFile>,
) -> Result<()> {
    let read_dir = match fs::read_dir(current) {
        Ok(rd) => rd,
        Err(err) => {
            diagnostics.push(CodeGraphDiagnostic::warning(
                "CG2004",
                format!("failed to read directory {}: {}", current.display(), err),
            ));
            return Ok(());
        }
    };

    let mut entries = Vec::new();
    for entry in read_dir {
        match entry {
            Ok(e) => entries.push(e),
            Err(err) => diagnostics.push(CodeGraphDiagnostic::warning(
                "CG2005",
                format!("failed to access directory entry: {}", err),
            )),
        }
    }

    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let path = entry.path();
        let rel = normalize_path(
            path.strip_prefix(root)
                .with_context(|| format!("failed to strip prefix {}", root.display()))?,
        );

        if rel.is_empty() {
            continue;
        }

        let file_type = match entry.file_type() {
            Ok(ft) => ft,
            Err(err) => {
                diagnostics.push(CodeGraphDiagnostic::warning(
                    "CG2005",
                    format!("failed to read file type for {}: {}", rel, err),
                ));
                continue;
            }
        };

        if !config.include_hidden && is_hidden_path(&rel) {
            continue;
        }

        if file_type.is_dir() {
            let dir_name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();

            if exclude_dirs.contains(&dir_name) || matcher.is_ignored(&rel, true) {
                continue;
            }

            collect_repository_files_recursive(
                root,
                &path,
                include_exts,
                exclude_dirs,
                config,
                matcher,
                diagnostics,
                out,
            )?;
            continue;
        }

        if !file_type.is_file() {
            continue;
        }

        if matcher.is_ignored(&rel, false) {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase())
            .unwrap_or_default();

        if !include_exts.contains(&ext) {
            continue;
        }

        if let Some(language) = extension_language(&ext) {
            out.push(RepoFile {
                absolute_path: path,
                relative_path: rel,
                language,
            });
        } else {
            diagnostics.push(
                CodeGraphDiagnostic::info("CG2007", format!("unsupported extension '.{}'", ext))
                    .with_path(rel),
            );
        }
    }

    Ok(())
}

pub(super) fn extension_language(ext: &str) -> Option<CodeLanguage> {
    match ext {
        "rs" => Some(CodeLanguage::Rust),
        "py" => Some(CodeLanguage::Python),
        "ts" | "tsx" => Some(CodeLanguage::TypeScript),
        "js" | "jsx" => Some(CodeLanguage::JavaScript),
        _ => None,
    }
}

pub(super) fn unique_symbol_logical_key(
    file_path: &str,
    symbol_name: &str,
    line: usize,
    used: &mut HashSet<String>,
) -> String {
    let base = format!("symbol:{}::{}", file_path, symbol_name);
    if used.insert(base.clone()) {
        return base;
    }

    let with_line = format!("{}#{}", base, line);
    if used.insert(with_line.clone()) {
        return with_line;
    }

    let mut n = 2usize;
    loop {
        let candidate = format!("{}#{}", with_line, n);
        if used.insert(candidate.clone()) {
            return candidate;
        }
        n += 1;
    }
}

pub(super) fn ancestor_directories(path: &str) -> Vec<String> {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() <= 1 {
        return Vec::new();
    }

    let mut dirs = Vec::new();
    for i in 1..parts.len() {
        let dir = parts[..i].join("/");
        if !dir.is_empty() {
            dirs.push(dir);
        }
    }
    dirs
}

pub(super) fn parent_directory_id(
    dir: &str,
    directory_ids: &BTreeMap<String, BlockId>,
) -> Option<BlockId> {
    let parent = parent_directory(dir);
    if parent.is_empty() {
        None
    } else {
        directory_ids.get(&parent).copied()
    }
}

pub(super) fn parent_id_for_file(
    path: &str,
    repo_id: BlockId,
    directory_ids: &BTreeMap<String, BlockId>,
) -> BlockId {
    let parent_dir = parent_directory(path);
    if parent_dir.is_empty() {
        repo_id
    } else {
        directory_ids.get(&parent_dir).copied().unwrap_or(repo_id)
    }
}

pub(super) fn parent_directory(path: &str) -> String {
    match path.rsplit_once('/') {
        Some((parent, _)) => parent.to_string(),
        None => String::new(),
    }
}

pub(super) fn normalize_relative_join(base: &str, relative: &str) -> String {
    let mut segments = Vec::new();

    if !base.is_empty() {
        segments.extend(
            base.split('/')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string()),
        );
    }

    for part in relative.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                segments.pop();
            }
            other => segments.push(other.to_string()),
        }
    }

    segments.join("/")
}

pub(super) fn ascend_directory(path: &str, levels: usize) -> String {
    let mut parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    for _ in 0..levels {
        if parts.is_empty() {
            break;
        }
        parts.pop();
    }
    parts.join("/")
}

pub(super) fn sanitize_identifier(raw: &str) -> String {
    raw.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

pub(super) fn normalize_path(path: &Path) -> String {
    path.components()
        .filter_map(|component| {
            let s = component.as_os_str().to_string_lossy();
            if s == "." || s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

pub(super) fn is_hidden_path(path: &str) -> bool {
    path.split('/').any(|part| part.starts_with('.'))
}

#[derive(Debug, Clone)]
pub(super) struct GitignoreMatcher {
    rules: Vec<GitignoreRule>,
}

#[derive(Debug, Clone)]
pub(super) struct GitignoreRule {
    pub(super) regex: Regex,
    pub(super) directory_only: bool,
}

impl GitignoreMatcher {
    pub(super) fn from_repository(repo_root: &Path) -> Result<Self> {
        let gitignore_path = repo_root.join(".gitignore");
        if !gitignore_path.exists() {
            return Ok(Self { rules: Vec::new() });
        }

        let raw = fs::read_to_string(&gitignore_path)
            .with_context(|| format!("failed to read {}", gitignore_path.display()))?;

        let mut rules = Vec::new();
        for line in raw.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('!') {
                continue;
            }

            if let Some(rule) = GitignoreRule::from_pattern(trimmed) {
                rules.push(rule);
            }
        }

        Ok(Self { rules })
    }

    pub(super) fn is_ignored(&self, rel_path: &str, is_dir: bool) -> bool {
        for rule in &self.rules {
            if rule.directory_only && !is_dir {
                continue;
            }
            if rule.regex.is_match(rel_path) {
                return true;
            }
        }
        false
    }
}

impl GitignoreRule {
    pub(super) fn from_pattern(pattern: &str) -> Option<Self> {
        let directory_only = pattern.ends_with('/');
        let mut core = pattern.trim_end_matches('/').trim_start_matches("./");

        if core.is_empty() {
            return None;
        }

        let anchored = core.starts_with('/');
        core = core.trim_start_matches('/');

        let mut regex = String::new();
        if anchored {
            regex.push('^');
        } else {
            regex.push_str("(^|.*/)");
        }

        regex.push_str(&glob_to_regex(core));

        if directory_only {
            regex.push_str("($|/.*)");
        } else {
            regex.push('$');
        }

        let compiled = Regex::new(&regex).ok()?;

        Some(Self {
            regex: compiled,
            directory_only,
        })
    }
}

pub(super) fn glob_to_regex(glob: &str) -> String {
    let mut out = String::new();
    let mut chars = glob.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '*' => {
                if matches!(chars.peek(), Some('*')) {
                    chars.next();
                    out.push_str(".*");
                } else {
                    out.push_str("[^/]*");
                }
            }
            '?' => out.push_str("[^/]"),
            '.' | '+' | '(' | ')' | '|' | '^' | '$' | '{' | '}' | '[' | ']' | '\\' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }

    out
}
