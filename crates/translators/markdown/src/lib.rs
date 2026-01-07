//! Markdown translator for UCP.
//!
//! Converts between Markdown and UCM documents.

mod from_markdown;
mod to_markdown;

pub use from_markdown::MarkdownParser;
pub use to_markdown::MarkdownRenderer;

use thiserror::Error;
use ucm_core::Document;

#[derive(Debug, Error)]
pub enum TranslatorError {
    #[error("Parse error at line {line}: {message}")]
    ParseError { line: usize, message: String },
    #[error("Render error: {0}")]
    RenderError(String),
    #[error("Invalid structure: {0}")]
    InvalidStructure(String),
}

pub type Result<T> = std::result::Result<T, TranslatorError>;

/// Parse markdown into a UCM document
pub fn parse_markdown(markdown: &str) -> Result<Document> {
    MarkdownParser::new().parse(markdown)
}

/// Render a UCM document to markdown
pub fn render_markdown(doc: &Document) -> Result<String> {
    MarkdownRenderer::new().render(doc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_simple() {
        let md = "# Hello\n\nThis is a paragraph.\n";
        let doc = parse_markdown(md).unwrap();
        let rendered = render_markdown(&doc).unwrap();
        assert!(rendered.contains("Hello"));
        assert!(rendered.contains("paragraph"));
    }

    #[test]
    fn test_roundtrip_exact() {
        let md = "# Hello\n\nThis is a paragraph.\n";
        let doc = parse_markdown(md).unwrap();
        let rendered = render_markdown(&doc).unwrap();
        assert_eq!(md, rendered);
    }
}
