## CodeGraph query runner edge-case transcript

This transcript exercises common ergonomic edge cases for model-authored Python queries against the UCP repo CodeGraph.

## Indented triple-quoted snippet

```json
{
  "error": null,
  "export": null,
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
  }
}
```

## Common builtins like type()

```json
{
  "error": null,
  "export": null,
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
  }
}
```

## Parameterized regex via bindings

```json
{
  "error": null,
  "export": null,
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
  }
}
```

## Raw session compatibility

```json
{
  "error": null,
  "export": null,
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
  }
}
```
