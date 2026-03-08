# Programmatic CodeGraph API

CodeGraph now exposes a first-class programmatic navigation surface for agent workflows.

## Design

The programmatic API keeps graph semantics in CodeGraph and leaves control flow to the host language:

- use Python or Rust for loops, conditionals, branching, ranking, and regex orchestration
- use CodeGraph for selection, expansion, hydration, provenance, path-finding, and structured export

This avoids forcing agents to write graph-database queries while still enabling rich automated traversal.

## Rust entry points

`ucp-api` re-exports these types from `ucp-codegraph`:

- `CodeGraphNavigator`
- `CodeGraphNavigatorSession`
- `CodeGraphFindQuery`
- `CodeGraphExpandMode`
- `CodeGraphSelectionExplanation`
- `CodeGraphSessionDiff`

Example:

```rust
use ucp_api::{CodeGraphBuildInput, CodeGraphExpandMode, CodeGraphNavigator};

let graph = CodeGraphNavigator::build(&CodeGraphBuildInput {
    repository_path: "./repo".into(),
    commit_hash: "HEAD".into(),
    config: Default::default(),
})?;

let mut session = graph.session();
session.seed_overview(Some(3));
session.expand("src/lib.rs", CodeGraphExpandMode::File, &Default::default())?;
let why = session.why_selected("symbol:src/lib.rs::add")?;
```

## Python entry points

The Python bindings export `ucp.CodeGraph` and `ucp.CodeGraphSession`.

Example:

```python
import ucp

graph = ucp.CodeGraph.build("./repo")
session = graph.session()
session.seed_overview(max_depth=3)
session.expand("src/lib.rs", mode="file")

for node in graph.find_nodes(node_class="symbol", name_regex="Session|Context"):
    session.focus(node["logical_key"])
    session.apply_recommended(top=1, padding=2)
```

## Core operations

### Graph-level

- `resolve(selector)`
- `describe(selector)`
- `find_nodes(...)`
- `path_between(start, end, max_hops=...)`
- `session()`
- `to_json()`, `from_json(...)`, `save(...)`, `load(...)`

### Session-level

- `seed_overview(...)`
- `focus(...)`
- `select(...)`
- `expand(..., mode="file|dependencies|dependents")`
- `hydrate(...)`
- `collapse(...)`
- `pin(...)`
- `prune(...)`
- `export(...)`
- `render_prompt(...)`
- `why_selected(...)`
- `apply_recommended(...)`
- `fork()` / `diff(...)`

## Agent-oriented patterns

### Regex-driven discovery

Use `find_nodes(...)` to collect a candidate set, then loop in Python or Rust to expand only the most relevant symbols.

### Branch-and-compare investigations

Fork a session, explore two hypotheses independently, then compare them with `diff(...)`.

### Explainability

Use `why_selected(...)` to understand how a node entered the current working set and which anchor introduced it.

### Focused path-finding

Use `path_between(...)` to inspect the shortest discovered chain between two selectors without dumping a large neighborhood.

## Related docs

- `docs/ucp-cli/codegraph.md`
- `docs/ucp-api/README.md`
- `crates/ucp-python/README.md`
- `scripts/demo_codegraph_context_walk.py`