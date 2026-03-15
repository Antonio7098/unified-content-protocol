use std::{fs, path::Path, sync::Arc};

use anyhow::Result;
use ucm_core::{BlockId, Document};

use crate::{build_code_graph, CodeGraphBuildInput};

use super::{
    query,
    session::CodeGraphNavigatorSession,
    types::{
        CodeGraphFindQuery, CodeGraphNodeSummary, CodeGraphPathResult,
        CodeGraphSelectorResolutionExplanation,
    },
};

#[derive(Debug, Clone)]
pub struct CodeGraphNavigator {
    document: Arc<Document>,
    graph: ucp_graph::GraphNavigator,
}

impl CodeGraphNavigator {
    pub fn new(document: Document) -> Self {
        let graph = ucp_graph::GraphNavigator::from_document(document.clone());
        Self {
            document: Arc::new(document),
            graph,
        }
    }

    pub fn build(input: &CodeGraphBuildInput) -> Result<Self> {
        let result = build_code_graph(input)?;
        Ok(Self::new(result.document))
    }

    pub fn document(&self) -> &Document {
        self.document.as_ref()
    }

    pub fn graph(&self) -> &ucp_graph::GraphNavigator {
        &self.graph
    }

    pub fn session(&self) -> CodeGraphNavigatorSession {
        CodeGraphNavigatorSession::new(self.clone())
    }

    pub fn resolve_selector(&self, selector: &str) -> Option<BlockId> {
        crate::resolve_codegraph_selector(self.document(), selector)
    }

    pub fn describe_node(&self, block_id: BlockId) -> Option<CodeGraphNodeSummary> {
        query::describe_node(self.document(), block_id)
    }

    pub fn explain_selector(&self, selector: &str) -> CodeGraphSelectorResolutionExplanation {
        query::explain_selector(self.document(), selector)
    }

    pub fn find_nodes(&self, query: &CodeGraphFindQuery) -> Result<Vec<CodeGraphNodeSummary>> {
        query::find_nodes(self.document(), query)
    }

    pub fn path_between(
        &self,
        start: BlockId,
        end: BlockId,
        max_hops: usize,
    ) -> Option<CodeGraphPathResult> {
        query::path_between(self.document(), start, end, max_hops)
    }

    pub(crate) fn resolve_required(&self, selector: &str) -> Result<BlockId> {
        query::resolve_required(self.document(), selector)
    }

    pub fn load_session_json(&self, payload: &str) -> Result<CodeGraphNavigatorSession> {
        let persisted = serde_json::from_str(payload)?;
        CodeGraphNavigatorSession::from_persisted(self.clone(), persisted)
    }

    pub fn load_session(&self, path: impl AsRef<Path>) -> Result<CodeGraphNavigatorSession> {
        self.load_session_json(&fs::read_to_string(path)?)
    }
}
