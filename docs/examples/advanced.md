# Advanced Examples

This document provides advanced examples demonstrating complex UCP usage patterns and integrations.

## Example 1: Building a Document Pipeline

```rust
use ucp_api::UcpClient;
use ucm_core::{Block, Content, Document, Edge, EdgeType};
use ucm_engine::{Engine, Operation};
use ucp_translator_markdown::parse_markdown;

/// A document processing pipeline
struct DocumentPipeline {
    client: UcpClient,
    engine: Engine,
    stages: Vec<Box<dyn PipelineStage>>,
}

trait PipelineStage {
    fn name(&self) -> &str;
    fn process(&self, doc: &mut Document) -> Result<(), String>;
}

struct AddMetadataStage {
    author: String,
    version: String,
}

impl PipelineStage for AddMetadataStage {
    fn name(&self) -> &str { "add-metadata" }
    
    fn process(&self, doc: &mut Document) -> Result<(), String> {
        doc.metadata.authors.push(self.author.clone());
        doc.metadata.custom.insert(
            "version".to_string(),
            serde_json::json!(self.version)
        );
        Ok(())
    }
}

struct TagCodeBlocksStage;

impl PipelineStage for TagCodeBlocksStage {
    fn name(&self) -> &str { "tag-code-blocks" }
    
    fn process(&self, doc: &mut Document) -> Result<(), String> {
        let code_ids: Vec<_> = doc.indices.find_by_type("code").iter().cloned().collect();
        
        for id in code_ids {
            if let Some(block) = doc.get_block_mut(&id) {
                if let Content::Code(code) = &block.content {
                    block.metadata.tags.push(format!("lang:{}", code.language));
                }
            }
        }
        
        doc.rebuild_indices();
        Ok(())
    }
}

struct ValidateStructureStage;

impl PipelineStage for ValidateStructureStage {
    fn name(&self) -> &str { "validate-structure" }
    
    fn process(&self, doc: &mut Document) -> Result<(), String> {
        let issues = doc.validate();
        let errors: Vec<_> = issues.iter()
            .filter(|i| i.severity == ucm_core::ValidationSeverity::Error)
            .collect();
        
        if !errors.is_empty() {
            return Err(format!("{} validation errors", errors.len()));
        }
        Ok(())
    }
}

impl DocumentPipeline {
    fn new() -> Self {
        Self {
            client: UcpClient::new(),
            engine: Engine::new(),
            stages: Vec::new(),
        }
    }
    
    fn add_stage(mut self, stage: Box<dyn PipelineStage>) -> Self {
        self.stages.push(stage);
        self
    }
    
    fn process(&self, doc: &mut Document) -> Result<(), String> {
        for stage in &self.stages {
            println!("Running stage: {}", stage.name());
            stage.process(doc)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create pipeline
    let pipeline = DocumentPipeline::new()
        .add_stage(Box::new(AddMetadataStage {
            author: "Pipeline".to_string(),
            version: "1.0.0".to_string(),
        }))
        .add_stage(Box::new(TagCodeBlocksStage))
        .add_stage(Box::new(ValidateStructureStage));
    
    // Process document
    let markdown = r#"
# Example Document

Some introduction text.

```rust
fn main() {
    println!("Hello!");
}
```

```python
def hello():
    print("Hello!")
```
"#;
    
    let mut doc = parse_markdown(markdown)?;
    pipeline.process(&mut doc)?;
    
    println!("\nProcessed document:");
    println!("  Authors: {:?}", doc.metadata.authors);
    println!("  Rust code blocks: {}", doc.indices.find_by_tag("lang:rust").len());
    println!("  Python code blocks: {}", doc.indices.find_by_tag("lang:python").len());
    
    Ok(())
}
```

## Example 2: Implementing a Document Diff

```rust
use ucm_core::{Block, BlockId, Content, Document};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum DiffChange {
    Added(BlockId),
    Removed(BlockId),
    Modified { id: BlockId, old_hash: String, new_hash: String },
    Moved { id: BlockId, old_parent: BlockId, new_parent: BlockId },
}

struct DocumentDiff {
    changes: Vec<DiffChange>,
}

impl DocumentDiff {
    fn compute(old: &Document, new: &Document) -> Self {
        let mut changes = Vec::new();
        
        let old_ids: HashSet<_> = old.blocks.keys().cloned().collect();
        let new_ids: HashSet<_> = new.blocks.keys().cloned().collect();
        
        // Find added blocks
        for id in new_ids.difference(&old_ids) {
            changes.push(DiffChange::Added(id.clone()));
        }
        
        // Find removed blocks
        for id in old_ids.difference(&new_ids) {
            changes.push(DiffChange::Removed(id.clone()));
        }
        
        // Find modified and moved blocks
        for id in old_ids.intersection(&new_ids) {
            let old_block = old.get_block(id).unwrap();
            let new_block = new.get_block(id).unwrap();
            
            // Check content change
            let old_hash = format!("{:?}", old_block.metadata.content_hash);
            let new_hash = format!("{:?}", new_block.metadata.content_hash);
            
            if old_hash != new_hash {
                changes.push(DiffChange::Modified {
                    id: id.clone(),
                    old_hash,
                    new_hash,
                });
            }
            
            // Check parent change
            let old_parent = old.parent(id);
            let new_parent = new.parent(id);
            
            if old_parent != new_parent {
                if let (Some(op), Some(np)) = (old_parent, new_parent) {
                    changes.push(DiffChange::Moved {
                        id: id.clone(),
                        old_parent: op.clone(),
                        new_parent: np.clone(),
                    });
                }
            }
        }
        
        Self { changes }
    }
    
    fn summary(&self) -> String {
        let added = self.changes.iter().filter(|c| matches!(c, DiffChange::Added(_))).count();
        let removed = self.changes.iter().filter(|c| matches!(c, DiffChange::Removed(_))).count();
        let modified = self.changes.iter().filter(|c| matches!(c, DiffChange::Modified { .. })).count();
        let moved = self.changes.iter().filter(|c| matches!(c, DiffChange::Moved { .. })).count();
        
        format!(
            "+{} -{} ~{} >{} (added/removed/modified/moved)",
            added, removed, modified, moved
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create original document
    let mut doc1 = Document::create();
    let root = doc1.root.clone();
    
    let block1 = Block::new(Content::text("Original content"), Some("paragraph"));
    let block1_id = doc1.add_block(block1, &root)?;
    
    let block2 = Block::new(Content::text("Will be removed"), Some("paragraph"));
    let block2_id = doc1.add_block(block2, &root)?;
    
    // Create modified document
    let mut doc2 = Document::create();
    let root2 = doc2.root.clone();
    
    // Modified block1
    let block1_mod = Block::new(Content::text("Modified content"), Some("paragraph"));
    doc2.add_block(block1_mod, &root2)?;
    
    // New block
    let block3 = Block::new(Content::text("New block"), Some("paragraph"));
    doc2.add_block(block3, &root2)?;
    
    // Compute diff
    let diff = DocumentDiff::compute(&doc1, &doc2);
    
    println!("Document diff: {}", diff.summary());
    println!("\nChanges:");
    for change in &diff.changes {
        println!("  {:?}", change);
    }
    
    Ok(())
}
```

## Example 3: Building a Query Language

```rust
use ucm_core::{Block, BlockId, Content, Document};

#[derive(Debug, Clone)]
enum Query {
    All,
    ByType(String),
    ByTag(String),
    ByLabel(String),
    ByRole(String),
    And(Box<Query>, Box<Query>),
    Or(Box<Query>, Box<Query>),
    Not(Box<Query>),
    HasChild(Box<Query>),
    HasParent(Box<Query>),
}

struct QueryEngine;

impl QueryEngine {
    fn execute(doc: &Document, query: &Query) -> Vec<BlockId> {
        let all_ids: Vec<_> = doc.blocks.keys().cloned().collect();
        
        all_ids.into_iter()
            .filter(|id| Self::matches(doc, id, query))
            .collect()
    }
    
    fn matches(doc: &Document, id: &BlockId, query: &Query) -> bool {
        let block = match doc.get_block(id) {
            Some(b) => b,
            None => return false,
        };
        
        match query {
            Query::All => true,
            
            Query::ByType(t) => block.content_type() == t,
            
            Query::ByTag(tag) => block.has_tag(tag),
            
            Query::ByLabel(label) => {
                block.metadata.label.as_ref() == Some(label)
            }
            
            Query::ByRole(role) => {
                block.metadata.semantic_role
                    .as_ref()
                    .map(|r| r.category.as_str() == role)
                    .unwrap_or(false)
            }
            
            Query::And(a, b) => {
                Self::matches(doc, id, a) && Self::matches(doc, id, b)
            }
            
            Query::Or(a, b) => {
                Self::matches(doc, id, a) || Self::matches(doc, id, b)
            }
            
            Query::Not(q) => !Self::matches(doc, id, q),
            
            Query::HasChild(child_query) => {
                doc.children(id).iter().any(|child_id| {
                    Self::matches(doc, child_id, child_query)
                })
            }
            
            Query::HasParent(parent_query) => {
                doc.parent(id)
                    .map(|parent_id| Self::matches(doc, parent_id, parent_query))
                    .unwrap_or(false)
            }
        }
    }
}

// Query builder for ergonomic API
struct QueryBuilder;

impl QueryBuilder {
    fn all() -> Query { Query::All }
    fn by_type(t: &str) -> Query { Query::ByType(t.to_string()) }
    fn by_tag(t: &str) -> Query { Query::ByTag(t.to_string()) }
    fn by_label(l: &str) -> Query { Query::ByLabel(l.to_string()) }
    fn by_role(r: &str) -> Query { Query::ByRole(r.to_string()) }
    
    fn and(a: Query, b: Query) -> Query { Query::And(Box::new(a), Box::new(b)) }
    fn or(a: Query, b: Query) -> Query { Query::Or(Box::new(a), Box::new(b)) }
    fn not(q: Query) -> Query { Query::Not(Box::new(q)) }
    
    fn has_child(q: Query) -> Query { Query::HasChild(Box::new(q)) }
    fn has_parent(q: Query) -> Query { Query::HasParent(Box::new(q)) }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Build document
    let section = Block::new(Content::text("Section"), Some("heading2"))
        .with_tag("important");
    let section_id = doc.add_block(section, &root)?;
    
    let para = Block::new(Content::text("Paragraph"), Some("paragraph"))
        .with_tag("draft");
    doc.add_block(para, &section_id)?;
    
    let code = Block::new(Content::code("rust", "fn main() {}"), Some("code"))
        .with_tag("example");
    doc.add_block(code, &section_id)?;
    
    // Execute queries
    use QueryBuilder as Q;
    
    // Find all code blocks
    let code_blocks = QueryEngine::execute(&doc, &Q::by_type("code"));
    println!("Code blocks: {}", code_blocks.len());
    
    // Find important blocks
    let important = QueryEngine::execute(&doc, &Q::by_tag("important"));
    println!("Important blocks: {}", important.len());
    
    // Find blocks that are either code or have 'draft' tag
    let query = Q::or(Q::by_type("code"), Q::by_tag("draft"));
    let results = QueryEngine::execute(&doc, &query);
    println!("Code OR draft: {}", results.len());
    
    // Find sections that have code children
    let query = Q::and(
        Q::by_role("heading2"),
        Q::has_child(Q::by_type("code"))
    );
    let sections_with_code = QueryEngine::execute(&doc, &query);
    println!("Sections with code: {}", sections_with_code.len());
    
    Ok(())
}
```

## Example 4: Implementing Document Templates

```rust
use ucm_core::{Block, Content, Document, BlockId};
use std::collections::HashMap;

struct DocumentTemplate {
    name: String,
    structure: Vec<TemplateNode>,
}

struct TemplateNode {
    role: String,
    content_type: String,
    placeholder: Option<String>,
    children: Vec<TemplateNode>,
}

impl DocumentTemplate {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            structure: Vec::new(),
        }
    }
    
    fn add_node(mut self, node: TemplateNode) -> Self {
        self.structure.push(node);
        self
    }
    
    fn instantiate(&self, values: &HashMap<String, String>) -> Result<Document, String> {
        let mut doc = Document::create();
        let root = doc.root.clone();
        
        for node in &self.structure {
            Self::instantiate_node(&mut doc, &root, node, values)?;
        }
        
        Ok(doc)
    }
    
    fn instantiate_node(
        doc: &mut Document,
        parent: &BlockId,
        node: &TemplateNode,
        values: &HashMap<String, String>,
    ) -> Result<BlockId, String> {
        let content_text = if let Some(placeholder) = &node.placeholder {
            values.get(placeholder)
                .cloned()
                .unwrap_or_else(|| format!("[{}]", placeholder))
        } else {
            String::new()
        };
        
        let content = match node.content_type.as_str() {
            "text" => Content::text(&content_text),
            "code" => Content::code("text", &content_text),
            _ => Content::text(&content_text),
        };
        
        let block = Block::new(content, Some(&node.role));
        let block_id = doc.add_block(block, parent).map_err(|e| e.to_string())?;
        
        for child in &node.children {
            Self::instantiate_node(doc, &block_id, child, values)?;
        }
        
        Ok(block_id)
    }
}

impl TemplateNode {
    fn new(role: &str, content_type: &str) -> Self {
        Self {
            role: role.to_string(),
            content_type: content_type.to_string(),
            placeholder: None,
            children: Vec::new(),
        }
    }
    
    fn with_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = Some(placeholder.to_string());
        self
    }
    
    fn with_child(mut self, child: TemplateNode) -> Self {
        self.children.push(child);
        self
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define a blog post template
    let template = DocumentTemplate::new("blog-post")
        .add_node(
            TemplateNode::new("title", "text")
                .with_placeholder("title")
        )
        .add_node(
            TemplateNode::new("heading2", "text")
                .with_placeholder("intro_heading")
                .with_child(
                    TemplateNode::new("paragraph", "text")
                        .with_placeholder("intro_content")
                )
        )
        .add_node(
            TemplateNode::new("heading2", "text")
                .with_placeholder("main_heading")
                .with_child(
                    TemplateNode::new("paragraph", "text")
                        .with_placeholder("main_content")
                )
                .with_child(
                    TemplateNode::new("code", "code")
                        .with_placeholder("code_example")
                )
        )
        .add_node(
            TemplateNode::new("heading2", "text")
                .with_placeholder("conclusion_heading")
                .with_child(
                    TemplateNode::new("paragraph", "text")
                        .with_placeholder("conclusion_content")
                )
        );
    
    // Instantiate with values
    let mut values = HashMap::new();
    values.insert("title".to_string(), "My Blog Post".to_string());
    values.insert("intro_heading".to_string(), "Introduction".to_string());
    values.insert("intro_content".to_string(), "Welcome to my blog post.".to_string());
    values.insert("main_heading".to_string(), "Main Content".to_string());
    values.insert("main_content".to_string(), "Here's the main content.".to_string());
    values.insert("code_example".to_string(), "fn example() {}".to_string());
    values.insert("conclusion_heading".to_string(), "Conclusion".to_string());
    values.insert("conclusion_content".to_string(), "Thanks for reading!".to_string());
    
    let doc = template.instantiate(&values)?;
    
    println!("Created document from template:");
    println!("  Blocks: {}", doc.block_count());
    
    // Render to markdown
    let markdown = ucp_translator_markdown::render_markdown(&doc)?;
    println!("\n{}", markdown);
    
    Ok(())
}
```

## Example 5: Building a Document Index

```rust
use ucm_core::{Block, BlockId, Content, Document};
use std::collections::HashMap;

/// Full-text search index for documents
struct DocumentIndex {
    /// Word -> (BlockId, frequency)
    word_index: HashMap<String, Vec<(BlockId, u32)>>,
    /// BlockId -> word count
    block_lengths: HashMap<BlockId, u32>,
    /// Total documents indexed
    total_blocks: u32,
}

impl DocumentIndex {
    fn new() -> Self {
        Self {
            word_index: HashMap::new(),
            block_lengths: HashMap::new(),
            total_blocks: 0,
        }
    }
    
    fn index_document(&mut self, doc: &Document) {
        for (id, block) in &doc.blocks {
            self.index_block(id, block);
        }
    }
    
    fn index_block(&mut self, id: &BlockId, block: &Block) {
        let text = Self::extract_text(&block.content);
        let words = Self::tokenize(&text);
        
        let mut word_freq: HashMap<String, u32> = HashMap::new();
        for word in &words {
            *word_freq.entry(word.clone()).or_insert(0) += 1;
        }
        
        for (word, freq) in word_freq {
            self.word_index
                .entry(word)
                .or_insert_with(Vec::new)
                .push((id.clone(), freq));
        }
        
        self.block_lengths.insert(id.clone(), words.len() as u32);
        self.total_blocks += 1;
    }
    
    fn search(&self, query: &str) -> Vec<(BlockId, f64)> {
        let query_words = Self::tokenize(query);
        let mut scores: HashMap<BlockId, f64> = HashMap::new();
        
        for word in &query_words {
            if let Some(postings) = self.word_index.get(word) {
                let idf = (self.total_blocks as f64 / postings.len() as f64).ln();
                
                for (block_id, freq) in postings {
                    let block_len = self.block_lengths.get(block_id).copied().unwrap_or(1) as f64;
                    let tf = *freq as f64 / block_len;
                    let score = tf * idf;
                    
                    *scores.entry(block_id.clone()).or_insert(0.0) += score;
                }
            }
        }
        
        let mut results: Vec<_> = scores.into_iter().collect();
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        results
    }
    
    fn extract_text(content: &Content) -> String {
        match content {
            Content::Text(t) => t.text.clone(),
            Content::Code(c) => c.source.clone(),
            Content::Table(t) => {
                t.rows.iter()
                    .flat_map(|r| r.cells.iter())
                    .filter_map(|c| match c {
                        ucm_core::Cell::Text(s) => Some(s.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            }
            _ => String::new(),
        }
    }
    
    fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty() && s.len() > 2)
            .map(String::from)
            .collect()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Add searchable content
    let blocks = vec![
        ("Rust Programming", "Rust is a systems programming language focused on safety."),
        ("Memory Safety", "Rust provides memory safety without garbage collection."),
        ("Concurrency", "Rust enables fearless concurrency through ownership."),
        ("Performance", "Rust offers zero-cost abstractions for high performance."),
        ("WebAssembly", "Rust compiles to WebAssembly for web applications."),
    ];
    
    for (title, content) in blocks {
        let heading = Block::new(Content::text(title), Some("heading2"));
        let heading_id = doc.add_block(heading, &root)?;
        
        let para = Block::new(Content::text(content), Some("paragraph"));
        doc.add_block(para, &heading_id)?;
    }
    
    // Build index
    let mut index = DocumentIndex::new();
    index.index_document(&doc);
    
    // Search
    let queries = vec!["memory safety", "programming", "web", "performance"];
    
    for query in queries {
        println!("\nSearch: '{}'", query);
        let results = index.search(query);
        
        for (id, score) in results.iter().take(3) {
            if let Some(block) = doc.get_block(id) {
                let preview = match &block.content {
                    Content::Text(t) => t.text.chars().take(50).collect::<String>(),
                    _ => "[non-text]".to_string(),
                };
                println!("  {:.3}: {}...", score, preview);
            }
        }
    }
    
    Ok(())
}
```

## Example 6: Event-Driven Document Processing

```rust
use ucm_core::{Block, BlockId, Content, Document};
use std::sync::mpsc::{channel, Sender, Receiver};

#[derive(Debug, Clone)]
enum DocumentEvent {
    BlockAdded { id: BlockId, parent: BlockId },
    BlockRemoved { id: BlockId },
    BlockModified { id: BlockId },
    EdgeAdded { source: BlockId, target: BlockId },
    DocumentValidated { valid: bool, issue_count: usize },
}

trait EventHandler: Send {
    fn handle(&self, event: &DocumentEvent);
}

struct LoggingHandler;

impl EventHandler for LoggingHandler {
    fn handle(&self, event: &DocumentEvent) {
        println!("[LOG] {:?}", event);
    }
}

struct MetricsHandler {
    sender: Sender<DocumentEvent>,
}

impl EventHandler for MetricsHandler {
    fn handle(&self, event: &DocumentEvent) {
        let _ = self.sender.send(event.clone());
    }
}

struct EventEmitter {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl EventEmitter {
    fn new() -> Self {
        Self { handlers: Vec::new() }
    }
    
    fn add_handler(&mut self, handler: Box<dyn EventHandler>) {
        self.handlers.push(handler);
    }
    
    fn emit(&self, event: DocumentEvent) {
        for handler in &self.handlers {
            handler.handle(&event);
        }
    }
}

struct ObservableDocument {
    doc: Document,
    emitter: EventEmitter,
}

impl ObservableDocument {
    fn new(emitter: EventEmitter) -> Self {
        Self {
            doc: Document::create(),
            emitter,
        }
    }
    
    fn add_block(&mut self, block: Block, parent: &BlockId) -> Result<BlockId, String> {
        let id = self.doc.add_block(block, parent).map_err(|e| e.to_string())?;
        
        self.emitter.emit(DocumentEvent::BlockAdded {
            id: id.clone(),
            parent: parent.clone(),
        });
        
        Ok(id)
    }
    
    fn delete_block(&mut self, id: &BlockId) -> Result<(), String> {
        self.doc.delete_block(id).map_err(|e| e.to_string())?;
        
        self.emitter.emit(DocumentEvent::BlockRemoved {
            id: id.clone(),
        });
        
        Ok(())
    }
    
    fn validate(&self) -> bool {
        let issues = self.doc.validate();
        let valid = issues.iter().all(|i| i.severity != ucm_core::ValidationSeverity::Error);
        
        self.emitter.emit(DocumentEvent::DocumentValidated {
            valid,
            issue_count: issues.len(),
        });
        
        valid
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up event handling
    let (tx, rx) = channel();
    
    let mut emitter = EventEmitter::new();
    emitter.add_handler(Box::new(LoggingHandler));
    emitter.add_handler(Box::new(MetricsHandler { sender: tx }));
    
    // Create observable document
    let mut doc = ObservableDocument::new(emitter);
    let root = doc.doc.root.clone();
    
    // Perform operations (events will be emitted)
    let block1 = Block::new(Content::text("First block"), Some("paragraph"));
    let id1 = doc.add_block(block1, &root)?;
    
    let block2 = Block::new(Content::text("Second block"), Some("paragraph"));
    let id2 = doc.add_block(block2, &root)?;
    
    doc.validate();
    
    doc.delete_block(&id2)?;
    
    // Process received events
    println!("\nReceived events:");
    while let Ok(event) = rx.try_recv() {
        println!("  {:?}", event);
    }
    
    Ok(())
}
```

## Example 7: Multi-Format Export

```rust
use ucm_core::{Block, Content, Document};
use ucp_translator_markdown::render_markdown;

trait DocumentExporter {
    fn export(&self, doc: &Document) -> Result<String, String>;
    fn format_name(&self) -> &str;
}

struct MarkdownExporter;

impl DocumentExporter for MarkdownExporter {
    fn export(&self, doc: &Document) -> Result<String, String> {
        render_markdown(doc).map_err(|e| e.to_string())
    }
    
    fn format_name(&self) -> &str { "markdown" }
}

struct HtmlExporter;

impl DocumentExporter for HtmlExporter {
    fn export(&self, doc: &Document) -> Result<String, String> {
        let mut html = String::from("<!DOCTYPE html>\n<html>\n<head><title>Document</title></head>\n<body>\n");
        
        Self::render_block(doc, &doc.root, &mut html, 0);
        
        html.push_str("</body>\n</html>");
        Ok(html)
    }
    
    fn format_name(&self) -> &str { "html" }
}

impl HtmlExporter {
    fn render_block(doc: &Document, id: &ucm_core::BlockId, html: &mut String, depth: usize) {
        if let Some(block) = doc.get_block(id) {
            if !block.is_root() {
                let role = block.metadata.semantic_role
                    .as_ref()
                    .map(|r| r.category.as_str())
                    .unwrap_or("div");
                
                match &block.content {
                    Content::Text(t) => {
                        let tag = match role {
                            "heading1" => "h1",
                            "heading2" => "h2",
                            "heading3" => "h3",
                            "paragraph" => "p",
                            "quote" => "blockquote",
                            _ => "div",
                        };
                        html.push_str(&format!("<{}>{}</{}>\n", tag, t.text, tag));
                    }
                    Content::Code(c) => {
                        html.push_str(&format!(
                            "<pre><code class=\"language-{}\">{}</code></pre>\n",
                            c.language, c.source
                        ));
                    }
                    _ => {}
                }
            }
        }
        
        if let Some(children) = doc.structure.get(id) {
            for child in children {
                Self::render_block(doc, child, html, depth + 1);
            }
        }
    }
}

struct JsonExporter;

impl DocumentExporter for JsonExporter {
    fn export(&self, doc: &Document) -> Result<String, String> {
        serde_json::to_string_pretty(doc).map_err(|e| e.to_string())
    }
    
    fn format_name(&self) -> &str { "json" }
}

struct ExportManager {
    exporters: Vec<Box<dyn DocumentExporter>>,
}

impl ExportManager {
    fn new() -> Self {
        Self { exporters: Vec::new() }
    }
    
    fn register(mut self, exporter: Box<dyn DocumentExporter>) -> Self {
        self.exporters.push(exporter);
        self
    }
    
    fn export(&self, doc: &Document, format: &str) -> Result<String, String> {
        self.exporters
            .iter()
            .find(|e| e.format_name() == format)
            .ok_or_else(|| format!("Unknown format: {}", format))?
            .export(doc)
    }
    
    fn export_all(&self, doc: &Document) -> Vec<(String, Result<String, String>)> {
        self.exporters
            .iter()
            .map(|e| (e.format_name().to_string(), e.export(doc)))
            .collect()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create document
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    let title = Block::new(Content::text("My Document"), Some("heading1"));
    let title_id = doc.add_block(title, &root)?;
    
    let para = Block::new(Content::text("This is a paragraph."), Some("paragraph"));
    doc.add_block(para, &title_id)?;
    
    let code = Block::new(Content::code("rust", "fn main() {}"), Some("code"));
    doc.add_block(code, &title_id)?;
    
    // Set up export manager
    let manager = ExportManager::new()
        .register(Box::new(MarkdownExporter))
        .register(Box::new(HtmlExporter))
        .register(Box::new(JsonExporter));
    
    // Export to all formats
    for (format, result) in manager.export_all(&doc) {
        println!("=== {} ===", format.to_uppercase());
        match result {
            Ok(content) => println!("{}\n", &content[..content.len().min(500)]),
            Err(e) => println!("Error: {}\n", e),
        }
    }
    
    Ok(())
}
```

## Example 8: Undoable Section Replace (Rust & Python)

This scenario shows how to replace a section's content from Markdown while keeping a restore payload that can be replayed later.

```rust
use ucm_core::{Content, Document};
use ucm_engine::{Engine, Operation};
use ucm_engine::section::{clear_section_content_with_undo, restore_deleted_content};
use ucp_translator_markdown::parse_markdown;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Document::create();
    let root = doc.root.clone();

    // Seed a section
    let section = doc.add_block(Content::text("Chapter 1"), Some("heading1"), &root)?;
    doc.add_block(Content::text("Legacy paragraph"), Some("paragraph"), &section)?;

    // Capture existing content before replacement
    let ClearSectionResult { removed_ids, deleted_content } =
        clear_section_content_with_undo(&mut doc, &section)?;
    assert!(!removed_ids.is_empty());

    // Integrate new markdown using the WriteSection operation
    let markdown = "## New Intro\\n\\nFresh content.".to_string();
    let mut engine = Engine::new();
    engine.execute(&mut doc, Operation::WriteSection {
        section_id: section.clone(),
        markdown,
        base_heading_level: Some(1),
    })?;

    // ... later, roll back to the snapshot
    let restored = restore_deleted_content(&mut doc, &deleted_content)?;
    assert_eq!(restored.len(), removed_ids.len());
    Ok(())
}
```

```python
import ucp

doc = ucp.parse("""# Chapter 1\n\nLegacy paragraph\n""")
section_id = ucp.find_section_by_path(doc, "Chapter 1")

snapshot = ucp.clear_section_with_undo(doc, section_id)
ucp.write_section(doc, section_id, "## New Intro\n\nFresh content", base_heading_level=1)

# ... persist snapshot.deleted_content somewhere durable ...

ucp.restore_deleted_section(doc, snapshot.deleted_content)
assert ucp.find_section_by_path(doc, "Chapter 1 > Legacy paragraph")
```

## Example 9: Context Window Management with Traversal (Python)

Combine the traversal engine with the context manager to collect relevant blocks, curate them, and render an LLM-ready prompt.

```python
from ucp import (
    create, add_block, find_section_by_path,
    TraversalEngine, TraversalFilter, NavigateDirection,
    create_context, ExpandDirection, CompressionMethod,
)

doc = create()
root = doc.root_id
chapter = add_block(doc, root, "Chapter 1", role="heading1")
section = add_block(doc, chapter, "1.1 Overview", role="heading2")
para = add_block(doc, section, "This section introduces the topic.", role="paragraph")

# Traverse downward two levels starting at Chapter 1
engine = TraversalEngine()
nodes = engine.traverse(
    doc,
    start_id=chapter,
    direction=NavigateDirection.BREADTH_FIRST,
    max_depth=2,
    filter=TraversalFilter(include_roles=["heading", "paragraph"]),
)

print(f"Collected {len(nodes.nodes)} nodes for consideration")

# Build a context window around the same section
ctx = create_context("analysis", max_tokens=2000)
ctx.initialize_focus(doc, section, "Summarize the overview")
ctx.expand_context(doc, ExpandDirection.DOWN, depth=2)

# Drop any block you don't want to keep
for block_id in list(ctx.window.blocks.keys()):
    meta = ctx.window.blocks[block_id]
    if meta.relevance_score < 0.2:
        ctx.remove_block(block_id)

# Compress if over budget, then render for the LLM
ctx.compress(doc, CompressionMethod.TRUNCATE)
prompt = ctx.render_for_prompt(doc)
print(prompt)
```

## Example 10: HTML Ingestion Pipeline

Use the HTML translator to harvest structured content from arbitrary markup, normalize heading depth, and append it under an existing section.

```rust
use ucm_core::{Content, Document};
use ucp_translator_html::{HtmlParser, ParseConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Document::create();
    let root = doc.root.clone();
    let appendix = doc.add_block(Content::text("Appendix"), Some("heading1"), &root)?;

    let html = r#"
    <article>
        <h1>Release Notes</h1>
        <p>Highlights of the latest release.</p>
        <h2>Bug Fixes</h2>
        <ul><li>Fixed context window bug</li><li>Improved traversal speed</li></ul>
    </article>
    "#;

    let parser = HtmlParser::new(ParseConfig {
        base_heading_level: Some(2), // slot beneath Appendix (H1)
        denied_nodes: Some(vec!["script", "style"]),
        capture_attributes: true,
        ..Default::default()
    });

    let imported = parser.parse(html)?;

    // Integrate imported doc as a subtree under Appendix
    for child in imported.children(&imported.root) {
        doc.clone_subtree_from(&imported, child, &appendix)?;
    }

    println!("Appendix now has {} children", doc.children(&appendix).len());
    Ok(())
}
```

## See Also

- [Basic Examples](./basic.md) - Getting started examples
- [Intermediate Examples](./intermediate.md) - More complex scenarios
- [UCM Core](../ucm-core/README.md) - Core types reference
- [UCM Engine](../ucm-engine/README.md) - Engine documentation
