# Quick Start Guide

This guide walks you through creating your first UCP application, from CLI usage to advanced operations.

## CLI Quick Start (Recommended for New Users)

The fastest way to get started with UCP is using the command-line interface:

```bash
# Install the CLI
cargo install ucp-cli

# Create a new document
ucp create --title "My First Document" --output doc.json

# View the document
cat doc.json | jq

# Get help
ucp --help
```

### CLI Workflow Example

```bash
# 1. Create a document
ucp create --title "Hello UCP" --output doc.json

# 2. Add a text block
ucp block add \
  --input doc.json \
  --output doc.json \
  --parent blk_root \
  --content-type text \
  --content "Welcome to the Unified Content Protocol!" \
  --role intro

# 3. View document structure
ucp tree --input doc.json --format text

# 4. Export to Markdown
ucp export markdown --input doc.json --output doc.md

# 5. Validate the document
ucp validate --input doc.json
```

### Import Content

```bash
# Import from Markdown
ucp import markdown README.md --output doc.json

# Import from HTML (with image extraction)
ucp import html article.html --output doc.json --extract-images --extract-links
```

### Search and Navigation

```bash
# Find blocks by tag
ucp find --input doc.json --tag intro --limit 10

# Find orphaned blocks
ucp orphans --input doc.json

# Navigate document structure
ucp nav children --input doc.json --id blk_root

# View as tree
ucp tree --input doc.json --depth 3 --ids
```

## Rust Library Quick Start

=== "Rust"
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

=== "Python"
    ```python
    from ucp_content import Document

    # Create a new document
    doc = Document.create()

    print(f"Created document: {doc.id}")
    print(f"Root block: {doc.root_id}")
    ```

=== "JavaScript"
    ```javascript
    import { Document } from 'ucp-content';

    // Create a new document
    const doc = Document.create();

    console.log(`Created document: ${doc.id}`);
    console.log(`Root block: ${doc.rootId}`);
    ```

## Adding Content

### Text Blocks

=== "Rust"
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

=== "Python"
    ```python
    # Add a text block with a semantic role
    intro_id = doc.add_block(
        parent_id=doc.root_id,
        content="Welcome to UCP!",
        role="intro"
    )

    # Add another text block
    body_id = doc.add_block(
        parent_id=doc.root_id,
        content="This is the body content.",
        role="body"
    )

    print(f"Document now has {doc.block_count()} blocks")
    ```

=== "JavaScript"
    ```javascript
    // Add a text block with a semantic role
    const introId = doc.addBlock(
        doc.rootId,
        "Welcome to UCP!",
        "intro"
    );

    // Add another text block
    const bodyId = doc.addBlock(
        doc.rootId,
        "This is the body content.",
        "body"
    );

    console.log(`Document now has ${doc.blockCount()} blocks`);
    ```

### Code Blocks

=== "Rust"
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

=== "Python"
    ```python
    code_id = doc.add_code(
        parent_id=doc.root_id,
        language="rust",
        source="""fn hello() {
        println!("Hello, world!");
    }"""
    )
    ```

=== "JavaScript"
    ```javascript
    const codeId = doc.addCode(
        doc.rootId,
        "rust",
        `fn hello() {
        println!("Hello, world!");
    }`
    );
    ```

### Block Options (Labels & Tags)

For more control, you can add labels and tags when creating blocks.

=== "Rust"
    ```rust
    use ucm_core::{Block, Content};

    // Create a block with tags and labels
    let block = Block::new(Content::text("Important note"), Some("note"))
        .with_label("warning-note")
        .with_tag("important")
        .with_tag("review-needed");

    let block_id = doc.add_block(block, &root).unwrap();
    ```

=== "Python"
    ```python
    block_id = doc.add_block(
        parent_id=doc.root_id,
        content="Important note",
        role="note",
        label="warning-note",
        tags=["important", "review-needed"]
    )
    ```

=== "JavaScript"
    ```javascript
    const blockId = doc.addBlock(
        doc.rootId,
        "Important note",
        "note",
        "warning-note" // label
    );
    
    // Add tags
    doc.addTag(blockId, "important");
    doc.addTag(blockId, "review-needed");
    ```

## Document Structure

UCP documents are hierarchical. Blocks can have children.

=== "Rust"
    ```rust
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
    ```

=== "Python"
    ```python
    # Create a section
    section_id = doc.add_block(doc.root_id, "Chapter 1: Introduction", role="heading1")

    # Add content under the section
    doc.add_block(section_id, "First paragraph...", role="paragraph")
    doc.add_block(section_id, "Second paragraph...", role="paragraph")

    # Check the structure
    children = doc.children(section_id)
    print(f"Section has {len(children)} children")
    ```

=== "JavaScript"
    ```javascript
    // Create a section
    const sectionId = doc.addBlock(doc.rootId, "Chapter 1: Introduction", "heading1");

    // Add content under the section
    doc.addBlock(sectionId, "First paragraph...", "paragraph");
    doc.addBlock(sectionId, "Second paragraph...", "paragraph");

    // Check the structure
    const children = doc.children(sectionId);
    console.log(`Section has ${children.length} children`);
    ```

## Using UCL Commands

UCL (Unified Content Language) provides a token-efficient way to manipulate documents.

=== "Rust"
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

=== "Python"
    ```python
    import ucp_content

    # Execute UCL commands
    # Note: Replace the ID with your actual root ID
    affected_ids = ucp_content.execute_ucl(doc, f"""
        APPEND {doc.root_id} text WITH label="greeting" :: "Hello, UCL!"
        APPEND {doc.root_id} code WITH label="example" :: "fn main() {{}}"
    """)
    
    print(f"Affected blocks: {affected_ids}")
    ```

=== "JavaScript"
    ```javascript
    import { executeUcl } from 'ucp-content';

    // Execute UCL commands
    const affectedIds = executeUcl(doc, `
        APPEND ${doc.rootId} text WITH label="greeting" :: "Hello, UCL!"
        APPEND ${doc.rootId} code WITH label="example" :: "fn main() {}"
    `);

    console.log(`Affected blocks: ${affectedIds}`);
    ```

## Querying Documents

### Find Blocks by Tag

=== "Rust"
    ```rust
    let important_blocks = doc.indices.find_by_tag("important");
    for block_id in important_blocks {
        if let Some(block) = doc.get_block(&block_id) {
            println!("Found: {}", block_id);
        }
    }
    ```

=== "Python"
    ```python
    important_blocks = doc.find_by_tag("important")
    for block_id in important_blocks:
        print(f"Found: {block_id}")
    ```

=== "JavaScript"
    ```javascript
    const importantBlocks = doc.findByTag("important");
    for (const blockId of importantBlocks) {
        console.log(`Found: ${blockId}`);
    }
    ```

### Find Blocks by Label

=== "Rust"
    ```rust
    if let Some(block_id) = doc.indices.find_by_label("warning-note") {
        let block = doc.get_block(&block_id).unwrap();
        println!("Found block by label: {}", block_id);
    }
    ```

=== "Python"
    ```python
    block_id = doc.find_by_label("warning-note")
    if block_id:
        print(f"Found block by label: {block_id}")
    ```

=== "JavaScript"
    ```javascript
    const blockId = doc.findByLabel("warning-note");
    if (blockId) {
        console.log(`Found block by label: ${blockId}`);
    }
    ```

### Find Blocks by Content Type

=== "Rust"
    ```rust
    let code_blocks = doc.indices.find_by_type("code");
    println!("Document has {} code blocks", code_blocks.len());
    ```

=== "Python"
    ```python
    code_blocks = doc.find_by_type("code")
    print(f"Document has {len(code_blocks)} code blocks")
    ```

=== "JavaScript"
    ```javascript
    const codeBlocks = doc.findByType("code");
    console.log(`Document has ${codeBlocks.length} code blocks`);
    ```

## Modifying Content

### Direct Modification

=== "Rust"
    ```rust
    if let Some(block) = doc.get_block_mut(&block_id) {
        block.update_content(Content::text("Updated text"), Some("paragraph"));
    }
    ```

=== "Python"
    ```python
    doc.edit_block(block_id, "Updated text", role="paragraph")
    ```

=== "JavaScript"
    ```javascript
    doc.editBlock(blockId, "Updated text", "paragraph");
    ```

## Moving Blocks

=== "Rust"
    ```rust
    // Move a block to a new parent
    doc.move_block(&child_id, &new_parent_id).unwrap();

    // Move to a specific position
    doc.move_block_at(&child_id, &new_parent_id, 0).unwrap(); // Insert at beginning
    ```

=== "Python"
    ```python
    # Move a block to a new parent
    doc.move_block(child_id, new_parent_id)

    # Move to a specific position
    doc.move_block(child_id, new_parent_id, index=0)
    ```

=== "JavaScript"
    ```javascript
    // Move a block to a new parent
    doc.moveBlock(childId, newParentId);

    // Move to a specific position
    doc.moveBlock(childId, newParentId, 0);
    ```

## Deleting Blocks

=== "Rust"
    ```rust
    // Delete a single block (children become orphaned)
    doc.delete_block(&block_id).unwrap();

    // Delete with all descendants
    doc.delete_cascade(&block_id).unwrap();

    // Clean up orphaned blocks
    let pruned = doc.prune_unreachable();
    println!("Pruned {} orphaned blocks", pruned.len());
    ```

=== "Python"
    ```python
    # Delete a single block (children become orphaned)
    doc.delete_block(block_id)

    # Delete with all descendants
    doc.delete_block(block_id, cascade=True)

    // Clean up orphaned blocks
    pruned = doc.prune_unreachable()
    print(f"Pruned {len(pruned)} orphaned blocks")
    ```

=== "JavaScript"
    ```javascript
    // Delete a single block (children become orphaned)
    doc.deleteBlock(blockId);

    // Delete with all descendants
    doc.deleteBlock(blockId, true);

    // Clean up orphaned blocks
    const pruned = doc.pruneUnreachable();
    console.log(`Pruned ${pruned.length} orphaned blocks`);
    ```

## Serialization

### To JSON

=== "Rust"
    ```rust
    let json = client.to_json(&doc).unwrap();
    println!("{}", json);
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

## Working with Edges (Relationships)

=== "Rust"
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

=== "Python"
    ```python
    from ucp_content import EdgeType

    # Add a reference relationship
    doc.add_edge(source_block_id, EdgeType.References, target_block_id)

    # Query relationships
    outgoing = doc.outgoing_edges(source_block_id)
    incoming = doc.incoming_edges(target_block_id)
    ```

=== "JavaScript"
    ```javascript
    import { EdgeType } from 'ucp-content';

    // Add a reference relationship
    doc.addEdge(sourceBlockId, EdgeType.References, targetBlockId);

    // Query relationships
    const outgoing = doc.outgoingEdges(sourceBlockId);
    const incoming = doc.incomingEdges(targetBlockId);
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

=== "Python"
    ```python
    from ucp_content import Document, EdgeType

    def main():
        # Create document
        doc = Document.create(title="My Document")
        root = doc.root_id
        
        # Build structure
        title_id = doc.add_block(root, "My Document", role="title")
        
        intro_id = doc.add_block(title_id, "This document demonstrates UCP.", role="intro")
        
        section_id = doc.add_block(title_id, "Main Content", role="heading2")
        
        code_id = doc.add_code(
            section_id, 
            "rust", 
            'println!("Hello!");',
            label="example-code"
        )
        doc.add_tag(code_id, "example")
        
        # Add a reference
        doc.add_edge(intro_id, EdgeType.References, code_id)
        
        # Validate
        issues = doc.validate()
        if not issues:
            print("Document is valid!")
        else:
            for severity, code, msg in issues:
                print(f"Issue: {msg}")
                
        # Statistics
        print(f"Total blocks: {doc.block_count()}")
        print(f"Code blocks: {len(doc.find_by_type('code'))}")

    if __name__ == "__main__":
        main()
    ```

=== "JavaScript"
    ```javascript
    import { Document, EdgeType } from 'ucp-content';

    function main() {
        // Create document
        const doc = Document.create("My Document");
        const root = doc.rootId;
        
        // Build structure
        const titleId = doc.addBlock(root, "My Document", "title");
        
        const introId = doc.addBlock(titleId, "This document demonstrates UCP.", "intro");
        
        const sectionId = doc.addBlock(titleId, "Main Content", "heading2");
        
        const codeId = doc.addCode(
            sectionId, 
            "rust", 
            'println!("Hello!");',
            "example-code"
        );
        doc.addTag(codeId, "example");
        
        // Add a reference
        doc.addEdge(introId, EdgeType.References, codeId);
        
        // Validate
        const issues = doc.validate();
        if (!issues) {
            console.log("Document is valid!");
        } else {
            // Note: validate() in JS might return null if valid or an array of issues
            for (const issue of issues) {
                console.log(`Issue: ${issue.message}`);
            }
        }
        
        // Statistics
        console.log(`Total blocks: ${doc.blockCount()}`);
        console.log(`Code blocks: ${doc.findByType('code').length}`);
    }

    main();
    ```

## Next Steps

- [Core Concepts](./concepts.md) - Deep dive into UCP architecture
- [UCM Core Documentation](../ucm-core/README.md) - Detailed type reference
- [UCL Syntax Guide](../ucl-parser/syntax.md) - Complete UCL reference
- [Examples](../examples/basic.md) - More code examples
