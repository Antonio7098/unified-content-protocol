# UCP API

**ucp-api** provides a high-level API for working with UCP, combining all core crates into a convenient interface for application development.

## Overview

The UCP API is the recommended entry point for most applications. It provides:

- **UcpClient** - Main client for document manipulation
- **Unified interface** - Access to all UCP functionality
- **UCL integration** - Execute UCL commands directly
- **Convenience methods** - Common operations simplified

## Installation

=== "Rust"
    ```toml
    [dependencies]
    ucp-api = "0.1.7"
    ```

=== "Python"
    ```bash
    pip install ucp-content
    ```

=== "JavaScript"
    ```bash
    npm install ucp-content
    ```

## Quick Start

=== "Rust"
    ```rust
    use ucp_api::UcpClient;

    fn main() {
        // Create client
        let client = UcpClient::new();
        
        // Create document
        let mut doc = client.create_document();
        let root = doc.root.clone();
        
        // Add content
        client.add_text(&mut doc, &root, "Hello, UCP!", Some("intro")).unwrap();
        
        // Execute UCL
        client.execute_ucl(&mut doc, r#"
            APPEND blk_root text :: "More content"
        "#).unwrap();
        
        // Serialize
        let json = client.to_json(&doc).unwrap();
        println!("{}", json);
    }
    ```

=== "Python"
    ```python
    from ucp_content import Document, execute_ucl

    # Create document
    doc = Document.create()
    root = doc.root_id

    # Add content
    doc.add_block(root, "Hello, UCP!", role="intro")

    # Execute UCL
    execute_ucl(doc, """
        APPEND blk_root text :: "More content"
    """)

    # Serialize
    print(doc.to_json())
    ```

=== "JavaScript"
    ```javascript
    import { Document, executeUcl } from 'ucp-content';

    // Create document
    const doc = Document.create();
    const root = doc.rootId;

    // Add content
    doc.addBlock(root, "Hello, UCP!", "intro");

    // Execute UCL
    executeUcl(doc, `
        APPEND blk_root text :: "More content"
    `);

    // Serialize
    console.log(JSON.stringify(doc.toJson(), null, 2));
    ```

## UcpClient

The main entry point for UCP operations.

### Creating a Client

=== "Rust"
    ```rust
    use ucp_api::UcpClient;

    // Default client
    let client = UcpClient::new();
    ```

=== "Python"
    *The Python SDK exposes functionality through the `Document` class and module-level functions.*

=== "JavaScript"
    *The JavaScript SDK exposes functionality through the `Document` class and module-level functions.*

### Document Operations

=== "Rust"
    ```rust
    // Create new document
    let mut doc = client.create_document();

    // Get document info
    println!("Document ID: {}", doc.id);
    println!("Root block: {}", doc.root);
    println!("Block count: {}", doc.block_count());
    ```

=== "Python"
    ```python
    doc = Document.create()
    
    print(f"Document ID: {doc.id}")
    print(f"Root block: {doc.root_id}")
    print(f"Block count: {doc.block_count()}")
    ```

=== "JavaScript"
    ```javascript
    const doc = Document.create();
    
    console.log(`Document ID: ${doc.id}`);
    console.log(`Root block: ${doc.rootId}`);
    console.log(`Block count: ${doc.blockCount()}`);
    ```

### Adding Content

=== "Rust"
    ```rust
    let root = doc.root.clone();

    // Add text block
    let text_id = client.add_text(
        &mut doc,
        &root,
        "Paragraph content",
        Some("paragraph")  // semantic role
    ).unwrap();

    // Add code block
    let code_id = client.add_code(
        &mut doc,
        &root,
        "rust",
        "fn main() {\n    println!(\"Hello!\");\n}"
    ).unwrap();
    ```

=== "Python"
    ```python
    root = doc.root_id
    
    # Add text block
    text_id = doc.add_block(root, "Paragraph content", role="paragraph")
    
    # Add code block
    code_id = doc.add_code(root, "rust", 'fn main() {\n    println!("Hello!");\n}')
    ```

=== "JavaScript"
    ```javascript
    const root = doc.rootId;
    
    // Add text block
    const textId = doc.addBlock(root, "Paragraph content", "paragraph");
    
    // Add code block
    const codeId = doc.addCode(root, "rust", 'fn main() {\n    console.log("Hello!");\n}');
    ```

### Executing UCL

=== "Rust"
    ```rust
    // Parse UCL (without executing)
    let commands = client.parse_ucl(r#"
        EDIT blk_abc SET content.text = "Hello"
        APPEND blk_root text :: "New block"
    "#).unwrap();

    println!("Parsed {} commands", commands.len());

    // Execute UCL commands
    let results = client.execute_ucl(&mut doc, r#"
        APPEND blk_root text WITH label="intro" :: "Introduction"
        EDIT blk_intro SET metadata.tags += ["important"]
    "#).unwrap();

    for result in &results {
        if result.success {
            println!("Success: {:?}", result.affected_blocks);
        } else {
            println!("Failed: {:?}", result.error);
        }
    }
    ```

=== "Python"
    ```python
    # Execute UCL commands
    affected_ids = execute_ucl(doc, """
        APPEND blk_root text WITH label="intro" :: "Introduction"
        EDIT blk_intro SET metadata.tags += ["important"]
    """)
    
    print(f"Affected blocks: {affected_ids}")
    ```

=== "JavaScript"
    ```javascript
    // Execute UCL commands
    const affectedIds = executeUcl(doc, `
        APPEND blk_root text WITH label="intro" :: "Introduction"
        EDIT blk_intro SET metadata.tags += ["important"]
    `);
    
    console.log(`Affected blocks: ${affectedIds}`);
    ```

### Serialization

=== "Rust"
    ```rust
    // Serialize document to JSON
    let json = client.to_json(&doc).unwrap();
    println!("{}", json);

    // Pretty-print if needed
    let pretty: serde_json::Value = serde_json::from_str(&json).unwrap();
    println!("{}", serde_json::to_string_pretty(&pretty).unwrap());
    ```

=== "Python"
    ```python
    json_str = doc.to_json()
    print(json_str)
    ```

=== "JavaScript"
    ```javascript
    const jsonObj = doc.toJson();
    console.log(JSON.stringify(jsonObj, null, 2));
    ```

## Complete Example

=== "Rust"
    ```rust
    use ucp_api::UcpClient;
    use ucm_core::{Block, Content, Edge, EdgeType};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let client = UcpClient::new();
        let mut doc = client.create_document();
        let root = doc.root.clone();
        
        // Build document structure using convenience methods
        let title_id = client.add_text(
            &mut doc,
            &root,
            "My Technical Guide",
            Some("title")
        )?;
        
        let intro_id = client.add_text(
            &mut doc,
            &root,
            "Welcome to this comprehensive guide.",
            Some("intro")
        )?;
        
        // Add code example
        let code_id = client.add_code(
            &mut doc,
            &root,
            "rust",
            r#"fn main() {
        println!("Hello, UCP!");
    }"#
        )?;
        
        // Use UCL for more complex operations
        client.execute_ucl(&mut doc, r#"
            // Add tags to intro
            EDIT blk_intro SET metadata.tags += ["overview", "start-here"]
            
            // Add reference from intro to code
            LINK blk_intro references blk_code
            
            // Create a snapshot
            SNAPSHOT CREATE "v1" WITH description="Initial version"
        "#)?;
        
        // Query document
        let code_blocks = doc.indices.find_by_type("code");
        println!("Code blocks: {}", code_blocks.len());
        
        let important = doc.indices.find_by_tag("overview");
        println!("Overview blocks: {}", important.len());
        
        // Validate
        let issues = doc.validate();
        if issues.is_empty() {
            println!("Document is valid!");
        }
        
        // Serialize
        let json = client.to_json(&doc)?;
        println!("\nDocument JSON:\n{}", json);
        
        Ok(())
    }
    ```

=== "Python"
    ```python
    from ucp_content import Document, execute_ucl, EdgeType

    # Create document
    doc = Document.create()
    root = doc.root_id
    
    # Build structure
    title_id = doc.add_block(root, "My Technical Guide", role="title")
    
    intro_id = doc.add_block(root, "Welcome to this comprehensive guide.", role="intro", label="intro")
    
    code_id = doc.add_code(root, "rust", 'fn main() {\n    println!("Hello, UCP!");\n}', label="code")
    
    # Use UCL for complex operations
    execute_ucl(doc, """
        // Add tags to intro
        EDIT intro SET metadata.tags += ["overview", "start-here"]
        
        // Add reference
        LINK intro references code
        
        // Create snapshot
        SNAPSHOT CREATE "v1" WITH description="Initial version"
    """)
    
    # Query document
    code_blocks = doc.find_by_type("code")
    print(f"Code blocks: {len(code_blocks)}")
    
    important = doc.find_by_tag("overview")
    print(f"Overview blocks: {len(important)}")
    
    # Validate
    issues = doc.validate()
    if not issues:
        print("Document is valid!")
        
    # Serialize
    print(f"\nDocument JSON:\n{doc.to_json()}")
    ```

=== "JavaScript"
    ```javascript
    import { Document, executeUcl } from 'ucp-content';

    // Create document
    const doc = Document.create();
    const root = doc.rootId;
    
    // Build structure
    const titleId = doc.addBlock(root, "My Technical Guide", "title");
    
    const introId = doc.addBlock(root, "Welcome to this comprehensive guide.", "intro");
    // Note: To label the block for UCL, we might need to set it explicitly if not passed in addBlock
    doc.setLabel(introId, "intro");
    
    const codeId = doc.addCode(root, "rust", 'fn main() {\n    console.log("Hello, UCP!");\n}');
    doc.setLabel(codeId, "code");
    
    // Use UCL for complex operations
    executeUcl(doc, `
        // Add tags to intro
        EDIT intro SET metadata.tags += ["overview", "start-here"]
        
        // Add reference
        LINK intro references code
        
        // Create snapshot
        SNAPSHOT CREATE "v1" WITH description="Initial version"
    `);
    
    // Query document
    const codeBlocks = doc.findByType("code");
    console.log(`Code blocks: ${codeBlocks.length}`);
    
    const important = doc.findByTag("overview");
    console.log(`Overview blocks: ${important.length}`);
    
    // Validate
    const issues = doc.validate();
    if (!issues) {
        console.log("Document is valid!");
    }
    
    // Serialize
    console.log(`\nDocument JSON:\n${JSON.stringify(doc.toJson(), null, 2)}`);
    ```

## Integration with Other Crates

UCP API re-exports types from underlying crates:

```rust
use ucp_api::UcpClient;

// From ucm-core
use ucm_core::{Block, Content, Document, Edge, EdgeType, BlockId};
use ucm_core::metadata::{SemanticRole, RoleCategory, TokenEstimate};

// From ucm-engine
use ucm_engine::{Engine, Operation, OperationResult};

// From ucl-parser
use ucl_parser::{parse, parse_commands, Command};
```

## Error Handling

=== "Rust"
    ```rust
    use ucp_api::UcpClient;

    let client = UcpClient::new();
    let mut doc = client.create_document();

    // Handle UCL errors
    match client.execute_ucl(&mut doc, "INVALID SYNTAX") {
        Ok(results) => {
            for result in results {
                if !result.success {
                    eprintln!("Operation failed: {:?}", result.error);
                }
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
    }

    // Handle add errors
    match client.add_text(&mut doc, &invalid_parent, "text", None) {
        Ok(id) => println!("Added: {}", id),
        Err(e) => eprintln!("Failed to add: {}", e),
    }
    ```

=== "Python"
    ```python
    try:
        execute_ucl(doc, "INVALID SYNTAX")
    except Exception as e:
        print(f"Error: {e}")
    ```

=== "JavaScript"
    ```javascript
    try {
        executeUcl(doc, "INVALID SYNTAX");
    } catch (e) {
        console.error(`Error: ${e}`);
    }
    ```

## Best Practices

### 1. Use UCL for Complex Operations

=== "Rust"
    ```rust
    // Good - UCL for multiple related operations
    client.execute_ucl(&mut doc, r#"
        APPEND blk_root text WITH label="section" :: "Section 1"
        APPEND blk_section text :: "Content..."
        LINK blk_section references blk_intro
    "#)?;

    // Less ideal - multiple separate calls
    let section = client.add_text(&mut doc, &root, "Section 1", None)?;
    let content = client.add_text(&mut doc, &section, "Content...", None)?;
    // Manual edge management...
    ```

### 2. Use Convenience Methods for Simple Operations

=== "Rust"
    ```rust
    // Good - simple addition
    let id = client.add_text(&mut doc, &root, "Hello", Some("intro"))?;

    // Overkill for simple cases
    client.execute_ucl(&mut doc, r#"
        APPEND blk_root text WITH role="intro" :: "Hello"
    "#)?;
    ```

### 3. Validate Before Serialization

=== "Rust"
    ```rust
    let issues = doc.validate();
    if !issues.is_empty() {
        for issue in &issues {
            log::warn!("Validation issue: {}", issue.message);
        }
    }

    let json = client.to_json(&doc)?;
    ```

### 4. Use Snapshots for Safety

=== "Rust"
    ```rust
    // Before risky operations
    client.execute_ucl(&mut doc, r#"
        SNAPSHOT CREATE "checkpoint"
    "#)?;

    // Perform operations...

    // If something goes wrong, restore
    // client.execute_ucl(&mut doc, "SNAPSHOT RESTORE \"checkpoint\"")?;
    ```

## See Also

- [Quick Start Guide](../getting-started/quick-start.md) - Getting started with UCP
- [UCL Commands](../ucl-parser/commands.md) - UCL command reference
- [UCM Core](../ucm-core/README.md) - Core types documentation
- [UCM Engine](../ucm-engine/README.md) - Engine documentation

