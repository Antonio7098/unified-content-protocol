# Unified Content Protocol (UCP) Documentation

UCP is a graph-based content model and tooling stack for deterministic document operations, codebase extraction, and LLM context assembly.

## Main Components

- `ucm-core`: core document/block/edge model
- `ucm-engine`: operations, transactions, snapshots, validation
- `ucl-parser`: command language parser
- `ucp-api`: high-level Rust API
- `ucp-cli`: automation and terminal workflows
- `ucp-llm`: ID mapping and prompt builder utilities
- `ucp-agent`: traversal/session tools
- translators: Markdown and HTML

## Install

```bash
cargo install ucp-cli
```

```toml
[dependencies]
ucp-api = "0.1.12"
```

## CodeGraph Pipeline (Source Code -> UCM)

CodeGraph extraction uses Tree-sitter parsers for Rust, Python, TypeScript, and JavaScript.

```bash
# Build graph from a repository
ucp codegraph build --repo /path/to/repo --output graph.json --format json

# Validate CodeGraphProfile v1 and compute fingerprint
ucp codegraph inspect --input graph.json --format json

# Render prompt projection
ucp codegraph prompt --input graph.json --output graph-projection.txt
```

## LLM Workflow with CodeGraph

```bash
ucp llm id-map --input graph.json --output graph-ids.json
ucp llm context --input graph.json --max-tokens 3200 > graph-context.txt
ucp llm prompt --capabilities all > system-prompt.txt
```

## Docs Map

- `docs/index.md`
- `docs/ucp-api/README.md`
- `docs/ucp-cli/README.md`
- `docs/ucp-llm/README.md`
- `docs/ucp-agent/index.md`
