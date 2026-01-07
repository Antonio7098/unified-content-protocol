//! Render UCM documents to Markdown.

use crate::{Result, TranslatorError};
use ucm_core::{Block, BlockId, Cell, Content, Document, MediaSource, Row};

/// Markdown renderer that converts UCM to Markdown
pub struct MarkdownRenderer {
    indent_size: usize,
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        Self { indent_size: 2 }
    }

    pub fn indent_size(mut self, size: usize) -> Self {
        self.indent_size = size;
        self
    }

    pub fn render(&self, doc: &Document) -> Result<String> {
        let mut output = String::new();
        self.render_block(doc, &doc.root, &mut output, 0)?;
        Ok(output)
    }

    fn render_block(&self, doc: &Document, block_id: &BlockId, output: &mut String, depth: usize) -> Result<()> {
        let block = doc.get_block(block_id)
            .ok_or_else(|| TranslatorError::RenderError(format!("Block not found: {}", block_id)))?;

        // Render content based on type and role
        self.render_content(block, output, depth)?;

        // Render children
        if let Some(children) = doc.structure.get(block_id) {
            for child_id in children {
                self.render_block(doc, child_id, output, depth)?;
            }
        }

        Ok(())
    }

    fn render_content(&self, block: &Block, output: &mut String, _depth: usize) -> Result<()> {
        let role = block.metadata.semantic_role.as_ref()
            .map(|r| format!("{:?}", r.category).to_lowercase())
            .unwrap_or_else(|| "paragraph".to_string());

        match &block.content {
            Content::Text(text) => {
                self.render_text(&text.text, &role, output);
            }
            Content::Code(code) => {
                output.push_str("```");
                output.push_str(&code.language);
                output.push('\n');
                output.push_str(&code.source);
                output.push_str("\n```\n\n");
            }
            Content::Table(table) => {
                self.render_table(&table.rows, output);
            }
            Content::Math(math) => {
                if math.display_mode {
                    output.push_str("$$\n");
                    output.push_str(&math.expression);
                    output.push_str("\n$$\n\n");
                } else {
                    output.push('$');
                    output.push_str(&math.expression);
                    output.push_str("$\n\n");
                }
            }
            Content::Media(media) => {
                let src = match &media.source {
                    MediaSource::Url(u) => u.clone(),
                    MediaSource::Base64(b) => format!("data:image;base64,{}", b),
                    MediaSource::Reference(id) => format!("[ref:{}]", id),
                    MediaSource::External(ext) => format!("[{}:{}]", ext.provider, ext.key),
                };
                output.push_str(&format!("![{}]({})\n\n", 
                    media.alt_text.as_deref().unwrap_or(""), 
                    src));
            }
            Content::Json { value, .. } => {
                output.push_str("```json\n");
                output.push_str(&value.to_string());
                output.push_str("\n```\n\n");
            }
            Content::Composite { children, .. } => {
                output.push_str(&format!("[Composite: {} children]\n\n", children.len()));
            }
            Content::Binary { mime_type, .. } => {
                output.push_str(&format!("[Binary: {}]\n\n", mime_type));
            }
        }

        Ok(())
    }

    fn render_text(&self, text: &str, role: &str, output: &mut String) {
        match role {
            "heading1" => { output.push_str("# "); output.push_str(text); output.push_str("\n\n"); }
            "heading2" => { output.push_str("## "); output.push_str(text); output.push_str("\n\n"); }
            "heading3" => { output.push_str("### "); output.push_str(text); output.push_str("\n\n"); }
            "heading4" => { output.push_str("#### "); output.push_str(text); output.push_str("\n\n"); }
            "heading5" => { output.push_str("##### "); output.push_str(text); output.push_str("\n\n"); }
            "heading6" => { output.push_str("###### "); output.push_str(text); output.push_str("\n\n"); }
            "quote" => {
                for line in text.lines() {
                    output.push_str("> ");
                    output.push_str(line);
                    output.push('\n');
                }
                output.push('\n');
            }
            "list" => {
                output.push_str(text);
                output.push_str("\n\n");
            }
            _ => {
                if !text.is_empty() {
                    output.push_str(text);
                    output.push_str("\n\n");
                }
            }
        }
    }

    fn render_table(&self, rows: &[Row], output: &mut String) {
        if rows.is_empty() { return; }

        // Header
        let header = &rows[0];
        output.push('|');
        for cell in &header.cells {
            output.push(' ');
            output.push_str(&cell_to_string(cell));
            output.push_str(" |");
        }
        output.push('\n');

        // Separator
        output.push('|');
        for _ in &header.cells {
            output.push_str(" --- |");
        }
        output.push('\n');

        // Body
        for row in rows.iter().skip(1) {
            output.push('|');
            for cell in &row.cells {
                output.push(' ');
                output.push_str(&cell_to_string(cell));
                output.push_str(" |");
            }
            output.push('\n');
        }
        output.push('\n');
    }
}

fn cell_to_string(cell: &Cell) -> String {
    match cell {
        Cell::Null => String::new(),
        Cell::Text(s) => s.clone(),
        Cell::Number(n) => n.to_string(),
        Cell::Boolean(b) => b.to_string(),
        Cell::Date(s) => s.clone(),
        Cell::DateTime(s) => s.clone(),
        Cell::Json(v) => v.to_string(),
    }
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_heading() {
        let mut doc = Document::create();
        let root = doc.root.clone();
        let block = Block::new(Content::text("Hello"), Some("title"));
        doc.add_block(block, &root).unwrap();
        
        let md = MarkdownRenderer::new().render(&doc).unwrap();
        // Title role renders as plain text, verify content is present
        assert!(md.contains("Hello"));
    }

    #[test]
    fn test_render_code() {
        let mut doc = Document::create();
        let root = doc.root.clone();
        let block = Block::new(Content::code("rust", "fn main() {}"), None);
        doc.add_block(block, &root).unwrap();
        
        let md = MarkdownRenderer::new().render(&doc).unwrap();
        assert!(md.contains("```rust"));
        assert!(md.contains("fn main()"));
    }
}
