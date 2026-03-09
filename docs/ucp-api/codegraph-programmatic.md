# Programmatic CodeGraph API

CodeGraph now exposes both:

- a low-level programmatic navigation surface for Rust/Python callers
- an agent-facing Python façade for minimal, loop-friendly querying

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

The Python bindings export low-level `ucp.CodeGraph` / `ucp.CodeGraphSession`, plus the higher-level `ucp.query(...)` façade and `ucp.run_python_query(...)` helper.

Example:

```python
import ucp

graph = ucp.query(ucp.CodeGraph.build("./repo"))
session = graph.session()

for node in graph.find(node_class="symbol", name_regex="Session|Context"):
    session.add(node, detail="summary")
    session.walk(node, mode="dependencies", depth=1)
```

## Core operations

### Low-level graph/session surface

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

### Agent-facing façade

Wrap a raw graph with `ucp.query(...)` and use the thinner names:

- `graph.find(...)`
- `graph.describe(...)`
- `graph.path(...)`
- `graph.session()`
- `session.add(...)`
- `session.walk(...)`
- `session.focus(...)`
- `session.why(...)`
- `session.export(...)`
- `session.fork()` / `session.diff(...)`
- `session.hydrate(...)` on CodeGraph

Use `ucp.run_python_query(...)` when you want the model to mix those primitives with Python loops, regex, and branching logic.

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
- `docs/ucp-api/python-query-tools.md`
- `crates/ucp-python/README.md`
- `scripts/demo_codegraph_context_walk.py`