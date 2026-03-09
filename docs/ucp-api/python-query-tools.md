# Agent-facing Python Query Tools

UCP now exposes a thin Python façade for graph/codegraph exploration that keeps the core API small and pushes orchestration into normal Python.

## Design goal

Use a minimal graph surface:

- `find(...)`
- `describe(...)`
- `path(...)`
- `session()`

Then use Python for:

- loops
- conditionals
- regex filtering
- branching and scoring
- stopping criteria

## Entry points

```python
import ucp

raw = ucp.CodeGraph.build("./repo")
graph = ucp.query(raw)
session = graph.session()
```

`ucp.query(...)` wraps either `ucp.Graph` or `ucp.CodeGraph` and exposes agent-friendly names.

## Minimal façade

### Graph façade

- `graph.find(...)`
- `graph.describe(selector)`
- `graph.resolve(selector)`
- `graph.path(start, end, max_hops=8)`
- `graph.session()`
- `graph.run(code, ...)`

### Session façade

- `session.add(target, detail="summary")`
- `session.walk(target, ...)`
- `session.focus(target=None)`
- `session.why(target)`
- `session.export(...)`
- `session.fork()` / `session.diff(other)`
- `session.run(code, ...)`

CodeGraph sessions also expose:

- `session.hydrate(target, padding=2)`

Selectors can be:

- selector strings
- block ids
- node dictionaries returned by `find(...)`

## Python query runner

Use `ucp.run_python_query(...)` to execute short Python snippets against prebound objects:

- `graph`
- `session`
- `raw_graph`
- `raw_session`
- `re`, `json`, `math`, `collections`

Useful optional arguments:

- `bindings={...}` to inject parameters or precomputed regexes into the query
- `include_export=True` to return the final session export alongside the query result
- `export_kwargs={...}` to control that export

Example:

```python
run = ucp.run_python_query(
    graph,
    """
candidates = graph.find(node_class="symbol", name_regex="auth|login", limit=8)
for node in candidates:
    branch = session.fork()
    branch.add(node, detail="summary")
    branch.walk(node, mode="dependencies", depth=1)
    if any("test" in (item.get("path") or "") for item in branch.export(compact=True)["nodes"]):
        session.add(node, detail="summary")
        session.walk(node, mode="dependencies", depth=1)
        break
result = session.export(compact=True)
""",
    include_export=True,
)
```

The result includes:

- `ok`
- `result`
- `stdout`
- `summary`
- `selected_block_ids`
- optional `export`
- structured error details if execution failed

Queries are automatically `textwrap.dedent(...)`-ed before execution, so normal indented triple-quoted snippets work as expected.

## Typical agent patterns

### Regex discovery + walk

Use `find(...)` to gather candidates, then loop in Python and call `session.walk(...)` selectively.

### Branch and compare

Fork the current session, explore multiple hypotheses, and use `diff(...)` to compare what each branch added.

### Path as explanation

Use `graph.path(...)` or `session.path(...)` to connect two nodes without expanding a wide neighborhood.

### Hydrate only after ranking

On CodeGraph, delay `session.hydrate(...)` until Python has already ranked the interesting symbols.

## Concrete UCP-repo recipe ideas

### Compare mirrored CLI handlers

Use `find(...)` to locate both `agent.rs::context_show` and `codegraph.rs::context_show`, then fork a branch for each and compare dependency neighborhoods.

### Explain a command-to-render path

Use `graph.path(...)` to connect a CLI entrypoint like `context_show` to symbols such as `make_export_config` or `export_codegraph_context_with_config`.

### Rank symbols by local evidence

Start from regex hits like `session|context|render|export`, expand each candidate one hop in a branch, and score by selected-node count, visible edges, and frontier richness.

See:

- `scripts/demo_codegraph_query_recipes.py`
- `scripts/demo_codegraph_query_edge_cases.py`

## Safety model

`run_python_query(...)` is intended for trusted local automation.

It runs with a restricted builtin set and prebound helper modules, but it is not a hardened sandbox.

## Related docs

- `docs/ucp-api/codegraph-programmatic.md`
- `docs/ucp-api/graph-runtime.md`
- `crates/ucp-python/README.md`