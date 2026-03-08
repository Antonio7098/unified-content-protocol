use std::collections::{HashMap, HashSet, VecDeque};

use serde::{Deserialize, Serialize};
use ucm_core::BlockId;

use crate::{
    navigator::GraphNavigator,
    query::GraphNeighborMode,
    store::GraphStoreError,
    types::{GraphDetailLevel, GraphEdgeSummary, GraphNodeSummary},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphSelectionOriginKind {
    Overview,
    Manual,
    Children,
    Parents,
    Outgoing,
    Incoming,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSelectionOrigin {
    pub kind: GraphSelectionOriginKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<BlockId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSessionNode {
    pub detail_level: GraphDetailLevel,
    #[serde(default)]
    pub pinned: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<GraphSelectionOrigin>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GraphSessionUpdate {
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
pub struct GraphSessionSummary {
    pub selected: usize,
    pub pinned: usize,
    pub focused: bool,
    pub roots: usize,
    pub leaves: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSelectionExplanation {
    pub block_id: BlockId,
    pub selected: bool,
    pub focus: bool,
    pub pinned: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail_level: Option<GraphDetailLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<GraphSelectionOrigin>,
    pub explanation: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<GraphNodeSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<GraphNodeSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSessionDiff {
    #[serde(default)]
    pub added: Vec<GraphNodeSummary>,
    #[serde(default)]
    pub removed: Vec<GraphNodeSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus_before: Option<BlockId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus_after: Option<BlockId>,
    pub changed_focus: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphExportNode {
    pub block_id: BlockId,
    pub label: String,
    pub content_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_role: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub detail_level: GraphDetailLevel,
    pub pinned: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<BlockId>,
    pub children: usize,
    pub outgoing_edges: usize,
    pub incoming_edges: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphExportEdge {
    pub source: BlockId,
    pub target: BlockId,
    pub relation: String,
    pub direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphExport {
    pub summary: GraphSessionSummary,
    #[serde(default)]
    pub nodes: Vec<GraphExportNode>,
    #[serde(default)]
    pub edges: Vec<GraphExportEdge>,
}

#[derive(Debug, Clone)]
pub struct GraphSession {
    graph: GraphNavigator,
    selected: HashMap<BlockId, GraphSessionNode>,
    focus: Option<BlockId>,
    history: Vec<String>,
}

impl GraphSession {
    pub fn new(graph: GraphNavigator) -> Self {
        Self {
            graph,
            selected: HashMap::new(),
            focus: None,
            history: Vec::new(),
        }
    }

    pub fn selected_block_ids(&self) -> Vec<BlockId> {
        let mut ids = self.selected.keys().copied().collect::<Vec<_>>();
        ids.sort_by_key(|id| id.to_string());
        ids
    }

    pub fn summary(&self) -> GraphSessionSummary {
        let roots = self
            .selected
            .keys()
            .filter(|id| {
                self.graph
                    .describe_node(**id)
                    .map(|node| node.parent.is_none())
                    .unwrap_or(false)
            })
            .count();
        let leaves = self
            .selected
            .keys()
            .filter(|id| {
                self.graph
                    .describe_node(**id)
                    .map(|node| node.children == 0)
                    .unwrap_or(false)
            })
            .count();
        GraphSessionSummary {
            selected: self.selected.len(),
            pinned: self.selected.values().filter(|node| node.pinned).count(),
            focused: self.focus.is_some(),
            roots,
            leaves,
        }
    }

    pub fn fork(&self) -> Self {
        self.clone()
    }

    pub fn seed_overview(&mut self, max_depth: Option<usize>) -> GraphSessionUpdate {
        let max_depth = max_depth.unwrap_or(2).max(1);
        let mut update = GraphSessionUpdate::default();
        let root = self.graph.root_id();
        let mut queue = VecDeque::from([(root, 0usize)]);
        while let Some((current, depth)) = queue.pop_front() {
            self.select_id(
                current,
                GraphDetailLevel::Summary,
                Some(GraphSelectionOrigin {
                    kind: GraphSelectionOriginKind::Overview,
                    relation: None,
                    anchor: None,
                }),
                &mut update,
            );
            if depth < max_depth {
                for child in self.graph.neighbors(current, GraphNeighborMode::Children) {
                    queue.push_back((child.to, depth + 1));
                }
            }
        }
        update.focus = self.focus;
        self.history
            .push(format!("seed_overview depth={max_depth}"));
        update
    }

    pub fn focus(&mut self, selector: Option<&str>) -> Result<GraphSessionUpdate, GraphStoreError> {
        self.focus = selector.and_then(|value| self.graph.resolve_selector(value));
        Ok(GraphSessionUpdate {
            focus: self.focus,
            ..GraphSessionUpdate::default()
        })
    }

    pub fn select(
        &mut self,
        selector: &str,
        detail_level: GraphDetailLevel,
    ) -> Result<GraphSessionUpdate, GraphStoreError> {
        let block_id = self
            .graph
            .resolve_selector(selector)
            .ok_or_else(|| GraphStoreError::GraphNotFound(selector.to_string()))?;
        let mut update = GraphSessionUpdate::default();
        self.select_id(
            block_id,
            detail_level,
            Some(GraphSelectionOrigin {
                kind: GraphSelectionOriginKind::Manual,
                relation: None,
                anchor: None,
            }),
            &mut update,
        );
        self.history.push(format!("select {selector}"));
        Ok(update)
    }

    pub fn expand(
        &mut self,
        selector: &str,
        mode: GraphNeighborMode,
        depth: usize,
        max_add: Option<usize>,
    ) -> Result<GraphSessionUpdate, GraphStoreError> {
        let start = self
            .graph
            .resolve_selector(selector)
            .ok_or_else(|| GraphStoreError::GraphNotFound(selector.to_string()))?;
        let mut update = GraphSessionUpdate::default();
        let mut queue = VecDeque::from([(start, 0usize)]);
        let mut seen = HashSet::from([start]);
        let mut added = 0usize;

        while let Some((current, current_depth)) = queue.pop_front() {
            if current_depth >= depth.max(1) {
                continue;
            }
            for neighbor in self.graph.neighbors(current, mode) {
                if !seen.insert(neighbor.to) {
                    continue;
                }
                if max_add.map(|limit| added >= limit).unwrap_or(false) {
                    update.warnings.push(format!(
                        "Stopped expansion after reaching max_add={}",
                        max_add.unwrap_or_default()
                    ));
                    update.focus = self.focus;
                    return Ok(update);
                }
                self.select_id(
                    neighbor.to,
                    GraphDetailLevel::Summary,
                    Some(origin_for(mode, &neighbor.relation, start)),
                    &mut update,
                );
                queue.push_back((neighbor.to, current_depth + 1));
                added += 1;
            }
        }

        self.history
            .push(format!("expand {selector} mode={mode:?} depth={depth}"));
        update.focus = self.focus;
        Ok(update)
    }

    pub fn collapse(
        &mut self,
        selector: &str,
        include_descendants: bool,
    ) -> Result<GraphSessionUpdate, GraphStoreError> {
        let start = self
            .graph
            .resolve_selector(selector)
            .ok_or_else(|| GraphStoreError::GraphNotFound(selector.to_string()))?;
        let mut update = GraphSessionUpdate::default();
        let mut remove = vec![start];
        if include_descendants {
            let mut queue = VecDeque::from([start]);
            while let Some(current) = queue.pop_front() {
                for child in self.graph.neighbors(current, GraphNeighborMode::Children) {
                    remove.push(child.to);
                    queue.push_back(child.to);
                }
            }
        }
        for block_id in remove {
            if self.selected.remove(&block_id).is_some() {
                update.removed.push(block_id);
            }
        }
        if self
            .focus
            .map(|id| update.removed.contains(&id))
            .unwrap_or(false)
        {
            self.focus = None;
        }
        update.focus = self.focus;
        Ok(update)
    }

    pub fn pin(
        &mut self,
        selector: &str,
        pinned: bool,
    ) -> Result<GraphSessionUpdate, GraphStoreError> {
        let block_id = self
            .graph
            .resolve_selector(selector)
            .ok_or_else(|| GraphStoreError::GraphNotFound(selector.to_string()))?;
        let mut update = GraphSessionUpdate::default();
        if let Some(node) = self.selected.get_mut(&block_id) {
            node.pinned = pinned;
            update.changed.push(block_id);
        }
        update.focus = self.focus;
        Ok(update)
    }

    pub fn prune(&mut self, max_selected: Option<usize>) -> GraphSessionUpdate {
        let limit = max_selected.unwrap_or(32).max(1);
        let mut update = GraphSessionUpdate::default();
        if self.selected.len() <= limit {
            update.focus = self.focus;
            return update;
        }

        let mut candidates = self
            .selected
            .iter()
            .filter(|(id, node)| !node.pinned && Some(**id) != self.focus)
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();
        candidates.sort_by_key(|id| id.to_string());

        while self.selected.len() > limit {
            let Some(block_id) = candidates.pop() else {
                break;
            };
            if self.selected.remove(&block_id).is_some() {
                update.removed.push(block_id);
            }
        }
        update.focus = self.focus;
        update
    }

    pub fn export(&self) -> GraphExport {
        let mut nodes = self
            .selected
            .iter()
            .filter_map(|(block_id, node)| {
                self.graph
                    .describe_node(*block_id)
                    .map(|summary| GraphExportNode {
                        block_id: *block_id,
                        label: summary.label,
                        content_type: summary.content_type,
                        semantic_role: summary.semantic_role,
                        tags: summary.tags,
                        detail_level: node.detail_level,
                        pinned: node.pinned,
                        parent: summary.parent,
                        children: summary.children,
                        outgoing_edges: summary.outgoing_edges,
                        incoming_edges: summary.incoming_edges,
                    })
            })
            .collect::<Vec<_>>();
        nodes.sort_by(|left, right| left.label.cmp(&right.label));

        let selected_ids = self.selected.keys().copied().collect::<HashSet<_>>();
        let mut seen_edges = HashSet::new();
        let mut edges = Vec::new();
        for block_id in &selected_ids {
            for edge in self
                .graph
                .neighbors(*block_id, GraphNeighborMode::Neighborhood)
            {
                if !selected_ids.contains(&edge.to) {
                    continue;
                }
                let key = (
                    edge.from,
                    edge.to,
                    edge.relation.clone(),
                    edge.direction.clone(),
                );
                if seen_edges.insert(key.clone()) {
                    edges.push(GraphExportEdge {
                        source: edge.from,
                        target: edge.to,
                        relation: edge.relation,
                        direction: edge.direction,
                    });
                }
            }
        }
        edges.sort_by(|left, right| {
            left.relation
                .cmp(&right.relation)
                .then(left.source.to_string().cmp(&right.source.to_string()))
                .then(left.target.to_string().cmp(&right.target.to_string()))
        });

        GraphExport {
            summary: self.summary(),
            nodes,
            edges,
        }
    }

    pub fn why_selected(
        &self,
        selector: &str,
    ) -> Result<GraphSelectionExplanation, GraphStoreError> {
        let block_id = self
            .graph
            .resolve_selector(selector)
            .ok_or_else(|| GraphStoreError::GraphNotFound(selector.to_string()))?;
        let node = self.graph.describe_node(block_id);
        let Some(selected) = self.selected.get(&block_id) else {
            return Ok(GraphSelectionExplanation {
                block_id,
                selected: false,
                focus: self.focus == Some(block_id),
                pinned: false,
                detail_level: None,
                origin: None,
                explanation: "Node is not currently selected in the session.".to_string(),
                node,
                anchor: None,
            });
        };
        let anchor = selected
            .origin
            .as_ref()
            .and_then(|origin| origin.anchor)
            .and_then(|id| self.graph.describe_node(id));
        let explanation = match selected.origin.as_ref().map(|origin| origin.kind) {
            Some(GraphSelectionOriginKind::Overview) => {
                "Node was selected as part of the overview scaffold.".to_string()
            }
            Some(GraphSelectionOriginKind::Manual) => {
                "Node was selected directly by the agent.".to_string()
            }
            Some(GraphSelectionOriginKind::Children) => {
                "Node was selected while expanding child relationships.".to_string()
            }
            Some(GraphSelectionOriginKind::Parents) => {
                "Node was selected while traversing toward parent relationships.".to_string()
            }
            Some(GraphSelectionOriginKind::Outgoing) => {
                "Node was selected while following outgoing semantic edges.".to_string()
            }
            Some(GraphSelectionOriginKind::Incoming) => {
                "Node was selected while following incoming semantic edges.".to_string()
            }
            None => "Node is selected in the session.".to_string(),
        };
        Ok(GraphSelectionExplanation {
            block_id,
            selected: true,
            focus: self.focus == Some(block_id),
            pinned: selected.pinned,
            detail_level: Some(selected.detail_level),
            origin: selected.origin.clone(),
            explanation,
            node,
            anchor,
        })
    }

    pub fn diff(&self, other: &Self) -> GraphSessionDiff {
        let before = self.selected.keys().copied().collect::<HashSet<_>>();
        let after = other.selected.keys().copied().collect::<HashSet<_>>();
        let mut added = after
            .difference(&before)
            .copied()
            .filter_map(|id| other.graph.describe_node(id))
            .collect::<Vec<_>>();
        let mut removed = before
            .difference(&after)
            .copied()
            .filter_map(|id| self.graph.describe_node(id))
            .collect::<Vec<_>>();
        added.sort_by(|left, right| left.label.cmp(&right.label));
        removed.sort_by(|left, right| left.label.cmp(&right.label));
        GraphSessionDiff {
            added,
            removed,
            focus_before: self.focus,
            focus_after: other.focus,
            changed_focus: self.focus != other.focus,
        }
    }

    fn select_id(
        &mut self,
        block_id: BlockId,
        detail_level: GraphDetailLevel,
        origin: Option<GraphSelectionOrigin>,
        update: &mut GraphSessionUpdate,
    ) {
        match self.selected.get_mut(&block_id) {
            Some(node) => {
                if node.detail_level < detail_level {
                    node.detail_level = detail_level;
                    update.changed.push(block_id);
                }
                if node.origin.is_none() {
                    node.origin = origin;
                }
            }
            None => {
                self.selected.insert(
                    block_id,
                    GraphSessionNode {
                        detail_level,
                        pinned: false,
                        origin,
                    },
                );
                update.added.push(block_id);
            }
        }
    }
}

fn origin_for(mode: GraphNeighborMode, relation: &str, anchor: BlockId) -> GraphSelectionOrigin {
    GraphSelectionOrigin {
        kind: match mode {
            GraphNeighborMode::Children => GraphSelectionOriginKind::Children,
            GraphNeighborMode::Parents => GraphSelectionOriginKind::Parents,
            GraphNeighborMode::Outgoing => GraphSelectionOriginKind::Outgoing,
            GraphNeighborMode::Incoming => GraphSelectionOriginKind::Incoming,
            GraphNeighborMode::Neighborhood => GraphSelectionOriginKind::Outgoing,
        },
        relation: Some(relation.to_string()),
        anchor: Some(anchor),
    }
}

#[allow(dead_code)]
fn _edge_to_export(edge: GraphEdgeSummary) -> GraphExportEdge {
    GraphExportEdge {
        source: edge.source,
        target: edge.target,
        relation: edge.relation,
        direction: edge.direction,
    }
}
