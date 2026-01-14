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
ucp-translator-markdown = "0.1"
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

## Public API

```rust
pub use parser::MarkdownParser;
pub use renderer::{MarkdownRenderer, HeadingMode};
pub use {parse_markdown, render_markdown};
```

## See Also

- [UCM Core Content Types](../../docs/ucm-core/content-types.md) - Content type reference
- [Metadata](../../docs/ucm-core/metadata.md) - Semantic roles
- [Documents](../../docs/ucm-core/documents.md) - Document structure
