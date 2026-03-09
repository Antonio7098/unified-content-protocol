use std::collections::HashMap;

use chrono::Utc;
use ucm_core::{BlockId, Document, PortableDocument};

use super::{GraphNodeRecord, GraphStore, GraphStoreObservability, GraphStoreStats};
use crate::types::GraphEdgeSummary;

#[derive(Debug, Clone)]
pub struct InMemoryGraphStore {
    document: Document,
    parent_by_child: HashMap<BlockId, BlockId>,
    label_index: HashMap<String, BlockId>,
    stats: GraphStoreStats,
}

impl InMemoryGraphStore {
    pub fn from_document(document: Document) -> Self {
        let mut parent_by_child = HashMap::new();
        for (parent, children) in &document.structure {
            for child in children {
                parent_by_child.insert(*child, *parent);
            }
        }

        let label_index = document
            .blocks
            .values()
            .filter_map(|block| {
                block
                    .metadata
                    .label
                    .as_ref()
                    .map(|label| (label.clone(), block.id))
            })
            .collect::<HashMap<_, _>>();

        let explicit_edge_count = document
            .blocks
            .values()
            .map(|block| block.edges.len())
            .sum();
        let structural_edge_count = parent_by_child.len();
        let stats = GraphStoreStats {
            backend: "memory".to_string(),
            document_id: document.id.0.clone(),
            root_block_id: document.root,
            node_count: document.blocks.len(),
            explicit_edge_count,
            structural_edge_count,
            captured_at: Utc::now(),
            graph_key: None,
        };

        Self {
            document,
            parent_by_child,
            label_index,
            stats,
        }
    }

    pub fn from_portable(portable: &PortableDocument) -> Result<Self, super::GraphStoreError> {
        Ok(Self::from_document(portable.to_document()?))
    }

    pub fn from_json(payload: &str) -> Result<Self, super::GraphStoreError> {
        let portable: PortableDocument = serde_json::from_str(payload)?;
        Self::from_portable(&portable)
    }

    pub fn document(&self) -> &Document {
        &self.document
    }
}

impl GraphStore for InMemoryGraphStore {
    fn stats(&self) -> GraphStoreStats {
        self.stats.clone()
    }

    fn observability(&self) -> GraphStoreObservability {
        GraphStoreObservability {
            stats: self.stats(),
            indexed_fields: vec![
                "block_id".to_string(),
                "label".to_string(),
                "parent".to_string(),
                "content_type".to_string(),
            ],
        }
    }

    fn root_id(&self) -> BlockId {
        self.document.root
    }

    fn node_ids(&self) -> Vec<BlockId> {
        let mut ids = self.document.blocks.keys().copied().collect::<Vec<_>>();
        ids.sort_by_key(|id| id.to_string());
        ids
    }

    fn node(&self, block_id: BlockId) -> Option<GraphNodeRecord> {
        let block = self.document.get_block(&block_id)?;
        Some(GraphNodeRecord {
            block_id,
            label: block.metadata.label.clone(),
            content_type: block.content_type().to_string(),
            semantic_role: block
                .metadata
                .semantic_role
                .as_ref()
                .map(ToString::to_string),
            tags: block.metadata.tags.clone(),
            parent: self.parent_by_child.get(&block_id).copied(),
            children: self.document.children(&block_id).len(),
            outgoing_edges: self.document.edge_index.outgoing_from(&block_id).len(),
            incoming_edges: self.document.edge_index.incoming_to(&block_id).len(),
        })
    }

    fn children(&self, block_id: BlockId) -> Vec<BlockId> {
        self.document.children(&block_id).to_vec()
    }

    fn parent(&self, block_id: BlockId) -> Option<BlockId> {
        self.parent_by_child.get(&block_id).copied()
    }

    fn outgoing_edges(&self, block_id: BlockId) -> Vec<GraphEdgeSummary> {
        self.document
            .edge_index
            .outgoing_from(&block_id)
            .iter()
            .map(|(edge_type, target)| GraphEdgeSummary {
                source: block_id,
                target: *target,
                relation: edge_relation(edge_type),
                direction: "outgoing".to_string(),
            })
            .collect()
    }

    fn incoming_edges(&self, block_id: BlockId) -> Vec<GraphEdgeSummary> {
        self.document
            .edge_index
            .incoming_to(&block_id)
            .iter()
            .map(|(edge_type, source)| GraphEdgeSummary {
                source: *source,
                target: block_id,
                relation: edge_relation(edge_type),
                direction: "incoming".to_string(),
            })
            .collect()
    }

    fn resolve_selector(&self, selector: &str) -> Option<BlockId> {
        if selector == "root" {
            return Some(self.document.root);
        }
        selector
            .parse::<BlockId>()
            .ok()
            .filter(|id| self.document.blocks.contains_key(id))
            .or_else(|| self.label_index.get(selector).copied())
    }

    fn to_portable_document(&self) -> Result<PortableDocument, super::GraphStoreError> {
        Ok(self.document.to_portable())
    }
}

fn edge_relation(edge_type: &ucm_core::EdgeType) -> String {
    match edge_type {
        ucm_core::EdgeType::Custom(value) => value.clone(),
        _ => serde_json::to_value(edge_type)
            .ok()
            .and_then(|value| value.as_str().map(ToOwned::to_owned))
            .unwrap_or_else(|| format!("{:?}", edge_type).to_lowercase()),
    }
}
