# Intermediate Examples

This document provides intermediate examples demonstrating more complex UCP usage patterns.

## Example 1: Document Transformation with UCL

```rust
use ucp_api::UcpClient;
use ucm_core::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UcpClient::new();
    let mut doc = client.create_document();
    
    // Build initial structure
    client.execute_ucl(&mut doc, r#"
        APPEND blk_ff00000000000000000000 text WITH role="heading1" label="title" :: "Technical Guide"
        APPEND blk_ff00000000000000000000 text WITH role="heading2" label="intro" :: "Introduction"
        APPEND blk_ff00000000000000000000 text WITH role="heading2" label="setup" :: "Setup"
        APPEND blk_ff00000000000000000000 text WITH role="heading2" label="usage" :: "Usage"
    "#)?;
    
    // Add content under sections
    if let Some(intro_id) = doc.indices.find_by_label("intro") {
        client.execute_ucl(&mut doc, &format!(r#"
            APPEND {} text WITH role="paragraph" :: "Welcome to this technical guide."
            APPEND {} text WITH role="paragraph" :: "This guide covers installation and usage."
        "#, intro_id, intro_id))?;
    }
    
    if let Some(setup_id) = doc.indices.find_by_label("setup") {
        client.execute_ucl(&mut doc, &format!(r#"
            APPEND {} text WITH role="paragraph" :: "Follow these steps to set up:"
            APPEND {} code WITH lang="bash" :: "cargo add my-crate"
        "#, setup_id, setup_id))?;
    }
    
    // Transform: Add tags to all paragraphs
    for block_id in doc.indices.find_by_type("text").clone() {
        if let Some(block) = doc.get_block(&block_id) {
            if block.metadata.semantic_role
                .as_ref()
                .map(|r| r.category.as_str() == "paragraph")
                .unwrap_or(false)
            {
                client.execute_ucl(&mut doc, &format!(
                    r#"EDIT {} SET metadata.tags += ["content"]"#,
                    block_id
                ))?;
            }
        }
    }
    
    println!("Document has {} blocks", doc.block_count());
    println!("Content-tagged blocks: {}", doc.indices.find_by_tag("content").len());
    
    Ok(())
}
```

## Example 2: Working with Edges and Relationships

```rust
use ucm_core::{Block, Content, Document, Edge, EdgeType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Create a claim
    let claim = Block::new(
        Content::text("Rust provides memory safety without garbage collection."),
        Some("body.argument")
    ).with_label("main-claim");
    let claim_id = doc.add_block(claim, &root)?;
    
    // Create supporting evidence
    let evidence1 = Block::new(
        Content::text("The borrow checker enforces ownership rules at compile time."),
        Some("body.evidence")
    ).with_label("evidence-1");
    let evidence1_id = doc.add_block(evidence1, &root)?;
    
    let evidence2 = Block::new(
        Content::text("Lifetimes ensure references are always valid."),
        Some("body.evidence")
    ).with_label("evidence-2");
    let evidence2_id = doc.add_block(evidence2, &root)?;
    
    // Create counterargument
    let counter = Block::new(
        Content::text("Unsafe blocks can bypass safety guarantees."),
        Some("body.counterargument")
    ).with_label("counter-1");
    let counter_id = doc.add_block(counter, &root)?;
    
    // Create rebuttal
    let rebuttal = Block::new(
        Content::text("Unsafe is explicit and auditable, unlike implicit unsafety."),
        Some("body.rebuttal")
    ).with_label("rebuttal-1");
    let rebuttal_id = doc.add_block(rebuttal, &root)?;
    
    // Add relationships
    // Evidence supports claim
    let edge1 = Edge::new(EdgeType::Supports, claim_id.clone())
        .with_confidence(0.95);
    doc.get_block_mut(&evidence1_id).unwrap().add_edge(edge1.clone());
    doc.edge_index.add_edge(&evidence1_id, &edge1);
    
    let edge2 = Edge::new(EdgeType::Supports, claim_id.clone())
        .with_confidence(0.90);
    doc.get_block_mut(&evidence2_id).unwrap().add_edge(edge2.clone());
    doc.edge_index.add_edge(&evidence2_id, &edge2);
    
    // Counter contradicts claim
    let edge3 = Edge::new(EdgeType::Contradicts, claim_id.clone())
        .with_confidence(0.7);
    doc.get_block_mut(&counter_id).unwrap().add_edge(edge3.clone());
    doc.edge_index.add_edge(&counter_id, &edge3);
    
    // Rebuttal contradicts counter
    let edge4 = Edge::new(EdgeType::Contradicts, counter_id.clone())
        .with_confidence(0.85);
    doc.get_block_mut(&rebuttal_id).unwrap().add_edge(edge4.clone());
    doc.edge_index.add_edge(&rebuttal_id, &edge4);
    
    // Query relationships
    println!("Blocks supporting the claim:");
    for source_id in doc.edge_index.incoming_of_type(&claim_id, &EdgeType::Supports) {
        if let Some(block) = doc.get_block(&source_id) {
            if let Content::Text(text) = &block.content {
                println!("  - {}", &text.text[..50.min(text.text.len())]);
            }
        }
    }
    
    println!("\nBlocks contradicting the claim:");
    for source_id in doc.edge_index.incoming_of_type(&claim_id, &EdgeType::Contradicts) {
        if let Some(block) = doc.get_block(&source_id) {
            if let Content::Text(text) = &block.content {
                println!("  - {}", &text.text[..50.min(text.text.len())]);
            }
        }
    }
    
    Ok(())
}
```

## Example 3: Transactions and Snapshots

```rust
use ucm_engine::{Engine, Operation};
use ucm_core::{Content, Document};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = Engine::new();
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Create initial snapshot
    engine.create_snapshot("initial", &doc, Some("Empty document".into()))?;
    
    // Transaction 1: Add structure
    let txn1 = engine.begin_named_transaction("add-chapters");
    
    engine.add_to_transaction(&txn1, Operation::Append {
        parent_id: root.clone(),
        content: Content::text("Chapter 1"),
        label: Some("ch1".into()),
        tags: vec!["chapter".into()],
        semantic_role: Some("heading1".into()),
        index: None,
    })?;
    
    engine.add_to_transaction(&txn1, Operation::Append {
        parent_id: root.clone(),
        content: Content::text("Chapter 2"),
        label: Some("ch2".into()),
        tags: vec!["chapter".into()],
        semantic_role: Some("heading1".into()),
        index: None,
    })?;
    
    let results = engine.commit_transaction(&txn1, &mut doc)?;
    println!("Transaction 1: Added {} blocks", results.len());
    
    // Snapshot after chapters
    engine.create_snapshot("with-chapters", &doc, Some("Added chapters".into()))?;
    
    // Transaction 2: Add content (will be rolled back)
    let txn2 = engine.begin_named_transaction("add-content");
    
    let ch1_id = doc.indices.find_by_label("ch1").unwrap();
    engine.add_to_transaction(&txn2, Operation::Append {
        parent_id: ch1_id,
        content: Content::text("Wrong content - will be rolled back"),
        label: None,
        tags: vec![],
        semantic_role: Some("paragraph".into()),
        index: None,
    })?;
    
    // Oops, wrong content - rollback
    engine.rollback_transaction(&txn2)?;
    println!("Transaction 2: Rolled back");
    
    // Transaction 3: Add correct content
    let txn3 = engine.begin_named_transaction("add-correct-content");
    
    engine.add_to_transaction(&txn3, Operation::Append {
        parent_id: ch1_id,
        content: Content::text("This is the correct introduction to Chapter 1."),
        label: None,
        tags: vec![],
        semantic_role: Some("paragraph".into()),
        index: None,
    })?;
    
    engine.commit_transaction(&txn3, &mut doc)?;
    println!("Transaction 3: Committed");
    
    // Final snapshot
    engine.create_snapshot("final", &doc, Some("Complete document".into()))?;
    
    // List snapshots
    println!("\nSnapshots:");
    for name in engine.list_snapshots() {
        println!("  - {}", name);
    }
    
    // Demonstrate restore
    println!("\nCurrent block count: {}", doc.block_count());
    
    let restored = engine.restore_snapshot("with-chapters")?;
    println!("After restore to 'with-chapters': {} blocks", restored.block_count());
    
    Ok(())
}
```

## Example 4: Custom Validation Rules

```rust
use ucm_core::{Document, Block, Content, ValidationIssue, ValidationSeverity, ErrorCode};
use ucm_engine::validate::{ValidationPipeline, ValidationResult};

fn validate_document_rules(doc: &Document) -> ValidationResult {
    let mut issues = Vec::new();
    
    // Rule 1: Document must have a title
    let has_title = doc.blocks.values().any(|b| {
        b.metadata.semantic_role
            .as_ref()
            .map(|r| r.category.as_str() == "title")
            .unwrap_or(false)
    });
    
    if !has_title {
        issues.push(ValidationIssue::warning(
            ErrorCode::E202InvalidStructure,
            "Document should have a title block".to_string(),
        ));
    }
    
    // Rule 2: All code blocks should have a language
    for block in doc.blocks.values() {
        if let Content::Code(code) = &block.content {
            if code.language.is_empty() || code.language == "text" {
                issues.push(ValidationIssue::warning(
                    ErrorCode::E200SchemaViolation,
                    format!("Code block {} has no language specified", block.id),
                ));
            }
        }
    }
    
    // Rule 3: Headings should not be empty
    for block in doc.blocks.values() {
        if let Some(role) = &block.metadata.semantic_role {
            if role.category.as_str().starts_with("heading") {
                if let Content::Text(text) = &block.content {
                    if text.text.trim().is_empty() {
                        issues.push(ValidationIssue::error(
                            ErrorCode::E200SchemaViolation,
                            format!("Heading block {} is empty", block.id),
                        ));
                    }
                }
            }
        }
    }
    
    // Rule 4: Labels should be unique (already enforced, but let's check)
    let mut labels = std::collections::HashSet::new();
    for block in doc.blocks.values() {
        if let Some(label) = &block.metadata.label {
            if !labels.insert(label.clone()) {
                issues.push(ValidationIssue::error(
                    ErrorCode::E200SchemaViolation,
                    format!("Duplicate label: {}", label),
                ));
            }
        }
    }
    
    ValidationResult::invalid(issues)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Add blocks (intentionally missing title)
    let heading = Block::new(Content::text(""), Some("heading1")); // Empty heading
    doc.add_block(heading, &root)?;
    
    let code = Block::new(Content::code("", "print('hello')"), Some("code")); // No language
    doc.add_block(code, &root)?;
    
    // Standard validation
    let standard_validator = ValidationPipeline::new();
    let mut result = standard_validator.validate_document(&doc);
    
    // Custom validation
    result.merge(validate_document_rules(&doc));
    
    println!("Validation Results:");
    println!("  Valid: {}", result.valid);
    println!("  Errors: {}", result.errors().len());
    println!("  Warnings: {}", result.warnings().len());
    
    for issue in &result.issues {
        println!("  [{:?}] {}", issue.severity, issue.message);
    }
    
    Ok(())
}
```

## Example 5: Document Merging

```rust
use ucm_core::{Block, Content, Document, BlockId};
use std::collections::HashMap;

fn merge_documents(target: &mut Document, source: &Document, parent_id: &BlockId) -> Result<HashMap<BlockId, BlockId>, String> {
    let mut id_mapping = HashMap::new();
    
    // Copy blocks from source, maintaining structure
    fn copy_subtree(
        target: &mut Document,
        source: &Document,
        source_id: &BlockId,
        target_parent: &BlockId,
        id_mapping: &mut HashMap<BlockId, BlockId>,
    ) -> Result<(), String> {
        if let Some(block) = source.get_block(source_id) {
            if block.is_root() {
                // Don't copy root, just process children
                if let Some(children) = source.structure.get(source_id) {
                    for child_id in children {
                        copy_subtree(target, source, child_id, target_parent, id_mapping)?;
                    }
                }
            } else {
                // Clone block and add to target
                let new_block = Block::new(block.content.clone(), 
                    block.metadata.semantic_role.as_ref().map(|r| r.category.as_str()));
                
                let new_id = target.add_block(new_block, target_parent)
                    .map_err(|e| e.to_string())?;
                
                id_mapping.insert(source_id.clone(), new_id.clone());
                
                // Copy children
                if let Some(children) = source.structure.get(source_id) {
                    for child_id in children {
                        copy_subtree(target, source, child_id, &new_id, id_mapping)?;
                    }
                }
            }
        }
        Ok(())
    }
    
    copy_subtree(target, source, &source.root, parent_id, &mut id_mapping)?;
    
    Ok(id_mapping)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create main document
    let mut main_doc = Document::create();
    let main_root = main_doc.root.clone();
    
    let title = Block::new(Content::text("Main Document"), Some("title"));
    let title_id = main_doc.add_block(title, &main_root)?;
    
    let section = Block::new(Content::text("Existing Section"), Some("heading2"));
    let section_id = main_doc.add_block(section, &title_id)?;
    
    // Create document to merge
    let mut merge_doc = Document::create();
    let merge_root = merge_doc.root.clone();
    
    let merge_section = Block::new(Content::text("Merged Section"), Some("heading2"));
    let merge_section_id = merge_doc.add_block(merge_section, &merge_root)?;
    
    let merge_para = Block::new(Content::text("Content from merged document."), Some("paragraph"));
    merge_doc.add_block(merge_para, &merge_section_id)?;
    
    let merge_code = Block::new(Content::code("rust", "fn merged() {}"), Some("code"));
    merge_doc.add_block(merge_code, &merge_section_id)?;
    
    println!("Before merge:");
    println!("  Main doc blocks: {}", main_doc.block_count());
    println!("  Merge doc blocks: {}", merge_doc.block_count());
    
    // Merge under title
    let id_mapping = merge_documents(&mut main_doc, &merge_doc, &title_id)?;
    
    println!("\nAfter merge:");
    println!("  Main doc blocks: {}", main_doc.block_count());
    println!("  Mapped {} block IDs", id_mapping.len());
    
    Ok(())
}
```

## Example 6: Token-Aware Document Processing

```rust
use ucm_core::{Block, Content, Document};
use ucm_core::metadata::{TokenEstimate, TokenModel};

fn estimate_document_tokens(doc: &Document, model: TokenModel) -> u32 {
    doc.blocks.values()
        .map(|b| b.token_estimate().for_model(model))
        .sum()
}

fn find_blocks_within_budget(
    doc: &Document,
    block_ids: &[ucm_core::BlockId],
    budget: u32,
    model: TokenModel,
) -> Vec<ucm_core::BlockId> {
    let mut result = Vec::new();
    let mut used = 0u32;
    
    for id in block_ids {
        if let Some(block) = doc.get_block(id) {
            let tokens = block.token_estimate().for_model(model);
            if used + tokens <= budget {
                result.push(id.clone());
                used += tokens;
            }
        }
    }
    
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Add blocks with varying content sizes
    let short = Block::new(Content::text("Short text."), Some("paragraph"));
    let short_id = doc.add_block(short, &root)?;
    
    let medium = Block::new(
        Content::text("This is a medium-length paragraph with more content that will use more tokens in the estimation."),
        Some("paragraph")
    );
    let medium_id = doc.add_block(medium, &root)?;
    
    let long = Block::new(
        Content::text(&"Long content. ".repeat(100)),
        Some("paragraph")
    );
    let long_id = doc.add_block(long, &root)?;
    
    let code = Block::new(
        Content::code("rust", &"fn example() {\n    // code\n}\n".repeat(20)),
        Some("code")
    );
    let code_id = doc.add_block(code, &root)?;
    
    // Estimate tokens
    println!("Token estimates (GPT-4):");
    for (name, id) in [("Short", &short_id), ("Medium", &medium_id), ("Long", &long_id), ("Code", &code_id)] {
        let block = doc.get_block(id).unwrap();
        let tokens = block.token_estimate().for_model(TokenModel::GPT4);
        println!("  {}: {} tokens", name, tokens);
    }
    
    let total = estimate_document_tokens(&doc, TokenModel::GPT4);
    println!("\nTotal document: {} tokens", total);
    
    // Find blocks within budget
    let all_ids = vec![short_id, medium_id, long_id, code_id];
    let budget = 500;
    let within_budget = find_blocks_within_budget(&doc, &all_ids, budget, TokenModel::GPT4);
    
    println!("\nBlocks within {} token budget: {}", budget, within_budget.len());
    
    Ok(())
}
```

## Example 7: Batch Document Processing

```rust
use ucp_api::UcpClient;
use ucp_translator_markdown::{parse_markdown, render_markdown};
use std::collections::HashMap;

fn process_markdown_files(files: &[(&str, &str)]) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let client = UcpClient::new();
    let mut results = HashMap::new();
    
    for (name, content) in files {
        // Parse markdown
        let mut doc = parse_markdown(content)?;
        
        // Add processing metadata
        client.execute_ucl(&mut doc, &format!(r#"
            EDIT blk_ff00000000000000000000 SET metadata.processed = true
            EDIT blk_ff00000000000000000000 SET metadata.source = "{}"
        "#, name))?;
        
        // Add tags to all paragraphs
        for block_id in doc.indices.find_by_type("text").clone() {
            if let Some(block) = doc.get_block(&block_id) {
                if !block.is_root() {
                    client.execute_ucl(&mut doc, &format!(
                        r#"EDIT {} SET metadata.tags += ["processed"]"#,
                        block_id
                    ))?;
                }
            }
        }
        
        // Render back
        let rendered = render_markdown(&doc)?;
        results.insert(name.to_string(), rendered);
    }
    
    Ok(results)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = vec![
        ("doc1", "# Document 1\n\nFirst document content."),
        ("doc2", "# Document 2\n\nSecond document content.\n\n## Section\n\nMore content."),
        ("doc3", "# Document 3\n\n```rust\nfn main() {}\n```"),
    ];
    
    let results = process_markdown_files(&files)?;
    
    for (name, content) in &results {
        println!("=== {} ===", name);
        println!("{}", content);
        println!();
    }
    
    Ok(())
}
```

## Next Steps

- [Advanced Examples](./advanced.md) - Complex patterns and integrations
- [UCL Commands](../ucl-parser/commands.md) - Complete UCL reference
- [UCM Engine](../ucm-engine/README.md) - Engine documentation
