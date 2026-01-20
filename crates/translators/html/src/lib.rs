//! HTML to UCM document translator.
//!
//! This crate provides translation from HTML documents to UCM's block-based
//! document model. It extracts semantic structure from HTML elements and
//! creates appropriate blocks with proper hierarchy.
//!
//! # Example
//!
//! ```
//! use ucp_translator_html::{HtmlParser, HtmlParserConfig};
//!
//! let html = r#"<html><body>
//!     <h1>Title</h1>
//!     <p>Some content here.</p>
//! </body></html>"#;
//!
//! let parser = HtmlParser::new();
//! let doc = parser.parse(html).unwrap();
//! ```

mod error;
mod parser;

pub use error::{HtmlError, Result};
pub use parser::{HeadingStrategy, HtmlParser, HtmlParserConfig};

/// Parse HTML string into a UCM Document.
///
/// This is a convenience function that uses default configuration.
pub fn parse_html(html: &str) -> Result<ucm_core::Document> {
    HtmlParser::new().parse(html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_html() {
        let html = r#"<html><body>
            <h1>Hello World</h1>
            <p>This is a paragraph.</p>
        </body></html>"#;

        let doc = parse_html(html).unwrap();
        assert!(doc.block_count() > 1);
    }

    #[test]
    fn test_parse_nested_structure() {
        let html = r#"<html><body>
            <h1>Main Title</h1>
            <p>Intro paragraph</p>
            <h2>Section 1</h2>
            <p>Section 1 content</p>
            <h2>Section 2</h2>
            <p>Section 2 content</p>
        </body></html>"#;

        let doc = parse_html(html).unwrap();

        // Should have root + h1 + h2 + h2 + paragraphs
        assert!(doc.block_count() >= 5);
    }

    #[test]
    fn test_parse_with_links() {
        let html = r#"<html><body>
            <p>Check out <a href="https://example.com">this link</a>!</p>
        </body></html>"#;

        let doc = parse_html(html).unwrap();
        assert!(doc.block_count() >= 2);
    }

    #[test]
    fn test_parse_with_images() {
        let html = r#"<html><body>
            <h1>Gallery</h1>
            <img src="https://example.com/image.jpg" alt="Test image">
        </body></html>"#;

        let config = HtmlParserConfig {
            extract_images: true,
            ..Default::default()
        };
        let parser = HtmlParser::with_config(config);
        let doc = parser.parse(html).unwrap();

        assert!(doc.block_count() >= 2);
    }

    #[test]
    fn test_parse_code_blocks() {
        let html = r#"<html><body>
            <pre><code class="language-rust">fn main() {
    println!("Hello");
}</code></pre>
        </body></html>"#;

        let doc = parse_html(html).unwrap();
        assert!(doc.block_count() >= 2);
    }

    #[test]
    fn test_parse_lists() {
        let html = r#"<html><body>
            <ul>
                <li>Item 1</li>
                <li>Item 2</li>
                <li>Item 3</li>
            </ul>
        </body></html>"#;

        let doc = parse_html(html).unwrap();
        assert!(doc.block_count() >= 2);
    }

    #[test]
    fn test_parse_tables() {
        let html = r#"<html><body>
            <table>
                <tr><th>Name</th><th>Age</th></tr>
                <tr><td>Alice</td><td>30</td></tr>
                <tr><td>Bob</td><td>25</td></tr>
            </table>
        </body></html>"#;

        let doc = parse_html(html).unwrap();
        assert!(doc.block_count() >= 2);
    }

    #[test]
    fn test_empty_html() {
        let html = "<html><body></body></html>";
        let doc = parse_html(html).unwrap();
        assert_eq!(doc.block_count(), 1); // Just root
    }

    #[test]
    fn test_malformed_html() {
        // Should handle malformed HTML gracefully
        let html = "<p>Unclosed paragraph <b>bold";
        let result = parse_html(html);
        // Should not panic, may succeed with partial parsing
        assert!(result.is_ok());
    }
}
