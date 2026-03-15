# UCP Python bindings

Python bindings for the Rust UCP implementation, including both the generic UCP graph runtime and the specialized CodeGraph API.

For agent workflows, prefer the thin Python façade exposed by `ucp.query(...)` and `ucp.run_python_query(...)`.

For model/runtime integration, also see:

- `ucp.QueryLimits(...)`
- `ucp.PythonQueryTool(...)`
- `ucp.QueryBenchmarkCase(...)` and `ucp.run_query_benchmark_suite(...)`

## Installation

```bash
pip install ucp-content
```

## Document usage

```python
import ucp

doc = ucp.create("My Document")
root = doc.root_id
block = doc.add_block(root, "Hello, World!", role="paragraph")
doc.edit_block(block, "Updated content")
print(ucp.render(doc))
```

## CodeGraph usage

```python
import ucp

graph = ucp.CodeGraph.build("./repo")
session = graph.session()

session.seed_overview(max_depth=3)
session.expand("src/lib.rs", mode="file")

for node in graph.find_nodes(node_class="symbol", name_regex="Session|Context"):
    session.focus(node["logical_key"])
    session.apply_recommended(top=1, padding=2)

exported = session.export(compact=True, max_frontier_actions=6)
print(exported["summary"])
print(session.mutation_log()[-1]["operation"])
```

## Agent-facing query façade

```python
import ucp

graph = ucp.query(ucp.CodeGraph.build("./repo"))
session = graph.session()

for node in graph.find(node_class="symbol", name_regex="auth|login", limit=5):
    branch = session.fork()
    branch.add(node, detail="summary")
    branch.walk(node, mode="dependencies", depth=1, limit=8)
    if any("test" in (item.get("path") or "") for item in branch.export(compact=True)["nodes"]):
        session.add(node, detail="summary")
        session.walk(node, mode="dependencies", depth=1, limit=8)
        break
```

Core façade methods:

- `graph.find(...)`, `graph.describe(...)`, `graph.explain_selector(...)`, `graph.path(...)`
- `session.add(...)`, `session.walk(...)`, `session.focus(...)`, `session.why(...)`
- `session.export(...)`, `session.explain_export_omission(...)`, `session.why_pruned(...)`
- `session.recommendations(...)`, `session.estimate_expand(...)`, `session.estimate_hydrate(...)`
- `session.mutation_log()`, `session.event_log()`
- `session.fork()`, `session.diff(...)`
- `session.hydrate(...)` for CodeGraph

## Python query runner

```python
run = ucp.run_python_query(
    graph,
    """
candidates = graph.find(node_class="symbol", name_regex="auth|login", limit=5)
for node in candidates:
    session.add(node, detail="summary")
    session.walk(node, mode="dependencies", depth=1)
result = session.export(compact=True)
""",
    include_export=True,
)

print(run.ok)
print(run.summary)
```

The runner prebinds `graph`, `session`, `re`, `json`, `math`, and `collections` so the caller can use loops, conditionals, regex, and branching without writing a graph DSL.

It also accepts `bindings={...}` for parameterized queries and automatically dedents normal triple-quoted snippets before execution.

For CodeGraph, exported nodes also surface convenient top-level fields like `logical_key`, `path`, and `symbol_name`, so Python scoring/filtering code does not have to dig through nested `coderef` objects.

### Guarded execution

```python
run = ucp.run_python_query(
    graph,
    "result = graph.find(node_class='symbol', name_regex='auth', limit=5)",
    limits=ucp.QueryLimits(max_seconds=2.0, max_operations=40, max_trace_events=2000),
)
```

`QueryLimits` keeps model-authored queries bounded by wall-clock time, graph/session operations, traced Python events, and stdout size.

### Provider-facing tool wrapper

```python
tool = ucp.PythonQueryTool(
    graph,
    default_include_export=True,
    default_limits=ucp.QueryLimits(max_seconds=2.0, max_operations=40),
)

openai_tool = tool.openai_tool()
result = tool.execute({"code": "result = graph.find(node_class='symbol', name_regex='auth', limit=5)"})
```

The wrapper also supports:

- `tool.execute_openai_tool_call(...)`
- `tool.execute_anthropic_tool_use(...)`

### Benchmark helpers

```python
cases = [
    ucp.QueryBenchmarkCase(
        name="rank-tests",
        description="Rank likely tests for a symbol",
        code="result = graph.find(node_class='symbol', path_regex=r'tests/.*', limit=10)",
    )
]
results = ucp.run_query_benchmark_suite(graph, cases)
summary = ucp.summarize_query_benchmark_suite(results)
```

Example with parameterized regexes:

```python
run = ucp.run_python_query(
    graph,
    """
        hits = graph.find(node_class="symbol", path_regex=path_rx, name_regex=name_rx, limit=6)
        best = next(node for node in hits if "context_show" in node["logical_key"])
        session.add(best, detail="summary")
        result = session.export(compact=True)
    """,
    bindings={
        "path_rx": r"crates/ucp-cli/src/commands/(agent|codegraph)\.rs",
        "name_rx": r"context_show|get_session_mut",
    },
)
```

Example: rank likely tests for a symbol with plain Python heuristics:

```python
target = graph.find(node_class="symbol", path_regex=r"crates/ucp-python/python/ucp/query\.py", name_regex=r"^run_python_query$", limit=1)[0]
tests = graph.find(node_class="symbol", path_regex=r"crates/ucp-python/tests/.*\.py", name_regex=r"test_.*query.*", limit=80)
target_words = set(re.findall(r"[A-Za-z]+", target["logical_key"].lower()))
ranked = []
for node in tests:
    words = set(re.findall(r"[A-Za-z]+", (node.get("logical_key") or "").lower()))
    score = len((target_words & words) - {"symbol", "py", "python"})
    if score:
        ranked.append((score, node["logical_key"]))
print(sorted(ranked, reverse=True)[:5])
```

## Generic graph usage

```python
import ucp

doc = ucp.create("Graph demo")
section = doc.add_block(doc.root_id, "Section", role="section", label="section")
note = doc.add_block(section, "Important note", role="paragraph", label="note")
helper = doc.add_code(section, "rust", "fn helper() {}", label="helper")
doc.add_edge(note, ucp.EdgeType.References, helper)

graph = ucp.Graph.from_document(doc)
sqlite = graph.persist_sqlite("graph.db", "demo")
session = sqlite.session()
session.seed_overview(max_depth=1)
session.expand("note", mode="outgoing", depth=1)
print(session.export())
```

## Generic graph features

- `Graph.from_document(...)`
- `Graph.from_json(...)`, `Graph.load(...)`, `Graph.save(...)`
- `Graph.persist_sqlite(...)`, `Graph.from_sqlite(...)`
- `find_nodes(...)`, `describe(...)`, `path_between(...)`
- `GraphSession` with `seed_overview`, `select`, `focus`, `expand`, `collapse`, `pin`, `prune`, `why_selected`, and `diff`

## CodeGraph features

- `CodeGraph.build(...)` from a repository
- `find_nodes(...)` with regex filters
- `resolve(...)` and `describe(...)`
- `explain_selector(...)` for selector provenance / ambiguity
- `path_between(...)` for short graph explanations
- `CodeGraphSession` for stateful exploration
- `why_selected(...)` provenance/explainability with provenance chains
- `apply_recommended(...)` and structured `recommendations(...)`
- `estimate_expand(...)` / `estimate_hydrate(...)` for budget-aware traversal
- `mutation_log()` / `event_log()` for observability
- `explain_export_omission(...)` and `why_pruned(...)`
- `fork()` and `diff(...)` for branch-and-compare workflows
- `to_json()`, `from_json(...)`, `save(...)`, and `load(...)`
- session `to_json()`, `save(...)`, `graph.load_session(...)`, and `graph.load_session_json(...)`

## General features

- **Document operations**: create, edit, move, delete blocks
- **Traversal**: children, parent, ancestors, descendants, siblings
- **Finding**: by tag, label, role, content type
- **Edges**: create relationships between blocks
- **LLM utilities**: `IdMapper`, `PromptBuilder`, prompt presets
- **Snapshots**: snapshot and rollback helpers
- **UCL execution**: execute UCL commands on documents

## Related docs

- `docs/ucp-api/codegraph-programmatic.md`
- `docs/ucp-api/python-query-tools.md`
- `docs/ucp-api/graph-runtime.md`
- `docs/ucp-cli/codegraph.md`
- `scripts/demo_ucp_python_query.py`
- `scripts/demo_codegraph_python_query.py`
- `scripts/demo_codegraph_query_tool_wrapper.py`
- `scripts/demo_codegraph_query_benchmarks.py`
- `scripts/demo_codegraph_query_recipes.py`
- `scripts/demo_codegraph_query_edge_cases.py`
- `scripts/demo_codegraph_context_walk.py`
- `scripts/demo_codegraph_session_observability.py`
