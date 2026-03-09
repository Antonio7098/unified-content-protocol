## CodeGraph query recipes on the UCP repo

This transcript captures a few higher-level recipe patterns over the UCP codebase graph: branch-and-compare, explanation paths, and lightweight ranking via Python control flow.

## CodeGraph summary

```json
{
  "node_count": 5557,
  "repr": "CodeGraph(nodes=5557)"
}
```

## Compare mirrored context_show handlers

```json
{
  "error": null,
  "export": null,
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
  }
}
```

## Trace context_show to render configuration symbols

```json
{
  "error": null,
  "export": null,
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
  }
}
```

## Rank session-related symbols by local evidence

```json
{
  "error": null,
  "export": null,
  "ok": true,
  "result": [
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
      "score": 11,
      "selected": 4,
      "target": "symbol:crates/ucp-python/python/ucp/query.py::BaseQuerySession"
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
  }
}
```
