//! ID Mapper for LLM prompts
//!
//! Maps long BlockIds (24 chars) to short numeric IDs (1, 2, 3, etc.)
//! to save tokens when passing documents to LLMs.

use std::collections::HashMap;
use ucm_core::{BlockId, Content, Document};

/// Get a preview of content (first N characters)
fn content_preview(content: &Content, max_len: usize) -> String {
    let text = match content {
        Content::Text(t) => t.text.clone(),
        Content::Code(c) => format!("```{}\n{}```", c.language, c.source),
        Content::Table(t) => format!("Table {}x{}", t.columns.len(), t.rows.len()),
        Content::Math(m) => m.expression.clone(),
        Content::Json { value, .. } => value.to_string(),
        Content::Media(m) => format!("Media: {:?}", m.media_type),
        Content::Binary { mime_type, .. } => format!("Binary: {}", mime_type),
        Content::Composite { layout, children } => format!("{:?} ({} children)", layout, children.len()),
    };
    
    if text.len() > max_len {
        format!("{}...", &text[..max_len])
    } else {
        text
    }
}

/// Bidirectional mapping between BlockIds and short numeric IDs
#[derive(Debug, Clone)]
pub struct IdMapper {
    /// BlockId -> short ID
    to_short: HashMap<BlockId, u32>,
    /// short ID -> BlockId
    to_long: HashMap<u32, BlockId>,
    /// Next ID to assign
    next_id: u32,
}

impl IdMapper {
    pub fn new() -> Self {
        Self {
            to_short: HashMap::new(),
            to_long: HashMap::new(),
            next_id: 1,
        }
    }

    /// Create a mapper from a document, assigning sequential IDs to all blocks
    pub fn from_document(doc: &Document) -> Self {
        let mut mapper = Self::new();
        
        // Add root first
        mapper.register(&doc.root);
        
        // Add all other blocks in a deterministic order (sorted by ID)
        let mut block_ids: Vec<_> = doc.blocks.keys().collect();
        block_ids.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
        
        for block_id in block_ids {
            if block_id != &doc.root {
                mapper.register(block_id);
            }
        }
        
        mapper
    }

    /// Register a BlockId and get its short ID
    pub fn register(&mut self, block_id: &BlockId) -> u32 {
        if let Some(&short_id) = self.to_short.get(block_id) {
            return short_id;
        }
        
        let short_id = self.next_id;
        self.next_id += 1;
        self.to_short.insert(block_id.clone(), short_id);
        self.to_long.insert(short_id, block_id.clone());
        short_id
    }

    /// Get short ID for a BlockId
    pub fn to_short_id(&self, block_id: &BlockId) -> Option<u32> {
        self.to_short.get(block_id).copied()
    }

    /// Get BlockId for a short ID
    pub fn to_block_id(&self, short_id: u32) -> Option<&BlockId> {
        self.to_long.get(&short_id)
    }

    /// Convert a string containing block IDs to use short IDs
    /// Replaces patterns like "blk_abc123..." with "1", "2", etc.
    pub fn shorten_text(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (block_id, short_id) in &self.to_short {
            let long_str = block_id.to_string();
            let short_str = short_id.to_string();
            result = result.replace(&long_str, &short_str);
        }
        result
    }

    /// Convert a string containing short IDs back to block IDs
    /// Replaces patterns like "1", "2" back to "blk_abc123..."
    /// Note: This is context-sensitive - only replaces standalone numbers
    pub fn expand_text(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        // Sort by ID descending to avoid replacing "1" in "10"
        let mut ids: Vec<_> = self.to_long.iter().collect();
        ids.sort_by(|a, b| b.0.cmp(a.0));
        
        for (short_id, block_id) in ids {
            // Use word boundary matching to avoid partial replacements
            let short_str = short_id.to_string();
            let long_str = block_id.to_string();
            
            // Replace patterns like "block 1" or "id: 1" or "1," etc.
            let patterns = [
                (format!("block {}", short_str), format!("block {}", long_str)),
                (format!("id {}", short_str), format!("id {}", long_str)),
                (format!("#{}", short_str), format!("#{}", long_str)),
                (format!("[{}]", short_str), format!("[{}]", long_str)),
            ];
            
            for (from, to) in patterns {
                result = result.replace(&from, &to);
            }
        }
        
        result
    }

    /// Generate a compact document representation for LLM prompts
    pub fn document_to_prompt(&self, doc: &Document) -> String {
        let mut lines = Vec::new();
        
        // Header
        lines.push("Document Structure:".to_string());
        
        // Build hierarchy representation
        self.build_hierarchy_lines(doc, &doc.root, 0, &mut lines);
        
        lines.join("\n")
    }

    fn build_hierarchy_lines(
        &self,
        doc: &Document,
        block_id: &BlockId,
        depth: usize,
        lines: &mut Vec<String>,
    ) {
        let short_id = self.to_short.get(block_id).map(|id| id.to_string()).unwrap_or_else(|| "?".to_string());
        let indent = "  ".repeat(depth);
        
        if let Some(block) = doc.get_block(block_id) {
            let role = block.metadata.semantic_role.as_ref()
                .map(|r| format!("{:?}", r.category))
                .unwrap_or_else(|| "block".to_string());
            
            let content_preview = content_preview(&block.content, 50);
            lines.push(format!("{}[{}] {} - {}", indent, short_id, role, content_preview));
        }
        
        // Process children
        if let Some(children) = doc.structure.get(block_id) {
            for child_id in children {
                self.build_hierarchy_lines(doc, child_id, depth + 1, lines);
            }
        }
    }

    /// Get the mapping table as a string (useful for debugging)
    pub fn mapping_table(&self) -> String {
        let mut lines = Vec::new();
        lines.push("ID Mapping:".to_string());
        
        let mut entries: Vec<_> = self.to_short.iter().collect();
        entries.sort_by_key(|(_, &id)| id);
        
        for (block_id, short_id) in entries {
            lines.push(format!("  {} = {}", short_id, block_id));
        }
        
        lines.join("\n")
    }

    /// Total number of mappings
    pub fn len(&self) -> usize {
        self.to_short.len()
    }

    pub fn is_empty(&self) -> bool {
        self.to_short.is_empty()
    }
}

impl Default for IdMapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucm_core::{Block, Content};

    #[test]
    fn test_id_mapper() {
        let mut doc = Document::create();
        let root = doc.root.clone();
        
        let block1 = Block::new(Content::text("Hello"), Some("heading1"));
        let id1 = doc.add_block(block1, &root).unwrap();
        
        let block2 = Block::new(Content::text("World"), Some("paragraph"));
        let id2 = doc.add_block(block2, &id1).unwrap();
        
        let mapper = IdMapper::from_document(&doc);
        
        // Root should be 1
        assert_eq!(mapper.to_short_id(&root), Some(1));
        
        // Other blocks should have sequential IDs
        assert!(mapper.to_short_id(&id1).is_some());
        assert!(mapper.to_short_id(&id2).is_some());
        
        // Reverse mapping should work
        assert_eq!(mapper.to_block_id(1), Some(&root));
    }

    #[test]
    fn test_shorten_text() {
        let mut mapper = IdMapper::new();
        let block_id = BlockId::from_hex("aabbccdd11223344").unwrap();
        mapper.register(&block_id);
        
        let text = format!("Edit block {}", block_id);
        let shortened = mapper.shorten_text(&text);
        
        assert_eq!(shortened, "Edit block 1");
    }
}
