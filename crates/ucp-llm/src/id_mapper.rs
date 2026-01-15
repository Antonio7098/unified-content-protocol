//! ID Mapper for LLM prompts
//!
//! Maps long BlockIds (24 chars) to short numeric IDs (1, 2, 3, etc.)
//! to save tokens when passing documents to LLMs.
//!
//! ## Token Efficiency
//!
//! A typical BlockId like `blk_aabbccdd1122` uses ~20 tokens.
//! A short ID like `3` uses 1 token. For a document with 50 blocks
//! referenced 3 times each in a prompt, this saves ~2,850 tokens.
//!
//! ## UCL Integration
//!
//! The mapper can convert UCL commands between long and short ID formats:
//! - `shorten_ucl()`: Convert UCL with long IDs to short IDs
//! - `expand_ucl()`: Convert UCL with short IDs back to long IDs

use regex::Regex;
use std::collections::HashMap;
use ucm_core::{BlockId, Content, Document};

/// Get the full string representation of content
fn content_to_string(content: &Content) -> String {
    match content {
        Content::Text(t) => t.text.clone(),
        Content::Code(c) => c.source.clone(),
        Content::Table(t) => format!("Table {}x{}", t.columns.len(), t.rows.len()),
        Content::Math(m) => m.expression.clone(),
        Content::Json { value, .. } => value.to_string(),
        Content::Media(m) => format!("Media: {:?}", m.media_type),
        Content::Binary { mime_type, .. } => format!("Binary: {}", mime_type),
        Content::Composite { layout, children } => {
            format!("{:?} ({} children)", layout, children.len())
        }
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
        block_ids.sort_by_key(|a| a.to_string());

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
        self.to_short.insert(*block_id, short_id);
        self.to_long.insert(short_id, *block_id);
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
                (
                    format!("block {}", short_str),
                    format!("block {}", long_str),
                ),
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

    /// Convert UCL commands from long BlockIds to short numeric IDs
    ///
    /// This is the primary method for token-efficient UCL generation.
    /// The LLM receives prompts with short IDs and generates UCL with short IDs,
    /// which is then expanded back to full BlockIds before execution.
    pub fn shorten_ucl(&self, ucl: &str) -> String {
        let mut result = ucl.to_string();

        // Replace all block IDs with their short versions
        // Process longer IDs first to avoid partial matches
        let mut entries: Vec<_> = self.to_short.iter().collect();
        entries.sort_by(|a, b| b.0.to_string().len().cmp(&a.0.to_string().len()));

        for (block_id, short_id) in entries {
            result = result.replace(&block_id.to_string(), &short_id.to_string());
        }

        result
    }

    /// Convert UCL commands from short numeric IDs back to full BlockIds
    ///
    /// This expands short IDs in UCL commands back to full BlockIds.
    /// Uses regex to match UCL command patterns and replace IDs contextually.
    pub fn expand_ucl(&self, ucl: &str) -> String {
        // Pattern to match short IDs in UCL command contexts
        // Matches: EDIT 1, APPEND 1, MOVE 1, DELETE 1, LINK 1, TO 1, BEFORE 1, AFTER 1, etc.
        let ucl_id_pattern = Regex::new(
            r"(?x)
            (?P<prefix>
                \b(?:EDIT|APPEND|MOVE|DELETE|LINK|UNLINK|TO|BEFORE|AFTER)\s+
            )
            (?P<id>\d+)
            ",
        )
        .unwrap();

        let mut result = ucl.to_string();

        // Find all matches and collect replacements
        let replacements: Vec<_> = ucl_id_pattern
            .captures_iter(&result.clone())
            .filter_map(|cap| {
                let id_str = cap.name("id")?.as_str();
                let short_id: u32 = id_str.parse().ok()?;
                let block_id = self.to_long.get(&short_id)?;
                let full_match = cap.get(0)?;
                let prefix = cap.name("prefix")?.as_str();
                Some((
                    full_match.as_str().to_string(),
                    format!("{}{}", prefix, block_id),
                ))
            })
            .collect();

        // Apply replacements (in reverse order to preserve positions)
        for (from, to) in replacements.iter().rev() {
            result = result.replacen(from, to, 1);
        }

        // Also handle edge type targets (second ID in LINK commands)
        let link_target_pattern = Regex::new(
            r"(?x)
            (?P<prefix>
                \b(?:references|elaborates|summarizes|contradicts|supports|requires|parent_of)\s+
            )
            (?P<id>\d+)
            ",
        )
        .unwrap();

        let replacements: Vec<_> = link_target_pattern
            .captures_iter(&result.clone())
            .filter_map(|cap| {
                let id_str = cap.name("id")?.as_str();
                let short_id: u32 = id_str.parse().ok()?;
                let block_id = self.to_long.get(&short_id)?;
                let full_match = cap.get(0)?;
                let prefix = cap.name("prefix")?.as_str();
                Some((
                    full_match.as_str().to_string(),
                    format!("{}{}", prefix, block_id),
                ))
            })
            .collect();

        for (from, to) in replacements.iter().rev() {
            result = result.replacen(from, to, 1);
        }

        result
    }

    /// Estimate token savings from using short IDs
    ///
    /// Returns (original_tokens, shortened_tokens, savings)
    pub fn estimate_token_savings(&self, text: &str) -> (usize, usize, usize) {
        let shortened = self.shorten_text(text);

        // Rough token estimation: ~4 chars per token for English
        let original_tokens = text.len() / 4;
        let shortened_tokens = shortened.len() / 4;
        let savings = original_tokens.saturating_sub(shortened_tokens);

        (original_tokens, shortened_tokens, savings)
    }

    /// Generate a normalized document representation for LLM prompts
    pub fn document_to_prompt(&self, doc: &Document) -> String {
        let mut lines = Vec::new();

        // Header
        lines.push("Document structure:".to_string());

        // Collect all block IDs in BFS order
        let mut all_blocks = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(doc.root);
        while let Some(block_id) = queue.pop_front() {
            all_blocks.push(block_id);
            if let Some(children) = doc.structure.get(&block_id) {
                for child in children {
                    queue.push_back(*child);
                }
            }
        }

        // Document structure section: parent: child1 child2 ...
        for block_id in &all_blocks {
            let short_id = self
                .to_short
                .get(block_id)
                .map(|id| id.to_string())
                .unwrap_or_else(|| "?".to_string());

            let children = doc.children(block_id);
            if children.is_empty() {
                lines.push(format!("{}:", short_id));
            } else {
                let child_ids: Vec<String> = children
                    .iter()
                    .map(|c| {
                        self.to_short
                            .get(c)
                            .map(|id| id.to_string())
                            .unwrap_or_else(|| "?".to_string())
                    })
                    .collect();
                lines.push(format!("{}: {}", short_id, child_ids.join(" ")));
            }
        }

        // Blocks section
        lines.push(String::new());
        lines.push("Blocks:".to_string());
        for block_id in &all_blocks {
            if let Some(block) = doc.get_block(block_id) {
                let short_id = self
                    .to_short
                    .get(block_id)
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| "?".to_string());

                let content_type = block.content.type_tag();
                let content_str = content_to_string(&block.content);
                // Escape content for display
                let escaped_content = content_str
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace('\n', "\\n");
                lines.push(format!(
                    "{} type={} content=\"{}\"",
                    short_id, content_type, escaped_content
                ));
            }
        }

        lines.join("\n")
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
        let root = doc.root;

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

    #[test]
    fn test_shorten_ucl() {
        let mut mapper = IdMapper::new();
        let block1 = BlockId::from_hex("aabbccdd11223344").unwrap();
        let block2 = BlockId::from_hex("11223344aabbccdd").unwrap();
        mapper.register(&block1);
        mapper.register(&block2);

        let ucl = format!("EDIT {} SET text = \"hello\"", block1);
        let shortened = mapper.shorten_ucl(&ucl);
        assert_eq!(shortened, "EDIT 1 SET text = \"hello\"");

        let ucl = format!("MOVE {} TO {}", block1, block2);
        let shortened = mapper.shorten_ucl(&ucl);
        assert_eq!(shortened, "MOVE 1 TO 2");
    }

    #[test]
    fn test_expand_ucl() {
        let mut mapper = IdMapper::new();
        let block1 = BlockId::from_hex("aabbccdd11223344").unwrap();
        let block2 = BlockId::from_hex("11223344aabbccdd").unwrap();
        mapper.register(&block1);
        mapper.register(&block2);

        let ucl = "EDIT 1 SET text = \"hello\"";
        let expanded = mapper.expand_ucl(ucl);
        assert!(expanded.contains(&block1.to_string()));

        let ucl = "MOVE 1 TO 2";
        let expanded = mapper.expand_ucl(ucl);
        assert!(expanded.contains(&block1.to_string()));
        assert!(expanded.contains(&block2.to_string()));
    }

    #[test]
    fn test_ucl_roundtrip() {
        let mut mapper = IdMapper::new();
        let block1 = BlockId::from_hex("aabbccdd11223344").unwrap();
        let block2 = BlockId::from_hex("11223344aabbccdd").unwrap();
        mapper.register(&block1);
        mapper.register(&block2);

        let original = format!("LINK {} references {}", block1, block2);
        let shortened = mapper.shorten_ucl(&original);
        let expanded = mapper.expand_ucl(&shortened);

        assert_eq!(original, expanded);
    }

    #[test]
    fn test_token_savings() {
        let mut doc = Document::create();
        let root = doc.root;

        // Add several blocks
        for i in 0..10 {
            let block = Block::new(Content::text(format!("Block {}", i)), None);
            doc.add_block(block, &root).unwrap();
        }

        let mapper = IdMapper::from_document(&doc);

        // Create a prompt with multiple block references
        let mut prompt = String::new();
        for block_id in mapper.to_short.keys() {
            prompt.push_str(&format!("Block {} has content. ", block_id));
        }

        let (original, shortened, savings) = mapper.estimate_token_savings(&prompt);
        assert!(savings > 0, "Should have token savings");
        assert!(shortened < original, "Shortened should be smaller");
    }

    #[test]
    fn test_document_to_prompt_format() {
        let mut doc = Document::create();
        let root = doc.root;

        let block1 = Block::new(Content::text("Title"), Some("heading1"));
        let id1 = doc.add_block(block1, &root).unwrap();

        let block2 = Block::new(Content::text("Paragraph"), Some("paragraph"));
        doc.add_block(block2, &id1).unwrap();

        let mapper = IdMapper::from_document(&doc);
        let prompt = mapper.document_to_prompt(&doc);

        // Check normalized format
        assert!(prompt.contains("Document structure:"));
        assert!(prompt.contains("Blocks:"));
        assert!(prompt.contains("type="));
        assert!(prompt.contains("content=\""));
    }
}
