# Blocks

The **Block** is the fundamental unit of content in UCM. Every piece of content in a document is represented as a block.

## Block Structure

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

## Creating Blocks

### Basic Creation

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

### Builder Pattern

```rust
use ucm_core::{Block, Content, Edge, EdgeType};

let block = Block::new(Content::text("Important note"), Some("note"))
    .with_label("warning-001")
    .with_tag("important")
    .with_tag("review-needed")
    .with_edge(Edge::new(EdgeType::References, other_block_id));
```

### With Metadata

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

## Block Properties

### ID

The block ID is a content-addressed identifier:

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

### Content Type

```rust
let text_block = Block::new(Content::text("Hello"), None);
assert_eq!(text_block.content_type(), "text");

let code_block = Block::new(Content::code("rust", "fn main() {}"), None);
assert_eq!(code_block.content_type(), "code");
```

### Root Detection

```rust
let root = Block::root();
assert!(root.is_root());

let regular = Block::new(Content::text("Not root"), None);
assert!(!regular.is_root());
```

### Size and Tokens

```rust
let block = Block::new(Content::text("Hello, world!"), None);

// Content size in bytes
let size = block.size_bytes();

// Token estimate
let tokens = block.token_estimate();
println!("GPT-4 tokens: {}", tokens.gpt4);
println!("Claude tokens: {}", tokens.claude);
```

## Modifying Blocks

### Update Content

```rust
let mut block = Block::new(Content::text("Original"), Some("intro"));
let original_id = block.id.clone();

// Update content (regenerates ID)
block.update_content(Content::text("Updated"), Some("intro"));

assert_ne!(block.id, original_id);  // ID changed
assert!(block.version.counter > 1); // Version incremented
```

### Managing Edges

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

### Tags

```rust
let block = Block::new(Content::text("Test"), None)
    .with_tag("important")
    .with_tag("draft");

assert!(block.has_tag("important"));
assert!(block.has_tag("draft"));
assert!(!block.has_tag("final"));
```

## Block Lifecycle States

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

### Checking State

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

## Serialization

Blocks implement `Serialize` and `Deserialize`:

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

```rust
// Good - clear purpose
let intro = Block::new(Content::text("..."), Some("intro.hook"));
let code = Block::new(Content::code("rust", "..."), Some("code"));

// Less ideal - no semantic information
let block = Block::new(Content::text("..."), None);
```

### 2. Use Labels for Unique References

Labels provide human-readable identifiers:

```rust
let block = Block::new(Content::text("Important"), None)
    .with_label("main-warning");

// Later, find by label
let id = doc.indices.find_by_label("main-warning");
```

### 3. Use Tags for Categories

Tags enable filtering and grouping:

```rust
let block = Block::new(Content::text("..."), None)
    .with_tag("draft")
    .with_tag("needs-review")
    .with_tag("chapter-1");

// Find all drafts
let drafts = doc.indices.find_by_tag("draft");
```

### 4. Leverage Content Types

Use the appropriate content type for your data:

```rust
// Text for prose
Content::text("A paragraph of text...")

// Code for source code
Content::code("python", "def hello(): pass")

// Table for structured data
Content::table(vec![...])

// JSON for structured configuration
Content::json(serde_json::json!({...}))
```

## See Also

- [Content Types](./content-types.md) - All content variants
- [Metadata](./metadata.md) - Block metadata details
- [ID Generation](./id-generation.md) - How block IDs work
- [Edges](./edges.md) - Block relationships
