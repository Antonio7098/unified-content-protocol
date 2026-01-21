//! HTML parser implementation.

use crate::error::{HtmlError, Result};
use scraper::{ElementRef, Html, Selector};
use ucm_core::{Block, BlockId, Content, Document, MediaSource};

/// Strategy for handling heading levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HeadingStrategy {
    /// Use actual heading levels from HTML (h1-h6)
    #[default]
    AsIs,
    /// Flatten all headings to a single level
    Flatten(usize),
    /// Infer hierarchy from DOM nesting depth
    InferFromNesting,
}

/// Configuration for HTML parsing
#[derive(Debug, Clone)]
pub struct HtmlParserConfig {
    /// Whether to preserve whitespace in text nodes
    pub preserve_whitespace: bool,
    /// Whether to extract images as media blocks
    pub extract_images: bool,
    /// Whether to extract links and store href in edges
    pub extract_links: bool,
    /// Strategy for handling heading levels
    pub heading_strategy: HeadingStrategy,
    /// Maximum nesting depth to process
    pub max_depth: usize,
    /// Maximum number of blocks to create
    pub max_blocks: usize,
    /// Minimum text length to create a block (filters noise)
    pub min_text_length: usize,
}

impl Default for HtmlParserConfig {
    fn default() -> Self {
        Self {
            preserve_whitespace: false,
            extract_images: true,
            extract_links: true,
            heading_strategy: HeadingStrategy::AsIs,
            max_depth: 50,
            max_blocks: 10000,
            min_text_length: 1,
        }
    }
}

/// HTML to UCM document parser
pub struct HtmlParser {
    config: HtmlParserConfig,
}

impl HtmlParser {
    /// Create a new parser with default configuration
    pub fn new() -> Self {
        Self {
            config: HtmlParserConfig::default(),
        }
    }

    /// Create a parser with custom configuration
    pub fn with_config(config: HtmlParserConfig) -> Self {
        Self { config }
    }

    /// Parse HTML string into a UCM Document
    pub fn parse(&self, html: &str) -> Result<Document> {
        let mut doc = Document::create();
        let root = doc.root;

        // Parse HTML
        let fragment = Html::parse_document(html);

        // Find body or use root element
        let body_selector = Selector::parse("body").unwrap();
        let body = fragment.select(&body_selector).next();

        if let Some(body_element) = body {
            self.process_children(&mut doc, &root, body_element, 0)?;
        } else {
            // No body tag, process entire document
            if let Some(root_element) = fragment.root_element().first_child() {
                if let Some(element) = ElementRef::wrap(root_element) {
                    self.process_children(&mut doc, &root, element, 0)?;
                }
            }
        }

        Ok(doc)
    }

    /// Process all children of an element
    fn process_children(
        &self,
        doc: &mut Document,
        parent_id: &BlockId,
        element: ElementRef,
        depth: usize,
    ) -> Result<()> {
        if depth > self.config.max_depth {
            return Err(HtmlError::ResourceLimit(format!(
                "Maximum nesting depth {} exceeded",
                self.config.max_depth
            )));
        }

        if doc.block_count() > self.config.max_blocks {
            return Err(HtmlError::ResourceLimit(format!(
                "Maximum block count {} exceeded",
                self.config.max_blocks
            )));
        }

        let mut current_heading_parent = *parent_id;
        let mut heading_stack: Vec<(usize, BlockId)> = vec![(0, *parent_id)];

        for child in element.children() {
            if let Some(child_element) = ElementRef::wrap(child) {
                let tag_name = child_element.value().name();

                // Handle headings specially for hierarchy
                if let Some(level) = self.parse_heading_level(tag_name) {
                    // Pop stack until we find a heading with lower level
                    while heading_stack.len() > 1
                        && heading_stack
                            .last()
                            .map(|(l, _)| *l >= level)
                            .unwrap_or(false)
                    {
                        heading_stack.pop();
                    }

                    let heading_parent = heading_stack
                        .last()
                        .map(|(_, id)| *id)
                        .unwrap_or(*parent_id);

                    let heading_id =
                        self.process_heading(doc, &heading_parent, child_element, level)?;

                    if let Some(id) = heading_id {
                        heading_stack.push((level, id));
                        current_heading_parent = id;
                    }
                } else {
                    // Non-heading elements go under current heading
                    self.process_element(doc, &current_heading_parent, child_element, depth + 1)?;
                }
            } else if let Some(text_node) = child.value().as_text() {
                let text = if self.config.preserve_whitespace {
                    text_node.to_string()
                } else {
                    text_node.trim().to_string()
                };

                if text.len() >= self.config.min_text_length {
                    let block = Block::new(Content::text(&text), Some("text"));
                    doc.add_block(block, &current_heading_parent)?;
                }
            }
        }

        Ok(())
    }

    /// Process a single HTML element
    fn process_element(
        &self,
        doc: &mut Document,
        parent_id: &BlockId,
        element: ElementRef,
        depth: usize,
    ) -> Result<Option<BlockId>> {
        if depth > self.config.max_depth {
            return Ok(None);
        }

        let tag_name = element.value().name();

        match tag_name {
            // Skip script, style, meta, etc.
            "script" | "style" | "meta" | "link" | "head" | "noscript" => Ok(None),

            // Headings (handled separately in process_children for hierarchy)
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                let level = self.parse_heading_level(tag_name).unwrap_or(1);
                self.process_heading(doc, parent_id, element, level)
            }

            // Paragraphs
            "p" => self.process_paragraph(doc, parent_id, element),

            // Lists
            "ul" | "ol" => self.process_list(doc, parent_id, element),

            // Code blocks
            "pre" => self.process_code_block(doc, parent_id, element),
            "code" => {
                // Inline code - treat as text
                let code_text = element.text().collect::<String>();
                if !code_text.trim().is_empty() {
                    let formatted = format!("`{}`", code_text);
                    let block = Block::new(Content::text(&formatted), Some("code"));
                    Ok(Some(doc.add_block(block, parent_id)?))
                } else {
                    Ok(None)
                }
            }

            // Blockquotes
            "blockquote" => self.process_blockquote(doc, parent_id, element),

            // Images
            "img" => self.process_image(doc, parent_id, element),

            // Links
            "a" => self.process_link(doc, parent_id, element),

            // Tables
            "table" => self.process_table(doc, parent_id, element),

            // Container elements - process children
            "div" | "section" | "article" | "main" | "aside" | "nav" | "header" | "footer"
            | "span" | "figure" | "figcaption" => {
                self.process_children(doc, parent_id, element, depth)?;
                Ok(None)
            }

            // Line breaks
            "br" | "hr" => Ok(None),

            // Default: try to extract text content
            _ => {
                let text = self.extract_text_content(element);
                if !text.is_empty() && text.len() >= self.config.min_text_length {
                    let block = Block::new(Content::text(&text), Some("text"));
                    Ok(Some(doc.add_block(block, parent_id)?))
                } else {
                    // Process children for unknown container elements
                    self.process_children(doc, parent_id, element, depth)?;
                    Ok(None)
                }
            }
        }
    }

    /// Process a heading element
    fn process_heading(
        &self,
        doc: &mut Document,
        parent_id: &BlockId,
        element: ElementRef,
        level: usize,
    ) -> Result<Option<BlockId>> {
        let text = self.extract_text_content(element);
        if text.is_empty() {
            return Ok(None);
        }

        let adjusted_level = match self.config.heading_strategy {
            HeadingStrategy::AsIs => level,
            HeadingStrategy::Flatten(target) => target,
            HeadingStrategy::InferFromNesting => level, // Could be enhanced
        };

        let role = format!("heading{}", adjusted_level.clamp(1, 6));
        let block = Block::new(Content::text(&text), Some(&role));
        let block_id = doc.add_block(block, parent_id)?;

        Ok(Some(block_id))
    }

    /// Process a paragraph element
    fn process_paragraph(
        &self,
        doc: &mut Document,
        parent_id: &BlockId,
        element: ElementRef,
    ) -> Result<Option<BlockId>> {
        let text = self.extract_formatted_text(element);
        if text.is_empty() || text.len() < self.config.min_text_length {
            return Ok(None);
        }

        let block = Block::new(Content::text(&text), Some("paragraph"));
        Ok(Some(doc.add_block(block, parent_id)?))
    }

    /// Process a list (ul/ol)
    fn process_list(
        &self,
        doc: &mut Document,
        parent_id: &BlockId,
        element: ElementRef,
    ) -> Result<Option<BlockId>> {
        let li_selector = Selector::parse("li").unwrap();
        let items: Vec<String> = element
            .select(&li_selector)
            .map(|li| self.extract_formatted_text(li))
            .filter(|s| !s.is_empty())
            .collect();

        if items.is_empty() {
            return Ok(None);
        }

        let list_content = items.join("\n");
        let block = Block::new(Content::text(&list_content), Some("list"));
        Ok(Some(doc.add_block(block, parent_id)?))
    }

    /// Process a code block (pre/code)
    fn process_code_block(
        &self,
        doc: &mut Document,
        parent_id: &BlockId,
        element: ElementRef,
    ) -> Result<Option<BlockId>> {
        let code_selector = Selector::parse("code").unwrap();
        let code_element = element.select(&code_selector).next().unwrap_or(element);

        let code_text = code_element.text().collect::<String>();
        if code_text.trim().is_empty() {
            return Ok(None);
        }

        // Try to extract language from class
        let language = code_element
            .value()
            .attr("class")
            .and_then(|class| {
                class
                    .split_whitespace()
                    .find(|c| c.starts_with("language-") || c.starts_with("lang-"))
                    .map(|c| {
                        c.trim_start_matches("language-")
                            .trim_start_matches("lang-")
                    })
            })
            .unwrap_or("text");

        let block = Block::new(Content::code(language, &code_text), Some("code"));
        Ok(Some(doc.add_block(block, parent_id)?))
    }

    /// Process a blockquote
    fn process_blockquote(
        &self,
        doc: &mut Document,
        parent_id: &BlockId,
        element: ElementRef,
    ) -> Result<Option<BlockId>> {
        let text = self.extract_formatted_text(element);
        if text.is_empty() {
            return Ok(None);
        }

        let block = Block::new(Content::text(&text), Some("quote"));
        Ok(Some(doc.add_block(block, parent_id)?))
    }

    /// Process an image element
    fn process_image(
        &self,
        doc: &mut Document,
        parent_id: &BlockId,
        element: ElementRef,
    ) -> Result<Option<BlockId>> {
        if !self.config.extract_images {
            return Ok(None);
        }

        let src = element.value().attr("src").unwrap_or("");
        let alt = element.value().attr("alt").unwrap_or("");

        if src.is_empty() {
            return Ok(None);
        }

        // Create media content
        let media_source = if src.starts_with("data:") {
            // Base64 encoded image
            let base64_data = src.split(',').nth(1).unwrap_or("").to_string();
            MediaSource::Base64(base64_data)
        } else {
            MediaSource::Url(src.to_string())
        };

        let media = ucm_core::Media::image(media_source).with_alt(alt);
        let block = Block::new(Content::Media(media), Some("image"));
        Ok(Some(doc.add_block(block, parent_id)?))
    }

    /// Process a link element
    fn process_link(
        &self,
        doc: &mut Document,
        parent_id: &BlockId,
        element: ElementRef,
    ) -> Result<Option<BlockId>> {
        let text = self.extract_text_content(element);
        let href = element.value().attr("href").unwrap_or("");

        if text.is_empty() {
            return Ok(None);
        }

        if self.config.extract_links && !href.is_empty() {
            // Create link in markdown format
            let link_text = format!("[{}]({})", text, href);
            let block = Block::new(Content::text(&link_text), Some("link"));
            Ok(Some(doc.add_block(block, parent_id)?))
        } else {
            // Just extract as text
            let block = Block::new(Content::text(&text), Some("text"));
            Ok(Some(doc.add_block(block, parent_id)?))
        }
    }

    /// Process a table element
    fn process_table(
        &self,
        doc: &mut Document,
        parent_id: &BlockId,
        element: ElementRef,
    ) -> Result<Option<BlockId>> {
        let row_selector = Selector::parse("tr").unwrap();
        let cell_selector = Selector::parse("td, th").unwrap();

        let rows: Vec<Vec<String>> = element
            .select(&row_selector)
            .map(|row| {
                row.select(&cell_selector)
                    .map(|cell| self.extract_text_content(cell))
                    .collect()
            })
            .filter(|row: &Vec<String>| !row.is_empty())
            .collect();

        if rows.is_empty() {
            return Ok(None);
        }

        let block = Block::new(Content::table(rows), Some("table"));
        Ok(Some(doc.add_block(block, parent_id)?))
    }

    /// Parse heading level from tag name
    fn parse_heading_level(&self, tag_name: &str) -> Option<usize> {
        match tag_name {
            "h1" => Some(1),
            "h2" => Some(2),
            "h3" => Some(3),
            "h4" => Some(4),
            "h5" => Some(5),
            "h6" => Some(6),
            _ => None,
        }
    }

    /// Extract plain text content from an element
    fn extract_text_content(&self, element: ElementRef) -> String {
        let text: String = element.text().collect();
        if self.config.preserve_whitespace {
            text
        } else {
            // Normalize whitespace
            text.split_whitespace().collect::<Vec<_>>().join(" ")
        }
    }

    /// Extract text with some formatting preserved (bold, italic, etc.)
    fn extract_formatted_text(&self, element: ElementRef) -> String {
        let mut result = String::new();

        for child in element.children() {
            if let Some(child_element) = ElementRef::wrap(child) {
                let tag_name = child_element.value().name();
                let child_text = self.extract_formatted_text(child_element);

                match tag_name {
                    "strong" | "b" => {
                        result.push_str("**");
                        result.push_str(&child_text);
                        result.push_str("**");
                    }
                    "em" | "i" => {
                        result.push('*');
                        result.push_str(&child_text);
                        result.push('*');
                    }
                    "code" => {
                        result.push('`');
                        result.push_str(&child_text);
                        result.push('`');
                    }
                    "a" if self.config.extract_links => {
                        let href = child_element.value().attr("href").unwrap_or("");
                        if !href.is_empty() {
                            result.push_str(&format!("[{}]({})", child_text, href));
                        } else {
                            result.push_str(&child_text);
                        }
                    }
                    "br" => {
                        result.push('\n');
                    }
                    _ => {
                        result.push_str(&child_text);
                    }
                }
            } else if let Some(text_node) = child.value().as_text() {
                let text = if self.config.preserve_whitespace {
                    text_node.to_string()
                } else {
                    text_node.split_whitespace().collect::<Vec<_>>().join(" ")
                };
                result.push_str(&text);
            }
        }

        result.trim().to_string()
    }
}

impl Default for HtmlParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading_hierarchy() {
        let html = r#"<html><body>
            <h1>Main</h1>
            <p>Intro</p>
            <h2>Sub 1</h2>
            <p>Content 1</p>
            <h2>Sub 2</h2>
            <p>Content 2</p>
        </body></html>"#;

        let doc = HtmlParser::new().parse(html).unwrap();

        // Verify structure
        let root_children = doc.children(&doc.root);
        assert!(!root_children.is_empty());
    }

    #[test]
    fn test_code_language_extraction() {
        let html = r#"<pre><code class="language-rust">fn main() {}</code></pre>"#;
        let doc = HtmlParser::new().parse(html).unwrap();

        // Should have extracted the code block
        assert!(doc.block_count() >= 2);
    }

    #[test]
    fn test_max_depth_limit() {
        let config = HtmlParserConfig {
            max_depth: 2,
            ..Default::default()
        };
        let parser = HtmlParser::with_config(config);

        // Deeply nested HTML
        let html = "<div><div><div><div><div><p>Deep</p></div></div></div></div></div>";
        let result = parser.parse(html);

        // Should handle gracefully (either succeed with truncation or error)
        // The important thing is it doesn't stack overflow
        assert!(result.is_ok() || matches!(result, Err(HtmlError::ResourceLimit(_))));
    }

    #[test]
    fn test_heading_strategy_flatten() {
        let config = HtmlParserConfig {
            heading_strategy: HeadingStrategy::Flatten(3),
            ..Default::default()
        };
        let parser = HtmlParser::with_config(config);

        let html = "<h1>Title</h1><h2>Subtitle</h2>";
        let doc = parser.parse(html).unwrap();

        // All headings should be flattened to h3
        for block in doc.blocks.values() {
            if let Some(ref role) = block.metadata.semantic_role {
                if role.category.as_str().starts_with("heading") {
                    assert_eq!(role.category.as_str(), "heading3");
                }
            }
        }
    }
}
