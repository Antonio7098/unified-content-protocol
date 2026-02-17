# HTML Translator

**ucp-translator-html** provides conversion from HTML to UCM documents.

## Overview

The HTML translator enables:

- **Parsing** - Convert HTML to UCM documents
- **Semantic mapping** - Map HTML elements to semantic roles
- **Heading strategies** - Configure how headings are processed

## Installation

```toml
[dependencies]
ucp-translator-html = "0.1"
```

## Quick Start

```rust
use ucp_translator_html::parse_html;

let html = r#"
<!DOCTYPE html>
<html>
<head><title>My Document</title></head>
<body>
  <h1>Introduction</h1>
  <p>Welcome to the guide.</p>
  <h2>Getting Started</h2>
  <p>Here's some content.</p>
</body>
</html>
"#;

let doc = parse_html(html).unwrap();
println!("Parsed {} blocks", doc.block_count());
```

## Parsing HTML

### HtmlParser

```rust
use ucp_translator_html::{HtmlParser, HtmlParserConfig, HeadingStrategy};

// Default parser
let parser = HtmlParser::new();
let doc = parser.parse(html)?;

// With custom configuration
let config = HtmlParserConfig {
    heading_strategy: HeadingStrategy::FromHierarchy,
    ..Default::default()
};
let parser = HtmlParser::with_config(config);
let doc = parser.parse(html)?;
```

### Supported Elements

| HTML Element | UCM Content Type | Semantic Role |
|--------------|------------------|---------------|
| `<h1>` | Text | `heading1` |
| `<h2>` | Text | `heading2` |
| `<h3>` | Text | `heading3` |
| `<h4>` | Text | `heading4` |
| `<h5>` | Text | `heading5` |
| `<h6>` | Text | `heading6` |
| `<p>` | Text | `paragraph` |
| `<pre><code>` | Code | `code` |
| `<ul>/<ol>` | Text | `list` |
| `<blockquote>` | Text | `quote` |
| `<table>` | Table | `table` |

### Heading Strategies

| Strategy | Description |
|----------|-------------|
| `FromTags` | Use HTML heading tags directly (h1-h6) |
| `FromHierarchy` | Derive heading level from document structure |

## Public API

```rust
pub use error::{HtmlError, Result};
pub use parser::{HtmlParser, HtmlParserConfig, HeadingStrategy};
pub use parse_html;
```

## See Also

- [UCM Core Content Types](../../docs/ucm-core/content-types.md) - Content type reference
- [Markdown Translator](../markdown) - Markdown support
- [Documents](../../docs/ucm-core/documents.md) - Document structure
