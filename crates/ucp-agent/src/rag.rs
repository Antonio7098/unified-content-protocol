//! RAG (Retrieval-Augmented Generation) provider interface.

use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use ucm_core::BlockId;

/// Options for RAG search.
#[derive(Debug, Clone, Default)]
pub struct RagSearchOptions {
    /// Maximum results to return.
    pub limit: usize,
    /// Minimum similarity threshold (0.0 - 1.0).
    pub min_similarity: f32,
    /// Filter by block IDs (search only within these).
    pub filter_block_ids: Option<HashSet<BlockId>>,
    /// Filter by semantic roles.
    pub filter_roles: Option<HashSet<String>>,
    /// Filter by tags.
    pub filter_tags: Option<HashSet<String>>,
    /// Include content in results.
    pub include_content: bool,
}

impl RagSearchOptions {
    pub fn new() -> Self {
        Self {
            limit: 10,
            min_similarity: 0.0,
            filter_block_ids: None,
            filter_roles: None,
            filter_tags: None,
            include_content: true,
        }
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn with_min_similarity(mut self, threshold: f32) -> Self {
        self.min_similarity = threshold;
        self
    }

    pub fn with_roles(mut self, roles: impl IntoIterator<Item = String>) -> Self {
        self.filter_roles = Some(roles.into_iter().collect());
        self
    }

    pub fn with_tags(mut self, tags: impl IntoIterator<Item = String>) -> Self {
        self.filter_tags = Some(tags.into_iter().collect());
        self
    }

    pub fn with_block_ids(mut self, ids: impl IntoIterator<Item = BlockId>) -> Self {
        self.filter_block_ids = Some(ids.into_iter().collect());
        self
    }
}

/// A single match from semantic search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagMatch {
    /// Block ID of the match.
    pub block_id: BlockId,
    /// Similarity score (0.0 - 1.0).
    pub similarity: f32,
    /// Content preview (if requested).
    pub content_preview: Option<String>,
    /// Semantic role (if available).
    pub semantic_role: Option<String>,
    /// Highlight spans in content (character ranges).
    pub highlight_spans: Vec<(usize, usize)>,
}

/// Results from semantic search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagSearchResults {
    /// Matching blocks with similarity scores.
    pub matches: Vec<RagMatch>,
    /// Query that was executed.
    pub query: String,
    /// Total blocks searched.
    pub total_searched: usize,
    /// Search execution time in milliseconds.
    pub execution_time_ms: u64,
}

impl RagSearchResults {
    pub fn empty(query: String) -> Self {
        Self {
            matches: Vec::new(),
            query,
            total_searched: 0,
            execution_time_ms: 0,
        }
    }

    pub fn block_ids(&self) -> Vec<BlockId> {
        self.matches.iter().map(|m| m.block_id).collect()
    }
}

/// Capabilities of a RAG provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagCapabilities {
    /// Supports semantic search.
    pub supports_search: bool,
    /// Supports embedding generation.
    pub supports_embedding: bool,
    /// Supports filtering.
    pub supports_filtering: bool,
    /// Maximum query length.
    pub max_query_length: usize,
    /// Maximum results per query.
    pub max_results: usize,
}

impl Default for RagCapabilities {
    fn default() -> Self {
        Self {
            supports_search: true,
            supports_embedding: false,
            supports_filtering: true,
            max_query_length: 1000,
            max_results: 100,
        }
    }
}

/// Abstract interface for semantic search providers.
#[async_trait]
pub trait RagProvider: Send + Sync {
    /// Search for semantically similar content.
    async fn search(&self, query: &str, options: RagSearchOptions) -> Result<RagSearchResults>;

    /// Get embeddings for content (optional).
    async fn embed(&self, content: &str) -> Result<Vec<f32>> {
        let _ = content;
        Ok(Vec::new())
    }

    /// Provider capabilities.
    fn capabilities(&self) -> RagCapabilities;

    /// Provider name for identification.
    fn name(&self) -> &str;
}

/// No-op RAG provider for testing.
pub struct NullRagProvider;

#[async_trait]
impl RagProvider for NullRagProvider {
    async fn search(&self, query: &str, _options: RagSearchOptions) -> Result<RagSearchResults> {
        Ok(RagSearchResults::empty(query.to_string()))
    }

    fn capabilities(&self) -> RagCapabilities {
        RagCapabilities {
            supports_search: false,
            supports_embedding: false,
            supports_filtering: false,
            max_query_length: 0,
            max_results: 0,
        }
    }

    fn name(&self) -> &str {
        "null"
    }
}

/// In-memory RAG provider for testing with mock data.
pub struct MockRagProvider {
    results: Vec<RagMatch>,
}

impl MockRagProvider {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn with_results(mut self, results: Vec<RagMatch>) -> Self {
        self.results = results;
        self
    }

    pub fn add_result(&mut self, block_id: BlockId, similarity: f32, preview: Option<&str>) {
        self.results.push(RagMatch {
            block_id,
            similarity,
            content_preview: preview.map(String::from),
            semantic_role: None,
            highlight_spans: Vec::new(),
        });
    }
}

impl Default for MockRagProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RagProvider for MockRagProvider {
    async fn search(&self, query: &str, options: RagSearchOptions) -> Result<RagSearchResults> {
        let matches: Vec<_> = self
            .results
            .iter()
            .filter(|m| m.similarity >= options.min_similarity)
            .take(options.limit)
            .cloned()
            .collect();

        Ok(RagSearchResults {
            matches,
            query: query.to_string(),
            total_searched: self.results.len(),
            execution_time_ms: 1,
        })
    }

    fn capabilities(&self) -> RagCapabilities {
        RagCapabilities::default()
    }

    fn name(&self) -> &str {
        "mock"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block_id(s: &str) -> BlockId {
        s.parse().unwrap_or_else(|_| {
            // Create a deterministic ID from the input string for testing
            let mut bytes = [0u8; 12];
            let s_bytes = s.as_bytes();
            for (i, b) in s_bytes.iter().enumerate() {
                bytes[i % 12] ^= *b;
            }
            BlockId::from_bytes(bytes)
        })
    }

    #[tokio::test]
    async fn test_null_provider() {
        let provider = NullRagProvider;
        let result = provider
            .search("test query", RagSearchOptions::new())
            .await
            .unwrap();

        assert!(result.matches.is_empty());
        assert_eq!(result.query, "test query");
    }

    #[tokio::test]
    async fn test_mock_provider() {
        let mut provider = MockRagProvider::new();
        provider.add_result(block_id("blk_000000000001"), 0.9, Some("test content"));
        provider.add_result(block_id("blk_000000000002"), 0.8, None);

        let result = provider
            .search("test", RagSearchOptions::new().with_limit(5))
            .await
            .unwrap();

        assert_eq!(result.matches.len(), 2);
        assert_eq!(result.matches[0].similarity, 0.9);
    }

    #[tokio::test]
    async fn test_mock_provider_filtering() {
        let mut provider = MockRagProvider::new();
        provider.add_result(block_id("blk_000000000001"), 0.9, None);
        provider.add_result(block_id("blk_000000000002"), 0.5, None);

        let result = provider
            .search("test", RagSearchOptions::new().with_min_similarity(0.7))
            .await
            .unwrap();

        assert_eq!(result.matches.len(), 1);
        assert_eq!(result.matches[0].similarity, 0.9);
    }
}
