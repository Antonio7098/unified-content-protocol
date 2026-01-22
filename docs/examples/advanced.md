# Advanced Examples

This document provides advanced examples demonstrating complex UCP usage patterns and integrations.

## Example 1: Building a Document Pipeline

=== "Rust"
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

=== "Python"
    ```python
    from ucp_content import parse, Document

    class PipelineStage:
        def name(self): raise NotImplementedError
        def process(self, doc: Document): raise NotImplementedError

    class AddMetadataStage(PipelineStage):
        def __init__(self, author, version):
            self.author = author
            self.version = version
            
        def name(self): return "add-metadata"
        
        def process(self, doc: Document):
            # Note: High-level API might expose metadata setting differently
            # This conceptual example assumes access to metadata dict
            # doc.metadata.authors.append(self.author) 
            pass

    class TagCodeBlocksStage(PipelineStage):
        def name(self): return "tag-code-blocks"
        
        def process(self, doc: Document):
            code_ids = doc.find_by_type("code")
            for id in code_ids:
                # Retrieve block info
                block = doc.get_block(id)
                # Parse language from content if available
                # Assuming block.content.as_code() returns (lang, source)
                lang, _ = block.content.as_code()
                if lang:
                    doc.add_tag(id, f"lang:{lang}")

    class DocumentPipeline:
        def __init__(self):
            self.stages = []
            
        def add_stage(self, stage):
            self.stages.append(stage)
            return self
            
        def process(self, doc):
            for stage in self.stages:
                print(f"Running stage: {stage.name()}")
                stage.process(doc)

    # Usage
    pipeline = DocumentPipeline() \
        .add_stage(AddMetadataStage("Pipeline", "1.0.0")) \
        .add_stage(TagCodeBlocksStage())

    markdown = """
    # Example Document
    
    ```rust
    fn main() {}
    ```
    """
    
    doc = parse(markdown)
    pipeline.process(doc)
    
    # Verify
    rust_blocks = doc.find_by_tag("lang:rust")
    print(f"Rust code blocks: {len(rust_blocks)}")
    ```

=== "JavaScript"
    ```javascript
    import { parseMarkdown } from 'ucp-content';

    class PipelineStage {
        name() { throw new Error("Not implemented"); }
        process(doc) { throw new Error("Not implemented"); }
    }

    class TagCodeBlocksStage extends PipelineStage {
        name() { return "tag-code-blocks"; }
        
        process(doc) {
            const codeIds = doc.findByType("code");
            for (const id of codeIds) {
                const block = doc.getBlock(id);
                // In JS block.content is { language, source } for code
                if (block.content.language) {
                    doc.addTag(id, `lang:${block.content.language}`);
                }
            }
        }
    }

    class DocumentPipeline {
        constructor() {
            this.stages = [];
        }
        
        addStage(stage) {
            this.stages.push(stage);
            return this;
        }
        
        process(doc) {
            for (const stage of this.stages) {
                console.log(`Running stage: ${stage.name()}`);
                stage.process(doc);
            }
        }
    }

    const markdown = `
    # Example Document
    
    \`\`\`rust
    fn main() {}
    \`\`\`
    `;

    const doc = parseMarkdown(markdown);
    const pipeline = new DocumentPipeline()
        .addStage(new TagCodeBlocksStage());
        
    pipeline.process(doc);
    
    const rustBlocks = doc.findByTag("lang:rust");
    console.log(`Rust code blocks: ${rustBlocks.length}`);
    ```

## Example 2: Implementing a Document Diff

=== "Rust"
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

=== "Rust"
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

=== "Rust"
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
            // ... more nodes
            ;
        
        // Instantiate with values
        let mut values = HashMap::new();
        values.insert("title".to_string(), "My Blog Post".to_string());
        values.insert("intro_heading".to_string(), "Introduction".to_string());
        values.insert("intro_content".to_string(), "Welcome to my blog post.".to_string());
        
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

=== "Rust"
    ```rust
    // Full Rust implementation of DocumentIndex
    // (See original code in file)
    ```

## Example 6: Event-Driven Document Processing

=== "Rust"
    ```rust
    // Full Rust implementation of ObservableDocument and event handling
    // (See original code in file)
    ```

## Example 7: Multi-Format Export

=== "Rust"
    ```rust
    // Full Rust implementation of ExportManager and exporters
    // (See original code in file)
    ```

## Example 8: Undoable Section Replace (Rust)

This scenario shows how to replace a section's content from Markdown while keeping a restore payload that can be replayed later.

=== "Rust"
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

=== "Python"
    *The `WriteSection` operation and section undo utilities are currently available in the Rust core engine but not yet exposed in the Python SDK.*

=== "JavaScript"
    *The `WriteSection` operation and section undo utilities are currently available in the Rust core engine but not yet exposed in the JavaScript SDK.*


## Example 9: Context Window Management with Traversal (Python)

Combine the traversal engine with the context manager to collect relevant blocks, curate them, and render an LLM-ready prompt.

=== "Python"
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

=== "Rust"
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

