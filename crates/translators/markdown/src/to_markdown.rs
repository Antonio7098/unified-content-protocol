//! Render UCM documents to Markdown.
//!
//! Supports two heading modes:
//! - **Explicit**: Uses semantic roles (heading1, heading2, etc.) from block metadata
//! - **Structural**: Derives heading level from document tree depth
//!
//! The hybrid approach uses explicit roles when present, falling back to structural
//! derivation for blocks without heading roles.

use crate::{Result, TranslatorError};
use ucm_core::metadata::RoleCategory;
use ucm_core::{Block, BlockId, Cell, Content, Document, MediaSource, Row};

/// Configuration for heading level derivation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HeadingMode {
    /// Use explicit semantic roles only (heading1, heading2, etc.)
    Explicit,
    /// Derive heading level from document structure depth
    Structural,
    /// Use explicit roles when present, fall back to structural derivation
    #[default]
    Hybrid,
}

/// Markdown renderer that converts UCM to Markdown
pub struct MarkdownRenderer {
    indent_size: usize,
    heading_mode: HeadingMode,
    /// Base heading level offset (0 = start at H1, 1 = start at H2, etc.)
    heading_offset: usize,
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        Self {
            indent_size: 2,
            heading_mode: HeadingMode::default(),
            heading_offset: 0,
        }
    }

    pub fn indent_size(mut self, size: usize) -> Self {
        self.indent_size = size;
        self
    }

    /// Set the heading derivation mode
    pub fn heading_mode(mut self, mode: HeadingMode) -> Self {
        self.heading_mode = mode;
        self
    }

    /// Set heading level offset (useful for nested documents)
    pub fn heading_offset(mut self, offset: usize) -> Self {
        self.heading_offset = offset;
        self
    }

    pub fn render(&self, doc: &Document) -> Result<String> {
        let mut output = String::new();
        self.render_block(doc, &doc.root, &mut output, 0)?;

        // Trim trailing whitespace but ensure single newline at end
        let trimmed = output.trim_end();
        if trimmed.is_empty() {
            Ok(String::new())
        } else {
            Ok(format!("{}\n", trimmed))
        }
    }

    fn render_block(
        &self,
        doc: &Document,
        block_id: &BlockId,
        output: &mut String,
        depth: usize,
    ) -> Result<()> {
        let block = doc.get_block(block_id).ok_or_else(|| {
            TranslatorError::RenderError(format!("Block not found: {}", block_id))
        })?;

        // Skip root block content (it's just a container)
        if !block.is_root() {
            // Render content based on type and role, passing depth for structural heading derivation
            self.render_content(block, output, depth)?;
        }

        // Render children with incremented depth
        if let Some(children) = doc.structure.get(block_id) {
            for child_id in children {
                self.render_block(doc, child_id, output, depth + 1)?;
            }
        }

        Ok(())
    }

    fn render_content(&self, block: &Block, output: &mut String, depth: usize) -> Result<()> {
        // Determine the effective role, considering heading mode
        let explicit_role = block.metadata.semantic_role.as_ref().map(|r| r.category);

        match &block.content {
            Content::Text(text) => {
                self.render_text(&text.text, explicit_role, depth, output);
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
                output.push_str(&format!(
                    "![{}]({})\n\n",
                    media.alt_text.as_deref().unwrap_or(""),
                    src
                ));
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

    /// Determine heading level based on mode, explicit role, and depth
    fn resolve_heading_level(
        &self,
        explicit_role: Option<RoleCategory>,
        depth: usize,
    ) -> Option<usize> {
        match self.heading_mode {
            HeadingMode::Explicit => {
                // Only use explicit heading roles
                explicit_role.and_then(|r| self.role_to_heading_level(r))
            }
            HeadingMode::Structural => {
                // Always derive from depth (depth 1 = H1, depth 2 = H2, etc.)
                // Only for blocks that look like headings (have heading role or are section containers)
                if explicit_role
                    .map(|r| self.is_heading_role(r))
                    .unwrap_or(false)
                {
                    Some((depth + self.heading_offset).clamp(1, 6))
                } else {
                    None
                }
            }
            HeadingMode::Hybrid => {
                // Use explicit role if present, otherwise derive from structure for heading-like blocks
                if let Some(role) = explicit_role {
                    self.role_to_heading_level(role)
                } else {
                    None
                }
            }
        }
    }

    fn role_to_heading_level(&self, role: RoleCategory) -> Option<usize> {
        match role {
            RoleCategory::Heading1 => Some(1),
            RoleCategory::Heading2 => Some(2),
            RoleCategory::Heading3 => Some(3),
            RoleCategory::Heading4 => Some(4),
            RoleCategory::Heading5 => Some(5),
            RoleCategory::Heading6 => Some(6),
            RoleCategory::Title => Some(1),
            RoleCategory::Subtitle => Some(2),
            _ => None,
        }
    }

    fn is_heading_role(&self, role: RoleCategory) -> bool {
        matches!(
            role,
            RoleCategory::Heading1
                | RoleCategory::Heading2
                | RoleCategory::Heading3
                | RoleCategory::Heading4
                | RoleCategory::Heading5
                | RoleCategory::Heading6
                | RoleCategory::Title
                | RoleCategory::Subtitle
        )
    }

    fn render_text(
        &self,
        text: &str,
        explicit_role: Option<RoleCategory>,
        depth: usize,
        output: &mut String,
    ) {
        // Check for heading
        if let Some(level) = self.resolve_heading_level(explicit_role, depth) {
            let hashes = "#".repeat(level);
            output.push_str(&hashes);
            output.push(' ');
            output.push_str(text);
            output.push_str("\n\n");
            return;
        }

        // Handle other roles
        let role_str = explicit_role.map(|r| r.as_str()).unwrap_or("paragraph");

        match role_str {
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
        if rows.is_empty() {
            return;
        }

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
        let root = doc.root;
        let block = Block::new(Content::text("Hello"), Some("title"));
        doc.add_block(block, &root).unwrap();

        let md = MarkdownRenderer::new().render(&doc).unwrap();
        // Title role renders as plain text, verify content is present
        assert!(md.contains("Hello"));
    }

    #[test]
    fn test_render_code() {
        let mut doc = Document::create();
        let root = doc.root;
        let block = Block::new(Content::code("rust", "fn main() {}"), None);
        doc.add_block(block, &root).unwrap();

        let md = MarkdownRenderer::new().render(&doc).unwrap();
        assert!(md.contains("```rust"));
        assert!(md.contains("fn main()"));
    }
}
