# UCM Core

**ucm-core** provides the fundamental building blocks for the Unified Content Model — the core types and traits for representing structured content in a graph-based intermediate representation.

## Overview

UCM Core is the foundation of the UCP ecosystem. It defines:

- **Block** - The fundamental unit of content
- **Content** - Typed content variants (text, code, table, etc.)
- **Document** - A collection of blocks with hierarchical structure
- **Edge** - Explicit relationships between blocks
- **BlockId** - Content-addressed identifiers with 96-bit collision resistance
- **Metadata** - Semantic roles, tags, and token estimates

## Installation

```toml
[dependencies]
ucm-core = "0.1.5"
```

## Module Overview

| Module | Description |
|--------|-------------|
| [`block`](./blocks.md) | Block type and lifecycle |
| [`content`](./content-types.md) | Content type variants |
| [`document`](./documents.md) | Document structure and operations |
| [`edge`](./edges.md) | Relationship types and edge index |
| [`id`](./id-generation.md) | Block ID generation |
| [`metadata`](./metadata.md) | Block metadata and semantic roles |
| `error` | Error types and codes |
| `normalize` | Content normalization |
| `version` | Version tracking |

## Quick Example

```rust
use ucm_core::{Block, Content, Document, DocumentId};

fn main() {
    // Create a document
    let mut doc = Document::new(DocumentId::new("my-doc"));
    let root = doc.root.clone();
    
    // Create and add a block
    let block = Block::new(Content::text("Hello, UCM!"), Some("intro"))
        .with_label("greeting")
        .with_tag("example");
    
    let block_id = doc.add_block(block, &root).unwrap();
    
    // Query the block
    let block = doc.get_block(&block_id).unwrap();
    println!("Block ID: {}", block.id);
    println!("Content type: {}", block.content_type());
}
```

## Public API

### Re-exports

```rust
pub use block::{Block, BlockState};
pub use content::{
    BinaryEncoding, Cell, Code, Column, CompositeLayout, Content, 
    DataType, Dimensions, JsonSchema, LineRange, Math, MathFormat, 
    Media, MediaSource, MediaType, Row, Table, TableSchema, Text, TextFormat,
};
pub use document::{Document, DocumentId, DocumentMetadata};
pub use edge::{Edge, EdgeIndex, EdgeMetadata, EdgeType};
pub use error::{Error, ErrorCode, Result, ValidationIssue, ValidationSeverity};
pub use id::{BlockId, ContentHash, IdGenerator, IdGeneratorConfig};
pub use metadata::{BlockMetadata, RoleCategory, SemanticRole, TokenEstimate, TokenModel};
pub use version::{DocumentVersion, Version};
```

## Design Principles

### Content-Addressed Identity

Block IDs are derived deterministically from:
1. Content type discriminant
2. Semantic role (optional)
3. Normalized content
4. Namespace (optional, for multi-tenant scenarios)

This ensures:
- **Reproducibility**: Same content always produces same ID
- **Deduplication**: Identical blocks naturally share IDs
- **Integrity**: ID changes if content changes

### Immutability by Default

Blocks are conceptually immutable. When content changes:
1. A new ID is generated
2. Version counter increments
3. Timestamp updates

This enables:
- Change detection via ID comparison
- Efficient caching
- Audit trails

### Rich Typing

Content is strongly typed with variants for:
- Text (plain, markdown, rich)
- Code (with language hints)
- Tables (with schema)
- Math (LaTeX, MathML, AsciiMath)
- Media (images, audio, video)
- JSON (with optional schema)
- Binary (with MIME type)
- Composite (block references)

### LLM Optimization

Built-in support for LLM workflows:
- Token estimation per model (GPT-4, Claude, Llama)
- Semantic roles for context management
- Summaries for content folding
- Efficient serialization

## See Also

- [Blocks](./blocks.md) - Detailed block documentation
- [Content Types](./content-types.md) - All content variants
- [Documents](./documents.md) - Document operations
- [Edges](./edges.md) - Relationship types
- [ID Generation](./id-generation.md) - How IDs are generated
- [Metadata](./metadata.md) - Semantic roles and metadata
