# Unified Content Protocol (UCP) — Canonical Specification

**Version:** 2.0.0  
**Status:** Authoritative Specification  
**Last Updated:** 2026-01-07  
**Implementation Language:** Rust (core), with bindings for Python, JavaScript, Go  

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Design Principles](#2-design-principles)
3. [Architecture Overview](#3-architecture-overview)
4. [Unified Content Model (UCM)](#4-unified-content-model-ucm)
5. [Unified Content Language (UCL)](#5-unified-content-language-ucl)
6. [Query Language (UCQ)](#6-query-language-ucq)
7. [Transformation Engine](#7-transformation-engine)
8. [Concurrency and Transactions](#8-concurrency-and-transactions)
9. [Indexing and Performance](#9-indexing-and-performance)
10. [Observability](#10-observability)
11. [Security Model](#11-security-model)
12. [Error Handling](#12-error-handling)
13. [Versioning and Migration](#13-versioning-and-migration)
14. [Implementation Roadmap](#14-implementation-roadmap)
15. [Formal Grammar](#15-formal-grammar)
16. [Extended Examples](#16-extended-examples)

---

## 1. Executive Summary

The Unified Content Protocol (UCP) is a **deterministic, high-performance substrate** for representing and manipulating structured content in AI-powered systems. It serves as an intermediate representation (IR) that bridges the gap between diverse content formats and LLM-based agents.

### 1.1 Core Components

| Component | Purpose |
|-----------|---------|
| **Unified Content Model (UCM)** | Graph-based IR supporting documents, tables, code, multimedia |
| **Unified Content Language (UCL)** | Token-efficient command language for content mutation |
| **Unified Content Query (UCQ)** | Declarative query language for content retrieval |
| **Transformation Engine** | Stateless, transactional execution of UCL/UCQ operations |
| **Translator System** | Bidirectional conversion between external formats and UCM |

### 1.2 Key Guarantees

1. **Determinism** — Identical inputs always produce identical outputs
2. **Collision Resistance** — Block IDs have < 10⁻¹⁸ collision probability at 10M blocks
3. **Transactional Integrity** — All operations are atomic, consistent, isolated
4. **Observable** — Full tracing, metrics, and audit logging
5. **Performant** — Sub-millisecond operations, linear scaling

### 1.3 Scope Boundaries

**UCP IS:**
- A content transformation engine
- A structured intermediate representation
- A command/query language for content manipulation
- A deterministic, versionable format

**UCP IS NOT:**
- A database or persistence layer (but can integrate with one)
- An orchestration framework
- A multi-tenant service
- A rendering engine
- A version control system (but supports snapshots)

---

## 2. Design Principles

### 2.1 SOLID Principles Application

| Principle | Application in UCP |
|-----------|-------------------|
| **Single Responsibility** | Each module has one reason to change: Parser parses, Validator validates, Engine executes |
| **Open/Closed** | Content types and translators are extensible without modifying core |
| **Liskov Substitution** | All `Content` variants implement common trait behaviors |
| **Interface Segregation** | Separate traits for `Parse`, `Emit`, `Validate`, `Transform` |
| **Dependency Inversion** | Core depends on abstractions (traits), not concrete implementations |

### 2.2 Performance Mandates

1. **Zero-Copy Where Possible** — Use `&str` and `Cow<str>` to avoid allocations
2. **Lazy Evaluation** — Compute hashes, summaries, and indices on demand
3. **Cache-Friendly** — Contiguous memory layouts, arena allocation for blocks
4. **Parallel by Default** — All read operations are lock-free
5. **Bounded Memory** — Streaming APIs for large documents

### 2.3 Observability Mandates

1. **Structured Logging** — All operations emit structured log events
2. **Metrics** — Counters, histograms, and gauges for all operations
3. **Tracing** — Distributed tracing with span propagation
4. **Audit Trail** — Immutable log of all mutations

### 2.4 Maintainability Mandates

1. **Explicit Over Implicit** — No magic, all behavior documented
2. **Fail Fast** — Validate early, provide actionable errors
3. **Test Coverage** — Minimum 90% line coverage, 100% for core paths
4. **Documentation** — All public APIs have doc comments with examples

---

## 3. Architecture Overview

### 3.1 Layered Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        External Formats                              │
│            (Markdown, HTML, JSON, SQL, Code, Media, PDF)            │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      Translator Layer                                │
│              (Stateless, bidirectional, pluggable)                  │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                  Unified Content Model (UCM)                         │
│         (Graph-based IR with blocks, edges, indices)                │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Command/Query Layer                               │
│         UCL Parser (Mutations)    │    UCQ Parser (Queries)         │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                   Transformation Engine                              │
│      Transaction Manager │ Validation Engine │ Snapshot Manager     │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Storage Abstraction                               │
│       Memory (Arena) │ Persistent (RocksDB) │ CRDT (Yjs)           │
└─────────────────────────────────────────────────────────────────────┘
```

### 3.2 Crate Structure

```
ucp/
├── Cargo.toml                    # Workspace manifest
├── crates/
│   ├── ucm-core/                 # Core types and traits
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── block.rs          # Block, BlockId, Content
│   │       ├── document.rs       # Document structure
│   │       ├── edge.rs           # Edge types and relationships
│   │       ├── content.rs        # Content type implementations
│   │       ├── id.rs             # ID generation
│   │       ├── normalize.rs      # Content normalization
│   │       └── error.rs          # Error types
│   │
│   ├── ucm-engine/               # Transformation engine
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── engine.rs         # Main engine implementation
│   │       ├── transaction.rs    # Transaction management
│   │       ├── snapshot.rs       # Snapshot/restore
│   │       ├── index.rs          # Index structures
│   │       └── validate.rs       # Validation pipeline
│   │
│   ├── ucl-parser/               # UCL command parser
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── lexer.rs
│   │       ├── parser.rs
│   │       └── ast.rs
│   │
│   ├── ucq-parser/               # UCQ query parser
│   │
│   ├── translators/              # Format translators
│   │   ├── markdown/
│   │   ├── html/
│   │   └── json/
│   │
│   └── ucp-observe/              # Observability
│
├── tests/                        # Integration tests
├── benches/                      # Benchmarks
└── examples/                     # Usage examples
```

---

## 4. Unified Content Model (UCM)

### 4.1 Block Structure

```rust
/// A block is the fundamental unit of content in UCM.
pub struct Block {
    pub id: BlockId,
    pub content: Content,
    pub metadata: BlockMetadata,
    pub edges: Vec<Edge>,
    pub version: Version,
}

/// Block identifier - 96 bits of entropy from content hash
pub struct BlockId(pub [u8; 12]);

impl BlockId {
    pub fn to_string(&self) -> String {
        format!("blk_{}", hex::encode(&self.0))
    }
}
```

### 4.2 Block ID Generation (Collision-Resistant)

Using **96 bits of entropy** ensures < 10⁻¹⁵ collision probability at 10M blocks:

```rust
pub fn generate_block_id(
    content: &Content,
    semantic_role: Option<&str>,
    namespace: Option<&str>,
) -> BlockId {
    let mut hasher = Sha256::new();
    
    if let Some(ns) = namespace {
        hasher.update(ns.as_bytes());
        hasher.update(b":");
    }
    
    hasher.update(content.type_tag().as_bytes());
    hasher.update(b":");
    
    if let Some(role) = semantic_role {
        hasher.update(role.as_bytes());
    }
    hasher.update(b":");
    
    hasher.update(normalize_content(content).as_bytes());
    
    let hash = hasher.finalize();
    let mut id = [0u8; 12];
    id.copy_from_slice(&hash[..12]);
    BlockId(id)
}
```

**Collision Analysis:**

| Blocks | Collision Probability |
|--------|----------------------|
| 1M | 2.8 × 10⁻¹⁷ |
| 10M | 2.8 × 10⁻¹⁵ |

### 4.3 Content Normalization (Critical for Determinism)

```rust
pub fn normalize_content(content: &Content) -> String {
    match content {
        Content::Text { text, .. } => normalize_text(text),
        Content::Code { source, .. } => normalize_code(source),
        Content::Table { columns, rows, .. } => normalize_table(columns, rows),
        Content::Json { value, .. } => canonical_json(value),
        _ => content.to_canonical_string(),
    }
}

fn normalize_text(text: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    text.nfc()                           // Unicode NFC normalization
        .collect::<String>()
        .replace("\r\n", "\n")           // Normalize line endings
        .replace('\r', "\n")
        .split_whitespace()              // Collapse whitespace
        .collect::<Vec<_>>()
        .join(" ")
}

fn canonical_json(value: &serde_json::Value) -> String {
    // RFC 8785: sorted keys, no whitespace
    match value {
        serde_json::Value::Object(map) => {
            let mut pairs: Vec<_> = map.iter().collect();
            pairs.sort_by(|a, b| a.0.cmp(b.0));
            let inner: Vec<String> = pairs
                .iter()
                .map(|(k, v)| format!("\"{}\":{}", k, canonical_json(v)))
                .collect();
            format!("{{{}}}", inner.join(","))
        }
        serde_json::Value::Array(arr) => {
            format!("[{}]", arr.iter().map(canonical_json).collect::<Vec<_>>().join(","))
        }
        _ => value.to_string(),
    }
}
```

### 4.4 Content Types

```rust
pub enum Content {
    Text { text: String, format: TextFormat },
    Table { columns: Vec<Column>, rows: Vec<Row>, schema: Option<TableSchema> },
    Code { language: String, source: String, highlights: Vec<LineRange> },
    Math { format: MathFormat, expression: String, display_mode: bool },
    Media { media_type: MediaType, source: MediaSource, alt_text: Option<String>, content_hash: Option<[u8; 32]> },
    Json { value: serde_json::Value, schema: Option<JsonSchema> },
    Binary { mime_type: String, data: Vec<u8> },
    Composite { layout: CompositeLayout, children: Vec<BlockId> },
}

pub enum TextFormat { Plain, Markdown, Rich }
pub enum MathFormat { LaTeX, MathML, AsciiMath }
pub enum MediaType { Image, Audio, Video, Document }
pub enum MediaSource { Url(String), Base64(String), Reference(BlockId) }
pub enum CompositeLayout { Vertical, Horizontal, Grid(usize), Tabs }
```

### 4.5 Token Estimation (Model-Aware)

```rust
pub struct TokenEstimate {
    pub gpt4: u32,
    pub claude: u32,
    pub llama: u32,
    pub generic: u32,
}

impl TokenEstimate {
    pub fn compute(content: &Content) -> Self {
        match content {
            Content::Text { text, .. } => Self::estimate_text(text),
            Content::Code { source, language, .. } => Self::estimate_code(source, language),
            Content::Table { columns, rows, .. } => Self::estimate_table(columns, rows),
            _ => Self::default(),
        }
    }
    
    fn estimate_text(text: &str) -> Self {
        let chars = text.chars().count();
        let words = text.split_whitespace().count();
        let cjk_ratio = text.chars().filter(|c| is_cjk(*c)).count() as f32 / chars.max(1) as f32;
        
        let base = if cjk_ratio > 0.5 {
            (chars as f32 * 1.5) as u32  // CJK: ~1.5 tokens/char
        } else {
            (words as f32 * 1.3) as u32  // Latin: ~1.3 tokens/word
        };
        
        Self { gpt4: base, claude: (base as f32 * 1.1) as u32, llama: base, generic: base }
    }
}
```

### 4.6 Edge Types and Bidirectional Index

```rust
pub enum EdgeType {
    DerivedFrom, Supersedes, References, CitedBy,
    Supports, Contradicts, Elaborates, Summarizes,
    ParentOf, ChildOf, VersionOf, TranslationOf,
    Custom(String),
}

pub struct Edge {
    pub edge_type: EdgeType,
    pub target: BlockId,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Bidirectional index for O(1) edge lookups
pub struct EdgeIndex {
    outgoing: HashMap<BlockId, Vec<(EdgeType, BlockId)>>,
    incoming: HashMap<BlockId, Vec<(EdgeType, BlockId)>>,
}

impl EdgeIndex {
    pub fn add(&mut self, source: BlockId, edge: Edge) {
        self.outgoing.entry(source.clone()).or_default().push((edge.edge_type.clone(), edge.target.clone()));
        if let Some(inv) = edge.edge_type.inverse() {
            self.incoming.entry(edge.target).or_default().push((inv, source));
        }
    }
}
```

### 4.7 Document Structure

```rust
pub struct Document {
    pub id: DocumentId,
    pub root: BlockId,
    pub structure: HashMap<BlockId, Vec<BlockId>>,  // parent -> children
    pub blocks: HashMap<BlockId, Block>,
    pub metadata: DocumentMetadata,
    pub indices: DocumentIndices,
    pub edge_index: EdgeIndex,
    pub version: DocumentVersion,
}

pub struct DocumentIndices {
    pub by_tag: HashMap<String, HashSet<BlockId>>,
    pub by_role: HashMap<String, HashSet<BlockId>>,
    pub by_content_type: HashMap<ContentType, HashSet<BlockId>>,
    pub by_label: HashMap<String, BlockId>,
}
```

---

## 5. Unified Content Language (UCL)

### 5.1 Document Structure

```ucl
STRUCTURE
<adjacency declarations>

BLOCKS
<block definitions>

COMMANDS
<transformation commands>
```

### 5.2 Command Reference

#### EDIT - Content Modification
```ucl
EDIT <block_id> SET <path> = <value>
EDIT <block_id> SET <path> += <value>    # Append
EDIT <block_id> SET <path> -= <value>    # Remove
```

**Examples:**
```ucl
EDIT blk_intro SET content.text = "Welcome to UCP"
EDIT blk_users SET rows[0].email = "alice@example.com"
EDIT blk_users SET rows += [{"name": "Bob", "email": "bob@example.com"}]
EDIT blk_config SET $.database.host = "localhost"
EDIT blk_intro SET metadata.tags += ["important"]
```

#### MOVE - Structural Relocation
```ucl
MOVE <block_id> TO <parent_id> [AT <index>]
MOVE <block_id> BEFORE <sibling_id>
MOVE <block_id> AFTER <sibling_id>
SWAP <block_id_1> <block_id_2>
```

#### APPEND - Create New Blocks
```ucl
APPEND <parent_id> <type> [WITH <props>] :: <content>
```

**Examples:**
```ucl
APPEND blk_main text WITH label="Note" tags=["warning"] :: "Important notice"

APPEND blk_examples code WITH language="rust" ::
  ```
  fn main() { println!("Hello"); }
  ```

APPEND blk_data table WITH label="Sales" ::
  | Q | Revenue |
  |---|---------|
  | Q1| $100K   |
```

#### DELETE - Remove Blocks
```ucl
DELETE <block_id>
DELETE <block_id> CASCADE
DELETE <block_id> PRESERVE_CHILDREN
DELETE WHERE <condition>
```

#### PRUNE - Garbage Collection
```ucl
PRUNE UNREACHABLE
PRUNE WHERE <condition>
PRUNE UNREACHABLE DRY_RUN
```

#### FOLD - Context Compression
```ucl
FOLD <block_id> DEPTH <n>
FOLD <block_id> MAX_TOKENS <n>
FOLD <block_id> MAX_TOKENS <n> PRESERVE_TAGS ["critical"]
```

#### LINK - Edge Management
```ucl
LINK <source> <edge_type> <target> [WITH <metadata>]
UNLINK <source> <edge_type> <target>
```

#### SNAPSHOT - Version Management
```ucl
SNAPSHOT CREATE <name> [WITH description="..."]
SNAPSHOT RESTORE <name>
SNAPSHOT LIST
SNAPSHOT DELETE <name>
```

#### TRANSACTION - Atomic Operations
```ucl
BEGIN TRANSACTION [<name>]
COMMIT [<name>]
ROLLBACK [<name>]

ATOMIC {
  <commands>
}
```

### 5.3 Path Syntax

```
content.text              # Text content
rows[0]                   # First row (0-indexed)
rows[-1]                  # Last row
rows[1:3]                 # Slice
$.database.host           # JSONPath
source.lines[10:20]       # Code lines (1-indexed)
```

### 5.4 Conditions

```ucl
<path> = <value>
<path> CONTAINS <element>
<path> MATCHES <regex>
<cond1> AND <cond2>
<cond1> OR <cond2>
NOT <cond>
```

---

## 6. Query Language (UCQ)

### 6.1 Basic Queries

```ucq
SELECT * FROM blocks
SELECT id, content, metadata.label FROM blocks WHERE tags CONTAINS "important"
SELECT * FROM blocks WHERE content_type = "code" AND content.language = "rust"
```

### 6.2 Traversal Queries

```ucq
TRAVERSE FROM blk_root DEPTH 3
TRAVERSE FROM blk_root WHERE semantic_role STARTS_WITH "intro"
TRAVERSE EDGES FROM blk_source WHERE edge_type = "references"
```

### 6.3 Aggregation

```ucq
SELECT COUNT(*) FROM blocks WHERE tags CONTAINS "draft"
SELECT SUM(metadata.token_estimate.gpt4) FROM blocks
SELECT content_type, COUNT(*) FROM blocks GROUP BY content_type
```

---

## 7. Transformation Engine

### 7.1 Execution Pipeline

```
Input (UCL/UCQ) 
    → Lexer → Parser → AST 
    → Validator → Planner 
    → Executor → Output
```

### 7.2 Validation Pipeline

```rust
pub enum ValidationSeverity { Error, Warning, Info }

pub struct ValidationIssue {
    pub severity: ValidationSeverity,
    pub code: ErrorCode,
    pub message: String,
    pub location: Location,
    pub suggestion: Option<String>,
}

// Validation checks:
// 1. Structure: No cycles, all IDs exist, root reachable
// 2. Content: Valid for declared type, schema compliance
// 3. Commands: Valid syntax, valid targets, valid paths
// 4. Semantics: No orphans (warning), valid indices
```

### 7.3 Translator Interface

```rust
pub trait Translator: Send + Sync {
    fn parse(&self, input: &str, options: &ParseOptions) -> Result<Document, Error>;
    fn emit(&self, doc: &Document, options: &EmitOptions) -> Result<String, Error>;
    fn parse_streaming(&self, input: impl AsyncRead) -> impl Stream<Item = Result<Block, Error>>;
    fn capabilities(&self) -> TranslatorCapabilities;
}

pub struct TranslatorCapabilities {
    pub formats: Vec<String>,
    pub supports_streaming: bool,
    pub supports_incremental: bool,
    pub max_size: Option<usize>,
}
```

---

## 8. Concurrency and Transactions

### 8.1 Optimistic Concurrency Control

```rust
pub struct Version {
    pub counter: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub checksum: [u8; 8],
}

impl Document {
    pub fn apply_with_version(&mut self, op: Operation, expected: Version) -> Result<(), ConflictError> {
        if self.version != expected {
            return Err(ConflictError::VersionMismatch { expected, actual: self.version.clone() });
        }
        self.apply(op)?;
        self.version.increment();
        Ok(())
    }
}
```

### 8.2 Transaction Manager

```rust
pub struct Transaction {
    pub id: TransactionId,
    pub started_at: Instant,
    pub operations: Vec<Operation>,
    pub savepoints: Vec<Savepoint>,
    pub state: TransactionState,
}

pub enum TransactionState { Active, Committed, RolledBack }

impl TransactionManager {
    pub fn begin(&mut self) -> TransactionId { /* ... */ }
    pub fn commit(&mut self, id: TransactionId) -> Result<(), Error> { /* ... */ }
    pub fn rollback(&mut self, id: TransactionId) -> Result<(), Error> { /* ... */ }
}
```

### 8.3 Snapshot System

```rust
pub struct Snapshot {
    pub id: SnapshotId,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub document_version: DocumentVersion,
    pub data: SnapshotData,
}

pub enum SnapshotData {
    Full(Document),
    Delta { base: SnapshotId, changes: Vec<Change> },
}
```

---

## 9. Indexing and Performance

### 9.1 Index Structures

```rust
pub struct DocumentIndices {
    by_tag: HashMap<String, HashSet<BlockId>>,
    by_role: HashMap<String, HashSet<BlockId>>,
    by_type: HashMap<ContentType, HashSet<BlockId>>,
    by_label: HashMap<String, BlockId>,
}

impl DocumentIndices {
    pub fn update_on_insert(&mut self, block: &Block) { /* ... */ }
    pub fn update_on_delete(&mut self, block: &Block) { /* ... */ }
    pub fn update_on_modify(&mut self, old: &Block, new: &Block) { /* ... */ }
}
```

### 9.2 Performance Targets

| Operation | Target | Measurement |
|-----------|--------|-------------|
| Parse 10MB markdown | < 100ms | Time to UCM |
| Execute 1000 UCL commands | < 50ms | Batch execution |
| Validate 10k blocks | < 20ms | Full validation |
| Generate IDs (1M blocks) | < 1s | Deterministic |
| Index lookup | < 1µs | By tag/role |

### 9.3 Memory Management

```rust
pub struct ArenaAllocator {
    blocks: typed_arena::Arena<Block>,
    strings: bumpalo::Bump,
}

// Use arena allocation for batch operations
// Use Cow<str> for zero-copy string handling
// Implement streaming for large documents
```

---

## 10. Observability

### 10.1 Metrics

```rust
pub struct Metrics {
    // Counters
    pub operations_total: Counter,
    pub errors_total: Counter,
    pub transactions_total: Counter,
    
    // Histograms
    pub operation_duration: Histogram,
    pub document_size: Histogram,
    pub block_count: Histogram,
    
    // Gauges
    pub active_transactions: Gauge,
    pub memory_usage: Gauge,
}
```

### 10.2 Tracing

```rust
#[instrument(skip(self, doc), fields(doc_id = %doc.id, op_count = commands.len()))]
pub fn execute_batch(&self, doc: &mut Document, commands: Vec<Command>) -> Result<BatchResult> {
    let span = tracing::info_span!("execute_batch");
    let _guard = span.enter();
    // ...
}
```

### 10.3 Audit Log

```rust
pub struct AuditEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub operation: OperationType,
    pub document_id: DocumentId,
    pub block_ids: Vec<BlockId>,
    pub user_id: Option<String>,
    pub before_hash: [u8; 32],
    pub after_hash: [u8; 32],
}
```

---

## 11. Security Model

### 11.1 Input Validation Limits

```rust
pub struct ResourceLimits {
    pub max_document_size: usize,      // 50MB default
    pub max_block_count: usize,        // 100k blocks
    pub max_block_size: usize,         // 5MB per block
    pub max_nesting_depth: usize,      // 50 levels
    pub max_edges_per_block: usize,    // 1000 edges
    pub max_execution_time: Duration,  // 10s default
    pub max_memory: usize,             // 256MB default
}
```

### 11.2 Path Traversal Prevention

```rust
pub fn validate_media_source(source: &MediaSource) -> Result<(), SecurityError> {
    match source {
        MediaSource::Url(url) => {
            let parsed = Url::parse(url)?;
            if !["http", "https"].contains(&parsed.scheme()) {
                return Err(SecurityError::DisallowedScheme);
            }
            // No file:// or other local schemes
        }
        MediaSource::Reference(id) => {
            // Block references are always safe
        }
        _ => {}
    }
    Ok(())
}
```

---

## 12. Error Handling

### 12.1 Error Codes

| Code | Category | Description |
|------|----------|-------------|
| E001 | Reference | Block does not exist |
| E002 | Reference | Invalid block ID format |
| E003 | Syntax | Malformed UCL command |
| E004 | Syntax | Invalid path expression |
| E005 | Validation | Content schema violation |
| E006 | Validation | Cycle detected in structure |
| E007 | Concurrency | Version conflict |
| E008 | Concurrency | Transaction timeout |
| E009 | Resource | Document size exceeded |
| E010 | Resource | Memory limit exceeded |
| W001 | Warning | Orphaned block detected |
| W002 | Warning | Deprecated command usage |

### 12.2 Error Response Format

```json
{
  "success": false,
  "errors": [{
    "code": "E001",
    "message": "Block 'blk_invalid' does not exist",
    "location": { "line": 23, "column": 5 },
    "context": "MOVE blk_invalid TO blk_root",
    "suggestion": "Did you mean 'blk_intro'?"
  }],
  "warnings": [{
    "code": "W001",
    "message": "Block 'blk_orphan' is unreachable"
  }]
}
```

---

## 13. Versioning and Migration

### 13.1 Format Version

```rust
pub struct FormatVersion {
    pub major: u32,  // Breaking changes
    pub minor: u32,  // New features (backward compatible)
    pub patch: u32,  // Bug fixes
}

// Document header includes version
// Parser validates compatibility
```

### 13.2 Migration Strategy

- **Forward compatible**: v2.0 can read v1.x documents
- **Explicit migration**: `migrate_document(doc, target_version)`
- **Deprecation warnings**: Old features warn before removal

---

## 14. Implementation Roadmap

### Phase 1: Core (Weeks 1-4)
- [ ] `ucm-core`: Block, Document, Content types
- [ ] ID generation with 96-bit hashes
- [ ] Content normalization (Unicode NFC)
- [ ] Basic validation

### Phase 2: Engine (Weeks 5-8)
- [ ] `ucl-parser`: Lexer, parser, AST
- [ ] Transaction manager
- [ ] Snapshot system
- [ ] Index structures

### Phase 3: Query (Weeks 9-10)
- [ ] `ucq-parser`: Query language
- [ ] Query planner
- [ ] Aggregations

### Phase 4: Translators (Weeks 11-14)
- [ ] Markdown translator
- [ ] HTML translator
- [ ] JSON translator
- [ ] Streaming support

### Phase 5: Observability (Weeks 15-16)
- [ ] Metrics integration
- [ ] Tracing integration
- [ ] Audit logging

---

## 15. Formal Grammar

```ebnf
ucl_document    ::= structure_section blocks_section commands_section
structure_section ::= "STRUCTURE" EOL (structure_decl)*
structure_decl  ::= block_id ":" "[" block_id_list "]" EOL
block_id_list   ::= block_id ("," block_id)*
blocks_section  ::= "BLOCKS" EOL (block_decl)*
block_decl      ::= content_type "#" block_id properties? "::" content EOL
content_type    ::= "text" | "table" | "code" | "math" | "media" | "json" | "binary" | "composite"
properties      ::= property ("," property)*
property        ::= identifier "=" value
commands_section ::= "COMMANDS" EOL (command)*
command         ::= edit_cmd | move_cmd | append_cmd | delete_cmd | prune_cmd | link_cmd | snapshot_cmd | transaction_cmd
edit_cmd        ::= "EDIT" block_id "SET" path operator value
move_cmd        ::= "MOVE" block_id "TO" block_id ("AT" index)?
                  | "MOVE" block_id ("BEFORE" | "AFTER") block_id
append_cmd      ::= "APPEND" block_id content_type ("WITH" properties)? "::" content
delete_cmd      ::= "DELETE" block_id ("CASCADE" | "PRESERVE_CHILDREN")?
                  | "DELETE" "WHERE" condition
prune_cmd       ::= "PRUNE" ("UNREACHABLE" | "WHERE" condition) ("DRY_RUN")?
link_cmd        ::= "LINK" block_id edge_type block_id ("WITH" properties)?
                  | "UNLINK" block_id edge_type block_id
snapshot_cmd    ::= "SNAPSHOT" ("CREATE" string | "RESTORE" string | "LIST" | "DELETE" string)
transaction_cmd ::= "BEGIN" "TRANSACTION" string? | "COMMIT" string? | "ROLLBACK" string?
                  | "ATOMIC" "{" (command)* "}"
path            ::= identifier ("." identifier | "[" index_expr "]")*
                  | "$" json_path
operator        ::= "=" | "+=" | "-="
index_expr      ::= integer | integer ":" integer | ":" integer | integer ":"
condition       ::= comparison (("AND" | "OR") comparison)*
comparison      ::= path comp_op value | path "CONTAINS" value | path "MATCHES" string
                  | "NOT" condition | "(" condition ")"
comp_op         ::= "=" | "!=" | ">" | ">=" | "<" | "<="
value           ::= string | number | boolean | "null" | array | object | block_ref
string          ::= '"' characters '"' | "'" characters "'" | '"""' characters '"""'
number          ::= integer | float
boolean         ::= "true" | "false"
array           ::= "[" (value ("," value)*)? "]"
object          ::= "{" (string ":" value ("," string ":" value)*)? "}"
block_ref       ::= "@" block_id
block_id        ::= "blk_" hex_chars
edge_type       ::= "derived_from" | "supersedes" | "references" | "supports" | "contradicts" | "custom:" identifier
```

---

## 16. Extended Examples

### 16.1 Blog Post Document

```ucl
STRUCTURE
blk_root: [blk_header, blk_body, blk_footer]
blk_body: [blk_intro, blk_section1, blk_code_demo, blk_section2, blk_conclusion]

BLOCKS
text#blk_header label="Title" semantic_role="title" ::
  "# Understanding Async Rust"

text#blk_intro label="Introduction" semantic_role="intro.hook" tags=["overview"] ::
  """Asynchronous programming in Rust provides zero-cost abstractions
  for concurrent operations without sacrificing safety."""

code#blk_code_demo label="Async Example" language="rust" ::
  ```rust
  async fn fetch_data(url: &str) -> Result<String, Error> {
      let response = reqwest::get(url).await?;
      response.text().await
  }
  ```

table#blk_perf label="Performance" semantic_role="body.evidence" ::
  | Approach | Throughput | Latency |
  |----------|------------|---------|
  | Sync     | 1000 req/s | 10ms    |
  | Async    | 10000 req/s| 1ms     |

COMMANDS
EDIT blk_perf SET rows[0].Throughput = "1200 req/s"
LINK blk_code_demo elaborates blk_intro
SNAPSHOT CREATE "initial-draft"
```

### 16.2 Interactive Form

```ucl
STRUCTURE
blk_form: [blk_header, blk_personal, blk_submit]
blk_personal: [blk_name, blk_email]

BLOCKS
json#blk_personal label="Personal Info" ::
  {
    "fields": [
      {"name": "full_name", "type": "text", "required": true, "validation": {"minLength": 2}},
      {"name": "email", "type": "email", "required": true}
    ]
  }

COMMANDS
ATOMIC {
  EDIT blk_personal SET $.fields[0].validation.maxLength = 100
  APPEND blk_personal json WITH label="Phone" ::
    {"name": "phone", "type": "tel", "pattern": "[0-9]{3}-[0-9]{4}"}
}
```

### 16.3 Mathematical Document

```ucl
STRUCTURE
blk_theorem: [blk_statement, blk_equation, blk_proof]

BLOCKS
text#blk_statement label="Theorem" semantic_role="definition" ::
  "For any prime p and integer a not divisible by p:"

math#blk_equation label="Fermat's Little Theorem" format="latex" ::
  "a^{p-1} \\equiv 1 \\pmod{p}"

COMMANDS
LINK blk_equation elaborates blk_statement
FOLD blk_theorem MAX_TOKENS 200 PRESERVE_TAGS ["definition"]
```

---

## 17. LLM Benchmarking System

The UCP includes a comprehensive benchmarking system for evaluating LLM performance on document manipulation tasks. This enables systematic comparison of different models, providers, and configurations.

### 17.1 Design Goals

1. **Provider Abstraction**: Plug-and-play support for OpenAI, Anthropic, Google, local models, etc.
2. **Comprehensive Metrics**: Latency, cost, success rate, token usage, error categorization
3. **Command Coverage**: Test all UCL commands with varying complexity
4. **Reproducibility**: Deterministic test documents and evaluation criteria
5. **Cost Awareness**: Track and report actual API costs per model/test

### 17.2 Provider Abstraction

```rust
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Provider identifier (e.g., "openai", "anthropic")
    fn provider_id(&self) -> &str;
    
    /// Model identifier (e.g., "gpt-4o", "claude-3-sonnet")
    fn model_id(&self) -> &str;
    
    /// Execute a completion request
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;
    
    /// Get pricing info for cost calculation
    fn pricing(&self) -> TokenPricing;
}

pub struct TokenPricing {
    pub input_cost_per_1k: f64,   // USD per 1K input tokens
    pub output_cost_per_1k: f64,  // USD per 1K output tokens
    pub currency: String,
}

pub struct CompletionRequest {
    pub system_prompt: String,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub stop_sequences: Vec<String>,
}

pub struct CompletionResponse {
    pub content: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub latency_ms: u64,
    pub finish_reason: FinishReason,
}
```

### 17.3 Test Document Structure

The benchmark uses a canonical test document with diverse content types:

```
TestDocument
├── metadata (title, author, created)
├── introduction
│   ├── hook (text)
│   ├── context (text)  
│   └── thesis (text)
├── body
│   ├── section_1
│   │   ├── heading (text)
│   │   ├── paragraph (text)
│   │   ├── code_example (code)
│   │   └── table_data (table)
│   ├── section_2
│   │   ├── heading (text)
│   │   ├── math_formula (math)
│   │   └── diagram_ref (media)
│   └── section_3
│       ├── heading (text)
│       └── nested_list (composite)
├── conclusion
│   ├── summary (text)
│   └── call_to_action (text)
└── references (json)
```

### 17.4 Command Test Suite

Each UCL command is tested with multiple scenarios:

| Command | Test Scenarios |
|---------|---------------|
| `EDIT` | Simple text edit, JSON path edit, conditional edit, regex replacement |
| `APPEND` | Add text block, add code block, add at specific index |
| `MOVE` | Move to parent, move before sibling, move after sibling |
| `DELETE` | Single block, cascade delete, preserve children |
| `PRUNE` | Unreachable blocks, conditional prune |
| `LINK` | Create edge, typed edge with metadata |
| `UNLINK` | Remove specific edge |
| `FOLD` | Depth-based, token-based, tag preservation |
| `SNAPSHOT` | Create, restore, diff |
| `TRANSACTION` | Begin/commit, rollback, atomic blocks |

### 17.5 Agent Architecture

The benchmark agent follows a structured approach:

```
┌─────────────────────────────────────────────────────────┐
│                    Benchmark Runner                      │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
│  │ Test Case   │  │   Agent     │  │  Evaluator  │     │
│  │ Generator   │→ │  Executor   │→ │  & Scorer   │     │
│  └─────────────┘  └─────────────┘  └─────────────┘     │
│         │               │                │              │
│         ▼               ▼                ▼              │
│  ┌─────────────────────────────────────────────────┐   │
│  │              Metrics Collector                   │   │
│  │  • Latency (p50, p95, p99)                      │   │
│  │  • Token usage (input/output)                   │   │
│  │  • Cost (per test, cumulative)                  │   │
│  │  • Success rate (by command type)              │   │
│  │  • Error categorization                         │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### 17.6 Evaluation Criteria

Tests are evaluated on multiple dimensions:

1. **Correctness**: Does the generated UCL parse and execute without errors?
2. **Semantic Accuracy**: Does the command achieve the intended goal?
3. **Efficiency**: Is the command minimal (no unnecessary operations)?
4. **Safety**: Does the command avoid destructive side effects?

```rust
pub struct TestResult {
    pub test_id: String,
    pub command_type: String,
    pub model: String,
    pub provider: String,
    
    // Execution metrics
    pub latency_ms: u64,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cost_usd: f64,
    
    // Evaluation
    pub parse_success: bool,
    pub execute_success: bool,
    pub semantic_score: f32,  // 0.0 - 1.0
    pub efficiency_score: f32,
    
    // Error info
    pub error: Option<BenchmarkError>,
    pub error_category: Option<ErrorCategory>,
}

pub enum ErrorCategory {
    ParseError,        // UCL syntax error
    ExecutionError,    // Runtime error during execution
    SemanticError,     // Command doesn't achieve goal
    TimeoutError,      // LLM response timeout
    RateLimitError,    // Provider rate limiting
    InvalidResponse,   // Response not valid UCL
}
```

### 17.7 Benchmark Report Format

```json
{
  "benchmark_id": "bench_20240107_143022",
  "timestamp": "2024-01-07T14:30:22Z",
  "duration_seconds": 342,
  "models_tested": ["gpt-4o", "claude-3-sonnet", "gemini-pro"],
  
  "summary": {
    "total_tests": 150,
    "passed": 142,
    "failed": 8,
    "total_cost_usd": 2.47,
    "avg_latency_ms": 1234
  },
  
  "by_model": {
    "gpt-4o": {
      "tests": 50,
      "success_rate": 0.96,
      "avg_latency_ms": 1456,
      "total_cost_usd": 1.23,
      "by_command": {
        "EDIT": {"success_rate": 1.0, "avg_latency_ms": 1200},
        "APPEND": {"success_rate": 0.95, "avg_latency_ms": 1100}
      }
    }
  },
  
  "failures": [
    {
      "test_id": "edit_conditional_003",
      "model": "claude-3-sonnet",
      "error_category": "SemanticError",
      "expected": "EDIT blk_abc SET text = \"updated\"",
      "actual": "EDIT blk_abc SET content = \"updated\"",
      "diff": "path mismatch: text vs content"
    }
  ]
}
```

### 17.8 System Benchmarks

In addition to LLM benchmarks, the system includes performance benchmarks:

| Benchmark | Target | Metric |
|-----------|--------|--------|
| ID Generation | 100K blocks | < 50ms |
| Content Normalization | 1MB text | < 10ms |
| Block Append | 10K operations | < 100ms |
| Document Validation | 50K blocks | < 500ms |
| Snapshot Create | 10K blocks | < 200ms |
| UCL Parse | 1K commands | < 50ms |

---

## License

This specification is released under the MIT License.
