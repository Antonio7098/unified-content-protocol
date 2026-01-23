# Blocks

The **Block** is the fundamental unit of content in UCM. Every piece of content in a document is represented as a block.

## Block Structure

=== "Rust"
    ```rust
    pub struct Block {
        /// Unique, content-derived identifier
        pub id: BlockId,
        
        /// The actual content
        pub content: Content,
        
        /// Block metadata (role, tags, labels, etc.)
        pub metadata: BlockMetadata,
        
        /// Explicit relationships to other blocks
        pub edges: Vec<Edge>,
        
        /// Version for optimistic concurrency control
        pub version: Version,
    }
    ```

=== "Python"
    ```python
    class Block:
        @property
        def id(self) -> str: ...
        
        @property
        def content(self) -> Content: ...
        
        @property
        def content_type(self) -> str: ...
        
        @property
        def role(self) -> Optional[str]: ...
        
        @property
        def label(self) -> Optional[str]: ...
        
        @property
        def tags(self) -> List[str]: ...
        
        @property
        def edges(self) -> List[Edge]: ...
        
        @property
        def version(self) -> int: ...
    ```

=== "JavaScript"
    ```typescript
    // Blocks are returned as plain JSON objects
    interface Block {
        id: string;
        contentType: string;
        content: {
            text?: string;
            language?: string;
            source?: string;
            // ... other content fields
        };
        role?: string;
        label?: string;
        tags: string[];
        version: number;
    }
    ```

## Creating Blocks

### Basic Creation

=== "Rust"
    ```rust
    use ucm_core::{Block, Content};

    // Create with content and semantic role
    let block = Block::new(Content::text("Hello, world!"), Some("intro"));

    // Create without semantic role
    let block = Block::new(Content::text("Plain text"), None);

    // Create with specific ID (for deserialization or testing)
    let block = Block::with_id(block_id, Content::text("Test"));

    // Create root block
    let root = Block::root();
    ```

=== "Python"
    In Python, blocks are typically created via the `Document` to ensure proper ID generation and indexing.

    ```python
    # Add a block to a document
    block_id = doc.add_block(
        parent_id=doc.root_id,
        content="Hello, world!",
        role="intro"
    )
    ```

=== "JavaScript"
    In JavaScript, blocks are created via the `Document` instance.

    ```javascript
    // Add a block to a document
    const blockId = doc.addBlock(
        doc.rootId,
        "Hello, world!",
        "intro"
    );
    ```

### Builder Pattern

=== "Rust"
    ```rust
    use ucm_core::{Block, Content, Edge, EdgeType};

    let block = Block::new(Content::text("Important note"), Some("note"))
        .with_label("warning-001")
        .with_tag("important")
        .with_tag("review-needed")
        .with_edge(Edge::new(EdgeType::References, other_block_id));
    ```

=== "Python"
    ```python
    # Python uses optional arguments for these properties
    block_id = doc.add_block(
        parent_id=root_id,
        content="Important note",
        role="note",
        label="warning-001",
        tags=["important", "review-needed"]
    )
    ```

=== "JavaScript"
    ```javascript
    const blockId = doc.addBlock(
        rootId,
        "Important note",
        "note",
        "warning-001" // label
    );
    
    // Add tags separately
    doc.addTag(blockId, "important");
    doc.addTag(blockId, "review-needed");
    ```

### With Metadata

=== "Rust"
    ```rust
    use ucm_core::{Block, Content, BlockMetadata, id::compute_content_hash};

    let content = Content::text("Custom metadata example");
    let hash = compute_content_hash(&content);

    let metadata = BlockMetadata::new(hash)
        .with_label("custom-block")
        .with_tags(["tag1", "tag2"])
        .with_summary("A brief summary for folding");

    let block = Block::new(content, Some("example"))
        .with_metadata(metadata);
    ```

=== "Python"
    Metadata is managed through `Document` methods in the high-level API.

    ```python
    # Create block
    block_id = doc.add_block(root_id, "Custom metadata example", "example")
    
    # Update metadata
    doc.set_label(block_id, "custom-block")
    doc.add_tag(block_id, "tag1")
    doc.add_tag(block_id, "tag2")
    ```

=== "JavaScript"
    Metadata is managed through `Document` methods.

    ```javascript
    const blockId = doc.addBlock(rootId, "Custom metadata example", "example");
    doc.setLabel(blockId, "custom-block");
    doc.addTag(blockId, "tag1");
    doc.addTag(blockId, "tag2");
    ```

## Block Properties

### ID

The block ID is a content-addressed identifier:

=== "Rust"
    ```rust
    let block = Block::new(Content::text("Hello"), Some("intro"));

    println!("Block ID: {}", block.id);  // blk_a1b2c3d4e5f6...

    // IDs are deterministic
    let block2 = Block::new(Content::text("Hello"), Some("intro"));
    assert_eq!(block.id, block2.id);

    // Different role = different ID
    let block3 = Block::new(Content::text("Hello"), Some("conclusion"));
    assert_ne!(block.id, block3.id);
    ```

=== "Python"
    ```python
    block = doc.get_block(block_id)
    print(f"Block ID: {block.id}")
    ```

=== "JavaScript"
    ```javascript
    const block = doc.getBlock(blockId);
    console.log(`Block ID: ${block.id}`);
    ```

### Content Type

=== "Rust"
    ```rust
    let text_block = Block::new(Content::text("Hello"), None);
    assert_eq!(text_block.content_type(), "text");

    let code_block = Block::new(Content::code("rust", "fn main() {}"), None);
    assert_eq!(code_block.content_type(), "code");
    ```

=== "Python"
    ```python
    block = doc.get_block(block_id)
    print(f"Type: {block.content_type}") # "text", "code", etc.
    ```

=== "JavaScript"
    ```javascript
    const block = doc.getBlock(blockId);
    console.log(`Type: ${block.contentType}`);
    ```

### Root Detection

=== "Rust"
    ```rust
    let root = Block::root();
    assert!(root.is_root());

    let regular = Block::new(Content::text("Not root"), None);
    assert!(!regular.is_root());
    ```

=== "Python"
    ```python
    block = doc.get_block(block_id)
    if block.is_root():
        print("This is the root block")
    ```

=== "JavaScript"
    ```javascript
    if (block.id === doc.rootId) {
        console.log("This is the root block");
    }
    ```

### Size and Tokens

=== "Rust"
    ```rust
    let block = Block::new(Content::text("Hello, world!"), None);

    // Content size in bytes
    let size = block.size_bytes();

    // Token estimate
    let tokens = block.token_estimate();
    println!("GPT-4 tokens: {}", tokens.gpt4);
    println!("Claude tokens: {}", tokens.claude);
    ```

=== "Python"
    ```python
    block = doc.get_block(block_id)
    print(f"Size: {block.size_bytes} bytes")
    print(f"Tokens: {block.token_estimate}")
    ```

=== "JavaScript"
    ```javascript
    // Size and token estimation not directly exposed on block object in JS
    // Use IdMapper for token estimation
    import { IdMapper } from 'ucp-content';
    const mapper = new IdMapper();
    const estimate = mapper.estimateTokenSavings(block.text);
    console.log(`Tokens: ${estimate.originalTokens}`);
    ```

## Modifying Blocks

### Update Content

=== "Rust"
    ```rust
    let mut block = Block::new(Content::text("Original"), Some("intro"));
    let original_id = block.id.clone();

    // Update content (regenerates ID)
    block.update_content(Content::text("Updated"), Some("intro"));

    assert_ne!(block.id, original_id);  // ID changed
    assert!(block.version.counter > 1); // Version incremented
    ```

=== "Python"
    ```python
    # Updates modify the block in the document (and regenerate ID internally)
    doc.edit_block(block_id, "Updated content", role="intro")
    ```

=== "JavaScript"
    ```javascript
    // Updates modify the block in the document
    doc.editBlock(blockId, "Updated content", "intro");
    ```

### Managing Edges

=== "Rust"
    ```rust
    use ucm_core::{Block, Content, Edge, EdgeType, BlockId};

    let mut block = Block::new(Content::text("Source"), None);
    let target_id = BlockId::from_bytes([1u8; 12]);

    // Add edge
    let edge = Edge::new(EdgeType::References, target_id.clone());
    block.add_edge(edge);

    // Query edges
    let refs = block.edges_of_type(&EdgeType::References);
    assert_eq!(refs.len(), 1);

    // Remove edge
    block.remove_edge(&target_id, &EdgeType::References);
    assert!(block.edges.is_empty());
    ```

=== "Python"
    ```python
    from ucp_content import EdgeType

    # Add edge via document
    doc.add_edge(source_id, EdgeType.References, target_id)

    # Remove edge via document
    doc.remove_edge(source_id, EdgeType.References, target_id)
    ```

=== "JavaScript"
    ```javascript
    import { EdgeType } from 'ucp-content';

    // Add edge via document
    doc.addEdge(sourceId, EdgeType.References, targetId);

    // Remove edge via document
    doc.removeEdge(sourceId, EdgeType.References, targetId);
    ```

### Tags

=== "Rust"
    ```rust
    let block = Block::new(Content::text("Test"), None)
        .with_tag("important")
        .with_tag("draft");

    assert!(block.has_tag("important"));
    assert!(block.has_tag("draft"));
    assert!(!block.has_tag("final"));
    ```

=== "Python"
    ```python
    doc.add_tag(block_id, "important")
    doc.add_tag(block_id, "draft")
    
    # Check tags on retrieved block
    block = doc.get_block(block_id)
    assert "important" in block.tags
    ```

=== "JavaScript"
    ```javascript
    doc.addTag(blockId, "important");
    doc.addTag(blockId, "draft");
    
    // Check tags on retrieved block
    const block = doc.getBlock(blockId);
    console.log(block.tags.includes("important"));
    ```

## Block Lifecycle States

=== "Rust"
    ```rust
    pub enum BlockState {
        /// Reachable from document root
        Live,
        /// Not reachable from root but not deleted
        Orphaned,
        /// Marked for garbage collection
        Deleted,
    }
    ```

=== "Python"
    *State enum not directly exposed in Python bindings.*

=== "JavaScript"
    *State enum not directly exposed in JavaScript bindings.*

### Checking State

=== "Rust"
    ```rust
    use ucm_core::{Document, Block, Content};

    let mut doc = Document::create();
    let root = doc.root.clone();

    let block = Block::new(Content::text("Test"), None);
    let block_id = doc.add_block(block, &root).unwrap();

    // Check state
    assert_eq!(doc.block_state(&block_id), BlockState::Live);

    // Remove from structure (orphan it)
    doc.remove_from_structure(&block_id);
    assert_eq!(doc.block_state(&block_id), BlockState::Orphaned);

    // Delete completely
    doc.delete_block(&block_id).unwrap();
    assert_eq!(doc.block_state(&block_id), BlockState::Deleted);
    ```

=== "Python"
    ```python
    # Check reachability
    is_live = doc.is_reachable(block_id)
    
    # Check if exists (not deleted)
    exists = doc.get_block(block_id) is not None
    ```

=== "JavaScript"
    ```javascript
    // Check reachability
    const isLive = doc.isReachable(blockId);
    
    // Check if exists
    const exists = doc.getBlock(blockId) !== undefined;
    ```

## Serialization

Blocks implement `Serialize` and `Deserialize`:

=== "Rust"
    ```rust
    use ucm_core::{Block, Content};
    use serde_json;

    let block = Block::new(Content::text("Hello"), Some("intro"))
        .with_label("greeting")
        .with_tag("example");

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&block).unwrap();
    println!("{}", json);

    // Deserialize from JSON
    let restored: Block = serde_json::from_str(&json).unwrap();
    assert_eq!(block.id, restored.id);
    ```

=== "Python"
    ```python
    # Blocks retrieved from Document are typically used as-is
    # For JSON representation of the whole document:
    json_str = doc.to_json()
    ```

=== "JavaScript"
    ```javascript
    // Blocks are already JSON-compatible objects
    const block = doc.getBlock(blockId);
    console.log(JSON.stringify(block, null, 2));
    ```

### JSON Structure

```json
{
  "id": "a1b2c3d4e5f6a1b2c3d4e5f6",
  "content": {
    "type": "text",
    "text": "Hello",
    "format": "plain"
  },
  "metadata": {
    "semantic_role": {
      "category": "intro"
    },
    "label": "greeting",
    "tags": ["example"],
    "content_hash": "...",
    "created_at": "2024-01-01T00:00:00Z",
    "modified_at": "2024-01-01T00:00:00Z"
  },
  "version": {
    "counter": 1,
    "timestamp": "2024-01-01T00:00:00Z"
  }
}
```

## Best Practices

### 1. Use Semantic Roles

Always assign semantic roles when the block's purpose is known:

=== "Rust"
    ```rust
    // Good - clear purpose
    let intro = Block::new(Content::text("..."), Some("intro.hook"));
    let code = Block::new(Content::code("rust", "..."), Some("code"));

    // Less ideal - no semantic information
    let block = Block::new(Content::text("..."), None);
    ```

=== "Python"
    ```python
    doc.add_block(parent, "...", role="intro.hook")
    ```

=== "JavaScript"
    ```javascript
    doc.addBlock(parent, "...", "intro.hook");
    ```

### 2. Use Labels for Unique References

Labels provide human-readable identifiers:

=== "Rust"
    ```rust
    let block = Block::new(Content::text("Important"), None)
        .with_label("main-warning");

    // Later, find by label
    let id = doc.indices.find_by_label("main-warning");
    ```

=== "Python"
    ```python
    doc.add_block(parent, "Important", label="main-warning")
    
    # Find by label
    id = doc.find_by_label("main-warning")
    ```

=== "JavaScript"
    ```javascript
    doc.addBlock(parent, "Important", null, "main-warning");
    
    // Find by label
    const id = doc.findByLabel("main-warning");
    ```

### 3. Use Tags for Categories

Tags enable filtering and grouping:

=== "Rust"
    ```rust
    let block = Block::new(Content::text("..."), None)
        .with_tag("draft")
        .with_tag("needs-review")
        .with_tag("chapter-1");

    // Find all drafts
    let drafts = doc.indices.find_by_tag("draft");
    ```

=== "Python"
    ```python
    id = doc.add_block(parent, "...")
    doc.add_tag(id, "draft")
    
    # Find drafts
    drafts = doc.find_by_tag("draft")
    ```

=== "JavaScript"
    ```javascript
    const id = doc.addBlock(parent, "...");
    doc.addTag(id, "draft");
    
    // Find drafts
    const drafts = doc.findByTag("draft");
    ```

### 4. Leverage Content Types

Use the appropriate content type for your data:

=== "Rust"
    ```rust
    // Use Text for prose
    Content::text("A paragraph of text...")

    // Use Code for source code
    Content::code("python", "def hello(): pass")

    // Use Table for structured data
    Content::table(vec![...])

    // Use JSON for structured configuration
    Content::json(serde_json::json!({...}))
    ```

=== "Python"
    ```python
    from ucp_content import Content
    
    # Text
    Content.text("A paragraph...")
    
    # Code
    Content.code("python", "def hello(): pass")
    
    # Table and JSON are also supported
    ```

=== "JavaScript"
    ```javascript
    import { Content } from 'ucp-content';
    
    // Text
    Content.text("A paragraph...");
    
    // Code
    Content.code("python", "def hello(): pass");
    ```

## See Also

- [Content Types](./content-types.md) - All content variants
- [Metadata](./metadata.md) - Block metadata details
- [ID Generation](./id-generation.md) - How block IDs work
- [Edges](./edges.md) - Block relationships
