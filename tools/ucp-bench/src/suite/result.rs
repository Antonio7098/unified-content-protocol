//! Detailed test result structures with full context capture.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ucm_core::metadata::RoleCategory;

/// A detailed test result with full execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedTestResult {
    /// Test case identifier
    pub test_id: String,
    /// Category this test belongs to
    pub category_id: String,
    /// Provider used
    pub provider_id: String,
    /// Model used
    pub model_id: String,
    /// When the test was executed
    pub executed_at: chrono::DateTime<chrono::Utc>,

    // Execution metrics
    pub latency_ms: u64,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cost_usd: f64,

    // Results
    pub success: bool,
    pub parse_success: bool,
    pub execute_success: bool,
    pub semantic_score: f32,
    pub efficiency_score: f32,

    // Error information
    pub error: Option<TestError>,

    // Full context for debugging
    pub context: ExecutionContext,

    // Document snapshots (if captured)
    pub document_before: Option<DocumentSnapshot>,
    pub document_after: Option<DocumentSnapshot>,

    // Diff information
    pub diff: Option<DocumentDiff>,
}

fn role_category_to_heading_level(category: RoleCategory) -> Option<u8> {
    match category {
        RoleCategory::Heading1 => Some(1),
        RoleCategory::Heading2 => Some(2),
        RoleCategory::Heading3 => Some(3),
        RoleCategory::Heading4 => Some(4),
        RoleCategory::Heading5 => Some(5),
        RoleCategory::Heading6 => Some(6),
        _ => None,
    }
}

impl DetailedTestResult {
    pub fn new(
        test_id: String,
        category_id: String,
        provider_id: String,
        model_id: String,
    ) -> Self {
        Self {
            test_id,
            category_id,
            provider_id,
            model_id,
            executed_at: chrono::Utc::now(),
            latency_ms: 0,
            input_tokens: 0,
            output_tokens: 0,
            cost_usd: 0.0,
            success: false,
            parse_success: false,
            execute_success: false,
            semantic_score: 0.0,
            efficiency_score: 0.0,
            error: None,
            context: ExecutionContext::default(),
            document_before: None,
            document_after: None,
            diff: None,
        }
    }

    pub fn mark_success(&mut self) {
        self.success = true;
        self.parse_success = true;
        self.execute_success = true;
    }

    pub fn mark_parse_error(&mut self, message: String) {
        self.success = false;
        self.parse_success = false;
        self.error = Some(TestError {
            category: ErrorCategory::ParseError,
            message,
            details: None,
        });
    }

    pub fn mark_execution_error(&mut self, message: String) {
        self.success = false;
        self.execute_success = false;
        self.error = Some(TestError {
            category: ErrorCategory::ExecutionError,
            message,
            details: None,
        });
    }
}

/// Error information for a failed test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestError {
    pub category: ErrorCategory,
    pub message: String,
    pub details: Option<HashMap<String, String>>,
}

/// Categories of test errors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCategory {
    ParseError,
    ExecutionError,
    SemanticError,
    TimeoutError,
    RateLimitError,
    ProviderError,
    ValidationError,
}

/// Full execution context for a test
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExecutionContext {
    /// The test case description
    pub test_description: String,
    /// The task prompt given to the LLM
    pub task_prompt: String,
    /// The full system prompt
    pub system_prompt: String,
    /// The complete user prompt (including document context)
    pub full_user_prompt: String,
    /// Raw LLM response
    pub raw_response: String,
    /// Extracted UCL command(s)
    pub extracted_ucl: String,
    /// Parsed command representation (if successful)
    pub parsed_command: Option<String>,
    /// Expected pattern (if any)
    pub expected_pattern: Option<String>,
    /// Whether the pattern matched
    pub pattern_matched: Option<bool>,
}

/// A snapshot of the document state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSnapshot {
    /// When this snapshot was taken
    pub captured_at: chrono::DateTime<chrono::Utc>,
    /// Number of blocks in the document
    pub block_count: usize,
    /// Block structure (id -> block info)
    pub blocks: HashMap<String, BlockSnapshot>,
    /// Document metadata
    pub metadata: Option<serde_json::Value>,
}

impl DocumentSnapshot {
    pub fn from_document(doc: &ucm_core::Document) -> Self {
        let mut blocks = HashMap::new();
        let mut parent_lookup: HashMap<String, String> = HashMap::new();

        for (parent, children) in &doc.structure {
            for child in children {
                parent_lookup.insert(child.to_string(), parent.to_string());
            }
        }

        for (id, block) in &doc.blocks {
            let children_count = doc
                .structure
                .get(id)
                .map(|children| children.len())
                .unwrap_or(0);

            let semantic_role = block
                .metadata
                .semantic_role
                .as_ref()
                .map(|role| role.to_string());

            let role_category = block
                .metadata
                .semantic_role
                .as_ref()
                .map(|role| role.category);

            let heading_level = role_category.and_then(role_category_to_heading_level);
            let role_category_string = block
                .metadata
                .semantic_role
                .as_ref()
                .map(|role| role.category.as_str().to_string());

            blocks.insert(
                id.to_string(),
                BlockSnapshot {
                    id: id.to_string(),
                    content_type: format!("{:?}", block.content),
                    content_preview: content_preview(&block.content),
                    label: block.metadata.label.clone(),
                    parent_id: parent_lookup.get(&id.to_string()).cloned(),
                    children_count,
                    semantic_role,
                    role_category: role_category_string,
                    heading_level,
                },
            );
        }

        Self {
            captured_at: chrono::Utc::now(),
            block_count: doc.blocks.len(),
            blocks,
            metadata: None,
        }
    }
}

/// Snapshot of a single block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockSnapshot {
    pub id: String,
    pub content_type: String,
    pub content_preview: String,
    pub label: Option<String>,
    pub parent_id: Option<String>,
    pub children_count: usize,
    pub semantic_role: Option<String>,
    pub role_category: Option<String>,
    pub heading_level: Option<u8>,
}

/// Diff between two document states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDiff {
    /// Blocks that were added
    pub added_blocks: Vec<String>,
    /// Blocks that were removed
    pub removed_blocks: Vec<String>,
    /// Blocks that were modified
    pub modified_blocks: Vec<BlockModification>,
    /// Summary of changes
    pub summary: String,
}

impl DocumentDiff {
    pub fn compute(before: &DocumentSnapshot, after: &DocumentSnapshot) -> Self {
        let before_ids: std::collections::HashSet<_> = before.blocks.keys().collect();
        let after_ids: std::collections::HashSet<_> = after.blocks.keys().collect();

        let added_blocks: Vec<String> = after_ids
            .difference(&before_ids)
            .map(|s| (*s).clone())
            .collect();

        let removed_blocks: Vec<String> = before_ids
            .difference(&after_ids)
            .map(|s| (*s).clone())
            .collect();

        let mut modified_blocks = Vec::new();
        for id in before_ids.intersection(&after_ids) {
            let before_block = &before.blocks[*id];
            let after_block = &after.blocks[*id];

            if before_block.content_preview != after_block.content_preview {
                modified_blocks.push(BlockModification {
                    block_id: (*id).clone(),
                    field: "content".into(),
                    before: before_block.content_preview.clone(),
                    after: after_block.content_preview.clone(),
                });
            }
        }

        let summary = format!(
            "{} added, {} removed, {} modified",
            added_blocks.len(),
            removed_blocks.len(),
            modified_blocks.len()
        );

        Self {
            added_blocks,
            removed_blocks,
            modified_blocks,
            summary,
        }
    }
}

/// A modification to a block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockModification {
    pub block_id: String,
    pub field: String,
    pub before: String,
    pub after: String,
}

/// Helper to create a content preview
fn content_preview(content: &ucm_core::Content) -> String {
    match content {
        ucm_core::Content::Text(t) => {
            let text = &t.text;
            if text.len() > 100 {
                format!("{}...", &text[..100])
            } else {
                text.clone()
            }
        }
        ucm_core::Content::Code(c) => {
            let lang = &c.language;
            let source = &c.source;
            format!(
                "[code:{}] {}",
                lang,
                if source.len() > 50 {
                    &source[..50]
                } else {
                    source
                }
            )
        }
        ucm_core::Content::Json { value, .. } => {
            format!(
                "[json] {}",
                serde_json::to_string(value)
                    .unwrap_or_default()
                    .chars()
                    .take(50)
                    .collect::<String>()
            )
        }
        _ => format!("[{:?}]", std::mem::discriminant(content)),
    }
}
