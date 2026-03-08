use std::collections::HashSet;

use anyhow::Result;
use ucm_core::BlockId;

use crate::{
    export_codegraph_context_with_config, render_codegraph_context_prompt, CodeGraphContextExport,
    CodeGraphContextFrontierAction, CodeGraphContextSession, CodeGraphContextSummary,
    CodeGraphContextUpdate, CodeGraphDetailLevel, CodeGraphExportConfig, CodeGraphRenderConfig,
    CodeGraphSelectionOriginKind, CodeGraphTraversalConfig,
};

use super::{
    query,
    types::{
        CodeGraphExpandMode, CodeGraphFindQuery, CodeGraphNodeSummary,
        CodeGraphRecommendedActionsResult, CodeGraphSelectionExplanation, CodeGraphSessionDiff,
    },
    CodeGraphNavigator,
};

#[derive(Debug, Clone)]
pub struct CodeGraphNavigatorSession {
    graph: CodeGraphNavigator,
    context: CodeGraphContextSession,
}

impl CodeGraphNavigatorSession {
    pub fn new(graph: CodeGraphNavigator) -> Self {
        Self {
            graph,
            context: CodeGraphContextSession::new(),
        }
    }

    pub fn context(&self) -> &CodeGraphContextSession {
        &self.context
    }

    pub fn selected_block_ids(&self) -> Vec<BlockId> {
        let mut ids = self.context.selected.keys().copied().collect::<Vec<_>>();
        ids.sort_by_key(|value| value.to_string());
        ids
    }

    pub fn summary(&self) -> CodeGraphContextSummary {
        self.context.summary(self.graph.document())
    }

    pub fn fork(&self) -> Self {
        self.clone()
    }

    pub fn seed_overview(&mut self, max_depth: Option<usize>) -> CodeGraphContextUpdate {
        self.context
            .seed_overview_with_depth(self.graph.document(), max_depth)
    }

    pub fn focus(&mut self, selector: Option<&str>) -> Result<CodeGraphContextUpdate> {
        let block_id = selector
            .map(|value| self.graph.resolve_required(value))
            .transpose()?;
        Ok(self.context.set_focus(self.graph.document(), block_id))
    }

    pub fn select(
        &mut self,
        selector: &str,
        detail_level: CodeGraphDetailLevel,
    ) -> Result<CodeGraphContextUpdate> {
        let block_id = self.graph.resolve_required(selector)?;
        Ok(self
            .context
            .select_block(self.graph.document(), block_id, detail_level))
    }

    pub fn expand(
        &mut self,
        selector: &str,
        mode: CodeGraphExpandMode,
        traversal: &CodeGraphTraversalConfig,
    ) -> Result<CodeGraphContextUpdate> {
        let block_id = self.graph.resolve_required(selector)?;
        Ok(match mode {
            CodeGraphExpandMode::File => {
                self.context
                    .expand_file_with_config(self.graph.document(), block_id, traversal)
            }
            CodeGraphExpandMode::Dependencies => self.context.expand_dependencies_with_config(
                self.graph.document(),
                block_id,
                traversal,
            ),
            CodeGraphExpandMode::Dependents => self.context.expand_dependents_with_config(
                self.graph.document(),
                block_id,
                traversal,
            ),
        })
    }

    pub fn hydrate_source(
        &mut self,
        selector: &str,
        padding: usize,
    ) -> Result<CodeGraphContextUpdate> {
        let block_id = self.graph.resolve_required(selector)?;
        Ok(self
            .context
            .hydrate_source(self.graph.document(), block_id, padding))
    }

    pub fn collapse(
        &mut self,
        selector: &str,
        include_descendants: bool,
    ) -> Result<CodeGraphContextUpdate> {
        let block_id = self.graph.resolve_required(selector)?;
        Ok(self
            .context
            .collapse(self.graph.document(), block_id, include_descendants))
    }

    pub fn pin(&mut self, selector: &str, pinned: bool) -> Result<CodeGraphContextUpdate> {
        let block_id = self.graph.resolve_required(selector)?;
        Ok(self.context.pin(block_id, pinned))
    }

    pub fn prune(&mut self, max_selected: Option<usize>) -> CodeGraphContextUpdate {
        self.context.prune(self.graph.document(), max_selected)
    }

    pub fn export(
        &self,
        render: &CodeGraphRenderConfig,
        export: &CodeGraphExportConfig,
    ) -> CodeGraphContextExport {
        export_codegraph_context_with_config(self.graph.document(), &self.context, render, export)
    }

    pub fn render_prompt(&self, render: &CodeGraphRenderConfig) -> String {
        render_codegraph_context_prompt(self.graph.document(), &self.context, render)
    }

    pub fn find_nodes(&self, query: &CodeGraphFindQuery) -> Result<Vec<CodeGraphNodeSummary>> {
        self.graph.find_nodes(query)
    }

    pub fn why_selected(&self, selector: &str) -> Result<CodeGraphSelectionExplanation> {
        let block_id = self.graph.resolve_required(selector)?;
        let node = self.graph.describe_node(block_id);
        let Some(selected) = self.context.selected.get(&block_id) else {
            return Ok(CodeGraphSelectionExplanation {
                block_id,
                selected: false,
                focus: self.context.focus == Some(block_id),
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
            Some(CodeGraphSelectionOriginKind::Manual) => {
                "Node was selected directly by the agent.".to_string()
            }
            Some(CodeGraphSelectionOriginKind::Overview) => {
                "Node was selected as part of the overview scaffold.".to_string()
            }
            Some(CodeGraphSelectionOriginKind::FileSymbols) => {
                "Node was selected while expanding file symbols.".to_string()
            }
            Some(CodeGraphSelectionOriginKind::Dependencies) => format!(
                "Node was selected while following dependency edges{}.",
                relation_suffix(selected.origin.as_ref())
            ),
            Some(CodeGraphSelectionOriginKind::Dependents) => format!(
                "Node was selected while following dependent edges{}.",
                relation_suffix(selected.origin.as_ref())
            ),
            None => "Node is selected in the session.".to_string(),
        };

        Ok(CodeGraphSelectionExplanation {
            block_id,
            selected: true,
            focus: self.context.focus == Some(block_id),
            pinned: selected.pinned,
            detail_level: Some(selected.detail_level),
            origin: selected.origin.clone(),
            explanation,
            node,
            anchor,
        })
    }

    pub fn diff(&self, other: &Self) -> CodeGraphSessionDiff {
        let before = self
            .context
            .selected
            .keys()
            .copied()
            .collect::<HashSet<_>>();
        let after = other
            .context
            .selected
            .keys()
            .copied()
            .collect::<HashSet<_>>();
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
        CodeGraphSessionDiff {
            added,
            removed,
            focus_before: self.context.focus,
            focus_after: other.context.focus,
            changed_focus: self.context.focus != other.context.focus,
        }
    }

    pub fn apply_recommended_actions(
        &mut self,
        top: usize,
        padding: usize,
        depth: Option<usize>,
        max_add: Option<usize>,
        priority_threshold: Option<u16>,
    ) -> Result<CodeGraphRecommendedActionsResult> {
        let mut export_config = CodeGraphExportConfig::default();
        export_config.max_frontier_actions = top.max(1).max(8);
        let actions = self
            .export(&CodeGraphRenderConfig::default(), &export_config)
            .frontier
            .into_iter()
            .filter(|action| action.candidate_count > 0)
            .filter(|action| {
                priority_threshold
                    .map(|threshold| action.priority >= threshold)
                    .unwrap_or(true)
            })
            .take(top.max(1))
            .collect::<Vec<_>>();
        if actions.is_empty() {
            return Err(anyhow::anyhow!(
                "No recommended actions available for the current focus"
            ));
        }

        let mut update = CodeGraphContextUpdate::default();
        let mut applied_actions = Vec::new();
        for action in actions {
            let traversal = CodeGraphTraversalConfig {
                depth: depth.unwrap_or(1),
                relation_filters: action.relation.clone().into_iter().collect(),
                max_add,
                priority_threshold,
            };
            applied_actions.push(action_summary(&action));
            merge_update(
                &mut update,
                match action.action.as_str() {
                    "hydrate_source" => {
                        self.context
                            .hydrate_source(self.graph.document(), action.block_id, padding)
                    }
                    "expand_file" => self.context.expand_file_with_config(
                        self.graph.document(),
                        action.block_id,
                        &traversal,
                    ),
                    "expand_dependencies" => self.context.expand_dependencies_with_config(
                        self.graph.document(),
                        action.block_id,
                        &traversal,
                    ),
                    "expand_dependents" => self.context.expand_dependents_with_config(
                        self.graph.document(),
                        action.block_id,
                        &traversal,
                    ),
                    "collapse" => {
                        self.context
                            .collapse(self.graph.document(), action.block_id, false)
                    }
                    _ => CodeGraphContextUpdate::default(),
                },
            );
        }

        Ok(CodeGraphRecommendedActionsResult {
            applied_actions,
            update,
        })
    }

    pub fn path_between(
        &self,
        start_selector: &str,
        end_selector: &str,
        max_hops: usize,
    ) -> Result<Option<crate::programmatic::types::CodeGraphPathResult>> {
        let start = self.graph.resolve_required(start_selector)?;
        let end = self.graph.resolve_required(end_selector)?;
        Ok(query::path_between(
            self.graph.document(),
            start,
            end,
            max_hops,
        ))
    }
}

fn merge_update(into: &mut CodeGraphContextUpdate, next: CodeGraphContextUpdate) {
    into.added.extend(next.added);
    into.removed.extend(next.removed);
    into.changed.extend(next.changed);
    into.warnings.extend(next.warnings);
    if next.focus.is_some() {
        into.focus = next.focus;
    }
}

fn action_summary(action: &CodeGraphContextFrontierAction) -> String {
    match action.relation.as_deref() {
        Some(relation) => format!("{} {} via {}", action.action, action.short_id, relation),
        None => format!("{} {}", action.action, action.short_id),
    }
}

fn relation_suffix(origin: Option<&crate::CodeGraphSelectionOrigin>) -> String {
    origin
        .and_then(|value| value.relation.as_deref())
        .map(|relation| format!(" via `{}`", relation))
        .unwrap_or_default()
}
