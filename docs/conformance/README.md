# UCP Cross-Language Conformance Specification

This document defines the canonical behaviors that all UCP implementations (Rust, TypeScript, Python) must follow to ensure interoperability.

## Overview

The Rust implementation in `crates/` is the **reference implementation**. SDKs in `packages/` should either:
1. Use WASM/FFI bindings to the Rust core, or
2. Implement the spec exactly as defined here

## Core Types

### BlockId

A 96-bit (12-byte) content-addressed identifier.

**Format**: `blk_` prefix followed by 24 lowercase hexadecimal characters.

```
blk_000000000000  (root block)
blk_a1b2c3d4e5f6  (content-derived)
```

**Generation Algorithm**:
1. Normalize content (NFC Unicode normalization)
2. Compute SHA-256 hash of: `content_bytes || semantic_role || namespace`
3. Take first 12 bytes of hash
4. Encode as lowercase hex with `blk_` prefix

**Test Vectors**:
| Content | Role | Namespace | Expected ID |
|---------|------|-----------|-------------|
| "Hello" | "intro" | None | Implementation-defined (run Rust to get canonical) |
| "" | None | None | `blk_000000000000` (root) |

### ContentHash

A 256-bit SHA-256 hash of normalized content.

**Format**: 64 lowercase hexadecimal characters.

### Document

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | DocumentId | Yes | Unique document identifier |
| `root` | BlockId | Yes | Root block ID (always exists) |
| `blocks` | Map<BlockId, Block> | Yes | All blocks in document |
| `children` | Map<BlockId, Vec<BlockId>> | Yes | Parent → children mapping |
| `metadata` | DocumentMetadata | No | Document-level metadata |
| `version` | DocumentVersion | Yes | Version tracking |

### Block

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | BlockId | Yes | Content-derived identifier |
| `content` | Content | Yes | Typed content |
| `metadata` | BlockMetadata | Yes | Block metadata |
| `edges` | Vec<Edge> | No | Explicit relationships |
| `version` | Version | Yes | Optimistic concurrency |

### Content Types

All implementations must support:

| Type | Rust | TypeScript | Python |
|------|------|------------|--------|
| Text | `Content::Text` | `type: 'text'` | `ContentType.TEXT` |
| Code | `Content::Code` | `type: 'code'` | `ContentType.CODE` |
| Table | `Content::Table` | `type: 'table'` | `ContentType.TABLE` |
| Math | `Content::Math` | `type: 'math'` | `ContentType.MATH` |
| Media | `Content::Media` | `type: 'media'` | `ContentType.MEDIA` |
| Json | `Content::Json` | `type: 'json'` | `ContentType.JSON` |

### Edge Types

| Type | Description |
|------|-------------|
| `references` | General reference |
| `derived_from` | Content derivation |
| `supersedes` | Version replacement |
| `supports` | Semantic support |
| `contradicts` | Semantic contradiction |
| `elaborates` | Expansion of content |
| `summarizes` | Condensed version |
| `parent_of` | Structural parent |
| `child_of` | Structural child |

## UCL Commands

### EDIT

```ucl
EDIT <block_id> SET <path> = <value>
EDIT <block_id> APPEND <path> = <value>
EDIT <block_id> REMOVE <path> = <value>
```

**Paths**:
- `text` or `content.text` - Text content
- `metadata.label` - Block label
- `metadata.tags` - Tags array
- `metadata.summary` - Summary
- `metadata.custom.<key>` - Custom metadata

### APPEND

```ucl
APPEND <parent_id> <content_type> :: <content>
APPEND <parent_id> <content_type> WITH label="x", role="y" :: <content>
APPEND <parent_id> <content_type> AT <index> :: <content>
```

### MOVE

```ucl
MOVE <block_id> TO <new_parent_id>
MOVE <block_id> TO <new_parent_id> AT <index>
MOVE <block_id> BEFORE <sibling_id>
MOVE <block_id> AFTER <sibling_id>
```

### DELETE

```ucl
DELETE <block_id>
DELETE <block_id> CASCADE
DELETE <block_id> PRESERVE_CHILDREN
```

### LINK / UNLINK

```ucl
LINK <source_id> <edge_type> <target_id>
LINK <source_id> <edge_type> <target_id> WITH confidence=0.9
UNLINK <source_id> <edge_type> <target_id>
```

### PRUNE

```ucl
PRUNE UNREACHABLE
PRUNE WHERE tag = "draft"
```

### SNAPSHOT

```ucl
SNAPSHOT CREATE "name"
SNAPSHOT CREATE "name" WITH description="..."
SNAPSHOT RESTORE "name"
SNAPSHOT DELETE "name"
SNAPSHOT LIST
```

### ATOMIC

```ucl
ATOMIC {
  EDIT ...
  APPEND ...
}
```

## Validation

### Error Codes

| Code | Severity | Description |
|------|----------|-------------|
| E001 | Error | Block not found |
| E002 | Error | Invalid block ID format |
| E101 | Error | Parse error |
| E201 | Error | Cycle detected |
| E202 | Error | Invalid parent reference |
| E203 | Warning | Orphaned block |
| E301 | Error | Transaction conflict |
| E400 | Error | Block count limit exceeded |
| E401 | Error | Document size limit exceeded |
| E402 | Error | Block size limit exceeded |
| E403 | Error | Nesting depth limit exceeded |
| E404 | Error | Edge count limit exceeded |

### Default Limits

| Limit | Value |
|-------|-------|
| Max document size | 50 MB |
| Max block count | 100,000 |
| Max block size | 5 MB |
| Max nesting depth | 50 |
| Max edges per block | 1,000 |

## Serialization

### JSON Format

Documents serialize to JSON with stable key ordering:

```json
{
  "id": "doc_...",
  "root": "blk_000000000000",
  "blocks": {
    "blk_000000000000": {
      "id": "blk_000000000000",
      "content": { "type": "text", "text": "" },
      "metadata": { ... },
      "edges": [],
      "version": { "counter": 1, "timestamp": "..." }
    }
  },
  "children": {
    "blk_000000000000": ["blk_..."]
  },
  "metadata": { ... },
  "version": { ... }
}
```

### UCL Format

```ucl
STRUCTURE
blk_000000000000: [blk_111111111111, blk_222222222222]
blk_111111111111: []

BLOCKS
text #blk_111111111111 label="Introduction" :: "Hello, world!"

COMMANDS
EDIT blk_111111111111 SET text = "Updated"
```

## Conformance Test Suite

Each implementation must pass the test cases in `tests/conformance/`:

1. **ID Generation** - Verify BlockId generation matches reference
2. **Document Operations** - Create, add, move, delete blocks
3. **UCL Parsing** - Parse all command types
4. **UCL Execution** - Execute commands with expected results
5. **Validation** - Detect all error conditions
6. **Serialization** - Round-trip JSON/UCL formats
7. **Edge Operations** - Link/unlink with all edge types

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2026-01-09 | Initial spec |
