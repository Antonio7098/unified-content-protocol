//! Golden tests for markdown translator.
//!
//! These tests verify that markdown parsing and rendering produces
//! consistent, expected output across versions.

use ucp_translator_markdown::{parse_markdown, render_markdown};

const SIMPLE_MD: &str = include_str!("fixtures/simple.md");
const COMPLEX_MD: &str = include_str!("fixtures/complex.md");

#[test]
fn test_simple_roundtrip() {
    let doc = parse_markdown(SIMPLE_MD).expect("Failed to parse simple.md");
    let rendered = render_markdown(&doc).expect("Failed to render");

    // Verify roundtrip preserves content
    assert!(rendered.contains("Hello"), "Title should be preserved");
    assert!(
        rendered.contains("paragraph"),
        "Paragraph should be preserved"
    );
}

#[test]
fn test_simple_exact_roundtrip() {
    let doc = parse_markdown(SIMPLE_MD).expect("Failed to parse simple.md");
    let rendered = render_markdown(&doc).expect("Failed to render");

    // Exact roundtrip match
    assert_eq!(
        SIMPLE_MD, rendered,
        "Simple markdown should roundtrip exactly"
    );
}

#[test]
fn test_complex_structure() {
    let doc = parse_markdown(COMPLEX_MD).expect("Failed to parse complex.md");

    // Verify document structure
    assert!(doc.block_count() > 1, "Should have multiple blocks");

    // Verify root has children
    let children = doc.children(&doc.root);
    assert!(!children.is_empty(), "Root should have children");
}

#[test]
fn test_complex_content_preservation() {
    let doc = parse_markdown(COMPLEX_MD).expect("Failed to parse complex.md");
    let rendered = render_markdown(&doc).expect("Failed to render");

    // Verify key content is preserved
    assert!(
        rendered.contains("Document Title"),
        "Title should be preserved"
    );
    assert!(rendered.contains("Section One"), "H2 should be preserved");
    assert!(
        rendered.contains("Subsection 1.1"),
        "H3 should be preserved"
    );
    assert!(rendered.contains("println!"), "Code should be preserved");
    assert!(
        rendered.contains("blockquote"),
        "Quote content should be preserved"
    );
}

#[test]
fn test_heading_hierarchy() {
    let md = "# H1\n\n## H2\n\n### H3\n\n## Another H2\n";
    let doc = parse_markdown(md).expect("Failed to parse");

    // Verify structure: H1 -> H2 -> H3, H1 -> H2
    let root_children = doc.children(&doc.root);
    assert_eq!(root_children.len(), 1, "Root should have one H1 child");
}

#[test]
fn test_code_block_language() {
    let md = "# Test\n\n```rust\nlet x = 1;\n```\n";
    let doc = parse_markdown(md).expect("Failed to parse");
    let rendered = render_markdown(&doc).expect("Failed to render");

    assert!(
        rendered.contains("let x = 1"),
        "Code content should be preserved"
    );
}

#[test]
fn test_empty_document() {
    let md = "";
    let doc = parse_markdown(md).expect("Failed to parse empty");

    // Should have at least root block
    assert!(doc.block_count() >= 1, "Should have root block");
}

#[test]
fn test_only_paragraph() {
    let md = "Just a paragraph.\n";
    let doc = parse_markdown(md).expect("Failed to parse");
    let rendered = render_markdown(&doc).expect("Failed to render");

    assert!(
        rendered.contains("Just a paragraph"),
        "Paragraph should be preserved"
    );
}

#[test]
fn test_nested_headings() {
    let md = r#"# Level 1

Content under 1.

## Level 2

Content under 2.

### Level 3

Content under 3.

#### Level 4

Deep content.
"#;
    let doc = parse_markdown(md).expect("Failed to parse");
    let rendered = render_markdown(&doc).expect("Failed to render");

    assert!(rendered.contains("Level 1"), "H1 preserved");
    assert!(rendered.contains("Level 2"), "H2 preserved");
    assert!(rendered.contains("Level 3"), "H3 preserved");
    assert!(rendered.contains("Level 4"), "H4 preserved");
}

#[test]
fn test_multiple_paragraphs() {
    let md = r#"# Title

First paragraph.

Second paragraph.

Third paragraph.
"#;
    let doc = parse_markdown(md).expect("Failed to parse");
    let rendered = render_markdown(&doc).expect("Failed to render");

    assert!(rendered.contains("First paragraph"), "First para preserved");
    assert!(
        rendered.contains("Second paragraph"),
        "Second para preserved"
    );
    assert!(rendered.contains("Third paragraph"), "Third para preserved");
}

#[test]
fn test_unicode_content() {
    let md = "# 日本語タイトル\n\nこれは日本語のテキストです。\n";
    let doc = parse_markdown(md).expect("Failed to parse unicode");
    let rendered = render_markdown(&doc).expect("Failed to render");

    assert!(
        rendered.contains("日本語タイトル"),
        "Unicode title preserved"
    );
    assert!(
        rendered.contains("日本語のテキスト"),
        "Unicode content preserved"
    );
}

#[test]
fn test_special_characters() {
    let md = "# Test <>&\n\nContent with <html> & special chars.\n";
    let doc = parse_markdown(md).expect("Failed to parse special chars");
    let rendered = render_markdown(&doc).expect("Failed to render");

    assert!(rendered.contains("<html>"), "HTML-like content preserved");
    assert!(rendered.contains("&"), "Ampersand preserved");
}
