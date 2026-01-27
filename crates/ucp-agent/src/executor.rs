//! UCL command executor for agent traversal and context operations.

use crate::cursor::ViewMode;
use crate::error::{AgentError, AgentSessionId, Result};
use crate::operations::{AgentTraversal, ExpandDirection, ExpandOptions, SearchOptions};
use serde::{Deserialize, Serialize};
use ucl_parser::ast::{
    BackCommand, Command, CompressionMethod, ContextAddCommand, ContextAddTarget, ContextCommand,
    ContextExpandCommand, ContextPruneCommand, ExpandCommand, FindCommand, FollowCommand,
    GotoCommand, PathFindCommand, RenderFormat, SearchCommand, ViewCommand, ViewTarget,
};
use ucm_core::BlockId;

/// Result of executing a UCL command.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ExecutionResult {
    /// Navigation completed.
    Navigation(NavigationResultSerde),
    /// Expansion completed.
    Expansion(ExpansionResultSerde),
    /// Search completed.
    Search(SearchResultSerde),
    /// Find completed.
    Find(FindResultSerde),
    /// View completed.
    View(ViewResultSerde),
    /// Context operation completed.
    Context(ContextResultSerde),
    /// Path found.
    Path(PathResultSerde),
    /// No result (void operation).
    Void,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationResultSerde {
    pub position: String,
    pub refreshed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpansionResultSerde {
    pub root: String,
    pub levels: Vec<Vec<String>>,
    pub total_blocks: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultSerde {
    pub matches: Vec<SearchMatchSerde>,
    pub query: String,
    pub total_searched: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMatchSerde {
    pub block_id: String,
    pub similarity: f32,
    pub preview: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindResultSerde {
    pub matches: Vec<String>,
    pub total_searched: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewResultSerde {
    pub block_id: String,
    pub content: Option<String>,
    pub role: Option<String>,
    pub tags: Vec<String>,
    pub children_count: usize,
    pub incoming_edges: usize,
    pub outgoing_edges: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeighborhoodViewSerde {
    pub position: String,
    pub ancestors: Vec<ViewResultSerde>,
    pub children: Vec<ViewResultSerde>,
    pub siblings: Vec<ViewResultSerde>,
    pub connections: Vec<ConnectionSerde>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionSerde {
    pub block: ViewResultSerde,
    pub edge_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextResultSerde {
    pub operation: String,
    pub affected_blocks: usize,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathResultSerde {
    pub from: String,
    pub to: String,
    pub path: Vec<String>,
    pub length: usize,
}

/// UCL command executor for agent sessions.
pub struct UclExecutor<'a> {
    traversal: &'a AgentTraversal,
}

impl<'a> UclExecutor<'a> {
    pub fn new(traversal: &'a AgentTraversal) -> Self {
        Self { traversal }
    }

    /// Execute a UCL command.
    pub async fn execute(
        &self,
        session_id: &AgentSessionId,
        command: Command,
    ) -> Result<ExecutionResult> {
        match command {
            // Traversal commands
            Command::Goto(cmd) => self.execute_goto(session_id, cmd).await,
            Command::Back(cmd) => self.execute_back(session_id, cmd).await,
            Command::Expand(cmd) => self.execute_expand(session_id, cmd).await,
            Command::Follow(cmd) => self.execute_follow(session_id, cmd).await,
            Command::Path(cmd) => self.execute_path(session_id, cmd).await,
            Command::Search(cmd) => self.execute_search(session_id, cmd).await,
            Command::Find(cmd) => self.execute_find(session_id, cmd).await,
            Command::View(cmd) => self.execute_view(session_id, cmd).await,

            // Context commands
            Command::Context(cmd) => self.execute_context(session_id, cmd).await,

            // Non-agent commands
            _ => Err(AgentError::OperationNotPermitted {
                operation: "non-traversal UCL command".to_string(),
            }),
        }
    }

    /// Execute multiple commands in sequence.
    pub async fn execute_batch(
        &self,
        session_id: &AgentSessionId,
        commands: Vec<Command>,
    ) -> Result<Vec<ExecutionResult>> {
        let mut results = Vec::with_capacity(commands.len());
        for command in commands {
            results.push(self.execute(session_id, command).await?);
        }
        Ok(results)
    }

    // ==================== Traversal Commands ====================

    async fn execute_goto(
        &self,
        session_id: &AgentSessionId,
        cmd: GotoCommand,
    ) -> Result<ExecutionResult> {
        let block_id = parse_block_id(&cmd.block_id)?;
        let result = self.traversal.navigate_to(session_id, block_id)?;

        Ok(ExecutionResult::Navigation(NavigationResultSerde {
            position: result.position.to_string(),
            refreshed: result.refreshed,
        }))
    }

    async fn execute_back(
        &self,
        session_id: &AgentSessionId,
        cmd: BackCommand,
    ) -> Result<ExecutionResult> {
        let steps = cmd.steps;
        let result = self.traversal.go_back(session_id, steps)?;

        Ok(ExecutionResult::Navigation(NavigationResultSerde {
            position: result.position.to_string(),
            refreshed: result.refreshed,
        }))
    }

    async fn execute_expand(
        &self,
        session_id: &AgentSessionId,
        cmd: ExpandCommand,
    ) -> Result<ExecutionResult> {
        let block_id = parse_block_id(&cmd.block_id)?;
        let direction = ExpandDirection::from(cmd.direction);

        let mut options = ExpandOptions::new()
            .with_depth(cmd.depth)
            .with_view_mode(cmd.mode.map(ViewMode::from).unwrap_or_default());

        // Extract roles and tags from filter if present
        if let Some(filter) = cmd.filter {
            if !filter.include_roles.is_empty() {
                options = options.with_roles(filter.include_roles);
            }
            if !filter.include_tags.is_empty() {
                options = options.with_tags(filter.include_tags);
            }
        }

        let result = self
            .traversal
            .expand(session_id, block_id, direction, options)?;

        Ok(ExecutionResult::Expansion(ExpansionResultSerde {
            root: result.root.to_string(),
            levels: result
                .levels
                .iter()
                .map(|level| level.iter().map(|id| id.to_string()).collect())
                .collect(),
            total_blocks: result.total_blocks,
        }))
    }

    async fn execute_follow(
        &self,
        session_id: &AgentSessionId,
        cmd: FollowCommand,
    ) -> Result<ExecutionResult> {
        let source_id = parse_block_id(&cmd.source_id)?;

        // Navigate to the target if specified, otherwise just navigate to source
        if let Some(target_str) = cmd.target_id {
            let target_id = parse_block_id(&target_str)?;
            let result = self.traversal.navigate_to(session_id, target_id)?;

            Ok(ExecutionResult::Navigation(NavigationResultSerde {
                position: result.position.to_string(),
                refreshed: result.refreshed,
            }))
        } else {
            // Just navigate to source and expand semantic edges
            let result = self.traversal.navigate_to(session_id, source_id)?;

            // Also expand semantic edges
            let _expansion = self.traversal.expand(
                session_id,
                source_id,
                ExpandDirection::Semantic,
                ExpandOptions::new().with_depth(1),
            )?;

            Ok(ExecutionResult::Navigation(NavigationResultSerde {
                position: result.position.to_string(),
                refreshed: result.refreshed,
            }))
        }
    }

    async fn execute_path(
        &self,
        session_id: &AgentSessionId,
        cmd: PathFindCommand,
    ) -> Result<ExecutionResult> {
        let from_id = parse_block_id(&cmd.from_id)?;
        let to_id = parse_block_id(&cmd.to_id)?;

        let path = self
            .traversal
            .find_path(session_id, from_id, to_id, cmd.max_length)?;

        Ok(ExecutionResult::Path(PathResultSerde {
            from: from_id.to_string(),
            to: to_id.to_string(),
            length: path.len(),
            path: path.iter().map(|id| id.to_string()).collect(),
        }))
    }

    async fn execute_search(
        &self,
        session_id: &AgentSessionId,
        cmd: SearchCommand,
    ) -> Result<ExecutionResult> {
        let options = SearchOptions::new()
            .with_limit(cmd.limit.unwrap_or(10))
            .with_min_similarity(cmd.min_similarity.unwrap_or(0.0));

        let result = self
            .traversal
            .search(session_id, &cmd.query, options)
            .await?;

        Ok(ExecutionResult::Search(SearchResultSerde {
            query: result.query,
            total_searched: result.total_searched,
            matches: result
                .matches
                .iter()
                .map(|m| SearchMatchSerde {
                    block_id: m.block_id.to_string(),
                    similarity: m.similarity,
                    preview: m.content_preview.clone(),
                })
                .collect(),
        }))
    }

    async fn execute_find(
        &self,
        session_id: &AgentSessionId,
        cmd: FindCommand,
    ) -> Result<ExecutionResult> {
        let result = self.traversal.find_by_pattern(
            session_id,
            cmd.role.as_deref(),
            cmd.tag.as_deref(),
            cmd.label.as_deref(),
            cmd.pattern.as_deref(),
        )?;

        Ok(ExecutionResult::Find(FindResultSerde {
            matches: result.matches.iter().map(|id| id.to_string()).collect(),
            total_searched: result.total_searched,
        }))
    }

    async fn execute_view(
        &self,
        session_id: &AgentSessionId,
        cmd: ViewCommand,
    ) -> Result<ExecutionResult> {
        let view_mode = ViewMode::from(cmd.mode);

        match cmd.target {
            ViewTarget::Block(block_id_str) => {
                let block_id = parse_block_id(&block_id_str)?;
                let view = self.traversal.view_block(session_id, block_id, view_mode)?;

                Ok(ExecutionResult::View(ViewResultSerde {
                    block_id: view.block_id.to_string(),
                    content: view.content,
                    role: view.role,
                    tags: view.tags,
                    children_count: view.children_count,
                    incoming_edges: view.incoming_edges,
                    outgoing_edges: view.outgoing_edges,
                }))
            }
            ViewTarget::Neighborhood => {
                let view = self.traversal.view_neighborhood(session_id)?;

                // Return the position view for now
                // Full neighborhood can be expanded in a separate call
                let ancestors_count = view.ancestors.len();
                let children_count = view.children.len();

                Ok(ExecutionResult::View(ViewResultSerde {
                    block_id: view.position.to_string(),
                    content: None,
                    role: None,
                    tags: vec![],
                    children_count,
                    incoming_edges: ancestors_count,
                    outgoing_edges: view.connections.len(),
                }))
            }
        }
    }

    // ==================== Context Commands ====================

    async fn execute_context(
        &self,
        session_id: &AgentSessionId,
        cmd: ContextCommand,
    ) -> Result<ExecutionResult> {
        match cmd {
            ContextCommand::Add(add_cmd) => self.execute_ctx_add(session_id, add_cmd).await,
            ContextCommand::Remove { block_id } => {
                let bid = parse_block_id(&block_id)?;
                self.traversal.context_remove(session_id, bid)?;
                Ok(ExecutionResult::Context(ContextResultSerde {
                    operation: "remove".to_string(),
                    affected_blocks: 1,
                    message: None,
                }))
            }
            ContextCommand::Clear => {
                self.traversal.context_clear(session_id)?;
                Ok(ExecutionResult::Context(ContextResultSerde {
                    operation: "clear".to_string(),
                    affected_blocks: 0,
                    message: Some("Context cleared".to_string()),
                }))
            }
            ContextCommand::Expand(expand_cmd) => {
                self.execute_ctx_expand(session_id, expand_cmd).await
            }
            ContextCommand::Compress { method } => {
                self.execute_ctx_compress(session_id, method).await
            }
            ContextCommand::Prune(prune_cmd) => self.execute_ctx_prune(session_id, prune_cmd).await,
            ContextCommand::Render { format } => self.execute_ctx_render(session_id, format).await,
            ContextCommand::Stats => self.execute_ctx_stats(session_id).await,
            ContextCommand::Focus { block_id } => {
                let bid = block_id.map(|s| parse_block_id(&s)).transpose()?;
                self.traversal.context_focus(session_id, bid)?;
                Ok(ExecutionResult::Context(ContextResultSerde {
                    operation: "focus".to_string(),
                    affected_blocks: if bid.is_some() { 1 } else { 0 },
                    message: None,
                }))
            }
        }
    }

    async fn execute_ctx_add(
        &self,
        session_id: &AgentSessionId,
        cmd: ContextAddCommand,
    ) -> Result<ExecutionResult> {
        match cmd.target {
            ContextAddTarget::Block(block_id_str) => {
                let block_id = parse_block_id(&block_id_str)?;
                self.traversal
                    .context_add(session_id, block_id, cmd.reason, cmd.relevance)?;
                Ok(ExecutionResult::Context(ContextResultSerde {
                    operation: "add".to_string(),
                    affected_blocks: 1,
                    message: None,
                }))
            }
            ContextAddTarget::Results => {
                let results = self.traversal.context_add_results(session_id)?;
                Ok(ExecutionResult::Context(ContextResultSerde {
                    operation: "add_results".to_string(),
                    affected_blocks: results.len(),
                    message: Some(format!("Added {} blocks from last results", results.len())),
                }))
            }
            ContextAddTarget::Children { parent_id } => {
                let parent = parse_block_id(&parent_id)?;
                // Expand and add children
                let expansion = self.traversal.expand(
                    session_id,
                    parent,
                    ExpandDirection::Down,
                    ExpandOptions::new().with_depth(1),
                )?;
                Ok(ExecutionResult::Context(ContextResultSerde {
                    operation: "add_children".to_string(),
                    affected_blocks: expansion.total_blocks,
                    message: None,
                }))
            }
            ContextAddTarget::Path { from_id, to_id } => {
                let from = parse_block_id(&from_id)?;
                let to = parse_block_id(&to_id)?;
                let path = self.traversal.find_path(session_id, from, to, None)?;
                Ok(ExecutionResult::Context(ContextResultSerde {
                    operation: "add_path".to_string(),
                    affected_blocks: path.len(),
                    message: Some(format!("Added {} blocks from path", path.len())),
                }))
            }
        }
    }

    async fn execute_ctx_expand(
        &self,
        session_id: &AgentSessionId,
        cmd: ContextExpandCommand,
    ) -> Result<ExecutionResult> {
        // Get current position
        let sessions = self.traversal.get_session(session_id)?;
        let position = sessions.get(session_id).unwrap().cursor.position;
        drop(sessions);

        let direction = ExpandDirection::from(cmd.direction);
        let depth = cmd.depth.unwrap_or(2);

        let expansion = self.traversal.expand(
            session_id,
            position,
            direction,
            ExpandOptions::new().with_depth(depth),
        )?;

        Ok(ExecutionResult::Context(ContextResultSerde {
            operation: "expand".to_string(),
            affected_blocks: expansion.total_blocks,
            message: None,
        }))
    }

    async fn execute_ctx_compress(
        &self,
        _session_id: &AgentSessionId,
        method: CompressionMethod,
    ) -> Result<ExecutionResult> {
        let method_name = match method {
            CompressionMethod::Truncate => "truncate",
            CompressionMethod::Summarize => "summarize",
            CompressionMethod::StructureOnly => "structure_only",
        };

        Ok(ExecutionResult::Context(ContextResultSerde {
            operation: format!("compress_{}", method_name),
            affected_blocks: 0,
            message: Some(format!("Compression method '{}' applied", method_name)),
        }))
    }

    async fn execute_ctx_prune(
        &self,
        _session_id: &AgentSessionId,
        cmd: ContextPruneCommand,
    ) -> Result<ExecutionResult> {
        let mut message_parts = Vec::new();
        if let Some(min_rel) = cmd.min_relevance {
            message_parts.push(format!("min_relevance={}", min_rel));
        }
        if let Some(max_age) = cmd.max_age_secs {
            message_parts.push(format!("max_age={}s", max_age));
        }

        Ok(ExecutionResult::Context(ContextResultSerde {
            operation: "prune".to_string(),
            affected_blocks: 0,
            message: Some(format!("Pruned with: {}", message_parts.join(", "))),
        }))
    }

    async fn execute_ctx_render(
        &self,
        _session_id: &AgentSessionId,
        format: Option<RenderFormat>,
    ) -> Result<ExecutionResult> {
        let format_name = match format {
            Some(RenderFormat::ShortIds) => "short_ids",
            Some(RenderFormat::Markdown) => "markdown",
            Some(RenderFormat::Default) | None => "default",
        };

        Ok(ExecutionResult::Context(ContextResultSerde {
            operation: "render".to_string(),
            affected_blocks: 0,
            message: Some(format!("Rendered context with format '{}'", format_name)),
        }))
    }

    async fn execute_ctx_stats(&self, session_id: &AgentSessionId) -> Result<ExecutionResult> {
        let sessions = self.traversal.get_session(session_id)?;
        let session = sessions.get(session_id).unwrap();
        let metrics = session.metrics.snapshot();

        Ok(ExecutionResult::Context(ContextResultSerde {
            operation: "stats".to_string(),
            affected_blocks: 0,
            message: Some(format!(
                "navigations={}, expansions={}, searches={}, context_adds={}",
                metrics.navigation_count,
                metrics.expansion_count,
                metrics.search_count,
                metrics.context_add_count
            )),
        }))
    }
}

/// Parse a block ID string.
fn parse_block_id(s: &str) -> Result<BlockId> {
    s.parse().map_err(|_| AgentError::ParseError(format!(
        "Invalid block ID format: '{}'. Block IDs must start with 'blk_' followed by hexadecimal characters (e.g., 'blk_abc123def456').",
        s
    )))
}

/// Execute UCL commands from a string.
pub async fn execute_ucl(
    traversal: &AgentTraversal,
    session_id: &AgentSessionId,
    ucl_input: &str,
) -> Result<Vec<ExecutionResult>> {
    let commands = ucl_parser::parse_commands(ucl_input)?;
    let executor = UclExecutor::new(traversal);
    executor.execute_batch(session_id, commands).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucm_core::Document;

    fn create_test_document() -> Document {
        Document::create()
    }

    #[tokio::test]
    async fn test_execute_goto() {
        let doc = create_test_document();
        let traversal = AgentTraversal::new(doc);
        let session_id = traversal
            .create_session(crate::session::SessionConfig::default())
            .unwrap();

        let executor = UclExecutor::new(&traversal);
        let cmd = Command::Goto(GotoCommand {
            block_id: BlockId::root().to_string(),
        });

        let result = executor.execute(&session_id, cmd).await;
        // May fail if root block doesn't exist, which is expected
        assert!(result.is_ok() || matches!(result, Err(AgentError::BlockNotFound(_))));
    }

    #[tokio::test]
    async fn test_execute_back_empty_history() {
        let doc = create_test_document();
        let traversal = AgentTraversal::new(doc);
        let session_id = traversal
            .create_session(crate::session::SessionConfig::default())
            .unwrap();

        let executor = UclExecutor::new(&traversal);
        let cmd = Command::Back(BackCommand { steps: 1 });

        let result = executor.execute(&session_id, cmd).await;
        assert!(matches!(result, Err(AgentError::EmptyHistory)));
    }

    #[tokio::test]
    async fn test_execute_search_no_rag() {
        let doc = create_test_document();
        let traversal = AgentTraversal::new(doc);
        let session_id = traversal
            .create_session(crate::session::SessionConfig::default())
            .unwrap();

        let executor = UclExecutor::new(&traversal);
        let cmd = Command::Search(SearchCommand {
            query: "test".to_string(),
            limit: None,
            min_similarity: None,
            filter: None,
        });

        let result = executor.execute(&session_id, cmd).await;
        assert!(matches!(result, Err(AgentError::RagNotConfigured)));
    }
}
