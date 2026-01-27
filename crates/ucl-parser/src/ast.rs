//! Abstract Syntax Tree for UCL documents.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A complete UCL document
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UclDocument {
    /// Structure declarations (parent -> children)
    pub structure: HashMap<String, Vec<String>>,
    /// Block definitions
    pub blocks: Vec<BlockDef>,
    /// Commands to execute
    pub commands: Vec<Command>,
}

impl UclDocument {
    pub fn new() -> Self {
        Self {
            structure: HashMap::new(),
            blocks: Vec::new(),
            commands: Vec::new(),
        }
    }
}

impl Default for UclDocument {
    fn default() -> Self {
        Self::new()
    }
}

/// Block definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockDef {
    /// Content type (text, table, code, etc.)
    pub content_type: ContentType,
    /// Block ID
    pub id: String,
    /// Properties (label, tags, etc.)
    pub properties: HashMap<String, Value>,
    /// Content literal
    pub content: String,
}

/// Content type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Text,
    Table,
    Code,
    Math,
    Media,
    Json,
    Binary,
    Composite,
}

impl ContentType {
    pub fn parse_content_type(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "text" => Some(Self::Text),
            "table" => Some(Self::Table),
            "code" => Some(Self::Code),
            "math" => Some(Self::Math),
            "media" => Some(Self::Media),
            "json" => Some(Self::Json),
            "binary" => Some(Self::Binary),
            "composite" => Some(Self::Composite),
            _ => None,
        }
    }
}

/// UCL command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Command {
    // Document modification commands
    Edit(EditCommand),
    Move(MoveCommand),
    Append(AppendCommand),
    Delete(DeleteCommand),
    Prune(PruneCommand),
    Fold(FoldCommand),
    Link(LinkCommand),
    Unlink(UnlinkCommand),
    Snapshot(SnapshotCommand),
    Transaction(TransactionCommand),
    Atomic(Vec<Command>),
    WriteSection(WriteSectionCommand),

    // Agent traversal commands
    Goto(GotoCommand),
    Back(BackCommand),
    Expand(ExpandCommand),
    Follow(FollowCommand),
    Path(PathFindCommand),
    Search(SearchCommand),
    Find(FindCommand),
    View(ViewCommand),

    // Context window commands
    Context(ContextCommand),
}

/// EDIT command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditCommand {
    pub block_id: String,
    pub path: Path,
    pub operator: Operator,
    pub value: Value,
    pub condition: Option<Condition>,
}

/// MOVE command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveCommand {
    pub block_id: String,
    pub target: MoveTarget,
}

/// Move target
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MoveTarget {
    ToParent {
        parent_id: String,
        index: Option<usize>,
    },
    Before {
        sibling_id: String,
    },
    After {
        sibling_id: String,
    },
}

/// APPEND command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppendCommand {
    pub parent_id: String,
    pub content_type: ContentType,
    pub properties: HashMap<String, Value>,
    pub content: String,
    pub index: Option<usize>,
}

/// DELETE command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCommand {
    pub block_id: Option<String>,
    pub cascade: bool,
    pub preserve_children: bool,
    pub condition: Option<Condition>,
}

/// PRUNE command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PruneCommand {
    pub target: PruneTarget,
    pub dry_run: bool,
}

/// Prune target
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PruneTarget {
    Unreachable,
    Where(Condition),
}

/// FOLD command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FoldCommand {
    pub block_id: String,
    pub depth: Option<usize>,
    pub max_tokens: Option<usize>,
    pub preserve_tags: Vec<String>,
}

/// LINK command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkCommand {
    pub source_id: String,
    pub edge_type: String,
    pub target_id: String,
    pub metadata: HashMap<String, Value>,
}

/// UNLINK command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnlinkCommand {
    pub source_id: String,
    pub edge_type: String,
    pub target_id: String,
}

/// SNAPSHOT command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SnapshotCommand {
    Create {
        name: String,
        description: Option<String>,
    },
    Restore {
        name: String,
    },
    List,
    Delete {
        name: String,
    },
    Diff {
        name1: String,
        name2: String,
    },
}

/// Transaction command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionCommand {
    Begin { name: Option<String> },
    Commit { name: Option<String> },
    Rollback { name: Option<String> },
}

/// WRITE_SECTION command - write markdown to a section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WriteSectionCommand {
    /// Target section block ID
    pub section_id: String,
    /// Markdown content to write
    pub markdown: String,
    /// Base heading level for relative heading adjustment
    pub base_heading_level: Option<usize>,
}

// ============================================================================
// Agent Traversal Commands
// ============================================================================

/// GOTO command - navigate cursor to a specific block
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GotoCommand {
    /// Target block ID to navigate to
    pub block_id: String,
}

/// BACK command - go back in navigation history
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackCommand {
    /// Number of steps to go back (default: 1)
    pub steps: usize,
}

impl Default for BackCommand {
    fn default() -> Self {
        Self { steps: 1 }
    }
}

/// Direction for graph expansion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum ExpandDirection {
    /// Expand to children (BFS)
    #[default]
    Down,
    /// Expand to ancestors
    Up,
    /// Expand both directions
    Both,
    /// Follow semantic edges only
    Semantic,
}

impl ExpandDirection {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "down" => Some(Self::Down),
            "up" => Some(Self::Up),
            "both" => Some(Self::Both),
            "semantic" => Some(Self::Semantic),
            _ => None,
        }
    }
}

/// View mode for block content display
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum ViewMode {
    /// Show full content
    #[default]
    Full,
    /// Show first N characters as preview
    Preview { length: usize },
    /// Show only metadata (role, tags, edge counts)
    Metadata,
    /// Show only block IDs and structure
    IdsOnly,
}

impl ViewMode {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "full" => Some(Self::Full),
            "preview" => Some(Self::Preview { length: 100 }),
            "metadata" => Some(Self::Metadata),
            "ids" | "idsonly" | "ids_only" => Some(Self::IdsOnly),
            _ => None,
        }
    }
}

/// Filter criteria for traversal operations
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TraversalFilterCriteria {
    /// Include only blocks with these roles
    pub include_roles: Vec<String>,
    /// Exclude blocks with these roles
    pub exclude_roles: Vec<String>,
    /// Include only blocks with these tags
    pub include_tags: Vec<String>,
    /// Exclude blocks with these tags
    pub exclude_tags: Vec<String>,
    /// Filter by content pattern (regex)
    pub content_pattern: Option<String>,
    /// Filter by edge types to follow
    pub edge_types: Vec<String>,
}

/// EXPAND command - expand from a block in a direction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpandCommand {
    /// Block ID to expand from
    pub block_id: String,
    /// Direction to expand
    pub direction: ExpandDirection,
    /// Maximum depth to expand
    pub depth: usize,
    /// View mode for results
    pub mode: Option<ViewMode>,
    /// Filter criteria
    pub filter: Option<TraversalFilterCriteria>,
}

impl Default for ExpandCommand {
    fn default() -> Self {
        Self {
            block_id: String::new(),
            direction: ExpandDirection::Down,
            depth: 1,
            mode: None,
            filter: None,
        }
    }
}

/// FOLLOW command - follow edges from a block
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowCommand {
    /// Source block ID
    pub source_id: String,
    /// Edge types to follow
    pub edge_types: Vec<String>,
    /// Optional specific target block
    pub target_id: Option<String>,
}

/// PATH command - find path between two blocks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathFindCommand {
    /// Starting block ID
    pub from_id: String,
    /// Target block ID
    pub to_id: String,
    /// Maximum path length
    pub max_length: Option<usize>,
}

/// SEARCH command - semantic search (requires RAG provider)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchCommand {
    /// Search query string
    pub query: String,
    /// Maximum number of results
    pub limit: Option<usize>,
    /// Minimum similarity threshold (0.0-1.0)
    pub min_similarity: Option<f32>,
    /// Filter criteria for results
    pub filter: Option<TraversalFilterCriteria>,
}

impl Default for SearchCommand {
    fn default() -> Self {
        Self {
            query: String::new(),
            limit: Some(10),
            min_similarity: None,
            filter: None,
        }
    }
}

/// FIND command - pattern-based search (no RAG needed)
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FindCommand {
    /// Find by semantic role
    pub role: Option<String>,
    /// Find by tag
    pub tag: Option<String>,
    /// Find by label
    pub label: Option<String>,
    /// Find by content pattern (regex)
    pub pattern: Option<String>,
}

/// Target for VIEW command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ViewTarget {
    /// View a specific block
    Block(String),
    /// View current cursor neighborhood
    Neighborhood,
}

/// VIEW command - view block or neighborhood content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewCommand {
    /// What to view
    pub target: ViewTarget,
    /// View mode
    pub mode: ViewMode,
    /// Depth for neighborhood view
    pub depth: Option<usize>,
}

impl Default for ViewCommand {
    fn default() -> Self {
        Self {
            target: ViewTarget::Neighborhood,
            mode: ViewMode::Full,
            depth: None,
        }
    }
}

// ============================================================================
// Context Window Commands
// ============================================================================

/// CTX command - context window operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContextCommand {
    /// Add block(s) to context
    Add(ContextAddCommand),
    /// Remove block from context
    Remove { block_id: String },
    /// Clear entire context window
    Clear,
    /// Expand context in a direction
    Expand(ContextExpandCommand),
    /// Compress context using a method
    Compress { method: CompressionMethod },
    /// Prune context based on criteria
    Prune(ContextPruneCommand),
    /// Render context for LLM prompt
    Render { format: Option<RenderFormat> },
    /// Get context statistics
    Stats,
    /// Set/clear focus block
    Focus { block_id: Option<String> },
}

/// Target for CTX ADD command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContextAddTarget {
    /// Add a single block
    Block(String),
    /// Add all results from last search/find
    Results,
    /// Add all children of a block
    Children { parent_id: String },
    /// Add all blocks in a path
    Path { from_id: String, to_id: String },
}

/// CTX ADD command options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextAddCommand {
    /// What to add
    pub target: ContextAddTarget,
    /// Reason for inclusion (for tracking)
    pub reason: Option<String>,
    /// Custom relevance score (0.0-1.0)
    pub relevance: Option<f32>,
}

/// CTX EXPAND command options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextExpandCommand {
    /// Direction to expand
    pub direction: ExpandDirection,
    /// Maximum depth
    pub depth: Option<usize>,
    /// Token budget for auto-expansion
    pub token_budget: Option<usize>,
}

/// CTX PRUNE command criteria
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextPruneCommand {
    /// Remove blocks below this relevance threshold
    pub min_relevance: Option<f32>,
    /// Remove blocks not accessed in this many seconds
    pub max_age_secs: Option<u64>,
}

/// Compression method for context
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompressionMethod {
    /// Truncate low-relevance content
    Truncate,
    /// Summarize content (requires summarizer)
    Summarize,
    /// Keep only structure, no content
    StructureOnly,
}

impl CompressionMethod {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "truncate" => Some(Self::Truncate),
            "summarize" => Some(Self::Summarize),
            "structure_only" | "structureonly" | "structure" => Some(Self::StructureOnly),
            _ => None,
        }
    }
}

/// Render format for context output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum RenderFormat {
    /// Default format
    #[default]
    Default,
    /// Use short IDs (1, 2, 3...) for token efficiency
    ShortIds,
    /// Render as markdown
    Markdown,
}

impl RenderFormat {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "default" => Some(Self::Default),
            "short_ids" | "shortids" | "short" => Some(Self::ShortIds),
            "markdown" | "md" => Some(Self::Markdown),
            _ => None,
        }
    }
}

/// Path expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Path {
    pub segments: Vec<PathSegment>,
}

impl Path {
    pub fn new(segments: Vec<PathSegment>) -> Self {
        Self { segments }
    }

    pub fn simple(name: &str) -> Self {
        Self {
            segments: vec![PathSegment::Property(name.to_string())],
        }
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.segments
                .iter()
                .map(|s| match s {
                    PathSegment::Property(p) => p.clone(),
                    PathSegment::Index(i) => format!("[{}]", i),
                    PathSegment::Slice { start, end } => match (start, end) {
                        (Some(s), Some(e)) => format!("[{}:{}]", s, e),
                        (Some(s), None) => format!("[{}:]", s),
                        (None, Some(e)) => format!("[:{}]", e),
                        (None, None) => "[:]".to_string(),
                    },
                    PathSegment::JsonPath(p) => format!("${}", p),
                })
                .collect::<Vec<_>>()
                .join(".")
        )
    }
}

/// Path segment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PathSegment {
    Property(String),
    Index(i64),
    Slice {
        start: Option<i64>,
        end: Option<i64>,
    },
    JsonPath(String),
}

/// Operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    Set,       // =
    Append,    // +=
    Remove,    // -=
    Increment, // ++
    Decrement, // --
}

/// Value literal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    BlockRef(String),
}

impl Value {
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Value::Null => serde_json::Value::Null,
            Value::Bool(b) => serde_json::Value::Bool(*b),
            Value::Number(n) => serde_json::json!(*n),
            Value::String(s) => serde_json::Value::String(s.clone()),
            Value::Array(arr) => {
                serde_json::Value::Array(arr.iter().map(|v| v.to_json()).collect())
            }
            Value::Object(obj) => {
                let map: serde_json::Map<String, serde_json::Value> =
                    obj.iter().map(|(k, v)| (k.clone(), v.to_json())).collect();
                serde_json::Value::Object(map)
            }
            Value::BlockRef(id) => serde_json::json!({"$ref": id}),
        }
    }
}

/// Condition for WHERE clauses
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Condition {
    Comparison {
        path: Path,
        op: ComparisonOp,
        value: Value,
    },
    Contains {
        path: Path,
        value: Value,
    },
    StartsWith {
        path: Path,
        prefix: String,
    },
    EndsWith {
        path: Path,
        suffix: String,
    },
    Matches {
        path: Path,
        regex: String,
    },
    Exists {
        path: Path,
    },
    IsNull {
        path: Path,
    },
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
}

/// Comparison operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonOp {
    Eq, // =
    Ne, // !=
    Gt, // >
    Ge, // >=
    Lt, // <
    Le, // <=
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_simple() {
        let path = Path::simple("content.text");
        assert_eq!(path.segments.len(), 1);
    }

    #[test]
    fn test_value_to_json() {
        let value = Value::Object(
            [("key".to_string(), Value::String("value".to_string()))]
                .into_iter()
                .collect(),
        );
        let json = value.to_json();
        assert_eq!(json["key"], "value");
    }
}
