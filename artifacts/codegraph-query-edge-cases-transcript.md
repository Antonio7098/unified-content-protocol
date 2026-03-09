## CodeGraph query runner edge-case transcript

This transcript exercises common ergonomic edge cases for model-authored Python queries against the UCP repo CodeGraph.

## Indented triple-quoted snippet

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
  "result": "symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
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
    "elapsed_seconds": 0.183156,
    "operation_count": 1,
    "stdout_chars": 0,
    "trace_events": 54
  }
}
```

## Common builtins like type()

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
    "has_logical_key": true,
    "python_type": "dict"
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
    "elapsed_seconds": 0.177758,
    "operation_count": 1,
    "stdout_chars": 0,
    "trace_events": 52
  }
}
```

## Parameterized regex via bindings

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
    "count": 4,
    "first": "symbol:crates/ucp-cli/src/commands/agent.rs::context_show"
  },
  "selected_block_ids": [
    "blk_0cb4f27ad738e059268f66dc"
  ],
  "stdout": "",
  "summary": {
    "directories": 0,
    "files": 0,
    "hydrated_sources": 0,
    "max_selected": 48,
    "repositories": 0,
    "selected": 1,
    "symbols": 1
  },
  "usage": {
    "elapsed_seconds": 0.325546,
    "operation_count": 2,
    "stdout_chars": 0,
    "trace_events": 97
  }
}
```

## Raw session compatibility

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
  "result": 1,
  "selected_block_ids": [
    "blk_0cb4f27ad738e059268f66dc"
  ],
  "stdout": "",
  "summary": {
    "directories": 0,
    "files": 0,
    "hydrated_sources": 0,
    "max_selected": 48,
    "repositories": 0,
    "selected": 1,
    "symbols": 1
  },
  "usage": {
    "elapsed_seconds": 0.449877,
    "operation_count": 3,
    "stdout_chars": 0,
    "trace_events": 130
  }
}
```
