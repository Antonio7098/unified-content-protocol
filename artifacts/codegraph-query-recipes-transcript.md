## CodeGraph query recipes on the UCP repo

This transcript captures a few higher-level recipe patterns over the UCP codebase graph: branch-and-compare, explanation paths, and lightweight ranking via Python control flow.

## CodeGraph summary

```json
{
  "node_count": 5621,
  "repr": "CodeGraph(nodes=5621)"
}
```

## Compare mirrored context_show handlers

```json
{
  "error": null,
  "export": null,
  "limits": {
    "max_operations": null,
    "max_seconds": null,
    "max_stdout_chars": null,
    "max_trace_events": null
  },
  "ok": true,
  "result": [
    {
      "edges": 2,
      "frontier": [
        "hydrate_source",
        "expand_dependents",
        "collapse"
      ],
      "selected": 3,
      "target": "symbol:crates/ucp-cli/src/commands/agent.rs::context_show"
    },
    {
      "edges": 4,
      "frontier": [
        "hydrate_source",
        "expand_dependents",
        "collapse"
      ],
      "selected": 5,
      "target": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show"
    }
  ],
  "selected_block_ids": [],
  "stdout": "",
  "summary": {
    "directories": 0,
    "files": 0,
    "hydrated_sources": 0,
    "max_selected": 48,
    "repositories": 0,
    "selected": 0,
    "symbols": 0
  },
  "usage": {
    "elapsed_seconds": 1.612093,
    "operation_count": 11,
    "stdout_chars": 0,
    "trace_events": 457
  }
}
```

## Rank the most relevant tests for run_python_query

```json
{
  "error": null,
  "export": null,
  "limits": {
    "max_operations": null,
    "max_seconds": null,
    "max_stdout_chars": null,
    "max_trace_events": null
  },
  "ok": true,
  "result": {
    "ranked": [
      {
        "path": "crates/ucp-python/tests/test_query_api.py",
        "score": 6,
        "test": "symbol:crates/ucp-python/tests/test_query_api.py::test_run_python_query_accepts_raw_graph_and_raw_session"
      },
      {
        "path": "crates/ucp-python/tests/test_query_api.py",
        "score": 6,
        "test": "symbol:crates/ucp-python/tests/test_query_api.py::test_run_python_query_dedents_common_triple_quoted_snippets_and_exposes_common_builtins"
      },
      {
        "path": "crates/ucp-python/tests/test_query_api.py",
        "score": 6,
        "test": "symbol:crates/ucp-python/tests/test_query_api.py::test_run_python_query_executes_python_control_flow_and_returns_state"
      },
      {
        "path": "crates/ucp-python/tests/test_query_api.py",
        "score": 6,
        "test": "symbol:crates/ucp-python/tests/test_query_api.py::test_run_python_query_reports_errors_and_can_raise"
      },
      {
        "path": "crates/ucp-python/tests/test_query_api.py",
        "score": 6,
        "test": "symbol:crates/ucp-python/tests/test_query_api.py::test_run_python_query_supports_bindings_for_parameterized_queries"
      },
      {
        "path": "crates/ucp-python/tests/test_query_api.py",
        "score": 5,
        "test": "symbol:crates/ucp-python/tests/test_query_api.py::test_codegraph_query_facade_supports_minimal_agent_surface"
      },
      {
        "path": "crates/ucp-python/tests/test_query_api.py",
        "score": 5,
        "test": "symbol:crates/ucp-python/tests/test_query_api.py::test_codegraph_query_runner_supports_branch_and_compare"
      },
      {
        "path": "crates/ucp-python/tests/test_query_api.py",
        "score": 5,
        "test": "symbol:crates/ucp-python/tests/test_query_api.py::test_generic_query_facade_supports_agent_friendly_aliases"
      },
      {
        "path": "crates/ucp-python/tests/test_query_tools.py",
        "score": 4,
        "test": "symbol:crates/ucp-python/tests/test_query_tools.py::test_run_python_query_enforces_max_operations"
      },
      {
        "path": "crates/ucp-python/tests/test_query_tools.py",
        "score": 4,
        "test": "symbol:crates/ucp-python/tests/test_query_tools.py::test_run_python_query_enforces_max_seconds_via_monkeypatched_clock"
      },
      {
        "path": "crates/ucp-python/tests/test_query_tools.py",
        "score": 4,
        "test": "symbol:crates/ucp-python/tests/test_query_tools.py::test_run_python_query_enforces_max_stdout_chars"
      },
      {
        "path": "crates/ucp-python/tests/test_query_tools.py",
        "score": 4,
        "test": "symbol:crates/ucp-python/tests/test_query_tools.py::test_run_python_query_enforces_max_trace_events"
      }
    ],
    "target": "symbol:crates/ucp-python/python/ucp/query.py::run_python_query"
  },
  "selected_block_ids": [],
  "stdout": "",
  "summary": {
    "directories": 0,
    "files": 0,
    "hydrated_sources": 0,
    "max_selected": 48,
    "repositories": 0,
    "selected": 0,
    "symbols": 0
  },
  "usage": {
    "elapsed_seconds": 0.273682,
    "operation_count": 2,
    "stdout_chars": 0,
    "trace_events": 1171
  }
}
```

## Trace context_show to render configuration symbols

```json
{
  "error": null,
  "export": null,
  "limits": {
    "max_operations": null,
    "max_seconds": null,
    "max_stdout_chars": null,
    "max_trace_events": null
  },
  "ok": true,
  "result": [
    {
      "hops": 1,
      "start": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
      "target": "symbol:crates/ucp-cli/src/commands/codegraph.rs::make_export_config"
    },
    {
      "hops": 3,
      "start": "symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
      "target": "symbol:crates/ucp-cli/src/commands/codegraph.rs::make_export_config"
    }
  ],
  "selected_block_ids": [],
  "stdout": "",
  "summary": {
    "directories": 0,
    "files": 0,
    "hydrated_sources": 0,
    "max_selected": 48,
    "repositories": 0,
    "selected": 0,
    "symbols": 0
  },
  "usage": {
    "elapsed_seconds": 4.14365,
    "operation_count": 12,
    "stdout_chars": 0,
    "trace_events": 478
  }
}
```

## Rank session-related symbols by local evidence

```json
{
  "error": null,
  "export": null,
  "limits": {
    "max_operations": null,
    "max_seconds": null,
    "max_stdout_chars": null,
    "max_trace_events": null
  },
  "ok": true,
  "result": [
    {
      "edges": 9,
      "score": 17,
      "selected": 6,
      "target": "symbol:crates/ucp-python/python/ucp/query.py::BaseQuerySession"
    },
    {
      "edges": 8,
      "score": 15,
      "selected": 5,
      "target": "symbol:crates/ucp-cli/src/state.rs::AgentSessionState"
    },
    {
      "edges": 8,
      "score": 14,
      "selected": 4,
      "target": "symbol:crates/ucp-codegraph/src/context.rs::CodeGraphContextSession"
    },
    {
      "edges": 5,
      "score": 10,
      "selected": 3,
      "target": "symbol:crates/ucp-codegraph/src/context.rs::CodeGraphContextEdgeExport"
    },
    {
      "edges": 5,
      "score": 10,
      "selected": 3,
      "target": "symbol:crates/ucp-codegraph/src/context.rs::CodeGraphContextExport"
    },
    {
      "edges": 5,
      "score": 10,
      "selected": 3,
      "target": "symbol:crates/ucp-codegraph/src/context.rs::CodeGraphContextFrontierAction"
    }
  ],
  "selected_block_ids": [],
  "stdout": "",
  "summary": {
    "directories": 0,
    "files": 0,
    "hydrated_sources": 0,
    "max_selected": 48,
    "repositories": 0,
    "selected": 0,
    "symbols": 0
  },
  "usage": {
    "elapsed_seconds": 7.528892,
    "operation_count": 49,
    "stdout_chars": 0,
    "trace_events": 1967
  }
}
```

## Find lightweight public wrappers around run_python_query

```json
{
  "error": null,
  "export": null,
  "limits": {
    "max_operations": null,
    "max_seconds": null,
    "max_stdout_chars": null,
    "max_trace_events": null
  },
  "ok": true,
  "result": [
    {
      "detail": "neighborhood",
      "logical_key": "symbol:crates/ucp-python/python/ucp/query.py::run_python_query",
      "path": "crates/ucp-python/python/ucp/query.py"
    },
    {
      "detail": "symbol_card",
      "logical_key": "symbol:crates/ucp-python/python/ucp/query.py::BaseQueryGraph::run",
      "path": "crates/ucp-python/python/ucp/query.py"
    },
    {
      "detail": "symbol_card",
      "logical_key": "symbol:crates/ucp-python/python/ucp/query.py::BaseQuerySession::run",
      "path": "crates/ucp-python/python/ucp/query.py"
    },
    {
      "detail": "skeleton",
      "logical_key": "file:crates/ucp-python/python/ucp/query.py",
      "path": "crates/ucp-python/python/ucp/query.py"
    }
  ],
  "selected_block_ids": [],
  "stdout": "",
  "summary": {
    "directories": 0,
    "files": 0,
    "hydrated_sources": 0,
    "max_selected": 48,
    "repositories": 0,
    "selected": 0,
    "symbols": 0
  },
  "usage": {
    "elapsed_seconds": 0.780636,
    "operation_count": 5,
    "stdout_chars": 0,
    "trace_events": 254
  }
}
```
