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
- `CodeGraphOperationBudget`
- `CodeGraphSelectionExplanation`
- `CodeGraphSelectorResolutionExplanation`
- `CodeGraphMutationEstimate`
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

The Python bindings export low-level `ucp.CodeGraph` / `ucp.CodeGraphSession`, plus the higher-level `ucp.query(...)`, `ucp.run_python_query(...)`, and `ucp.prepare_python_query(...)` helpers.

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
- `explain_selector(...)`
- `explain_export_omission(...)`
- `why_pruned(...)`
- `apply_recommended(...)`
- `recommendations(...)`
- `estimate_expand(...)` / `estimate_hydrate(...)`
- `mutation_log()` / `event_log()`
- `to_json()` / `save(...)`
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
- `session.explain_export_omission(...)`
- `session.why_pruned(...)`
- `session.export(...)`
- `session.fork()` / `session.diff(...)`
- `session.hydrate(...)` on CodeGraph
- `session.recommendations(...)`
- `session.estimate_expand(...)` / `session.estimate_hydrate(...)`

## Session observability

Every session mutation now emits a typed telemetry record on the returned update and into the session log.

Recorded fields include:

- operation kind
- selector and resolved block ids
- traversal and budget parameters
- nodes added, removed, and changed
- focus before/after
- elapsed time
- mutation reason

Example:

```rust
use ucp_api::{CodeGraphExpandMode, CodeGraphOperationBudget, CodeGraphTraversalConfig};

let update = session.expand(
    "src/lib.rs",
    CodeGraphExpandMode::File,
    &CodeGraphTraversalConfig {
        budget: Some(CodeGraphOperationBudget {
            max_nodes_visited: Some(16),
            max_emitted_telemetry_events: Some(4),
            ..Default::default()
        }),
        ..Default::default()
    },
)?;

assert!(!update.telemetry.is_empty());
assert!(!session.mutation_log().is_empty());
```

## Persistence and replayability

Sessions are now stable persisted artifacts with:

- schema versioning
- session identity
- graph snapshot hash
- session snapshot hash
- structured mutation history

```rust
let payload = session.to_json()?;
let restored = graph.load_session_json(&payload)?;
assert_eq!(restored.selected_block_ids(), session.selected_block_ids());
```

## Recommendations and negative explanations

The richer recommendation/explanation APIs are designed for auditability and benchmark evaluation:

- `session.recommendations(top)` returns rationale, relation sets, evidence gain, and estimated token/hydration cost.
- `session.explain_export_omission(...)` explains hidden nodes caused by visible-level limits, class filters, or render budgets.
- `session.why_pruned(...)` explains the latest recorded prune outcome for a selector.
- `graph.explain_selector(...)` explains how selector resolution behaved, including ambiguity.

Use `ucp.run_python_query(...)` when you want the model to mix those primitives with Python loops, regex, and branching logic.

Use `ucp.prepare_python_query(...)` when the same snippet will run repeatedly across different bindings, sessions, or graphs and you want to reuse the compiled query object.

For richer cookbook-style examples on the UCP repo itself, see `docs/ucp-api/python-query-tools.md` plus the recipe/edge-case smoke scripts under `scripts/`.

For provider/tool integration, guarded execution, prepared-query reuse, and benchmark-style workflow evaluation, also see the `PythonQueryTool`, `PreparedQuery`, `QueryLimits`, and `QueryBenchmarkCase` sections in `docs/ucp-api/python-query-tools.md`.

## Agent-oriented patterns

### Regex-driven discovery

Use `find_nodes(...)` to collect a candidate set, then loop in Python or Rust to expand only the most relevant symbols.

### Branch-and-compare investigations

Fork a session, explore two hypotheses independently, then compare them with `diff(...)`.

### Explainability

Use `why_selected(...)` to understand how a node entered the current working set, which anchor introduced it, and the full provenance chain.

### Focused path-finding

Use `path_between(...)` to inspect the shortest discovered chain between two selectors without dumping a large neighborhood.

## Related docs

- `docs/ucp-cli/codegraph.md`
- `docs/ucp-api/README.md`
- `docs/ucp-api/python-query-tools.md`
- `crates/ucp-python/README.md`
- `scripts/demo_codegraph_context_walk.py`
- `scripts/demo_codegraph_session_observability.py`
