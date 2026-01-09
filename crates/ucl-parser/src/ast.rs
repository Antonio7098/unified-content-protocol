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
    pub fn from_str(s: &str) -> Option<Self> {
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

    pub fn to_string(&self) -> String {
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
