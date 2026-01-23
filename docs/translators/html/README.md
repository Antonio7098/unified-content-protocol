# HTML Translator

The **ucp-translator-html** crate converts HTML documents into the Unified Content Model (UCM).

## Capabilities

- **HTML parsing** using `html5ever` and `scraper`
- **Semantic mapping** for headings, lists, emphasis, tables, code blocks, media, and links
- **Metadata extraction** (ids, classes, aria labels, rel attributes)
- **Content normalization** (whitespace trimming, entity decoding)
- **Selective traversal** via CSS selectors or whitelist/blacklist policies

## Installation

=== "Rust"
    ```toml
    [dependencies]
    ucp-translator-html = "0.1.3"
    ```

=== "Python"
    ```bash
    pip install ucp-content
    ```

=== "JavaScript"
    ```bash
    npm install ucp-content
    ```

## Quick Start

=== "Rust"
    ```rust
    use ucp_translator_html::HtmlParser;

    let html = r#"
    <!doctype html>
    <html>
      <body>
        <h1>Intro</h1>
        <p>Hello <strong>HTML</strong>!</p>
      </body>
    </html>
    "#;

    let parser = HtmlParser::default();
    let doc = parser.parse(html)?;
    println!("Parsed {} blocks", doc.block_count());
    ```

=== "Python"
    ```python
    import ucp

    html = '''
    <!doctype html>
    <html>
      <body>
        <h1>Intro</h1>
        <p>Hello <strong>HTML</strong>!</p>
      </body>
    </html>
    '''

    doc = ucp.parse_html(html)
    print(f"Parsed {doc.block_count} blocks")
    ```

=== "JavaScript"
    ```javascript
    import { parseHtml } from 'ucp-content';

    const html = `
    <!doctype html>
    <html>
      <body>
        <h1>Intro</h1>
        <p>Hello <strong>HTML</strong>!</p>
      </body>
    </html>
    `;

    const doc = parseHtml(html);
    console.log(`Parsed ${doc.blockCount()} blocks`);
    ```

## Parser Configuration (Rust Only)

=== "Rust"
    ```rust
    use ucp_translator_html::{HtmlParser, ParseConfig};

    let parser = HtmlParser::new(ParseConfig {
        preserve_whitespace: false,
        max_depth: Some(12),
        allowed_nodes: None,
        denied_nodes: Some(vec!["script", "style"]),
        capture_attributes: true,
    });
    ```

### Key Options

| Option | Description |
| ------ | ----------- |
| `preserve_whitespace` | Keep raw whitespace blocks instead of collapsing |
| `max_depth` | Stop traversal after a depth threshold |
| `allowed_nodes` / `denied_nodes` | Filter elements by tag name or CSS selector |
| `capture_attributes` | Persist `id`, `class`, `href`, `src`, `data-*`, `aria-*` |
| `base_heading_level` | Offset heading levels when integrating into existing sections |

## Semantic Mapping

| HTML | UCM Role |
| ---- | -------- |
| `<h1>..</h1>` | `heading1` |
| `<h2>..</h2>` | `heading2` |
| `<p>` | `paragraph` |
| `<a>` | `link` |
| `<blockquote>` | `quote` |
| `<code>` / `<pre>` | `code` |
| `<ul>/<ol>` | `list` |
| `<figure>` | `media` |
| `<table>` | `table` |

## Error Handling

=== "Rust"
    ```rust
    match parser.parse(input) {
        Ok(doc) => println!("blocks: {}", doc.block_count()),
        Err(e) => eprintln!("HTML parse error: {e}")
    }
    ```

All errors use the crate-specific `HtmlError` enum, which includes:

- `ParseError { line, column, message }`
- `UnsupportedNode(String)`
- `DepthExceeded { max_depth }`
- `Io(std::io::Error)`

## Testing

The crate ships with property tests (`cargo test -p ucp-translator-html`) that fuzz real-world HTML snippets to ensure parsing stability. Add additional fixtures under `crates/translators/html/tests/fixtures` and reference them in the test module.

## See Also

- [Markdown Translator](../markdown/README.md)
- [UCM Core Content Types](../../ucm-core/content-types.md)
- [Section Writing APIs](../../ucm-engine/operations.md#write_section)
