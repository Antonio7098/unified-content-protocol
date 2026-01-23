# Basic Examples

This document provides basic examples for getting started with UCP.

## Example 1: Creating a Simple Document

=== "Rust"
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

=== "Python"
    ```python
    from ucp_content import Document

    # Create a new document
    doc = Document.create()
    root = doc.root_id

    # Add a title
    title_id = doc.add_block(root, "My First Document", role="title")

    # Add a paragraph
    doc.add_block(title_id, "This is my first UCP document.", role="paragraph")

    print(f"Created document with {doc.block_count()} blocks")
    ```

=== "JavaScript"
    ```javascript
    import { Document } from 'ucp-content';

    // Create a new document
    const doc = Document.create();
    const root = doc.rootId;

    // Add a title
    const titleId = doc.addBlock(root, "My First Document", "title");

    // Add a paragraph
    doc.addBlock(titleId, "This is my first UCP document.", "paragraph");

    console.log(`Created document with ${doc.blockCount()} blocks`);
    ```

## Example 2: Using Convenience Methods

=== "Rust"
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

=== "Python"
    ```python
    from ucp_content import Document

    doc = Document.create()
    root = doc.root_id

    # Add text
    intro_id = doc.add_block(root, "Welcome to UCP!", role="intro")

    # Add code
    code_id = doc.add_code(root, "rust", 'fn hello() {\n    println!("Hello!");\n}')

    # Serialize to JSON
    print(doc.to_json())
    ```

=== "JavaScript"
    ```javascript
    import { Document } from 'ucp-content';

    const doc = Document.create();
    const root = doc.rootId;

    // Add text
    const introId = doc.addBlock(root, "Welcome to UCP!", "intro");

    // Add code
    const codeId = doc.addCode(root, "rust", 'fn hello() {\n    console.log("Hello!");\n}');

    // Serialize to JSON
    console.log(JSON.stringify(doc.toJson(), null, 2));
    ```

## Example 3: Basic UCL Commands

=== "Rust"
    ```rust
    use ucp_api::UcpClient;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let client = UcpClient::new();
        let mut doc = client.create_document();
        
        // Execute UCL commands
        // Note: In a real app, use dynamic IDs instead of hardcoded ones
        let results = client.execute_ucl(&mut doc, r#"
            APPEND blk_ff00000000000000000000 text WITH role="heading1" :: "Getting Started"
            APPEND blk_ff00000000000000000000 text WITH role="paragraph" :: "This guide will help you get started with UCP."
            APPEND blk_ff00000000000000000000 code :: "cargo add ucp-api"
        "#)?;
        
        println!("Executed {} commands", results.len());
        println!("Document has {} blocks", doc.block_count());
        
        Ok(())
    }
    ```

=== "Python"
    ```python
    from ucp_content import Document, execute_ucl

    doc = Document.create()
    root = doc.root_id

    # Execute UCL commands
    execute_ucl(doc, f"""
        APPEND {root} text WITH role="heading1" :: "Getting Started"
        APPEND {root} text WITH role="paragraph" :: "This guide will help you get started with UCP."
        APPEND {root} code :: "pip install ucp-content"
    """)

    print(f"Document has {doc.block_count()} blocks")
    ```

=== "JavaScript"
    ```javascript
    import { Document, executeUcl } from 'ucp-content';

    const doc = Document.create();
    const root = doc.rootId;

    // Execute UCL commands
    executeUcl(doc, `
        APPEND ${root} text WITH role="heading1" :: "Getting Started"
        APPEND ${root} text WITH role="paragraph" :: "This guide will help you get started with UCP."
        APPEND ${root} code :: "npm install ucp-content"
    `);

    console.log(`Document has ${doc.blockCount()} blocks`);
    ```

## Example 4: Working with Content Types

=== "Rust"
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

=== "Python"
    ```python
    from ucp_content import Document, Content

    doc = Document.create()
    root = doc.root_id

    # Text content
    doc.add_block_with_content(root, Content.text("Plain text content"), role="paragraph")

    # Markdown content
    doc.add_block_with_content(root, Content.markdown("**Bold** and *italic* text"), role="paragraph")

    # Code content
    doc.add_block_with_content(root, Content.code("python", "def greet(name):\n    print(f'Hello, {name}!')"), role="code")

    # Table content
    doc.add_block_with_content(root, Content.table([
        ["Name", "Age", "City"],
        ["Alice", "30", "NYC"],
        ["Bob", "25", "LA"]
    ]), role="table")

    # JSON content
    doc.add_block_with_content(root, Content.json({
        "name": "config",
        "debug": True,
        "level": 5
    }), role="metadata")

    print(f"Created {doc.block_count()} blocks with different content types")
    ```

=== "JavaScript"
    ```javascript
    import { Document, Content } from 'ucp-content';

    const doc = Document.create();
    const root = doc.rootId;

    // Text content
    doc.addBlockWithContent(root, Content.text("Plain text content"), "paragraph");

    // Markdown content
    doc.addBlockWithContent(root, Content.markdown("**Bold** and *italic* text"), "paragraph");

    // Code content
    doc.addBlockWithContent(root, Content.code("javascript", "console.log('Hello');"), "code");

    // Note: Table and JSON content creation helpers are not currently exposed in the JS SDK.
    // Use text or markdown for tables in JS, or code blocks for JSON.
    
    console.log(`Created ${doc.blockCount()} blocks`);
    ```

## Example 5: Building Document Structure

=== "Rust"
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

=== "Python"
    ```python
    from ucp_content import Document

    def print_structure(doc, block_id, depth=0):
        indent = "  " * depth
        block = doc.get_block(block_id)
        
        if block:
            # Simple content preview
            preview = "Block"
            # In a real app, inspect block.content properties
            print(f"{indent}{block_id}: {block.role}")
            
        for child_id in doc.children(block_id):
            print_structure(doc, child_id, depth + 1)

    doc = Document.create()
    root = doc.root_id

    # Chapter 1
    ch1 = doc.add_block(root, "Chapter 1: Introduction", role="heading1")
    
    # Sections
    sec1 = doc.add_block(ch1, "1.1 Overview", role="heading2")
    doc.add_block(sec1, "This section provides an overview.", role="paragraph")
    
    sec2 = doc.add_block(ch1, "1.2 Background", role="heading2")
    doc.add_block(sec2, "Some background information.", role="paragraph")
    
    # Chapter 2
    ch2 = doc.add_block(root, "Chapter 2: Details", role="heading1")
    doc.add_block(ch2, "Detailed content here.", role="paragraph")

    print("Document structure:")
    print_structure(doc, root)
    ```

=== "JavaScript"
    ```javascript
    import { Document } from 'ucp-content';

    function printStructure(doc, blockId, depth = 0) {
        const indent = "  ".repeat(depth);
        const block = doc.getBlock(blockId);
        
        if (block) {
            const role = block.role || "none";
            console.log(`${indent}${blockId}: ${role}`);
        }
        
        const children = doc.children(blockId);
        for (const childId of children) {
            printStructure(doc, childId, depth + 1);
        }
    }

    const doc = Document.create();
    const root = doc.rootId;

    // Chapter 1
    const ch1 = doc.addBlock(root, "Chapter 1: Introduction", "heading1");
    
    // Sections
    const sec1 = doc.addBlock(ch1, "1.1 Overview", "heading2");
    doc.addBlock(sec1, "This section provides an overview.", "paragraph");
    
    const sec2 = doc.addBlock(ch1, "1.2 Background", "heading2");
    doc.addBlock(sec2, "Some background information.", "paragraph");
    
    // Chapter 2
    const ch2 = doc.addBlock(root, "Chapter 2: Details", "heading1");
    doc.addBlock(ch2, "Detailed content here.", "paragraph");

    console.log("Document structure:");
    printStructure(doc, root);
    ```

## Example 6: Using Tags and Labels

=== "Rust"
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
        
        // Query by tag
        let important_blocks = doc.indices.find_by_tag("important");
        println!("Important blocks: {}", important_blocks.len());
        
        // Query by label
        if let Some(id) = doc.indices.find_by_label("important-note") {
            println!("Found important-note: {}", id);
        }
        
        Ok(())
    }
    ```

=== "Python"
    ```python
    from ucp_content import Document

    doc = Document.create()
    root = doc.root_id

    # Create blocks with tags and labels
    important_id = doc.add_block(
        root, 
        "Important note", 
        role="note",
        label="important-note",
        tags=["important", "review-needed"]
    )

    draft_id = doc.add_block(
        root,
        "Draft content",
        role="paragraph",
        label="draft-section",
        tags=["draft", "wip"]
    )

    # Query by tag
    important_blocks = doc.find_by_tag("important")
    print(f"Important blocks: {len(important_blocks)}")

    # Query by label
    found_id = doc.find_by_label("important-note")
    if found_id:
        print(f"Found important-note: {found_id}")
    ```

=== "JavaScript"
    ```javascript
    import { Document } from 'ucp-content';

    const doc = Document.create();
    const root = doc.rootId;

    // Create blocks
    const importantId = doc.addBlock(root, "Important note", "note", "important-note");
    doc.addTag(importantId, "important");
    doc.addTag(importantId, "review-needed");

    const draftId = doc.addBlock(root, "Draft content", "paragraph", "draft-section");
    doc.addTag(draftId, "draft");
    doc.addTag(draftId, "wip");

    // Query by tag
    const importantBlocks = doc.findByTag("important");
    console.log(`Important blocks: ${importantBlocks.length}`);

    // Query by label
    const foundId = doc.findByLabel("important-note");
    if (foundId) {
        console.log(`Found important-note: ${foundId}`);
    }
    ```

## Example 7: Querying Documents

=== "Rust"
    ```rust
    use ucm_core::{Block, Content, Document};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut doc = Document::create();
        let root = doc.root.clone();
        
        // Build a document
        let heading = Block::new(Content::text("Title"), Some("heading1"));
        let heading_id = doc.add_block(heading, &root)?;
        
        let code = Block::new(Content::code("rust", "fn main() {}"), Some("code"));
        doc.add_block(code, &heading_id)?;
        
        // Query by content type
        let code_blocks = doc.indices.find_by_type("code");
        println!("Code blocks: {}", code_blocks.len());
        
        // Get children of a block
        let children = doc.children(&heading_id);
        println!("Heading has {} children", children.len());
        
        // Get parent
        if let Some(parent) = doc.parent(&heading_id) {
            println!("Heading's parent: {}", parent);
        }
        
        Ok(())
    }
    ```

=== "Python"
    ```python
    from ucp_content import Document, Content

    doc = Document.create()
    root = doc.root_id

    heading_id = doc.add_block(root, "Title", role="heading1")
    doc.add_code(heading_id, "rust", "fn main() {}")

    # Query by content type
    code_blocks = doc.find_by_type("code")
    print(f"Code blocks: {len(code_blocks)}")

    # Children
    children = doc.children(heading_id)
    print(f"Heading has {len(children)} children")

    # Parent
    parent = doc.parent(heading_id)
    if parent:
        print(f"Heading's parent: {parent}")
    ```

=== "JavaScript"
    ```javascript
    import { Document } from 'ucp-content';

    const doc = Document.create();
    const root = doc.rootId;

    const headingId = doc.addBlock(root, "Title", "heading1");
    doc.addCode(headingId, "rust", "fn main() {}");

    // Query by content type
    const codeBlocks = doc.findByType("code");
    console.log(`Code blocks: ${codeBlocks.length}`);

    // Children
    const children = doc.children(headingId);
    console.log(`Heading has ${children.length} children`);

    // Parent
    const parent = doc.parent(headingId);
    if (parent) {
        console.log(`Heading's parent: ${parent}`);
    }
    ```

## Example 8: Markdown Conversion

=== "Rust"
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
    "#;

        let doc = parse_markdown(markdown)?;
        println!("Parsed {} blocks from Markdown", doc.block_count());
        
        // Render back to Markdown
        let rendered = render_markdown(&doc)?;
        println!("\nRendered Markdown:\n{}", rendered);
        
        Ok(())
    }
    ```

=== "Python"
    ```python
    from ucp_content import parse, render

    markdown = """
    # Welcome

    This is a simple document.
    """

    # Parse
    doc = parse(markdown)
    print(f"Parsed {doc.block_count()} blocks")

    # Render
    rendered = render(doc)
    print(f"Rendered:\n{rendered}")
    ```

=== "JavaScript"
    ```javascript
    import { parseMarkdown, renderMarkdown } from 'ucp-content';

    const markdown = `
    # Welcome

    This is a simple document.
    `;

    // Parse
    const doc = parseMarkdown(markdown);
    console.log(`Parsed ${doc.blockCount()} blocks`);

    // Render
    const rendered = renderMarkdown(doc);
    console.log(`Rendered:\n${rendered}`);
    ```

## Example 9: Document Validation

=== "Rust"
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
                println!("[{:?}] {}", issue.severity, issue.message);
            }
        }
        
        Ok(())
    }
    ```

=== "Python"
    ```python
    from ucp_content import Document

    doc = Document.create()
    doc.add_block(doc.root_id, "Content", role="paragraph")

    issues = doc.validate()

    if not issues:
        print("Document is valid!")
    else:
        for severity, code, msg in issues:
            print(f"[{severity}] {msg}")
    ```

=== "JavaScript"
    ```javascript
    import { Document } from 'ucp-content';

    const doc = Document.create();
    doc.addBlock(doc.rootId, "Content", "paragraph");

    const issues = doc.validate();

    if (!issues || issues.length === 0) {
        console.log("Document is valid!");
    } else {
        for (const issue of issues) {
            console.log(`[${issue.severity}] ${issue.message}`);
        }
    }
    ```


## Next Steps

- [Intermediate Examples](./intermediate.md) - More complex scenarios
- [Advanced Examples](./advanced.md) - Advanced usage patterns
- [UCL Commands](../ucl-parser/commands.md) - UCL reference
- [Semantic Roles](../ucm-core/semantic-roles.md) - Complete role reference
