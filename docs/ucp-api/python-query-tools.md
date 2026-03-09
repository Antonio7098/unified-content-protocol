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

For CodeGraph sessions, exported nodes include convenient top-level fields like `logical_key`, `path`, and `symbol_name`, which makes lightweight Python ranking/filtering much easier.

## Typical agent patterns

This style is especially good for agents that need to build context incrementally rather than request one giant dump.

Common use cases include:

- tracing an entrypoint to an implementation detail
- ranking likely tests for a changed symbol
- comparing multiple candidate entrypoints before hydrating source
- finding mirrored implementations and diffing their neighborhoods
- gathering the smallest working set that still explains a bug or feature

### Regex discovery + walk

Use `find(...)` to gather candidates, then loop in Python and call `session.walk(...)` selectively.

### Branch and compare

Fork the current session, explore multiple hypotheses, and use `diff(...)` to compare what each branch added.

### Path as explanation

Use `graph.path(...)` or `session.path(...)` to connect two nodes without expanding a wide neighborhood.

### Hydrate only after ranking

On CodeGraph, delay `session.hydrate(...)` until Python has already ranked the interesting symbols.

### Rank tests with lightweight Python heuristics

An agent does not need a special backend primitive for “find the most relevant tests”. It can combine regex search with a small scoring function:

```python
target = graph.find(node_class="symbol", path_regex=target_rx, name_regex=r"^run_python_query$", limit=1)[0]
tests = graph.find(node_class="symbol", path_regex=r"crates/ucp-python/tests/.*\.py", name_regex=r"test_.*query.*", limit=80)
target_words = set(re.findall(r"[A-Za-z]+", target["logical_key"].lower()))
ranked = []
for node in tests:
    words = set(re.findall(r"[A-Za-z]+", (node.get("logical_key") or "").lower()))
    score = len((target_words & words) - {"symbol", "py", "python"})
    if "query_api" in (node.get("path") or ""):
        score += 2
    if score:
        ranked.append((score, node["logical_key"]))
best = sorted(ranked, reverse=True)[:5]
```

This keeps the backend minimal while still enabling highly targeted evidence gathering.

## Concrete UCP-repo recipe ideas

### Compare mirrored CLI handlers

Use `find(...)` to locate both `agent.rs::context_show` and `codegraph.rs::context_show`, then fork a branch for each and compare dependency neighborhoods.

### Explain a command-to-render path

Use `graph.path(...)` to connect a CLI entrypoint like `context_show` to symbols such as `make_export_config` or `export_codegraph_context_with_config`.

### Rank symbols by local evidence

Start from regex hits like `session|context|render|export`, expand each candidate one hop in a branch, and score by selected-node count, visible edges, and frontier richness.

### Find public wrappers before hydrating source

Use `branch.walk(target, mode="dependents", depth=1)` and rank the exported nodes by top-level `path` / `symbol_name` to find small public wrappers around a deeper helper before spending budget on source hydration.

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