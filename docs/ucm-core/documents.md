# Documents

A **Document** is a collection of blocks organized in a hierarchical tree structure. It provides the container for content and manages relationships between blocks.

## Document Structure

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

## Creating Documents

### Basic Creation

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

### With Metadata

```rust
use ucm_core::{Document, DocumentMetadata};

let metadata = DocumentMetadata::new()
    .with_title("My Document");

let doc = Document::create()
    .with_metadata(metadata);
```

## Document Metadata

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

## Adding Blocks

### Basic Addition

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

### Building Hierarchies

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

## Querying Documents

### Get Block by ID

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

### Get Children

```rust
let children: &[BlockId] = doc.children(&parent_id);
for child_id in children {
    let child = doc.get_block(child_id).unwrap();
    println!("Child: {}", child_id);
}
```

### Get Parent

```rust
if let Some(parent_id) = doc.parent(&child_id) {
    println!("Parent: {}", parent_id);
}
```

### Get Descendants

```rust
// Get all descendants (recursive)
let descendants: Vec<BlockId> = doc.descendants(&block_id);
println!("Block has {} descendants", descendants.len());
```

### Check Ancestry

```rust
// Check if block_a is an ancestor of block_b
if doc.is_ancestor(&block_a, &block_b) {
    println!("block_a is an ancestor of block_b");
}
```

## Secondary Indices

Documents maintain indices for fast lookup:

```rust
pub struct DocumentIndices {
    pub by_tag: HashMap<String, HashSet<BlockId>>,
    pub by_role: HashMap<String, HashSet<BlockId>>,
    pub by_content_type: HashMap<String, HashSet<BlockId>>,
    pub by_label: HashMap<String, BlockId>,
}
```

### Find by Tag

```rust
let important_blocks = doc.indices.find_by_tag("important");
for block_id in important_blocks {
    println!("Important block: {}", block_id);
}
```

### Find by Content Type

```rust
let code_blocks = doc.indices.find_by_type("code");
let text_blocks = doc.indices.find_by_type("text");
let table_blocks = doc.indices.find_by_type("table");
```

### Find by Label

```rust
if let Some(block_id) = doc.indices.find_by_label("main-content") {
    let block = doc.get_block(&block_id).unwrap();
    // ...
}
```

### Rebuild Indices

After bulk modifications, rebuild indices:

```rust
doc.rebuild_indices();
```

## Moving Blocks

### Move to New Parent

```rust
// Move block to end of new parent's children
doc.move_block(&block_id, &new_parent_id).unwrap();

// Move to specific position
doc.move_block_at(&block_id, &new_parent_id, 0).unwrap(); // First position
```

### Cycle Detection

Moving a block to one of its descendants is prevented:

```rust
let result = doc.move_block(&parent_id, &child_id);
assert!(result.is_err()); // CycleDetected error
```

## Deleting Blocks

### Delete Single Block

```rust
// Delete block (children become orphaned)
let deleted_block = doc.delete_block(&block_id).unwrap();
```

### Delete with Cascade

```rust
// Delete block and all descendants
let deleted_blocks = doc.delete_cascade(&block_id).unwrap();
println!("Deleted {} blocks", deleted_blocks.len());
```

### Remove from Structure Only

```rust
// Remove from tree but keep in document (orphan)
doc.remove_from_structure(&block_id);

// Block is now orphaned but still exists
assert!(doc.get_block(&block_id).is_some());
assert!(!doc.is_reachable(&block_id));
```

## Orphan Management

### Find Orphans

```rust
let orphans = doc.find_orphans();
for orphan_id in &orphans {
    println!("Orphaned block: {}", orphan_id);
}
```

### Check Reachability

```rust
if doc.is_reachable(&block_id) {
    println!("Block is reachable from root");
} else {
    println!("Block is orphaned");
}
```

### Prune Orphans

```rust
// Remove all unreachable blocks
let pruned = doc.prune_unreachable();
println!("Pruned {} orphaned blocks", pruned.len());
```

### Block State

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

### Validation Checks

The validator checks for:
- **Cycles** in document structure
- **Orphaned blocks** (warning)
- **Dangling references** (edges to non-existent blocks)
- **Invalid structure** (references to missing blocks)

## Token Estimation

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

## Complete Example

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

## Best Practices

### 1. Use Meaningful Document IDs

```rust
// Good - descriptive
Document::new(DocumentId::new("user-guide-v2"))
Document::new(DocumentId::new("api-spec-2024-01"))

// Less ideal - generic
Document::create() // Auto-generated ID
```

### 2. Maintain Document Metadata

```rust
let mut doc = Document::create();
doc.metadata.title = Some("Important Document".to_string());
doc.metadata.authors.push("Author Name".to_string());
doc.metadata.language = Some("en".to_string());
```

### 3. Use Labels for Key Blocks

```rust
let toc = Block::new(Content::text("Table of Contents"), Some("toc"))
    .with_label("table-of-contents");

// Later, find easily
let toc_id = doc.indices.find_by_label("table-of-contents");
```

### 4. Validate After Bulk Operations

```rust
// After many modifications
doc.rebuild_indices();
let issues = doc.validate();
```

### 5. Clean Up Orphans

```rust
// Periodically or before serialization
doc.prune_unreachable();
```

## See Also

- [Blocks](./blocks.md) - Block structure
- [Edges](./edges.md) - Relationship management
- [Metadata](./metadata.md) - Document and block metadata
