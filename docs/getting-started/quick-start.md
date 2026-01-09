# Quick Start Guide

This guide walks you through creating your first UCP application, from basic document creation to advanced operations.

## Your First Document

```rust
use ucp_api::UcpClient;
use ucm_core::{Block, Content};

fn main() {
    // Initialize the client
    let client = UcpClient::new();
    
    // Create a new document (automatically includes a root block)
    let mut doc = client.create_document();
    
    println!("Created document: {}", doc.id);
    println!("Root block: {}", doc.root);
}
```

## Adding Content

### Text Blocks

```rust
use ucp_api::UcpClient;
use ucm_core::{Block, Content};

fn main() {
    let client = UcpClient::new();
    let mut doc = client.create_document();
    let root = doc.root.clone();
    
    // Add a text block with a semantic role
    let intro_id = client.add_text(
        &mut doc, 
        &root, 
        "Welcome to UCP!", 
        Some("intro")
    ).unwrap();
    
    // Add another text block
    let body_id = client.add_text(
        &mut doc,
        &root,
        "This is the body content.",
        Some("body")
    ).unwrap();
    
    println!("Document now has {} blocks", doc.block_count());
}
```

### Code Blocks

```rust
let code_id = client.add_code(
    &mut doc,
    &root,
    "rust",
    r#"fn hello() {
    println!("Hello, world!");
}"#
).unwrap();
```

### Using Block Builder Pattern

For more control, create blocks directly:

```rust
use ucm_core::{Block, Content};

// Create a block with tags and labels
let block = Block::new(Content::text("Important note"), Some("note"))
    .with_label("warning-note")
    .with_tag("important")
    .with_tag("review-needed");

let block_id = doc.add_block(block, &root).unwrap();
```

## Document Structure

UCP documents are hierarchical. Blocks can have children:

```rust
use ucp_api::UcpClient;
use ucm_core::{Block, Content};

fn main() {
    let client = UcpClient::new();
    let mut doc = client.create_document();
    let root = doc.root.clone();
    
    // Create a section
    let section = Block::new(Content::text("Chapter 1: Introduction"), Some("heading1"));
    let section_id = doc.add_block(section, &root).unwrap();
    
    // Add content under the section
    let para1 = Block::new(Content::text("First paragraph..."), Some("paragraph"));
    doc.add_block(para1, &section_id).unwrap();
    
    let para2 = Block::new(Content::text("Second paragraph..."), Some("paragraph"));
    doc.add_block(para2, &section_id).unwrap();
    
    // Check the structure
    let children = doc.children(&section_id);
    println!("Section has {} children", children.len());
}
```

## Using UCL Commands

UCL (Unified Content Language) provides a token-efficient way to manipulate documents:

```rust
use ucp_api::UcpClient;

fn main() {
    let client = UcpClient::new();
    let mut doc = client.create_document();
    
    // Execute UCL commands
    let results = client.execute_ucl(&mut doc, r#"
        APPEND blk_ff00000000000000000000 text WITH label="greeting" :: "Hello, UCL!"
        APPEND blk_ff00000000000000000000 code WITH label="example" :: "fn main() {}"
    "#).unwrap();
    
    for result in results {
        if result.success {
            println!("Operation succeeded, affected: {:?}", result.affected_blocks);
        }
    }
}
```

## Querying Documents

### Find Blocks by Tag

```rust
let important_blocks = doc.indices.find_by_tag("important");
for block_id in important_blocks {
    if let Some(block) = doc.get_block(&block_id) {
        println!("Found: {}", block_id);
    }
}
```

### Find Blocks by Label

```rust
if let Some(block_id) = doc.indices.find_by_label("warning-note") {
    let block = doc.get_block(&block_id).unwrap();
    println!("Found block by label: {}", block_id);
}
```

### Find Blocks by Content Type

```rust
let code_blocks = doc.indices.find_by_type("code");
println!("Document has {} code blocks", code_blocks.len());
```

## Modifying Content

### Direct Modification

```rust
if let Some(block) = doc.get_block_mut(&block_id) {
    block.update_content(Content::text("Updated text"), Some("paragraph"));
}
```

### Using UCL EDIT Command

```rust
client.execute_ucl(&mut doc, r#"
    EDIT blk_abc123def456 SET content.text = "New content"
    EDIT blk_abc123def456 SET metadata.tags += ["updated"]
"#).unwrap();
```

## Moving Blocks

```rust
// Move a block to a new parent
doc.move_block(&child_id, &new_parent_id).unwrap();

// Move to a specific position
doc.move_block_at(&child_id, &new_parent_id, 0).unwrap(); // Insert at beginning
```

## Deleting Blocks

```rust
// Delete a single block (children become orphaned)
doc.delete_block(&block_id).unwrap();

// Delete with all descendants
doc.delete_cascade(&block_id).unwrap();

// Clean up orphaned blocks
let pruned = doc.prune_unreachable();
println!("Pruned {} orphaned blocks", pruned.len());
```

## Serialization

### To JSON

```rust
let json = client.to_json(&doc).unwrap();
println!("{}", json);
```

### Block Serialization

```rust
use serde_json;

let block = doc.get_block(&block_id).unwrap();
let json = serde_json::to_string_pretty(block).unwrap();
```

## Working with Edges (Relationships)

```rust
use ucm_core::{Edge, EdgeType};

// Add a reference relationship
let edge = Edge::new(EdgeType::References, target_block_id.clone());
if let Some(block) = doc.get_block_mut(&source_block_id) {
    block.add_edge(edge);
}

// Query relationships
let outgoing = doc.edge_index.outgoing_from(&source_block_id);
let incoming = doc.edge_index.incoming_to(&target_block_id);
```

## Complete Example

```rust
use ucp_api::UcpClient;
use ucm_core::{Block, Content, Edge, EdgeType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UcpClient::new();
    let mut doc = client.create_document();
    let root = doc.root.clone();
    
    // Build document structure
    let title = Block::new(Content::text("My Document"), Some("title"));
    let title_id = doc.add_block(title, &root)?;
    
    let intro = Block::new(Content::text("This document demonstrates UCP."), Some("intro"));
    let intro_id = doc.add_block(intro, &title_id)?;
    
    let section = Block::new(Content::text("Main Content"), Some("heading2"));
    let section_id = doc.add_block(section, &title_id)?;
    
    let code = Block::new(
        Content::code("rust", "println!(\"Hello!\");"),
        Some("code")
    ).with_tag("example");
    let code_id = doc.add_block(code, &section_id)?;
    
    // Add a reference from intro to code
    let edge = Edge::new(EdgeType::References, code_id.clone());
    doc.get_block_mut(&intro_id).unwrap().add_edge(edge.clone());
    doc.edge_index.add_edge(&intro_id, &edge);
    
    // Validate the document
    let issues = doc.validate();
    if issues.is_empty() {
        println!("Document is valid!");
    } else {
        for issue in issues {
            println!("Issue: {}", issue.message);
        }
    }
    
    // Print statistics
    println!("Total blocks: {}", doc.block_count());
    println!("Code blocks: {}", doc.indices.find_by_type("code").len());
    
    Ok(())
}
```

## Next Steps

- [Core Concepts](./concepts.md) - Deep dive into UCP architecture
- [UCM Core Documentation](../ucm-core/README.md) - Detailed type reference
- [UCL Syntax Guide](../ucl-parser/syntax.md) - Complete UCL reference
- [Examples](../examples/basic.md) - More code examples
