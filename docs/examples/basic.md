# Basic Examples

This document provides basic examples for getting started with UCP.

## Example 1: Creating a Simple Document

```rust
use ucp_api::UcpClient;
use ucm_core::{Block, Content};

fn main() {
    let client = UcpClient::new();
    
    // Create a new document
    let mut doc = client.create_document();
    let root = doc.root.clone();
    
    // Add a title
    let title = Block::new(Content::text("My First Document"), Some("title"));
    let title_id = doc.add_block(title, &root).unwrap();
    
    // Add a paragraph
    let para = Block::new(
        Content::text("This is my first UCP document."),
        Some("paragraph")
    );
    doc.add_block(para, &title_id).unwrap();
    
    println!("Created document with {} blocks", doc.block_count());
}
```

## Example 2: Using the UcpClient

```rust
use ucp_api::UcpClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UcpClient::new();
    let mut doc = client.create_document();
    let root = doc.root.clone();
    
    // Add text using convenience method
    let intro_id = client.add_text(
        &mut doc,
        &root,
        "Welcome to UCP!",
        Some("intro")
    )?;
    
    // Add code using convenience method
    let code_id = client.add_code(
        &mut doc,
        &root,
        "rust",
        "fn hello() {\n    println!(\"Hello!\");\n}"
    )?;
    
    // Serialize to JSON
    let json = client.to_json(&doc)?;
    println!("{}", json);
    
    Ok(())
}
```

## Example 3: Basic UCL Commands

```rust
use ucp_api::UcpClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UcpClient::new();
    let mut doc = client.create_document();
    
    // Execute UCL commands
    let results = client.execute_ucl(&mut doc, r#"
        // Add a heading
        APPEND blk_ff00000000000000000000 text WITH role="heading1" :: "Getting Started"
        
        // Add a paragraph
        APPEND blk_ff00000000000000000000 text WITH role="paragraph" :: "This guide will help you get started with UCP."
        
        // Add a code example
        APPEND blk_ff00000000000000000000 code :: "cargo add ucp-api"
    "#)?;
    
    println!("Executed {} commands", results.len());
    println!("Document has {} blocks", doc.block_count());
    
    Ok(())
}
```

## Example 4: Working with Content Types

```rust
use ucm_core::{Block, Content, Document};

fn main() {
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Text content
    let text = Block::new(Content::text("Plain text content"), Some("paragraph"));
    doc.add_block(text, &root).unwrap();
    
    // Markdown content
    let markdown = Block::new(
        Content::markdown("**Bold** and *italic* text"),
        Some("paragraph")
    );
    doc.add_block(markdown, &root).unwrap();
    
    // Code content
    let code = Block::new(
        Content::code("python", "def greet(name):\n    print(f'Hello, {name}!')"),
        Some("code")
    );
    doc.add_block(code, &root).unwrap();
    
    // Table content
    let table = Block::new(
        Content::table(vec![
            vec!["Name".into(), "Age".into(), "City".into()],
            vec!["Alice".into(), "30".into(), "NYC".into()],
            vec!["Bob".into(), "25".into(), "LA".into()],
        ]),
        Some("table")
    );
    doc.add_block(table, &root).unwrap();
    
    // JSON content
    let json = Block::new(
        Content::json(serde_json::json!({
            "name": "config",
            "debug": true,
            "level": 5
        })),
        Some("metadata")
    );
    doc.add_block(json, &root).unwrap();
    
    println!("Created {} blocks with different content types", doc.block_count());
}
```

## Example 5: Building Document Structure

```rust
use ucm_core::{Block, Content, Document};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Create chapter 1
    let ch1 = Block::new(Content::text("Chapter 1: Introduction"), Some("heading1"));
    let ch1_id = doc.add_block(ch1, &root)?;
    
    // Add sections to chapter 1
    let sec1_1 = Block::new(Content::text("1.1 Overview"), Some("heading2"));
    let sec1_1_id = doc.add_block(sec1_1, &ch1_id)?;
    
    let para1 = Block::new(Content::text("This section provides an overview."), Some("paragraph"));
    doc.add_block(para1, &sec1_1_id)?;
    
    let sec1_2 = Block::new(Content::text("1.2 Background"), Some("heading2"));
    let sec1_2_id = doc.add_block(sec1_2, &ch1_id)?;
    
    let para2 = Block::new(Content::text("Some background information."), Some("paragraph"));
    doc.add_block(para2, &sec1_2_id)?;
    
    // Create chapter 2
    let ch2 = Block::new(Content::text("Chapter 2: Details"), Some("heading1"));
    let ch2_id = doc.add_block(ch2, &root)?;
    
    let para3 = Block::new(Content::text("Detailed content here."), Some("paragraph"));
    doc.add_block(para3, &ch2_id)?;
    
    // Print structure
    println!("Document structure:");
    print_structure(&doc, &root, 0);
    
    Ok(())
}

fn print_structure(doc: &Document, block_id: &ucm_core::BlockId, depth: usize) {
    let indent = "  ".repeat(depth);
    
    if let Some(block) = doc.get_block(block_id) {
        let content_preview = match &block.content {
            ucm_core::Content::Text(t) => t.text.chars().take(30).collect::<String>(),
            _ => block.content_type().to_string(),
        };
        println!("{}{}: {}", indent, block_id, content_preview);
    }
    
    if let Some(children) = doc.structure.get(block_id) {
        for child in children {
            print_structure(doc, child, depth + 1);
        }
    }
}
```

## Example 6: Using Tags and Labels

```rust
use ucm_core::{Block, Content, Document};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Create blocks with tags and labels
    let important = Block::new(Content::text("Important note"), Some("note"))
        .with_label("important-note")
        .with_tag("important")
        .with_tag("review-needed");
    let important_id = doc.add_block(important, &root)?;
    
    let draft = Block::new(Content::text("Draft content"), Some("paragraph"))
        .with_label("draft-section")
        .with_tag("draft")
        .with_tag("wip");
    doc.add_block(draft, &root)?;
    
    let final_content = Block::new(Content::text("Final content"), Some("paragraph"))
        .with_label("final-section")
        .with_tag("final")
        .with_tag("approved");
    doc.add_block(final_content, &root)?;
    
    // Query by tag
    let important_blocks = doc.indices.find_by_tag("important");
    println!("Important blocks: {}", important_blocks.len());
    
    let draft_blocks = doc.indices.find_by_tag("draft");
    println!("Draft blocks: {}", draft_blocks.len());
    
    // Query by label
    if let Some(id) = doc.indices.find_by_label("important-note") {
        println!("Found important-note: {}", id);
    }
    
    // Check if block has tag
    let block = doc.get_block(&important_id).unwrap();
    println!("Has 'important' tag: {}", block.has_tag("important"));
    println!("Has 'draft' tag: {}", block.has_tag("draft"));
    
    Ok(())
}
```

## Example 7: Querying Documents

```rust
use ucm_core::{Block, Content, Document};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Build a document
    let heading = Block::new(Content::text("Title"), Some("heading1"));
    let heading_id = doc.add_block(heading, &root)?;
    
    let para1 = Block::new(Content::text("First paragraph"), Some("paragraph"));
    doc.add_block(para1, &heading_id)?;
    
    let code = Block::new(Content::code("rust", "fn main() {}"), Some("code"));
    doc.add_block(code, &heading_id)?;
    
    let para2 = Block::new(Content::text("Second paragraph"), Some("paragraph"));
    doc.add_block(para2, &heading_id)?;
    
    // Query by content type
    let text_blocks = doc.indices.find_by_type("text");
    println!("Text blocks: {}", text_blocks.len());
    
    let code_blocks = doc.indices.find_by_type("code");
    println!("Code blocks: {}", code_blocks.len());
    
    // Get children of a block
    let children = doc.children(&heading_id);
    println!("Heading has {} children", children.len());
    
    // Get parent
    if let Some(parent) = doc.parent(&heading_id) {
        println!("Heading's parent: {}", parent);
    }
    
    // Get all descendants
    let descendants = doc.descendants(&root);
    println!("Root has {} descendants", descendants.len());
    
    Ok(())
}
```

## Example 8: Markdown Conversion

```rust
use ucp_translator_markdown::{parse_markdown, render_markdown};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse Markdown to UCM
    let markdown = r#"
# Welcome

This is a simple document.

## Features

- Easy to use
- Powerful
- Flexible

```rust
fn main() {
    println!("Hello, UCP!");
}
```
"#;

    let doc = parse_markdown(markdown)?;
    println!("Parsed {} blocks from Markdown", doc.block_count());
    
    // Render back to Markdown
    let rendered = render_markdown(&doc)?;
    println!("\nRendered Markdown:\n{}", rendered);
    
    Ok(())
}
```

## Example 9: Document Validation

```rust
use ucm_core::{Block, Content, Document, ValidationSeverity};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Add some blocks
    let block = Block::new(Content::text("Content"), Some("paragraph"));
    let block_id = doc.add_block(block, &root)?;
    
    // Validate document
    let issues = doc.validate();
    
    if issues.is_empty() {
        println!("Document is valid!");
    } else {
        for issue in &issues {
            match issue.severity {
                ValidationSeverity::Error => eprintln!("ERROR: {}", issue.message),
                ValidationSeverity::Warning => println!("WARNING: {}", issue.message),
                ValidationSeverity::Info => println!("INFO: {}", issue.message),
            }
        }
    }
    
    // Create an orphan to demonstrate warning
    doc.remove_from_structure(&block_id);
    
    let issues = doc.validate();
    println!("\nAfter creating orphan:");
    for issue in &issues {
        println!("[{:?}] {}", issue.severity, issue.message);
    }
    
    Ok(())
}
```

## Example 10: Serialization

```rust
use ucp_api::UcpClient;
use ucm_core::{Block, Content};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UcpClient::new();
    let mut doc = client.create_document();
    let root = doc.root.clone();
    
    // Add content
    client.add_text(&mut doc, &root, "Hello, World!", Some("greeting"))?;
    client.add_code(&mut doc, &root, "rust", "fn main() {}")?;
    
    // Serialize document to JSON
    let json = client.to_json(&doc)?;
    println!("Document JSON:\n{}", json);
    
    // Serialize individual block
    let block = doc.get_block(&root).unwrap();
    let block_json = serde_json::to_string_pretty(block)?;
    println!("\nRoot block JSON:\n{}", block_json);
    
    Ok(())
}
```

## Example 11: Python SDK - Semantic Roles

```python
from ucp import (
    create, parse, Block, SemanticRole,
    ValidationResult, ValidationIssue, ValidationSeverity
)

# Create a document with semantic roles
doc = create()

# Add blocks with various semantic roles
doc.add_block(doc.root_id, "My Document", role=SemanticRole.TITLE)
doc.add_block(doc.root_id, "Welcome to this guide", role=SemanticRole.INTRO)

# Use callout roles for important content
doc.add_block(doc.root_id, "Remember to save your work", role=SemanticRole.NOTE)
doc.add_block(doc.root_id, "This action cannot be undone!", role=SemanticRole.WARNING)
doc.add_block(doc.root_id, "Pro tip: Use keyboard shortcuts", role=SemanticRole.TIP)

# Technical content
doc.add_block(doc.root_id, "E = mc^2", role=SemanticRole.EQUATION)
doc.add_block(doc.root_id, "[1] Author, Title, 2024", role=SemanticRole.CITATION)

print(f"Created document with {len(doc.blocks)} blocks")

# List all blocks with their roles
for block_id, block in doc.blocks.items():
    role_name = block.role.value if block.role else "none"
    print(f"  {block_id[:12]}... - {role_name}: {block.content[:30]}")
```

## Example 12: Python SDK - Validation API

```python
from ucp import (
    create, ValidationResult, ValidationIssue, ValidationSeverity
)

# Create validation result with different severities
result = ValidationResult(
    valid=True,
    issues=[
        ValidationIssue.error("E001", "Missing required field", block_id="blk_123"),
        ValidationIssue.warning("W001", "Deprecated syntax used"),
        ValidationIssue.info("I001", "Consider adding a description"),
    ]
)

# Filter issues by severity
errors = result.errors()
warnings = result.warnings()
infos = result.infos()

print(f"Errors: {len(errors)}")
print(f"Warnings: {len(warnings)}")
print(f"Infos: {len(infos)}")

# Check document validation
doc = create()
doc.add_block(doc.root_id, "Content")

validation = doc.validate()
if validation.valid:
    print("Document is valid!")
else:
    for error in validation.errors():
        print(f"ERROR [{error.code}]: {error.message}")
```

## Example 13: Python SDK - Block Hashability

```python
from ucp import create, Block, SemanticRole

# Blocks can be used in sets and as dictionary keys
block1 = Block.text("Hello", role=SemanticRole.PARAGRAPH)
block2 = Block.text("World", role=SemanticRole.PARAGRAPH)

# Add to set
block_set = {block1, block2}
print(f"Set contains {len(block_set)} blocks")

# Use as dictionary key
block_metadata = {
    block1: {"priority": "high"},
    block2: {"priority": "low"},
}
print(f"Block 1 priority: {block_metadata[block1]['priority']}")

# Equality is based on block ID
same_id_block = Block(id=block1.id, content="Different content")
print(f"Same ID means equal: {block1 == same_id_block}")
```

## Next Steps

- [Intermediate Examples](./intermediate.md) - More complex scenarios
- [Advanced Examples](./advanced.md) - Advanced usage patterns
- [UCL Commands](../ucl-parser/commands.md) - UCL reference
- [Semantic Roles](../ucm-core/semantic-roles.md) - Complete role reference
