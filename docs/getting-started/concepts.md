# Core Concepts

This document explains the fundamental concepts and architecture of the Unified Content Protocol.

## The Content Graph Model

UCP represents documents as **directed acyclic graphs (DAGs)** where:

- **Nodes** are content blocks
- **Structural edges** define parent-child relationships (the document tree)
- **Semantic edges** define relationships like references, derivations, and contradictions

```
                    ┌─────────────┐
                    │    Root     │
                    └──────┬──────┘
                           │
            ┌──────────────┼──────────────┐
            │              │              │
      ┌─────▼─────┐  ┌─────▼─────┐  ┌─────▼─────┐
      │  Title    │  │  Section  │  │  Section  │
      └───────────┘  └─────┬─────┘  └─────┬─────┘
                           │              │
                     ┌─────┴─────┐        │
                     │           │        │
               ┌─────▼───┐ ┌─────▼───┐ ┌──▼──────┐
               │  Para   │ │  Code   │ │  Para   │
               └─────────┘ └─────────┘ └─────────┘
```

## Blocks

### What is a Block?

A **Block** is the fundamental unit of content in UCP. Every piece of content—whether a paragraph, code snippet, table, or image—is represented as a block.

```rust
pub struct Block {
    pub id: BlockId,           // Content-addressed identifier
    pub content: Content,      // The actual content
    pub metadata: BlockMetadata, // Searchable metadata
    pub edges: Vec<Edge>,      // Relationships to other blocks
    pub version: Version,      // For optimistic concurrency
}
```

### Block Identity

Block IDs are **deterministic** and **content-addressed**:

- Generated from content hash + semantic role + optional namespace
- 96-bit entropy (24 hex characters) ensures collision resistance < 10⁻¹⁵ at 10M blocks
- Format: `blk_<24 hex chars>` (e.g., `blk_a1b2c3d4e5f6a1b2c3d4e5f6`)

```rust
use ucm_core::{Content, id::generate_block_id};

let content = Content::text("Hello, world!");
let id1 = generate_block_id(&content, Some("intro"), None);
let id2 = generate_block_id(&content, Some("intro"), None);

assert_eq!(id1, id2); // Same content + role = same ID
```

### Block Lifecycle States

```
┌─────────┐     delete      ┌───────────┐
│  Live   │ ───────────────►│  Deleted  │
└────┬────┘                 └───────────┘
     │
     │ remove from structure
     ▼
┌──────────┐    prune       ┌───────────┐
│ Orphaned │ ──────────────►│  Deleted  │
└──────────┘                └───────────┘
```

- **Live**: Reachable from document root
- **Orphaned**: Exists but not reachable (can be restored or pruned)
- **Deleted**: Removed from document

## Content Types

UCP supports rich, typed content:

| Type | Description | Example |
|------|-------------|---------|
| `Text` | Plain, Markdown, or rich text | Paragraphs, headings |
| `Code` | Source code with language hint | Code snippets |
| `Table` | Tabular data with schema | Data tables |
| `Math` | LaTeX, MathML, AsciiMath | Equations |
| `Media` | Images, audio, video | Embedded media |
| `Json` | Structured JSON data | Configuration |
| `Binary` | Raw binary with MIME type | Files |
| `Composite` | Container referencing other blocks | Layouts |

```rust
use ucm_core::Content;

// Text content
let text = Content::text("Hello, world!");
let markdown = Content::markdown("**Bold** and *italic*");

// Code content
let code = Content::code("rust", "fn main() {}");

// Table content
let table = Content::table(vec![
    vec!["Name".into(), "Age".into()],
    vec!["Alice".into(), "30".into()],
]);

// JSON content
let json = Content::json(serde_json::json!({
    "key": "value"
}));
```

## Documents

A **Document** is a collection of blocks with hierarchical structure:

```rust
pub struct Document {
    pub id: DocumentId,
    pub root: BlockId,
    pub structure: HashMap<BlockId, Vec<BlockId>>,  // Adjacency map
    pub blocks: HashMap<BlockId, Block>,
    pub metadata: DocumentMetadata,
    pub indices: DocumentIndices,      // Secondary indices
    pub edge_index: EdgeIndex,         // Relationship index
    pub version: DocumentVersion,
}
```

### Document Operations

| Operation | Description |
|-----------|-------------|
| `add_block` | Add a block as child of a parent |
| `add_block_at` | Add at specific position |
| `move_block` | Move to new parent |
| `delete_block` | Remove single block |
| `delete_cascade` | Remove block and descendants |
| `prune_unreachable` | Remove orphaned blocks |

### Secondary Indices

Documents maintain indices for fast lookup:

- **by_tag**: Find blocks with specific tags
- **by_role**: Find blocks by semantic role category
- **by_content_type**: Find blocks by content type
- **by_label**: Find block by unique label

## Edges (Relationships)

Edges represent explicit relationships between blocks:

### Edge Types

**Derivation Relationships:**
- `DerivedFrom` - Block created from another
- `Supersedes` - Block replaces another
- `TransformedFrom` - Block is transformation of another

**Reference Relationships:**
- `References` - Block references another
- `CitedBy` - Inverse of References (auto-maintained)
- `LinksTo` - Hyperlink relationship

**Semantic Relationships:**
- `Supports` - Provides evidence for
- `Contradicts` - Contradicts (symmetric)
- `Elaborates` - Expands on
- `Summarizes` - Summarizes

**Structural Relationships (auto-maintained):**
- `ParentOf` / `ChildOf`
- `SiblingOf`
- `PreviousSibling` / `NextSibling`

**Version Relationships:**
- `VersionOf` - Different version
- `AlternativeOf` - Alternative representation
- `TranslationOf` - Translation

```rust
use ucm_core::{Edge, EdgeType};

let edge = Edge::new(EdgeType::References, target_id)
    .with_confidence(0.95)
    .with_description("Important reference");
```

## Metadata

### Block Metadata

```rust
pub struct BlockMetadata {
    pub semantic_role: Option<SemanticRole>,  // Document role
    pub label: Option<String>,                 // Unique identifier
    pub tags: Vec<String>,                     // Searchable tags
    pub summary: Option<String>,               // For folding
    pub token_estimate: Option<TokenEstimate>, // LLM optimization
    pub content_hash: ContentHash,             // Change detection
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub custom: HashMap<String, Value>,        // Extension point
}
```

### Semantic Roles

Semantic roles describe a block's function in the document:

```rust
use ucm_core::metadata::{SemanticRole, RoleCategory};

// Parse from string
let role = SemanticRole::parse("intro.hook").unwrap();

// Build programmatically
let role = SemanticRole::new(RoleCategory::Intro)
    .with_subcategory("hook")
    .with_qualifier("v2");
```

**Role Categories:**

| Category | Description |
|----------|-------------|
| `Title`, `Subtitle`, `Abstract` | Document structure |
| `Heading1` - `Heading6` | Section headings |
| `Intro`, `IntroHook`, `IntroThesis` | Introduction elements |
| `Body`, `BodyArgument`, `BodyEvidence` | Body elements |
| `Conclusion`, `ConclusionSummary` | Conclusion elements |
| `Code`, `Definition`, `Theorem` | Technical elements |
| `Quote`, `Note`, `Warning`, `Callout` | Special sections |

### Token Estimation

UCP provides token estimates for LLM context management:

```rust
use ucm_core::metadata::{TokenEstimate, TokenModel};

let estimate = TokenEstimate::compute(&content);

println!("GPT-4 tokens: {}", estimate.for_model(TokenModel::GPT4));
println!("Claude tokens: {}", estimate.for_model(TokenModel::Claude));
println!("Llama tokens: {}", estimate.for_model(TokenModel::Llama));
```

## Normalization

Content is normalized before hashing to ensure deterministic IDs:

- **Unicode normalization** (NFC by default)
- **Whitespace normalization** (collapse for text, preserve for code)
- **Line ending normalization** (LF)
- **Canonical JSON** (sorted keys, no whitespace)

```rust
use ucm_core::normalize::{normalize_content, normalize_text, NormalizationConfig};

// Normalize content for hashing
let normalized = normalize_content(&content);

// Custom normalization
let config = NormalizationConfig {
    whitespace: WhitespaceNorm::Preserve,
    ..Default::default()
};
let normalized = normalize_text("  hello  world  ", config);
```

## Versioning

### Block Versioning

Each block has a version for optimistic concurrency:

```rust
pub struct Version {
    pub counter: u64,
    pub timestamp: DateTime<Utc>,
}
```

### Document Versioning

Documents track version with state hash:

```rust
pub struct DocumentVersion {
    pub counter: u64,
    pub timestamp: DateTime<Utc>,
    pub state_hash: [u8; 8],
}
```

## Error Handling

UCP uses structured error codes for categorization:

| Range | Category |
|-------|----------|
| E001-E099 | Reference errors (block not found, invalid ID) |
| E100-E199 | Syntax errors (malformed commands) |
| E200-E299 | Validation errors (cycles, orphans) |
| E300-E399 | Concurrency errors (version conflicts) |
| E400-E499 | Resource errors (size limits) |
| E500-E599 | Security errors (path traversal) |
| E900-E999 | Internal errors |

```rust
use ucm_core::{Error, ErrorCode};

let err = Error::new(ErrorCode::E001BlockNotFound, "Block not found")
    .with_location(Location::new(10, 5))
    .with_suggestion("Did you mean 'blk_abc'?");
```

## Next Steps

- [UCM Core Reference](../ucm-core/README.md) - Detailed API documentation
- [UCL Syntax](../ucl-parser/syntax.md) - Command language reference
- [Examples](../examples/basic.md) - Practical examples
