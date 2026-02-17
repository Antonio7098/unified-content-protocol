# Unified Content Protocol Documentation Index

Welcome to the Unified Content Protocol (UCP) docs.

## Quick Install

### CLI (Recommended)

```bash
cargo install ucp-cli
ucp --version
```

### Rust Library

```toml
[dependencies]
ucp-api = "0.1.13"
```

## Getting Started

- [Installation](./getting-started/installation.md)
- [Quick Start Guide](./getting-started/quick-start.md)
- [Core Concepts](./getting-started/concepts.md)

## API and Tools

- [UCP API](./ucp-api/README.md)
- [UCP CLI](./ucp-cli/README.md)
- [UCP LLM Utilities](./ucp-llm/README.md)
- [UCP Agent](./ucp-agent/index.md)
- [UCP Observe](./ucp-observe/README.md)

## CodeGraph (Tree-sitter)

- Build graph: `ucp codegraph build`
- Validate graph profile + fingerprint: `ucp codegraph inspect`
- Create projection for LLM context: `ucp codegraph prompt`

Example:

```bash
ucp codegraph build --repo /path/to/repo --output graph.json --format json
ucp codegraph inspect --input graph.json --format json
ucp codegraph prompt --input graph.json --output graph-projection.txt
```

## Core Model

- [ucm-core](./ucm-core/README.md)
- [ucm-engine](./ucm-engine/README.md)
- [ucl-parser](./ucl-parser/README.md)

## Translators

- [Markdown](./translators/markdown/README.md)
- [HTML](./translators/html/README.md)

## Examples

- [Basic](./examples/basic.md)
- [Intermediate](./examples/intermediate.md)
- [Advanced](./examples/advanced.md)
