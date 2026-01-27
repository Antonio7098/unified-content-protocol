//! Output formatting utilities

use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use tabled::{Table, Tabled};
use ucm_core::{Block, BlockId, Document, Edge};

use crate::cli::OutputFormat;

/// Serializable document representation for JSON I/O
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentJson {
    pub id: String,
    pub root: String,
    pub structure: HashMap<String, Vec<String>>,
    pub blocks: HashMap<String, Block>,
    #[serde(default)]
    pub metadata: DocumentMetadataJson,
    #[serde(default)]
    pub version: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocumentMetadataJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
}

impl DocumentJson {
    /// Convert Document to JSON-serializable form
    pub fn from_document(doc: &Document) -> Self {
        let structure: HashMap<String, Vec<String>> = doc
            .structure
            .iter()
            .map(|(k, v)| (k.to_string(), v.iter().map(|id| id.to_string()).collect()))
            .collect();

        let blocks: HashMap<String, Block> = doc
            .blocks
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();

        Self {
            id: doc.id.0.clone(),
            root: doc.root.to_string(),
            structure,
            blocks,
            metadata: DocumentMetadataJson {
                title: doc.metadata.title.clone(),
                description: doc.metadata.description.clone(),
                authors: doc.metadata.authors.clone(),
                created_at: Some(doc.metadata.created_at.to_rfc3339()),
                modified_at: Some(doc.metadata.modified_at.to_rfc3339()),
            },
            version: doc.version.counter,
        }
    }

    /// Convert back to Document
    pub fn to_document(&self) -> anyhow::Result<Document> {
        use std::str::FromStr;
        use ucm_core::{DocumentId, DocumentMetadata};

        let root = BlockId::from_str(&self.root)
            .map_err(|_| anyhow::anyhow!("Invalid root block ID: {}", self.root))?;

        let mut structure: HashMap<BlockId, Vec<BlockId>> = HashMap::new();
        for (k, v) in &self.structure {
            let parent = BlockId::from_str(k)
                .map_err(|_| anyhow::anyhow!("Invalid block ID in structure: {}", k))?;
            let children: Result<Vec<BlockId>, _> = v
                .iter()
                .map(|id| {
                    BlockId::from_str(id)
                        .map_err(|_| anyhow::anyhow!("Invalid block ID in structure: {}", id))
                })
                .collect();
            structure.insert(parent, children?);
        }

        let mut blocks: HashMap<BlockId, Block> = HashMap::new();
        for (k, v) in &self.blocks {
            let id = BlockId::from_str(k)
                .map_err(|_| anyhow::anyhow!("Invalid block ID: {}", k))?;
            blocks.insert(id, v.clone());
        }

        let now = chrono::Utc::now();
        let metadata = DocumentMetadata {
            title: self.metadata.title.clone(),
            description: self.metadata.description.clone(),
            authors: self.metadata.authors.clone(),
            created_at: self
                .metadata
                .created_at
                .as_ref()
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or(now),
            modified_at: self
                .metadata
                .modified_at
                .as_ref()
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or(now),
            language: None,
            custom: HashMap::new(),
        };

        Ok(Document {
            id: DocumentId::new(&self.id),
            root,
            structure,
            blocks,
            metadata,
            indices: Default::default(),
            edge_index: Default::default(),
            version: ucm_core::DocumentVersion {
                counter: self.version,
                timestamp: chrono::Utc::now(),
                state_hash: [0u8; 8],
            },
        })
    }
}

/// Print a value in the specified format
pub fn print_value<T: Serialize + std::fmt::Display>(value: &T, format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            if let Ok(json) = serde_json::to_string_pretty(value) {
                println!("{}", json);
            }
        }
        OutputFormat::Text => println!("{}", value),
    }
}

/// Print a serializable value as JSON or a custom text format
pub fn print_output<T: Serialize>(value: &T, format: OutputFormat, text_fn: impl FnOnce(&T)) {
    match format {
        OutputFormat::Json => {
            if let Ok(json) = serde_json::to_string_pretty(value) {
                println!("{}", json);
            }
        }
        OutputFormat::Text => text_fn(value),
    }
}

/// Print a success message
pub fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

/// Print an error message
pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message);
}

/// Print a warning message
pub fn print_warning(message: &str) {
    eprintln!("{} {}", "⚠".yellow().bold(), message);
}

/// Print an info message
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

/// Print a block in a human-readable format
pub fn print_block(block: &Block, show_content: bool) {
    println!("{}", "─".repeat(60).dimmed());
    println!(
        "{}: {}",
        "Block ID".cyan().bold(),
        block.id.to_string().yellow()
    );

    if let Some(label) = &block.metadata.label {
        println!("{}: {}", "Label".cyan(), label);
    }

    if let Some(role) = &block.metadata.semantic_role {
        println!("{}: {}", "Role".cyan(), role);
    }

    println!("{}: {}", "Content Type".cyan(), block.content.type_tag());

    if !block.metadata.tags.is_empty() {
        println!(
            "{}: {}",
            "Tags".cyan(),
            block.metadata.tags.join(", ").dimmed()
        );
    }

    if let Some(summary) = &block.metadata.summary {
        println!("{}: {}", "Summary".cyan(), summary.dimmed());
    }

    if let Some(estimate) = &block.metadata.token_estimate {
        println!("{}: ~{} tokens", "Size".cyan(), estimate.generic);
    }

    if show_content {
        println!("{}", "Content:".cyan().bold());
        let content_str = content_preview(&block.content, 500);
        println!("{}", content_str);
    }

    if !block.edges.is_empty() {
        println!(
            "{}: {}",
            "Edges".cyan(),
            format!("{} outgoing", block.edges.len()).dimmed()
        );
    }
}

/// Get a preview of content (truncated to max_len)
pub fn content_preview(content: &ucm_core::Content, max_len: usize) -> String {
    let full = match content {
        ucm_core::Content::Text(text) => text.text.clone(),
        ucm_core::Content::Code(code) => format!("```{}\n{}\n```", code.language, code.source),
        ucm_core::Content::Table(table) => format!(
            "Table: {} columns, {} rows",
            table.columns.len(),
            table.rows.len()
        ),
        ucm_core::Content::Math(math) => format!("Math: {}", math.expression),
        ucm_core::Content::Media(media) => {
            format!("Media: {:?} - {:?}", media.media_type, media.source)
        }
        ucm_core::Content::Json { value, .. } => {
            serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".to_string())
        }
        ucm_core::Content::Binary { mime_type, .. } => format!("Binary: {}", mime_type),
        ucm_core::Content::Composite { children, .. } => {
            format!("Composite: {} children", children.len())
        }
    };

    if full.len() > max_len {
        format!("{}...", &full[..max_len])
    } else {
        full
    }
}

/// Print document info
pub fn print_document_info(doc: &Document) {
    println!("{}", "═".repeat(60).cyan());
    println!("{}", "Document Information".cyan().bold());
    println!("{}", "═".repeat(60).cyan());

    println!("{}: {}", "Document ID".white().bold(), doc.id.to_string());
    println!("{}: {}", "Root Block".white(), doc.root.to_string());
    println!(
        "{}: {}",
        "Block Count".white(),
        doc.block_count().to_string().green()
    );

    let total_tokens = doc.total_tokens(ucm_core::TokenModel::Generic);
    println!(
        "{}: ~{}",
        "Total Tokens".white(),
        total_tokens.to_string().yellow()
    );

    println!("{}: v{}", "Version".white(), doc.version.counter);

    if let Some(title) = &doc.metadata.title {
        println!("{}: {}", "Title".white(), title);
    }

    if let Some(desc) = &doc.metadata.description {
        println!("{}: {}", "Description".white(), desc);
    }

    // Count edges
    let edge_count = doc.edge_index.edge_count();
    println!("{}: {}", "Edge Count".white(), edge_count.to_string().blue());

    println!("{}", "═".repeat(60).cyan());
}

/// Print a tree representation of the document
pub fn print_tree(doc: &Document, max_depth: Option<usize>, show_ids: bool) {
    fn print_tree_recursive(
        doc: &Document,
        block_id: &BlockId,
        prefix: &str,
        is_last: bool,
        depth: usize,
        max_depth: Option<usize>,
        show_ids: bool,
    ) {
        if let Some(max) = max_depth {
            if depth > max {
                return;
            }
        }

        let block = match doc.get_block(block_id) {
            Some(b) => b,
            None => return,
        };

        // Build the line
        let connector = if is_last { "└── " } else { "├── " };
        let mut line = format!("{}{}", prefix, connector);

        // Add block info
        if show_ids {
            line.push_str(&format!("[{}] ", block_id.to_string().yellow()));
        }

        if let Some(role) = &block.metadata.semantic_role {
            line.push_str(&format!("{} ", role.to_string().cyan()));
        }

        if let Some(label) = &block.metadata.label {
            line.push_str(&format!("\"{}\" ", label.green()));
        } else {
            // Show content preview
            let preview = content_preview(&block.content, 40);
            let preview_trimmed = preview.lines().next().unwrap_or("");
            line.push_str(&format!("\"{}\"", preview_trimmed.dimmed()));
        }

        println!("{}", line);

        // Get children
        let children = doc.children(block_id);
        let child_count = children.len();

        for (i, child_id) in children.iter().enumerate() {
            let child_is_last = i == child_count - 1;
            let new_prefix = format!("{}{}   ", prefix, if is_last { " " } else { "│" });
            print_tree_recursive(
                doc,
                child_id,
                &new_prefix,
                child_is_last,
                depth + 1,
                max_depth,
                show_ids,
            );
        }
    }

    println!("{}", "Document Tree".cyan().bold());
    println!("{}", "─".repeat(40).dimmed());

    // Start from root
    let root_children = doc.children(&doc.root);
    let root_count = root_children.len();

    // Print root
    if show_ids {
        println!("[{}] {}", doc.root.to_string().yellow(), "root".cyan());
    } else {
        println!("{}", "root".cyan());
    }

    for (i, child_id) in root_children.iter().enumerate() {
        let is_last = i == root_count - 1;
        print_tree_recursive(doc, child_id, "", is_last, 1, max_depth, show_ids);
    }
}

/// Block summary for tables
#[derive(Tabled, Serialize)]
pub struct BlockSummary {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "Type")]
    pub content_type: String,
    #[tabled(rename = "Role")]
    pub role: String,
    #[tabled(rename = "Label")]
    pub label: String,
    #[tabled(rename = "Tokens")]
    pub tokens: String,
}

impl BlockSummary {
    pub fn from_block(block: &Block) -> Self {
        Self {
            id: block.id.to_string(),
            content_type: block.content.type_tag().to_string(),
            role: block
                .metadata
                .semantic_role
                .as_ref()
                .map(|r| r.to_string())
                .unwrap_or_else(|| "-".to_string()),
            label: block
                .metadata
                .label
                .clone()
                .unwrap_or_else(|| "-".to_string()),
            tokens: block
                .metadata
                .token_estimate
                .as_ref()
                .map(|t| t.generic.to_string())
                .unwrap_or_else(|| "-".to_string()),
        }
    }
}

/// Print a table of blocks
pub fn print_block_table(blocks: &[&Block]) {
    let summaries: Vec<BlockSummary> = blocks.iter().map(|b| BlockSummary::from_block(b)).collect();
    let table = Table::new(&summaries).to_string();
    println!("{}", table);
}

/// Edge summary for tables
#[derive(Tabled, Serialize)]
pub struct EdgeSummary {
    #[tabled(rename = "Source")]
    pub source: String,
    #[tabled(rename = "Type")]
    pub edge_type: String,
    #[tabled(rename = "Target")]
    pub target: String,
}

impl EdgeSummary {
    pub fn new(source: &BlockId, edge: &Edge) -> Self {
        Self {
            source: source.to_string(),
            edge_type: format!("{:?}", edge.edge_type),
            target: edge.target.to_string(),
        }
    }
}

/// Print edges in a table
pub fn print_edge_table(edges: &[(BlockId, Edge)]) {
    let summaries: Vec<EdgeSummary> = edges
        .iter()
        .map(|(src, edge)| EdgeSummary::new(src, edge))
        .collect();
    let table = Table::new(&summaries).to_string();
    println!("{}", table);
}

/// Print validation results
pub fn print_validation_result(result: &ucm_engine::ValidationResult) {
    if result.valid {
        print_success("Document is valid");
    } else {
        print_error("Document validation failed");
    }

    if !result.issues.is_empty() {
        println!("\n{}", "Issues:".yellow().bold());
        for issue in &result.issues {
            let severity_str = match issue.severity {
                ucm_core::ValidationSeverity::Error => "ERROR".red().bold(),
                ucm_core::ValidationSeverity::Warning => "WARN".yellow().bold(),
                ucm_core::ValidationSeverity::Info => "INFO".blue().bold(),
            };
            println!("  {} {}", severity_str, issue.message);
        }
    }
}

/// Read document from file or stdin
pub fn read_document(input: Option<String>) -> anyhow::Result<Document> {
    let json = if let Some(path) = input {
        std::fs::read_to_string(&path)?
    } else {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        buffer
    };

    let doc_json: DocumentJson = serde_json::from_str(&json)?;
    doc_json.to_document()
}

/// Write document to file or stdout
pub fn write_document(doc: &Document, output: Option<String>) -> anyhow::Result<()> {
    let doc_json = DocumentJson::from_document(doc);
    let json = serde_json::to_string_pretty(&doc_json)?;

    if let Some(path) = output {
        std::fs::write(&path, &json)?;
        print_success(&format!("Document written to {}", path));
    } else {
        println!("{}", json);
    }

    Ok(())
}

/// Read content from file
pub fn read_file(path: &str) -> anyhow::Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

/// Write content to file or stdout
pub fn write_output(content: &str, output: Option<String>) -> anyhow::Result<()> {
    if let Some(path) = output {
        std::fs::write(&path, content)?;
        print_success(&format!("Written to {}", path));
    } else {
        println!("{}", content);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucm_core::{Block, Content};

    #[test]
    fn test_document_json_roundtrip() {
        let doc = Document::create();
        let json = DocumentJson::from_document(&doc);
        let restored = json.to_document().expect("Should restore document");

        assert_eq!(doc.id.0, restored.id.0);
        assert_eq!(doc.root, restored.root);
        assert_eq!(doc.block_count(), restored.block_count());
    }

    #[test]
    fn test_document_json_preserves_blocks() {
        let mut doc = Document::create();
        let block = Block::new(Content::text("Hello, world!"), Some("paragraph"));
        let block_id = doc.add_block(block, &doc.root.clone()).expect("Should add block");

        let json = DocumentJson::from_document(&doc);
        let restored = json.to_document().expect("Should restore document");

        assert!(restored.get_block(&block_id).is_some());
    }

    #[test]
    fn test_document_json_preserves_metadata() {
        let mut doc = Document::create();
        doc.metadata.title = Some("Test Title".to_string());
        doc.metadata.description = Some("Test Description".to_string());

        let json = DocumentJson::from_document(&doc);
        let restored = json.to_document().expect("Should restore document");

        assert_eq!(restored.metadata.title, Some("Test Title".to_string()));
        assert_eq!(restored.metadata.description, Some("Test Description".to_string()));
    }

    #[test]
    fn test_content_preview_text() {
        let content = Content::text("Hello, world!");
        let preview = content_preview(&content, 100);
        assert_eq!(preview, "Hello, world!");
    }

    #[test]
    fn test_content_preview_truncation() {
        let content = Content::text("This is a very long text that should be truncated");
        let preview = content_preview(&content, 20);
        assert!(preview.len() <= 23); // 20 + "..."
        assert!(preview.ends_with("..."));
    }

    #[test]
    fn test_content_preview_code() {
        let content = Content::code("rust", "fn main() {}");
        let preview = content_preview(&content, 100);
        assert!(preview.contains("rust"));
        assert!(preview.contains("fn main()"));
    }

    #[test]
    fn test_block_summary_from_block() {
        let block = Block::new(Content::text("Test content"), Some("paragraph"));
        let summary = BlockSummary::from_block(&block);

        assert!(!summary.id.is_empty());
        assert_eq!(summary.content_type, "text");
        assert_eq!(summary.role, "paragraph");
    }

    #[test]
    fn test_edge_summary() {
        use ucm_core::Edge;

        let source_id = BlockId::root();
        let target_id = BlockId::from_hex("aabbccddeeff001122334455").unwrap();
        let edge = Edge::new(ucm_core::EdgeType::References, target_id.clone());

        let summary = EdgeSummary::new(&source_id, &edge);

        assert_eq!(summary.source, source_id.to_string());
        assert_eq!(summary.target, target_id.to_string());
        assert!(summary.edge_type.contains("References"));
    }
}
