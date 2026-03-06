use anyhow::{anyhow, Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tree_sitter::{Language, Node, Parser};
use ucm_core::{
    normalize::{canonical_json, normalize_content},
    Block, BlockId, Content, Document, DocumentId, DocumentMetadata, Edge, EdgeType,
};
use ucp_llm::IdMapper;

pub const CODEGRAPH_PROFILE: &str = "codegraph";
pub const CODEGRAPH_PROFILE_VERSION: &str = "v1";
pub const CODEGRAPH_PROFILE_MARKER: &str = "codegraph.v1";
pub const CODEGRAPH_EXTRACTOR_VERSION: &str = "ucp-codegraph-extractor.v1";

const META_NODE_CLASS: &str = "node_class";
const META_LOGICAL_KEY: &str = "logical_key";
const META_PATH: &str = "path";
const META_LANGUAGE: &str = "language";
const META_SYMBOL_KIND: &str = "symbol_kind";
const META_SYMBOL_NAME: &str = "name";
const META_SPAN: &str = "span";
const META_EXPORTED: &str = "exported";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeGraphSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeGraphDiagnostic {
    pub severity: CodeGraphSeverity,
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_key: Option<String>,
}

impl CodeGraphDiagnostic {
    fn error(code: &str, message: impl Into<String>) -> Self {
        Self {
            severity: CodeGraphSeverity::Error,
            code: code.to_string(),
            message: message.into(),
            path: None,
            logical_key: None,
        }
    }

    fn warning(code: &str, message: impl Into<String>) -> Self {
        Self {
            severity: CodeGraphSeverity::Warning,
            code: code.to_string(),
            message: message.into(),
            path: None,
            logical_key: None,
        }
    }

    fn info(code: &str, message: impl Into<String>) -> Self {
        Self {
            severity: CodeGraphSeverity::Info,
            code: code.to_string(),
            message: message.into(),
            path: None,
            logical_key: None,
        }
    }

    fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    fn with_logical_key(mut self, logical_key: impl Into<String>) -> Self {
        self.logical_key = Some(logical_key.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CodeGraphValidationResult {
    pub valid: bool,
    pub diagnostics: Vec<CodeGraphDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CodeGraphStats {
    pub total_nodes: usize,
    pub repository_nodes: usize,
    pub directory_nodes: usize,
    pub file_nodes: usize,
    pub symbol_nodes: usize,
    pub total_edges: usize,
    pub reference_edges: usize,
    pub export_edges: usize,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub languages: BTreeMap<String, usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeGraphBuildStatus {
    Success,
    PartialSuccess,
    FailedValidation,
}

#[derive(Debug, Clone)]
pub struct CodeGraphBuildResult {
    pub document: Document,
    pub diagnostics: Vec<CodeGraphDiagnostic>,
    pub stats: CodeGraphStats,
    pub profile_version: String,
    pub canonical_fingerprint: String,
    pub status: CodeGraphBuildStatus,
}

impl CodeGraphBuildResult {
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == CodeGraphSeverity::Error)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphBuildInput {
    pub repository_path: PathBuf,
    pub commit_hash: String,
    #[serde(default)]
    pub config: CodeGraphExtractorConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphExtractorConfig {
    #[serde(default = "default_include_extensions")]
    pub include_extensions: Vec<String>,
    #[serde(default = "default_exclude_dirs")]
    pub exclude_dirs: Vec<String>,
    #[serde(default = "default_continue_on_parse_error")]
    pub continue_on_parse_error: bool,
    #[serde(default)]
    pub include_hidden: bool,
    #[serde(default = "default_max_file_bytes")]
    pub max_file_bytes: usize,
    #[serde(default = "default_emit_export_edges")]
    pub emit_export_edges: bool,
}

impl Default for CodeGraphExtractorConfig {
    fn default() -> Self {
        Self {
            include_extensions: default_include_extensions(),
            exclude_dirs: default_exclude_dirs(),
            continue_on_parse_error: default_continue_on_parse_error(),
            include_hidden: false,
            max_file_bytes: default_max_file_bytes(),
            emit_export_edges: default_emit_export_edges(),
        }
    }
}

fn default_include_extensions() -> Vec<String> {
    vec!["rs", "py", "ts", "tsx", "js", "jsx"]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

fn default_exclude_dirs() -> Vec<String> {
    vec![".git", "target", "node_modules", "dist", "build"]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

fn default_continue_on_parse_error() -> bool {
    true
}

fn default_max_file_bytes() -> usize {
    2 * 1024 * 1024
}

fn default_emit_export_edges() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortableDocument {
    pub id: String,
    pub root: String,
    pub structure: BTreeMap<String, Vec<String>>,
    pub blocks: BTreeMap<String, Block>,
    pub metadata: DocumentMetadata,
    pub version: u64,
}

impl PortableDocument {
    pub fn from_document(doc: &Document) -> Self {
        let mut structure = BTreeMap::new();
        for (parent, children) in &doc.structure {
            let mut sorted = children.clone();
            sorted.sort_by_key(|id| id.to_string());
            structure.insert(
                parent.to_string(),
                sorted.into_iter().map(|id| id.to_string()).collect(),
            );
        }

        let mut blocks = BTreeMap::new();
        for (id, block) in &doc.blocks {
            blocks.insert(id.to_string(), block.clone());
        }

        Self {
            id: doc.id.0.clone(),
            root: doc.root.to_string(),
            structure,
            blocks,
            metadata: doc.metadata.clone(),
            version: doc.version.counter,
        }
    }

    pub fn to_document(&self) -> Result<Document> {
        let root = BlockId::from_str(&self.root)
            .map_err(|_| anyhow!("invalid root block id: {}", self.root))?;

        let mut structure: HashMap<BlockId, Vec<BlockId>> = HashMap::new();
        for (parent, children) in &self.structure {
            let parent_id = BlockId::from_str(parent)
                .map_err(|_| anyhow!("invalid structure parent id: {}", parent))?;
            let mut parsed_children = Vec::with_capacity(children.len());
            for child in children {
                let child_id = BlockId::from_str(child)
                    .map_err(|_| anyhow!("invalid structure child id: {}", child))?;
                parsed_children.push(child_id);
            }
            structure.insert(parent_id, parsed_children);
        }

        let mut blocks: HashMap<BlockId, Block> = HashMap::new();
        for (id, block) in &self.blocks {
            let block_id = BlockId::from_str(id)
                .map_err(|_| anyhow!("invalid block id in blocks map: {}", id))?;
            blocks.insert(block_id, block.clone());
        }

        let mut doc = Document {
            id: DocumentId::new(self.id.clone()),
            root,
            structure,
            blocks,
            metadata: self.metadata.clone(),
            indices: Default::default(),
            edge_index: Default::default(),
            version: ucm_core::DocumentVersion {
                counter: self.version,
                timestamp: deterministic_timestamp(),
                state_hash: [0u8; 8],
            },
        };
        doc.rebuild_indices();
        Ok(doc)
    }
}

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

        let file_block = make_file_block(&file.relative_path, file.language.as_str());
        let file_block_id = doc.add_block(file_block, &parent_id)?;
        file_ids.insert(file.relative_path.clone(), file_block_id);

        let FileAnalysis {
            mut symbols,
            imports,
            relationships,
            usages,
            aliases,
            default_exported_symbol_names,
            diagnostics: analysis_diagnostics,
            ..
        } = analyze_file(&file.relative_path, &source, file.language);
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
    let mut alias_records_by_scope: BTreeMap<(String, String), BTreeMap<String, Vec<ExtractedAlias>>> =
        BTreeMap::new();
    let mut aliased_symbol_targets_by_scope: BTreeMap<(String, String), BTreeMap<String, Vec<BlockId>>> =
        BTreeMap::new();
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
        let entry = exported_symbol_targets_by_file.entry(file.clone()).or_default();
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

                let ImportResolution::Resolved(target) = resolve_import(
                    &record.file,
                    &record.language,
                    &import.module,
                    &known_files,
                ) else {
                    continue;
                };

                let target_exports = exported_symbol_targets_by_file
                    .get(&target)
                    .cloned()
                    .unwrap_or_default();
                let entry = exported_symbol_targets_by_file.entry(record.file.clone()).or_default();

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
            match resolve_import(
                &record.file,
                &record.language,
                &import.module,
                &known_files,
            ) {
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
        let Some(target_symbol_ids) = top_level_symbol_ids.get(&(target_path.clone(), symbol_name.clone()))
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
            let mut edge = Edge::new(EdgeType::Custom("imports_symbol".to_string()), *target_symbol_id);
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
                let mut edge = Edge::new(EdgeType::Custom("exports".to_string()), *target_symbol_id);
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

        for (source_path, target_path, raw_import, filter_names) in pending_wildcard_reexport_edges {
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
                let mut edge = Edge::new(EdgeType::Custom("exports".to_string()), *target_symbol_id);
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
    })
}

pub fn validate_code_graph_profile(doc: &Document) -> CodeGraphValidationResult {
    let mut diagnostics = Vec::new();

    match doc.metadata.custom.get("profile").and_then(|v| v.as_str()) {
        Some(CODEGRAPH_PROFILE) => {}
        Some(other) => diagnostics.push(CodeGraphDiagnostic::error(
            "CG1001",
            format!(
                "invalid profile marker '{}', expected '{}'",
                other, CODEGRAPH_PROFILE
            ),
        )),
        None => diagnostics.push(CodeGraphDiagnostic::error(
            "CG1001",
            "missing document metadata.custom.profile marker",
        )),
    }

    match doc
        .metadata
        .custom
        .get("profile_version")
        .and_then(|v| v.as_str())
    {
        Some(CODEGRAPH_PROFILE_VERSION) => {}
        Some(other) => diagnostics.push(CodeGraphDiagnostic::error(
            "CG1002",
            format!(
                "invalid profile version '{}', expected '{}'",
                other, CODEGRAPH_PROFILE_VERSION
            ),
        )),
        None => diagnostics.push(CodeGraphDiagnostic::error(
            "CG1002",
            "missing document metadata.custom.profile_version marker",
        )),
    }

    let mut logical_keys: HashMap<String, Vec<BlockId>> = HashMap::new();
    let mut class_counts: HashMap<String, usize> = HashMap::new();

    for (id, block) in &doc.blocks {
        if *id == doc.root {
            continue;
        }

        let class = node_class(block);
        let Some(class_name) = class else {
            diagnostics.push(
                CodeGraphDiagnostic::error(
                    "CG1010",
                    "block missing node_class metadata (or custom semantic role)",
                )
                .with_path(block_path(block).unwrap_or_else(|| id.to_string())),
            );
            continue;
        };

        *class_counts.entry(class_name.clone()).or_default() += 1;

        match block_logical_key(block) {
            Some(logical_key) => {
                logical_keys.entry(logical_key).or_default().push(*id);
            }
            None => diagnostics.push(
                CodeGraphDiagnostic::error("CG1011", "missing required logical_key metadata")
                    .with_path(block_path(block).unwrap_or_else(|| id.to_string())),
            ),
        }

        validate_required_metadata(&class_name, block, &mut diagnostics);
    }

    for class in ["repository", "directory", "file", "symbol"] {
        if class_counts.get(class).copied().unwrap_or(0) == 0 {
            diagnostics.push(CodeGraphDiagnostic::warning(
                "CG1012",
                format!("profile has no '{}' nodes", class),
            ));
        }
    }

    for (logical_key, ids) in logical_keys {
        if ids.len() > 1 {
            diagnostics.push(
                CodeGraphDiagnostic::error(
                    "CG1013",
                    format!(
                        "logical_key '{}' is duplicated by {} blocks",
                        logical_key,
                        ids.len()
                    ),
                )
                .with_logical_key(logical_key),
            );
        }
    }

    let logical_by_id = logical_key_index(doc);

    for (source_id, block) in &doc.blocks {
        let Some(source_class) = node_class(block) else {
            continue;
        };
        for edge in &block.edges {
            let target_block = match doc.get_block(&edge.target) {
                Some(b) => b,
                None => {
                    diagnostics.push(
                        CodeGraphDiagnostic::error(
                            "CG1014",
                            format!("edge references missing target block {}", edge.target),
                        )
                        .with_logical_key(
                            logical_by_id
                                .get(source_id)
                                .cloned()
                                .unwrap_or_else(|| source_id.to_string()),
                        ),
                    );
                    continue;
                }
            };

            let target_class = node_class(target_block).unwrap_or_default();

            match &edge.edge_type {
                EdgeType::References => {
                    if source_class != "file" || target_class != "file" {
                        diagnostics.push(
                            CodeGraphDiagnostic::error(
                                "CG1015",
                                "references edges must connect file -> file",
                            )
                            .with_logical_key(
                                logical_by_id
                                    .get(source_id)
                                    .cloned()
                                    .unwrap_or_else(|| source_id.to_string()),
                            ),
                        );
                    }
                }
                EdgeType::Custom(name) if name == "exports" => {
                    if source_class != "file" || target_class != "symbol" {
                        diagnostics.push(
                            CodeGraphDiagnostic::error(
                                "CG1016",
                                "exports edges must connect file -> symbol",
                            )
                            .with_logical_key(
                                logical_by_id
                                    .get(source_id)
                                    .cloned()
                                    .unwrap_or_else(|| source_id.to_string()),
                            ),
                        );
                    }
                }
                _ => {}
            }
        }
    }

    CodeGraphValidationResult {
        valid: diagnostics
            .iter()
            .all(|d| d.severity != CodeGraphSeverity::Error),
        diagnostics,
    }
}

pub fn canonical_codegraph_json(doc: &Document) -> Result<String> {
    let logical_by_id = logical_key_index(doc);

    let mut node_entries = Vec::new();
    for (id, block) in &doc.blocks {
        if *id == doc.root {
            continue;
        }

        let logical_key = logical_by_id
            .get(id)
            .cloned()
            .unwrap_or_else(|| id.to_string());

        let class = node_class(block).unwrap_or_else(|| "unknown".to_string());
        let metadata = normalized_block_metadata(block);

        node_entries.push(json!({
            "logical_key": logical_key,
            "node_class": class,
            "semantic_role": block.metadata.semantic_role.as_ref().map(|r| r.to_string()),
            "content_type": block.content.type_tag(),
            "content": normalize_content(&block.content),
            "metadata": metadata,
        }));
    }

    node_entries.sort_by(|a, b| {
        let ak = a
            .get("logical_key")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let bk = b
            .get("logical_key")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        ak.cmp(bk)
    });

    let mut structure_entries = Vec::new();
    for (parent, children) in &doc.structure {
        let parent_key = logical_by_id
            .get(parent)
            .cloned()
            .unwrap_or_else(|| parent.to_string());

        let mut child_keys: Vec<String> = children
            .iter()
            .map(|child| {
                logical_by_id
                    .get(child)
                    .cloned()
                    .unwrap_or_else(|| child.to_string())
            })
            .collect();
        child_keys.sort();

        structure_entries.push(json!({
            "parent": parent_key,
            "children": child_keys,
        }));
    }

    structure_entries.sort_by(|a, b| {
        let ak = a.get("parent").and_then(|v| v.as_str()).unwrap_or_default();
        let bk = b.get("parent").and_then(|v| v.as_str()).unwrap_or_default();
        ak.cmp(bk)
    });

    let mut edge_entries = Vec::new();
    for (source_id, block) in &doc.blocks {
        let source_key = logical_by_id
            .get(source_id)
            .cloned()
            .unwrap_or_else(|| source_id.to_string());

        for edge in &block.edges {
            let target_key = logical_by_id
                .get(&edge.target)
                .cloned()
                .unwrap_or_else(|| edge.target.to_string());
            edge_entries.push(json!({
                "source": source_key,
                "edge_type": edge.edge_type.as_str(),
                "target": target_key,
                "metadata": normalized_edge_metadata(edge),
            }));
        }
    }

    edge_entries.sort_by(|a, b| {
        let a_source = a.get("source").and_then(|v| v.as_str()).unwrap_or_default();
        let b_source = b.get("source").and_then(|v| v.as_str()).unwrap_or_default();
        a_source
            .cmp(b_source)
            .then_with(|| {
                a.get("edge_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .cmp(
                        b.get("edge_type")
                            .and_then(|v| v.as_str())
                            .unwrap_or_default(),
                    )
            })
            .then_with(|| {
                a.get("target")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .cmp(b.get("target").and_then(|v| v.as_str()).unwrap_or_default())
            })
    });

    let canonical = json!({
        "profile": CODEGRAPH_PROFILE,
        "profile_version": CODEGRAPH_PROFILE_VERSION,
        "nodes": node_entries,
        "structure": structure_entries,
        "edges": edge_entries,
        "document_metadata": normalized_document_metadata(doc),
    });

    Ok(canonical_json(&canonical))
}

pub fn canonical_fingerprint(doc: &Document) -> Result<String> {
    let canonical = canonical_codegraph_json(doc)?;
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    let digest = hasher.finalize();
    Ok(hex::encode(digest))
}

pub fn codegraph_prompt_projection(doc: &Document) -> String {
    let mapper = IdMapper::from_document(doc);
    mapper.document_to_prompt(doc)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CodeLanguage {
    Rust,
    Python,
    TypeScript,
    JavaScript,
}

impl CodeLanguage {
    fn as_str(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::TypeScript => "typescript",
            Self::JavaScript => "javascript",
        }
    }
}

#[derive(Debug, Clone)]
struct RepoFile {
    absolute_path: PathBuf,
    relative_path: String,
    language: CodeLanguage,
}

#[derive(Debug, Clone)]
struct ExtractedSymbol {
    name: String,
    qualified_name: String,
    identity: String,
    parent_identity: Option<String>,
    kind: String,
    exported: bool,
    start_line: usize,
    start_col: usize,
    end_line: usize,
    end_col: usize,
}

#[derive(Debug, Clone)]
struct ExtractedImport {
    module: String,
    symbols: Vec<String>,
    bindings: Vec<ImportBinding>,
    module_aliases: Vec<String>,
    reexported: bool,
    wildcard: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ImportBinding {
    source_name: String,
    local_name: String,
}

#[derive(Debug, Clone)]
struct ExtractedRelationship {
    source_identity: String,
    relation: String,
    target_expr: String,
    target_name: String,
}

#[derive(Debug, Clone)]
struct ExtractedUsage {
    source_identity: String,
    target_expr: String,
    target_name: String,
}

#[derive(Debug, Clone)]
struct ExtractedAlias {
    name: String,
    target_expr: String,
    target_name: String,
    owner_identity: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ImportResolution {
    Resolved(String),
    External,
    Unresolved,
}

#[derive(Debug, Clone, Default)]
struct FileAnalysis {
    symbols: Vec<ExtractedSymbol>,
    imports: Vec<ExtractedImport>,
    relationships: Vec<ExtractedRelationship>,
    usages: Vec<ExtractedUsage>,
    aliases: Vec<ExtractedAlias>,
    exported_symbol_names: BTreeSet<String>,
    default_exported_symbol_names: BTreeSet<String>,
    diagnostics: Vec<CodeGraphDiagnostic>,
}

#[derive(Debug, Clone)]
struct FileAnalysisRecord {
    file: String,
    language: CodeLanguage,
    imports: Vec<ExtractedImport>,
    relationships: Vec<ExtractedRelationship>,
    usages: Vec<ExtractedUsage>,
    aliases: Vec<ExtractedAlias>,
}

impl ExtractedImport {
    fn module(module: impl Into<String>) -> Self {
        Self {
            module: module.into(),
            symbols: Vec::new(),
            bindings: Vec::new(),
            module_aliases: Vec::new(),
            reexported: false,
            wildcard: false,
        }
    }

    fn symbol(module: impl Into<String>, symbol: impl Into<String>) -> Self {
        let symbol = symbol.into();
        Self {
            module: module.into(),
            symbols: vec![symbol.clone()],
            bindings: vec![ImportBinding::same(symbol)],
            module_aliases: Vec::new(),
            reexported: false,
            wildcard: false,
        }
    }

    fn bindings(module: impl Into<String>, bindings: Vec<ImportBinding>) -> Self {
        let mut symbols = bindings
            .iter()
            .map(|binding| binding.source_name.clone())
            .collect::<Vec<_>>();
        symbols.sort();
        symbols.dedup();

        Self {
            module: module.into(),
            symbols,
            bindings,
            module_aliases: Vec::new(),
            reexported: false,
            wildcard: false,
        }
    }

    fn with_module_alias(mut self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        if !alias.is_empty() && !self.module_aliases.contains(&alias) {
            self.module_aliases.push(alias);
        }
        self
    }

    fn reexported(mut self) -> Self {
        self.reexported = true;
        self
    }

    fn wildcard(mut self) -> Self {
        self.wildcard = true;
        self
    }
}

impl ImportBinding {
    fn new(source_name: impl Into<String>, local_name: impl Into<String>) -> Self {
        Self {
            source_name: source_name.into(),
            local_name: local_name.into(),
        }
    }

    fn same(name: impl Into<String>) -> Self {
        let name = name.into();
        Self::new(name.clone(), name)
    }
}

impl ExtractedRelationship {
    fn new(
        source_identity: impl Into<String>,
        relation: impl Into<String>,
        target_expr: impl Into<String>,
        target_name: impl Into<String>,
    ) -> Self {
        Self {
            source_identity: source_identity.into(),
            relation: relation.into(),
            target_expr: target_expr.into(),
            target_name: target_name.into(),
        }
    }
}

impl ExtractedUsage {
    fn new(
        source_identity: impl Into<String>,
        target_expr: impl Into<String>,
        target_name: impl Into<String>,
    ) -> Self {
        Self {
            source_identity: source_identity.into(),
            target_expr: target_expr.into(),
            target_name: target_name.into(),
        }
    }
}

impl ExtractedAlias {
    fn new(
        name: impl Into<String>,
        target_expr: impl Into<String>,
        target_name: impl Into<String>,
        owner_identity: Option<&str>,
    ) -> Self {
        Self {
            name: name.into(),
            target_expr: target_expr.into(),
            target_name: target_name.into(),
            owner_identity: owner_identity.map(str::to_string),
        }
    }
}

fn initialize_document_metadata(
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

fn make_repository_block(repo_name: &str, commit_hash: &str) -> Block {
    let mut block = Block::new(
        Content::json(json!({
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
}

fn make_directory_block(path: &str) -> Block {
    let mut block = Block::new(
        Content::json(json!({
            "path": path,
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
        .insert(META_PATH.to_string(), json!(path));
    block.metadata.custom.insert(
        META_LOGICAL_KEY.to_string(),
        json!(format!("directory:{}", path)),
    );
    block
}

fn make_file_block(path: &str, language: &str) -> Block {
    let mut block = Block::new(
        Content::json(json!({
            "path": path,
            "language": language,
        })),
        Some("custom.file"),
    );
    block.metadata.label = Some(path.to_string());
    block
        .metadata
        .custom
        .insert(META_NODE_CLASS.to_string(), json!("file"));
    block
        .metadata
        .custom
        .insert(META_PATH.to_string(), json!(path));
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

fn make_symbol_block(
    logical_key: &str,
    path: &str,
    language: &str,
    symbol: &ExtractedSymbol,
) -> Block {
    let span = json!({
        "start_line": symbol.start_line,
        "start_col": symbol.start_col,
        "end_line": symbol.end_line,
        "end_col": symbol.end_col,
    });

    let mut block = Block::new(
        Content::json(json!({
            "name": symbol.name,
            "kind": symbol.kind,
            "path": path,
            "span": span,
            "exported": symbol.exported,
        })),
        Some("custom.symbol"),
    );

    block.metadata.label = Some(symbol.name.clone());
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
        .insert(META_PATH.to_string(), json!(path));
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
    block.metadata.custom.insert(META_SPAN.to_string(), span);
    block
        .metadata
        .custom
        .insert(META_EXPORTED.to_string(), json!(symbol.exported));
    block
}

fn analyze_file(path: &str, source: &str, language: CodeLanguage) -> FileAnalysis {
    let mut analysis = FileAnalysis::default();
    let mut parser = Parser::new();
    if parser.set_language(language_for(language)).is_err() {
        analysis.diagnostics.push(
            CodeGraphDiagnostic::error(
                "CG2010",
                format!(
                    "failed to initialize tree-sitter parser for {}",
                    language.as_str()
                ),
            )
            .with_path(path.to_string()),
        );
        return analysis;
    }

    let Some(tree) = parser.parse(source, None) else {
        analysis.diagnostics.push(
            CodeGraphDiagnostic::error("CG2011", "tree-sitter returned no parse tree")
                .with_path(path.to_string()),
        );
        return analysis;
    };

    let root = tree.root_node();
    if root.has_error() {
        analysis.diagnostics.push(
            CodeGraphDiagnostic::warning(
                "CG2002",
                "tree-sitter parser reported syntax errors; extraction continues",
            )
            .with_path(path.to_string()),
        );
    }

    match language {
        CodeLanguage::Rust => analyze_rust_tree(source, root, &mut analysis),
        CodeLanguage::Python => analyze_python_tree(path, source, root, &mut analysis),
        CodeLanguage::TypeScript | CodeLanguage::JavaScript => {
            analyze_ts_tree(source, root, &mut analysis)
        }
    }

    if analysis.symbols.is_empty() {
        analysis.diagnostics.push(
            CodeGraphDiagnostic::info(
                "CG2001",
                format!("no symbols extracted for {}", path),
            )
            .with_path(path.to_string()),
        );
    }

    analysis
}

fn language_for(language: CodeLanguage) -> Language {
    match language {
        CodeLanguage::Rust => tree_sitter_rust::language(),
        CodeLanguage::Python => tree_sitter_python::language(),
        CodeLanguage::TypeScript => tree_sitter_typescript::language_typescript(),
        CodeLanguage::JavaScript => tree_sitter_javascript::language(),
    }
}

fn is_python_package_init(path: &str) -> bool {
    path == "__init__.py" || path.ends_with("/__init__.py")
}

fn analyze_rust_tree(source: &str, root: Node<'_>, analysis: &mut FileAnalysis) {
    let mut cursor = root.walk();
    for node in root.named_children(&mut cursor) {
        analyze_rust_node(source, node, analysis, &[], None);
    }
}

fn analyze_rust_node(
    source: &str,
    node: Node<'_>,
    analysis: &mut FileAnalysis,
    scope: &[String],
    parent_identity: Option<&str>,
) {
    if let Some(source_identity) = parent_identity {
        if let Some((target_expr, target_name)) = rust_call_target(node, source) {
            analysis.usages.push(ExtractedUsage::new(
                source_identity,
                target_expr,
                target_name,
            ));
        }
    }

    if node.kind() == "use_declaration" {
        let use_text = node_text(source, node);
        let reexported = use_text.trim_start().starts_with("pub use ");
        for (module, local_alias, wildcard) in expand_rust_use_declaration_items(use_text) {
            let mut import = if wildcard {
                ExtractedImport::module(module)
            } else if let Some(local_name) = local_alias {
                if let Some(source_name) = rust_imported_symbol_name(&module) {
                    ExtractedImport::bindings(
                        module,
                        vec![ImportBinding::new(source_name, local_name.clone())],
                    )
                    .with_module_alias(local_name)
                } else {
                    ExtractedImport::module(module).with_module_alias(local_name)
                }
            } else if let Some(symbol) = rust_imported_symbol_name(&module) {
                ExtractedImport::symbol(module, symbol)
            } else {
                ExtractedImport::module(module)
            };
            if reexported {
                import = import.reexported();
            }
            if wildcard {
                import = import.wildcard();
            }
            analysis.imports.push(import);
        }
    }

    if node.kind() == "let_declaration" {
        if let Some(source_identity) = parent_identity {
            analysis
                .aliases
                .extend(rust_aliases_from_let_declaration(node, source, source_identity));
        }
    }

    if node.kind() == "mod_item" {
        let text = node_text(source, node);
        if text.trim().ends_with(';') {
            if let Some(name) = rust_symbol_name(node, source) {
                analysis
                    .imports
                    .push(ExtractedImport::module(format!("mod:{}", name)));
            }
        }
    }

    let mut child_scope = scope.to_vec();
    let mut child_parent_identity = parent_identity.map(str::to_string);

    if let Some(symbol) = rust_symbol_from_node(node, source, scope, parent_identity) {
        analysis
            .relationships
            .extend(rust_symbol_relationships(node, source, &symbol));
        child_scope.push(symbol.name.clone());
        child_parent_identity = Some(symbol.identity.clone());
        analysis.symbols.push(symbol);
    }

    let scope_ref = if child_scope.len() == scope.len() {
        scope
    } else {
        &child_scope
    };

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        analyze_rust_node(
            source,
            child,
            analysis,
            scope_ref,
            child_parent_identity.as_deref(),
        );
    }
}

fn rust_symbol_from_node(
    node: Node<'_>,
    source: &str,
    scope: &[String],
    parent_identity: Option<&str>,
) -> Option<ExtractedSymbol> {
    let kind = match node.kind() {
        "function_item" => "function",
        "struct_item" => "struct",
        "enum_item" => "enum",
        "trait_item" => "trait",
        "impl_item" => "impl",
        "type_item" => "type",
        "const_item" => "const",
        "mod_item" => "module",
        _ => return None,
    };

    let name = rust_symbol_name(node, source)?;
    let exported = scope.is_empty() && node_text(source, node).trim_start().starts_with("pub");

    Some(make_extracted_symbol(
        name,
        kind,
        exported,
        scope,
        parent_identity,
        node,
    ))
}

fn rust_symbol_name(node: Node<'_>, source: &str) -> Option<String> {
    if let Some(name_node) = node.child_by_field_name("name") {
        let name = node_text(source, name_node).trim().to_string();
        if !name.is_empty() {
            return Some(name);
        }
    }

    if node.kind() == "impl_item" {
        if let Some(type_node) = node.child_by_field_name("type") {
            let name = node_text(source, type_node).trim().to_string();
            if !name.is_empty() {
                return Some(name);
            }
        }
    }

    first_named_identifier(node, source)
}

fn rust_imported_symbol_name(module: &str) -> Option<String> {
    if module.starts_with("mod:") {
        return None;
    }

    let stripped = module
        .trim_start_matches("crate::")
        .trim_start_matches("self::")
        .trim_start_matches("super::");
    let segments: Vec<&str> = stripped.split("::").filter(|segment| !segment.is_empty()).collect();

    if module.starts_with("crate::") && segments.len() == 1 {
        return segments.first().map(|segment| (*segment).to_string());
    }

    if segments.len() >= 2 {
        segments.last().map(|segment| (*segment).to_string())
    } else {
        None
    }
}

fn rust_symbol_relationships(
    node: Node<'_>,
    source: &str,
    symbol: &ExtractedSymbol,
) -> Vec<ExtractedRelationship> {
    let mut relationships = Vec::new();

    match node.kind() {
        "impl_item" => {
            if let Some(type_node) = node.child_by_field_name("type") {
                if let Some((target_expr, target_name)) = rust_type_reference(type_node, source) {
                    relationships.push(ExtractedRelationship::new(
                        symbol.identity.clone(),
                        "for_type",
                        target_expr,
                        target_name,
                    ));
                }
            }

            if let Some(trait_node) = node.child_by_field_name("trait") {
                if let Some((target_expr, target_name)) = rust_type_reference(trait_node, source) {
                    relationships.push(ExtractedRelationship::new(
                        symbol.identity.clone(),
                        "implements",
                        target_expr,
                        target_name,
                    ));
                }
            }
        }
        "trait_item" => {
            if let Some(bounds_node) = node.child_by_field_name("bounds") {
                let mut cursor = bounds_node.walk();
                for bound in bounds_node.named_children(&mut cursor) {
                    if let Some((target_expr, target_name)) = rust_type_reference(bound, source) {
                        relationships.push(ExtractedRelationship::new(
                            symbol.identity.clone(),
                            "extends",
                            target_expr,
                            target_name,
                        ));
                    }
                }
            }
        }
        _ => {}
    }

    relationships
}

fn rust_type_reference(node: Node<'_>, source: &str) -> Option<(String, String)> {
    let raw = node_text(source, node).trim();
    let trimmed = raw.trim_start_matches('&').trim_start();
    let trimmed = trimmed.strip_prefix("mut ").unwrap_or(trimmed).trim();
    let core = trimmed.split('<').next().unwrap_or(trimmed).trim();
    let name = rust_last_path_segment(core)?;
    Some((core.to_string(), name))
}

fn rust_call_target(node: Node<'_>, source: &str) -> Option<(String, String)> {
    if node.kind() != "call_expression" {
        return None;
    }

    let function_node = node.child_by_field_name("function")?;
    rust_callable_reference(function_node, source)
}

fn rust_aliases_from_let_declaration(
    node: Node<'_>,
    source: &str,
    owner_identity: &str,
) -> Vec<ExtractedAlias> {
    let Some(pattern_node) = node.child_by_field_name("pattern") else {
        return Vec::new();
    };
    if pattern_node.kind() != "identifier" {
        return Vec::new();
    }

    let alias_name = node_text(source, pattern_node).trim().to_string();
    if alias_name.is_empty() {
        return Vec::new();
    }

    let (target_expr, target_name) = node
        .child_by_field_name("value")
        .and_then(|value_node| rust_callable_reference(value_node, source))
        .unwrap_or_else(|| (String::new(), String::new()));

    vec![ExtractedAlias::new(
        alias_name,
        target_expr,
        target_name,
        Some(owner_identity),
    )]
}

fn rust_callable_reference(node: Node<'_>, source: &str) -> Option<(String, String)> {
    let raw = match node.kind() {
        "identifier" | "scoped_identifier" => node_text(source, node).trim().to_string(),
        "generic_function" => node_text(source, node).trim().to_string(),
        _ => return None,
    };
    let core = raw.split('<').next().unwrap_or(raw.as_str()).trim();
    let name = rust_last_path_segment(core)?;
    Some((core.to_string(), name))
}

fn rust_last_path_segment(path: &str) -> Option<String> {
    let segment = path.rsplit("::").next().unwrap_or(path).trim();
    if segment.is_empty() {
        None
    } else {
        Some(segment.to_string())
    }
}

fn analyze_python_tree(path: &str, source: &str, root: Node<'_>, analysis: &mut FileAnalysis) {
    let mut cursor = root.walk();
    for node in root.named_children(&mut cursor) {
        analyze_python_node(source, node, analysis, &[], None);
    }

    apply_python_explicit_exports(analysis);

    if is_python_package_init(path) {
        for import in &mut analysis.imports {
            import.reexported = true;
        }
    }
}

fn analyze_python_node(
    source: &str,
    node: Node<'_>,
    analysis: &mut FileAnalysis,
    scope: &[String],
    parent_identity: Option<&str>,
) {
    if let Some(source_identity) = parent_identity {
        if let Some((target_expr, target_name)) = python_call_target(node, source) {
            analysis.usages.push(ExtractedUsage::new(
                source_identity,
                target_expr,
                target_name,
            ));
        }
    }

    match node.kind() {
        "import_statement" => {
            analysis
                .imports
                .extend(python_imports_from_import_statement(node, source));
        }
        "import_from_statement" => {
            analysis
                .imports
                .extend(python_imports_from_from_statement(node, source));
        }
        "expression_statement" => {
            if scope.is_empty() {
                analysis
                    .exported_symbol_names
                    .extend(python_explicit_exports_from_statement(node, source));
            }
            if scope.is_empty() || parent_identity.is_some() {
                analysis
                    .aliases
                    .extend(python_aliases_from_statement(node, source, parent_identity));
            }
        }
        _ => {}
    }

    let mut child_scope = scope.to_vec();
    let mut child_parent_identity = parent_identity.map(str::to_string);

    if let Some(symbol) = python_symbol_from_node(node, source, scope, parent_identity) {
        analysis
            .relationships
            .extend(python_symbol_relationships(node, source, &symbol));
        child_scope.push(symbol.name.clone());
        child_parent_identity = Some(symbol.identity.clone());
        analysis.symbols.push(symbol);
    }

    let scope_ref = if child_scope.len() == scope.len() {
        scope
    } else {
        &child_scope
    };

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        analyze_python_node(
            source,
            child,
            analysis,
            scope_ref,
            child_parent_identity.as_deref(),
        );
    }
}

fn python_symbol_from_node(
    node: Node<'_>,
    source: &str,
    scope: &[String],
    parent_identity: Option<&str>,
) -> Option<ExtractedSymbol> {
    let kind = match node.kind() {
        "class_definition" => "class",
        "function_definition" => "function",
        _ => return None,
    };

    let name_node = node
        .child_by_field_name("name")
        .or_else(|| node.child_by_field_name("property"))?;
    let name = node_text(source, name_node).trim().to_string();
    if name.is_empty() {
        return None;
    }

    Some(make_extracted_symbol(
        name.clone(),
        kind,
        scope.is_empty() && !name.starts_with('_'),
        scope,
        parent_identity,
        node,
    ))
}

fn python_symbol_relationships(
    node: Node<'_>,
    source: &str,
    symbol: &ExtractedSymbol,
) -> Vec<ExtractedRelationship> {
    if node.kind() != "class_definition" {
        return Vec::new();
    }

    let Some(superclasses) = node.child_by_field_name("superclasses") else {
        return Vec::new();
    };

    let mut relationships = Vec::new();
    let mut cursor = superclasses.walk();
    for child in superclasses.named_children(&mut cursor) {
        if let Some((target_expr, target_name)) = python_class_base_reference(child, source) {
            relationships.push(ExtractedRelationship::new(
                symbol.identity.clone(),
                "extends",
                target_expr,
                target_name,
            ));
        }
    }

    relationships
}

fn python_class_base_reference(node: Node<'_>, source: &str) -> Option<(String, String)> {
    if matches!(node.kind(), "keyword_argument" | "list_splat" | "dictionary_splat") {
        return None;
    }

    let text = node_text(source, node).trim();
    let name = simple_symbol_reference_name(text)?;
    Some((text.to_string(), name))
}

fn python_call_target(node: Node<'_>, source: &str) -> Option<(String, String)> {
    if node.kind() != "call" {
        return None;
    }

    let function_node = node.child_by_field_name("function")?;
    match function_node.kind() {
        "identifier" => {
            let target_expr = node_text(source, function_node).trim().to_string();
            if target_expr.is_empty() {
                return None;
            }
            Some((target_expr.clone(), target_expr))
        }
        "attribute" => {
            let object = function_node.child_by_field_name("object")?;
            let attribute = function_node.child_by_field_name("attribute")?;
            if object.kind() != "identifier" {
                return None;
            }
            let object_name = node_text(source, object).trim().to_string();
            let attr_name = node_text(source, attribute).trim().to_string();
            if object_name.is_empty() || attr_name.is_empty() {
                return None;
            }
            Some((format!("{object_name}.{attr_name}"), attr_name))
        }
        _ => None,
    }
}

fn python_imports_from_import_statement(node: Node<'_>, source: &str) -> Vec<ExtractedImport> {
    let mut imports = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "aliased_import" => {
                let module_name = child
                    .child_by_field_name("name")
                    .map(|name_node| node_text(source, name_node).trim().to_string())
                    .unwrap_or_default();
                let alias = child
                    .child_by_field_name("alias")
                    .map(|alias_node| node_text(source, alias_node).trim().to_string())
                    .unwrap_or_default();
                if !module_name.is_empty() {
                    imports.push(ExtractedImport::module(module_name).with_module_alias(alias));
                }
            }
            "dotted_name" => {
                let module_name = node_text(source, child).trim().to_string();
                if module_name.is_empty() {
                    continue;
                }
                let mut import = ExtractedImport::module(module_name.clone());
                if !module_name.contains('.') {
                    import = import.with_module_alias(module_name);
                }
                imports.push(import);
            }
            _ => {}
        }
    }
    imports
}

fn python_aliases_from_statement(
    node: Node<'_>,
    source: &str,
    owner_identity: Option<&str>,
) -> Vec<ExtractedAlias> {
    if node.kind() != "expression_statement" {
        return Vec::new();
    }

    let mut aliases = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() != "assignment" {
            continue;
        }

        let Some(left) = child.child_by_field_name("left") else {
            continue;
        };
        let Some(right) = child.child_by_field_name("right") else {
            continue;
        };
        if left.kind() != "identifier" {
            continue;
        }

        let alias_name = node_text(source, left).trim().to_string();
        if alias_name.is_empty() {
            continue;
        }

        let (target_expr, target_name) =
            python_alias_reference(right, source).unwrap_or_else(|| (String::new(), String::new()));
        aliases.push(ExtractedAlias::new(
            alias_name,
            target_expr,
            target_name,
            owner_identity,
        ));
    }

    aliases
}

fn python_alias_reference(node: Node<'_>, source: &str) -> Option<(String, String)> {
    match node.kind() {
        "identifier" => {
            let text = node_text(source, node).trim().to_string();
            if text.is_empty() {
                None
            } else {
                Some((text.clone(), text))
            }
        }
        "attribute" => {
            let object = node.child_by_field_name("object")?;
            let attribute = node.child_by_field_name("attribute")?;
            if object.kind() != "identifier" {
                return None;
            }
            let object_name = node_text(source, object).trim().to_string();
            let attr_name = node_text(source, attribute).trim().to_string();
            if object_name.is_empty() || attr_name.is_empty() {
                None
            } else {
                Some((format!("{object_name}.{attr_name}"), attr_name))
            }
        }
        _ => None,
    }
}

fn python_imports_from_from_statement(node: Node<'_>, source: &str) -> Vec<ExtractedImport> {
    let module_name = node
        .child_by_field_name("module_name")
        .map(|module| node_text(source, module).trim().to_string())
        .unwrap_or_default();
    let imported_bindings = python_imported_bindings(node, source);
    let wildcard = python_has_wildcard_import(node);

    if module_name.is_empty() {
        return Vec::new();
    }

    if module_name.chars().all(|ch| ch == '.') {
        return imported_bindings
            .into_iter()
            .map(|binding| {
                let mut import = ExtractedImport::module(format!("{}{}", module_name, binding.source_name));
                if !binding.local_name.is_empty() {
                    import = import.with_module_alias(binding.local_name);
                }
                import
            })
            .collect();
    }

    let mut import = ExtractedImport::bindings(module_name, imported_bindings);
    if wildcard {
        import = import.wildcard();
    }
    vec![import]
}

fn python_imported_bindings(node: Node<'_>, source: &str) -> Vec<ImportBinding> {
    let module_name = node.child_by_field_name("module_name");
    let mut cursor = node.walk();
    let mut bindings = Vec::new();

    for child in node.named_children(&mut cursor) {
        if Some(child) == module_name || child.kind() == "wildcard_import" {
            continue;
        }

        let binding = match child.kind() {
            "aliased_import" => {
                let source_name = child
                    .child_by_field_name("name")
                    .map(|name_node| node_text(source, name_node).trim().to_string())
                    .unwrap_or_default();
                let local_name = child
                    .child_by_field_name("alias")
                    .map(|alias_node| node_text(source, alias_node).trim().to_string())
                    .unwrap_or_default();
                let source_name = source_name.rsplit('.').next().unwrap_or("").trim();
                if source_name.is_empty() || local_name.is_empty() {
                    None
                } else {
                    Some(ImportBinding::new(source_name, local_name))
                }
            }
            _ => {
                let source_name = node_text(source, child)
                    .trim()
                    .rsplit('.')
                    .next()
                    .unwrap_or("")
                    .trim();
                if source_name.is_empty() {
                    None
                } else {
                    Some(ImportBinding::same(source_name))
                }
            }
        };

        if let Some(binding) = binding {
            bindings.push(binding);
        }
    }

    bindings.sort();
    bindings.dedup();
    bindings
}

fn python_has_wildcard_import(node: Node<'_>) -> bool {
    let mut cursor = node.walk();
    let has_wildcard = node
        .named_children(&mut cursor)
        .any(|child| child.kind() == "wildcard_import");
    has_wildcard
}

fn python_explicit_exports_from_statement(node: Node<'_>, source: &str) -> Vec<String> {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() != "assignment" {
            continue;
        }

        let Some(left) = child.child_by_field_name("left") else {
            continue;
        };
        if node_text(source, left).trim() != "__all__" {
            continue;
        }

        let Some(right) = child.child_by_field_name("right") else {
            continue;
        };
        return python_string_sequence_values(right, source);
    }

    Vec::new()
}

fn python_string_sequence_values(node: Node<'_>, source: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut stack = vec![node];

    while let Some(current) = stack.pop() {
        if current.kind() == "string" {
            if let Some(value) = python_string_literal_value(node_text(source, current)) {
                values.push(value);
            }
            continue;
        }

        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }

    values
}

fn python_string_literal_value(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    let quote_index = trimmed.find(['\'', '"'])?;
    let quote = trimmed[quote_index..].chars().next()?;
    let rest = &trimmed[quote_index + quote.len_utf8()..];
    let end_index = rest.rfind(quote)?;
    let value = rest[..end_index].trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn apply_python_explicit_exports(analysis: &mut FileAnalysis) {
    if analysis.exported_symbol_names.is_empty() {
        return;
    }

    for symbol in analysis.symbols.iter_mut() {
        if symbol.parent_identity.is_none() {
            symbol.exported = analysis.exported_symbol_names.contains(&symbol.name);
        }
    }

    for import in analysis.imports.iter_mut() {
        if import.wildcard && !analysis.exported_symbol_names.is_empty() {
            import.symbols = analysis.exported_symbol_names.iter().cloned().collect();
            import.reexported = true;
        }

        if import
            .symbols
            .iter()
            .any(|symbol| analysis.exported_symbol_names.contains(symbol))
        {
            import.reexported = true;
        }
    }
}

fn analyze_ts_tree(source: &str, root: Node<'_>, analysis: &mut FileAnalysis) {
    let mut cursor = root.walk();
    for node in root.named_children(&mut cursor) {
        analyze_ts_node(source, node, analysis, &[], None, false);
    }

    mark_exported_symbols(&mut analysis.symbols, &analysis.exported_symbol_names);
}

fn analyze_ts_node(
    source: &str,
    node: Node<'_>,
    analysis: &mut FileAnalysis,
    scope: &[String],
    parent_identity: Option<&str>,
    exported_context: bool,
) {
    if let Some(source_identity) = parent_identity {
        if let Some((target_expr, target_name)) = ts_call_target(node, source) {
            analysis.usages.push(ExtractedUsage::new(
                source_identity,
                target_expr,
                target_name,
            ));
        }
    }

    match node.kind() {
        "import_statement" => {
            if let Some(module) = extract_ts_module_from_text(node_text(source, node)) {
                let imported_bindings = ts_import_bindings(node, source);
                let mut import = ExtractedImport::bindings(module, imported_bindings);
                for alias in ts_namespace_import_aliases(node, source) {
                    import = import.with_module_alias(alias);
                }
                analysis.imports.push(import);
            }
        }
        "export_statement" => {
            if let Some(module) = extract_ts_module_from_text(node_text(source, node)) {
                let export_bindings = ts_reexport_bindings(node, source);
                let mut import = ExtractedImport::bindings(module, export_bindings).reexported();
                if ts_is_wildcard_reexport(node, source) {
                    import = import.wildcard();
                }
                analysis.imports.push(import);
            }
            collect_ts_local_export_names(node, source, &mut analysis.exported_symbol_names);
            analysis
                .default_exported_symbol_names
                .extend(ts_default_export_names_from_text(node_text(source, node)));

            let mut cursor = node.walk();
            for child in node.named_children(&mut cursor) {
                analyze_ts_node(source, child, analysis, scope, parent_identity, true);
            }
            return;
        }
        "lexical_declaration" | "variable_statement" => {
            analysis
                .imports
                .extend(ts_require_imports_from_variable_statement(node, source));
            analysis.symbols.extend(ts_variable_symbols(
                node,
                source,
                exported_context,
                scope,
                parent_identity,
            ));
            if scope.is_empty() || parent_identity.is_some() {
                analysis
                    .aliases
                    .extend(ts_aliases_from_variable_statement(node, source, parent_identity));
            }
            return;
        }
        "expression_statement" => {
            if scope.is_empty() && parent_identity.is_none() {
                collect_ts_commonjs_exports(node, source, analysis);
            }
        }
        _ => {}
    }

    let mut child_scope = scope.to_vec();
    let mut child_parent_identity = parent_identity.map(str::to_string);

    if let Some(symbol) =
        ts_symbol_from_declaration(node, source, exported_context, scope, parent_identity)
    {
        analysis
            .relationships
            .extend(ts_symbol_relationships(node, source, &symbol));
        child_scope.push(symbol.name.clone());
        child_parent_identity = Some(symbol.identity.clone());
        analysis.symbols.push(symbol);
    }

    let scope_ref = if child_scope.len() == scope.len() {
        scope
    } else {
        &child_scope
    };

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        analyze_ts_node(
            source,
            child,
            analysis,
            scope_ref,
            child_parent_identity.as_deref(),
            false,
        );
    }
}

fn ts_symbol_from_declaration(
    node: Node<'_>,
    source: &str,
    exported_hint: bool,
    scope: &[String],
    parent_identity: Option<&str>,
) -> Option<ExtractedSymbol> {
    let kind = match node.kind() {
        "function_declaration" => "function",
        "generator_function_declaration" => "function",
        "class_declaration" => "class",
        "interface_declaration" => "interface",
        "type_alias_declaration" => "type",
        "enum_declaration" => "enum",
        "module" => "namespace",
        "method_definition" => "method",
        "public_field_definition" => ts_public_field_kind(node),
        "field_definition" => ts_public_field_kind(node),
        _ => return None,
    };

    let name = node
        .child_by_field_name("name")
        .or_else(|| node.child_by_field_name("property"))
        .map(|n| node_text(source, n).trim().to_string())
        .or_else(|| first_named_identifier(node, source))?;
    if name.is_empty() {
        return None;
    }
    let exported = scope.is_empty()
        && (exported_hint || node_text(source, node).trim_start().starts_with("export "));

    Some(make_extracted_symbol(
        name,
        kind,
        exported,
        scope,
        parent_identity,
        node,
    ))
}

fn ts_variable_symbols(
    node: Node<'_>,
    source: &str,
    exported_hint: bool,
    scope: &[String],
    parent_identity: Option<&str>,
) -> Vec<ExtractedSymbol> {
    let mut out = Vec::new();
    let exported = scope.is_empty()
        && (exported_hint || node_text(source, node).trim_start().starts_with("export "));

    let mut stack = vec![node];
    while let Some(current) = stack.pop() {
        if current.kind() == "variable_declarator" {
            if ts_is_require_declarator(current, source) {
                continue;
            }
            if let Some(name_node) = current.child_by_field_name("name") {
                let name = node_text(source, name_node).trim().to_string();
                if !name.is_empty() {
                    let kind = ts_variable_symbol_kind(current);
                    if !scope.is_empty() && kind == "variable" {
                        continue;
                    }
                    out.push(make_extracted_symbol(
                        name,
                        kind,
                        exported,
                        scope,
                        parent_identity,
                        current,
                    ));
                }
            }
            continue;
        }

        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }

    out
}

fn ts_symbol_relationships(
    node: Node<'_>,
    source: &str,
    symbol: &ExtractedSymbol,
) -> Vec<ExtractedRelationship> {
    if !matches!(node.kind(), "class_declaration" | "interface_declaration") {
        return Vec::new();
    }

    let mut relationships = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "class_heritage" => {
                let mut heritage_cursor = child.walk();
                for clause in child.named_children(&mut heritage_cursor) {
                    match clause.kind() {
                        "extends_clause" => {
                            if let Some(value_node) = clause.child_by_field_name("value") {
                                if let Some((target_expr, target_name)) =
                                    ts_type_reference(value_node, source)
                                {
                                    relationships.push(ExtractedRelationship::new(
                                        symbol.identity.clone(),
                                        "extends",
                                        target_expr,
                                        target_name,
                                    ));
                                }
                            }
                        }
                        "implements_clause" => {
                            let mut clause_cursor = clause.walk();
                            for type_node in clause.named_children(&mut clause_cursor) {
                                if let Some((target_expr, target_name)) =
                                    ts_type_reference(type_node, source)
                                {
                                    relationships.push(ExtractedRelationship::new(
                                        symbol.identity.clone(),
                                        "implements",
                                        target_expr,
                                        target_name,
                                    ));
                                }
                            }
                        }
                        _ => {
                            if let Some((target_expr, target_name)) =
                                ts_type_reference(clause, source)
                            {
                                relationships.push(ExtractedRelationship::new(
                                    symbol.identity.clone(),
                                    "extends",
                                    target_expr,
                                    target_name,
                                ));
                            }
                        }
                    }
                }
            }
            "extends_type_clause" => {
                let mut clause_cursor = child.walk();
                for type_node in child.named_children(&mut clause_cursor) {
                    if let Some((target_expr, target_name)) = ts_type_reference(type_node, source)
                    {
                        relationships.push(ExtractedRelationship::new(
                            symbol.identity.clone(),
                            "extends",
                            target_expr,
                            target_name,
                        ));
                    }
                }
            }
            _ => {}
        }
    }

    relationships
}

fn ts_type_reference(node: Node<'_>, source: &str) -> Option<(String, String)> {
    let raw = if node.kind() == "generic_type" {
        node.child_by_field_name("type")
            .map(|type_node| node_text(source, type_node).trim().to_string())
            .unwrap_or_else(|| node_text(source, node).trim().to_string())
    } else {
        node_text(source, node).trim().to_string()
    };
    let name = simple_symbol_reference_name(&raw)?;
    Some((raw, name))
}

fn ts_call_target(node: Node<'_>, source: &str) -> Option<(String, String)> {
    let callable = match node.kind() {
        "call_expression" => node.child_by_field_name("function")?,
        "new_expression" => node.child_by_field_name("constructor")?,
        _ => return None,
    };

    match callable.kind() {
        "identifier" => {
            let target_expr = node_text(source, callable).trim().to_string();
            if target_expr.is_empty() {
                return None;
            }
            Some((target_expr.clone(), target_expr))
        }
        "member_expression" => {
            let object = callable.child_by_field_name("object")?;
            let property = callable.child_by_field_name("property")?;
            if object.kind() != "identifier" {
                return None;
            }
            let object_name = node_text(source, object).trim().to_string();
            let property_name = node_text(source, property).trim().to_string();
            if object_name.is_empty() || property_name.is_empty() {
                return None;
            }
            Some((format!("{object_name}.{property_name}"), property_name))
        }
        _ => None,
    }
}

fn ts_require_imports_from_variable_statement(node: Node<'_>, source: &str) -> Vec<ExtractedImport> {
    let mut imports = Vec::new();
    let mut stack = vec![node];

    while let Some(current) = stack.pop() {
        if current.kind() == "variable_declarator" {
            let Some(name_node) = current.child_by_field_name("name") else {
                continue;
            };
            let Some(value_node) = current.child_by_field_name("value") else {
                continue;
            };
            let Some(module) = ts_require_module_from_value(value_node, source) else {
                continue;
            };

            match name_node.kind() {
                "identifier" => {
                    let local_name = node_text(source, name_node).trim().to_string();
                    if !local_name.is_empty() {
                        imports.push(
                            ExtractedImport::bindings(
                                module.clone(),
                                vec![ImportBinding::new("default", local_name.clone())],
                            )
                            .with_module_alias(local_name),
                        );
                    }
                }
                "object_pattern" => {
                    let bindings = ts_object_pattern_bindings(name_node, source);
                    if !bindings.is_empty() {
                        imports.push(ExtractedImport::bindings(module.clone(), bindings));
                    }
                }
                _ => {}
            }
            continue;
        }

        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }

    imports
}

fn ts_require_module_from_value(node: Node<'_>, source: &str) -> Option<String> {
    if node.kind() != "call_expression" {
        return None;
    }
    let function = node.child_by_field_name("function")?;
    if function.kind() != "identifier" || node_text(source, function).trim() != "require" {
        return None;
    }
    let arguments = node.child_by_field_name("arguments")?;
    let mut cursor = arguments.walk();
    let argument = arguments.named_children(&mut cursor).next()?;
    if argument.kind() != "string" {
        return None;
    }
    let module = node_text(source, argument)
        .trim()
        .trim_matches(|c| c == '\'' || c == '"')
        .to_string();
    if module.is_empty() {
        None
    } else {
        Some(module)
    }
}

fn ts_is_require_declarator(node: Node<'_>, source: &str) -> bool {
    node.child_by_field_name("value")
        .and_then(|value| ts_require_module_from_value(value, source))
        .is_some()
}

fn ts_object_pattern_bindings(node: Node<'_>, source: &str) -> Vec<ImportBinding> {
    let text = node_text(source, node).trim();
    let Some(inner) = text.strip_prefix('{').and_then(|rest| rest.strip_suffix('}')) else {
        return Vec::new();
    };

    let mut bindings = Vec::new();
    for part in inner.split(',') {
        let binding = part.trim();
        if binding.is_empty() || binding.starts_with("...") {
            continue;
        }
        let binding = binding.split('=').next().unwrap_or(binding).trim();
        let mut pieces = binding.split(':').map(str::trim);
        let Some(source_name) = pieces.next() else {
            continue;
        };
        let local_name = pieces.next().unwrap_or(source_name);
        if !source_name.is_empty()
            && !local_name.is_empty()
            && leading_js_identifier(source_name).as_deref() == Some(source_name)
            && leading_js_identifier(local_name).as_deref() == Some(local_name)
        {
            bindings.push(ImportBinding::new(source_name, local_name));
        }
    }

    bindings.sort();
    bindings.dedup();
    bindings
}

fn collect_ts_commonjs_exports(node: Node<'_>, source: &str, analysis: &mut FileAnalysis) {
    let Some(assignment) = node.named_child(0).filter(|child| child.kind() == "assignment_expression")
    else {
        return;
    };
    let Some(left) = assignment.child_by_field_name("left") else {
        return;
    };
    let Some(right) = assignment.child_by_field_name("right") else {
        return;
    };
    let Some(export_name) = ts_commonjs_export_name(left, source) else {
        return;
    };

    if export_name == "default" {
        let object_export_names = ts_commonjs_object_export_names(right, source);
        if !object_export_names.is_empty() {
            analysis.exported_symbol_names.extend(object_export_names);
            return;
        }
    }

    let target_name = if right.kind() == "identifier" {
        let name = node_text(source, right).trim().to_string();
        if name.is_empty() { None } else { Some(name) }
    } else if let Some(symbol) = ts_commonjs_symbol_from_expression(right, source, &export_name) {
        let name = symbol.name.clone();
        analysis.symbols.push(symbol);
        Some(name)
    } else {
        None
    };

    if let Some(target_name) = target_name {
        analysis.exported_symbol_names.insert(target_name.clone());
        if export_name == "default" {
            analysis.default_exported_symbol_names.insert(target_name);
        }
    }
}

fn ts_commonjs_object_export_names(node: Node<'_>, source: &str) -> Vec<String> {
    if node.kind() != "object" {
        return Vec::new();
    }

    let text = node_text(source, node).trim();
    let Some(inner) = text.strip_prefix('{').and_then(|rest| rest.strip_suffix('}')) else {
        return Vec::new();
    };

    let mut names = Vec::new();
    for part in inner.split(',') {
        let property = part.trim();
        if property.is_empty() || property.starts_with("...") {
            continue;
        }
        let property = property.split('=').next().unwrap_or(property).trim();
        let mut pieces = property.split(':').map(str::trim);
        let Some(export_name) = pieces.next() else {
            continue;
        };
        let target_name = pieces.next().unwrap_or(export_name);
        if !export_name.is_empty()
            && export_name == target_name
            && leading_js_identifier(export_name).as_deref() == Some(export_name)
        {
            names.push(export_name.to_string());
        }
    }

    names.sort();
    names.dedup();
    names
}

fn ts_commonjs_export_name(node: Node<'_>, source: &str) -> Option<String> {
    if node.kind() != "member_expression" {
        return None;
    }
    let object = node.child_by_field_name("object")?;
    let property = node.child_by_field_name("property")?;
    let property_name = node_text(source, property).trim().to_string();
    if property_name.is_empty() {
        return None;
    }

    if object.kind() == "identifier" {
        let object_name = node_text(source, object).trim();
        if object_name == "module" && property_name == "exports" {
            return Some("default".to_string());
        }
        if object_name == "exports" {
            return Some(property_name);
        }
    }

    if object.kind() == "member_expression" {
        let inner_object = object.child_by_field_name("object")?;
        let inner_property = object.child_by_field_name("property")?;
        if inner_object.kind() == "identifier"
            && node_text(source, inner_object).trim() == "module"
            && node_text(source, inner_property).trim() == "exports"
        {
            return Some(property_name);
        }
    }

    None
}

fn ts_commonjs_symbol_from_expression(
    node: Node<'_>,
    source: &str,
    export_name: &str,
) -> Option<ExtractedSymbol> {
    let kind = match node.kind() {
        "function_expression" | "generator_function" => "function",
        "class" => "class",
        _ => return None,
    };

    let name = node
        .child_by_field_name("name")
        .map(|name_node| node_text(source, name_node).trim().to_string())
        .filter(|name| !name.is_empty())
        .or_else(|| {
            if export_name != "default" {
                Some(export_name.to_string())
            } else {
                None
            }
        })?;

    Some(make_extracted_symbol(name, kind, true, &[], None, node))
}

fn ts_import_bindings(node: Node<'_>, source: &str) -> Vec<ImportBinding> {
    let mut bindings = Vec::new();
    let mut stack = vec![node];

    while let Some(current) = stack.pop() {
        if current.kind() == "import_clause" {
            let mut cursor = current.walk();
            for child in current.named_children(&mut cursor) {
                if child.kind() == "identifier" {
                    let local_name = node_text(source, child).trim().to_string();
                    if !local_name.is_empty() {
                        bindings.push(ImportBinding::new("default", local_name));
                    }
                } else {
                    stack.push(child);
                }
            }
            continue;
        }

        if current.kind() == "import_specifier" {
            if let Some(name_node) = current.child_by_field_name("name") {
                let source_name = node_text(source, name_node).trim().to_string();
                if !source_name.is_empty() {
                    let local_name = current
                        .child_by_field_name("alias")
                        .map(|alias_node| node_text(source, alias_node).trim().to_string())
                        .filter(|alias| !alias.is_empty())
                        .unwrap_or_else(|| source_name.clone());
                    bindings.push(ImportBinding::new(source_name, local_name));
                }
            }
            continue;
        }

        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }

    bindings.sort();
    bindings.dedup();
    bindings
}

fn ts_namespace_import_aliases(node: Node<'_>, source: &str) -> Vec<String> {
    let mut aliases = Vec::new();
    let mut stack = vec![node];

    while let Some(current) = stack.pop() {
        if current.kind() == "namespace_import" {
            let mut cursor = current.walk();
            for child in current.named_children(&mut cursor) {
                let alias = node_text(source, child).trim().to_string();
                if !alias.is_empty() {
                    aliases.push(alias);
                }
            }
            continue;
        }

        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }

    aliases.sort();
    aliases.dedup();
    aliases
}

fn ts_aliases_from_variable_statement(
    node: Node<'_>,
    source: &str,
    owner_identity: Option<&str>,
) -> Vec<ExtractedAlias> {
    let mut aliases = Vec::new();
    let mut stack = vec![node];

    while let Some(current) = stack.pop() {
        if current.kind() == "variable_declarator" {
            let Some(name_node) = current.child_by_field_name("name") else {
                continue;
            };
            let Some(value_node) = current.child_by_field_name("value") else {
                continue;
            };
            if ts_require_module_from_value(value_node, source).is_some() {
                continue;
            }
            if name_node.kind() != "identifier" {
                continue;
            }

            let alias_name = node_text(source, name_node).trim().to_string();
            if alias_name.is_empty() {
                continue;
            }

            let (target_expr, target_name) =
                ts_alias_reference(value_node, source).unwrap_or_else(|| (String::new(), String::new()));
            aliases.push(ExtractedAlias::new(
                alias_name,
                target_expr,
                target_name,
                owner_identity,
            ));
            continue;
        }

        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }

    aliases
}

fn ts_alias_reference(node: Node<'_>, source: &str) -> Option<(String, String)> {
    match node.kind() {
        "identifier" => {
            let text = node_text(source, node).trim().to_string();
            if text.is_empty() {
                None
            } else {
                Some((text.clone(), text))
            }
        }
        "member_expression" => {
            let object = node.child_by_field_name("object")?;
            let property = node.child_by_field_name("property")?;
            if object.kind() != "identifier" {
                return None;
            }
            let object_name = node_text(source, object).trim().to_string();
            let property_name = node_text(source, property).trim().to_string();
            if object_name.is_empty() || property_name.is_empty() {
                None
            } else {
                Some((format!("{object_name}.{property_name}"), property_name))
            }
        }
        _ => None,
    }
}

fn ts_reexport_bindings(node: Node<'_>, source: &str) -> Vec<ImportBinding> {
    let mut bindings = Vec::new();
    let mut stack = vec![node];

    while let Some(current) = stack.pop() {
        if current.kind() == "export_specifier" {
            if let Some(name_node) = current.child_by_field_name("name") {
                let source_name = node_text(source, name_node).trim().to_string();
                if !source_name.is_empty() {
                    let local_name = current
                        .child_by_field_name("alias")
                        .map(|alias_node| node_text(source, alias_node).trim().to_string())
                        .filter(|alias| !alias.is_empty())
                        .unwrap_or_else(|| source_name.clone());
                    bindings.push(ImportBinding::new(source_name, local_name));
                }
            }
            continue;
        }

        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }

    bindings.sort();
    bindings.dedup();
    bindings
}

fn ts_is_wildcard_reexport(node: Node<'_>, source: &str) -> bool {
    let text = node_text(source, node);
    text.contains("export *") && !text.contains('{')
}

fn mark_exported_symbols(symbols: &mut [ExtractedSymbol], exported_names: &BTreeSet<String>) {
    if exported_names.is_empty() {
        return;
    }

    for symbol in symbols.iter_mut() {
        if symbol.parent_identity.is_none() && exported_names.contains(&symbol.name) {
            symbol.exported = true;
        }
    }
}

fn collect_ts_local_export_names(
    node: Node<'_>,
    source: &str,
    exported_names: &mut BTreeSet<String>,
) {
    if node.child_by_field_name("source").is_some() {
        return;
    }

    for name in ts_local_export_names_from_text(node_text(source, node)) {
        exported_names.insert(name);
    }
}

fn ts_local_export_names_from_text(text: &str) -> Vec<String> {
    let trimmed = text.trim().trim_end_matches(';').trim();
    let mut names = Vec::new();

    if let Some(rest) = trimmed.strip_prefix("export default ") {
        let rest = rest.trim();
        if !matches!(rest.split_whitespace().next(), Some("function" | "class" | "async")) {
            if let Some(name) = leading_js_identifier(rest) {
                names.push(name);
            }
        }
    }

    if let (Some(open), Some(close)) = (trimmed.find('{'), trimmed.rfind('}')) {
        if close > open {
            for item in split_top_level_commas(&trimmed[open + 1..close]) {
                let local = item
                    .split_once(" as ")
                    .map(|(left, _)| left)
                    .unwrap_or(item.as_str())
                    .trim();
                let local = local.strip_prefix("type ").unwrap_or(local).trim();
                if let Some(name) = leading_js_identifier(local) {
                    names.push(name);
                }
            }
        }
    }

    names
}

fn ts_default_export_names_from_text(text: &str) -> Vec<String> {
    let trimmed = text.trim().trim_end_matches(';').trim();
    let Some(rest) = trimmed.strip_prefix("export default ") else {
        return Vec::new();
    };
    let rest = rest.trim();

    let candidate = if let Some(rest) = rest.strip_prefix("async function ") {
        leading_js_identifier(rest)
    } else if let Some(rest) = rest.strip_prefix("function ") {
        leading_js_identifier(rest)
    } else if let Some(rest) = rest.strip_prefix("class ") {
        leading_js_identifier(rest)
    } else {
        leading_js_identifier(rest)
    };

    candidate.into_iter().collect()
}

fn extend_unique_block_ids<I>(target: &mut Vec<BlockId>, ids: I) -> bool
where
    I: IntoIterator<Item = BlockId>,
{
    let before = target.len();
    for id in ids {
        if !target.contains(&id) {
            target.push(id);
        }
    }
    target.len() != before
}

fn leading_js_identifier(text: &str) -> Option<String> {
    let mut chars = text.chars();
    let first = chars.next()?;
    if !(first == '_' || first == '$' || first.is_ascii_alphabetic()) {
        return None;
    }

    let mut ident = String::from(first);
    for ch in chars {
        if ch == '_' || ch == '$' || ch.is_ascii_alphanumeric() {
            ident.push(ch);
        } else {
            break;
        }
    }

    Some(ident)
}

fn simple_symbol_reference_name(text: &str) -> Option<String> {
    let trimmed = text.trim();
    let trimmed = trimmed.strip_prefix("readonly ").unwrap_or(trimmed).trim();
    let name = leading_js_identifier(trimmed)?;
    let rest = trimmed[name.len()..].trim_start();
    if rest.is_empty() || rest.starts_with('<') || rest.starts_with('[') || rest.starts_with('?')
    {
        Some(name)
    } else {
        None
    }
}

fn ts_public_field_kind(node: Node<'_>) -> &'static str {
    node.child_by_field_name("value")
        .map(|value| {
            if is_ts_function_like_kind(value.kind()) {
                "method"
            } else {
                "field"
            }
        })
        .unwrap_or("field")
}

fn ts_variable_symbol_kind(node: Node<'_>) -> &'static str {
    node.child_by_field_name("value")
        .map(|value| match value.kind() {
            kind if is_ts_function_like_kind(kind) => "function",
            "class" => "class",
            _ => "variable",
        })
        .unwrap_or("variable")
}

fn is_ts_function_like_kind(kind: &str) -> bool {
    matches!(kind, "arrow_function" | "function_expression" | "generator_function")
}

fn extract_ts_module_from_text(text: &str) -> Option<String> {
    let patterns = [
        Regex::new(r#"(?i)\bfrom\s+['"]([^'"]+)['"]"#).ok()?,
        Regex::new(r#"(?i)\bimport\s+['"]([^'"]+)['"]"#).ok()?,
        Regex::new(r#"(?i)require\(\s*['"]([^'"]+)['"]\s*\)"#).ok()?,
    ];
    for pattern in patterns {
        if let Some(caps) = pattern.captures(text) {
            if let Some(module) = caps.get(1).map(|m| m.as_str().trim()) {
                if !module.is_empty() {
                    return Some(module.to_string());
                }
            }
        }
    }
    None
}

fn node_text<'a>(source: &'a str, node: Node<'_>) -> &'a str {
    let start = node.start_byte().min(source.len());
    let end = node.end_byte().min(source.len());
    &source[start..end]
}

fn node_span(node: Node<'_>) -> (usize, usize, usize, usize) {
    let start = node.start_position();
    let end = node.end_position();
    (start.row + 1, start.column + 1, end.row + 1, end.column + 1)
}

fn first_named_identifier(node: Node<'_>, source: &str) -> Option<String> {
    let mut stack = vec![node];
    while let Some(current) = stack.pop() {
        if matches!(current.kind(), "identifier" | "type_identifier") {
            let text = node_text(source, current).trim().to_string();
            if !text.is_empty() {
                return Some(text);
            }
        }

        let mut cursor = current.walk();
        for child in current.named_children(&mut cursor) {
            stack.push(child);
        }
    }
    None
}

fn make_extracted_symbol(
    name: String,
    kind: &str,
    exported: bool,
    scope: &[String],
    parent_identity: Option<&str>,
    node: Node<'_>,
) -> ExtractedSymbol {
    let qualified_name = qualify_symbol_name(scope, &name);
    let (start_line, start_col, end_line, end_col) = node_span(node);

    ExtractedSymbol {
        name,
        qualified_name: qualified_name.clone(),
        identity: format!("{}@{}:{}", qualified_name, start_line, start_col),
        parent_identity: parent_identity.map(|s| s.to_string()),
        kind: kind.to_string(),
        exported,
        start_line,
        start_col,
        end_line,
        end_col,
    }
}

fn qualify_symbol_name(scope: &[String], name: &str) -> String {
    if scope.is_empty() {
        name.to_string()
    } else {
        format!("{}::{}", scope.join("::"), name)
    }
}

fn compare_extracted_symbols(a: &ExtractedSymbol, b: &ExtractedSymbol) -> std::cmp::Ordering {
    a.start_line
        .cmp(&b.start_line)
        .then_with(|| a.start_col.cmp(&b.start_col))
        .then_with(|| b.end_line.cmp(&a.end_line))
        .then_with(|| b.end_col.cmp(&a.end_col))
        .then_with(|| a.qualified_name.cmp(&b.qualified_name))
}

fn resolve_relationship_target_ids(
    source_file: &str,
    language: CodeLanguage,
    relationship: &ExtractedRelationship,
    top_level_symbol_ids: &BTreeMap<(String, String), Vec<BlockId>>,
    imported_symbol_targets_by_file: &BTreeMap<String, BTreeMap<String, Vec<BlockId>>>,
    known_files: &BTreeSet<String>,
) -> Vec<BlockId> {
    let mut target_ids = Vec::new();

    if let Some(local_ids) = top_level_symbol_ids
        .get(&(source_file.to_string(), relationship.target_name.clone()))
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

fn resolve_alias_target_ids(
    source_file: &str,
    language: CodeLanguage,
    alias: &ExtractedAlias,
    top_level_symbol_ids: &BTreeMap<(String, String), Vec<BlockId>>,
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

    if let Some(local_ids) = top_level_symbol_ids.get(&(source_file.to_string(), alias.target_name.clone())) {
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
                if let Some(ids) = top_level_symbol_ids.get(&(target_file.clone(), member_name.clone())) {
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
                    if let ImportResolution::Resolved(target_file) = resolve_import(
                        source_file,
                        &language,
                        &expanded_expr,
                        known_files,
                    ) {
                        if let Some(name) = rust_last_path_segment(&expanded_expr) {
                            if let Some(ids) = top_level_symbol_ids.get(&(target_file, name)) {
                                target_ids.extend(ids.iter().copied());
                            }
                        }
                    }
                }
            }
        }

        if let ImportResolution::Resolved(target_file) = resolve_import(
            source_file,
            &language,
            &alias.target_expr,
            known_files,
        ) {
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

fn resolve_usage_target_ids(
    source_file: &str,
    language: CodeLanguage,
    usage: &ExtractedUsage,
    top_level_symbol_ids: &BTreeMap<(String, String), Vec<BlockId>>,
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

    if let Some(local_ids) = top_level_symbol_ids.get(&(source_file.to_string(), usage.target_name.clone())) {
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
            if let Some(ids) = top_level_symbol_ids.get(&(target_file, member_name.clone())) {
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
                    if let ImportResolution::Resolved(target_file) = resolve_import(
                        source_file,
                        &language,
                        &expanded_expr,
                        known_files,
                    ) {
                        if let Some(name) = rust_last_path_segment(&expanded_expr) {
                            if let Some(ids) = top_level_symbol_ids.get(&(target_file, name)) {
                                target_ids.extend(ids.iter().copied());
                            }
                        }
                    }
                }
            }
        }

        if let ImportResolution::Resolved(target_file) = resolve_import(
            source_file,
            &language,
            &usage.target_expr,
            known_files,
        ) {
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

fn member_usage_parts(text: &str) -> Option<(String, String)> {
    let (left, right) = text.split_once('.')?;
    let left = left.trim();
    let right = right.trim();
    if left.is_empty() || right.is_empty() {
        None
    } else {
        Some((left.to_string(), right.to_string()))
    }
}

fn resolve_module_alias_target_files(
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

fn resolve_module_alias_target_files_recursive(
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

fn rust_alias_path_parts(text: &str) -> Option<(String, String)> {
    let (left, right) = text.split_once("::")?;
    let left = left.trim();
    let right = right.trim();
    if left.is_empty() || right.is_empty() {
        None
    } else {
        Some((left.to_string(), right.to_string()))
    }
}

fn alias_scope_key(owner_identity: Option<&str>) -> String {
    owner_identity.unwrap_or("").to_string()
}

fn resolve_import(
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

fn resolve_ts_import(
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

fn ts_candidates(base: &str) -> Vec<String> {
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

fn resolve_python_import(
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

fn py_candidates(base: &str) -> Vec<String> {
    if base.is_empty() {
        return Vec::new();
    }

    if base.ends_with(".py") {
        return vec![base.to_string()];
    }

    vec![format!("{}.py", base), format!("{}/__init__.py", base)]
}

fn resolve_rust_import(
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

    let first_segment = path_segments.first().map(|s| s.as_str()).unwrap_or_default();
    if rust_root_module_exists(&crate_root, first_segment, known_files) {
        ImportResolution::Unresolved
    } else {
        ImportResolution::External
    }
}

fn find_known_candidate<I>(candidates: I, known_files: &BTreeSet<String>) -> Option<String>
where
    I: IntoIterator<Item = String>,
{
    candidates.into_iter().find(|candidate| known_files.contains(candidate))
}

fn rust_crate_entry_file(crate_root: &str, known_files: &BTreeSet<String>) -> Option<String> {
    find_known_candidate(
        [
            format!("{}/lib.rs", crate_root),
            format!("{}/main.rs", crate_root),
            format!("{}/mod.rs", crate_root),
        ],
        known_files,
    )
}

fn rust_module_root(source_file: &str) -> String {
    let parts: Vec<&str> = source_file.split('/').collect();
    if let Some((index, _)) = parts.iter().enumerate().rfind(|(_, part)| **part == "src") {
        return parts[..=index].join("/");
    }

    parent_directory(source_file)
}

fn rust_root_module_exists(crate_root: &str, segment: &str, known_files: &BTreeSet<String>) -> bool {
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
fn expand_rust_use_declaration(text: &str) -> Vec<String> {
    expand_rust_use_declaration_items(text)
        .into_iter()
        .map(|(module, _, _)| module)
        .collect()
}

fn expand_rust_use_declaration_items(text: &str) -> Vec<(String, Option<String>, bool)> {
    let trimmed = text.trim();
    let Some(use_index) = trimmed.find("use ") else {
        return Vec::new();
    };

    let expr = trimmed[use_index + 4..].trim().trim_end_matches(';').trim();
    expand_rust_use_tree_items("", expr)
}

fn expand_rust_use_tree_items(prefix: &str, expr: &str) -> Vec<(String, Option<String>, bool)> {
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

fn split_top_level_commas(input: &str) -> Vec<String> {
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

fn find_top_level_char(input: &str, needle: char) -> Option<usize> {
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

fn matching_brace_index(input: &str, open_idx: usize) -> Option<usize> {
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

fn join_rust_use_prefix(prefix: &str, segment: &str) -> String {
    let clean_prefix = prefix.trim().trim_end_matches("::").trim_start_matches("::");
    let clean_segment = segment.trim().trim_end_matches("::").trim_start_matches("::");

    if clean_prefix.is_empty() {
        clean_segment.to_string()
    } else if clean_segment.is_empty() {
        clean_prefix.to_string()
    } else {
        format!("{}::{}", clean_prefix, clean_segment)
    }
}

fn has_known_extension(path: &str, exts: &[&str]) -> bool {
    exts.iter().any(|ext| path.ends_with(&format!(".{}", ext)))
}

fn normalize_temporal_fields(doc: &mut Document) {
    let ts = deterministic_timestamp();
    doc.metadata.created_at = ts;
    doc.metadata.modified_at = ts;
    doc.version.timestamp = ts;

    for block in doc.blocks.values_mut() {
        block.metadata.created_at = ts;
        block.metadata.modified_at = ts;
        block.version.timestamp = ts;

        for edge in &mut block.edges {
            edge.created_at = ts;
        }
    }
}

fn deterministic_timestamp() -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc)
}

fn sort_structure_children_by_logical_key(doc: &mut Document) {
    let key_index = logical_key_index(doc);

    for children in doc.structure.values_mut() {
        children.sort_by(|a, b| {
            let ka = key_index.get(a).cloned().unwrap_or_else(|| a.to_string());
            let kb = key_index.get(b).cloned().unwrap_or_else(|| b.to_string());
            ka.cmp(&kb)
        });
    }
}

fn sort_edges(doc: &mut Document) {
    let key_index = logical_key_index(doc);

    for block in doc.blocks.values_mut() {
        block.edges.sort_by(|a, b| {
            let at = key_index
                .get(&a.target)
                .cloned()
                .unwrap_or_else(|| a.target.to_string());
            let bt = key_index
                .get(&b.target)
                .cloned()
                .unwrap_or_else(|| b.target.to_string());

            a.edge_type
                .as_str()
                .cmp(&b.edge_type.as_str())
                .then_with(|| at.cmp(&bt))
        });
    }
}

fn compute_stats(doc: &Document) -> CodeGraphStats {
    let mut stats = CodeGraphStats::default();

    for (id, block) in &doc.blocks {
        if *id == doc.root {
            continue;
        }

        stats.total_nodes += 1;

        match node_class(block).as_deref() {
            Some("repository") => stats.repository_nodes += 1,
            Some("directory") => stats.directory_nodes += 1,
            Some("file") => {
                stats.file_nodes += 1;
                if let Some(lang) = block
                    .metadata
                    .custom
                    .get(META_LANGUAGE)
                    .and_then(|v| v.as_str())
                {
                    *stats.languages.entry(lang.to_string()).or_default() += 1;
                }
            }
            Some("symbol") => stats.symbol_nodes += 1,
            _ => {}
        }

        for edge in &block.edges {
            stats.total_edges += 1;
            match &edge.edge_type {
                EdgeType::References => stats.reference_edges += 1,
                EdgeType::Custom(name) if name == "exports" => stats.export_edges += 1,
                _ => {}
            }
        }
    }

    stats
}

fn block_logical_key(block: &Block) -> Option<String> {
    block
        .metadata
        .custom
        .get(META_LOGICAL_KEY)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn block_path(block: &Block) -> Option<String> {
    block
        .metadata
        .custom
        .get(META_PATH)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn node_class(block: &Block) -> Option<String> {
    if let Some(class) = block
        .metadata
        .custom
        .get(META_NODE_CLASS)
        .and_then(|v| v.as_str())
    {
        return Some(class.to_string());
    }

    if let Some(role) = &block.metadata.semantic_role {
        if role.category == ucm_core::RoleCategory::Custom {
            if let Some(sub) = &role.subcategory {
                return Some(sub.to_string());
            }
        }
    }

    None
}

fn validate_required_metadata(
    class_name: &str,
    block: &Block,
    diagnostics: &mut Vec<CodeGraphDiagnostic>,
) {
    let required = match class_name {
        "repository" => vec![META_LOGICAL_KEY],
        "directory" => vec![META_LOGICAL_KEY, META_PATH],
        "file" => vec![META_LOGICAL_KEY, META_PATH, META_LANGUAGE],
        "symbol" => vec![
            META_LOGICAL_KEY,
            META_PATH,
            META_LANGUAGE,
            META_SYMBOL_KIND,
            META_SYMBOL_NAME,
            META_SPAN,
            META_EXPORTED,
        ],
        _ => {
            diagnostics.push(CodeGraphDiagnostic::error(
                "CG1017",
                format!("invalid node_class '{}'", class_name),
            ));
            return;
        }
    };

    for key in required {
        if !block.metadata.custom.contains_key(key) {
            diagnostics.push(
                CodeGraphDiagnostic::error(
                    "CG1018",
                    format!(
                        "node class '{}' missing required metadata key '{}'",
                        class_name, key
                    ),
                )
                .with_logical_key(block_logical_key(block).unwrap_or_else(|| block.id.to_string())),
            );
        }
    }

    if let Some(logical_key) = block_logical_key(block) {
        let expected_prefix = match class_name {
            "repository" => "repository:",
            "directory" => "directory:",
            "file" => "file:",
            "symbol" => "symbol:",
            _ => "",
        };

        if !expected_prefix.is_empty() && !logical_key.starts_with(expected_prefix) {
            diagnostics.push(
                CodeGraphDiagnostic::error(
                    "CG1019",
                    format!(
                        "logical_key '{}' must start with '{}'",
                        logical_key, expected_prefix
                    ),
                )
                .with_logical_key(logical_key),
            );
        }
    }
}

fn logical_key_index(doc: &Document) -> HashMap<BlockId, String> {
    doc.blocks
        .iter()
        .map(|(id, block)| {
            (
                *id,
                block_logical_key(block).unwrap_or_else(|| id.to_string()),
            )
        })
        .collect()
}

fn normalized_document_metadata(doc: &Document) -> serde_json::Value {
    let mut custom = serde_json::Map::new();
    let mut custom_entries: Vec<_> = doc.metadata.custom.iter().collect();
    custom_entries.sort_by(|a, b| a.0.cmp(b.0));
    for (k, v) in custom_entries {
        if is_volatile_metadata_key(k) {
            continue;
        }
        custom.insert(k.clone(), v.clone());
    }

    json!({
        "title": doc.metadata.title,
        "description": doc.metadata.description,
        "authors": doc.metadata.authors,
        "language": doc.metadata.language,
        "custom": custom,
    })
}

fn normalized_block_metadata(block: &Block) -> serde_json::Value {
    let mut custom = serde_json::Map::new();
    let mut entries: Vec<_> = block.metadata.custom.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));
    for (k, v) in entries {
        if is_volatile_metadata_key(k) {
            continue;
        }
        custom.insert(k.clone(), v.clone());
    }

    json!({
        "label": block.metadata.label,
        "semantic_role": block.metadata.semantic_role.as_ref().map(|r| r.to_string()),
        "tags": block.metadata.tags,
        "summary": block.metadata.summary,
        "custom": custom,
    })
}

fn normalized_edge_metadata(edge: &Edge) -> serde_json::Value {
    let mut custom = serde_json::Map::new();
    let mut entries: Vec<_> = edge.metadata.custom.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));
    for (k, v) in entries {
        if is_volatile_metadata_key(k) {
            continue;
        }
        custom.insert(k.clone(), v.clone());
    }

    json!({
        "confidence": edge.metadata.confidence,
        "description": edge.metadata.description,
        "custom": custom,
    })
}

fn is_volatile_metadata_key(key: &str) -> bool {
    matches!(key, "generated_at" | "runtime" | "session" | "timestamp")
}

fn collect_repository_files(
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
fn collect_repository_files_recursive(
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

fn extension_language(ext: &str) -> Option<CodeLanguage> {
    match ext {
        "rs" => Some(CodeLanguage::Rust),
        "py" => Some(CodeLanguage::Python),
        "ts" | "tsx" => Some(CodeLanguage::TypeScript),
        "js" | "jsx" => Some(CodeLanguage::JavaScript),
        _ => None,
    }
}

fn unique_symbol_logical_key(
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

fn ancestor_directories(path: &str) -> Vec<String> {
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

fn parent_directory_id(dir: &str, directory_ids: &BTreeMap<String, BlockId>) -> Option<BlockId> {
    let parent = parent_directory(dir);
    if parent.is_empty() {
        None
    } else {
        directory_ids.get(&parent).copied()
    }
}

fn parent_id_for_file(
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

fn parent_directory(path: &str) -> String {
    match path.rsplit_once('/') {
        Some((parent, _)) => parent.to_string(),
        None => String::new(),
    }
}

fn normalize_relative_join(base: &str, relative: &str) -> String {
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

fn ascend_directory(path: &str, levels: usize) -> String {
    let mut parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    for _ in 0..levels {
        if parts.is_empty() {
            break;
        }
        parts.pop();
    }
    parts.join("/")
}

fn sanitize_identifier(raw: &str) -> String {
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

fn normalize_path(path: &Path) -> String {
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

fn is_hidden_path(path: &str) -> bool {
    path.split('/').any(|part| part.starts_with('.'))
}

#[derive(Debug, Clone)]
struct GitignoreMatcher {
    rules: Vec<GitignoreRule>,
}

#[derive(Debug, Clone)]
struct GitignoreRule {
    regex: Regex,
    directory_only: bool,
}

impl GitignoreMatcher {
    fn from_repository(repo_root: &Path) -> Result<Self> {
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

    fn is_ignored(&self, rel_path: &str, is_dir: bool) -> bool {
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
    fn from_pattern(pattern: &str) -> Option<Self> {
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

fn glob_to_regex(glob: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::io::Write;
    use tempfile::tempdir;

    fn symbol_logical_keys(doc: &Document) -> Vec<String> {
        let mut out: Vec<_> = doc
            .blocks
            .values()
            .filter(|block| node_class(block).as_deref() == Some("symbol"))
            .filter_map(block_logical_key)
            .collect();
        out.sort();
        out
    }

    fn logical_key_to_block_id(doc: &Document) -> BTreeMap<String, BlockId> {
        doc.blocks
            .iter()
            .filter_map(|(id, block)| block_logical_key(block).map(|key| (key, *id)))
            .collect()
    }

    fn symbol_block_by_prefix<'a>(doc: &'a Document, prefix: &str) -> Option<&'a Block> {
        doc.blocks
            .values()
            .filter(|block| node_class(block).as_deref() == Some("symbol"))
            .filter_map(|block| block_logical_key(block).map(|key| (key, block)))
            .find(|(key, _)| *key == prefix)
            .map(|(_, block)| block)
            .or_else(|| {
                doc.blocks.values().find(|block| {
                    node_class(block).as_deref() == Some("symbol")
                        && block_logical_key(block)
                            .map(|key| key.starts_with(prefix))
                            .unwrap_or(false)
                })
            })
    }

    fn symbol_exported(doc: &Document, prefix: &str) -> bool {
        symbol_block_by_prefix(doc, prefix)
            .and_then(|block| block.metadata.custom.get(META_EXPORTED))
            .and_then(|value| value.as_bool())
            .unwrap_or(false)
    }

    fn symbol_kind(doc: &Document, prefix: &str) -> Option<String> {
        symbol_block_by_prefix(doc, prefix)
            .and_then(|block| block.metadata.custom.get(META_SYMBOL_KIND))
            .and_then(|value| value.as_str())
            .map(|value| value.to_string())
    }

    fn file_block_by_key<'a>(doc: &'a Document, logical_key: &str) -> Option<&'a Block> {
        doc.blocks.values().find(|block| {
            node_class(block).as_deref() == Some("file")
                && block_logical_key(block).as_deref() == Some(logical_key)
        })
    }

    fn block_logical_key_by_id(doc: &Document, block_id: BlockId) -> Option<String> {
        doc.blocks.get(&block_id).and_then(block_logical_key)
    }

    fn file_has_edge_to_symbol(
        doc: &Document,
        file_key: &str,
        edge_type: &str,
        relation: &str,
        symbol_prefix: &str,
    ) -> bool {
        let Some(block) = file_block_by_key(doc, file_key) else {
            return false;
        };

        block.edges.iter().any(|edge| {
            edge_type_name(&edge.edge_type) == edge_type
                && edge
                    .metadata
                    .custom
                    .get("relation")
                    .and_then(|value| value.as_str())
                    == Some(relation)
                && block_logical_key_by_id(doc, edge.target)
                    .map(|key| key.starts_with(symbol_prefix))
                    .unwrap_or(false)
        })
    }

    fn symbol_has_edge_to_symbol(
        doc: &Document,
        source_prefix: &str,
        edge_type: &str,
        relation: &str,
        target_prefix: &str,
    ) -> bool {
        let Some(block) = symbol_block_by_prefix(doc, source_prefix) else {
            return false;
        };

        block.edges.iter().any(|edge| {
            edge_type_name(&edge.edge_type) == edge_type
                && edge
                    .metadata
                    .custom
                    .get("relation")
                    .and_then(|value| value.as_str())
                    == Some(relation)
                && block_logical_key_by_id(doc, edge.target)
                    .map(|key| key.starts_with(target_prefix))
                    .unwrap_or(false)
        })
    }

    fn edge_type_name(edge_type: &EdgeType) -> String {
        match edge_type {
            EdgeType::References => "references".to_string(),
            EdgeType::Custom(name) => name.clone(),
            other => format!("{other:?}"),
        }
    }

    #[test]
    fn test_validate_profile_detects_missing_markers() {
        let doc = Document::create();
        let result = validate_code_graph_profile(&doc);
        assert!(!result.valid);
        assert!(result
            .diagnostics
            .iter()
            .any(|d| d.code == "CG1001" || d.code == "CG1002"));
    }

    #[test]
    fn test_canonical_fingerprint_stable_for_equivalent_docs() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/lib.rs"), "pub fn a() {}\n").unwrap();

        let input = CodeGraphBuildInput {
            repository_path: root.to_path_buf(),
            commit_hash: "abc123".to_string(),
            config: CodeGraphExtractorConfig::default(),
        };

        let first = build_code_graph(&input).unwrap();
        let second = build_code_graph(&input).unwrap();

        assert_eq!(first.canonical_fingerprint, second.canonical_fingerprint);
        assert_eq!(
            canonical_codegraph_json(&first.document).unwrap(),
            canonical_codegraph_json(&second.document).unwrap()
        );
    }

    #[test]
    fn test_portable_document_roundtrip_preserves_fingerprint() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("pkg")).unwrap();
        fs::write(
            dir.path().join("pkg/main.py"),
            "from .util import helper\n\ndef run():\n    return helper()\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("pkg/util.py"),
            "def helper():\n    return 1\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "def456".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        let portable = PortableDocument::from_document(&build.document);
        let json = serde_json::to_string_pretty(&portable).unwrap();
        let decoded: PortableDocument = serde_json::from_str(&json).unwrap();
        let roundtripped = decoded.to_document().unwrap();

        let fp1 = canonical_fingerprint(&build.document).unwrap();
        let fp2 = canonical_fingerprint(&roundtripped).unwrap();
        assert_eq!(fp1, fp2);
    }

    #[test]
    fn test_unresolved_import_produces_diagnostic() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::write(
            dir.path().join("src/lib.rs"),
            "use crate::missing::thing;\npub fn keep() {}\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "ghi789".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(build
            .diagnostics
            .iter()
            .any(|d| d.code == "CG2006" && d.severity == CodeGraphSeverity::Warning));
    }

    #[test]
    fn test_gitignore_rule_matches() {
        let rule = GitignoreRule::from_pattern("target/").unwrap();
        assert!(rule.regex.is_match("target"));
        assert!(rule.regex.is_match("target/debug/app"));
    }

    #[test]
    fn test_import_resolution_ts_relative() {
        let mut known = BTreeSet::new();
        known.insert("src/main.ts".to_string());
        known.insert("src/util.ts".to_string());

        let resolved = resolve_ts_import("src/main.ts", "./util", &known);
        assert_eq!(resolved, ImportResolution::Resolved("src/util.ts".to_string()));
    }

    #[test]
    fn test_rust_use_group_expansion() {
        let imports = expand_rust_use_declaration(
            "use crate::{block::{Block, BlockState}, edge::Edge, util::{self, helper as helper_fn}};",
        );

        assert_eq!(
            imports,
            vec![
                "crate::block::Block".to_string(),
                "crate::block::BlockState".to_string(),
                "crate::edge::Edge".to_string(),
                "crate::util".to_string(),
                "crate::util::helper".to_string(),
            ]
        );
    }

    #[test]
    fn test_workspace_rust_imports_resolve_without_external_warnings() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("crates/demo/src")).unwrap();
        fs::write(
            dir.path().join("crates/demo/src/lib.rs"),
            "use anyhow::Result;\nuse crate::block::{helper, Thing};\npub fn run() -> Result<()> { helper(); let _ = Thing; Ok(()) }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("crates/demo/src/block.rs"),
            "pub struct Thing;\npub fn helper() {}\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "workspace-imports".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(!build.diagnostics.iter().any(|d| d.code == "CG2006"));
        assert!(build.stats.reference_edges >= 1);
    }

    #[test]
    fn test_rust_crate_root_symbol_imports_resolve_to_entry_file() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::write(
            dir.path().join("src/lib.rs"),
            "pub struct Document;\npub type Result<T> = std::result::Result<T, ()>;\nmod inner;\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("src/inner.rs"),
            "use crate::Document;\nuse crate::Result;\npub fn run() {}\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "crate-root-symbols".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(!build.diagnostics.iter().any(|d| d.code == "CG2006"));
        assert!(build.stats.reference_edges >= 1);
    }

    #[test]
    fn test_python_relative_module_import_and_symbol_edges_are_captured() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("pkg")).unwrap();
        fs::write(dir.path().join("pkg/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("pkg/mod.py"),
            "from . import helper\nfrom .helper import helper as helper_fn\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "python-relative-imports".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(!build.diagnostics.iter().any(|d| d.code == "CG2006"));
        assert!(file_has_edge_to_symbol(
            &build.document,
            "file:pkg/mod.py",
            "imports_symbol",
            "imports_symbol",
            "symbol:pkg/helper.py::helper",
        ));
    }

    #[test]
    fn test_python_all_marks_reexported_package_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("pkg")).unwrap();
        fs::write(dir.path().join("pkg/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("pkg/__init__.py"),
            "from .helper import helper\n__all__ = [\"helper\"]\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "python-all-reexports".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(file_has_edge_to_symbol(
            &build.document,
            "file:pkg/__init__.py",
            "exports",
            "reexports",
            "symbol:pkg/helper.py::helper",
        ));
    }

    #[test]
    fn test_rust_and_ts_reexports_point_to_target_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::write(dir.path().join("src/helper.rs"), "pub fn helper() {}\n").unwrap();
        fs::write(
            dir.path().join("src/lib.rs"),
            "pub mod helper;\npub use helper::helper;\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/helper.ts"),
            "export function helper() { return 1; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/mod.ts"),
            "export { helper } from './helper';\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "reexports".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(file_has_edge_to_symbol(
            &build.document,
            "file:src/lib.rs",
            "exports",
            "reexports",
            "symbol:src/helper.rs::helper",
        ));
        assert!(file_has_edge_to_symbol(
            &build.document,
            "file:web/mod.ts",
            "exports",
            "reexports",
            "symbol:web/helper.ts::helper",
        ));
        assert!(file_has_edge_to_symbol(
            &build.document,
            "file:web/mod.ts",
            "imports_symbol",
            "imports_symbol",
            "symbol:web/helper.ts::helper",
        ));
    }

    #[test]
    fn test_wildcard_imports_and_reexports_expand_to_exported_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("pkg")).unwrap();

        fs::write(
            dir.path().join("src/helper.rs"),
            "pub fn a() {}\npub fn b() {}\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("src/lib.rs"),
            "pub mod helper;\npub use helper::*;\n",
        )
        .unwrap();

        fs::write(
            dir.path().join("web/helper.ts"),
            "export function a() { return 1; }\nexport const b = 2;\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/mod.ts"),
            "export * from './helper';\n",
        )
        .unwrap();

        fs::write(
            dir.path().join("pkg/helper.py"),
            "def a():\n    return 1\n\ndef _hidden():\n    return 2\n__all__ = [\"a\"]\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("pkg/__init__.py"),
            "from .helper import *\n__all__ = [\"a\"]\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "wildcard-semantics".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(file_has_edge_to_symbol(
            &build.document,
            "file:src/lib.rs",
            "imports_symbol",
            "imports_symbol",
            "symbol:src/helper.rs::a",
        ));
        assert!(file_has_edge_to_symbol(
            &build.document,
            "file:src/lib.rs",
            "exports",
            "reexports",
            "symbol:src/helper.rs::b",
        ));

        assert!(file_has_edge_to_symbol(
            &build.document,
            "file:web/mod.ts",
            "imports_symbol",
            "imports_symbol",
            "symbol:web/helper.ts::a",
        ));
        assert!(file_has_edge_to_symbol(
            &build.document,
            "file:web/mod.ts",
            "exports",
            "reexports",
            "symbol:web/helper.ts::b",
        ));

        assert!(file_has_edge_to_symbol(
            &build.document,
            "file:pkg/__init__.py",
            "imports_symbol",
            "imports_symbol",
            "symbol:pkg/helper.py::a",
        ));
        assert!(file_has_edge_to_symbol(
            &build.document,
            "file:pkg/__init__.py",
            "exports",
            "reexports",
            "symbol:pkg/helper.py::a",
        ));
        assert!(!file_has_edge_to_symbol(
            &build.document,
            "file:pkg/__init__.py",
            "exports",
            "reexports",
            "symbol:pkg/helper.py::_hidden",
        ));
    }

    #[test]
    fn test_explicit_type_relationship_edges_resolve_across_supported_languages() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/base.ts"),
            "export class Base {}\nexport interface Face {}\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/app.ts"),
            "import { Base as ImportedBase, Face as ImportedFace } from './base';\nclass Child extends ImportedBase implements ImportedFace {}\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/base.py"), "class Base:\n    pass\n").unwrap();
        fs::write(
            dir.path().join("py/app.py"),
            "from .base import Base as ImportedBase\nclass Child(ImportedBase):\n    pass\n",
        )
        .unwrap();

        fs::write(
            dir.path().join("src/base.rs"),
            "pub trait Face {}\npub struct Thing;\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("src/lib.rs"),
            "mod base;\nuse crate::base::Face;\nstruct Thing;\nimpl Face for Thing {}\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "type-relationships".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/app.ts::Child",
            "extends",
            "extends",
            "symbol:web/base.ts::Base",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/app.ts::Child",
            "implements",
            "implements",
            "symbol:web/base.ts::Face",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/app.py::Child",
            "extends",
            "extends",
            "symbol:py/base.py::Base",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::Thing#",
            "implements",
            "implements",
            "symbol:src/base.rs::Face",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::Thing#",
            "for_type",
            "for_type",
            "symbol:src/lib.rs::Thing",
        ));
    }

    #[test]
    fn test_call_sites_resolve_to_same_file_and_imported_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("src/util.rs"),
            "pub fn greet() -> String { \"hi\".to_string() }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("src/lib.rs"),
            "mod util;\npub fn run() -> String { util::greet() }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from .helper import helper\ndef execute():\n    return helper()\n",
        )
        .unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import { util } from './util';\nexport function run() { return util(); }\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "call-sites".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:src/util.rs::greet",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::execute",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
    }

    #[test]
    fn test_ts_constructor_aliases_resolve_to_class_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();

        fs::write(dir.path().join("web/thing.ts"), "export class Thing {}\n").unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import { Thing } from './thing';\nconst Ctor = Thing;\nexport function make() { return new Ctor(); }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/member.ts"),
            "import * as ns from './thing';\nconst First = ns.Thing;\nconst Second = First;\nexport function build() { return new Second(); }\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "ts-constructor-aliases".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::make",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/thing.ts::Thing",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/member.ts::build",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/thing.ts::Thing",
        ));
    }

    #[test]
    fn test_ts_namespace_and_module_alias_member_constructors_resolve_to_class_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();

        fs::write(dir.path().join("web/thing.ts"), "export class Thing {}\n").unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import * as ns from './thing';\nexport function direct() { return new ns.Thing(); }\nconst alias = ns;\nexport function top() { return new alias.Thing(); }\nexport function local() { const first = ns; const second = first; return new second.Thing(); }\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "ts-namespace-member-constructors".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::direct",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/thing.ts::Thing",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::top",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/thing.ts::Thing",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::local",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/thing.ts::Thing",
        ));
    }

    #[test]
    fn test_ts_default_import_calls_and_constructors_resolve_to_default_export_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export default function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/thing.ts"),
            "export default class Thing {}\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import util from './util';\nimport { default as util_spec } from './util';\nimport Thing from './thing';\nconst first = util;\nconst second = first;\nconst third = util_spec;\nconst Alias = Thing;\nexport function run() { return second(); }\nexport function run_spec() { return third(); }\nexport function make() { return new Alias(); }\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "ts-default-import-calls-and-constructors".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run_spec",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::make",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/thing.ts::Thing",
        ));
    }

    #[test]
    fn test_ts_default_import_shadowing_does_not_fall_back_and_anonymous_defaults_stay_unresolved() {
        let shadow_dir = tempdir().unwrap();
        fs::create_dir_all(shadow_dir.path().join("web")).unwrap();

        fs::write(
            shadow_dir.path().join("web/util.ts"),
            "export default function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            shadow_dir.path().join("web/main.ts"),
            "import util from './util';\nexport function missing_case() { const util = missing; return util(); }\nexport function expr_case() { const util = true ? missing : util; return util(); }\n",
        )
        .unwrap();

        let shadow_build = build_code_graph(&CodeGraphBuildInput {
            repository_path: shadow_dir.path().to_path_buf(),
            commit_hash: "ts-default-import-shadowing".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(!symbol_has_edge_to_symbol(
            &shadow_build.document,
            "symbol:web/main.ts::missing_case",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(!symbol_has_edge_to_symbol(
            &shadow_build.document,
            "symbol:web/main.ts::expr_case",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));

        let anon_dir = tempdir().unwrap();
        fs::create_dir_all(anon_dir.path().join("web")).unwrap();
        fs::write(
            anon_dir.path().join("web/util.ts"),
            "export default function() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            anon_dir.path().join("web/main.ts"),
            "import util from './util';\nexport function run() { return util(); }\n",
        )
        .unwrap();

        let anon_build = build_code_graph(&CodeGraphBuildInput {
            repository_path: anon_dir.path().to_path_buf(),
            commit_hash: "ts-anonymous-default-import".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(!symbol_has_edge_to_symbol(
            &anon_build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
    }

    #[test]
    fn test_ts_reexported_calls_and_constructors_resolve_to_underlying_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export default function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/thing.ts"),
            "export default class Thing {}\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/index.ts"),
            "export { default as util } from './util';\nexport { default as Thing } from './thing';\nexport * from './util';\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/index2.ts"),
            "export { util as util_chain } from './index';\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/default_index.ts"),
            "export * from './util';\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import { util, Thing } from './index';\nimport { util as util_wildcard } from './index';\nimport { util_chain } from './index2';\nimport util_default from './default_index';\nconst alias = util_chain;\nexport function run() { return util(); }\nexport function run_wildcard() { return util_wildcard(); }\nexport function run_chain() { return alias(); }\nexport function run_default_excluded() { return util_default(); }\nexport function make() { return new Thing(); }\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "ts-reexported-calls-and-constructors".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run_wildcard",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run_chain",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(!symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run_default_excluded",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::make",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/thing.ts::Thing",
        ));
    }

    #[test]
    fn test_js_commonjs_require_calls_resolve_to_underlying_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();

        fs::write(
            dir.path().join("web/default_util.js"),
            "module.exports = function util() { return 42; };\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/named_util.js"),
            "exports.greet = function greet() { return 42; };\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/object_util.js"),
            "function greet() { return 42; }\nmodule.exports = { greet };\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.js"),
            "const util = require('./default_util');\nconst { greet } = require('./named_util');\nconst named = require('./named_util');\nconst { greet: object_greet } = require('./object_util');\nexport function run_default() { return util(); }\nexport function run_named() { return greet(); }\nexport function run_member() { return named.greet(); }\nexport function run_object_named() { return object_greet(); }\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "js-commonjs-require-calls".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.js::run_default",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/default_util.js::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.js::run_named",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/named_util.js::greet",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.js::run_member",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/named_util.js::greet",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.js::run_object_named",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/object_util.js::greet",
        ));
    }

    #[test]
    fn test_python_package_reexported_calls_resolve_to_underlying_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("py/pkg")).unwrap();
        fs::create_dir_all(dir.path().join("py/pkg_wild")).unwrap();

        fs::write(
            dir.path().join("py/pkg/helper.py"),
            "def helper():\n    return 1\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("py/pkg/__init__.py"),
            "from .helper import helper\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("py/pkg_wild/helper.py"),
            "def helper():\n    return 1\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("py/pkg_wild/__init__.py"),
            "from .helper import *\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from .pkg import helper\nfrom .pkg import helper as alias\nfrom .pkg_wild import helper as wild_helper\ndef run_pkg():\n    helper()\n    return alias()\ndef run_pkg_wild():\n    return wild_helper()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "python-package-reexported-calls".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::run_pkg",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/pkg/helper.py::helper",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::run_pkg_wild",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/pkg_wild/helper.py::helper",
        ));
    }

    #[test]
    fn test_python_wildcard_imports_resolve_to_underlying_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("py/pkg")).unwrap();

        fs::write(
            dir.path().join("py/helper.py"),
            "def greet():\n    return 1\ndef wave():\n    return 2\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("py/pkg/helper.py"),
            "def helper():\n    return 1\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("py/pkg/__init__.py"),
            "from .helper import helper\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from .helper import *\nfrom .pkg import *\ndef run_module():\n    greet()\n    return wave()\ndef run_package():\n    return helper()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "python-wildcard-imports".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::run_module",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::greet",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::run_module",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::wave",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::run_package",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/pkg/helper.py::helper",
        ));
    }

    #[test]
    fn test_python_wildcard_imports_respect_export_rules() {
        let underscore_dir = tempdir().unwrap();
        fs::create_dir_all(underscore_dir.path().join("py")).unwrap();

        fs::write(
            underscore_dir.path().join("py/helper.py"),
            "def public():\n    return 1\ndef _hidden():\n    return 2\n",
        )
        .unwrap();
        fs::write(
            underscore_dir.path().join("py/main.py"),
            "from .helper import *\ndef run_public():\n    return public()\ndef run_hidden():\n    return _hidden()\n",
        )
        .unwrap();

        let underscore_build = build_code_graph(&CodeGraphBuildInput {
            repository_path: underscore_dir.path().to_path_buf(),
            commit_hash: "python-wildcard-imports-underscore-rules".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &underscore_build.document,
            "symbol:py/main.py::run_public",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::public",
        ));
        assert!(!symbol_has_edge_to_symbol(
            &underscore_build.document,
            "symbol:py/main.py::run_hidden",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::_hidden",
        ));

        let all_dir = tempdir().unwrap();
        fs::create_dir_all(all_dir.path().join("py")).unwrap();
        fs::write(
            all_dir.path().join("py/helper.py"),
            "__all__ = ['chosen']\n\ndef chosen():\n    return 1\n\ndef extra():\n    return 2\n",
        )
        .unwrap();
        fs::write(
            all_dir.path().join("py/main.py"),
            "from .helper import *\ndef run_chosen():\n    return chosen()\ndef run_extra():\n    return extra()\n",
        )
        .unwrap();

        let all_build = build_code_graph(&CodeGraphBuildInput {
            repository_path: all_dir.path().to_path_buf(),
            commit_hash: "python-wildcard-imports-all-rules".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &all_build.document,
            "symbol:py/main.py::run_chosen",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::chosen",
        ));
        assert!(!symbol_has_edge_to_symbol(
            &all_build.document,
            "symbol:py/main.py::run_extra",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::extra",
        ));
    }

    #[test]
    fn test_rust_import_aliases_and_nested_paths_resolve_to_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src/nested")).unwrap();

        fs::write(dir.path().join("src/util.rs"), "pub fn greet() {}\n").unwrap();
        fs::write(dir.path().join("src/nested/mod.rs"), "pub mod util;\n").unwrap();
        fs::write(dir.path().join("src/nested/util.rs"), "pub fn wave() {}\n").unwrap();
        fs::write(
            dir.path().join("src/lib.rs"),
            "mod util;\nmod nested;\nuse util::greet as hello;\npub fn run() { hello(); nested::util::wave(); }\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "rust-import-alias-and-nested-paths".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:src/util.rs::greet",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:src/nested/util.rs::wave",
        ));
    }

    #[test]
    fn test_rust_pub_use_reexports_and_wildcards_resolve_to_underlying_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src/nested")).unwrap();

        fs::write(dir.path().join("src/util.rs"), "pub fn greet() {}\npub fn wave() {}\n").unwrap();
        fs::write(dir.path().join("src/nested/mod.rs"), "pub mod util;\n").unwrap();
        fs::write(dir.path().join("src/nested/util.rs"), "pub fn ping() {}\n").unwrap();
        fs::write(dir.path().join("src/barrel1.rs"), "pub use crate::util::greet;\n").unwrap();
        fs::write(
            dir.path().join("src/barrel2.rs"),
            "pub use crate::barrel1::greet as hello;\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("src/lib.rs"),
            "mod util;\nmod nested;\nmod barrel1;\nmod barrel2;\npub use util::greet;\npub use util::wave as wave_alias;\npub use util::*;\npub use nested::util as util_mod;\npub use nested::{util as util_mod_two};\nuse barrel2::hello;\npub fn run() { greet(); wave(); wave_alias(); util_mod::ping(); util_mod_two::ping(); hello(); }\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "rust-pub-use-reexports-and-wildcards".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:src/util.rs::greet",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:src/util.rs::wave",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:src/nested/util.rs::ping",
        ));
    }

    #[test]
    fn test_rust_module_alias_paths_resolve_to_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src/nested")).unwrap();

        fs::write(dir.path().join("src/nested/mod.rs"), "pub mod util;\n").unwrap();
        fs::write(dir.path().join("src/nested/util.rs"), "pub fn wave() {}\n").unwrap();
        fs::write(
            dir.path().join("src/lib.rs"),
            "mod nested;\nuse nested::util as util_mod;\nuse nested::util::{self as util_mod_two};\nuse nested::{util as util_mod_three};\npub fn run() { util_mod::wave(); util_mod_two::wave(); util_mod_three::wave(); }\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "rust-module-alias-paths".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:src/nested/util.rs::wave",
        ));
    }

    #[test]
    fn test_rust_function_local_aliases_resolve_to_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src/nested")).unwrap();

        fs::write(dir.path().join("src/util.rs"), "pub fn greet() {}\n").unwrap();
        fs::write(dir.path().join("src/nested/mod.rs"), "pub mod util;\n").unwrap();
        fs::write(dir.path().join("src/nested/util.rs"), "pub fn wave() {}\n").unwrap();
        fs::write(
            dir.path().join("src/lib.rs"),
            "mod util;\nmod nested;\nuse util::greet;\nuse nested::util as util_mod;\nuse nested::util::{wave as hello};\npub fn run() { let first = greet; let second = first; let wave_alias = util_mod::wave; let hello_alias = hello; second(); wave_alias(); hello_alias(); }\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "rust-function-local-aliases".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:src/util.rs::greet",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:src/nested/util.rs::wave",
        ));
    }

    #[test]
    fn test_rust_local_aliases_shadow_imported_names_and_stay_isolated() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();

        fs::write(dir.path().join("src/one.rs"), "pub fn one() {}\n").unwrap();
        fs::write(dir.path().join("src/two.rs"), "pub fn two() {}\n").unwrap();
        fs::write(
            dir.path().join("src/lib.rs"),
            "mod one;\nmod two;\nuse one::one;\nuse two::two;\npub fn run() { let one = two; one(); }\npub fn first() { let alias = one; alias(); }\npub fn second() { two(); }\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "rust-local-shadowing-and-isolation".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:src/two.rs::two",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::first",
            "uses_symbol",
            "uses_symbol",
            "symbol:src/one.rs::one",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::second",
            "uses_symbol",
            "uses_symbol",
            "symbol:src/two.rs::two",
        ));
    }

    #[test]
    fn test_rust_unresolved_or_unsupported_local_shadowing_does_not_fall_back() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();

        fs::write(dir.path().join("src/one.rs"), "pub fn one() {}\n").unwrap();
        fs::write(
            dir.path().join("src/lib.rs"),
            "mod one;\nuse one::one;\npub fn missing_case() { let one = missing; one(); }\npub fn expr_case() { let one = if true { missing } else { one }; one(); }\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "rust-local-shadowing-no-fallback".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(!symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::missing_case",
            "uses_symbol",
            "uses_symbol",
            "symbol:src/one.rs::one",
        ));
        assert!(!symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::expr_case",
            "uses_symbol",
            "uses_symbol",
            "symbol:src/one.rs::one",
        ));
    }

    #[test]
    fn test_alias_cycles_remain_unresolved_without_symbol_edges() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();

        fs::write(
            dir.path().join("web/main.ts"),
            "const a = b;\nconst b = a;\nexport function run() { return a(); }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "a = b\nb = a\ndef run():\n    return a()\n",
        )
        .unwrap();
        fs::write(dir.path().join("src/util.rs"), "pub fn greet() {}\n").unwrap();
        fs::write(
            dir.path().join("src/lib.rs"),
            "mod util;\nuse util::greet;\npub fn run() { let a = b; let b = a; a(); }\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "alias-cycles-unresolved".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(!symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/main.ts::a",
        ));
        assert!(!symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/main.py::a",
        ));
        assert!(!symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/lib.rs::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:src/util.rs::greet",
        ));
    }

    #[test]
    fn test_interface_and_trait_inheritance_edges_are_emitted() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();

        fs::write(
            dir.path().join("src/base.rs"),
            "pub trait Parent {}\npub trait Child: Parent {}\n",
        )
        .unwrap();
        fs::write(dir.path().join("src/lib.rs"), "mod base;\n").unwrap();

        fs::write(
            dir.path().join("web/base.ts"),
            "export interface Parent {}\nexport interface Child extends Parent {}\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "interface-trait-inheritance".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/base.ts::Child",
            "extends",
            "extends",
            "symbol:web/base.ts::Parent",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:src/base.rs::Child",
            "extends",
            "extends",
            "symbol:src/base.rs::Parent",
        ));
    }

    #[test]
    fn test_namespace_and_module_member_calls_resolve_to_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import * as utilns from './util';\nexport function run() { return utilns.util(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\ndef execute():\n    return helper_mod.helper()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "member-call-sites".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::execute",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
    }

    #[test]
    fn test_top_level_aliases_resolve_call_sites_to_underlying_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import { util } from './util';\nconst alias = util;\nexport function run() { return alias(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from .helper import helper\nalias = helper\ndef execute():\n    return alias()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "top-level-aliases".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::execute",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
    }

    #[test]
    fn test_top_level_member_aliases_resolve_call_sites_to_underlying_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import * as utilns from './util';\nconst f = utilns.util;\nexport function run() { return f(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\nalias = helper_mod.helper\ndef execute():\n    return alias()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "top-level-member-aliases".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::execute",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
    }

    #[test]
    fn test_top_level_module_aliases_resolve_member_call_sites_to_underlying_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import * as ns from './util';\nconst alias = ns;\nexport function run() { return alias.util(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\nalias = helper_mod\ndef execute():\n    return alias.helper()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "top-level-module-aliases".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::execute",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
    }

    #[test]
    fn test_unresolved_or_unsupported_module_alias_shadowing_does_not_fall_back() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import * as ns from './util';\nconst alias = ns;\nexport function missing_case() { const alias = missing; return alias.util(); }\nexport function expr_case() { const alias = true ? ns : ns; return alias.util(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\nalias = helper_mod\ndef missing_case():\n    alias = missing\n    return alias.helper()\ndef expr_case():\n    alias = helper_mod if True else helper_mod\n    return alias.helper()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "module-alias-shadowing-no-fallback".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(!symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::missing_case",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(!symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::expr_case",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(!symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::missing_case",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
        assert!(!symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::expr_case",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
    }

    #[test]
    fn test_alias_chains_resolve_call_sites_to_underlying_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import { util } from './util';\nconst first = util;\nconst second = first;\nexport function run() { return second(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from .helper import helper\nfirst = helper\nsecond = first\ndef execute():\n    return second()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "alias-chains".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::execute",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
    }

    #[test]
    fn test_top_level_member_alias_chains_resolve_call_sites_to_underlying_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import * as utilns from './util';\nconst first = utilns.util;\nconst second = first;\nexport function run() { return second(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\nfirst = helper_mod.helper\nsecond = first\ndef execute():\n    return second()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "top-level-member-alias-chains".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::execute",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
    }

    #[test]
    fn test_mixed_scope_member_aliases_resolve_call_sites_to_underlying_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import * as utilns from './util';\nconst top = utilns.util;\nexport function run() { const local = top; return local(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\ntop = helper_mod.helper\ndef execute():\n    local = top\n    return local()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "mixed-scope-member-aliases".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::execute",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
    }

    #[test]
    fn test_local_aliases_shadow_top_level_aliases_of_same_name() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/one.ts"),
            "export function one() { return 1; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/two.ts"),
            "export function two() { return 2; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import { one } from './one';\nimport { two } from './two';\nconst alias = one;\nexport function run() { const alias = two; return alias(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/one.py"), "def one():\n    return 1\n").unwrap();
        fs::write(dir.path().join("py/two.py"), "def two():\n    return 2\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from .one import one\nfrom .two import two\nalias = one\ndef execute():\n    alias = two\n    return alias()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "alias-shadowing".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/two.ts::two",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::execute",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/two.py::two",
        ));
    }

    #[test]
    fn test_unresolved_or_unsupported_local_shadowing_does_not_fall_back_to_top_level_aliases() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/one.ts"),
            "export function one() { return 1; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import { one } from './one';\nconst alias = one;\nexport function run_missing() { const alias = missing; return alias(); }\nexport function run_expr() { const alias = true ? one : one; return alias(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/one.py"), "def one():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from .one import one\nalias = one\ndef execute_missing():\n    alias = missing\n    return alias()\ndef execute_expr():\n    alias = 1 if True else one\n    return alias()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "shadowing-no-fallback".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(!symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run_missing",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/one.ts::one",
        ));
        assert!(!symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run_expr",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/one.ts::one",
        ));
        assert!(!symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::execute_missing",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/one.py::one",
        ));
        assert!(!symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::execute_expr",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/one.py::one",
        ));
    }

    #[test]
    fn test_local_aliases_are_isolated_to_their_own_enclosing_symbol() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/one.ts"),
            "export function one() { return 1; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/two.ts"),
            "export function two() { return 2; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import { one } from './one';\nimport { two } from './two';\nexport function first() { const alias = one; return alias(); }\nexport function second() { return two(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/one.py"), "def one():\n    return 1\n").unwrap();
        fs::write(dir.path().join("py/two.py"), "def two():\n    return 2\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from .one import one\nfrom .two import two\ndef first():\n    alias = one\n    return alias()\ndef second():\n    return two()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "alias-scope-isolation".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::first",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/one.ts::one",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::second",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/two.ts::two",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::first",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/one.py::one",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::second",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/two.py::two",
        ));
    }

    #[test]
    fn test_function_local_aliases_resolve_call_sites_to_underlying_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import { util } from './util';\nexport function run() { const localAlias = util; return localAlias(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from .helper import helper\ndef execute():\n    alias = helper\n    return alias()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "function-local-aliases".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::execute",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
    }

    #[test]
    fn test_function_local_alias_chains_resolve_call_sites_to_underlying_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import { util } from './util';\nexport function run() { const first = util; const second = first; return second(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from .helper import helper\ndef execute():\n    first = helper\n    second = first\n    return second()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "function-local-alias-chains".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::execute",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
    }

    #[test]
    fn test_function_local_member_aliases_resolve_call_sites_to_underlying_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import * as utilns from './util';\nexport function run() { const f = utilns.util; return f(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\ndef execute():\n    alias = helper_mod.helper\n    return alias()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "function-local-member-aliases".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::execute",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
    }

    #[test]
    fn test_function_local_module_alias_chains_resolve_member_call_sites_to_underlying_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import * as ns from './util';\nexport function run() { const first = ns; const second = first; return second.util(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\ndef execute():\n    first = helper_mod\n    second = first\n    return second.helper()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "function-local-module-alias-chains".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::execute",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
    }

    #[test]
    fn test_function_local_module_aliases_are_isolated_to_their_own_enclosing_symbol() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import * as ns from './util';\nexport function first() { const alias = ns; return alias.util(); }\nexport function second() { return ns.util(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\ndef first():\n    alias = helper_mod\n    return alias.helper()\ndef second():\n    return helper_mod.helper()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "function-local-module-alias-isolation".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::first",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::second",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::first",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::second",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
    }

    #[test]
    fn test_function_local_member_alias_chains_resolve_call_sites_to_underlying_symbols() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();
        fs::create_dir_all(dir.path().join("py")).unwrap();

        fs::write(
            dir.path().join("web/util.ts"),
            "export function util() { return 42; }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/main.ts"),
            "import * as utilns from './util';\nexport function run() { const first = utilns.util; const second = first; return second(); }\n",
        )
        .unwrap();

        fs::write(dir.path().join("py/helper.py"), "def helper():\n    return 1\n").unwrap();
        fs::write(
            dir.path().join("py/main.py"),
            "from . import helper as helper_mod\ndef execute():\n    first = helper_mod.helper\n    second = first\n    return second()\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "function-local-member-alias-chains".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:web/main.ts::run",
            "uses_symbol",
            "uses_symbol",
            "symbol:web/util.ts::util",
        ));
        assert!(symbol_has_edge_to_symbol(
            &build.document,
            "symbol:py/main.py::execute",
            "uses_symbol",
            "uses_symbol",
            "symbol:py/helper.py::helper",
        ));
    }

    #[test]
    fn test_nested_symbols_are_captured_and_nested_in_structure() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::create_dir_all(dir.path().join("pkg")).unwrap();
        fs::create_dir_all(dir.path().join("web")).unwrap();

        fs::write(
            dir.path().join("src/lib.rs"),
            "pub struct Thing;\nimpl Thing { pub fn method(&self) {} }\npub fn top() { fn inner() {} inner(); }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("pkg/mod.py"),
            "class Thing:\n    def method(self):\n        return 1\n\ndef top():\n    def inner():\n        return 2\n    return inner()\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("web/mod.ts"),
            "export class Thing {\n  method() { return 1; }\n}\nexport function top() {\n  function inner() { return 2; }\n  return inner();\n}\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "nested-symbols".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        let keys = symbol_logical_keys(&build.document);
        assert!(keys.iter().any(|k| k.starts_with("symbol:src/lib.rs::Thing::method")));
        assert!(keys.iter().any(|k| k.starts_with("symbol:src/lib.rs::top::inner")));
        assert!(keys.iter().any(|k| k.starts_with("symbol:pkg/mod.py::Thing::method")));
        assert!(keys.iter().any(|k| k.starts_with("symbol:pkg/mod.py::top::inner")));
        assert!(keys.iter().any(|k| k.starts_with("symbol:web/mod.ts::Thing::method")));
        assert!(keys.iter().any(|k| k.starts_with("symbol:web/mod.ts::top::inner")));

        let key_index = logical_key_to_block_id(&build.document);
        let rust_method_id = key_index
            .iter()
            .find(|(key, _)| key.starts_with("symbol:src/lib.rs::Thing::method"))
            .map(|(_, id)| *id)
            .unwrap();
        let rust_parent_id = build
            .document
            .structure
            .iter()
            .find(|(_, children)| children.contains(&rust_method_id))
            .map(|(id, _)| *id)
            .unwrap();

        let rust_parent_key = key_index
            .iter()
            .find(|(_, id)| **id == rust_parent_id)
            .map(|(key, _)| key.as_str())
            .unwrap();

        assert!(rust_parent_key.starts_with("symbol:src/lib.rs::Thing"));
    }

    #[test]
    fn test_ts_js_export_aliases_generators_and_function_like_members_are_captured() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();

        fs::write(
            dir.path().join("src/mod.ts"),
            "function internal() { return 1; }\nexport { internal };\nexport function* gen() { yield 1; }\nexport const arrow = () => 1;\nclass Example {\n  handler = () => 1;\n}\nexport default Example;\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("src/mod.js"),
            "function internalJs() { return 1; }\nexport { internalJs };\nexport function* jsGen() { yield 1; }\nclass JsExample {\n  handler = () => 1;\n}\nexport default JsExample;\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "ts-js-coverage".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        let keys = symbol_logical_keys(&build.document);
        assert!(keys.iter().any(|k| k.starts_with("symbol:src/mod.ts::gen")));
        assert!(keys.iter().any(|k| k.starts_with("symbol:src/mod.ts::Example::handler")));
        assert!(keys.iter().any(|k| k.starts_with("symbol:src/mod.js::jsGen")));
        assert!(keys.iter().any(|k| k.starts_with("symbol:src/mod.js::JsExample::handler")));

        assert!(symbol_exported(&build.document, "symbol:src/mod.ts::internal"));
        assert!(symbol_exported(&build.document, "symbol:src/mod.ts::Example"));
        assert!(symbol_exported(&build.document, "symbol:src/mod.js::internalJs"));
        assert!(symbol_exported(&build.document, "symbol:src/mod.js::JsExample"));

        assert_eq!(symbol_kind(&build.document, "symbol:src/mod.ts::arrow").as_deref(), Some("function"));
        assert_eq!(
            symbol_kind(&build.document, "symbol:src/mod.ts::Example::handler").as_deref(),
            Some("method")
        );
    }

    #[test]
    fn test_performance_smoke_medium_fixture() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src");
        fs::create_dir_all(&src).unwrap();

        for i in 0..300usize {
            let mut file = fs::File::create(src.join(format!("m{}.rs", i))).unwrap();
            writeln!(file, "pub fn f{}() {{}}", i).unwrap();
            if i > 0 {
                writeln!(file, "use crate::m{}::f{};", i - 1, i - 1).unwrap();
            }
        }

        let start = std::time::Instant::now();
        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "perf-smoke".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();
        let elapsed = start.elapsed();

        assert!(build.stats.file_nodes >= 300);
        assert!(elapsed.as_secs_f64() < 3.0, "elapsed: {elapsed:?}");
    }
}
