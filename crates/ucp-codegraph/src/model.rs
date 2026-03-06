use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;
use std::str::FromStr;
use ucm_core::{Block, BlockId, Document, DocumentId, DocumentMetadata};

pub const CODEGRAPH_PROFILE: &str = "codegraph";
pub const CODEGRAPH_PROFILE_VERSION: &str = "v1";
pub const CODEGRAPH_PROFILE_MARKER: &str = "codegraph.v1";
pub const CODEGRAPH_EXTRACTOR_VERSION: &str = "ucp-codegraph-extractor.v1";

pub(crate) const META_NODE_CLASS: &str = "node_class";
pub(crate) const META_LOGICAL_KEY: &str = "logical_key";
pub(crate) const META_CODEREF: &str = "coderef";
pub(crate) const META_LANGUAGE: &str = "language";
pub(crate) const META_SYMBOL_KIND: &str = "symbol_kind";
pub(crate) const META_SYMBOL_NAME: &str = "name";
pub(crate) const META_EXPORTED: &str = "exported";

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
    pub(crate) fn error(code: &str, message: impl Into<String>) -> Self {
        Self {
            severity: CodeGraphSeverity::Error,
            code: code.to_string(),
            message: message.into(),
            path: None,
            logical_key: None,
        }
    }

    pub(crate) fn warning(code: &str, message: impl Into<String>) -> Self {
        Self {
            severity: CodeGraphSeverity::Warning,
            code: code.to_string(),
            message: message.into(),
            path: None,
            logical_key: None,
        }
    }

    pub(crate) fn info(code: &str, message: impl Into<String>) -> Self {
        Self {
            severity: CodeGraphSeverity::Info,
            code: code.to_string(),
            message: message.into(),
            path: None,
            logical_key: None,
        }
    }

    pub(crate) fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub(crate) fn with_logical_key(mut self, logical_key: impl Into<String>) -> Self {
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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CodeLanguage {
    Rust,
    Python,
    TypeScript,
    JavaScript,
}

impl CodeLanguage {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::TypeScript => "typescript",
            Self::JavaScript => "javascript",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RepoFile {
    pub(crate) absolute_path: PathBuf,
    pub(crate) relative_path: String,
    pub(crate) language: CodeLanguage,
}

#[derive(Debug, Clone)]
pub(crate) struct ExtractedSymbol {
    pub(crate) name: String,
    pub(crate) qualified_name: String,
    pub(crate) identity: String,
    pub(crate) parent_identity: Option<String>,
    pub(crate) kind: String,
    pub(crate) description: Option<String>,
    pub(crate) modifiers: ExtractedModifiers,
    pub(crate) inputs: Vec<ExtractedInput>,
    pub(crate) output: Option<String>,
    pub(crate) type_info: Option<String>,
    pub(crate) exported: bool,
    pub(crate) start_line: usize,
    pub(crate) start_col: usize,
    pub(crate) end_line: usize,
    pub(crate) end_col: usize,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ExtractedInput {
    pub(crate) name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub(crate) type_name: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ExtractedSignature {
    pub(crate) inputs: Vec<ExtractedInput>,
    pub(crate) output: Option<String>,
    pub(crate) type_info: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub(crate) struct ExtractedModifiers {
    #[serde(rename = "async", skip_serializing_if = "is_false")]
    pub(crate) is_async: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub(crate) generator: bool,
    #[serde(rename = "static", skip_serializing_if = "is_false")]
    pub(crate) is_static: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) visibility: Option<String>,
}

impl ExtractedModifiers {
    pub(crate) fn is_empty(&self) -> bool {
        !self.is_async && !self.generator && !self.is_static && self.visibility.is_none()
    }
}

fn is_false(value: &bool) -> bool {
    !*value
}

#[derive(Debug, Clone)]
pub(crate) struct ExtractedImport {
    pub(crate) module: String,
    pub(crate) symbols: Vec<String>,
    pub(crate) bindings: Vec<ImportBinding>,
    pub(crate) module_aliases: Vec<String>,
    pub(crate) reexported: bool,
    pub(crate) wildcard: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ImportBinding {
    pub(crate) source_name: String,
    pub(crate) local_name: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ExtractedRelationship {
    pub(crate) source_identity: String,
    pub(crate) relation: String,
    pub(crate) target_expr: String,
    pub(crate) target_name: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ExtractedUsage {
    pub(crate) source_identity: String,
    pub(crate) target_expr: String,
    pub(crate) target_name: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ExtractedAlias {
    pub(crate) name: String,
    pub(crate) target_expr: String,
    pub(crate) target_name: String,
    pub(crate) owner_identity: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ImportResolution {
    Resolved(String),
    External,
    Unresolved,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FileAnalysis {
    pub(crate) file_description: Option<String>,
    pub(crate) symbols: Vec<ExtractedSymbol>,
    pub(crate) imports: Vec<ExtractedImport>,
    pub(crate) relationships: Vec<ExtractedRelationship>,
    pub(crate) usages: Vec<ExtractedUsage>,
    pub(crate) aliases: Vec<ExtractedAlias>,
    pub(crate) export_bindings: Vec<ImportBinding>,
    pub(crate) exported_symbol_names: BTreeSet<String>,
    pub(crate) default_exported_symbol_names: BTreeSet<String>,
    pub(crate) diagnostics: Vec<CodeGraphDiagnostic>,
}

#[derive(Debug, Clone)]
pub(crate) struct FileAnalysisRecord {
    pub(crate) file: String,
    pub(crate) language: CodeLanguage,
    pub(crate) imports: Vec<ExtractedImport>,
    pub(crate) relationships: Vec<ExtractedRelationship>,
    pub(crate) usages: Vec<ExtractedUsage>,
    pub(crate) aliases: Vec<ExtractedAlias>,
    pub(crate) export_bindings: Vec<ImportBinding>,
}

impl ExtractedImport {
    pub(crate) fn module(module: impl Into<String>) -> Self {
        Self {
            module: module.into(),
            symbols: Vec::new(),
            bindings: Vec::new(),
            module_aliases: Vec::new(),
            reexported: false,
            wildcard: false,
        }
    }

    pub(crate) fn symbol(module: impl Into<String>, symbol: impl Into<String>) -> Self {
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

    pub(crate) fn bindings(module: impl Into<String>, bindings: Vec<ImportBinding>) -> Self {
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

    pub(crate) fn with_module_alias(mut self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        if !alias.is_empty() && !self.module_aliases.contains(&alias) {
            self.module_aliases.push(alias);
        }
        self
    }

    pub(crate) fn reexported(mut self) -> Self {
        self.reexported = true;
        self
    }

    pub(crate) fn wildcard(mut self) -> Self {
        self.wildcard = true;
        self
    }
}

impl ImportBinding {
    pub(crate) fn new(source_name: impl Into<String>, local_name: impl Into<String>) -> Self {
        Self {
            source_name: source_name.into(),
            local_name: local_name.into(),
        }
    }

    pub(crate) fn same(name: impl Into<String>) -> Self {
        let name = name.into();
        Self::new(name.clone(), name)
    }
}

impl ExtractedRelationship {
    pub(crate) fn new(
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
    pub(crate) fn new(
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
    pub(crate) fn new(
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

fn deterministic_timestamp() -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
        .expect("valid deterministic timestamp")
        .with_timezone(&chrono::Utc)
}
