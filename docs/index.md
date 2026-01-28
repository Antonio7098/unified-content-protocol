# Unified Content Protocol Documentation Index

Welcome to the Unified Content Protocol (UCP) docs. Jump directly to any major area below.

## Quick Install

=== "Rust"
    ```toml
    [dependencies]
    ucp-api = "0.1.9"
    ```

=== "Python"
    ```bash
    pip install ucp-content
    ```

=== "JavaScript"
    ```bash
    npm install ucp-content
    ```

## Getting Started

- [Installation](./getting-started/installation.md)
- [Quick Start Guide](./getting-started/quick-start.md)
- [Core Concepts](./getting-started/concepts.md)
- [CLI Usage Guide](./getting-started/cli-guide.md)

## Core Model (ucm-core)

- [Overview](./ucm-core/README.md)
- [Blocks](./ucm-core/blocks.md)
- [Content Types](./ucm-core/content-types.md)
- [Documents](./ucm-core/documents.md)
- [Edges & Relationships](./ucm-core/edges.md)
- [Metadata & Semantic Roles](./ucm-core/metadata.md)
- [ID Generation](./ucm-core/id-generation.md)

## Transformation Engine (ucm-engine)

- [Overview](./ucm-engine/README.md)
- [Operations](./ucm-engine/operations.md)
- [Transactions](./ucm-engine/transactions.md)
- [Snapshots](./ucm-engine/snapshots.md)
- [Validation](./ucm-engine/validation.md)

## Unified Content Language (ucl-parser)

- [Overview](./ucl-parser/README.md)
- [Syntax Reference](./ucl-parser/syntax.md)
- [Command Reference](./ucl-parser/commands.md)
- [Expressions](./ucl-parser/expressions.md)

## High-Level API (ucp-api)

- [Client Overview](./ucp-api/README.md)

## JavaScript SDK (@ucp-core/core)

- [SDK Reference & Installation](./ucp-js/README.md)

## Command-Line Interface (ucp-cli)

- [Overview & Command Reference](./ucp-cli/README.md)
- [Hands-on Usage Guide](./getting-started/cli-guide.md)

```bash
cargo run -p ucp-cli -- --help
cargo run -p ucp-cli -- create --title "My Document"
```

## Translators

- [Markdown Translator](./translators/markdown/README.md)
- [HTML Translator](./translators/html/README.md)

## LLM Utilities (ucp-llm)

- [Overview & Context Management](./ucp-llm/README.md)

## Agent Graph Traversal (ucp-agent)

- [Overview](./ucp-agent/index.md)
- [API Reference](./ucp-agent/api.md)
- [UCL Commands](./ucp-agent/ucl-commands.md)
- [Usage Examples](./ucp-agent/examples.md)
- [Architecture](./ucp-agent/architecture.md)

## Observability (ucp-observe)

- [Tracing, Audit, Metrics](./ucp-observe/README.md)

## Examples

- [Basic Examples](./examples/basic.md)
- [Intermediate Examples](./examples/intermediate.md)
- [Advanced Examples](./examples/advanced.md)
