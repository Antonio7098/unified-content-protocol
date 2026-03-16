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

Additional integration helpers:

- `ucp.prepare_python_query(...)` / `ucp.PreparedQuery.run(...)` for compile-once, run-many workflows
- `ucp.QueryLimits(...)` for guarded execution
- `ucp.PythonQueryTool(...)` for provider-facing tool definitions and execution
- `ucp.QueryBenchmarkCase(...)` / `ucp.run_query_benchmark_suite(...)` for workflow evaluation

## Minimal façade

### Graph façade

- `graph.find(...)`
- `graph.describe(selector)`
- `graph.explain_selector(selector)`
- `graph.resolve(selector)`
- `graph.path(start, end, max_hops=8)`
- `graph.session()`
- `graph.run(code, ...)`

### Session façade

- `session.add(target, detail="summary")`
- `session.walk(target, ...)`
- `session.focus(target=None)`
- `session.why(target)`
- `session.explain_export_omission(target, ...)`
- `session.why_pruned(target)`
- `session.export(...)`
- `session.fork()` / `session.diff(other)`
- `session.mutation_log()`
- `session.event_log()`
- `session.run(code, ...)`

CodeGraph sessions also expose:

- `session.hydrate(target, padding=2)`
- `session.recommendations(top=3)`
- `session.estimate_expand(...)`
- `session.estimate_hydrate(...)`

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
- `limits=ucp.QueryLimits(...)` to bound model-authored queries

`run_python_query(...)` automatically dedents and LRU-caches compiled snippets, so repeated calls with identical code reuse the compiled Python object rather than paying `compile(...)` each time.

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

### Prepared queries and warm execution

For benchmark suites, agent loops, or any other repeated workflow, compile once and reuse the prepared object:

```python
prepared = ucp.prepare_python_query(
    """
    hits = graph.find(node_class="symbol", name_regex=name_rx, limit=8)
    result = [node["logical_key"] for node in hits]
    """
)

first = prepared.run(graph, bindings={"name_rx": r"auth|login"})
second = prepared.run(graph, bindings={"name_rx": r"session|context"})
```

This is the closest analogue to Monty's reusable `MontyRun`: the expensive parsing/compilation step is separated from execution, but queries still run inside normal CPython with the UCP graph/session objects bound into the environment.

For CodeGraph sessions, exported nodes include convenient top-level fields like `logical_key`, `path`, and `symbol_name`, which makes lightweight Python ranking/filtering much easier.

## Session observability and persistence

CodeGraph sessions now expose first-class observability primitives that are useful for agents, benchmarks, and UI tooling:

- mutation telemetry on every update (`update["telemetry"]`)
- cumulative `session.mutation_log()` and `session.event_log()`
- selector resolution explanations via `graph.explain_selector(...)`
- omission explanations via `session.explain_export_omission(...)`
- prune explanations via `session.why_pruned(...)`
- replayable session persistence via `session.save(...)`, `session.to_json()`, `graph.load_session(...)`, and `graph.load_session_json(...)`

Example:

```python
session = ucp.CodeGraph.build("./repo").session()
session.seed_overview(max_depth=3)
update = session.expand("src/lib.rs", mode="file", max_nodes_visited=16)
print(update["telemetry"][0]["operation"])
print(session.mutation_log()[-1]["elapsed_ms"])
```

Recommendations are now structured objects rather than bare action strings, so agents can rank them by evidence gain or estimated hydration/token cost before applying them.

## Guarded execution

Use `QueryLimits` to keep model-authored queries bounded:

- `max_seconds`
- `max_operations`
- `max_trace_events`
- `max_stdout_chars`

Example:

```python
run = ucp.run_python_query(
    graph,
    "result = graph.find(node_class='symbol', name_regex='auth', limit=5)",
    limits=ucp.QueryLimits(max_seconds=2.0, max_operations=40, max_trace_events=2000),
)
```

These guards are designed for agent workflows: they keep queries short and cheap without adding a separate graph DSL.

Tracing is only enabled when it is actually needed for `max_seconds` or `max_trace_events`. If a workflow only sets `max_operations` and/or `max_stdout_chars`, the runner stays on the faster no-trace path and counts those limits directly from graph/session/stdout activity.

## Provider-facing tool wrapper

`PythonQueryTool` binds a graph/session once and exposes provider-friendly tool definitions plus execution helpers.

```python
tool = ucp.PythonQueryTool(
    graph,
    default_include_export=True,
    default_limits=ucp.QueryLimits(max_seconds=2.0, max_operations=40),
)

openai_tool = tool.openai_tool()
anthropic_tool = tool.anthropic_tool()
result = tool.execute({"code": "result = graph.find(node_class='symbol', name_regex='auth', limit=5)"})
```

It also provides:

- `execute_openai_tool_call(...)`
- `execute_anthropic_tool_use(...)`

See:

- `scripts/demo_codegraph_query_tool_wrapper.py`

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

## Benchmark / evaluation helpers

Use `QueryBenchmarkCase` plus `run_query_benchmark_suite(...)` to encode repeatable agent workflows and track whether the API remains expressive and cheap enough over time.

Typical benchmark cases include:

- trace a public entrypoint to its implementation path
- rank likely tests for a target symbol
- compare mirrored handlers or implementations
- rank candidates before hydrating source

See:

- `scripts/demo_codegraph_query_benchmarks.py`
- `artifacts/codegraph-query-benchmarks-transcript.md`

See:

- `scripts/demo_codegraph_query_recipes.py`
- `scripts/demo_codegraph_query_edge_cases.py`
- `scripts/demo_codegraph_session_observability.py`

## Safety model

`run_python_query(...)` is intended for trusted local automation.

It runs with a restricted builtin set and prebound helper modules, but it is not a hardened sandbox.

## Related docs

- `docs/ucp-api/codegraph-programmatic.md`
- `docs/ucp-api/graph-runtime.md`
- `crates/ucp-python/README.md`
