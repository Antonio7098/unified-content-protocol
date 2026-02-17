# UCP API

`ucp-api` is the high-level Rust entry point for document operations, UCL execution, and codebase-to-UCM CodeGraph extraction.

## Installation

```toml
[dependencies]
ucp-api = "0.1.13"
```

## Core Client (`UcpClient`)

```rust
use ucp_api::UcpClient;

fn main() -> anyhow::Result<()> {
    let client = UcpClient::new();
    let mut doc = client.create_document();
    let root = doc.root;

    client.add_text(&mut doc, &root, "Hello, UCP", Some("intro"))?;

    client.execute_ucl(&mut doc, r#"
        APPEND blk_root text WITH label="next" :: "More content"
    "#)?;

    println!("{}", client.to_json(&doc)?);
    Ok(())
}
```

## CodeGraph API (Tree-sitter)

CodeGraph extraction uses Tree-sitter parsers for:
- Rust (`tree-sitter-rust`)
- Python (`tree-sitter-python`)
- TypeScript/TSX (`tree-sitter-typescript`)
- JavaScript/JSX (`tree-sitter-javascript`)

### Build a CodeGraph

```rust
use std::path::PathBuf;
use ucp_api::{
    build_code_graph, canonical_fingerprint, validate_code_graph_profile,
    CodeGraphBuildInput, CodeGraphExtractorConfig,
};

fn main() -> anyhow::Result<()> {
    let mut cfg = CodeGraphExtractorConfig::default();
    cfg.include_extensions = vec!["rs".into(), "py".into(), "ts".into(), "js".into()];
    cfg.continue_on_parse_error = true;
    cfg.emit_export_edges = true;

    let result = build_code_graph(&CodeGraphBuildInput {
        repository_path: PathBuf::from("./my-repo"),
        commit_hash: "manual-test".to_string(),
        config: cfg,
    })?;

    println!("status={:?}", result.status);
    println!("profile_version={}", result.profile_version);
    println!("canonical_fingerprint={}", result.canonical_fingerprint);
    println!("file_nodes={}", result.stats.file_nodes);
    println!("symbol_nodes={}", result.stats.symbol_nodes);

    let validation = validate_code_graph_profile(&result.document);
    assert!(validation.valid, "graph must satisfy CodeGraphProfile v1");

    let fp = canonical_fingerprint(&result.document)?;
    assert_eq!(fp, result.canonical_fingerprint);
    Ok(())
}
```

### Prompt Projection for LLM Context

```rust
use ucp_api::codegraph_prompt_projection;

# fn demo(doc: &ucm_core::Document) {
let projection = codegraph_prompt_projection(doc);
println!("{}", projection);
# }
```

Use this projection as the codebase summary input for prompt assembly and constitution checks.

## CodeGraph Contract Fields

The generated `Document.metadata` includes contract data expected by downstream consumers:
- `profile` = `"codegraph"`
- `profile_version` = `"v1"`
- `profile_marker` = `"codegraph.v1"`
- `canonical_fingerprint` = stable canonical hash of the graph artifact
- extractor version marker via `CODEGRAPH_EXTRACTOR_VERSION`

## Related Docs

- `docs/ucp-cli/README.md` for `ucp codegraph` commands
- `docs/ucp-llm/README.md` for id mapping + prompt builder flow
