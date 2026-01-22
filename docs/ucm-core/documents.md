# Documents

A **Document** is a collection of blocks organized in a hierarchical tree structure. It provides the container for content and manages relationships between blocks.

## Document Structure

=== "Rust"
    ```rust
    pub struct Document {
        /// Document identifier
        pub id: DocumentId,
        
        /// Root block ID
        pub root: BlockId,
        
        /// Adjacency map: parent -> ordered children
        pub structure: HashMap<BlockId, Vec<BlockId>>,
        
        /// All blocks in the document
        pub blocks: HashMap<BlockId, Block>,
        
        /// Document-level metadata
        pub metadata: DocumentMetadata,
        
        /// Secondary indices for fast lookup
        pub indices: DocumentIndices,
        
        /// Edge index for relationship traversal
        pub edge_index: EdgeIndex,
        
        /// Document version for concurrency control
        pub version: DocumentVersion,
    }
    ```

=== "Python"
    ```python
    class Document:
        @property
        def id(self) -> str: ...
        
        @property
        def root_id(self) -> str: ...
        
        @property
        def title(self) -> Optional[str]: ...
        
        @property
        def description(self) -> Optional[str]: ...
        
        @property
        def version(self) -> int: ...
        
        # ... methods for manipulation
    ```

=== "JavaScript"
    ```typescript
    class Document {
        get id(): string;
        get rootId(): string;
        get title(): string | undefined;
        get description(): string | undefined;
        get version(): number;
        
        // ... methods for manipulation
    }
    ```

## Creating Documents

### Basic Creation

=== "Rust"
    ```rust
    use ucm_core::{Document, DocumentId};

    // With specific ID
    let doc = Document::new(DocumentId::new("my-document"));

    // With auto-generated ID
    let doc = Document::create();

    println!("Document ID: {}", doc.id);
    println!("Root block: {}", doc.root);
    println!("Block count: {}", doc.block_count()); // 1 (root)
    ```

=== "Python"
    ```python
    from ucp_content import Document

    # With auto-generated ID (and optional title)
    doc = Document.create(title="My Document")

    print(f"Document ID: {doc.id}")
    print(f"Root block: {doc.root_id}")
    print(f"Block count: {doc.block_count()}")
    ```

=== "JavaScript"
    ```javascript
    import { Document } from 'ucp-content';

    // With auto-generated ID (and optional title)
    const doc = Document.create("My Document");

    console.log(`Document ID: ${doc.id}`);
    console.log(`Root block: ${doc.rootId}`);
    console.log(`Block count: ${doc.blockCount()}`);
    ```

### With Metadata

=== "Rust"
    ```rust
    use ucm_core::{Document, DocumentMetadata};

    let metadata = DocumentMetadata::new()
        .with_title("My Document");

    let doc = Document::create()
        .with_metadata(metadata);
    ```

=== "Python"
    ```python
    doc = Document.create(title="My Document")
    doc.set_description("A comprehensive guide")
    ```

=== "JavaScript"
    ```javascript
    const doc = Document.create("My Document");
    doc.setDescription("A comprehensive guide");
    ```

## Document Metadata

=== "Rust"
    ```rust
    pub struct DocumentMetadata {
        pub title: Option<String>,
        pub description: Option<String>,
        pub authors: Vec<String>,
        pub created_at: DateTime<Utc>,
        pub modified_at: DateTime<Utc>,
        pub language: Option<String>,  // ISO 639-1
        pub custom: HashMap<String, serde_json::Value>,
    }
    ```

### Working with Metadata

=== "Rust"
    ```rust
    let mut metadata = DocumentMetadata::new()
        .with_title("Technical Specification");

    metadata.authors.push("Alice".to_string());
    metadata.language = Some("en".to_string());
    metadata.custom.insert(
        "version".to_string(),
        serde_json::json!("1.0.0")
    );

    // Update modification time
    metadata.touch();
    ```

=== "Python"
    ```python
    doc.set_title("Technical Specification")
    doc.set_description("Version 1.0.0")
    
    print(f"Created: {doc.created_at}")
    print(f"Modified: {doc.modified_at}")
    ```

=== "JavaScript"
    ```javascript
    doc.setTitle("Technical Specification");
    doc.setDescription("Version 1.0.0");
    
    console.log(`Created: ${doc.createdAt}`);
    console.log(`Modified: ${doc.modifiedAt}`);
    ```

## Adding Blocks

### Basic Addition

=== "Rust"
    ```rust
    use ucm_core::{Document, Block, Content};

    let mut doc = Document::create();
    let root = doc.root.clone();

    // Add block as child of root
    let block = Block::new(Content::text("Hello"), Some("intro"));
    let block_id = doc.add_block(block, &root).unwrap();

    // Add block at specific position
    let block2 = Block::new(Content::text("First!"), Some("intro"));
    let block2_id = doc.add_block_at(block2, &root, 0).unwrap(); // Insert at beginning
    ```

=== "Python"
    ```python
    root = doc.root_id
    
    # Add block as child of root
    block_id = doc.add_block(root, "Hello", role="intro")
    
    # Add block at specific position
    block2_id = doc.add_block_with_content(
        parent_id=root,
        content=Content.text("First!"),
        role="intro",
        index=0
    )
    ```

=== "JavaScript"
    ```javascript
    const root = doc.rootId;
    
    // Add block as child of root
    const blockId = doc.addBlock(root, "Hello", "intro");
    
    // Add block at specific position
    const block2Id = doc.addBlockAt(root, "First!", 0, "intro");
    ```

### Building Hierarchies

=== "Rust"
    ```rust
    let mut doc = Document::create();
    let root = doc.root.clone();

    // Create chapter
    let chapter = Block::new(Content::text("Chapter 1"), Some("heading1"));
    let chapter_id = doc.add_block(chapter, &root).unwrap();

    // Add sections under chapter
    let section1 = Block::new(Content::text("Section 1.1"), Some("heading2"));
    let section1_id = doc.add_block(section1, &chapter_id).unwrap();

    let section2 = Block::new(Content::text("Section 1.2"), Some("heading2"));
    let section2_id = doc.add_block(section2, &chapter_id).unwrap();

    // Add content under section
    let para = Block::new(Content::text("Paragraph content..."), Some("paragraph"));
    doc.add_block(para, &section1_id).unwrap();
    ```

=== "Python"
    ```python
    # Create chapter
    chapter_id = doc.add_block(doc.root_id, "Chapter 1", role="heading1")
    
    # Add sections under chapter
    section1_id = doc.add_block(chapter_id, "Section 1.1", role="heading2")
    section2_id = doc.add_block(chapter_id, "Section 1.2", role="heading2")
    
    # Add content under section
    doc.add_block(section1_id, "Paragraph content...", role="paragraph")
    ```

=== "JavaScript"
    ```javascript
    // Create chapter
    const chapterId = doc.addBlock(doc.rootId, "Chapter 1", "heading1");
    
    // Add sections under chapter
    const section1Id = doc.addBlock(chapterId, "Section 1.1", "heading2");
    const section2Id = doc.addBlock(chapterId, "Section 1.2", "heading2");
    
    // Add content under section
    doc.addBlock(section1Id, "Paragraph content...", "paragraph");
    ```

## Querying Documents

### Get Block by ID

=== "Rust"
    ```rust
    // Immutable reference
    if let Some(block) = doc.get_block(&block_id) {
        println!("Content type: {}", block.content_type());
    }

    // Mutable reference
    if let Some(block) = doc.get_block_mut(&block_id) {
        block.metadata.tags.push("modified".to_string());
    }
    ```

=== "Python"
    ```python
    block = doc.get_block(block_id)
    if block:
        print(f"Content type: {block.content_type}")
    ```

=== "JavaScript"
    ```javascript
    const block = doc.getBlock(blockId);
    if (block) {
        console.log(`Content type: ${block.contentType}`);
    }
    ```

### Get Children

=== "Rust"
    ```rust
    let children: &[BlockId] = doc.children(&parent_id);
    for child_id in children {
        let child = doc.get_block(child_id).unwrap();
        println!("Child: {}", child_id);
    }
    ```

=== "Python"
    ```python
    children = doc.children(parent_id)
    for child_id in children:
        print(f"Child: {child_id}")
    ```

=== "JavaScript"
    ```javascript
    const children = doc.children(parentId);
    for (const childId of children) {
        console.log(`Child: ${childId}`);
    }
    ```

### Get Parent

=== "Rust"
    ```rust
    if let Some(parent_id) = doc.parent(&child_id) {
        println!("Parent: {}", parent_id);
    }
    ```

=== "Python"
    ```python
    parent_id = doc.parent(child_id)
    if parent_id:
        print(f"Parent: {parent_id}")
    ```

=== "JavaScript"
    ```javascript
    const parentId = doc.parent(childId);
    if (parentId) {
        console.log(`Parent: ${parentId}`);
    }
    ```

### Get Descendants

=== "Rust"
    ```rust
    // Get all descendants (recursive)
    let descendants: Vec<BlockId> = doc.descendants(&block_id);
    println!("Block has {} descendants", descendants.len());
    ```

=== "Python"
    ```python
    descendants = doc.descendants(block_id)
    print(f"Block has {len(descendants)} descendants")
    ```

=== "JavaScript"
    ```javascript
    const descendants = doc.descendants(blockId);
    console.log(`Block has ${descendants.length} descendants`);
    ```

### Check Ancestry

=== "Rust"
    ```rust
    // Check if block_a is an ancestor of block_b
    if doc.is_ancestor(&block_a, &block_b) {
        println!("block_a is an ancestor of block_b");
    }
    ```

=== "Python"
    ```python
    if doc.is_ancestor(block_a, block_b):
        print("block_a is an ancestor of block_b")
    ```

=== "JavaScript"
    ```javascript
    if (doc.isAncestor(blockA, blockB)) {
        console.log("blockA is an ancestor of blockB");
    }
    ```

## Secondary Indices

Documents maintain indices for fast lookup:

=== "Rust"
    ```rust
    pub struct DocumentIndices {
        pub by_tag: HashMap<String, HashSet<BlockId>>,
        pub by_role: HashMap<String, HashSet<BlockId>>,
        pub by_content_type: HashMap<String, HashSet<BlockId>>,
        pub by_label: HashMap<String, BlockId>,
    }
    ```

### Find by Tag

=== "Rust"
    ```rust
    let important_blocks = doc.indices.find_by_tag("important");
    for block_id in important_blocks {
        println!("Important block: {}", block_id);
    }
    ```

=== "Python"
    ```python
    important_blocks = doc.find_by_tag("important")
    for block_id in important_blocks:
        print(f"Important block: {block_id}")
    ```

=== "JavaScript"
    ```javascript
    const importantBlocks = doc.findByTag("important");
    for (const blockId of importantBlocks) {
        console.log(`Important block: ${blockId}`);
    }
    ```

### Find by Content Type

=== "Rust"
    ```rust
    let code_blocks = doc.indices.find_by_type("code");
    let text_blocks = doc.indices.find_by_type("text");
    let table_blocks = doc.indices.find_by_type("table");
    ```

=== "Python"
    ```python
    code_blocks = doc.find_by_type("code")
    text_blocks = doc.find_by_type("text")
    ```

=== "JavaScript"
    ```javascript
    const codeBlocks = doc.findByType("code");
    const textBlocks = doc.findByType("text");
    ```

### Find by Label

=== "Rust"
    ```rust
    if let Some(block_id) = doc.indices.find_by_label("main-content") {
        let block = doc.get_block(&block_id).unwrap();
        // ...
    }
    ```

=== "Python"
    ```python
    block_id = doc.find_by_label("main-content")
    if block_id:
        block = doc.get_block(block_id)
    ```

=== "JavaScript"
    ```javascript
    const blockId = doc.findByLabel("main-content");
    if (blockId) {
        const block = doc.getBlock(blockId);
    }
    ```

### Rebuild Indices

After bulk modifications, rebuild indices:

=== "Rust"
    ```rust
    doc.rebuild_indices();
    ```

## Moving Blocks

### Move to New Parent

=== "Rust"
    ```rust
    // Move block to end of new parent's children
    doc.move_block(&block_id, &new_parent_id).unwrap();

    // Move to specific position
    doc.move_block_at(&block_id, &new_parent_id, 0).unwrap(); // First position
    ```

=== "Python"
    ```python
    # Move to new parent
    doc.move_block(block_id, new_parent_id)
    
    # Move to specific position
    doc.move_block(block_id, new_parent_id, index=0)
    ```

=== "JavaScript"
    ```javascript
    // Move to new parent
    doc.moveBlock(blockId, newParentId);
    
    // Move to specific position
    doc.moveBlock(blockId, newParentId, 0);
    ```

### Cycle Detection

Moving a block to one of its descendants is prevented:

=== "Rust"
    ```rust
    let result = doc.move_block(&parent_id, &child_id);
    assert!(result.is_err()); // CycleDetected error
    ```

## Deleting Blocks

### Delete Single Block

=== "Rust"
    ```rust
    // Delete block (children become orphaned)
    let deleted_block = doc.delete_block(&block_id).unwrap();
    ```

=== "Python"
    ```python
    doc.delete_block(block_id)
    ```

=== "JavaScript"
    ```javascript
    doc.deleteBlock(blockId);
    ```

### Delete with Cascade

=== "Rust"
    ```rust
    // Delete block and all descendants
    let deleted_blocks = doc.delete_cascade(&block_id).unwrap();
    println!("Deleted {} blocks", deleted_blocks.len());
    ```

=== "Python"
    ```python
    deleted_blocks = doc.delete_block(block_id, cascade=True)
    print(f"Deleted {len(deleted_blocks)} blocks")
    ```

=== "JavaScript"
    ```javascript
    const deletedBlocks = doc.deleteBlock(blockId, true);
    console.log(`Deleted ${deletedBlocks.length} blocks`);
    ```

### Remove from Structure Only

=== "Rust"
    ```rust
    // Remove from tree but keep in document (orphan)
    doc.remove_from_structure(&block_id);

    // Block is now orphaned but still exists
    assert!(doc.get_block(&block_id).is_some());
    assert!(!doc.is_reachable(&block_id));
    ```

## Orphan Management

### Find Orphans

=== "Rust"
    ```rust
    let orphans = doc.find_orphans();
    for orphan_id in &orphans {
        println!("Orphaned block: {}", orphan_id);
    }
    ```

=== "Python"
    ```python
    orphans = doc.find_orphans()
    for orphan_id in orphans:
        print(f"Orphaned block: {orphan_id}")
    ```

=== "JavaScript"
    ```javascript
    const orphans = doc.findOrphans();
    for (const orphanId of orphans) {
        console.log(`Orphaned block: ${orphanId}`);
    }
    ```

### Check Reachability

=== "Rust"
    ```rust
    if doc.is_reachable(&block_id) {
        println!("Block is reachable from root");
    } else {
        println!("Block is orphaned");
    }
    ```

=== "Python"
    ```python
    if doc.is_reachable(block_id):
        print("Reachable")
    ```

=== "JavaScript"
    ```javascript
    if (doc.isReachable(blockId)) {
        console.log("Reachable");
    }
    ```

### Prune Orphans

=== "Rust"
    ```rust
    // Remove all unreachable blocks
    let pruned = doc.prune_unreachable();
    println!("Pruned {} orphaned blocks", pruned.len());
    ```

=== "Python"
    ```python
    pruned = doc.prune_unreachable()
    print(f"Pruned {len(pruned)} orphaned blocks")
    ```

=== "JavaScript"
    ```javascript
    const pruned = doc.pruneUnreachable();
    console.log(`Pruned ${pruned.length} orphaned blocks`);
    ```

### Block State

=== "Rust"
    ```rust
    use ucm_core::block::BlockState;

    let state = doc.block_state(&block_id);
    match state {
        BlockState::Live => println!("Reachable from root"),
        BlockState::Orphaned => println!("Exists but unreachable"),
        BlockState::Deleted => println!("Not in document"),
    }
    ```

## Validation

### Validate Document

=== "Rust"
    ```rust
    let issues = doc.validate();

    for issue in &issues {
        match issue.severity {
            ValidationSeverity::Error => eprintln!("ERROR: {}", issue.message),
            ValidationSeverity::Warning => println!("WARNING: {}", issue.message),
            ValidationSeverity::Info => println!("INFO: {}", issue.message),
        }
    }

    if issues.iter().any(|i| i.severity == ValidationSeverity::Error) {
        println!("Document has errors!");
    }
    ```

=== "Python"
    ```python
    issues = doc.validate()
    for severity, code, message in issues:
        print(f"{severity}: {message}")
    ```

=== "JavaScript"
    ```javascript
    const issues = doc.validate();
    for (const issue of issues) {
        console.log(`${issue.severity}: ${issue.message}`);
    }
    ```

### Validation Checks

The validator checks for:
- **Cycles** in document structure
- **Orphaned blocks** (warning)
- **Dangling references** (edges to non-existent blocks)
- **Invalid structure** (references to missing blocks)

## Token Estimation

=== "Rust"
    ```rust
    use ucm_core::metadata::TokenModel;

    // Total tokens for the document
    let gpt4_tokens = doc.total_tokens(TokenModel::GPT4);
    let claude_tokens = doc.total_tokens(TokenModel::Claude);

    println!("GPT-4 tokens: {}", gpt4_tokens);
    println!("Claude tokens: {}", claude_tokens);
    ```

## Edge Index

The document maintains an edge index for relationship traversal:

=== "Rust"
    ```rust
    // Get outgoing edges from a block
    let outgoing = doc.edge_index.outgoing_from(&block_id);
    for (edge_type, target) in outgoing {
        println!("{} -> {} ({:?})", block_id, target, edge_type);
    }

    // Get incoming edges to a block
    let incoming = doc.edge_index.incoming_to(&block_id);

    // Check if edge exists
    let has_ref = doc.edge_index.has_edge(&source, &target, &EdgeType::References);

    // Get edges of specific type
    let refs = doc.edge_index.outgoing_of_type(&block_id, &EdgeType::References);
    ```

=== "Python"
    ```python
    # Get outgoing edges
    outgoing = doc.outgoing_edges(block_id)
    
    # Get incoming edges
    incoming = doc.incoming_edges(block_id)
    ```

=== "JavaScript"
    ```javascript
    // Get outgoing edges
    const outgoing = doc.outgoingEdges(blockId);
    
    // Get incoming edges
    const incoming = doc.incomingEdges(blockId);
    ```

## Complete Example

=== "Rust"
    ```rust
    use ucm_core::{Document, DocumentMetadata, Block, Content, Edge, EdgeType};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Create document with metadata
        let metadata = DocumentMetadata::new()
            .with_title("Technical Guide");
        
        let mut doc = Document::create().with_metadata(metadata);
        let root = doc.root.clone();
        
        // Build structure
        let intro = Block::new(Content::text("Introduction"), Some("heading1"))
            .with_label("intro");
        let intro_id = doc.add_block(intro, &root)?;
        
        let overview = Block::new(
            Content::text("This guide covers..."),
            Some("paragraph")
        );
        let overview_id = doc.add_block(overview, &intro_id)?;
        
        let chapter1 = Block::new(Content::text("Getting Started"), Some("heading1"))
            .with_label("chapter-1");
        let chapter1_id = doc.add_block(chapter1, &root)?;
        
        let code_example = Block::new(
            Content::code("rust", "fn main() {\n    println!(\"Hello!\");\n}"),
            Some("code")
        ).with_tag("example");
        let code_id = doc.add_block(code_example, &chapter1_id)?;
        
        // Add reference from overview to code
        let edge = Edge::new(EdgeType::References, code_id.clone());
        doc.get_block_mut(&overview_id).unwrap().add_edge(edge.clone());
        doc.edge_index.add_edge(&overview_id, &edge);
        
        // Query document
        println!("Total blocks: {}", doc.block_count());
        println!("Code blocks: {}", doc.indices.find_by_type("code").len());
        println!("Example blocks: {}", doc.indices.find_by_tag("example").len());
        
        // Find by label
        if let Some(id) = doc.indices.find_by_label("chapter-1") {
            println!("Found chapter-1: {}", id);
        }
        
        // Validate
        let issues = doc.validate();
        if issues.is_empty() {
            println!("Document is valid!");
        }
        
        // Check structure
        let intro_children = doc.children(&intro_id);
        println!("Intro has {} children", intro_children.len());
        
        Ok(())
    }
    ```

=== "Python"
    ```python
    from ucp_content import Document, EdgeType

    # Create document
    doc = Document.create(title="Technical Guide")
    root = doc.root_id
    
    # Build structure
    intro_id = doc.add_block(root, "Introduction", role="heading1", label="intro")
    
    overview_id = doc.add_block(intro_id, "This guide covers...", role="paragraph")
    
    chapter1_id = doc.add_block(root, "Getting Started", role="heading1", label="chapter-1")
    
    code_id = doc.add_code(
        chapter1_id,
        "rust",
        'fn main() {\n    println!("Hello!");\n}',
        label="example-code"
    )
    doc.add_tag(code_id, "example")
    
    # Add reference
    doc.add_edge(overview_id, EdgeType.References, code_id)
    
    # Query document
    print(f"Total blocks: {doc.block_count()}")
    print(f"Code blocks: {len(doc.find_by_type('code'))}")
    print(f"Example blocks: {len(doc.find_by_tag('example'))}")
    
    # Find by label
    chap1 = doc.find_by_label("chapter-1")
    if chap1:
        print(f"Found chapter-1: {chap1}")
        
    # Validate
    issues = doc.validate()
    if not issues:
        print("Document is valid!")
    ```

## Best Practices

### 1. Use Meaningful Document IDs

=== "Rust"
    ```rust
    // Good - descriptive
    Document::new(DocumentId::new("user-guide-v2"))
    Document::new(DocumentId::new("api-spec-2024-01"))

    // Less ideal - generic
    Document::create() // Auto-generated ID
    ```

### 2. Maintain Document Metadata

=== "Rust"
    ```rust
    let mut doc = Document::create();
    doc.metadata.title = Some("Important Document".to_string());
    doc.metadata.authors.push("Author Name".to_string());
    doc.metadata.language = Some("en".to_string());
    ```

=== "Python"
    ```python
    doc = Document.create()
    doc.set_title("Important Document")
    doc.set_description("Author Name")
    ```

=== "JavaScript"
    ```javascript
    const doc = Document.create();
    doc.setTitle("Important Document");
    doc.setDescription("Author Name");
    ```

### 3. Use Labels for Key Blocks

=== "Rust"
    ```rust
    let toc = Block::new(Content::text("Table of Contents"), Some("toc"))
        .with_label("table-of-contents");

    // Later, find easily
    let toc_id = doc.indices.find_by_label("table-of-contents");
    ```

### 4. Validate After Bulk Operations

=== "Rust"
    ```rust
    // After many modifications
    doc.rebuild_indices();
    let issues = doc.validate();
    ```

### 5. Clean Up Orphans

=== "Rust"
    ```rust
    // Periodically or before serialization
    doc.prune_unreachable();
    ```

## See Also

- [Blocks](./blocks.md) - Block structure
- [Edges](./edges.md) - Relationship management
- [Metadata](./metadata.md) - Document and block metadata
