# Unified Content Protocol Documentation Index

Welcome to the Unified Content Protocol (UCP) docs. Jump directly to any major area below.

## Quick Install

=== "CLI (Recommended)"
    ```bash
    # Install the command-line tool
    cargo install ucp-cli

    # Verify installation
    ucp --version

    # Create your first document
    ucp create --title "My First Document" --output doc.json
    ```

    **Or from source:**
    ```bash
    cargo install --path crates/ucp-cli
    ```

    [CLI Documentation](./ucp-cli/README.md) | [Getting Started Guide](./getting-started/quick-start.md)

=== "Rust Library"
    ```toml
    [dependencies]
    ucp-api = "0.1.10"
    ```

    [Rust API Docs](./ucp-api/README.md) | [Installation Guide](./getting-started/installation.md)

=== "Python"
    ```bash
    pip install ucp-content
    ```

    [Python SDK Reference](./ucp-js/README.md)

=== "JavaScript"
    ```bash
    npm install ucp-content
    ```

    [JavaScript SDK Reference](./ucp-js/README.md)

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

- [CLI Overview & Installation](./ucp-cli/README.md)
- [Hands-on Usage Guide](./getting-started/cli-guide.md)

```bash
# Install from crates.io
cargo install ucp-cli

# Or install from source
cargo install --path crates/ucp-cli

# Get help
ucp --help

# Create a document
ucp create --title "My Document" --output doc.json
```

**crates.io:** [ucp-cli](https://crates.io/crates/ucp-cli)

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
