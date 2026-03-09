use anyhow::Result;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use ucm_core::{
    normalize::{canonical_json, normalize_content},
    Block, BlockId, Document, Edge, EdgeType,
};

use crate::model::*;

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

pub(super) fn normalize_temporal_fields(doc: &mut Document) {
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

pub(super) fn deterministic_timestamp() -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc)
}

pub(super) fn sort_structure_children_by_logical_key(doc: &mut Document) {
    let key_index = logical_key_index(doc);

    for children in doc.structure.values_mut() {
        children.sort_by(|a, b| {
            let ka = key_index.get(a).cloned().unwrap_or_else(|| a.to_string());
            let kb = key_index.get(b).cloned().unwrap_or_else(|| b.to_string());
            ka.cmp(&kb)
        });
    }
}

pub(super) fn sort_edges(doc: &mut Document) {
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

pub(super) fn compute_stats(doc: &Document) -> CodeGraphStats {
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

pub(super) fn block_logical_key(block: &Block) -> Option<String> {
    block
        .metadata
        .custom
        .get(META_LOGICAL_KEY)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

pub(super) fn block_path(block: &Block) -> Option<String> {
    block
        .metadata
        .custom
        .get(META_CODEREF)
        .and_then(|v| v.get("path"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

pub(super) fn node_class(block: &Block) -> Option<String> {
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

pub(super) fn validate_required_metadata(
    class_name: &str,
    block: &Block,
    diagnostics: &mut Vec<CodeGraphDiagnostic>,
) {
    let required = match class_name {
        "repository" => vec![META_LOGICAL_KEY, META_CODEREF],
        "directory" => vec![META_LOGICAL_KEY, META_CODEREF],
        "file" => vec![META_LOGICAL_KEY, META_CODEREF, META_LANGUAGE],
        "symbol" => vec![
            META_LOGICAL_KEY,
            META_CODEREF,
            META_LANGUAGE,
            META_SYMBOL_KIND,
            META_SYMBOL_NAME,
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

pub(super) fn logical_key_index(doc: &Document) -> HashMap<BlockId, String> {
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

pub(super) fn normalized_document_metadata(doc: &Document) -> serde_json::Value {
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

pub(super) fn normalized_block_metadata(block: &Block) -> serde_json::Value {
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

pub(super) fn normalized_edge_metadata(edge: &Edge) -> serde_json::Value {
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

pub(super) fn is_volatile_metadata_key(key: &str) -> bool {
    matches!(key, "generated_at" | "runtime" | "session" | "timestamp")
}
