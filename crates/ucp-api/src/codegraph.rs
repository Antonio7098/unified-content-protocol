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

        let analysis = analyze_file(&file.relative_path, &source, file.language);
        for diag in &analysis.diagnostics {
            diagnostics.push(diag.clone().with_path(file.relative_path.clone()));
        }

        for symbol in &analysis.symbols {
            let logical_key = unique_symbol_logical_key(
                &file.relative_path,
                &symbol.name,
                symbol.start_line,
                &mut used_symbol_keys,
            );
            let symbol_block = make_symbol_block(
                &logical_key,
                &file.relative_path,
                file.language.as_str(),
                symbol,
            );
            let symbol_id = doc.add_block(symbol_block, &file_block_id)?;

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
            imports: analysis.imports,
        });
    }

    let known_files: BTreeSet<String> = file_ids.keys().cloned().collect();
    let mut pending_reference_edges: BTreeSet<(String, String, String)> = BTreeSet::new();

    for record in &file_analyses {
        for import in &record.imports {
            match resolve_import(&record.file, &record.language, &import.module, &known_files) {
                Some(target) if target != record.file => {
                    pending_reference_edges.insert((
                        record.file.clone(),
                        target,
                        import.module.clone(),
                    ));
                }
                Some(_) => {}
                None => {
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
}

#[derive(Debug, Clone, Default)]
struct FileAnalysis {
    symbols: Vec<ExtractedSymbol>,
    imports: Vec<ExtractedImport>,
    diagnostics: Vec<CodeGraphDiagnostic>,
}

#[derive(Debug, Clone)]
struct FileAnalysisRecord {
    file: String,
    language: CodeLanguage,
    imports: Vec<ExtractedImport>,
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
        CodeLanguage::Python => analyze_python_tree(source, root, &mut analysis),
        CodeLanguage::TypeScript | CodeLanguage::JavaScript => {
            analyze_ts_tree(source, root, &mut analysis)
        }
    }

    if analysis.symbols.is_empty() {
        analysis.diagnostics.push(
            CodeGraphDiagnostic::info(
                "CG2001",
                format!("no top-level symbols extracted for {}", path),
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

fn analyze_rust_tree(source: &str, root: Node<'_>, analysis: &mut FileAnalysis) {
    let mut cursor = root.walk();
    for node in root.named_children(&mut cursor) {
        match node.kind() {
            "use_declaration" => {
                let import_text = node_text(source, node)
                    .trim()
                    .trim_start_matches("pub ")
                    .trim_start_matches("use ")
                    .trim_end_matches(';')
                    .trim()
                    .to_string();
                if !import_text.is_empty() {
                    analysis.imports.push(ExtractedImport {
                        module: import_text,
                    });
                }
            }
            "mod_item" => {
                let text = node_text(source, node);
                if text.trim().ends_with(';') {
                    if let Some(name) = rust_symbol_name(node, source) {
                        analysis.imports.push(ExtractedImport {
                            module: format!("mod:{}", name),
                        });
                    }
                }
                if let Some(symbol) = rust_symbol_from_node(node, source) {
                    analysis.symbols.push(symbol);
                }
            }
            "function_item" | "struct_item" | "enum_item" | "trait_item" | "impl_item"
            | "type_item" | "const_item" => {
                if let Some(symbol) = rust_symbol_from_node(node, source) {
                    analysis.symbols.push(symbol);
                }
            }
            _ => {}
        }
    }
}

fn rust_symbol_from_node(node: Node<'_>, source: &str) -> Option<ExtractedSymbol> {
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
    let exported = node_text(source, node).trim_start().starts_with("pub");
    let (start_line, start_col, end_line, end_col) = node_span(node);

    Some(ExtractedSymbol {
        name,
        kind: kind.to_string(),
        exported,
        start_line,
        start_col,
        end_line,
        end_col,
    })
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

fn analyze_python_tree(source: &str, root: Node<'_>, analysis: &mut FileAnalysis) {
    let mut cursor = root.walk();
    for node in root.named_children(&mut cursor) {
        match node.kind() {
            "import_statement" => {
                let text = node_text(source, node).trim().to_string();
                if let Some(list) = text.strip_prefix("import ") {
                    for item in list.split(',') {
                        let name = item.split_whitespace().next().unwrap_or("").trim();
                        if !name.is_empty() {
                            analysis.imports.push(ExtractedImport {
                                module: name.to_string(),
                            });
                        }
                    }
                }
            }
            "import_from_statement" => {
                let text = node_text(source, node).trim().to_string();
                if let Some(rest) = text.strip_prefix("from ") {
                    if let Some((module, _)) = rest.split_once(" import ") {
                        let module_name = module.trim();
                        if !module_name.is_empty() {
                            analysis.imports.push(ExtractedImport {
                                module: module_name.to_string(),
                            });
                        }
                    }
                }
            }
            "function_definition" | "class_definition" => {
                let Some(name_node) = node.child_by_field_name("name") else {
                    continue;
                };
                let name = node_text(source, name_node).trim().to_string();
                if name.is_empty() {
                    continue;
                }
                let (start_line, start_col, end_line, end_col) = node_span(node);
                analysis.symbols.push(ExtractedSymbol {
                    name: name.clone(),
                    kind: if node.kind() == "class_definition" {
                        "class".to_string()
                    } else {
                        "function".to_string()
                    },
                    exported: !name.starts_with('_'),
                    start_line,
                    start_col,
                    end_line,
                    end_col,
                });
            }
            _ => {}
        }
    }
}

fn analyze_ts_tree(source: &str, root: Node<'_>, analysis: &mut FileAnalysis) {
    let mut cursor = root.walk();
    for node in root.named_children(&mut cursor) {
        match node.kind() {
            "import_statement" => {
                if let Some(module) = extract_ts_module_from_text(node_text(source, node)) {
                    analysis.imports.push(ExtractedImport { module });
                }
            }
            "export_statement" => {
                if let Some(module) = extract_ts_module_from_text(node_text(source, node)) {
                    analysis.imports.push(ExtractedImport { module });
                }
                analysis
                    .symbols
                    .extend(ts_symbols_from_export_statement(node, source));
            }
            "function_declaration"
            | "class_declaration"
            | "interface_declaration"
            | "type_alias_declaration"
            | "enum_declaration"
            | "module" => {
                if let Some(symbol) = ts_symbol_from_declaration(node, source, false) {
                    analysis.symbols.push(symbol);
                }
            }
            "lexical_declaration" | "variable_statement" => {
                analysis
                    .symbols
                    .extend(ts_variable_symbols(node, source, false));
            }
            _ => {}
        }
    }
}

fn ts_symbols_from_export_statement(node: Node<'_>, source: &str) -> Vec<ExtractedSymbol> {
    let mut out = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "function_declaration"
            | "class_declaration"
            | "interface_declaration"
            | "type_alias_declaration"
            | "enum_declaration"
            | "module" => {
                if let Some(symbol) = ts_symbol_from_declaration(child, source, true) {
                    out.push(symbol);
                }
            }
            "lexical_declaration" | "variable_statement" => {
                out.extend(ts_variable_symbols(child, source, true));
            }
            _ => {}
        }
    }
    out
}

fn ts_symbol_from_declaration(
    node: Node<'_>,
    source: &str,
    exported_hint: bool,
) -> Option<ExtractedSymbol> {
    let kind = match node.kind() {
        "function_declaration" => "function",
        "class_declaration" => "class",
        "interface_declaration" => "interface",
        "type_alias_declaration" => "type",
        "enum_declaration" => "enum",
        "module" => "namespace",
        _ => return None,
    };

    let name = node
        .child_by_field_name("name")
        .map(|n| node_text(source, n).trim().to_string())
        .or_else(|| first_named_identifier(node, source))?;
    if name.is_empty() {
        return None;
    }
    let exported = exported_hint || node_text(source, node).trim_start().starts_with("export ");
    let (start_line, start_col, end_line, end_col) = node_span(node);

    Some(ExtractedSymbol {
        name,
        kind: kind.to_string(),
        exported,
        start_line,
        start_col,
        end_line,
        end_col,
    })
}

fn ts_variable_symbols(node: Node<'_>, source: &str, exported_hint: bool) -> Vec<ExtractedSymbol> {
    let mut out = Vec::new();
    let exported = exported_hint || node_text(source, node).trim_start().starts_with("export ");

    let mut stack = vec![node];
    while let Some(current) = stack.pop() {
        if current.kind() == "variable_declarator" {
            if let Some(name_node) = current.child_by_field_name("name") {
                let name = node_text(source, name_node).trim().to_string();
                if !name.is_empty() {
                    let (start_line, start_col, end_line, end_col) = node_span(current);
                    out.push(ExtractedSymbol {
                        name,
                        kind: "variable".to_string(),
                        exported,
                        start_line,
                        start_col,
                        end_line,
                        end_col,
                    });
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

fn resolve_import(
    source_file: &str,
    language: &CodeLanguage,
    module: &str,
    known_files: &BTreeSet<String>,
) -> Option<String> {
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
) -> Option<String> {
    if !module.starts_with('.') {
        return None;
    }

    let source_dir = parent_directory(source_file);
    let joined = normalize_relative_join(&source_dir, module);

    ts_candidates(&joined)
        .into_iter()
        .find(|candidate| known_files.contains(candidate))
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
) -> Option<String> {
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

    py_candidates(&joined)
        .into_iter()
        .find(|candidate| known_files.contains(candidate))
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
) -> Option<String> {
    if module.starts_with("std::") || module.starts_with("core::") || module.starts_with("alloc::")
    {
        return None;
    }

    if let Some(name) = module.strip_prefix("mod:") {
        let source_dir = parent_directory(source_file);
        let local = normalize_relative_join(&source_dir, name);
        for candidate in [format!("{}.rs", local), format!("{}/mod.rs", local)] {
            if known_files.contains(&candidate) {
                return Some(candidate);
            }
        }
        return None;
    }

    let source_dir = parent_directory(source_file);

    let (base_dir, path_segments) = if let Some(rest) = module.strip_prefix("crate::") {
        (
            "src".to_string(),
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
            "src".to_string(),
            module
                .split("::")
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        )
    };

    for trimmed in (1..=path_segments.len()).rev() {
        let joined = path_segments[..trimmed].join("/");
        if joined.is_empty() {
            continue;
        }
        let candidate_base = if base_dir.is_empty() {
            joined
        } else {
            format!("{}/{}", base_dir, joined)
        };

        for candidate in [
            format!("{}.rs", candidate_base),
            format!("{}/mod.rs", candidate_base),
        ] {
            if known_files.contains(&candidate) {
                return Some(candidate);
            }
        }
    }

    None
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
    use std::io::Write;
    use tempfile::tempdir;

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
        assert_eq!(resolved.as_deref(), Some("src/util.ts"));
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
