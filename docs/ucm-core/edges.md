# Edges

**Edges** represent explicit relationships between blocks. They enable rich semantic connections beyond the basic parent-child hierarchy.

## Edge Structure

```rust
pub struct Edge {
    /// Type of relationship
    pub edge_type: EdgeType,
    
    /// Target block
    pub target: BlockId,
    
    /// Edge-specific metadata
    pub metadata: EdgeMetadata,
    
    /// When the edge was created
    pub created_at: DateTime<Utc>,
}
```

## Creating Edges

### Basic Creation

```rust
use ucm_core::{Edge, EdgeType, BlockId};

let target_id = BlockId::from_bytes([1u8; 12]);

// Simple edge
let edge = Edge::new(EdgeType::References, target_id);
```

### With Metadata

```rust
let edge = Edge::new(EdgeType::References, target_id)
    .with_confidence(0.95)
    .with_description("Important reference to supporting evidence");
```

### With Custom Metadata

```rust
use ucm_core::edge::EdgeMetadata;

let metadata = EdgeMetadata::new()
    .with_confidence(0.8)
    .with_description("Derived from original source")
    .with_custom("source_version", serde_json::json!("1.0"));

let edge = Edge::new(EdgeType::DerivedFrom, target_id)
    .with_metadata(metadata);
```

## Edge Types

### Derivation Relationships

Track content provenance and transformations:

| Type | Description | Inverse |
|------|-------------|---------|
| `DerivedFrom` | Block was created from another | - |
| `Supersedes` | Block replaces another | - |
| `TransformedFrom` | Block is a transformation of another | - |

```rust
// Mark block as derived from another
let edge = Edge::new(EdgeType::DerivedFrom, original_id);

// Mark block as superseding (replacing) another
let edge = Edge::new(EdgeType::Supersedes, old_version_id);

// Mark as transformation (e.g., summary, translation)
let edge = Edge::new(EdgeType::TransformedFrom, source_id);
```

### Reference Relationships

Track citations and links:

| Type | Description | Inverse |
|------|-------------|---------|
| `References` | Block references another | `CitedBy` |
| `CitedBy` | Inverse of References (auto-maintained) | `References` |
| `LinksTo` | Hyperlink relationship | - |

```rust
// Add a reference
let edge = Edge::new(EdgeType::References, cited_block_id);

// The edge index automatically maintains the inverse CitedBy relationship
```

### Semantic Relationships

Express meaning and argumentation:

| Type | Description | Inverse |
|------|-------------|---------|
| `Supports` | Provides evidence for | - |
| `Contradicts` | Contradicts (symmetric) | `Contradicts` |
| `Elaborates` | Expands on | - |
| `Summarizes` | Summarizes | - |

```rust
// Evidence supports a claim
let edge = Edge::new(EdgeType::Supports, claim_id)
    .with_confidence(0.9);

// Two blocks contradict each other
let edge = Edge::new(EdgeType::Contradicts, other_id);

// Block elaborates on another
let edge = Edge::new(EdgeType::Elaborates, topic_id);

// Block summarizes another
let edge = Edge::new(EdgeType::Summarizes, detailed_id);
```

### Structural Relationships

Auto-maintained from document structure:

| Type | Description | Inverse |
|------|-------------|---------|
| `ParentOf` | Structural parent | `ChildOf` |
| `ChildOf` | Structural child | `ParentOf` |
| `SiblingOf` | Same parent (symmetric) | `SiblingOf` |
| `PreviousSibling` | Immediate previous sibling | `NextSibling` |
| `NextSibling` | Immediate next sibling | `PreviousSibling` |

```rust
// These are typically auto-maintained by the document structure
// but can be queried via the edge index
```

### Version Relationships

Track versions and alternatives:

| Type | Description | Inverse |
|------|-------------|---------|
| `VersionOf` | Different version of same logical content | - |
| `AlternativeOf` | Alternative representation | - |
| `TranslationOf` | Translation to different language | - |

```rust
// Mark as version of another
let edge = Edge::new(EdgeType::VersionOf, original_id);

// Mark as alternative (e.g., different format)
let edge = Edge::new(EdgeType::AlternativeOf, primary_id);

// Mark as translation
let edge = Edge::new(EdgeType::TranslationOf, source_id)
    .with_metadata(EdgeMetadata::new()
        .with_custom("source_language", serde_json::json!("en"))
        .with_custom("target_language", serde_json::json!("es")));
```

### Custom Relationships

Define your own relationship types:

```rust
let edge = Edge::new(
    EdgeType::Custom("implements".to_string()),
    interface_id
);
```

## Edge Type Properties

### Inverse Relationships

```rust
let edge_type = EdgeType::References;
let inverse = edge_type.inverse();
assert_eq!(inverse, Some(EdgeType::CitedBy));

let edge_type = EdgeType::DerivedFrom;
let inverse = edge_type.inverse();
assert_eq!(inverse, None); // No automatic inverse
```

### Symmetric Relationships

```rust
assert!(EdgeType::Contradicts.is_symmetric());
assert!(EdgeType::SiblingOf.is_symmetric());
assert!(!EdgeType::References.is_symmetric());
```

### Structural Relationships

```rust
assert!(EdgeType::ParentOf.is_structural());
assert!(EdgeType::ChildOf.is_structural());
assert!(!EdgeType::References.is_structural());
```

### Parsing from String

```rust
let edge_type = EdgeType::from_str("references").unwrap();
assert_eq!(edge_type, EdgeType::References);

let custom = EdgeType::from_str("custom:my_type").unwrap();
assert_eq!(custom, EdgeType::Custom("my_type".to_string()));
```

### Converting to String

```rust
let s = EdgeType::References.as_str();
assert_eq!(s, "references");

let s = EdgeType::Custom("my_type".to_string()).as_str();
assert_eq!(s, "custom:my_type");
```

## Edge Metadata

```rust
pub struct EdgeMetadata {
    /// Confidence score (0.0 - 1.0) for inferred relationships
    pub confidence: Option<f32>,
    
    /// Human-readable description
    pub description: Option<String>,
    
    /// Custom key-value pairs
    pub custom: HashMap<String, serde_json::Value>,
}
```

### Working with Metadata

```rust
use ucm_core::edge::EdgeMetadata;

let metadata = EdgeMetadata::new()
    .with_confidence(0.85)
    .with_description("Inferred from content similarity")
    .with_custom("similarity_score", serde_json::json!(0.92))
    .with_custom("method", serde_json::json!("cosine"));

// Check if empty
assert!(!metadata.is_empty());
```

## Edge Index

The `EdgeIndex` provides efficient bidirectional traversal of relationships.

### Structure

```rust
pub struct EdgeIndex {
    /// Outgoing edges: source -> [(type, target)]
    outgoing: HashMap<BlockId, Vec<(EdgeType, BlockId)>>,
    
    /// Incoming edges: target -> [(type, source)]
    incoming: HashMap<BlockId, Vec<(EdgeType, BlockId)>>,
}
```

### Adding Edges

```rust
use ucm_core::edge::EdgeIndex;

let mut index = EdgeIndex::new();
let source = BlockId::from_bytes([1u8; 12]);
let target = BlockId::from_bytes([2u8; 12]);

let edge = Edge::new(EdgeType::References, target.clone());
index.add_edge(&source, &edge);

// Inverse relationship is automatically maintained
```

### Querying Edges

```rust
// Get all outgoing edges from a block
let outgoing = index.outgoing_from(&source);
for (edge_type, target) in outgoing {
    println!("{:?} -> {}", edge_type, target);
}

// Get all incoming edges to a block
let incoming = index.incoming_to(&target);
for (edge_type, source) in incoming {
    println!("{} <- {:?}", source, edge_type);
}

// Get edges of specific type
let refs = index.outgoing_of_type(&source, &EdgeType::References);
let cited_by = index.incoming_of_type(&target, &EdgeType::CitedBy);

// Check if edge exists
let exists = index.has_edge(&source, &target, &EdgeType::References);
```

### Removing Edges

```rust
// Remove specific edge
index.remove_edge(&source, &target, &EdgeType::References);

// Remove all edges involving a block
index.remove_block(&block_id);

// Clear all edges
index.clear();
```

### Statistics

```rust
let count = index.edge_count();
println!("Total edges: {}", count);
```

## Working with Edges in Documents

### Adding Edges to Blocks

```rust
use ucm_core::{Document, Block, Content, Edge, EdgeType};

let mut doc = Document::create();
let root = doc.root.clone();

// Create blocks
let source = Block::new(Content::text("Source"), None);
let source_id = doc.add_block(source, &root).unwrap();

let target = Block::new(Content::text("Target"), None);
let target_id = doc.add_block(target, &root).unwrap();

// Add edge to source block
let edge = Edge::new(EdgeType::References, target_id.clone());
doc.get_block_mut(&source_id).unwrap().add_edge(edge.clone());

// Update document's edge index
doc.edge_index.add_edge(&source_id, &edge);
```

### Querying Edges via Document

```rust
// Find all blocks that reference a target
let referencing = doc.edge_index.incoming_of_type(&target_id, &EdgeType::CitedBy);

// Find all blocks referenced by a source
let referenced = doc.edge_index.outgoing_of_type(&source_id, &EdgeType::References);

// Check relationship
if doc.edge_index.has_edge(&source_id, &target_id, &EdgeType::References) {
    println!("Source references Target");
}
```

### Removing Edges from Blocks

```rust
// Remove from block
let removed = doc.get_block_mut(&source_id)
    .unwrap()
    .remove_edge(&target_id, &EdgeType::References);

// Update index
if removed {
    doc.edge_index.remove_edge(&source_id, &target_id, &EdgeType::References);
}
```

## Complete Example

```rust
use ucm_core::{Document, Block, Content, Edge, EdgeType};
use ucm_core::edge::EdgeMetadata;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Document::create();
    let root = doc.root.clone();
    
    // Create a claim
    let claim = Block::new(
        Content::text("Rust is memory safe"),
        Some("body.argument")
    ).with_label("main-claim");
    let claim_id = doc.add_block(claim, &root)?;
    
    // Create evidence
    let evidence1 = Block::new(
        Content::text("The borrow checker prevents data races"),
        Some("body.evidence")
    );
    let evidence1_id = doc.add_block(evidence1, &root)?;
    
    let evidence2 = Block::new(
        Content::text("No null pointer dereferences"),
        Some("body.evidence")
    );
    let evidence2_id = doc.add_block(evidence2, &root)?;
    
    // Create counterargument
    let counter = Block::new(
        Content::text("Unsafe blocks can bypass safety"),
        Some("body.counterargument")
    );
    let counter_id = doc.add_block(counter, &root)?;
    
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
    
    // Counterargument contradicts claim
    let edge3 = Edge::new(EdgeType::Contradicts, claim_id.clone())
        .with_confidence(0.7)
        .with_description("Partial contradiction - unsafe exists but is explicit");
    doc.get_block_mut(&counter_id).unwrap().add_edge(edge3.clone());
    doc.edge_index.add_edge(&counter_id, &edge3);
    
    // Query relationships
    println!("Blocks supporting the claim:");
    let supporters = doc.edge_index.incoming_of_type(&claim_id, &EdgeType::Supports);
    for supporter_id in supporters {
        let block = doc.get_block(&supporter_id).unwrap();
        if let Content::Text(text) = &block.content {
            println!("  - {}", text.text);
        }
    }
    
    println!("\nBlocks contradicting the claim:");
    let contradictors = doc.edge_index.incoming_of_type(&claim_id, &EdgeType::Contradicts);
    for contradictor_id in contradictors {
        let block = doc.get_block(&contradictor_id).unwrap();
        if let Content::Text(text) = &block.content {
            println!("  - {}", text.text);
        }
    }
    
    println!("\nTotal edges: {}", doc.edge_index.edge_count());
    
    Ok(())
}
```

## Best Practices

### 1. Use Appropriate Edge Types

```rust
// Good - semantic meaning is clear
Edge::new(EdgeType::Supports, claim_id)
Edge::new(EdgeType::DerivedFrom, original_id)

// Less ideal - generic reference
Edge::new(EdgeType::References, some_id)
```

### 2. Add Confidence for Inferred Relationships

```rust
// For relationships inferred by analysis
Edge::new(EdgeType::Supports, claim_id)
    .with_confidence(0.85)

// For explicit relationships, confidence can be omitted or set to 1.0
Edge::new(EdgeType::References, cited_id)
```

### 3. Document Relationships

```rust
Edge::new(EdgeType::Contradicts, other_id)
    .with_description("Contradicts on the point of performance")
```

### 4. Keep Edge Index in Sync

```rust
// Always update both block and index
let edge = Edge::new(EdgeType::References, target_id);
block.add_edge(edge.clone());
doc.edge_index.add_edge(&source_id, &edge);

// Same for removal
block.remove_edge(&target_id, &EdgeType::References);
doc.edge_index.remove_edge(&source_id, &target_id, &EdgeType::References);
```

### 5. Use Custom Types Sparingly

```rust
// Prefer built-in types when they fit
EdgeType::References  // Not Custom("references")

// Use custom only for domain-specific relationships
EdgeType::Custom("implements_interface".to_string())
```

## See Also

- [Blocks](./blocks.md) - Block structure
- [Documents](./documents.md) - Document operations
- [UCL Commands](../ucl-parser/commands.md) - LINK/UNLINK commands
