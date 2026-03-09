# UCP API

`ucp-api` is the high-level Rust entry point for document operations, generic graph navigation, UCL execution, and codebase-to-UCM CodeGraph extraction.

## Installation

```toml
[dependencies]
ucp-api = "0.1.14"
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

## Generic graph runtime

For graph traversal that should work across ordinary UCP documents, use `GraphNavigator` and `GraphSession`.

```rust
use ucp_api::{GraphFindQuery, GraphNavigator, GraphNeighborMode};

let graph = GraphNavigator::from_document(doc);
let matches = graph.find_nodes(&GraphFindQuery {
    label_regex: Some("intro|section".into()),
    ..Default::default()
})?;

let mut session = graph.session();
session.seed_overview(Some(2));
session.expand("root", GraphNeighborMode::Children, 2, Some(16))?;
```

Storage options:

- in-memory / JSON via `from_document`, `from_json`, `load`, `to_json`, `save`
- SQLite via `persist_sqlite(...)` and `GraphNavigator::open_sqlite(...)`

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

### Programmatic navigation for agents

For stateful graph navigation, use `CodeGraphNavigator` and `CodeGraphNavigatorSession`.

```rust
use ucp_api::{CodeGraphBuildInput, CodeGraphExpandMode, CodeGraphNavigator};

let graph = CodeGraphNavigator::build(&CodeGraphBuildInput {
    repository_path: "./my-repo".into(),
    commit_hash: "manual-test".into(),
    config: Default::default(),
})?;

let mut session = graph.session();
session.seed_overview(Some(3));
session.expand("src/lib.rs", CodeGraphExpandMode::File, &Default::default())?;
let why = session.why_selected("symbol:src/lib.rs::add")?;
println!("{}", why.explanation);
```

Useful programmatic helpers include:

- regex-driven `find_nodes(...)`
- `path_between(...)`
- `why_selected(...)`
- `apply_recommended_actions(...)`
- `fork()` and `diff(...)`

### Incremental rebuilds

`ucp-api` also exposes `build_code_graph_incremental(...)` plus `CodeGraphIncrementalBuildInput` and `CodeGraphIncrementalStats` for persisted per-file rebuilds with reuse, fallback reasons, and invalidation metrics.

## CodeGraph Contract Fields

The generated `Document.metadata` includes contract data expected by downstream consumers:
- `profile` = `"codegraph"`
- `profile_version` = `"v1"`
- `profile_marker` = `"codegraph.v1"`
- `canonical_fingerprint` = stable canonical hash of the graph artifact
- extractor version marker via `CODEGRAPH_EXTRACTOR_VERSION`

## Related Docs

- `docs/ucp-api/graph-runtime.md` for the generic UCP graph runtime
- `docs/ucp-api/python-query-tools.md` for the agent-facing Python query façade and query runner
- `docs/ucp-cli/README.md` for `ucp codegraph` commands
- `docs/ucp-cli/codegraph.md` for detailed CodeGraph coverage, selectors, context sessions, incremental rebuilds, and benchmark examples
- `docs/ucp-api/codegraph-programmatic.md` for the programmatic Rust + Python navigation surface
- `docs/ucp-llm/README.md` for id mapping + prompt builder flow
