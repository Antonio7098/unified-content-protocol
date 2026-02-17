# Unified Content Protocol (UCP)

> Latest release: v0.1.13

Unified Content Protocol is a graph-based intermediate representation (IR) for structured documents. It provides deterministic, token-efficient building blocks so teams can ingest, transform, and serve complex knowledge graphs to both traditional applications and LLM-powered agents.

## Highlights

- **Deterministic Rust core** – Content-addressed blocks (`BlockId`) ensure reproducible IDs, metadata, and edges no matter where content was created.
- **Full fidelity content types** – Text, code, tables, math, media, JSON, composites, and binary payloads all follow the same schema and validation rules.
- **Powerful automation surface** – The `ucp-cli` binary exposes every operation (document, block, edge, agent traversal, LLM utilities) with JSON-friendly output for CI and scripting.
- **Consistent SDKs** – Rust (`ucp-api`), Python (`ucp-content` via PyO3), and JavaScript/WASM (wasm-bindgen) all share the same core engine so workflows stay portable.
- **LLM-ready toolkit** – `ucp-llm` ships IdMapper + PromptBuilder utilities, along with CLI helpers to shorten/expand UCL for token efficiency.

## Ecosystem at a Glance

| Component | Description |
| --- | --- |
| `ucm-core` | Core data model (Block, Document, Content, Edge, Metadata, IDs). |
| `ucm-engine` | Transformation engine offering edit operators, transactions, snapshots, validation. |
| `ucl-parser` | Lexer/parser/AST for the Unified Content Language (UCL). |
| `ucp-api` | High-level Rust client that re-exports all core capabilities. |
| `ucp-cli` | Command-line interface covering documents, blocks, edges, navigation, agents, import/export, and LLM tooling. |
| `ucp-llm` | IdMapper + PromptBuilder for token-efficient prompts and UCL scaffolding. |
| `ucp-content` (Python) | PyO3 bindings offering the full Document + Engine + Agent stack. |
| `@ucp-core/core` (JS/WASM) | wasm-bindgen bindings for browsers, Node, and edge runtimes. |
| Translators | Markdown & HTML ingestion pipelines with semantic role mapping. |
| `ucp-observe` | Tracing, audit logging, and metrics helpers for production deployments. |

## Repository Layout

```
.
├── crates/
│   ├── ucm-core/            # Core types (Block, Content, Document, Edge, Metadata)
│   ├── ucm-engine/          # Transformation engine, transactions, snapshots, validation
│   ├── ucl-parser/          # Lexer, parser, and AST for Unified Content Language
│   ├── ucp-api/             # High-level API surface bundling core crates
│   ├── ucp-cli/             # Command-line interface + integration tests
│   ├── ucp-llm/             # IdMapper, PromptBuilder, presets for LLM prompts
│   ├── ucp-observe/         # Tracing, audit logging, metrics helpers
│   └── translators/         # Markdown ⇄ UCP, HTML → UCP
├── packages/                # Python + JS/WASM SDKs (PyO3 & wasm-bindgen bindings)
├── docs/                    # Full documentation site
└── ...                      # Tooling, workspace config, CI, etc.
```

## Getting Started

1. **Install Rust** (1.70+) and clone the repository:
   ```bash
   git clone https://github.com/<org>/unified-content-protocol.git
   cd unified-content-protocol
   ```
2. **Build & test** the entire workspace:
   ```bash
   cargo build --workspace
   cargo test --workspace
   ```
3. **Explore the CLI** (JSON output is perfect for automation):
   ```bash
   cargo run -p ucp-cli -- --help
   cargo run -p ucp-cli -- create --title "CLI Demo" --format json
   ```
4. **Add the Rust API to another project**:
   ```toml
   [dependencies]
   ucp-api = "0.1.13"
   ```

### SDK Installation Matrix

| Target | Command |
| --- | --- |
| Rust | `ucp-api = "0.1.13"` (or depend on individual crates at the same version). |
| Python | `pip install ucp-content==0.1.13` |
| JavaScript / TypeScript | `npm install @ucp-core/core@0.1.13` |

## Documentation & CLI Guide

The full docs live in [`/docs`](./docs/index.md). Highlights include Getting Started sequences, crate deep dives, translator walkthroughs, agent architecture notes, and an end-to-end CLI usage guide. Every CLI subcommand mirrors the Rust `Commands` enum, so the documentation always stays in sync with the source.

## Example: Deterministic Edits with IdMapper

```rust
use anyhow::Result;
use ucp_api::UcpClient;
use ucp_llm::IdMapper;

fn main() -> Result<()> {
    // 1. Build a document using the Rust client
    let client = UcpClient::new();
    let mut doc = client.create_document();
    let root = doc.root.clone();

    client.add_text(&mut doc, &root, "Product Spec", Some("title"))?;
    client.add_text(&mut doc, &root, "CLI-based editing demo", Some("intro"))?;

    // 2. Execute UCL to append a code sample
    client.execute_ucl(&mut doc, r#"
        APPEND blk_root code WITH label="example" :: "fn add(a: i32, b: i32) -> i32 { a + b }"
    "#)?;

    // 3. Generate a token-efficient mapper for LLM prompts
    let mapper = IdMapper::from_document(&doc);
    let prompt = mapper.document_to_prompt(&doc);
    println!("Prompt snippet:\n{}", prompt);

    // 4. Shorten and expand UCL using the mapper
    let long_ucl = "EDIT blk_root SET metadata.tags += [\"release\"]";
    let short_ucl = mapper.shorten_ucl(long_ucl);
    assert_eq!(short_ucl, "EDIT 1 SET metadata.tags += [\"release\"]");
    let expanded = mapper.expand_ucl(&short_ucl);
    assert_eq!(expanded, long_ucl);

    Ok(())
}
```

This pattern mirrors the CLI’s `llm` subcommands (`id-map`, `shorten-ucl`, `expand-ucl`), so shell scripts and SDKs can share the exact same workflow.

## Contributing

1. Fork the repository, create a feature branch, and keep commits scoped.
2. Run `cargo fmt`, `cargo clippy --all-targets`, `cargo test --workspace`, plus any relevant `npm`/`pytest` suites when touching SDKs.
3. Update docs or examples when adding new capabilities—especially CLI and SDK surfaces.

## License

This project follows the license declared in the workspace `Cargo.toml`. See [`LICENSE`](./LICENSE) for details.
