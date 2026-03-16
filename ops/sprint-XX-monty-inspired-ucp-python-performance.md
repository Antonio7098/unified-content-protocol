# Sprint: Monty-Inspired ucp-python Query Runtime Performance

## What changed after reading the source

The original draft assumed `ucp-python` was spawning a fresh subprocess per query. That is not how the current implementation works.

Today, `ucp.run_python_query(...)` executes in-process inside CPython from `crates/ucp-python/python/ucp/query.py`. The real hot-path costs are:

1. normalizing and compiling the query text on every call
2. installing `sys.settrace(...)` even when the active limits do not need line-by-line tracing

That shifts the Monty-inspired work from "persistent worker processes" to "prepare once, execute many times, and only pay for guardrails when enabled."

## Monty references that actually map to this codebase

| Monty concept | Source | How it maps here |
|---|---|---|
| Reusable prepared runner | `crates/monty/src/run.rs` | mirror the `MontyRun::new()` / reuse pattern with a prepared Python query object |
| Pay-for-what-you-use resource tracking | `crates/monty/src/resource.rs` | avoid enabling expensive tracing unless `max_seconds` or `max_trace_events` require it |
| Resumable execution snapshots | `crates/monty/src/run_progress.rs` | useful future direction, but not a fit for the current CPython `exec`/`eval` runner |
| Versioned snapshot serialization | `crates/monty-python/src/serialization.rs` | useful future direction for persisted prepared queries or session artifacts, not required for this sprint |
| External callback registry | `crates/monty-python/src/external.rs` | useful future direction if we move beyond the current prebound `graph`/`session` model |

## Implemented in this sprint

### 1. Prepared query reuse

Added a new public API:

- `ucp.prepare_python_query(code) -> ucp.PreparedQuery`
- `ucp.PreparedQuery.run(...)`

This gives the query runner a Monty-style "prepare once, run many times" path. Prepared queries hold the normalized source plus the compiled Python code object.

`ucp.run_python_query(...)` also now accepts a `PreparedQuery` directly.

### 2. Automatic compile caching

Added an internal LRU cache keyed by normalized query source. Repeated calls to `ucp.run_python_query(...)` with the same snippet now reuse the compiled object automatically even if the caller does not hold onto a `PreparedQuery`.

This is the main performance win for benchmark suites and agent loops that issue the same query shape with different bindings.

### 3. Selective tracing

The old runner always installed `sys.settrace(...)`, which is disproportionately expensive for short, repeated queries.

The runner now enables tracing only when it is needed to enforce:

- `max_seconds`
- `max_trace_events`

Queries that only use:

- `max_operations`
- `max_stdout_chars`

stay on a cheaper path and still enforce those limits correctly.

## What we explicitly did not build

These items from the first draft are not justified by the current architecture and are deferred:

- subprocess workers
- process pools
- IPC protocols
- graph reload messages
- host callback pause/resume over stdio

Those would only make sense if the runtime moved out of process. The current implementation already has a persistent Python process: the caller's process.

## Verification

Validated with:

- targeted query-runtime tests in `crates/ucp-python/tests/test_query_api.py`
- targeted limit/tooling tests in `crates/ucp-python/tests/test_query_tools.py`
- broader `crates/ucp-python/tests` pass
- benchmark smoke run via `scripts/demo_codegraph_query_benchmarks.py`

New regression coverage includes:

- prepared query reuse across multiple executions
- compile-cache reuse for identical snippets
- proof that the no-trace fast path is used when only operation limits are active

## Follow-up options

If we want to keep pushing Monty ideas into this area, the next credible steps are:

1. add explicit cache controls and cache stats for prepared queries
2. benchmark prepared-query reuse versus ad hoc query calls in CI smoke scripts
3. explore persisted prepared-query metadata or session artifacts with versioned headers
4. revisit external callback registries only if the query runtime needs host-provided side effects beyond the current bound objects
