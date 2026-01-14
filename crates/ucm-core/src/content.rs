//! Content types for UCM blocks.
//!
//! Each block contains typed content that can be text, tables, code,
//! math expressions, media, JSON, or binary data.

use crate::id::BlockId;
use serde::{Deserialize, Serialize};

/// The content payload of a block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Content {
    /// Plain, markdown, or rich text
    Text(Text),

    /// Tabular data with optional schema
    Table(Table),

    /// Source code with language hint
    Code(Code),

    /// Mathematical expressions
    Math(Math),

    /// Media references (images, audio, video)
    Media(Media),

    /// Structured JSON data
    Json {
        value: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        schema: Option<JsonSchema>,
    },

    /// Raw binary data
    Binary {
        mime_type: String,
        #[serde(with = "base64_serde")]
        data: Vec<u8>,
        #[serde(default)]
        encoding: BinaryEncoding,
    },

    /// Composite block (contains other blocks by reference)
    Composite {
        layout: CompositeLayout,
        children: Vec<BlockId>,
    },
}

impl Content {
    /// Get the type tag for hashing and identification
    pub fn type_tag(&self) -> &'static str {
        match self {
            Content::Text(_) => "text",
            Content::Table(_) => "table",
            Content::Code(_) => "code",
            Content::Math(_) => "math",
            Content::Media(_) => "media",
            Content::Json { .. } => "json",
            Content::Binary { .. } => "binary",
            Content::Composite { .. } => "composite",
        }
    }

    /// Create a simple text content
    pub fn text(text: impl Into<String>) -> Self {
        Content::Text(Text {
            text: text.into(),
            format: TextFormat::Plain,
        })
    }

    /// Create markdown text content
    pub fn markdown(text: impl Into<String>) -> Self {
        Content::Text(Text {
            text: text.into(),
            format: TextFormat::Markdown,
        })
    }

    /// Create code content
    pub fn code(language: impl Into<String>, source: impl Into<String>) -> Self {
        Content::Code(Code {
            language: language.into(),
            source: source.into(),
            highlights: Vec::new(),
        })
    }

    /// Create JSON content
    pub fn json(value: serde_json::Value) -> Self {
        Content::Json {
            value,
            schema: None,
        }
    }

    /// Create table content from rows of strings
    pub fn table(rows: Vec<Vec<String>>) -> Self {
        let columns = if rows.is_empty() {
            Vec::new()
        } else {
            rows[0]
                .iter()
                .enumerate()
                .map(|(i, _)| Column {
                    name: format!("col{}", i),
                    data_type: Some(DataType::Text),
                    nullable: true,
                })
                .collect()
        };

        let table_rows = rows
            .into_iter()
            .map(|r| Row {
                cells: r.into_iter().map(Cell::Text).collect(),
            })
            .collect();

        Content::Table(Table {
            columns,
            rows: table_rows,
            schema: None,
        })
    }

    /// Check if the content is empty
    pub fn is_empty(&self) -> bool {
        match self {
            Content::Text(t) => t.text.is_empty(),
            Content::Table(t) => t.rows.is_empty(),
            Content::Code(c) => c.source.is_empty(),
            Content::Math(m) => m.expression.is_empty(),
            Content::Media(_) => false,
            Content::Json { value, .. } => value.is_null(),
            Content::Binary { data, .. } => data.is_empty(),
            Content::Composite { children, .. } => children.is_empty(),
        }
    }

    /// Get approximate size in bytes
    pub fn size_bytes(&self) -> usize {
        match self {
            Content::Text(t) => t.text.len(),
            Content::Table(t) => {
                t.columns.iter().map(|c| c.name.len()).sum::<usize>()
                    + t.rows
                        .iter()
                        .flat_map(|r| &r.cells)
                        .map(|c| c.size_bytes())
                        .sum::<usize>()
            }
            Content::Code(c) => c.source.len(),
            Content::Math(m) => m.expression.len(),
            Content::Media(m) => match &m.source {
                MediaSource::Base64(s) => s.len(),
                MediaSource::Url(s) => s.len(),
                _ => 32,
            },
            Content::Json { value, .. } => value.to_string().len(),
            Content::Binary { data, .. } => data.len(),
            Content::Composite { children, .. } => children.len() * 12,
        }
    }
}

/// Text content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Text {
    pub text: String,
    #[serde(default)]
    pub format: TextFormat,
}

/// Text format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextFormat {
    #[default]
    Plain,
    Markdown,
    Rich,
}

/// Table content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<TableSchema>,
}

impl Table {
    pub fn new(columns: Vec<Column>) -> Self {
        Self {
            columns,
            rows: Vec::new(),
            schema: None,
        }
    }

    pub fn with_rows(mut self, rows: Vec<Row>) -> Self {
        self.rows = rows;
        self
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}

/// Table column definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<DataType>,
    #[serde(default = "default_true")]
    pub nullable: bool,
}

fn default_true() -> bool {
    true
}

impl Column {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data_type: None,
            nullable: true,
        }
    }

    pub fn with_type(mut self, data_type: DataType) -> Self {
        self.data_type = Some(data_type);
        self
    }

    pub fn not_null(mut self) -> Self {
        self.nullable = false;
        self
    }
}

/// Data type for table columns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    Text,
    Integer,
    Float,
    Boolean,
    Date,
    DateTime,
    Json,
}

/// Table row
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Row {
    pub cells: Vec<Cell>,
}

impl Row {
    pub fn new(cells: Vec<Cell>) -> Self {
        Self { cells }
    }

    pub fn from_strings(values: Vec<&str>) -> Self {
        Self {
            cells: values
                .into_iter()
                .map(|s| Cell::Text(s.to_string()))
                .collect(),
        }
    }
}

/// Table cell value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Cell {
    Null,
    Text(String),
    Number(f64),
    Boolean(bool),
    Date(String),     // ISO 8601 date
    DateTime(String), // ISO 8601 datetime
    Json(serde_json::Value),
}

impl Cell {
    pub fn size_bytes(&self) -> usize {
        match self {
            Cell::Null => 0,
            Cell::Text(s) => s.len(),
            Cell::Number(_) => 8,
            Cell::Boolean(_) => 1,
            Cell::Date(s) => s.len(),
            Cell::DateTime(s) => s.len(),
            Cell::Json(v) => v.to_string().len(),
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Cell::Null)
    }
}

impl From<&str> for Cell {
    fn from(s: &str) -> Self {
        Cell::Text(s.to_string())
    }
}

impl From<String> for Cell {
    fn from(s: String) -> Self {
        Cell::Text(s)
    }
}

impl From<i64> for Cell {
    fn from(n: i64) -> Self {
        Cell::Number(n as f64)
    }
}

impl From<f64> for Cell {
    fn from(n: f64) -> Self {
        Cell::Number(n)
    }
}

impl From<bool> for Cell {
    fn from(b: bool) -> Self {
        Cell::Boolean(b)
    }
}

/// Table schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<Constraint>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indices: Vec<IndexDef>,
}

/// Table constraint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Constraint {
    Unique {
        columns: Vec<String>,
    },
    Check {
        expression: String,
    },
    ForeignKey {
        columns: Vec<String>,
        references: String,
    },
}

/// Index definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexDef {
    pub name: String,
    pub columns: Vec<String>,
    #[serde(default)]
    pub unique: bool,
}

/// Code content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Code {
    pub language: String,
    pub source: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub highlights: Vec<LineRange>,
}

impl Code {
    pub fn new(language: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            language: language.into(),
            source: source.into(),
            highlights: Vec::new(),
        }
    }

    pub fn line_count(&self) -> usize {
        self.source.lines().count()
    }

    pub fn get_lines(&self, start: usize, end: usize) -> Option<String> {
        let lines: Vec<&str> = self.source.lines().collect();
        if start > 0 && end <= lines.len() && start <= end {
            Some(lines[start - 1..end].join("\n"))
        } else {
            None
        }
    }
}

/// Line range for code highlights
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineRange {
    pub start: usize,
    pub end: usize,
}

impl LineRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn single(line: usize) -> Self {
        Self {
            start: line,
            end: line,
        }
    }
}

/// Math content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Math {
    pub format: MathFormat,
    pub expression: String,
    #[serde(default)]
    pub display_mode: bool,
}

impl Math {
    pub fn latex(expression: impl Into<String>) -> Self {
        Self {
            format: MathFormat::LaTeX,
            expression: expression.into(),
            display_mode: false,
        }
    }

    pub fn display(mut self) -> Self {
        self.display_mode = true;
        self
    }
}

/// Math format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MathFormat {
    #[default]
    LaTeX,
    MathML,
    AsciiMath,
}

/// Media content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Media {
    pub media_type: MediaType,
    pub source: MediaSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<[u8; 32]>,
}

impl Media {
    pub fn image(source: MediaSource) -> Self {
        Self {
            media_type: MediaType::Image,
            source,
            alt_text: None,
            dimensions: None,
            content_hash: None,
        }
    }

    pub fn with_alt(mut self, alt: impl Into<String>) -> Self {
        self.alt_text = Some(alt.into());
        self
    }

    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.dimensions = Some(Dimensions { width, height });
        self
    }
}

/// Media type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Image,
    Audio,
    Video,
    Document,
}

/// Media source
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "lowercase")]
pub enum MediaSource {
    Url(String),
    Base64(String),
    Reference(BlockId),
    External(ExternalRef),
}

impl MediaSource {
    pub fn url(url: impl Into<String>) -> Self {
        MediaSource::Url(url.into())
    }
}

/// External storage reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalRef {
    pub provider: String,
    pub bucket: String,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// Dimensions for media
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

/// JSON schema reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonSchema {
    Uri(String),
    Inline(serde_json::Value),
}

/// Binary encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BinaryEncoding {
    #[default]
    Raw,
    Base64,
    Hex,
}

/// Composite layout
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompositeLayout {
    #[default]
    Vertical,
    Horizontal,
    Grid(usize),
    Tabs,
}

// Base64 serde helper
mod base64_serde {
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&STANDARD.encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        STANDARD.decode(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_type_tag() {
        assert_eq!(Content::text("hello").type_tag(), "text");
        assert_eq!(Content::code("rust", "fn main() {}").type_tag(), "code");
        assert_eq!(Content::json(serde_json::json!({})).type_tag(), "json");
    }

    #[test]
    fn test_text_content() {
        let content = Content::text("Hello, world!");
        match content {
            Content::Text(t) => {
                assert_eq!(t.text, "Hello, world!");
                assert_eq!(t.format, TextFormat::Plain);
            }
            _ => panic!("Expected Text content"),
        }
    }

    #[test]
    fn test_table_content() {
        let mut table = Table::new(vec![
            Column::new("name").with_type(DataType::Text),
            Column::new("age").with_type(DataType::Integer),
        ]);
        table.add_row(Row::new(vec![Cell::from("Alice"), Cell::from(30i64)]));

        assert_eq!(table.column_count(), 2);
        assert_eq!(table.row_count(), 1);
    }

    #[test]
    fn test_code_lines() {
        let code = Code::new("rust", "line1\nline2\nline3\nline4");
        assert_eq!(code.line_count(), 4);
        assert_eq!(code.get_lines(2, 3), Some("line2\nline3".to_string()));
    }

    #[test]
    fn test_content_serialization() {
        let content = Content::text("Hello");
        let json = serde_json::to_string(&content).unwrap();
        let parsed: Content = serde_json::from_str(&json).unwrap();
        assert_eq!(content, parsed);
    }
}
