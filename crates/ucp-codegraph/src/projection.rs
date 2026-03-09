use std::fmt::Write as _;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use ucm_core::{Block, BlockId, Content, Document, EdgeType};

use crate::model::{META_CODEREF, META_LANGUAGE, META_LOGICAL_KEY, META_NODE_CLASS};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGraphPromptProjectionConfig {
    #[serde(default = "default_max_files")]
    pub max_files: usize,
    #[serde(default = "default_max_symbols_total")]
    pub max_symbols_total: usize,
    #[serde(default = "default_max_symbols_per_file")]
    pub max_symbols_per_file: usize,
    #[serde(default = "default_max_edges_per_symbol")]
    pub max_edges_per_symbol: usize,
}

impl Default for CodeGraphPromptProjectionConfig {
    fn default() -> Self {
        Self {
            max_files: default_max_files(),
            max_symbols_total: default_max_symbols_total(),
            max_symbols_per_file: default_max_symbols_per_file(),
            max_edges_per_symbol: default_max_edges_per_symbol(),
        }
    }
}

const fn default_max_files() -> usize {
    40
}

const fn default_max_symbols_total() -> usize {
    160
}

const fn default_max_symbols_per_file() -> usize {
    8
}

const fn default_max_edges_per_symbol() -> usize {
    4
}

pub fn codegraph_prompt_projection(doc: &Document) -> String {
    codegraph_prompt_projection_with_config(doc, &CodeGraphPromptProjectionConfig::default())
}

pub fn codegraph_prompt_projection_with_config(
    doc: &Document,
    config: &CodeGraphPromptProjectionConfig,
) -> String {
    let repo = repository_block(doc);
    let mut file_ids = file_block_ids(doc);
    let file_total = file_ids.len();
    if file_ids.len() > config.max_files {
        file_ids.truncate(config.max_files);
    }

    let total_edges: usize = doc.blocks.values().map(|block| block.edges.len()).sum();
    let total_symbols = doc
        .blocks
        .values()
        .filter(|block| node_class(block).as_deref() == Some("symbol"))
        .count();

    let mut out = String::new();
    out.push_str("CodeGraph projection\n");
    if let Some(block) = repo {
        let name = content_string(block, "name").unwrap_or_else(|| "repository".to_string());
        let coderef = content_coderef_display(block).or_else(|| metadata_coderef_display(block));
        let _ = writeln!(
            out,
            "repo: {}{}",
            name,
            coderef
                .map(|value| format!(" @ {value}"))
                .unwrap_or_default()
        );
    }
    let _ = writeln!(
        out,
        "summary: files={} symbols={} edges={}",
        file_total, total_symbols, total_edges
    );

    if !file_ids.is_empty() {
        out.push_str("\nfiles:\n");
    }

    let mut emitted_symbols = 0usize;
    for file_id in file_ids {
        let Some(file_block) = doc.get_block(&file_id) else {
            continue;
        };
        let path = content_coderef_display(file_block)
            .or_else(|| metadata_coderef_display(file_block))
            .unwrap_or_else(|| block_logical_key(file_block).unwrap_or_else(|| "file".to_string()));
        let language = file_block
            .metadata
            .custom
            .get(META_LANGUAGE)
            .and_then(|value| value.as_str())
            .unwrap_or("unknown");
        let _ = writeln!(out, "- file {} [{}]", path, language);
        if let Some(description) = content_string(file_block, "description") {
            let _ = writeln!(out, "  docs: {}", description);
        }

        let descendants = symbol_descendants(doc, file_id);
        let remaining_total = config.max_symbols_total.saturating_sub(emitted_symbols);
        let take = descendants
            .len()
            .min(config.max_symbols_per_file)
            .min(remaining_total);
        for symbol_id in descendants.into_iter().take(take) {
            emitted_symbols += 1;
            render_symbol(doc, &mut out, &symbol_id, 1, config.max_edges_per_symbol);
        }

        if emitted_symbols >= config.max_symbols_total {
            let omitted = total_symbols.saturating_sub(emitted_symbols);
            if omitted > 0 {
                let _ = writeln!(out, "  … {} more symbols omitted by budget", omitted);
            }
            break;
        }
    }

    if file_total > config.max_files {
        let _ = writeln!(
            out,
            "\n… {} more files omitted by budget",
            file_total - config.max_files
        );
    }

    out.trim_end().to_string()
}

fn render_symbol(
    doc: &Document,
    out: &mut String,
    symbol_id: &BlockId,
    indent: usize,
    max_edges_per_symbol: usize,
) {
    let Some(block) = doc.get_block(symbol_id) else {
        return;
    };
    let pad = "  ".repeat(indent);
    let label = format_symbol_signature(block);
    let coderef = content_coderef_display(block)
        .or_else(|| metadata_coderef_display(block))
        .unwrap_or_else(|| block_logical_key(block).unwrap_or_else(|| "symbol".to_string()));
    let modifiers = format_symbol_modifiers(block);
    let _ = writeln!(out, "{}- {}{} @ {}", pad, label, modifiers, coderef);
    if let Some(description) =
        content_string(block, "description").or_else(|| block.metadata.summary.clone())
    {
        let _ = writeln!(out, "{}  docs: {}", pad, description);
    }

    let mut edges = rendered_edges(doc, block);
    if edges.len() > max_edges_per_symbol {
        edges.truncate(max_edges_per_symbol);
    }
    for edge in edges {
        let _ = writeln!(out, "{}  edge: {}", pad, edge);
    }

    for child in child_symbol_ids(doc, *symbol_id) {
        render_symbol(doc, out, &child, indent + 1, max_edges_per_symbol);
    }
}

fn rendered_edges(doc: &Document, block: &Block) -> Vec<String> {
    let mut rendered = block
        .edges
        .iter()
        .map(|edge| {
            let relation = edge
                .metadata
                .custom
                .get("relation")
                .and_then(|value| value.as_str())
                .or(match &edge.edge_type {
                    EdgeType::Custom(value) => Some(value.as_str()),
                    _ => None,
                })
                .unwrap_or("edge");
            let target = doc
                .get_block(&edge.target)
                .and_then(block_logical_key)
                .or_else(|| {
                    edge.metadata
                        .custom
                        .get("raw_target")
                        .and_then(|value| value.as_str())
                        .map(|value| value.to_string())
                })
                .unwrap_or_else(|| edge.target.to_string());
            format!("{} -> {}", relation, target)
        })
        .collect::<Vec<_>>();
    rendered.sort();
    rendered.dedup();
    rendered
}

fn repository_block(doc: &Document) -> Option<&Block> {
    doc.blocks
        .values()
        .find(|block| node_class(block).as_deref() == Some("repository"))
}

fn file_block_ids(doc: &Document) -> Vec<BlockId> {
    let mut files = doc
        .blocks
        .iter()
        .filter(|(_, block)| node_class(block).as_deref() == Some("file"))
        .map(|(id, _)| *id)
        .collect::<Vec<_>>();
    files.sort_by_key(|id| {
        doc.get_block(id)
            .and_then(content_coderef_display)
            .or_else(|| doc.get_block(id).and_then(metadata_coderef_display))
            .unwrap_or_else(|| id.to_string())
    });
    files
}

fn symbol_descendants(doc: &Document, root: BlockId) -> Vec<BlockId> {
    let mut out = Vec::new();
    let mut stack = doc.children(&root).to_vec();
    while let Some(block_id) = stack.pop() {
        let Some(block) = doc.get_block(&block_id) else {
            continue;
        };
        if node_class(block).as_deref() == Some("symbol") {
            out.push(block_id);
        }
        let mut children = doc.children(&block_id).to_vec();
        children.reverse();
        stack.extend(children);
    }
    out.sort_by_key(|id| sort_key_for_block(doc, id));
    out
}

fn child_symbol_ids(doc: &Document, root: BlockId) -> Vec<BlockId> {
    let mut children = doc
        .children(&root)
        .iter()
        .copied()
        .filter(|child| {
            doc.get_block(child)
                .map(|block| node_class(block).as_deref() == Some("symbol"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    children.sort_by_key(|id| sort_key_for_block(doc, id));
    children
}

fn sort_key_for_block(doc: &Document, block_id: &BlockId) -> (String, String) {
    let Some(block) = doc.get_block(block_id) else {
        return (String::new(), block_id.to_string());
    };
    (
        content_coderef_display(block)
            .or_else(|| metadata_coderef_display(block))
            .unwrap_or_default(),
        block_logical_key(block).unwrap_or_else(|| block_id.to_string()),
    )
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

fn metadata_coderef_display(block: &Block) -> Option<String> {
    block
        .metadata
        .custom
        .get(META_CODEREF)
        .and_then(|value| value.get("display"))
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

    #[test]
    fn prompt_projection_renders_compact_codegraph_view() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::write(
            dir.path().join("src/util.rs"),
            "pub fn util() -> i32 { 1 }\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("src/lib.rs"),
            "mod util;\n/// Add values.\npub async fn add(a: i32, b: i32) -> i32 { util::util() + a + b }\n",
        )
        .unwrap();

        let build = build_code_graph(&CodeGraphBuildInput {
            repository_path: dir.path().to_path_buf(),
            commit_hash: "projection".to_string(),
            config: CodeGraphExtractorConfig::default(),
        })
        .unwrap();

        let projection = codegraph_prompt_projection(&build.document);
        assert!(projection.contains("CodeGraph projection"));
        assert!(projection.contains("- file src/lib.rs [rust]"));
        assert!(projection.contains("function add(a: i32, b: i32) -> i32 [async, public]"));
        assert!(projection.contains("docs: Add values."));
        assert!(projection.contains("edge: uses_symbol -> symbol:src/util.rs::util"));
    }
}
