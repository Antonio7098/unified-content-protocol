use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::Write;
use std::path::PathBuf;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use ucm_core::{Block, BlockId, Content, Document};

use crate::model::{
    CODEGRAPH_PROFILE_MARKER, META_CODEREF, META_EXPORTED, META_LANGUAGE, META_LOGICAL_KEY,
    META_NODE_CLASS, META_SYMBOL_NAME,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CodeGraphDetailLevel {
    #[default]
    Skeleton,
    SymbolCard,
    Neighborhood,
    Source,
}

impl CodeGraphDetailLevel {
    fn max(self, other: Self) -> Self {
        std::cmp::max(self, other)
    }

    fn demoted(self) -> Self {
        match self {
            Self::Source => Self::Neighborhood,
            Self::Neighborhood => Self::SymbolCard,
            Self::SymbolCard => Self::Skeleton,
            Self::Skeleton => Self::Skeleton,
        }
    }

    fn includes_neighborhood(self) -> bool {
        matches!(self, Self::Neighborhood | Self::Source)
    }

    fn includes_source(self) -> bool {
        matches!(self, Self::Source)
    }
}

fn default_true() -> bool {
    true
}

fn default_relation_prune_priority() -> BTreeMap<String, u8> {
    [
        ("references", 60),
        ("cited_by", 60),
        ("links_to", 55),
        ("uses_symbol", 35),
        ("imports_symbol", 30),
        ("reexports_symbol", 25),
        ("calls", 20),
        ("inherits", 15),
        ("implements", 15),
    ]
    .into_iter()
    .map(|(name, score)| (name.to_string(), score))
    .collect()
}

fn selection_origin(
    kind: CodeGraphSelectionOriginKind,
    relation: Option<&str>,
    anchor: Option<BlockId>,
) -> Option<CodeGraphSelectionOrigin> {
    Some(CodeGraphSelectionOrigin {
        kind,
        relation: relation.map(str::to_string),
        anchor,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydratedSourceExcerpt {
    pub path: String,
    pub display: String,
    pub start_line: usize,
    pub end_line: usize,
    pub snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphContextNode {
    pub block_id: BlockId,
    pub detail_level: CodeGraphDetailLevel,
    #[serde(default)]
    pub pinned: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<CodeGraphSelectionOrigin>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hydrated_source: Option<HydratedSourceExcerpt>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeGraphSelectionOriginKind {
    Overview,
    Manual,
    FileSymbols,
    Dependencies,
    Dependents,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphSelectionOrigin {
    pub kind: CodeGraphSelectionOriginKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<BlockId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphPrunePolicy {
    pub max_selected: usize,
    #[serde(default = "default_true")]
    pub demote_before_remove: bool,
    #[serde(default = "default_true")]
    pub protect_focus: bool,
    #[serde(default = "default_relation_prune_priority")]
    pub relation_prune_priority: BTreeMap<String, u8>,
}

impl Default for CodeGraphPrunePolicy {
    fn default() -> Self {
        Self {
            max_selected: 48,
            demote_before_remove: true,
            protect_focus: true,
            relation_prune_priority: default_relation_prune_priority(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeGraphContextSession {
    #[serde(default)]
    pub selected: HashMap<BlockId, CodeGraphContextNode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus: Option<BlockId>,
    #[serde(default)]
    pub prune_policy: CodeGraphPrunePolicy,
    #[serde(default)]
    pub history: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeGraphContextUpdate {
    #[serde(default)]
    pub added: Vec<BlockId>,
    #[serde(default)]
    pub removed: Vec<BlockId>,
    #[serde(default)]
    pub changed: Vec<BlockId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus: Option<BlockId>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphContextSummary {
    pub selected: usize,
    pub max_selected: usize,
    pub repositories: usize,
    pub directories: usize,
    pub files: usize,
    pub symbols: usize,
    pub hydrated_sources: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphRenderConfig {
    pub max_edges_per_node: usize,
    pub max_source_lines: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphCoderef {
    pub path: String,
    pub display: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_line: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_line: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphContextNodeExport {
    pub block_id: BlockId,
    pub short_id: String,
    pub node_class: String,
    pub label: String,
    pub detail_level: CodeGraphDetailLevel,
    pub pinned: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<CodeGraphSelectionOrigin>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coderef: Option<CodeGraphCoderef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hydrated_source: Option<HydratedSourceExcerpt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphContextEdgeExport {
    pub source: BlockId,
    pub source_short_id: String,
    pub target: BlockId,
    pub target_short_id: String,
    pub relation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphContextFrontierAction {
    pub block_id: BlockId,
    pub short_id: String,
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    pub candidate_count: usize,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphContextExport {
    pub summary: CodeGraphContextSummary,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus: Option<BlockId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus_short_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus_label: Option<String>,
    pub nodes: Vec<CodeGraphContextNodeExport>,
    pub edges: Vec<CodeGraphContextEdgeExport>,
    pub frontier: Vec<CodeGraphContextFrontierAction>,
    pub omitted_symbol_count: usize,
    pub rendered: String,
}

impl Default for CodeGraphRenderConfig {
    fn default() -> Self {
        Self {
            max_edges_per_node: 6,
            max_source_lines: 12,
        }
    }
}

impl CodeGraphRenderConfig {
    pub fn for_max_tokens(max_tokens: usize) -> Self {
        if max_tokens <= 512 {
            Self {
                max_edges_per_node: 2,
                max_source_lines: 4,
            }
        } else if max_tokens <= 1024 {
            Self {
                max_edges_per_node: 3,
                max_source_lines: 6,
            }
        } else if max_tokens <= 2048 {
            Self {
                max_edges_per_node: 4,
                max_source_lines: 8,
            }
        } else {
            Self::default()
        }
    }
}

#[derive(Debug, Clone)]
struct IndexedEdge {
    other: BlockId,
    relation: String,
}

#[derive(Debug, Clone)]
struct CodeGraphQueryIndex {
    logical_keys: HashMap<BlockId, String>,
    logical_key_to_id: HashMap<String, BlockId>,
    paths_to_id: HashMap<String, BlockId>,
    display_to_id: HashMap<String, BlockId>,
    symbol_names_to_id: HashMap<String, Vec<BlockId>>,
    node_classes: HashMap<BlockId, String>,
    outgoing: HashMap<BlockId, Vec<IndexedEdge>>,
    incoming: HashMap<BlockId, Vec<IndexedEdge>>,
    file_symbols: HashMap<BlockId, Vec<BlockId>>,
    symbol_children: HashMap<BlockId, Vec<BlockId>>,
}

impl CodeGraphContextSession {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn selected_block_ids(&self) -> Vec<BlockId> {
        let mut ids: Vec<_> = self.selected.keys().copied().collect();
        ids.sort_by_key(BlockId::to_string);
        ids
    }

    pub fn summary(&self, doc: &Document) -> CodeGraphContextSummary {
        let index = CodeGraphQueryIndex::new(doc);
        let mut summary = CodeGraphContextSummary {
            selected: self.selected.len(),
            max_selected: self.prune_policy.max_selected,
            repositories: 0,
            directories: 0,
            files: 0,
            symbols: 0,
            hydrated_sources: 0,
        };

        for node in self.selected.values() {
            match index.node_class(&node.block_id).unwrap_or("unknown") {
                "repository" => summary.repositories += 1,
                "directory" => summary.directories += 1,
                "file" => summary.files += 1,
                "symbol" => summary.symbols += 1,
                _ => {}
            }
            if node.hydrated_source.is_some() {
                summary.hydrated_sources += 1;
            }
        }

        summary
    }

    pub fn clear(&mut self) {
        self.selected.clear();
        self.focus = None;
        self.history.push("clear".to_string());
    }

    pub fn set_prune_policy(&mut self, policy: CodeGraphPrunePolicy) {
        self.prune_policy = policy;
        self.history.push(format!(
            "policy:max_selected:{}:demote:{}:protect_focus:{}",
            self.prune_policy.max_selected,
            self.prune_policy.demote_before_remove,
            self.prune_policy.protect_focus
        ));
    }

    pub fn set_focus(&mut self, doc: &Document, block_id: Option<BlockId>) -> CodeGraphContextUpdate {
        let mut update = CodeGraphContextUpdate::default();
        if let Some(block_id) = block_id {
            if doc.get_block(&block_id).is_none() {
                update
                    .warnings
                    .push(format!("focus block not found: {}", block_id));
                return update;
            }
            self.ensure_selected_with_origin(
                block_id,
                CodeGraphDetailLevel::Skeleton,
                selection_origin(CodeGraphSelectionOriginKind::Manual, None, None),
                &mut update,
            );
        }
        self.focus = block_id;
        self.apply_prune_policy(doc, &mut update);
        update.focus = self.focus;
        self.history.push(match self.focus {
            Some(id) => format!("focus:{}", id),
            None => "focus:clear".to_string(),
        });
        update
    }

    pub fn select_block(
        &mut self,
        doc: &Document,
        block_id: BlockId,
        detail_level: CodeGraphDetailLevel,
    ) -> CodeGraphContextUpdate {
        let mut update = CodeGraphContextUpdate::default();
        if doc.get_block(&block_id).is_none() {
            update
                .warnings
                .push(format!("block not found: {}", block_id));
            return update;
        }
        self.ensure_selected_with_origin(
            block_id,
            detail_level,
            selection_origin(CodeGraphSelectionOriginKind::Manual, None, None),
            &mut update,
        );
        self.apply_prune_policy(doc, &mut update);
        self.history
            .push(format!("select:{}:{:?}", block_id, detail_level));
        update.focus = self.focus;
        update
    }

    pub fn remove_block(&mut self, block_id: BlockId) -> CodeGraphContextUpdate {
        let mut update = CodeGraphContextUpdate::default();
        if self.selected.remove(&block_id).is_some() {
            update.removed.push(block_id);
            if self.focus == Some(block_id) {
                self.focus = None;
            }
            self.history.push(format!("remove:{}", block_id));
        }
        update.focus = self.focus;
        update
    }

    pub fn pin(&mut self, block_id: BlockId, pinned: bool) -> CodeGraphContextUpdate {
        let mut update = CodeGraphContextUpdate::default();
        if let Some(node) = self.selected.get_mut(&block_id) {
            node.pinned = pinned;
            update.changed.push(block_id);
            self.history.push(format!(
                "{}:{}",
                if pinned { "pin" } else { "unpin" },
                block_id
            ));
        }
        update.focus = self.focus;
        update
    }

    pub fn seed_overview(&mut self, doc: &Document) -> CodeGraphContextUpdate {
        let index = CodeGraphQueryIndex::new(doc);
        let previous: HashSet<_> = self.selected.keys().copied().collect();
        self.selected.clear();
        self.focus = None;

        let mut update = CodeGraphContextUpdate::default();
        let mut selected = Vec::new();
        for block_id in index.overview_nodes() {
            self.ensure_selected_with_origin(
                block_id,
                CodeGraphDetailLevel::Skeleton,
                selection_origin(CodeGraphSelectionOriginKind::Overview, None, None),
                &mut update,
            );
            selected.push(block_id);
        }

        if self.focus.is_none() {
            self.focus = selected.first().copied().or(Some(doc.root));
        }
        self.apply_prune_policy(doc, &mut update);
        update.focus = self.focus;
        update.removed = previous
            .into_iter()
            .filter(|block_id| !self.selected.contains_key(block_id))
            .collect();
        update.removed.sort_by_key(BlockId::to_string);
        self.history.push("seed:overview".to_string());
        update
    }

    pub fn expand_file(&mut self, doc: &Document, file_id: BlockId) -> CodeGraphContextUpdate {
        let index = CodeGraphQueryIndex::new(doc);
        let mut update = CodeGraphContextUpdate::default();
        self.ensure_selected_with_origin(
            file_id,
            CodeGraphDetailLevel::Neighborhood,
            selection_origin(CodeGraphSelectionOriginKind::Manual, None, None),
            &mut update,
        );
        for symbol_id in index.file_symbols(&file_id) {
            self.ensure_selected_with_origin(
                symbol_id,
                CodeGraphDetailLevel::SymbolCard,
                selection_origin(CodeGraphSelectionOriginKind::FileSymbols, None, Some(file_id)),
                &mut update,
            );
        }
        self.focus = Some(file_id);
        self.apply_prune_policy(doc, &mut update);
        update.focus = self.focus;
        self.history.push(format!("expand:file:{}", file_id));
        update
    }

    pub fn expand_dependencies(
        &mut self,
        doc: &Document,
        block_id: BlockId,
        relation_filter: Option<&str>,
    ) -> CodeGraphContextUpdate {
        self.expand_neighbors(doc, block_id, relation_filter, TraversalKind::Outgoing)
    }

    pub fn expand_dependents(
        &mut self,
        doc: &Document,
        block_id: BlockId,
        relation_filter: Option<&str>,
    ) -> CodeGraphContextUpdate {
        self.expand_neighbors(doc, block_id, relation_filter, TraversalKind::Incoming)
    }

    pub fn collapse(
        &mut self,
        doc: &Document,
        block_id: BlockId,
        include_descendants: bool,
    ) -> CodeGraphContextUpdate {
        let index = CodeGraphQueryIndex::new(doc);
        let mut update = CodeGraphContextUpdate::default();
        let mut to_remove = vec![block_id];
        if include_descendants {
            to_remove.extend(index.descendants(block_id));
        }

        for id in to_remove {
            let Some(node) = self.selected.get(&id) else {
                continue;
            };
            if node.pinned {
                update
                    .warnings
                    .push(format!("{} is pinned and was not removed", id));
                continue;
            }
            self.selected.remove(&id);
            update.removed.push(id);
            if self.focus == Some(id) {
                self.focus = None;
            }
        }

        if self.focus.is_none() {
            self.focus = self.selected.keys().next().copied();
        }
        update.focus = self.focus;
        self.history
            .push(format!("collapse:{}:{}", block_id, include_descendants));
        update
    }

    pub fn hydrate_source(
        &mut self,
        doc: &Document,
        block_id: BlockId,
        padding: usize,
    ) -> CodeGraphContextUpdate {
        let mut update = CodeGraphContextUpdate::default();
        self.ensure_selected_with_origin(
            block_id,
            CodeGraphDetailLevel::Source,
            selection_origin(CodeGraphSelectionOriginKind::Manual, None, None),
            &mut update,
        );
        match hydrate_source_excerpt(doc, block_id, padding) {
            Ok(Some(excerpt)) => {
                if let Some(node) = self.selected.get_mut(&block_id) {
                    node.detail_level = CodeGraphDetailLevel::Source;
                    node.hydrated_source = Some(excerpt);
                    update.changed.push(block_id);
                }
            }
            Ok(None) => update
                .warnings
                .push(format!("no coderef available for {}", block_id)),
            Err(error) => update.warnings.push(error),
        }
        self.focus = Some(block_id);
        self.apply_prune_policy(doc, &mut update);
        update.focus = self.focus;
        self.history.push(format!("hydrate:{}", block_id));
        update
    }

    pub fn prune(&mut self, doc: &Document, max_selected: Option<usize>) -> CodeGraphContextUpdate {
        let mut update = CodeGraphContextUpdate::default();
        if let Some(limit) = max_selected {
            self.prune_policy.max_selected = limit.max(1);
        }
        self.apply_prune_policy(doc, &mut update);
        self.history
            .push(format!("prune:{}", self.prune_policy.max_selected));
        update.focus = self.focus;
        update
    }

    pub fn render_for_prompt(&self, doc: &Document, config: &CodeGraphRenderConfig) -> String {
        let index = CodeGraphQueryIndex::new(doc);
        let summary = self.summary(doc);
        let short_ids = make_short_ids(self, &index);
        let selected_ids: HashSet<_> = self.selected.keys().copied().collect();

        let mut repository_nodes = Vec::new();
        let mut directory_nodes = Vec::new();
        let mut file_nodes = Vec::new();
        let mut symbol_nodes = Vec::new();

        for block_id in self.selected_block_ids() {
            match index.node_class(&block_id).unwrap_or("unknown") {
                "repository" => repository_nodes.push(block_id),
                "directory" => directory_nodes.push(block_id),
                "file" => file_nodes.push(block_id),
                "symbol" => symbol_nodes.push(block_id),
                _ => {}
            }
        }

        let mut out = String::new();
        let _ = writeln!(out, "CodeGraph working set");
        let focus = self.focus.and_then(|id| render_reference(doc, &index, &short_ids, id));
        let _ = writeln!(
            out,
            "focus: {}",
            focus.unwrap_or_else(|| "none".to_string())
        );
        let _ = writeln!(
            out,
            "summary: selected={}/{} repositories={} directories={} files={} symbols={} hydrated={}",
            summary.selected,
            summary.max_selected,
            summary.repositories,
            summary.directories,
            summary.files,
            summary.symbols,
            summary.hydrated_sources
        );

        if !repository_nodes.is_empty() || !directory_nodes.is_empty() || !file_nodes.is_empty() {
            let _ = writeln!(out, "\nfilesystem:");
            for block_id in repository_nodes
                .into_iter()
                .chain(directory_nodes.into_iter())
                .chain(file_nodes.into_iter())
            {
                let block = match doc.get_block(&block_id) {
                    Some(block) => block,
                    None => continue,
                };
                let short = short_ids
                    .get(&block_id)
                    .cloned()
                    .unwrap_or_else(|| block_id.to_string());
                let label = index
                    .display_label(doc, &block_id)
                    .unwrap_or_else(|| block_id.to_string());
                let language = block
                    .metadata
                    .custom
                    .get(META_LANGUAGE)
                    .and_then(Value::as_str)
                    .map(|value| format!(" [{}]", value))
                    .unwrap_or_default();
                let pin = self
                    .selected
                    .get(&block_id)
                    .filter(|node| node.pinned)
                    .map(|_| " [pinned]")
                    .unwrap_or("");
                let _ = writeln!(out, "- [{}] {}{}{}", short, label, language, pin);
            }
        }

        if !symbol_nodes.is_empty() {
            let _ = writeln!(out, "\nopened symbols:");
            for block_id in symbol_nodes {
                let Some(block) = doc.get_block(&block_id) else {
                    continue;
                };
                let Some(node) = self.selected.get(&block_id) else {
                    continue;
                };
                let short = short_ids
                    .get(&block_id)
                    .cloned()
                    .unwrap_or_else(|| block_id.to_string());
                let coderef = metadata_coderef_display(block)
                    .or_else(|| content_coderef_display(block))
                    .unwrap_or_else(|| index.display_label(doc, &block_id).unwrap_or_else(|| block_id.to_string()));
                let pin = if node.pinned { " [pinned]" } else { "" };
                let _ = writeln!(
                    out,
                    "- [{}] {}{} @ {}",
                    short,
                    format_symbol_signature(block),
                    format_symbol_modifiers(block),
                    coderef
                );
                if !pin.is_empty() {
                    let _ = writeln!(out, "  flags:{}", pin);
                }
                if let Some(description) = content_string(block, "description")
                    .or_else(|| block.metadata.summary.clone())
                {
                    let _ = writeln!(out, "  docs: {}", description);
                }

                if node.detail_level.includes_neighborhood() {
                    render_edge_section(
                        &mut out,
                        "outgoing",
                        index.outgoing_edges(&block_id),
                        &selected_ids,
                        &short_ids,
                        doc,
                        &index,
                        config.max_edges_per_node,
                    );
                    render_edge_section(
                        &mut out,
                        "incoming",
                        index.incoming_edges(&block_id),
                        &selected_ids,
                        &short_ids,
                        doc,
                        &index,
                        config.max_edges_per_node,
                    );
                }

                if node.detail_level.includes_source() {
                    if let Some(source) = &node.hydrated_source {
                        let _ = writeln!(out, "  source: {}:{}-{}", source.path, source.start_line, source.end_line);
                        for line in source
                            .snippet
                            .lines()
                            .take(config.max_source_lines)
                        {
                            let _ = writeln!(out, "    {}", line);
                        }
                    }
                }
            }
        }

        let total_symbols = index.total_symbols();
        let omitted_symbols = total_symbols.saturating_sub(summary.symbols);
        let _ = writeln!(out, "\nomissions:");
        let _ = writeln!(out, "- symbols omitted from working set: {}", omitted_symbols);
        let _ = writeln!(
            out,
            "- prune policy: max_selected={} demote_before_remove={} protect_focus={}",
            self.prune_policy.max_selected,
            self.prune_policy.demote_before_remove,
            self.prune_policy.protect_focus
        );

        let _ = writeln!(out, "\nfrontier:");
        if let Some(focus_id) = self.focus {
            match index.node_class(&focus_id).unwrap_or("unknown") {
                "file" => {
                    let short = short_ids
                        .get(&focus_id)
                        .cloned()
                        .unwrap_or_else(|| focus_id.to_string());
                    let _ = writeln!(out, "- [{}] expand file symbols", short);
                    let _ = writeln!(out, "- [{}] hydrate file source", short);
                }
                "symbol" => {
                    let short = short_ids
                        .get(&focus_id)
                        .cloned()
                        .unwrap_or_else(|| focus_id.to_string());
                    let _ = writeln!(out, "- [{}] expand dependencies", short);
                    let _ = writeln!(out, "- [{}] expand dependents", short);
                    let _ = writeln!(out, "- [{}] hydrate source", short);
                    let _ = writeln!(out, "- [{}] collapse", short);
                }
                _ => {
                    let _ = writeln!(out, "- set focus to a file or symbol to expand the working set");
                }
            }
        } else {
            let _ = writeln!(out, "- no focus block set");
        }

        out.trim_end().to_string()
    }

    pub fn export(&self, doc: &Document, config: &CodeGraphRenderConfig) -> CodeGraphContextExport {
        let index = CodeGraphQueryIndex::new(doc);
        let summary = self.summary(doc);
        let short_ids = make_short_ids(self, &index);
        let selected_ids: HashSet<_> = self.selected.keys().copied().collect();
        let rendered = self.render_for_prompt(doc, config);

        let mut nodes = Vec::new();
        for block_id in self.selected_block_ids() {
            let Some(block) = doc.get_block(&block_id) else {
                continue;
            };
            let Some(node) = self.selected.get(&block_id) else {
                continue;
            };
            let node_class = index.node_class(&block_id).unwrap_or("unknown").to_string();
            let label = index
                .display_label(doc, &block_id)
                .unwrap_or_else(|| block_id.to_string());
            let logical_key = block_logical_key(block);
            let signature = if node_class == "symbol" {
                Some(format!(
                    "{}{}",
                    format_symbol_signature(block),
                    format_symbol_modifiers(block)
                ))
            } else {
                None
            };
            let docs = content_string(block, "description").or_else(|| block.metadata.summary.clone());
            let coderef = block_coderef(block).map(|coderef| CodeGraphCoderef {
                path: coderef.path,
                display: coderef.display,
                start_line: coderef.start_line,
                end_line: coderef.end_line,
            });

            nodes.push(CodeGraphContextNodeExport {
                block_id,
                short_id: short_ids
                    .get(&block_id)
                    .cloned()
                    .unwrap_or_else(|| block_id.to_string()),
                node_class,
                label,
                detail_level: node.detail_level,
                pinned: node.pinned,
                logical_key,
                signature,
                docs,
                origin: node.origin.clone(),
                coderef,
                hydrated_source: node.hydrated_source.clone(),
            });
        }

        let mut edges = Vec::new();
        for source in self.selected_block_ids() {
            for edge in index.outgoing_edges(&source) {
                if !selected_ids.contains(&edge.other) {
                    continue;
                }
                edges.push(CodeGraphContextEdgeExport {
                    source,
                    source_short_id: short_ids
                        .get(&source)
                        .cloned()
                        .unwrap_or_else(|| source.to_string()),
                    target: edge.other,
                    target_short_id: short_ids
                        .get(&edge.other)
                        .cloned()
                        .unwrap_or_else(|| edge.other.to_string()),
                    relation: edge.relation,
                });
            }
        }
        edges.sort_by_key(|edge| {
            (
                edge.source_short_id.clone(),
                edge.relation.clone(),
                edge.target_short_id.clone(),
            )
        });

        let frontier = self.export_frontier(doc, &index, &short_ids, &selected_ids);
        let omitted_symbol_count = index.total_symbols().saturating_sub(summary.symbols);

        CodeGraphContextExport {
            summary,
            focus: self.focus,
            focus_short_id: self
                .focus
                .and_then(|id| short_ids.get(&id).cloned()),
            focus_label: self.focus.and_then(|id| index.display_label(doc, &id)),
            nodes,
            edges,
            frontier,
            omitted_symbol_count,
            rendered,
        }
    }

    fn export_frontier(
        &self,
        doc: &Document,
        index: &CodeGraphQueryIndex,
        short_ids: &HashMap<BlockId, String>,
        selected_ids: &HashSet<BlockId>,
    ) -> Vec<CodeGraphContextFrontierAction> {
        let Some(focus_id) = self.focus else {
            return Vec::new();
        };
        let short_id = short_ids
            .get(&focus_id)
            .cloned()
            .unwrap_or_else(|| focus_id.to_string());
        let label = index
            .display_label(doc, &focus_id)
            .unwrap_or_else(|| focus_id.to_string());
        match index.node_class(&focus_id).unwrap_or("unknown") {
            "file" => {
                let hidden = index
                    .file_symbols(&focus_id)
                    .into_iter()
                    .filter(|id| !selected_ids.contains(id))
                    .count();
                vec![CodeGraphContextFrontierAction {
                    block_id: focus_id,
                    short_id,
                    action: "expand_file".to_string(),
                    relation: None,
                    direction: None,
                    candidate_count: hidden,
                    description: format!("Expand file symbols for {}", label),
                }]
            }
            "symbol" => {
                let mut actions = Vec::new();
                append_relation_frontier(
                    &mut actions,
                    focus_id,
                    &short_id,
                    &label,
                    index.outgoing_edges(&focus_id),
                    selected_ids,
                    "expand_dependencies",
                    "outgoing",
                );
                append_relation_frontier(
                    &mut actions,
                    focus_id,
                    &short_id,
                    &label,
                    index.incoming_edges(&focus_id),
                    selected_ids,
                    "expand_dependents",
                    "incoming",
                );
                actions.push(CodeGraphContextFrontierAction {
                    block_id: focus_id,
                    short_id: short_id.clone(),
                    action: "hydrate_source".to_string(),
                    relation: None,
                    direction: None,
                    candidate_count: usize::from(
                        self.selected
                            .get(&focus_id)
                            .and_then(|node| node.hydrated_source.as_ref())
                            .is_none(),
                    ),
                    description: format!("Hydrate source for {}", label),
                });
                actions.push(CodeGraphContextFrontierAction {
                    block_id: focus_id,
                    short_id,
                    action: "collapse".to_string(),
                    relation: None,
                    direction: None,
                    candidate_count: 1,
                    description: format!("Collapse {} from working set", label),
                });
                actions
            }
            _ => Vec::new(),
        }
    }

    fn ensure_selected_with_origin(
        &mut self,
        block_id: BlockId,
        detail_level: CodeGraphDetailLevel,
        origin: Option<CodeGraphSelectionOrigin>,
        update: &mut CodeGraphContextUpdate,
    ) {
        match self.selected.get_mut(&block_id) {
            Some(node) => {
                let next = node.detail_level.max(detail_level);
                if next != node.detail_level {
                    node.detail_level = next;
                    update.changed.push(block_id);
                }
                if origin_is_more_protective(origin.as_ref(), node.origin.as_ref()) {
                    node.origin = origin;
                    push_unique(&mut update.changed, block_id);
                }
            }
            None => {
                self.selected.insert(
                    block_id,
                    CodeGraphContextNode {
                        block_id,
                        detail_level,
                        pinned: false,
                        origin,
                        hydrated_source: None,
                    },
                );
                update.added.push(block_id);
            }
        }
    }

    fn expand_neighbors(
        &mut self,
        doc: &Document,
        block_id: BlockId,
        relation_filter: Option<&str>,
        traversal: TraversalKind,
    ) -> CodeGraphContextUpdate {
        let index = CodeGraphQueryIndex::new(doc);
        let mut update = CodeGraphContextUpdate::default();
        self.ensure_selected_with_origin(
            block_id,
            CodeGraphDetailLevel::Neighborhood,
            selection_origin(CodeGraphSelectionOriginKind::Manual, relation_filter, None),
            &mut update,
        );

        let edges = match traversal {
            TraversalKind::Outgoing => index.outgoing_edges(&block_id),
            TraversalKind::Incoming => index.incoming_edges(&block_id),
        };

        for edge in edges {
            if relation_filter.map(|filter| filter != edge.relation).unwrap_or(false) {
                continue;
            }
            let class = index.node_class(&edge.other).unwrap_or("unknown");
            let level = if class == "symbol" {
                CodeGraphDetailLevel::SymbolCard
            } else {
                CodeGraphDetailLevel::Skeleton
            };
            self.ensure_selected_with_origin(
                edge.other,
                level,
                selection_origin(
                    match traversal {
                        TraversalKind::Outgoing => CodeGraphSelectionOriginKind::Dependencies,
                        TraversalKind::Incoming => CodeGraphSelectionOriginKind::Dependents,
                    },
                    Some(edge.relation.as_str()),
                    Some(block_id),
                ),
                &mut update,
            );
        }

        self.focus = Some(block_id);
        self.apply_prune_policy(doc, &mut update);
        update.focus = self.focus;
        self.history.push(format!(
            "expand:{}:{}:{}",
            match traversal {
                TraversalKind::Outgoing => "dependencies",
                TraversalKind::Incoming => "dependents",
            },
            block_id,
            relation_filter.unwrap_or("*")
        ));
        update
    }

    fn apply_prune_policy(&mut self, doc: &Document, update: &mut CodeGraphContextUpdate) {
        if self.selected.len() <= self.prune_policy.max_selected.max(1) {
            return;
        }

        let index = CodeGraphQueryIndex::new(doc);
        let protected_focus = if self.prune_policy.protect_focus {
            self.focus
        } else {
            None
        };

        if self.prune_policy.demote_before_remove {
            while self.selected.len() > self.prune_policy.max_selected.max(1) {
                let Some(block_id) = self.next_demotable_block(&index, protected_focus) else {
                    break;
                };
                let Some(node) = self.selected.get_mut(&block_id) else {
                    continue;
                };
                let next_level = node.detail_level.demoted();
                if next_level == node.detail_level {
                    break;
                }
                node.detail_level = next_level;
                if !node.detail_level.includes_source() {
                    node.hydrated_source = None;
                }
                push_unique(&mut update.changed, block_id);
            }
        }

        while self.selected.len() > self.prune_policy.max_selected.max(1) {
            let Some(block_id) = self.next_removable_block(&index, protected_focus) else {
                update.warnings.push(format!(
                    "working set has {} nodes but no removable nodes remain under current prune policy",
                    self.selected.len()
                ));
                break;
            };
            self.selected.remove(&block_id);
            push_unique(&mut update.removed, block_id);
            update.added.retain(|id| id != &block_id);
            update.changed.retain(|id| id != &block_id);
            if self.focus == Some(block_id) {
                self.focus = None;
            }
        }

        if self.focus.is_none() {
            self.focus = self.next_focus_candidate(&index);
        }
        update.focus = self.focus;
    }

    fn next_demotable_block(
        &self,
        index: &CodeGraphQueryIndex,
        protected_focus: Option<BlockId>,
    ) -> Option<BlockId> {
        self.selected
            .values()
            .filter(|node| Some(node.block_id) != protected_focus && !node.pinned)
            .filter(|node| node.detail_level.demoted() != node.detail_level)
            .max_by_key(|node| {
                (
                    origin_prune_rank(node.origin.as_ref(), &self.prune_policy),
                    relation_prune_rank(node.origin.as_ref(), &self.prune_policy),
                    node.detail_level as u8,
                    prune_removal_rank(index.node_class(&node.block_id).unwrap_or("unknown")),
                    node.block_id.to_string(),
                )
            })
            .map(|node| node.block_id)
    }

    fn next_removable_block(
        &self,
        index: &CodeGraphQueryIndex,
        protected_focus: Option<BlockId>,
    ) -> Option<BlockId> {
        self.selected
            .values()
            .filter(|node| Some(node.block_id) != protected_focus && !node.pinned)
            .max_by_key(|node| {
                (
                    origin_prune_rank(node.origin.as_ref(), &self.prune_policy),
                    relation_prune_rank(node.origin.as_ref(), &self.prune_policy),
                    prune_removal_rank(index.node_class(&node.block_id).unwrap_or("unknown")),
                    node.detail_level as u8,
                    node.block_id.to_string(),
                )
            })
            .map(|node| node.block_id)
    }

    fn next_focus_candidate(&self, index: &CodeGraphQueryIndex) -> Option<BlockId> {
        self.selected
            .values()
            .min_by_key(|node| {
                (
                    focus_preference_rank(index.node_class(&node.block_id).unwrap_or("unknown")),
                    node.block_id.to_string(),
                )
            })
            .map(|node| node.block_id)
    }
}

#[derive(Debug, Clone, Copy)]
enum TraversalKind {
    Outgoing,
    Incoming,
}

impl CodeGraphQueryIndex {
    fn new(doc: &Document) -> Self {
        let mut logical_keys = HashMap::new();
        let mut logical_key_to_id = HashMap::new();
        let mut paths_to_id = HashMap::new();
        let mut display_to_id = HashMap::new();
        let mut symbol_names_to_id: HashMap<String, Vec<BlockId>> = HashMap::new();
        let mut node_classes = HashMap::new();
        let mut outgoing: HashMap<BlockId, Vec<IndexedEdge>> = HashMap::new();
        let mut incoming: HashMap<BlockId, Vec<IndexedEdge>> = HashMap::new();
        let mut file_symbols: HashMap<BlockId, Vec<BlockId>> = HashMap::new();
        let mut symbol_children: HashMap<BlockId, Vec<BlockId>> = HashMap::new();

        for (block_id, block) in &doc.blocks {
            if let Some(key) = block_logical_key(block) {
                logical_keys.insert(*block_id, key.clone());
                logical_key_to_id.insert(key, *block_id);
            }
            if let Some(class) = node_class(block) {
                node_classes.insert(*block_id, class.clone());
            }
            if let Some(path) = metadata_coderef_path(block).or_else(|| content_coderef_path(block)) {
                let should_replace = match paths_to_id.get(&path) {
                    Some(existing_id) => {
                        let existing_rank = path_selector_rank(
                            node_classes.get(existing_id).map(String::as_str).unwrap_or("unknown"),
                        );
                        let next_rank = path_selector_rank(
                            node_classes.get(block_id).map(String::as_str).unwrap_or("unknown"),
                        );
                        next_rank < existing_rank
                    }
                    None => true,
                };
                if should_replace {
                    paths_to_id.insert(path, *block_id);
                }
            }
            if let Some(display) = metadata_coderef_display(block).or_else(|| content_coderef_display(block)) {
                display_to_id.insert(display, *block_id);
            }
            let content_name = content_string(block, "name");
            if let Some(symbol_name) = block
                .metadata
                .custom
                .get(META_SYMBOL_NAME)
                .and_then(Value::as_str)
                .or(content_name.as_deref())
            {
                symbol_names_to_id
                    .entry(symbol_name.to_string())
                    .or_default()
                    .push(*block_id);
            }
        }

        for (source, block) in &doc.blocks {
            for edge in &block.edges {
                let relation = edge_type_to_string(&edge.edge_type);
                outgoing.entry(*source).or_default().push(IndexedEdge {
                    other: edge.target,
                    relation: relation.clone(),
                });
                incoming.entry(edge.target).or_default().push(IndexedEdge {
                    other: *source,
                    relation,
                });
            }
        }

        for (parent, children) in &doc.structure {
            let parent_class = node_classes.get(parent).map(String::as_str).unwrap_or("unknown");
            for child in children {
                let child_class = node_classes.get(child).map(String::as_str).unwrap_or("unknown");
                if parent_class == "file" && child_class == "symbol" {
                    file_symbols.entry(*parent).or_default().push(*child);
                }
                if parent_class == "symbol" && child_class == "symbol" {
                    symbol_children.entry(*parent).or_default().push(*child);
                }
            }
        }

        Self {
            logical_keys,
            logical_key_to_id,
            paths_to_id,
            display_to_id,
            symbol_names_to_id,
            node_classes,
            outgoing,
            incoming,
            file_symbols,
            symbol_children,
        }
    }

    fn resolve_selector(&self, selector: &str) -> Option<BlockId> {
        BlockId::from_str(selector)
            .ok()
            .or_else(|| self.logical_key_to_id.get(selector).copied())
            .or_else(|| self.paths_to_id.get(selector).copied())
            .or_else(|| self.display_to_id.get(selector).copied())
            .or_else(|| {
                self.symbol_names_to_id.get(selector).and_then(|ids| {
                    if ids.len() == 1 {
                        ids.first().copied()
                    } else {
                        None
                    }
                })
            })
    }

    fn overview_nodes(&self) -> Vec<BlockId> {
        let mut nodes: Vec<_> = self
            .node_classes
            .iter()
            .filter_map(|(block_id, class)| match class.as_str() {
                "repository" | "directory" | "file" => Some(*block_id),
                _ => None,
            })
            .collect();
        nodes.sort_by_key(|block_id| {
            self.logical_keys
                .get(block_id)
                .cloned()
                .unwrap_or_else(|| block_id.to_string())
        });
        nodes
    }

    fn outgoing_edges(&self, block_id: &BlockId) -> Vec<IndexedEdge> {
        self.outgoing.get(block_id).cloned().unwrap_or_default()
    }

    fn incoming_edges(&self, block_id: &BlockId) -> Vec<IndexedEdge> {
        self.incoming.get(block_id).cloned().unwrap_or_default()
    }

    fn file_symbols(&self, block_id: &BlockId) -> Vec<BlockId> {
        let mut symbols = self.file_symbols.get(block_id).cloned().unwrap_or_default();
        symbols.sort_by_key(|id| {
            self.logical_keys
                .get(id)
                .cloned()
                .unwrap_or_else(|| id.to_string())
        });
        symbols
    }

    fn descendants(&self, block_id: BlockId) -> Vec<BlockId> {
        let mut out = Vec::new();
        let mut queue: VecDeque<BlockId> = self
            .symbol_children
            .get(&block_id)
            .cloned()
            .unwrap_or_default()
            .into();
        while let Some(next) = queue.pop_front() {
            out.push(next);
            if let Some(children) = self.symbol_children.get(&next) {
                for child in children {
                    queue.push_back(*child);
                }
            }
        }
        out
    }

    fn node_class(&self, block_id: &BlockId) -> Option<&str> {
        self.node_classes.get(block_id).map(String::as_str)
    }

    fn total_symbols(&self) -> usize {
        self.node_classes
            .values()
            .filter(|class| class.as_str() == "symbol")
            .count()
    }

    fn display_label(&self, doc: &Document, block_id: &BlockId) -> Option<String> {
        let block = doc.get_block(block_id)?;
        match self.node_class(block_id) {
            Some("file") | Some("directory") | Some("repository") => metadata_coderef_path(block)
                .or_else(|| content_coderef_path(block))
                .or_else(|| block_logical_key(block)),
            Some("symbol") => block_logical_key(block)
                .or_else(|| metadata_coderef_display(block))
                .or_else(|| content_coderef_display(block)),
            _ => block_logical_key(block),
        }
    }
}

pub fn is_codegraph_document(doc: &Document) -> bool {
    let profile = doc
        .metadata
        .custom
        .get("profile")
        .and_then(Value::as_str);
    let marker = doc
        .metadata
        .custom
        .get("profile_marker")
        .and_then(Value::as_str);

    profile == Some("codegraph") || marker == Some(CODEGRAPH_PROFILE_MARKER)
}

pub fn resolve_codegraph_selector(doc: &Document, selector: &str) -> Option<BlockId> {
    CodeGraphQueryIndex::new(doc).resolve_selector(selector)
}

pub fn render_codegraph_context_prompt(
    doc: &Document,
    session: &CodeGraphContextSession,
    config: &CodeGraphRenderConfig,
) -> String {
    session.render_for_prompt(doc, config)
}

pub fn export_codegraph_context(
    doc: &Document,
    session: &CodeGraphContextSession,
    config: &CodeGraphRenderConfig,
) -> CodeGraphContextExport {
    session.export(doc, config)
}

pub fn approximate_prompt_tokens(rendered: &str) -> u32 {
    ((rendered.len() as f32) / 4.0).ceil() as u32
}

fn origin_is_more_protective(
    next: Option<&CodeGraphSelectionOrigin>,
    current: Option<&CodeGraphSelectionOrigin>,
) -> bool {
    match (next, current) {
        (Some(next), Some(current)) => selection_origin_protection_rank(next) < selection_origin_protection_rank(current),
        (Some(_), None) => true,
        _ => false,
    }
}

fn selection_origin_protection_rank(origin: &CodeGraphSelectionOrigin) -> u8 {
    match origin.kind {
        CodeGraphSelectionOriginKind::Manual => 0,
        CodeGraphSelectionOriginKind::Overview => 1,
        CodeGraphSelectionOriginKind::FileSymbols => 2,
        CodeGraphSelectionOriginKind::Dependencies => 3,
        CodeGraphSelectionOriginKind::Dependents => 4,
    }
}

fn origin_prune_rank(origin: Option<&CodeGraphSelectionOrigin>, policy: &CodeGraphPrunePolicy) -> u8 {
    let _ = policy;
    match origin.map(|origin| origin.kind) {
        Some(CodeGraphSelectionOriginKind::Dependents) => 5,
        Some(CodeGraphSelectionOriginKind::Dependencies) => 4,
        Some(CodeGraphSelectionOriginKind::FileSymbols) => 2,
        Some(CodeGraphSelectionOriginKind::Overview) => 1,
        Some(CodeGraphSelectionOriginKind::Manual) => 0,
        None => 0,
    }
}

fn relation_prune_rank(origin: Option<&CodeGraphSelectionOrigin>, policy: &CodeGraphPrunePolicy) -> u8 {
    origin
        .and_then(|origin| origin.relation.as_ref())
        .and_then(|relation| policy.relation_prune_priority.get(relation).copied())
        .unwrap_or(0)
}

fn push_unique(ids: &mut Vec<BlockId>, block_id: BlockId) {
    if !ids.contains(&block_id) {
        ids.push(block_id);
    }
}

fn prune_removal_rank(node_class: &str) -> u8 {
    match node_class {
        "symbol" => 4,
        "file" => 3,
        "directory" => 2,
        "repository" => 1,
        _ => 0,
    }
}

fn focus_preference_rank(node_class: &str) -> u8 {
    match node_class {
        "symbol" => 0,
        "file" => 1,
        "directory" => 2,
        "repository" => 3,
        _ => 4,
    }
}

fn path_selector_rank(node_class: &str) -> u8 {
    match node_class {
        "file" => 0,
        "directory" => 1,
        "repository" => 2,
        "symbol" => 3,
        _ => 4,
    }
}

fn render_edge_section(
    out: &mut String,
    label: &str,
    edges: Vec<IndexedEdge>,
    selected_ids: &HashSet<BlockId>,
    short_ids: &HashMap<BlockId, String>,
    doc: &Document,
    index: &CodeGraphQueryIndex,
    limit: usize,
) {
    let visible: Vec<_> = edges
        .into_iter()
        .filter(|edge| selected_ids.contains(&edge.other))
        .collect();

    if visible.is_empty() {
        return;
    }

    let _ = writeln!(out, "  {}:", label);
    for edge in visible.iter().take(limit) {
        let short = short_ids
            .get(&edge.other)
            .cloned()
            .unwrap_or_else(|| edge.other.to_string());
        let target = index
            .display_label(doc, &edge.other)
            .unwrap_or_else(|| edge.other.to_string());
        let _ = writeln!(out, "    - {} -> [{}] {}", edge.relation, short, target);
    }

    if visible.len() > limit {
        let _ = writeln!(out, "    - ... {} more", visible.len() - limit);
    }
}

fn append_relation_frontier(
    out: &mut Vec<CodeGraphContextFrontierAction>,
    block_id: BlockId,
    short_id: &str,
    label: &str,
    edges: Vec<IndexedEdge>,
    selected_ids: &HashSet<BlockId>,
    action: &str,
    direction: &str,
) {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for edge in edges {
        if selected_ids.contains(&edge.other) {
            continue;
        }
        *counts.entry(edge.relation).or_default() += 1;
    }
    for (relation, candidate_count) in counts {
        out.push(CodeGraphContextFrontierAction {
            block_id,
            short_id: short_id.to_string(),
            action: action.to_string(),
            relation: Some(relation.clone()),
            direction: Some(direction.to_string()),
            candidate_count,
            description: format!("{} {} neighbors via {} for {}", action, direction, relation, label),
        });
    }
}

fn make_short_ids(
    session: &CodeGraphContextSession,
    index: &CodeGraphQueryIndex,
) -> HashMap<BlockId, String> {
    let mut by_class: BTreeMap<&str, Vec<BlockId>> = BTreeMap::new();
    for block_id in session.selected.keys().copied() {
        by_class
            .entry(index.node_class(&block_id).unwrap_or("node"))
            .or_default()
            .push(block_id);
    }

    let mut result = HashMap::new();
    for (class, ids) in by_class {
        let mut ids = ids;
        ids.sort_by_key(|block_id| {
            index
                .logical_keys
                .get(block_id)
                .cloned()
                .unwrap_or_else(|| block_id.to_string())
        });
        for (idx, block_id) in ids.into_iter().enumerate() {
            let prefix = match class {
                "repository" => "R",
                "directory" => "D",
                "file" => "F",
                "symbol" => "S",
                _ => "N",
            };
            result.insert(block_id, format!("{}{}", prefix, idx + 1));
        }
    }
    result
}

fn render_reference(
    doc: &Document,
    index: &CodeGraphQueryIndex,
    short_ids: &HashMap<BlockId, String>,
    block_id: BlockId,
) -> Option<String> {
    Some(format!(
        "[{}] {}",
        short_ids.get(&block_id)?.clone(),
        index.display_label(doc, &block_id)?
    ))
}

fn edge_type_to_string(edge_type: &ucm_core::EdgeType) -> String {
    match edge_type {
        ucm_core::EdgeType::DerivedFrom => "derived_from".to_string(),
        ucm_core::EdgeType::Supersedes => "supersedes".to_string(),
        ucm_core::EdgeType::TransformedFrom => "transformed_from".to_string(),
        ucm_core::EdgeType::References => "references".to_string(),
        ucm_core::EdgeType::CitedBy => "cited_by".to_string(),
        ucm_core::EdgeType::LinksTo => "links_to".to_string(),
        ucm_core::EdgeType::Supports => "supports".to_string(),
        ucm_core::EdgeType::Contradicts => "contradicts".to_string(),
        ucm_core::EdgeType::Elaborates => "elaborates".to_string(),
        ucm_core::EdgeType::Summarizes => "summarizes".to_string(),
        ucm_core::EdgeType::ParentOf => "parent_of".to_string(),
        ucm_core::EdgeType::SiblingOf => "sibling_of".to_string(),
        ucm_core::EdgeType::PreviousSibling => "previous_sibling".to_string(),
        ucm_core::EdgeType::NextSibling => "next_sibling".to_string(),
        ucm_core::EdgeType::VersionOf => "version_of".to_string(),
        ucm_core::EdgeType::AlternativeOf => "alternative_of".to_string(),
        ucm_core::EdgeType::TranslationOf => "translation_of".to_string(),
        ucm_core::EdgeType::ChildOf => "child_of".to_string(),
        ucm_core::EdgeType::Custom(name) => name.clone(),
    }
}

fn hydrate_source_excerpt(
    doc: &Document,
    block_id: BlockId,
    padding: usize,
) -> Result<Option<HydratedSourceExcerpt>, String> {
    let Some(block) = doc.get_block(&block_id) else {
        return Err(format!("block not found: {}", block_id));
    };
    let coderef = block_coderef(block).ok_or_else(|| format!("missing coderef for {}", block_id))?;
    let repo = repository_root(doc).ok_or_else(|| "missing repository_path metadata".to_string())?;
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (repo, coderef, padding);
        Err("source hydration is not available on wasm32".to_string())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = repo.join(&coderef.path);
        let source = std::fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
        let lines: Vec<_> = source.lines().collect();
        let line_count = lines.len().max(1);
        let start_line = coderef.start_line.unwrap_or(1).max(1);
        let end_line = coderef.end_line.unwrap_or(start_line).max(start_line).min(line_count);
        let slice_start = start_line.saturating_sub(padding + 1);
        let slice_end = (end_line + padding).min(line_count);

        let mut snippet = String::new();
        for (idx, line) in lines[slice_start..slice_end].iter().enumerate() {
            let number = slice_start + idx + 1;
            let _ = writeln!(snippet, "{:>4} | {}", number, line);
        }

        Ok(Some(HydratedSourceExcerpt {
            path: coderef.path,
            display: coderef.display,
            start_line,
            end_line,
            snippet: snippet.trim_end().to_string(),
        }))
    }
}

fn repository_root(doc: &Document) -> Option<PathBuf> {
    doc.metadata
        .custom
        .get("repository_path")
        .and_then(Value::as_str)
        .map(PathBuf::from)
}

#[derive(Debug, Clone)]
struct BlockCoderef {
    path: String,
    display: String,
    start_line: Option<usize>,
    end_line: Option<usize>,
}

fn block_coderef(block: &Block) -> Option<BlockCoderef> {
    let value = block
        .metadata
        .custom
        .get(META_CODEREF)
        .or_else(|| match &block.content {
            Content::Json { value, .. } => value.get("coderef"),
            _ => None,
        })?;

    Some(BlockCoderef {
        path: value.get("path")?.as_str()?.to_string(),
        display: value
            .get("display")
            .and_then(Value::as_str)
            .unwrap_or_else(|| value.get("path").and_then(Value::as_str).unwrap_or("unknown"))
            .to_string(),
        start_line: value.get("start_line").and_then(Value::as_u64).map(|v| v as usize),
        end_line: value.get("end_line").and_then(Value::as_u64).map(|v| v as usize),
    })
}

fn format_symbol_signature(block: &Block) -> String {
    let kind = content_string(block, "kind").unwrap_or_else(|| "symbol".to_string());
    let name = content_string(block, "name").unwrap_or_else(|| "unknown".to_string());
    let inputs = content_array(block, "inputs")
        .into_iter()
        .map(|value| {
            let name = value.get("name").and_then(Value::as_str).unwrap_or("_");
            match value.get("type").and_then(Value::as_str) {
                Some(type_name) => format!("{}: {}", name, type_name),
                None => name.to_string(),
            }
        })
        .collect::<Vec<_>>();
    let output = content_string(block, "output");
    let type_info = content_string(block, "type");
    match kind.as_str() {
        "function" | "method" => {
            let mut rendered = format!("{} {}({})", kind, name, inputs.join(", "));
            if let Some(output) = output {
                let _ = write!(rendered, " -> {}", output);
            }
            rendered
        }
        _ => {
            let mut rendered = format!("{} {}", kind, name);
            if let Some(type_info) = type_info {
                let _ = write!(rendered, " : {}", type_info);
            }
            if block
                .metadata
                .custom
                .get(META_EXPORTED)
                .and_then(Value::as_bool)
                .unwrap_or(false)
            {
                let _ = write!(rendered, " [exported]");
            }
            rendered
        }
    }
}

fn format_symbol_modifiers(block: &Block) -> String {
    let Content::Json { value, .. } = &block.content else {
        return String::new();
    };
    let Some(modifiers) = value.get("modifiers").and_then(Value::as_object) else {
        return String::new();
    };

    let mut parts = Vec::new();
    if modifiers.get("async").and_then(Value::as_bool) == Some(true) {
        parts.push("async".to_string());
    }
    if modifiers.get("static").and_then(Value::as_bool) == Some(true) {
        parts.push("static".to_string());
    }
    if modifiers.get("generator").and_then(Value::as_bool) == Some(true) {
        parts.push("generator".to_string());
    }
    if let Some(visibility) = modifiers.get("visibility").and_then(Value::as_str) {
        parts.push(visibility.to_string());
    }

    if parts.is_empty() {
        String::new()
    } else {
        format!(" [{}]", parts.join(", "))
    }
}

fn content_string(block: &Block, field: &str) -> Option<String> {
    let Content::Json { value, .. } = &block.content else {
        return None;
    };
    value.get(field)?.as_str().map(|value| value.to_string())
}

fn content_array(block: &Block, field: &str) -> Vec<Value> {
    let Content::Json { value, .. } = &block.content else {
        return Vec::new();
    };
    value
        .get(field)
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}

fn node_class(block: &Block) -> Option<String> {
    block
        .metadata
        .custom
        .get(META_NODE_CLASS)
        .and_then(Value::as_str)
        .map(|value| value.to_string())
}

fn block_logical_key(block: &Block) -> Option<String> {
    block
        .metadata
        .custom
        .get(META_LOGICAL_KEY)
        .and_then(Value::as_str)
        .map(|value| value.to_string())
}

fn metadata_coderef_path(block: &Block) -> Option<String> {
    block
        .metadata
        .custom
        .get(META_CODEREF)
        .and_then(|value| value.get("path"))
        .and_then(Value::as_str)
        .map(|value| value.to_string())
}

fn metadata_coderef_display(block: &Block) -> Option<String> {
    block
        .metadata
        .custom
        .get(META_CODEREF)
        .and_then(|value| value.get("display"))
        .and_then(Value::as_str)
        .map(|value| value.to_string())
}

fn content_coderef_path(block: &Block) -> Option<String> {
    let Content::Json { value, .. } = &block.content else {
        return None;
    };
    value
        .get("coderef")
        .and_then(|value| value.get("path"))
        .and_then(Value::as_str)
        .map(|value| value.to_string())
}

fn content_coderef_display(block: &Block) -> Option<String> {
    let Content::Json { value, .. } = &block.content else {
        return None;
    };
    value
        .get("coderef")
        .and_then(|value| value.get("display"))
        .and_then(Value::as_str)
        .map(|value| value.to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;
    use crate::{build_code_graph, CodeGraphBuildInput, CodeGraphExtractorConfig};

    fn build_test_graph() -> Document {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::write(dir.path().join("src/util.rs"), "pub fn util() -> i32 { 1 }\n").unwrap();
        fs::write(
            dir.path().join("src/lib.rs"),
            "mod util;\n/// Add values.\npub async fn add(a: i32, b: i32) -> i32 { util::util() + a + b }\n\npub fn sub(a: i32, b: i32) -> i32 { util::util() + a - b }\n",
        )
        .unwrap();

        let repository_path = dir.path().to_path_buf();
        std::mem::forget(dir);

        build_code_graph(&CodeGraphBuildInput {
            repository_path,
            commit_hash: "context-tests".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap()
        .document
    }

    #[test]
    fn overview_expand_dependents_and_hydrate_source_work() {
        let doc = build_test_graph();
        let mut session = CodeGraphContextSession::new();
        let update = session.seed_overview(&doc);
        assert!(!update.added.is_empty());
        assert_eq!(session.summary(&doc).symbols, 0);

        let file_id = resolve_codegraph_selector(&doc, "src/lib.rs").unwrap();
        session.expand_file(&doc, file_id);
        assert!(session.summary(&doc).symbols >= 1);

        let add_id = resolve_codegraph_selector(&doc, "symbol:src/lib.rs::add").unwrap();
        let util_id = resolve_codegraph_selector(&doc, "symbol:src/util.rs::util").unwrap();
        let deps = session.expand_dependencies(&doc, add_id, Some("uses_symbol"));
        assert!(deps.added.contains(&util_id) || session.selected.contains_key(&util_id));

        let dependents = session.expand_dependents(&doc, util_id, Some("uses_symbol"));
        assert!(dependents.added.contains(&add_id) || session.selected.contains_key(&add_id));

        let hydrated = session.hydrate_source(&doc, add_id, 1);
        assert!(hydrated.changed.contains(&add_id));
        assert!(session
            .selected
            .get(&add_id)
            .and_then(|node| node.hydrated_source.as_ref())
            .is_some());

        let rendered = session.render_for_prompt(&doc, &CodeGraphRenderConfig::default());
        assert!(rendered.contains("CodeGraph working set"));
        assert!(rendered.contains("expand dependents"));
        assert!(rendered.contains("uses_symbol"));
        assert!(rendered.contains("source:"));
    }

    #[test]
    fn resolve_selector_prefers_logical_key_and_path() {
        let doc = build_test_graph();
        let file_id = resolve_codegraph_selector(&doc, "src/lib.rs").unwrap();
        let logical_id = resolve_codegraph_selector(&doc, "symbol:src/lib.rs::add").unwrap();
        let display_id = resolve_codegraph_selector(&doc, "src/lib.rs:2-2").unwrap_or(logical_id);
        assert!(doc.get_block(&file_id).is_some());
        assert!(doc.get_block(&logical_id).is_some());
        assert_eq!(logical_id, display_id);
    }

    #[test]
    fn prune_policy_demotes_before_removing() {
        let doc = build_test_graph();
        let mut session = CodeGraphContextSession::new();
        session.set_prune_policy(CodeGraphPrunePolicy {
            max_selected: 10,
            ..CodeGraphPrunePolicy::default()
        });

        let file_id = resolve_codegraph_selector(&doc, "src/lib.rs").unwrap();
        let add_id = resolve_codegraph_selector(&doc, "symbol:src/lib.rs::add").unwrap();
        session.seed_overview(&doc);
        session.expand_file(&doc, file_id);
        session.expand_dependencies(&doc, add_id, Some("uses_symbol"));
        session.hydrate_source(&doc, add_id, 1);
        assert!(session
            .selected
            .get(&add_id)
            .and_then(|node| node.hydrated_source.as_ref())
            .is_some());

        session.set_focus(&doc, Some(file_id));
        let update = session.prune(&doc, Some(4));
        assert!(session.selected.len() <= 4);
        assert!(!update.changed.is_empty() || !update.removed.is_empty());

        let rendered = session.render_for_prompt(&doc, &CodeGraphRenderConfig::default());
        assert!(rendered.contains("selected=4/4"));
        assert!(!rendered.contains("source:"));
    }

    #[test]
    fn prune_prefers_dependents_before_file_skeletons() {
        let doc = build_test_graph();
        let mut session = CodeGraphContextSession::new();
        session.set_prune_policy(CodeGraphPrunePolicy {
            max_selected: 20,
            ..CodeGraphPrunePolicy::default()
        });

        let util_file_id = resolve_codegraph_selector(&doc, "src/util.rs").unwrap();
        let util_symbol_id = resolve_codegraph_selector(&doc, "symbol:src/util.rs::util").unwrap();
        let lib_file_id = resolve_codegraph_selector(&doc, "src/lib.rs").unwrap();
        let add_id = resolve_codegraph_selector(&doc, "symbol:src/lib.rs::add").unwrap();
        let sub_id = resolve_codegraph_selector(&doc, "symbol:src/lib.rs::sub").unwrap();

        session.seed_overview(&doc);
        session.expand_file(&doc, util_file_id);
        session.expand_dependents(&doc, util_symbol_id, Some("uses_symbol"));
        assert!(session.selected.contains_key(&add_id));
        assert!(session.selected.contains_key(&sub_id));

        session.set_focus(&doc, Some(util_file_id));
        session.prune(&doc, Some(5));
        assert!(session.selected.contains_key(&lib_file_id));
        assert!(!session.selected.contains_key(&add_id));
        assert!(!session.selected.contains_key(&sub_id));
    }

    #[test]
    fn export_includes_frontier_and_origin_metadata() {
        let doc = build_test_graph();
        let mut session = CodeGraphContextSession::new();
        let file_id = resolve_codegraph_selector(&doc, "src/lib.rs").unwrap();
        let add_id = resolve_codegraph_selector(&doc, "symbol:src/lib.rs::add").unwrap();

        session.seed_overview(&doc);
        session.expand_file(&doc, file_id);
        session.focus = Some(add_id);
        let export = session.export(&doc, &CodeGraphRenderConfig::default());

        assert_eq!(export.focus, Some(add_id));
        assert!(export.nodes.iter().any(|node| {
            node.block_id == add_id
                && node
                    .origin
                    .as_ref()
                    .map(|origin| origin.kind == CodeGraphSelectionOriginKind::FileSymbols)
                    .unwrap_or(false)
        }));
        assert!(export
            .frontier
            .iter()
            .any(|action| action.action == "hydrate_source"));
        assert!(export
            .frontier
            .iter()
            .any(|action| action.action == "expand_dependencies" && action.relation.as_deref() == Some("uses_symbol")));
    }
}