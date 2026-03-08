use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;
use regex::{Regex, RegexBuilder};
use ucm_core::{BlockId, Document, EdgeType};

use crate::model::{
    META_CODEREF, META_EXPORTED, META_LOGICAL_KEY, META_NODE_CLASS, META_SYMBOL_NAME,
};
use crate::{resolve_codegraph_selector, CodeGraphCoderef};

use super::types::{
    CodeGraphFindQuery, CodeGraphNodeSummary, CodeGraphPathHop, CodeGraphPathResult,
};

pub(super) fn describe_node(doc: &Document, block_id: BlockId) -> Option<CodeGraphNodeSummary> {
    let block = doc.get_block(&block_id)?;
    let coderef = coderef(block);
    let logical_key = string_meta(block, META_LOGICAL_KEY);
    Some(CodeGraphNodeSummary {
        block_id,
        node_class: string_meta(block, META_NODE_CLASS).unwrap_or_else(|| "unknown".to_string()),
        label: block
            .metadata
            .label
            .clone()
            .or_else(|| logical_key.clone())
            .or_else(|| string_meta(block, META_SYMBOL_NAME))
            .unwrap_or_else(|| short_block_id(block_id)),
        logical_key,
        symbol_name: string_meta(block, META_SYMBOL_NAME),
        path: coderef.as_ref().map(|value| value.path.clone()),
        exported: bool_meta(block, META_EXPORTED).unwrap_or(false),
        coderef,
    })
}

pub(super) fn find_nodes(
    doc: &Document,
    query: &CodeGraphFindQuery,
) -> Result<Vec<CodeGraphNodeSummary>> {
    let name = compile(query.name_regex.as_deref(), query.case_sensitive)?;
    let path = compile(query.path_regex.as_deref(), query.case_sensitive)?;
    let logical_key = compile(query.logical_key_regex.as_deref(), query.case_sensitive)?;
    let mut matches = doc
        .blocks
        .keys()
        .copied()
        .filter_map(|id| describe_node(doc, id))
        .filter(|node| {
            query
                .node_class
                .as_ref()
                .map(|value| &node.node_class == value)
                .unwrap_or(true)
        })
        .filter(|node| {
            query
                .exported
                .map(|value| node.exported == value)
                .unwrap_or(true)
        })
        .filter(|node| {
            regex_match(&name, &node.label)
                || regex_match_option(&name, node.symbol_name.as_deref())
        })
        .filter(|node| regex_match_option(&path, node.path.as_deref()))
        .filter(|node| regex_match_option(&logical_key, node.logical_key.as_deref()))
        .collect::<Vec<_>>();
    matches.sort_by(|left, right| {
        left.label
            .cmp(&right.label)
            .then(left.block_id.to_string().cmp(&right.block_id.to_string()))
    });
    if let Some(limit) = query.limit {
        matches.truncate(limit);
    }
    Ok(matches)
}

pub(super) fn path_between(
    doc: &Document,
    start: BlockId,
    end: BlockId,
    max_hops: usize,
) -> Option<CodeGraphPathResult> {
    if start == end {
        return Some(CodeGraphPathResult {
            start: describe_node(doc, start)?,
            end: describe_node(doc, end)?,
            hops: Vec::new(),
        });
    }

    let mut queue = VecDeque::from([(start, 0usize)]);
    let mut visited = HashSet::from([start]);
    let mut prev: HashMap<BlockId, (BlockId, CodeGraphPathHop)> = HashMap::new();
    while let Some((current, depth)) = queue.pop_front() {
        if depth >= max_hops {
            continue;
        }
        for hop in neighbors(doc, current) {
            if !visited.insert(hop.to) {
                continue;
            }
            prev.insert(hop.to, (current, hop.clone()));
            if hop.to == end {
                let mut hops = Vec::new();
                let mut cursor = end;
                while let Some((parent, hop)) = prev.get(&cursor) {
                    hops.push(hop.clone());
                    cursor = *parent;
                    if cursor == start {
                        break;
                    }
                }
                hops.reverse();
                return Some(CodeGraphPathResult {
                    start: describe_node(doc, start)?,
                    end: describe_node(doc, end)?,
                    hops,
                });
            }
            queue.push_back((hop.to, depth + 1));
        }
    }
    None
}

pub(super) fn resolve_required(doc: &Document, selector: &str) -> anyhow::Result<BlockId> {
    resolve_codegraph_selector(doc, selector)
        .ok_or_else(|| anyhow::anyhow!("No codegraph node matches selector: {}", selector))
}

fn neighbors(doc: &Document, current: BlockId) -> Vec<CodeGraphPathHop> {
    let mut result = Vec::new();
    if let Some(block) = doc.get_block(&current) {
        for edge in &block.edges {
            result.push(CodeGraphPathHop {
                from: current,
                to: edge.target,
                relation: edge_type_label(&edge.edge_type),
                direction: "outgoing".to_string(),
            });
        }
    }

    for (other_id, other) in &doc.blocks {
        for edge in &other.edges {
            if edge.target == current {
                result.push(CodeGraphPathHop {
                    from: current,
                    to: *other_id,
                    relation: edge_type_label(&edge.edge_type),
                    direction: "incoming".to_string(),
                });
            }
        }
    }

    for child in doc.children(&current) {
        result.push(CodeGraphPathHop {
            from: current,
            to: *child,
            relation: "contains".to_string(),
            direction: "structural".to_string(),
        });
    }
    if let Some(parent) = doc.parent(&current) {
        result.push(CodeGraphPathHop {
            from: current,
            to: *parent,
            relation: "parent".to_string(),
            direction: "structural".to_string(),
        });
    }

    result.sort_by(|left, right| {
        left.relation
            .cmp(&right.relation)
            .then(left.to.to_string().cmp(&right.to.to_string()))
    });
    result
}

fn compile(pattern: Option<&str>, case_sensitive: bool) -> Result<Option<Regex>> {
    pattern
        .map(|value| {
            RegexBuilder::new(value)
                .case_insensitive(!case_sensitive)
                .build()
        })
        .transpose()
        .map_err(Into::into)
}

fn regex_match(regex: &Option<Regex>, value: &str) -> bool {
    regex
        .as_ref()
        .map(|compiled| compiled.is_match(value))
        .unwrap_or(true)
}

fn regex_match_option(regex: &Option<Regex>, value: Option<&str>) -> bool {
    regex
        .as_ref()
        .map(|compiled| value.map(|inner| compiled.is_match(inner)).unwrap_or(false))
        .unwrap_or(true)
}

fn string_meta(block: &ucm_core::Block, key: &str) -> Option<String> {
    block
        .metadata
        .custom
        .get(key)?
        .as_str()
        .map(ToOwned::to_owned)
}

fn bool_meta(block: &ucm_core::Block, key: &str) -> Option<bool> {
    block.metadata.custom.get(key)?.as_bool()
}

fn coderef(block: &ucm_core::Block) -> Option<CodeGraphCoderef> {
    serde_json::from_value(block.metadata.custom.get(META_CODEREF)?.clone()).ok()
}

fn edge_type_label(edge_type: &EdgeType) -> String {
    match edge_type {
        EdgeType::Custom(value) => value.clone(),
        _ => serde_json::to_value(edge_type)
            .ok()
            .and_then(|value| value.as_str().map(ToOwned::to_owned))
            .unwrap_or_else(|| format!("{:?}", edge_type).to_lowercase()),
    }
}

fn short_block_id(block_id: BlockId) -> String {
    let value = block_id.to_string();
    value.chars().take(8).collect()
}
