# Markdown Translator

**ucp-translator-markdown** provides bidirectional conversion between Markdown and UCM documents.

## Overview

The Markdown translator enables:

- **Parsing** - Convert Markdown to UCM documents
- **Rendering** - Convert UCM documents to Markdown
- **Round-trip** - Preserve structure through conversion cycles
- **Semantic mapping** - Map Markdown elements to semantic roles

## Installation

```toml
[dependencies]
ucp-translator-markdown = "0.1.4"
```

## Quick Start

```rust
use ucp_translator_markdown::{parse_markdown, render_markdown};

// Parse Markdown to UCM
let markdown = r#"
# Introduction

Welcome to the guide.

## Getting Started

Here's some code:

```rust
fn main() {
    println!("Hello!");
}
```
"#;

let doc = parse_markdown(markdown).unwrap();
println!("Parsed {} blocks", doc.block_count());

// Render UCM back to Markdown
let rendered = render_markdown(&doc).unwrap();
println!("{}", rendered);
```

## Parsing Markdown

### MarkdownParser

```rust
use ucp_translator_markdown::MarkdownParser;

// Default parser
let parser = MarkdownParser::new();
let doc = parser.parse(markdown)?;

// With raw preservation (stores original markdown)
let parser = MarkdownParser::new().preserve_raw(true);
let doc = parser.parse(markdown)?;
```

### Supported Elements

| Markdown Element | UCM Content Type | Semantic Role |
|------------------|------------------|---------------|
| `# Heading` | Text | `heading1` |
| `## Heading` | Text | `heading2` |
| `### Heading` | Text | `heading3` |
| `#### Heading` | Text | `heading4` |
| `##### Heading` | Text | `heading5` |
| `###### Heading` | Text | `heading6` |
| Paragraph | Text | `paragraph` |
| `` ```code``` `` | Code | `code` |
| `- list item` | Text | `list` |
| `> quote` | Text | `quote` |
| `\| table \|` | Table | `table` |

### Inline Formatting

**Important**: Inline formatting (bold, italic, inline code, links) is **preserved as raw text**, not parsed into separate elements.

```markdown
This is **bold** and *italic* text with `code`.
```

Is stored as a single text block containing the literal markdown characters:
```
"This is **bold** and *italic* text with `code`."
```

This design choice:
- Preserves fidelity during round-trip conversion
- Keeps the block structure simple
- Delegates inline rendering to consuming applications

### List Marker Preservation

List markers (ordered and unordered) are stored in the raw text content:

```markdown
- First item
- Second item
1. Numbered item
2. Another numbered
```

The list content is stored with markers intact, ensuring round-trip fidelity.

### Heading Hierarchy

The parser creates a hierarchical structure based on heading levels:

```markdown
# Title           <- Child of root
## Section 1      <- Child of Title
### Subsection    <- Child of Section 1
## Section 2      <- Child of Title (sibling of Section 1)
```

Results in:

```
root
└── Title (H1)
    ├── Section 1 (H2)
    │   └── Subsection (H3)
    └── Section 2 (H2)
```

### Example: Parsing

```rust
use ucp_translator_markdown::MarkdownParser;
use ucm_core::Content;

let markdown = r#"
# My Document

This is the introduction.

## Chapter 1

Some content here.

```python
def hello():
    print("Hello!")
```

## Chapter 2

More content.
"#;

let parser = MarkdownParser::new();
let doc = parser.parse(markdown).unwrap();

// Check structure
println!("Total blocks: {}", doc.block_count());

// Find headings
for block in doc.blocks.values() {
    if let Some(role) = &block.metadata.semantic_role {
        if role.category.as_str().starts_with("heading") {
            if let Content::Text(text) = &block.content {
                println!("{}: {}", role.category.as_str(), text.text);
            }
        }
    }
}
```

## Rendering Markdown

### MarkdownRenderer

```rust
use ucp_translator_markdown::{MarkdownRenderer, HeadingMode};

// Default renderer
let renderer = MarkdownRenderer::new();
let markdown = renderer.render(&doc)?;

// With custom settings
let renderer = MarkdownRenderer::new()
    .indent_size(4)
    .heading_mode(HeadingMode::Explicit)
    .heading_offset(1);  // Start at H2
let markdown = renderer.render(&doc)?;
```

### Heading Modes

| Mode | Description |
|------|-------------|
| `Explicit` | Use semantic roles only (heading1, heading2, etc.) |
| `Structural` | Derive heading level from document tree depth |
| `Hybrid` | Use explicit roles when present, fall back to structural |

```rust
use ucp_translator_markdown::{MarkdownRenderer, HeadingMode};

// Explicit - only blocks with heading roles become headings
let renderer = MarkdownRenderer::new()
    .heading_mode(HeadingMode::Explicit);

// Structural - depth 1 = H1, depth 2 = H2, etc.
let renderer = MarkdownRenderer::new()
    .heading_mode(HeadingMode::Structural);

// Hybrid (default) - explicit when available, structural fallback
let renderer = MarkdownRenderer::new()
    .heading_mode(HeadingMode::Hybrid);
```

### Heading Offset

For nested documents (e.g., embedding in another document):

```rust
// Start headings at H2 instead of H1
let renderer = MarkdownRenderer::new()
    .heading_offset(1);

// H1 in UCM becomes ## in Markdown
// H2 in UCM becomes ### in Markdown
```

### Content Type Rendering

| UCM Content | Markdown Output |
|-------------|-----------------|
| Text (heading role) | `# Heading` |
| Text (paragraph) | Plain paragraph |
| Text (quote role) | `> Quoted text` |
| Text (list role) | Preserved list format |
| Code | `` ```lang\ncode\n``` `` |
| Table | Pipe-delimited table |
| Math (display) | `$$\nexpression\n$$` |
| Math (inline) | `$expression$` |
| Media | `![alt](url)` |
| JSON | `` ```json\n{...}\n``` `` |

### Example: Rendering

```rust
use ucp_translator_markdown::MarkdownRenderer;
use ucm_core::{Document, Block, Content};

let mut doc = Document::create();
let root = doc.root.clone();

// Add heading
let heading = Block::new(Content::text("My Document"), Some("heading1"));
let heading_id = doc.add_block(heading, &root).unwrap();

// Add paragraph
let para = Block::new(Content::text("This is a paragraph."), Some("paragraph"));
doc.add_block(para, &heading_id).unwrap();

// Add code
let code = Block::new(Content::code("rust", "fn main() {}"), Some("code"));
doc.add_block(code, &heading_id).unwrap();

// Render
let renderer = MarkdownRenderer::new();
let markdown = renderer.render(&doc).unwrap();

println!("{}", markdown);
// Output:
// # My Document
//
// This is a paragraph.
//
// ```rust
// fn main() {}
// ```
```

## Round-Trip Conversion

```rust
use ucp_translator_markdown::{parse_markdown, render_markdown};

let original = r#"# Title

Introduction paragraph.

## Section

Content here.
"#;

// Parse to UCM
let doc = parse_markdown(original).unwrap();

// Render back to Markdown
let rendered = render_markdown(&doc).unwrap();

// Should be equivalent
assert_eq!(original, rendered);
```

## Error Handling

```rust
use ucp_translator_markdown::{parse_markdown, TranslatorError};

match parse_markdown(markdown) {
    Ok(doc) => println!("Parsed {} blocks", doc.block_count()),
    Err(TranslatorError::ParseError { line, message }) => {
        eprintln!("Parse error at line {}: {}", line, message);
    }
    Err(TranslatorError::InvalidStructure(msg)) => {
        eprintln!("Invalid structure: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Complete Example

```rust
use ucp_translator_markdown::{MarkdownParser, MarkdownRenderer, HeadingMode};
use ucm_core::{Block, Content};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse complex Markdown
    let markdown = r#"
# Technical Guide

Welcome to this comprehensive technical guide.

## Installation

Install using cargo:

```bash
cargo add my-crate
```

## Usage

Here's a basic example:

```rust
use my_crate::Client;

fn main() {
    let client = Client::new();
    client.run();
}
```

### Advanced Usage

For advanced scenarios:

| Option | Description | Default |
|--------|-------------|---------|
| `debug` | Enable debug mode | `false` |
| `timeout` | Request timeout | `30s` |

> **Note**: Always configure timeouts in production.

## Conclusion

That's all you need to know!
"#;

    // Parse
    let parser = MarkdownParser::new();
    let mut doc = parser.parse(markdown)?;
    
    println!("Parsed document:");
    println!("  Blocks: {}", doc.block_count());
    println!("  Code blocks: {}", doc.indices.find_by_type("code").len());
    println!("  Tables: {}", doc.indices.find_by_type("table").len());
    
    // Modify document
    let root = doc.root.clone();
    let appendix = Block::new(Content::text("Appendix"), Some("heading2"));
    let appendix_id = doc.add_block(appendix, &root)?;
    
    let note = Block::new(
        Content::text("Additional resources available online."),
        Some("paragraph")
    );
    doc.add_block(note, &appendix_id)?;
    
    // Render back
    let renderer = MarkdownRenderer::new()
        .heading_mode(HeadingMode::Hybrid);
    let rendered = renderer.render(&doc)?;
    
    println!("\nRendered Markdown:\n{}", rendered);
    
    Ok(())
}
```

## Best Practices

### 1. Use Semantic Roles

```rust
// Good - semantic role preserved
let block = Block::new(Content::text("Title"), Some("heading1"));

// Less ideal - no semantic information
let block = Block::new(Content::text("# Title"), None);
```

### 2. Preserve Structure

```rust
// Parse and render maintain hierarchy
let doc = parse_markdown(markdown)?;
// Modify document...
let rendered = render_markdown(&doc)?;
```

### 3. Handle Code Languages

```rust
// Language is preserved
let code = Block::new(Content::code("python", "print('hello')"), Some("code"));

// Renders as:
// ```python
// print('hello')
// ```
```

### 4. Use Appropriate Heading Mode

```rust
// For documents with explicit roles
let renderer = MarkdownRenderer::new()
    .heading_mode(HeadingMode::Explicit);

// For documents derived from structure
let renderer = MarkdownRenderer::new()
    .heading_mode(HeadingMode::Structural);
```

## See Also

- [UCM Core Content Types](../ucm-core/content-types.md) - Content type reference
- [Metadata](../ucm-core/metadata.md) - Semantic roles
- [Documents](../ucm-core/documents.md) - Document structure
