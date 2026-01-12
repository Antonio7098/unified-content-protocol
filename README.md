# Unified Content Protocol (UCP)

Unified Content Protocol is a graph-based intermediate representation for structured content. It provides deterministic, token-efficient building blocks that make it easy to ingest, transform, and serve complex documents to both humans and LLM-powered systems.

## Why UCP?

- **Deterministic core model** – Content-addressed blocks (`BlockId`) guarantee that identical content always hashes to the same ID.
- **Rich content types** – Text, code, tables, math, media, JSON, binary, and composite blocks share consistent metadata, edges, and versioning.
- **Transformation engine** – Batched operations, transactions, snapshots, and validation keep documents consistent while edits stream in.
- **Unified Content Language (UCL)** – A token-efficient DSL for scripting structural changes, suitable for LLM prompting and automation.
- **High-level API** – `ucp-api` wraps all core crates into an ergonomic client for applications and services.
- **Observability-first** – Built-in tracing hooks, audit helpers, and metrics collection make it production friendly.

## Repository Layout

```
.
├── crates/
│   ├── ucm-core/            # Core types (Block, Content, Document, Edge, Metadata)
│   ├── ucm-engine/          # Transformation engine, transactions, snapshots, validation
│   ├── ucl-parser/          # Lexer, parser, and AST for Unified Content Language
│   ├── ucp-api/             # High-level API surface bundling core crates
│   ├── ucp-observe/         # Tracing, audit logging, metrics helpers
│   └── translators/
│       └── markdown/        # Markdown ⇄ UCP translator
├── docs/                    # Full documentation set (see below)
└── ...                      # Tooling, workspace config, CI, etc.
```

## Getting Started

1. **Install Rust** (1.70 or newer) and clone the repository:
   ```bash
   git clone https://github.com/<org>/unified-content-protocol.git
   cd unified-content-protocol
   ```
2. **Build the workspace**:
   ```bash
   cargo build --workspace
   ```
3. **Run tests**:
   ```bash
   cargo test --workspace
   ```
4. **Add to another project** (example using the high-level API):
   ```toml
   [dependencies]
   ucp-api = { path = "../unified-content-protocol/crates/ucp-api" }
   ```

## Documentation

All documentation lives in [`/docs`](./docs/index.md) and is structured for direct use by the documentation frontend. Highlights:

- **Getting Started** – Installation, quick start, and core concepts
- **ucm-core** – Blocks, content types, metadata, IDs, documents, edges
- **ucm-engine** – Operations, transactions, snapshots, validation
- **ucl-parser** – Full UCL syntax + command reference
- **ucp-api** – Client usage patterns and examples
- **Translators** – Markdown conversion walkthrough
- **ucp-observe** – Tracing, audit, and metrics helpers
- **Examples** – Basic → Advanced programs covering real workflows

## Example Snippet

```rust
use serde_json::Value;
use ucp_api::UcpClient;
use ucm_core::{Content, Document};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UcpClient::new();
    let mut doc = client.create_document();
    let root = doc.root.clone();

    // Add content
    client.add_text(&mut doc, &root, "Hello, UCP!", Some("intro"))?;
    client.execute_ucl(&mut doc, r#"
        APPEND blk_ff00000000000000000000 code WITH lang="rust" :: "fn main() {}"
    "#)?;

    // Validate and serialize
    if doc.validate().is_empty() {
        println!("Document is valid");
    }

    let json = client.to_json(&doc)?;
    println!("{}", json);

    // Pretty-print if desired
    let pretty: Value = serde_json::from_str(&json)?;
    println!("{}", serde_json::to_string_pretty(&pretty)?);
    Ok(())
}
```

## Contributing

1. Fork the repository and create a feature branch.
2. Run `cargo fmt`, `cargo clippy --all-targets`, and `cargo test --workspace` before opening a PR.
3. Follow the documentation style and include examples where appropriate.

## License

This project follows the license declared in the workspace `Cargo.toml`. See the [`LICENSE`](./LICENSE) file for details.
