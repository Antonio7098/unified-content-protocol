# Generic UCP Graph Runtime

`ucp-graph` is the reusable graph runtime for UCP documents.

It provides:

- graph navigation over any `ucm_core::Document`
- JSON-backed in-memory graph storage
- SQLite-backed persistent graph storage
- stateful working-set sessions for traversal workflows
- structured observability about indexes and store shape

## Core idea

UCP documents are already graph-shaped:

- hierarchy via `Document.structure`
- semantic edges via `Block.edges`
- block metadata for labels, roles, and tags

`ucp-graph` turns that into a reusable navigation layer that is not specific to CodeGraph.

## Rust API

```rust
use ucp_api::{GraphFindQuery, GraphNavigator, GraphNeighborMode};

let graph = GraphNavigator::from_document(doc);
let matches = graph.find_nodes(&GraphFindQuery {
    label_regex: Some("section|helper".into()),
    ..Default::default()
})?;

let mut session = graph.session();
session.seed_overview(Some(2));
session.expand("section", GraphNeighborMode::Children, 2, Some(16))?;
```

## Storage backends

### In-memory JSON-backed

Use when you want:

- fast local scripting
- tests
- transient agent workflows
- portable graph artifacts

Key methods:

- `GraphNavigator::from_document(...)`
- `GraphNavigator::from_json(...)`
- `GraphNavigator::load(...)`
- `to_json()` / `save(...)`

### SQLite-backed

Use when you want:

- persistent graph indexes
- repeated graph queries without rebuilding the document payload each time
- local durable session and graph artifacts

Key methods:

- `persist_sqlite(path, graph_key)`
- `GraphNavigator::open_sqlite(path, graph_key)`

## Session operations

Sessions support reusable graph workflows:

- `seed_overview(...)`
- `select(...)`
- `focus(...)`
- `expand(...)`
- `collapse(...)`
- `pin(...)`
- `prune(...)`
- `why_selected(...)`
- `fork()` / `diff(...)`
- `export()`

## Observability

Call `store_stats()` and `observability()` to inspect:

- backend kind
- node / edge counts
- structural edge counts
- graph key for persisted stores
- indexed fields exposed by the current backend

## Relationship to CodeGraph

CodeGraph remains the code-specific extraction and semantics layer.

Use:

- `ucp-graph` for generic graph traversal and persistence
- `ucp-codegraph` for repository extraction, code selectors, source hydration, and code-aware frontier logic

## Related docs

- `docs/ucp-api/README.md`
- `docs/ucp-api/codegraph-programmatic.md`
- `docs/ucp-cli/codegraph.md`
- `crates/ucp-python/README.md`