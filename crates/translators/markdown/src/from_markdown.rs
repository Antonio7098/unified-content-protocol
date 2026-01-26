//! Parse Markdown into UCM documents.

use crate::{Result, TranslatorError};
use ucm_core::{Block, Content, Document};

/// Markdown parser that converts to UCM
#[derive(Debug, Clone)]
pub struct MarkdownParser {
    preserve_raw: bool,
}

impl MarkdownParser {
    pub fn new() -> Self {
        Self {
            preserve_raw: false,
        }
    }

    pub fn preserve_raw(mut self, preserve: bool) -> Self {
        self.preserve_raw = preserve;
        self
    }

    pub fn parse(&self, markdown: &str) -> Result<Document> {
        use ucm_core::BlockId;

        let mut doc = Document::create();
        let root = doc.root;
        let lines: Vec<&str> = markdown.lines().collect();
        let mut i = 0;

        // Track heading hierarchy: heading_stack[level-1] = BlockId of that level's heading
        // This allows H2 to be child of H1, H3 to be child of H2, etc.
        let mut heading_stack: Vec<Option<BlockId>> = vec![None; 6];

        while i < lines.len() {
            let line = lines[i];

            // Skip empty lines
            if line.trim().is_empty() {
                i += 1;
                continue;
            }

            // Heading - creates hierarchical structure
            if let Some(heading) = self.parse_heading(line) {
                let block = Block::new(Content::text(&heading.text), Some(&heading.role));

                // Find the parent: look for the nearest heading of a higher level
                let parent = if heading.level == 1 {
                    root
                } else {
                    // Find parent from heading_stack (levels 1 to level-1)
                    let mut parent_id = root;
                    for lvl in (0..heading.level - 1).rev() {
                        if let Some(ref id) = heading_stack[lvl] {
                            parent_id = *id;
                            break;
                        }
                    }
                    parent_id
                };

                let block_id = doc
                    .add_block(block, &parent)
                    .map_err(|e| TranslatorError::InvalidStructure(e.to_string()))?;

                // Update heading stack: set this level and clear all lower levels
                heading_stack[heading.level - 1] = Some(block_id);
                for h in heading_stack
                    .iter_mut()
                    .skip(heading.level)
                    .take(6 - heading.level)
                {
                    *h = None;
                }

                i += 1;
                continue;
            }

            // Find current parent (most recent heading or root)
            let current_parent = heading_stack.iter().rev().find_map(|h| *h).unwrap_or(root);

            // Code block
            if line.starts_with("```") {
                let (code_block, consumed) = self.parse_code_block(&lines[i..])?;
                let block = Block::new(code_block, Some("code"));
                doc.add_block(block, &current_parent)
                    .map_err(|e| TranslatorError::InvalidStructure(e.to_string()))?;
                i += consumed;
                continue;
            }

            // List item
            if self.is_list_item(line) {
                let (list_content, consumed) = self.parse_list(&lines[i..]);
                let block = Block::new(Content::text(&list_content), Some("list"));
                doc.add_block(block, &current_parent)
                    .map_err(|e| TranslatorError::InvalidStructure(e.to_string()))?;
                i += consumed;
                continue;
            }

            // Blockquote
            if line.starts_with('>') {
                let (quote, consumed) = self.parse_blockquote(&lines[i..]);
                let block = Block::new(Content::text(&quote), Some("quote"));
                doc.add_block(block, &current_parent)
                    .map_err(|e| TranslatorError::InvalidStructure(e.to_string()))?;
                i += consumed;
                continue;
            }

            // Table
            if self.is_table_start(line, lines.get(i + 1).copied()) {
                let (table, consumed) = self.parse_table(&lines[i..])?;
                let block = Block::new(table, Some("table"));
                doc.add_block(block, &current_parent)
                    .map_err(|e| TranslatorError::InvalidStructure(e.to_string()))?;
                i += consumed;
                continue;
            }

            // Regular paragraph
            let (para, consumed) = self.parse_paragraph(&lines[i..]);
            let block = Block::new(Content::text(&para), Some("paragraph"));
            doc.add_block(block, &current_parent)
                .map_err(|e| TranslatorError::InvalidStructure(e.to_string()))?;
            i += consumed;
        }

        Ok(doc)
    }

    fn parse_heading(&self, line: &str) -> Option<Heading> {
        let trimmed = line.trim_start();
        if !trimmed.starts_with('#') {
            return None;
        }

        let level = trimmed.chars().take_while(|&c| c == '#').count();
        if level > 6 {
            return None;
        }

        let text = trimmed[level..].trim().to_string();
        let role = format!("heading{}", level);
        Some(Heading { level, text, role })
    }

    fn parse_code_block(&self, lines: &[&str]) -> Result<(Content, usize)> {
        let first = lines[0];
        let lang = first.trim_start_matches('`').trim().to_string();
        let lang = if lang.is_empty() {
            "text".to_string()
        } else {
            lang
        };

        let mut code_lines = Vec::new();
        let mut i = 1;

        while i < lines.len() {
            if lines[i].starts_with("```") {
                i += 1;
                break;
            }
            code_lines.push(lines[i]);
            i += 1;
        }

        let code = code_lines.join("\n");
        Ok((Content::code(&lang, &code), i))
    }

    fn is_list_item(&self, line: &str) -> bool {
        let trimmed = line.trim_start();
        trimmed.starts_with("- ")
            || trimmed.starts_with("* ")
            || trimmed.starts_with("+ ")
            || Self::is_ordered_item(trimmed)
    }

    fn is_ordered_item(line: &str) -> bool {
        let mut chars = line.chars().peekable();
        while chars.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            chars.next();
        }
        matches!((chars.next(), chars.next()), (Some('.'), Some(' ')))
    }

    fn parse_list(&self, lines: &[&str]) -> (String, usize) {
        let mut items = Vec::new();
        let mut i = 0;

        while i < lines.len() && self.is_list_item(lines[i]) {
            items.push(lines[i]);
            i += 1;
        }

        (items.join("\n"), i.max(1))
    }

    fn parse_blockquote(&self, lines: &[&str]) -> (String, usize) {
        let mut quote_lines = Vec::new();
        let mut i = 0;

        while i < lines.len() && lines[i].starts_with('>') {
            let content = lines[i].trim_start_matches('>').trim();
            quote_lines.push(content);
            i += 1;
        }

        (quote_lines.join("\n"), i.max(1))
    }

    fn is_table_start(&self, line: &str, next: Option<&str>) -> bool {
        if !line.contains('|') {
            return false;
        }
        if let Some(next_line) = next {
            next_line.contains('|') && next_line.contains('-')
        } else {
            false
        }
    }

    fn parse_table(&self, lines: &[&str]) -> Result<(Content, usize)> {
        let mut rows = Vec::new();
        let mut i = 0;

        while i < lines.len() && lines[i].contains('|') {
            // Skip separator row
            if lines[i]
                .chars()
                .all(|c| c == '|' || c == '-' || c == ':' || c == ' ')
            {
                i += 1;
                continue;
            }

            let cells: Vec<String> = lines[i]
                .split('|')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            rows.push(cells);
            i += 1;
        }

        Ok((Content::table(rows), i.max(1)))
    }

    fn parse_paragraph(&self, lines: &[&str]) -> (String, usize) {
        let mut para_lines = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];
            if line.trim().is_empty()
                || line.starts_with('#')
                || line.starts_with("```")
                || line.starts_with('>')
                || self.is_list_item(line)
            {
                break;
            }
            para_lines.push(line.trim_end());
            i += 1;
        }

        let paragraph = para_lines.join("\n");
        (paragraph.trim_end_matches('\n').to_string(), i.max(1))
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self::new()
    }
}

struct Heading {
    level: usize,
    text: String,
    role: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucm_core::metadata::RoleCategory;

    #[test]
    fn test_parse_heading() {
        let parser = MarkdownParser::new();
        let h = parser.parse_heading("## Hello World").unwrap();
        assert_eq!(h.level, 2);
        assert_eq!(h.text, "Hello World");
        assert_eq!(h.role, "heading2");
    }

    #[test]
    fn test_parse_document() {
        let md = "# Title\n\nSome text here.\n\n```rust\nfn main() {}\n```\n";
        let doc = MarkdownParser::new().parse(md).unwrap();
        assert!(doc.block_count() > 1);
    }

    #[test]
    fn test_heading_semantic_roles() {
        let md = "# H1 Title\n\n## H2 Section\n\n### H3 Subsection\n";
        let doc = MarkdownParser::new().parse(md).unwrap();

        // Find blocks with heading roles
        let headings: Vec<_> = doc
            .blocks
            .values()
            .filter(|b| b.metadata.semantic_role.is_some())
            .collect();

        assert!(headings.len() >= 3, "Should have at least 3 heading blocks");

        // Check for specific heading roles
        let has_h1 = headings.iter().any(|b| {
            b.metadata
                .semantic_role
                .as_ref()
                .map(|r| r.category == RoleCategory::Heading1)
                .unwrap_or(false)
        });
        let has_h2 = headings.iter().any(|b| {
            b.metadata
                .semantic_role
                .as_ref()
                .map(|r| r.category == RoleCategory::Heading2)
                .unwrap_or(false)
        });
        let has_h3 = headings.iter().any(|b| {
            b.metadata
                .semantic_role
                .as_ref()
                .map(|r| r.category == RoleCategory::Heading3)
                .unwrap_or(false)
        });

        assert!(has_h1, "Should have heading1 role");
        assert!(has_h2, "Should have heading2 role");
        assert!(has_h3, "Should have heading3 role");
    }

    #[test]
    fn test_heading_hierarchy() {
        let md = r#"# Title

## Section 1

Content under section 1.

### Subsection 1.1

More content.

## Section 2

Content under section 2.
"#;
        let doc = MarkdownParser::new().parse(md).unwrap();

        // Root should have exactly one child (the H1)
        let root_children = doc
            .structure
            .get(&doc.root)
            .expect("Root should have children");
        assert_eq!(
            root_children.len(),
            1,
            "Root should have exactly one H1 child"
        );

        // H1 should have Section 1 and Section 2 as children
        let h1_id = &root_children[0];
        let h1_children = doc.structure.get(h1_id).expect("H1 should have children");
        assert_eq!(
            h1_children.len(),
            2,
            "H1 should have 2 H2 children (Section 1 and Section 2)"
        );

        // First H2 (Section 1) should have content and H3 as children
        let h2_section1_id = &h1_children[0];
        let h2_section1_children = doc
            .structure
            .get(h2_section1_id)
            .expect("Section 1 should have children");
        assert!(
            h2_section1_children.len() >= 2,
            "Section 1 should have content and subsection"
        );
    }

    #[test]
    fn test_content_under_heading() {
        let md = r#"# Title

This is content under the title.

## Section

This is content under section.
"#;
        let doc = MarkdownParser::new().parse(md).unwrap();

        // Find the H1 block
        let h1_block = doc
            .blocks
            .values()
            .find(|b| {
                b.metadata
                    .semantic_role
                    .as_ref()
                    .map(|r| r.category == RoleCategory::Heading1)
                    .unwrap_or(false)
            })
            .expect("Should have H1 block");

        // H1 should have children (paragraph and H2)
        let h1_children = doc
            .structure
            .get(&h1_block.id)
            .expect("H1 should have children");
        assert!(
            h1_children.len() >= 2,
            "H1 should have paragraph and H2 as children"
        );
    }

    #[test]
    fn test_code_block_under_heading() {
        let md = r#"# Code Example

```python
def hello():
    print("Hello")
```
"#;
        let doc = MarkdownParser::new().parse(md).unwrap();

        // Find the H1 block
        let h1_block = doc
            .blocks
            .values()
            .find(|b| {
                b.metadata
                    .semantic_role
                    .as_ref()
                    .map(|r| r.category == RoleCategory::Heading1)
                    .unwrap_or(false)
            })
            .expect("Should have H1 block");

        // H1 should have code block as child
        let h1_children = doc
            .structure
            .get(&h1_block.id)
            .expect("H1 should have children");
        assert!(
            !h1_children.is_empty(),
            "H1 should have code block as child"
        );

        // Check that there's a code block in children
        let has_code_child = h1_children.iter().any(|child_id| {
            doc.blocks
                .get(child_id)
                .map(|b| matches!(b.content, ucm_core::Content::Code(_)))
                .unwrap_or(false)
        });
        assert!(has_code_child, "H1 should have code block as child");
    }

    #[test]
    fn test_list_semantic_role() {
        let md = r#"# List Example

- Item 1
- Item 2
- Item 3
"#;
        let doc = MarkdownParser::new().parse(md).unwrap();

        // Find the list block
        let list_block = doc.blocks.values().find(|b| {
            b.metadata
                .semantic_role
                .as_ref()
                .map(|r| r.category == RoleCategory::List)
                .unwrap_or(false)
        });

        assert!(
            list_block.is_some(),
            "Should have list block with List semantic role"
        );
    }

    #[test]
    fn test_paragraph_semantic_role() {
        let md = r#"# Title

This is a paragraph.
"#;
        let doc = MarkdownParser::new().parse(md).unwrap();

        // Find the paragraph block
        let para_block = doc.blocks.values().find(|b| {
            b.metadata
                .semantic_role
                .as_ref()
                .map(|r| r.category == RoleCategory::Paragraph)
                .unwrap_or(false)
        });

        assert!(
            para_block.is_some(),
            "Should have paragraph block with Paragraph semantic role"
        );
    }

    #[test]
    fn test_quote_semantic_role() {
        let md = r#"# Quote Example

> This is a quote.
"#;
        let doc = MarkdownParser::new().parse(md).unwrap();

        // Find the quote block
        let quote_block = doc.blocks.values().find(|b| {
            b.metadata
                .semantic_role
                .as_ref()
                .map(|r| r.category == RoleCategory::Quote)
                .unwrap_or(false)
        });

        assert!(
            quote_block.is_some(),
            "Should have quote block with Quote semantic role"
        );
    }
}
