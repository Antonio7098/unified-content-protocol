use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::Path,
    sync::Arc,
};

use regex::{Regex, RegexBuilder};
use ucm_core::{BlockId, Document, PortableDocument};

#[cfg(not(target_arch = "wasm32"))]
use crate::store::SqliteGraphStore;
use crate::{
    query::{GraphFindQuery, GraphNeighborMode},
    session::GraphSession,
    store::{
        GraphStore, GraphStoreError, GraphStoreObservability, GraphStoreStats, InMemoryGraphStore,
    },
    types::{GraphNodeSummary, GraphPathHop, GraphPathResult},
};

#[derive(Clone)]
pub struct GraphNavigator {
    store: Arc<dyn GraphStore>,
}

impl std::fmt::Debug for GraphNavigator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphNavigator")
            .field("stats", &self.store.stats())
            .finish()
    }
}

impl GraphNavigator {
    pub fn new(store: impl GraphStore + 'static) -> Self {
        Self {
            store: Arc::new(store),
        }
    }

    pub fn from_document(document: Document) -> Self {
        Self::new(InMemoryGraphStore::from_document(document))
    }

    pub fn from_portable(portable: &PortableDocument) -> Result<Self, GraphStoreError> {
        Ok(Self::new(InMemoryGraphStore::from_portable(portable)?))
    }

    pub fn from_json(payload: &str) -> Result<Self, GraphStoreError> {
        Ok(Self::new(InMemoryGraphStore::from_json(payload)?))
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, GraphStoreError> {
        Self::from_json(&std::fs::read_to_string(path)?)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn persist_sqlite(
        &self,
        path: impl AsRef<Path>,
        graph_key: impl Into<String>,
    ) -> Result<Self, GraphStoreError> {
        let portable = self.store.to_portable_document()?;
        Ok(Self::new(SqliteGraphStore::import_document(
            path, graph_key, &portable,
        )?))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn open_sqlite(
        path: impl AsRef<Path>,
        graph_key: impl Into<String>,
    ) -> Result<Self, GraphStoreError> {
        Ok(Self::new(SqliteGraphStore::open(path, graph_key)?))
    }

    pub fn store_stats(&self) -> GraphStoreStats {
        self.store.stats()
    }

    pub fn observability(&self) -> GraphStoreObservability {
        self.store.observability()
    }

    pub fn session(&self) -> GraphSession {
        GraphSession::new(self.clone())
    }

    pub fn root_id(&self) -> BlockId {
        self.store.root_id()
    }

    pub fn resolve_selector(&self, selector: &str) -> Option<BlockId> {
        self.store.resolve_selector(selector)
    }

    pub fn describe_node(&self, block_id: BlockId) -> Option<GraphNodeSummary> {
        let record = self.store.node(block_id)?;
        Some(GraphNodeSummary {
            block_id,
            label: record.label.unwrap_or_else(|| short_block_id(block_id)),
            content_type: record.content_type,
            semantic_role: record.semantic_role,
            tags: record.tags,
            parent: record.parent,
            children: record.children,
            outgoing_edges: record.outgoing_edges,
            incoming_edges: record.incoming_edges,
        })
    }

    pub fn describe(&self, selector: &str) -> Option<GraphNodeSummary> {
        self.resolve_selector(selector)
            .and_then(|block_id| self.describe_node(block_id))
    }

    pub fn find_nodes(
        &self,
        query: &GraphFindQuery,
    ) -> Result<Vec<GraphNodeSummary>, GraphStoreError> {
        let label = compile(query.label_regex.as_deref(), query.case_sensitive)?;
        let role = compile(query.semantic_role_regex.as_deref(), query.case_sensitive)?;
        let tag = compile(query.tag_regex.as_deref(), query.case_sensitive)?;
        let mut matches = self
            .store
            .node_ids()
            .into_iter()
            .filter_map(|id| self.store.node(id))
            .filter(|node| {
                query
                    .content_type
                    .as_ref()
                    .map(|value| &node.content_type == value)
                    .unwrap_or(true)
            })
            .filter(|node| regex_match_option(&label, node.label.as_deref()))
            .filter(|node| regex_match_option(&role, node.semantic_role.as_deref()))
            .filter(|node| {
                tag.as_ref()
                    .map(|compiled| node.tags.iter().any(|value| compiled.is_match(value)))
                    .unwrap_or(true)
            })
            .filter_map(|record| self.describe_node(record.block_id))
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

    pub fn path_between(
        &self,
        start: BlockId,
        end: BlockId,
        max_hops: usize,
    ) -> Option<GraphPathResult> {
        if start == end {
            return Some(GraphPathResult {
                start: self.describe_node(start)?,
                end: self.describe_node(end)?,
                hops: Vec::new(),
            });
        }

        let mut queue = VecDeque::from([(start, 0usize)]);
        let mut visited = HashSet::from([start]);
        let mut previous: HashMap<BlockId, (BlockId, GraphPathHop)> = HashMap::new();

        while let Some((current, depth)) = queue.pop_front() {
            if depth >= max_hops {
                continue;
            }
            for hop in self.neighbors(current, GraphNeighborMode::Neighborhood) {
                if !visited.insert(hop.to) {
                    continue;
                }
                previous.insert(hop.to, (current, hop.clone()));
                if hop.to == end {
                    let mut hops = Vec::new();
                    let mut cursor = end;
                    while let Some((parent, hop)) = previous.get(&cursor) {
                        hops.push(hop.clone());
                        cursor = *parent;
                        if cursor == start {
                            break;
                        }
                    }
                    hops.reverse();
                    return Some(GraphPathResult {
                        start: self.describe_node(start)?,
                        end: self.describe_node(end)?,
                        hops,
                    });
                }
                queue.push_back((hop.to, depth + 1));
            }
        }
        None
    }

    pub fn neighbors(&self, block_id: BlockId, mode: GraphNeighborMode) -> Vec<GraphPathHop> {
        let mut hops = Vec::new();
        if matches!(
            mode,
            GraphNeighborMode::Children | GraphNeighborMode::Neighborhood
        ) {
            hops.extend(
                self.store
                    .children(block_id)
                    .into_iter()
                    .map(|child| GraphPathHop {
                        from: block_id,
                        to: child,
                        relation: "contains".to_string(),
                        direction: "structural".to_string(),
                    }),
            );
        }
        if matches!(
            mode,
            GraphNeighborMode::Parents | GraphNeighborMode::Neighborhood
        ) {
            if let Some(parent) = self.store.parent(block_id) {
                hops.push(GraphPathHop {
                    from: block_id,
                    to: parent,
                    relation: "parent".to_string(),
                    direction: "structural".to_string(),
                });
            }
        }
        if matches!(
            mode,
            GraphNeighborMode::Outgoing | GraphNeighborMode::Neighborhood
        ) {
            hops.extend(
                self.store
                    .outgoing_edges(block_id)
                    .into_iter()
                    .map(|edge| GraphPathHop {
                        from: block_id,
                        to: edge.target,
                        relation: edge.relation,
                        direction: edge.direction,
                    }),
            );
        }
        if matches!(
            mode,
            GraphNeighborMode::Incoming | GraphNeighborMode::Neighborhood
        ) {
            hops.extend(
                self.store
                    .incoming_edges(block_id)
                    .into_iter()
                    .map(|edge| GraphPathHop {
                        from: block_id,
                        to: edge.source,
                        relation: edge.relation,
                        direction: edge.direction,
                    }),
            );
        }
        hops.sort_by(|left, right| {
            left.relation
                .cmp(&right.relation)
                .then(left.to.to_string().cmp(&right.to.to_string()))
        });
        hops
    }

    pub fn to_portable_document(&self) -> Result<PortableDocument, GraphStoreError> {
        self.store.to_portable_document()
    }

    pub fn to_document(&self) -> Result<Document, GraphStoreError> {
        Ok(self.store.to_portable_document()?.to_document()?)
    }

    pub fn to_json(&self) -> Result<String, GraphStoreError> {
        Ok(serde_json::to_string_pretty(
            &self.store.to_portable_document()?,
        )?)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), GraphStoreError> {
        std::fs::write(path, self.to_json()?)?;
        Ok(())
    }
}

fn compile(pattern: Option<&str>, case_sensitive: bool) -> Result<Option<Regex>, GraphStoreError> {
    pattern
        .map(|value| {
            RegexBuilder::new(value)
                .case_insensitive(!case_sensitive)
                .build()
        })
        .transpose()
        .map_err(Into::into)
}

fn regex_match_option(regex: &Option<Regex>, value: Option<&str>) -> bool {
    regex
        .as_ref()
        .map(|compiled| value.map(|inner| compiled.is_match(inner)).unwrap_or(false))
        .unwrap_or(true)
}

fn short_block_id(block_id: BlockId) -> String {
    block_id.to_string().chars().take(8).collect()
}
