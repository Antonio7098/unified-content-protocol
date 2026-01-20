# Unified Content Protocol (UCP) Documentation

Welcome to the comprehensive documentation for the **Unified Content Protocol (UCP)** — a graph-based intermediate representation for structured content, designed for efficient manipulation by Large Language Models (LLMs).

## Overview

UCP provides a token-efficient, deterministic framework for representing and transforming structured documents. It consists of several interconnected crates that work together to provide a complete content management solution.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         ucp-api                                  │
│              (High-level API for applications)                   │
├─────────────────────────────────────────────────────────────────┤
│     ucl-parser          │         ucm-engine                     │
│  (Command Language)     │    (Transformation Engine)             │
├─────────────────────────────────────────────────────────────────┤
│                         ucm-core                                 │
│         (Core Types: Block, Document, Content, Edge)             │
├─────────────────────────────────────────────────────────────────┤
│ ucp-translator-markdown / ucp-translator-html │    ucp-observe      │
│             (Format Translators)              │ (Observability)    │
└─────────────────────────────────────────────────────────────────┘
```

## Core Crates

| Crate | Description |
|-------|-------------|
| [`ucm-core`](./ucm-core/README.md) | Core types and traits for the Unified Content Model |
| [`ucm-engine`](./ucm-engine/README.md) | Transformation engine for applying operations to documents |
| [`ucl-parser`](./ucl-parser/README.md) | Parser for the Unified Content Language (UCL) |
| [`ucp-api`](./ucp-api/README.md) | High-level API for application integration |
| [`ucp-translator-markdown`](./translators/markdown/README.md) | Bidirectional Markdown conversion |
| [`ucp-translator-html`](./translators/html/README.md) | HTML → UCM translator with semantic extraction |
| [`ucp-llm`](./ucp-llm/README.md) | LLM utilities (ID mapping, prompt building) |
| [`ucp-observe`](./ucp-observe/README.md) | Observability utilities (tracing, metrics, audit) |

## Quick Start

### Installation

Add UCP to your `Cargo.toml`:

```toml
[dependencies]
ucp-api = "0.1.4"
```

### Basic Usage

```rust
use ucp_api::UcpClient;

fn main() {
    // Create a client
    let client = UcpClient::new();
    
    // Create a new document
    let mut doc = client.create_document();
    
    // Add content
    let root = doc.root.clone();
    client.add_text(&mut doc, &root, "Hello, UCP!", Some("intro")).unwrap();
    
    // Execute UCL commands
    client.execute_ucl(&mut doc, r#"
        EDIT blk_abc123 SET content.text = "Updated content"
    "#).unwrap();
}
```

## Key Concepts

### Blocks

The fundamental unit of content in UCP. Each block:
- Has a **content-addressed ID** (96-bit, deterministic)
- Contains **typed content** (text, code, table, math, media, JSON, binary)
- Carries **metadata** (semantic role, tags, labels, token estimates)
- Can have **edges** to other blocks (relationships)

### Documents

A collection of blocks organized in a hierarchical tree structure:
- Single **root block** as entry point
- **Adjacency map** defining parent-child relationships
- **Secondary indices** for fast lookup by tag, role, label, or content type
- **Edge index** for relationship traversal
- **Traversal utilities** (BFS/DFS/path-finding) for navigation and context gathering

### UCL (Unified Content Language)

A token-efficient command language for document manipulation:
- `EDIT` - Modify block content or metadata
- `APPEND` - Add new blocks
- `MOVE` - Reorganize structure
- `DELETE` - Remove blocks
- `LINK/UNLINK` - Manage relationships
- `SNAPSHOT` - Version management
- `WRITE_SECTION` - Replace section content from Markdown with undo support

## Documentation Structure

```
docs/
├── README.md                    # This file
├── getting-started/
│   ├── installation.md
│   ├── quick-start.md
│   └── concepts.md
├── ucm-core/
│   ├── README.md
│   ├── blocks.md
│   ├── content-types.md
│   ├── documents.md
│   ├── edges.md
│   ├── metadata.md
│   └── id-generation.md
├── ucm-engine/
│   ├── README.md
│   ├── operations.md
│   ├── transactions.md
│   ├── snapshots.md
│   └── validation.md
├── ucl-parser/
│   ├── README.md
│   ├── syntax.md
│   ├── commands.md
│   └── expressions.md
├── ucp-api/
│   ├── README.md
│   └── client.md
├── translators/
│   ├── markdown/
│   │   └── README.md
│   └── html/
│       └── README.md
├── ucp-llm/
│   └── README.md
├── ucp-observe/
│   └── README.md
└── examples/
    ├── basic.md
    ├── intermediate.md
    └── advanced.md
```

## License

See the LICENSE file in the repository root.
