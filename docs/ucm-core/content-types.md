# Content Types

UCM supports rich, typed content through the `Content` enum. Each variant is optimized for its specific use case.

## Content Enum

```rust
pub enum Content {
    Text(Text),
    Table(Table),
    Code(Code),
    Math(Math),
    Media(Media),
    Json { value: serde_json::Value, schema: Option<JsonSchema> },
    Binary { mime_type: String, data: Vec<u8>, encoding: BinaryEncoding },
    Composite { layout: CompositeLayout, children: Vec<BlockId> },
}
```

## Text Content

For plain text, markdown, or rich text content.

### Structure

```rust
pub struct Text {
    pub text: String,
    pub format: TextFormat,
}

pub enum TextFormat {
    Plain,    // Default
    Markdown,
    Rich,
}
```

### Creating Text Content

```rust
use ucm_core::Content;

// Plain text (default)
let plain = Content::text("Hello, world!");

// Markdown text
let markdown = Content::markdown("**Bold** and *italic* text");

// Direct construction
use ucm_core::content::{Text, TextFormat};
let rich = Content::Text(Text {
    text: "Rich text content".to_string(),
    format: TextFormat::Rich,
});
```

### Properties

```rust
let content = Content::text("Hello");

// Type tag for identification
assert_eq!(content.type_tag(), "text");

// Check if empty
assert!(!content.is_empty());

// Size in bytes
let size = content.size_bytes();
```

## Code Content

For source code with language hints and optional line highlights.

### Structure

```rust
pub struct Code {
    pub language: String,
    pub source: String,
    pub highlights: Vec<LineRange>,
}

pub struct LineRange {
    pub start: usize,
    pub end: usize,
}
```

### Creating Code Content

```rust
use ucm_core::Content;
use ucm_core::content::{Code, LineRange};

// Simple creation
let code = Content::code("rust", r#"
fn main() {
    println!("Hello, world!");
}
"#);

// With highlights
let code = Content::Code(Code {
    language: "python".to_string(),
    source: "def hello():\n    print('Hello')\n\nhello()".to_string(),
    highlights: vec![
        LineRange::new(1, 2),  // Highlight lines 1-2
        LineRange::single(4),   // Highlight line 4
    ],
});
```

### Code Operations

```rust
use ucm_core::content::Code;

let code = Code::new("rust", "line1\nline2\nline3\nline4");

// Line count
assert_eq!(code.line_count(), 4);

// Extract lines (1-indexed)
let lines = code.get_lines(2, 3);
assert_eq!(lines, Some("line2\nline3".to_string()));
```

## Table Content

For tabular data with optional schema.

### Structure

```rust
pub struct Table {
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
    pub schema: Option<TableSchema>,
}

pub struct Column {
    pub name: String,
    pub data_type: Option<DataType>,
    pub nullable: bool,
}

pub struct Row {
    pub cells: Vec<Cell>,
}

pub enum Cell {
    Null,
    Text(String),
    Number(f64),
    Boolean(bool),
    Date(String),
    DateTime(String),
    Json(serde_json::Value),
}

pub enum DataType {
    Text,
    Integer,
    Float,
    Boolean,
    Date,
    DateTime,
    Json,
}
```

### Creating Tables

```rust
use ucm_core::Content;
use ucm_core::content::{Table, Column, Row, Cell, DataType};

// Simple table from string rows
let table = Content::table(vec![
    vec!["Name".into(), "Age".into(), "City".into()],
    vec!["Alice".into(), "30".into(), "NYC".into()],
    vec!["Bob".into(), "25".into(), "LA".into()],
]);

// Typed table with schema
let mut table = Table::new(vec![
    Column::new("name").with_type(DataType::Text).not_null(),
    Column::new("age").with_type(DataType::Integer),
    Column::new("active").with_type(DataType::Boolean),
]);

table.add_row(Row::new(vec![
    Cell::Text("Alice".into()),
    Cell::Number(30.0),
    Cell::Boolean(true),
]));

table.add_row(Row::from_strings(vec!["Bob", "25", "false"]));

let content = Content::Table(table);
```

### Table Operations

```rust
let table = Table::new(vec![
    Column::new("col1"),
    Column::new("col2"),
]);

assert_eq!(table.column_count(), 2);
assert_eq!(table.row_count(), 0);
```

### Table Schema

```rust
use ucm_core::content::{TableSchema, Constraint, IndexDef};

let schema = TableSchema {
    primary_key: Some(vec!["id".to_string()]),
    constraints: vec![
        Constraint::Unique { columns: vec!["email".to_string()] },
        Constraint::Check { expression: "age >= 0".to_string() },
    ],
    indices: vec![
        IndexDef {
            name: "idx_name".to_string(),
            columns: vec!["name".to_string()],
            unique: false,
        },
    ],
};
```

## Math Content

For mathematical expressions in various formats.

### Structure

```rust
pub struct Math {
    pub format: MathFormat,
    pub expression: String,
    pub display_mode: bool,
}

pub enum MathFormat {
    LaTeX,     // Default
    MathML,
    AsciiMath,
}
```

### Creating Math Content

```rust
use ucm_core::content::Math;
use ucm_core::Content;

// LaTeX (inline)
let inline = Content::Math(Math::latex("E = mc^2"));

// LaTeX (display mode)
let display = Content::Math(Math::latex(r"\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}").display());

// MathML
let mathml = Content::Math(Math {
    format: ucm_core::content::MathFormat::MathML,
    expression: "<math>...</math>".to_string(),
    display_mode: false,
});
```

## Media Content

For images, audio, video, and documents.

### Structure

```rust
pub struct Media {
    pub media_type: MediaType,
    pub source: MediaSource,
    pub alt_text: Option<String>,
    pub dimensions: Option<Dimensions>,
    pub content_hash: Option<[u8; 32]>,
}

pub enum MediaType {
    Image,
    Audio,
    Video,
    Document,
}

pub enum MediaSource {
    Url(String),
    Base64(String),
    Reference(BlockId),
    External(ExternalRef),
}

pub struct ExternalRef {
    pub provider: String,
    pub bucket: String,
    pub key: String,
    pub region: Option<String>,
}

pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}
```

### Creating Media Content

```rust
use ucm_core::content::{Media, MediaSource};
use ucm_core::Content;

// Image from URL
let image = Content::Media(
    Media::image(MediaSource::url("https://example.com/image.png"))
        .with_alt("Example image")
        .with_dimensions(800, 600)
);

// Image from base64
let image = Content::Media(
    Media::image(MediaSource::Base64("iVBORw0KGgo...".to_string()))
);

// External storage reference
use ucm_core::content::ExternalRef;
let image = Content::Media(Media {
    media_type: ucm_core::content::MediaType::Image,
    source: MediaSource::External(ExternalRef {
        provider: "s3".to_string(),
        bucket: "my-bucket".to_string(),
        key: "images/photo.jpg".to_string(),
        region: Some("us-east-1".to_string()),
    }),
    alt_text: Some("Photo".to_string()),
    dimensions: None,
    content_hash: None,
});
```

## JSON Content

For structured JSON data with optional schema validation.

### Structure

```rust
pub enum JsonSchema {
    Uri(String),           // Reference to external schema
    Inline(serde_json::Value),  // Embedded schema
}
```

### Creating JSON Content

```rust
use ucm_core::Content;
use ucm_core::content::JsonSchema;

// Simple JSON
let json = Content::json(serde_json::json!({
    "name": "Alice",
    "age": 30,
    "tags": ["developer", "rust"]
}));

// JSON with schema reference
let json = Content::Json {
    value: serde_json::json!({"type": "user", "id": 123}),
    schema: Some(JsonSchema::Uri("https://example.com/schemas/user.json".to_string())),
};

// JSON with inline schema
let json = Content::Json {
    value: serde_json::json!({"count": 42}),
    schema: Some(JsonSchema::Inline(serde_json::json!({
        "type": "object",
        "properties": {
            "count": {"type": "integer"}
        }
    }))),
};
```

## Binary Content

For raw binary data with MIME type.

### Structure

```rust
pub enum BinaryEncoding {
    Raw,     // Default
    Base64,
    Hex,
}
```

### Creating Binary Content

```rust
use ucm_core::Content;
use ucm_core::content::BinaryEncoding;

let binary = Content::Binary {
    mime_type: "application/pdf".to_string(),
    data: vec![0x25, 0x50, 0x44, 0x46], // PDF magic bytes
    encoding: BinaryEncoding::Raw,
};
```

## Composite Content

For blocks that contain references to other blocks.

### Structure

```rust
pub enum CompositeLayout {
    Vertical,   // Default - stack vertically
    Horizontal, // Side by side
    Grid(usize), // Grid with N columns
    Tabs,       // Tabbed interface
}
```

### Creating Composite Content

```rust
use ucm_core::Content;
use ucm_core::content::CompositeLayout;
use ucm_core::BlockId;

let composite = Content::Composite {
    layout: CompositeLayout::Grid(2),
    children: vec![
        child1_id,
        child2_id,
        child3_id,
        child4_id,
    ],
};
```

## Content Operations

### Type Identification

```rust
let content = Content::code("rust", "fn main() {}");
assert_eq!(content.type_tag(), "code");
```

### Empty Check

```rust
assert!(Content::text("").is_empty());
assert!(!Content::text("hello").is_empty());
assert!(Content::table(vec![]).is_empty());
```

### Size Estimation

```rust
let content = Content::text("Hello, world!");
let bytes = content.size_bytes();
```

## Serialization

All content types serialize to JSON with a `type` discriminator:

```json
// Text
{
  "type": "text",
  "text": "Hello, world!",
  "format": "plain"
}

// Code
{
  "type": "code",
  "language": "rust",
  "source": "fn main() {}",
  "highlights": []
}

// Table
{
  "type": "table",
  "columns": [{"name": "col1", "nullable": true}],
  "rows": [{"cells": ["value"]}]
}
```

## Best Practices

### 1. Choose the Right Type

```rust
// Use Text for prose
Content::text("A paragraph of explanation...")

// Use Code for source code (enables syntax highlighting)
Content::code("python", "def hello(): pass")

// Use Table for structured data (enables querying)
Content::table(vec![...])

// Use JSON for configuration/metadata
Content::json(serde_json::json!({...}))
```

### 2. Use Markdown for Rich Text

```rust
// Prefer markdown for formatted text
Content::markdown("**Important**: This is *emphasized*")

// Rather than plain text with formatting lost
Content::text("Important: This is emphasized")
```

### 3. Include Language Hints for Code

```rust
// Good - enables syntax highlighting and analysis
Content::code("typescript", "const x: number = 42;")

// Less ideal - no language information
Content::code("", "const x = 42")
```

### 4. Provide Alt Text for Media

```rust
Media::image(MediaSource::url("..."))
    .with_alt("Diagram showing system architecture")
```

## See Also

- [Blocks](./blocks.md) - Block structure
- [Metadata](./metadata.md) - Content metadata
- [ID Generation](./id-generation.md) - Content-addressed IDs
