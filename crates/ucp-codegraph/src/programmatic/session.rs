use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
    sync::{Arc, Mutex},
    time::Instant,
};

use anyhow::{anyhow, Result};
use sha2::{Digest, Sha256};
use ucm_core::BlockId;

use crate::{
    canonical_fingerprint, export_codegraph_context_with_config, render_codegraph_context_prompt,
    CodeGraphContextExport, CodeGraphContextFrontierAction, CodeGraphContextSession,
    CodeGraphContextSummary, CodeGraphContextUpdate, CodeGraphDetailLevel, CodeGraphExportConfig,
    CodeGraphOperationBudget, CodeGraphPersistedSession, CodeGraphRecommendation,
    CodeGraphRenderConfig, CodeGraphSelectionOriginKind, CodeGraphSessionEvent,
    CodeGraphSessionMutation, CodeGraphSessionMutationKind, CodeGraphSessionPersistenceMetadata,
    CodeGraphTraversalConfig,
};

use super::{
    query,
    types::{
        CodeGraphExpandMode, CodeGraphExportOmissionExplanation, CodeGraphFindQuery,
        CodeGraphMutationEstimate, CodeGraphNodeSummary, CodeGraphPathResult,
        CodeGraphProvenanceStep, CodeGraphPruneExplanation, CodeGraphRecommendedActionsResult,
        CodeGraphSelectionExplanation, CodeGraphSelectorResolutionExplanation,
        CodeGraphSessionDiff,
    },
    CodeGraphNavigator,
};

type SessionObserver = Arc<dyn Fn(&CodeGraphSessionEvent) + Send + Sync>;

#[derive(Clone, Default)]
struct ObserverRegistry {
    handlers: Arc<Mutex<Vec<SessionObserver>>>,
}

impl ObserverRegistry {
    fn subscribe<F>(&self, observer: F)
    where
        F: Fn(&CodeGraphSessionEvent) + Send + Sync + 'static,
    {
        if let Ok(mut handlers) = self.handlers.lock() {
            handlers.push(Arc::new(observer));
        }
    }

    fn emit(&self, event: &CodeGraphSessionEvent) {
        if let Ok(handlers) = self.handlers.lock() {
            for handler in handlers.iter() {
                handler(event);
            }
        }
    }

    fn count(&self) -> usize {
        self.handlers.lock().map(|value| value.len()).unwrap_or(0)
    }
}

impl std::fmt::Debug for ObserverRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObserverRegistry")
            .field("observer_count", &self.count())
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct CodeGraphNavigatorSession {
    graph: CodeGraphNavigator,
    context: CodeGraphContextSession,
    session_id: String,
    parent_session_id: Option<String>,
    mutation_log: Vec<CodeGraphSessionMutation>,
    event_log: Vec<CodeGraphSessionEvent>,
    prune_notes: HashMap<BlockId, String>,
    next_sequence: usize,
    observers: ObserverRegistry,
}

impl CodeGraphNavigatorSession {
    pub fn new(graph: CodeGraphNavigator) -> Self {
        Self {
            graph,
            context: CodeGraphContextSession::new(),
            session_id: new_session_id("root", 0),
            parent_session_id: None,
            mutation_log: Vec::new(),
            event_log: Vec::new(),
            prune_notes: HashMap::new(),
            next_sequence: 1,
            observers: ObserverRegistry::default(),
        }
    }

    pub(crate) fn from_persisted(
        graph: CodeGraphNavigator,
        persisted: CodeGraphPersistedSession,
    ) -> Result<Self> {
        let expected = canonical_fingerprint(graph.document())?;
        if persisted.metadata.graph_snapshot_hash != expected {
            return Err(anyhow!(
                "Persisted session targets graph snapshot {} but current graph snapshot is {}",
                persisted.metadata.graph_snapshot_hash,
                expected
            ));
        }

        let mut session = Self {
            graph,
            context: persisted.context,
            session_id: persisted.metadata.session_id.clone(),
            parent_session_id: persisted.metadata.parent_session_id.clone(),
            mutation_log: persisted.mutation_log,
            event_log: persisted.event_log,
            prune_notes: HashMap::new(),
            next_sequence: persisted.metadata.mutation_count.saturating_add(1),
            observers: ObserverRegistry::default(),
        };
        let loaded = CodeGraphSessionEvent::SessionLoaded {
            metadata: persisted.metadata,
        };
        session.event_log.push(loaded.clone());
        session.observers.emit(&loaded);
        Ok(session)
    }

    pub fn context(&self) -> &CodeGraphContextSession {
        &self.context
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn parent_session_id(&self) -> Option<&str> {
        self.parent_session_id.as_deref()
    }

    pub fn mutation_log(&self) -> &[CodeGraphSessionMutation] {
        &self.mutation_log
    }

    pub fn event_log(&self) -> &[CodeGraphSessionEvent] {
        &self.event_log
    }

    pub fn subscribe<F>(&mut self, observer: F)
    where
        F: Fn(&CodeGraphSessionEvent) + Send + Sync + 'static,
    {
        self.observers.subscribe(observer);
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
        let mut branch = self.clone();
        branch.parent_session_id = Some(self.session_id.clone());
        branch.session_id = new_session_id(&self.session_id, self.next_sequence);
        branch
    }

    pub fn seed_overview(&mut self, max_depth: Option<usize>) -> CodeGraphContextUpdate {
        let focus_before = self.context.focus;
        let started = Instant::now();
        let mut update = self
            .context
            .seed_overview_with_depth(self.graph.document(), max_depth);
        let resolved = update.added.clone();
        self.record_mutation(
            &mut update,
            CodeGraphSessionMutationKind::SeedOverview,
            "seed_overview",
            None,
            None,
            resolved,
            None,
            None,
            focus_before,
            started,
            Some(match max_depth {
                Some(depth) => format!("Seeded structural overview up to depth {}", depth),
                None => "Seeded full structural overview.".to_string(),
            }),
        );
        self.note_prune_effects("seed_overview", &update);
        update
    }

    pub fn focus(&mut self, selector: Option<&str>) -> Result<CodeGraphContextUpdate> {
        let focus_before = self.context.focus;
        let started = Instant::now();
        let block_id = selector
            .map(|value| self.graph.resolve_required(value))
            .transpose()?;
        let mut update = self.context.set_focus(self.graph.document(), block_id);
        self.record_mutation(
            &mut update,
            CodeGraphSessionMutationKind::Focus,
            "focus",
            selector.map(str::to_string),
            block_id,
            block_id.into_iter().collect(),
            None,
            None,
            focus_before,
            started,
            Some(match block_id {
                Some(id) => format!("Focused session on {}", id),
                None => "Cleared session focus.".to_string(),
            }),
        );
        self.note_prune_effects("focus", &update);
        Ok(update)
    }

    pub fn select(
        &mut self,
        selector: &str,
        detail_level: CodeGraphDetailLevel,
    ) -> Result<CodeGraphContextUpdate> {
        let focus_before = self.context.focus;
        let started = Instant::now();
        let block_id = self.graph.resolve_required(selector)?;
        let mut update = self
            .context
            .select_block(self.graph.document(), block_id, detail_level);
        self.record_mutation(
            &mut update,
            CodeGraphSessionMutationKind::Select,
            "select",
            Some(selector.to_string()),
            Some(block_id),
            vec![block_id],
            None,
            None,
            focus_before,
            started,
            Some(format!(
                "Selected {} at {:?} detail.",
                block_id, detail_level
            )),
        );
        self.note_prune_effects("select", &update);
        Ok(update)
    }

    pub fn expand(
        &mut self,
        selector: &str,
        mode: CodeGraphExpandMode,
        traversal: &CodeGraphTraversalConfig,
    ) -> Result<CodeGraphContextUpdate> {
        let focus_before = self.context.focus;
        let started = Instant::now();
        let block_id = self.graph.resolve_required(selector)?;
        let mut update = match mode {
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
        };
        self.record_mutation(
            &mut update,
            match mode {
                CodeGraphExpandMode::File => CodeGraphSessionMutationKind::ExpandFile,
                CodeGraphExpandMode::Dependencies => {
                    CodeGraphSessionMutationKind::ExpandDependencies
                }
                CodeGraphExpandMode::Dependents => CodeGraphSessionMutationKind::ExpandDependents,
            },
            match mode {
                CodeGraphExpandMode::File => "expand_file",
                CodeGraphExpandMode::Dependencies => "expand_dependencies",
                CodeGraphExpandMode::Dependents => "expand_dependents",
            },
            Some(selector.to_string()),
            Some(block_id),
            vec![block_id],
            Some(traversal.clone()),
            traversal.budget.clone(),
            focus_before,
            started,
            Some(format!("Expanded {} via {:?} traversal.", block_id, mode)),
        );
        self.note_prune_effects("expand", &update);
        Ok(update)
    }

    pub fn hydrate_source(
        &mut self,
        selector: &str,
        padding: usize,
    ) -> Result<CodeGraphContextUpdate> {
        self.hydrate_source_with_budget(selector, padding, None)
    }

    pub fn hydrate_source_with_budget(
        &mut self,
        selector: &str,
        padding: usize,
        budget: Option<CodeGraphOperationBudget>,
    ) -> Result<CodeGraphContextUpdate> {
        let focus_before = self.context.focus;
        let started = Instant::now();
        let block_id = self.graph.resolve_required(selector)?;
        let mut update = self.context.hydrate_source_with_budget(
            self.graph.document(),
            block_id,
            padding,
            budget.as_ref(),
        );
        self.record_mutation(
            &mut update,
            CodeGraphSessionMutationKind::Hydrate,
            "hydrate",
            Some(selector.to_string()),
            Some(block_id),
            vec![block_id],
            None,
            budget,
            focus_before,
            started,
            Some(format!(
                "Hydrated source for {} with padding {}.",
                block_id, padding
            )),
        );
        self.note_prune_effects("hydrate", &update);
        Ok(update)
    }

    pub fn collapse(
        &mut self,
        selector: &str,
        include_descendants: bool,
    ) -> Result<CodeGraphContextUpdate> {
        let focus_before = self.context.focus;
        let started = Instant::now();
        let block_id = self.graph.resolve_required(selector)?;
        let mut update =
            self.context
                .collapse(self.graph.document(), block_id, include_descendants);
        self.record_mutation(
            &mut update,
            CodeGraphSessionMutationKind::Collapse,
            "collapse",
            Some(selector.to_string()),
            Some(block_id),
            vec![block_id],
            None,
            None,
            focus_before,
            started,
            Some(format!(
                "Collapsed {} (include_descendants={}).",
                block_id, include_descendants
            )),
        );
        Ok(update)
    }

    pub fn pin(&mut self, selector: &str, pinned: bool) -> Result<CodeGraphContextUpdate> {
        let focus_before = self.context.focus;
        let started = Instant::now();
        let block_id = self.graph.resolve_required(selector)?;
        let mut update = self.context.pin(block_id, pinned);
        self.record_mutation(
            &mut update,
            if pinned {
                CodeGraphSessionMutationKind::Pin
            } else {
                CodeGraphSessionMutationKind::Unpin
            },
            if pinned { "pin" } else { "unpin" },
            Some(selector.to_string()),
            Some(block_id),
            vec![block_id],
            None,
            None,
            focus_before,
            started,
            Some(format!(
                "{} {} in the working set.",
                if pinned { "Pinned" } else { "Unpinned" },
                block_id
            )),
        );
        Ok(update)
    }

    pub fn prune(&mut self, max_selected: Option<usize>) -> CodeGraphContextUpdate {
        let focus_before = self.context.focus;
        let started = Instant::now();
        let mut update = self.context.prune(self.graph.document(), max_selected);
        self.record_mutation(
            &mut update,
            CodeGraphSessionMutationKind::Prune,
            "prune",
            None,
            None,
            Vec::new(),
            None,
            None,
            focus_before,
            started,
            Some(format!(
                "Applied prune policy with max_selected={}.",
                max_selected
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| self.context.prune_policy.max_selected.to_string())
            )),
        );
        self.note_prune_effects("prune", &update);
        update
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

    pub fn explain_selector(&self, selector: &str) -> CodeGraphSelectorResolutionExplanation {
        query::explain_selector(self.graph.document(), selector)
    }

    pub fn why_selected(&self, selector: &str) -> Result<CodeGraphSelectionExplanation> {
        let block_id = self.graph.resolve_required(selector)?;
        let node = self.graph.describe_node(block_id);
        let provenance_chain = self.provenance_chain(block_id);
        let Some(selected) = self.context.selected.get(&block_id) else {
            return Ok(CodeGraphSelectionExplanation {
                selector: selector.to_string(),
                block_id,
                selected: false,
                focus: self.context.focus == Some(block_id),
                pinned: false,
                detail_level: None,
                origin: None,
                explanation: "Node is not currently selected in the session.".to_string(),
                node,
                anchor: None,
                provenance_chain,
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
            selector: selector.to_string(),
            block_id,
            selected: true,
            focus: self.context.focus == Some(block_id),
            pinned: selected.pinned,
            detail_level: Some(selected.detail_level),
            origin: selected.origin.clone(),
            explanation,
            node,
            anchor,
            provenance_chain,
        })
    }

    pub fn explain_export_omission(
        &self,
        selector: &str,
        render: &CodeGraphRenderConfig,
        export: &CodeGraphExportConfig,
    ) -> Result<CodeGraphExportOmissionExplanation> {
        let resolved = self.explain_selector(selector);
        let export_result = self.export(render, export);
        if let Some(block_id) = resolved.resolved_block_id {
            if let Some(detail) = export_result
                .omissions
                .details
                .iter()
                .find(|detail| detail.block_id == Some(block_id))
                .cloned()
            {
                return Ok(CodeGraphExportOmissionExplanation {
                    selector: selector.to_string(),
                    omitted: true,
                    block_id: Some(block_id),
                    explanation: detail.explanation.clone(),
                    detail: Some(detail),
                });
            }
            return Ok(CodeGraphExportOmissionExplanation {
                selector: selector.to_string(),
                omitted: false,
                block_id: Some(block_id),
                detail: None,
                explanation: "Node is present in the current export/render output.".to_string(),
            });
        }
        Ok(CodeGraphExportOmissionExplanation {
            selector: selector.to_string(),
            omitted: false,
            block_id: None,
            detail: None,
            explanation: resolved.explanation,
        })
    }

    pub fn why_pruned(&self, selector: &str) -> Result<CodeGraphPruneExplanation> {
        let resolution = self.explain_selector(selector);
        let block_id = resolution.resolved_block_id;
        let explanation = block_id
            .and_then(|id| self.prune_notes.get(&id).cloned())
            .unwrap_or_else(|| "No recorded prune explanation for this selector.".to_string());
        Ok(CodeGraphPruneExplanation {
            selector: selector.to_string(),
            block_id,
            pruned: block_id
                .map(|id| self.prune_notes.contains_key(&id))
                .unwrap_or(false),
            explanation,
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

    pub fn recommendations(&self, top: usize) -> Vec<CodeGraphRecommendation> {
        self.export(
            &CodeGraphRenderConfig::default(),
            &CodeGraphExportConfig::default(),
        )
        .heuristics
        .recommendations
        .into_iter()
        .filter(|item| item.candidate_count > 0)
        .take(top.max(1))
        .collect()
    }

    pub fn estimate_expand(
        &self,
        selector: &str,
        mode: CodeGraphExpandMode,
        traversal: &CodeGraphTraversalConfig,
    ) -> Result<CodeGraphMutationEstimate> {
        let block_id = self.graph.resolve_required(selector)?;
        let mut branch = self.fork();
        let before_selected = branch.selected_block_ids().len() as isize;
        let update = branch.expand(selector, mode, traversal)?;
        let after_export = branch.export(
            &CodeGraphRenderConfig::default(),
            &CodeGraphExportConfig::compact(),
        );
        Ok(CodeGraphMutationEstimate {
            operation: format!("{:?}", mode).to_lowercase(),
            selector: Some(selector.to_string()),
            target_block_id: Some(block_id),
            resolved_block_ids: vec![block_id],
            budget: traversal.budget.clone(),
            estimated_nodes_added: update.added.len(),
            estimated_nodes_changed: update.changed.len(),
            estimated_nodes_visited: update.added.len().saturating_add(1),
            estimated_frontier_width: after_export.frontier.len(),
            estimated_rendered_bytes: after_export.rendered.len(),
            estimated_rendered_tokens: crate::approximate_prompt_tokens(&after_export.rendered),
            estimated_export_growth: branch.selected_block_ids().len() as isize - before_selected,
            explanation: format!(
                "Estimated {:?} expansion for {} by simulating the mutation on a forked session.",
                mode, selector
            ),
        })
    }

    pub fn estimate_hydrate(
        &self,
        selector: &str,
        padding: usize,
        budget: Option<CodeGraphOperationBudget>,
    ) -> Result<CodeGraphMutationEstimate> {
        let block_id = self.graph.resolve_required(selector)?;
        let mut branch = self.fork();
        let before_selected = branch.selected_block_ids().len() as isize;
        let update = branch.hydrate_source_with_budget(selector, padding, budget.clone())?;
        let after_export = branch.export(
            &CodeGraphRenderConfig::default(),
            &CodeGraphExportConfig::compact(),
        );
        Ok(CodeGraphMutationEstimate {
            operation: "hydrate".to_string(),
            selector: Some(selector.to_string()),
            target_block_id: Some(block_id),
            resolved_block_ids: vec![block_id],
            budget,
            estimated_nodes_added: update.added.len(),
            estimated_nodes_changed: update.changed.len(),
            estimated_nodes_visited: 1,
            estimated_frontier_width: after_export.frontier.len(),
            estimated_rendered_bytes: after_export.rendered.len(),
            estimated_rendered_tokens: crate::approximate_prompt_tokens(&after_export.rendered),
            estimated_export_growth: branch.selected_block_ids().len() as isize - before_selected,
            explanation: format!(
                "Estimated hydration cost for {} by simulating source hydration on a forked session.",
                selector
            ),
        })
    }

    pub fn apply_recommended_actions(
        &mut self,
        top: usize,
        padding: usize,
        depth: Option<usize>,
        max_add: Option<usize>,
        priority_threshold: Option<u16>,
    ) -> Result<CodeGraphRecommendedActionsResult> {
        let focus_before = self.context.focus;
        let started = Instant::now();
        let actions = self
            .recommendations(top.max(1))
            .into_iter()
            .filter(|action| {
                priority_threshold
                    .map(|threshold| action.priority >= threshold)
                    .unwrap_or(true)
            })
            .take(top.max(1))
            .collect::<Vec<_>>();
        if actions.is_empty() {
            return Err(anyhow!(
                "No recommended actions available for the current focus"
            ));
        }

        let mut update = CodeGraphContextUpdate::default();
        let mut applied_actions = Vec::new();
        let events_before = self.event_log.len();
        for action in &actions {
            let frontier_action = frontier_from_recommendation(action);
            let traversal = CodeGraphTraversalConfig {
                depth: depth.unwrap_or(1),
                relation_filters: action.relation_set.clone(),
                max_add,
                priority_threshold,
                budget: None,
            };
            applied_actions.push(action_summary(&frontier_action));
            let target_selector = action.target_block_id.to_string();
            let next = match action.action_kind.as_str() {
                "hydrate_source" => self.hydrate_source(&target_selector, padding)?,
                "expand_file" => {
                    self.expand(&target_selector, CodeGraphExpandMode::File, &traversal)?
                }
                "expand_dependencies" => self.expand(
                    &target_selector,
                    CodeGraphExpandMode::Dependencies,
                    &traversal,
                )?,
                "expand_dependents" => self.expand(
                    &target_selector,
                    CodeGraphExpandMode::Dependents,
                    &traversal,
                )?,
                "collapse" => self.collapse(&target_selector, false)?,
                _ => CodeGraphContextUpdate::default(),
            };
            merge_update(&mut update, next);
            let event = CodeGraphSessionEvent::Recommendation {
                recommendation: Box::new(action.clone()),
            };
            self.event_log.push(event.clone());
            self.observers.emit(&event);
        }

        self.record_mutation(
            &mut update,
            CodeGraphSessionMutationKind::ApplyRecommendedActions,
            "apply_recommended_actions",
            None,
            self.context.focus,
            actions.iter().map(|item| item.target_block_id).collect(),
            None,
            None,
            focus_before,
            started,
            Some(format!("Applied {} recommended action(s).", actions.len())),
        );
        let events = self.event_log[events_before..].to_vec();
        Ok(CodeGraphRecommendedActionsResult {
            applied_actions,
            recommendations: actions,
            update,
            events,
        })
    }

    pub fn path_between(
        &self,
        start_selector: &str,
        end_selector: &str,
        max_hops: usize,
    ) -> Result<Option<CodeGraphPathResult>> {
        let start = self.graph.resolve_required(start_selector)?;
        let end = self.graph.resolve_required(end_selector)?;
        Ok(query::path_between(
            self.graph.document(),
            start,
            end,
            max_hops,
        ))
    }

    pub fn to_persisted(&self) -> Result<CodeGraphPersistedSession> {
        let graph_snapshot_hash = canonical_fingerprint(self.graph.document())?;
        let session_snapshot_hash = session_snapshot_hash(
            &self.context,
            &self.mutation_log,
            &self.session_id,
            self.parent_session_id.as_deref(),
        )?;
        Ok(CodeGraphPersistedSession {
            metadata: CodeGraphSessionPersistenceMetadata {
                schema_version: "codegraph_session.v1".to_string(),
                session_id: self.session_id.clone(),
                parent_session_id: self.parent_session_id.clone(),
                graph_snapshot_hash,
                session_snapshot_hash,
                mutation_count: self.mutation_log.len(),
            },
            context: self.context.clone(),
            mutation_log: self.mutation_log.clone(),
            event_log: self.event_log.clone(),
        })
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.to_persisted()?).map_err(Into::into)
    }

    pub fn save(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let persisted = self.to_persisted()?;
        fs::write(path.as_ref(), serde_json::to_string_pretty(&persisted)?)
            .map_err(anyhow::Error::from)?;
        let event = CodeGraphSessionEvent::SessionSaved {
            metadata: persisted.metadata,
        };
        self.event_log.push(event.clone());
        self.observers.emit(&event);
        Ok(())
    }

    fn provenance_chain(&self, block_id: BlockId) -> Vec<CodeGraphProvenanceStep> {
        let mut chain = Vec::new();
        let mut current = Some(block_id);
        let mut visited = HashSet::new();
        while let Some(next_id) = current {
            if !visited.insert(next_id) {
                break;
            }
            let node = self.graph.describe_node(next_id);
            let selected = self.context.selected.get(&next_id);
            let explanation = match selected.and_then(|item| item.origin.as_ref()) {
                Some(origin) => match origin.kind {
                    CodeGraphSelectionOriginKind::Manual => {
                        "Selected directly by the agent.".to_string()
                    }
                    CodeGraphSelectionOriginKind::Overview => {
                        "Included by the session overview scaffold.".to_string()
                    }
                    CodeGraphSelectionOriginKind::FileSymbols => {
                        "Reached while opening file symbols.".to_string()
                    }
                    CodeGraphSelectionOriginKind::Dependencies => format!(
                        "Reached while following dependency edges{}.",
                        relation_suffix(selected.and_then(|item| item.origin.as_ref()))
                    ),
                    CodeGraphSelectionOriginKind::Dependents => format!(
                        "Reached while following dependent edges{}.",
                        relation_suffix(selected.and_then(|item| item.origin.as_ref()))
                    ),
                },
                None => "Selected without a recorded origin.".to_string(),
            };
            chain.push(CodeGraphProvenanceStep {
                block_id: next_id,
                node,
                origin: selected.and_then(|item| item.origin.clone()),
                explanation,
            });
            current = selected
                .and_then(|item| item.origin.as_ref())
                .and_then(|origin| origin.anchor);
        }
        chain
    }

    fn note_prune_effects(&mut self, operation: &str, update: &CodeGraphContextUpdate) {
        for block_id in &update.removed {
            self.prune_notes.insert(
                *block_id,
                format!(
                    "Node was removed while applying prune policy after {}.",
                    operation
                ),
            );
        }
        for block_id in &update.changed {
            self.prune_notes.entry(*block_id).or_insert_with(|| {
                format!(
                    "Node detail was adjusted while applying prune policy after {}.",
                    operation
                )
            });
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn record_mutation(
        &mut self,
        update: &mut CodeGraphContextUpdate,
        kind: CodeGraphSessionMutationKind,
        operation: &str,
        selector: Option<String>,
        target_block_id: Option<BlockId>,
        resolved_block_ids: Vec<BlockId>,
        traversal: Option<CodeGraphTraversalConfig>,
        budget: Option<CodeGraphOperationBudget>,
        focus_before: Option<BlockId>,
        started: Instant,
        reason: Option<String>,
    ) {
        let telemetry_budget = budget
            .as_ref()
            .and_then(|value| value.max_emitted_telemetry_events);
        if telemetry_budget == Some(0) {
            return;
        }

        let mutation = CodeGraphSessionMutation {
            sequence: self.next_sequence,
            kind,
            operation: operation.to_string(),
            selector,
            target_block_id,
            resolved_block_ids,
            traversal,
            budget,
            nodes_added: update.added.clone(),
            nodes_removed: update.removed.clone(),
            nodes_changed: update.changed.clone(),
            focus_before,
            focus_after: update.focus,
            elapsed_ms: started.elapsed().as_millis() as u64,
            reason,
            warnings: update.warnings.clone(),
        };
        self.next_sequence += 1;
        self.mutation_log.push(mutation.clone());
        update.telemetry.push(mutation.clone());
        let event = CodeGraphSessionEvent::Mutation {
            mutation: Box::new(mutation.clone()),
        };
        self.event_log.push(event.clone());
        self.observers.emit(&event);
    }
}

fn merge_update(into: &mut CodeGraphContextUpdate, next: CodeGraphContextUpdate) {
    into.added.extend(next.added);
    into.removed.extend(next.removed);
    into.changed.extend(next.changed);
    into.warnings.extend(next.warnings);
    into.telemetry.extend(next.telemetry);
    if next.focus.is_some() {
        into.focus = next.focus;
    }
}

fn frontier_from_recommendation(
    action: &CodeGraphRecommendation,
) -> CodeGraphContextFrontierAction {
    CodeGraphContextFrontierAction {
        block_id: action.target_block_id,
        short_id: action.target_short_id.clone(),
        action: action.action_kind.clone(),
        relation: action.relation_set.first().cloned(),
        direction: None,
        candidate_count: action.candidate_count,
        priority: action.priority,
        description: action.explanation.clone(),
        explanation: Some(action.rationale.clone()),
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

fn new_session_id(seed: &str, sequence: usize) -> String {
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    hasher.update(sequence.to_string().as_bytes());
    hasher.update(chrono::Utc::now().to_rfc3339().as_bytes());
    let digest = hex::encode(hasher.finalize());
    format!("cgs_{}", &digest[..16])
}

fn session_snapshot_hash(
    context: &CodeGraphContextSession,
    mutation_log: &[CodeGraphSessionMutation],
    session_id: &str,
    parent_session_id: Option<&str>,
) -> Result<String> {
    let payload = serde_json::json!({
        "session_id": session_id,
        "parent_session_id": parent_session_id,
        "context": context,
        "mutation_log": mutation_log,
    });
    let bytes = serde_json::to_vec(&payload)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(hex::encode(hasher.finalize()))
}
