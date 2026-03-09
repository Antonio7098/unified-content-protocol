## CodeGraph provider-facing Python query tool demo

This transcript demonstrates the provider/tool-facing wrapper around `run_python_query(...)`, including OpenAI-style and Anthropic-style envelopes plus execution limits.

## OpenAI tool definition

```json
{
  "function": {
    "description": "Execute a short Python query against a bound UCP Graph or CodeGraph. Use the prebound `graph` and `session` objects for traversal, then rely on normal Python for loops, conditionals, regex, and scoring. Return your final structured answer in a `result` variable.",
    "name": "run_python_query",
    "parameters": {
      "additionalProperties": false,
      "properties": {
        "bindings": {
          "description": "Optional parameters injected into the query environment.",
          "type": "object"
        },
        "code": {
          "description": "Python snippet to execute. Use `graph`, `session`, `re`, `json`, `math`, and `collections`.",
          "type": "string"
        },
        "export_kwargs": {
          "description": "Optional keyword arguments forwarded to session.export(...).",
          "type": "object"
        },
        "include_export": {
          "description": "If true, include the final session export in the tool result.",
          "type": "boolean"
        },
        "limits": {
          "description": "Optional execution guards for model-authored queries.",
          "properties": {
            "max_operations": {
              "type": "integer"
            },
            "max_seconds": {
              "type": "number"
            },
            "max_stdout_chars": {
              "type": "integer"
            },
            "max_trace_events": {
              "type": "integer"
            }
          },
          "type": "object"
        }
      },
      "required": [
        "code"
      ],
      "type": "object"
    }
  },
  "type": "function"
}
```

## Anthropic tool definition

```json
{
  "description": "Execute a short Python query against a bound UCP Graph or CodeGraph. Use the prebound `graph` and `session` objects for traversal, then rely on normal Python for loops, conditionals, regex, and scoring. Return your final structured answer in a `result` variable.",
  "input_schema": {
    "additionalProperties": false,
    "properties": {
      "bindings": {
        "description": "Optional parameters injected into the query environment.",
        "type": "object"
      },
      "code": {
        "description": "Python snippet to execute. Use `graph`, `session`, `re`, `json`, `math`, and `collections`.",
        "type": "string"
      },
      "export_kwargs": {
        "description": "Optional keyword arguments forwarded to session.export(...).",
        "type": "object"
      },
      "include_export": {
        "description": "If true, include the final session export in the tool result.",
        "type": "boolean"
      },
      "limits": {
        "description": "Optional execution guards for model-authored queries.",
        "properties": {
          "max_operations": {
            "type": "integer"
          },
          "max_seconds": {
            "type": "number"
          },
          "max_stdout_chars": {
            "type": "integer"
          },
          "max_trace_events": {
            "type": "integer"
          }
        },
        "type": "object"
      }
    },
    "required": [
      "code"
    ],
    "type": "object"
  },
  "name": "run_python_query"
}
```

## Direct tool execution

```json
{
  "error": null,
  "export": {
    "edges": [
      {
        "multiplicity": 1,
        "relation": "uses_symbol",
        "source": "blk_0cb4f27ad738e059268f66dc",
        "source_short_id": "S1",
        "target": "blk_6557d3b244263e4971245831",
        "target_short_id": "S2"
      },
      {
        "multiplicity": 1,
        "relation": "uses_symbol",
        "source": "blk_0cb4f27ad738e059268f66dc",
        "source_short_id": "S1",
        "target": "blk_0c6c13995a670d18a116596b",
        "target_short_id": "S3"
      }
    ],
    "export_mode": "compact",
    "focus": "blk_0cb4f27ad738e059268f66dc",
    "focus_label": "symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
    "focus_short_id": "S1",
    "frontier": [
      {
        "action": "hydrate_source",
        "block_id": "blk_0cb4f27ad738e059268f66dc",
        "candidate_count": 1,
        "description": "Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
        "priority": 121,
        "short_id": "S1"
      },
      {
        "action": "expand_dependents",
        "block_id": "blk_0cb4f27ad738e059268f66dc",
        "candidate_count": 1,
        "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
        "direction": "incoming",
        "priority": 69,
        "relation": "uses_symbol",
        "short_id": "S1"
      },
      {
        "action": "collapse",
        "block_id": "blk_0cb4f27ad738e059268f66dc",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/commands/agent.rs::context_show from working set",
        "priority": 6,
        "short_id": "S1"
      }
    ],
    "heuristics": {
      "hidden_candidate_count": 1,
      "low_value_candidate_count": 0,
      "recommended_actions": [
        {
          "action": "hydrate_source",
          "block_id": "blk_0cb4f27ad738e059268f66dc",
          "candidate_count": 1,
          "description": "Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
          "priority": 121,
          "short_id": "S1"
        },
        {
          "action": "expand_dependents",
          "block_id": "blk_0cb4f27ad738e059268f66dc",
          "candidate_count": 1,
          "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
          "direction": "incoming",
          "priority": 69,
          "relation": "uses_symbol",
          "short_id": "S1"
        },
        {
          "action": "collapse",
          "block_id": "blk_0cb4f27ad738e059268f66dc",
          "candidate_count": 1,
          "description": "Collapse symbol:crates/ucp-cli/src/commands/agent.rs::context_show from working set",
          "priority": 6,
          "short_id": "S1"
        }
      ],
      "recommended_next_action": {
        "action": "hydrate_source",
        "block_id": "blk_0cb4f27ad738e059268f66dc",
        "candidate_count": 1,
        "description": "Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
        "priority": 121,
        "short_id": "S1"
      },
      "should_stop": false
    },
    "hidden_unreachable_count": 0,
    "nodes": [
      {
        "block_id": "blk_0cb4f27ad738e059268f66dc",
        "coderef": {
          "display": "crates/ucp-cli/src/commands/agent.rs#L1012-L1085",
          "end_line": 1085,
          "path": "crates/ucp-cli/src/commands/agent.rs",
          "start_line": 1012
        },
        "detail_level": "neighborhood",
        "distance_from_focus": 0,
        "label": "symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
        "logical_key": "symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
        "node_class": "symbol",
        "origin": {
          "kind": "manual"
        },
        "path": "crates/ucp-cli/src/commands/agent.rs",
        "pinned": false,
        "relevance_score": 214,
        "short_id": "S1",
        "signature": "function context_show(input: Option<String>, session: String, format: OutputFormat) -> Result<()>",
        "symbol_name": "context_show"
      },
      {
        "block_id": "blk_6557d3b244263e4971245831",
        "coderef": {
          "display": "crates/ucp-cli/src/output.rs#L245-L272",
          "end_line": 272,
          "path": "crates/ucp-cli/src/output.rs",
          "start_line": 245
        },
        "detail_level": "symbol_card",
        "distance_from_focus": 1,
        "docs": "Get a preview of content (truncated to max_len)",
        "label": "symbol:crates/ucp-cli/src/output.rs::content_preview",
        "logical_key": "symbol:crates/ucp-cli/src/output.rs::content_preview",
        "node_class": "symbol",
        "origin": {
          "anchor": "0cb4f27ad738e059268f66dc",
          "kind": "dependencies",
          "relation": "uses_symbol"
        },
        "path": "crates/ucp-cli/src/output.rs",
        "pinned": false,
        "relevance_score": 84,
        "short_id": "S2",
        "signature": "function content_preview(content: &ucm_core::Content, max_len: usize) -> String [public]",
        "symbol_name": "content_preview"
      },
      {
        "block_id": "blk_0c6c13995a670d18a116596b",
        "coderef": {
          "display": "crates/ucp-cli/src/state.rs#L269-L296",
          "end_line": 296,
          "path": "crates/ucp-cli/src/state.rs",
          "start_line": 269
        },
        "detail_level": "symbol_card",
        "distance_from_focus": 1,
        "docs": "Read a stateful document from file or stdin",
        "label": "symbol:crates/ucp-cli/src/state.rs::read_stateful_document",
        "logical_key": "symbol:crates/ucp-cli/src/state.rs::read_stateful_document",
        "node_class": "symbol",
        "origin": {
          "anchor": "0cb4f27ad738e059268f66dc",
          "kind": "dependencies",
          "relation": "uses_symbol"
        },
        "path": "crates/ucp-cli/src/state.rs",
        "pinned": false,
        "relevance_score": 84,
        "short_id": "S3",
        "signature": "function read_stateful_document(input: Option<String>) -> anyhow::Result<StatefulDocument> [public]",
        "symbol_name": "read_stateful_document"
      }
    ],
    "omitted_symbol_count": 5377,
    "summary": {
      "directories": 0,
      "files": 0,
      "hydrated_sources": 0,
      "max_selected": 48,
      "repositories": 0,
      "selected": 3,
      "symbols": 3
    },
    "total_selected_edges": 2,
    "visible_node_count": 3
  },
  "limits": {
    "max_operations": 40,
    "max_seconds": 2.0,
    "max_stdout_chars": 4000,
    "max_trace_events": 2000
  },
  "ok": true,
  "result": {
    "first": "symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
    "selected": 3
  },
  "selected_block_ids": [
    "blk_0c6c13995a670d18a116596b",
    "blk_0cb4f27ad738e059268f66dc",
    "blk_6557d3b244263e4971245831"
  ],
  "stdout": "",
  "summary": {
    "directories": 0,
    "files": 0,
    "hydrated_sources": 0,
    "max_selected": 48,
    "repositories": 0,
    "selected": 3,
    "symbols": 3
  },
  "tool_name": "run_python_query",
  "usage": {
    "elapsed_seconds": 1.051813,
    "operation_count": 4,
    "stdout_chars": 0,
    "trace_events": 189
  }
}
```

## OpenAI tool result message

```json
{
  "content": "{\n  \"error\": null,\n  \"export\": {\n    \"edges\": [],\n    \"export_mode\": \"compact\",\n    \"frontier\": [],\n    \"heuristics\": {\n      \"hidden_candidate_count\": 0,\n      \"low_value_candidate_count\": 0,\n      \"reasons\": [\n        \"set focus to a file or symbol before continuing expansion\"\n      ],\n      \"should_stop\": false\n    },\n    \"hidden_unreachable_count\": 0,\n    \"nodes\": [\n      {\n        \"block_id\": \"blk_d0bc4d43952a282eb3ff9aa0\",\n        \"coderef\": {\n          \"display\": \"crates/ucp-python/python/ucp/query.py#L368-L432\",\n          \"end_line\": 432,\n          \"path\": \"crates/ucp-python/python/ucp/query.py\",\n          \"start_line\": 368\n        },\n        \"detail_level\": \"symbol_card\",\n        \"label\": \"symbol:crates/ucp-python/python/ucp/query.py::run_python_query\",\n        \"logical_key\": \"symbol:crates/ucp-python/python/ucp/query.py::run_python_query\",\n        \"node_class\": \"symbol\",\n        \"origin\": {\n          \"kind\": \"manual\"\n        },\n        \"path\": \"crates/ucp-python/python/ucp/query.py\",\n        \"pinned\": false,\n        \"relevance_score\": 76,\n        \"short_id\": \"S1\",\n        \"signature\": \"function run_python_query(graph: Graph | CodeGraph | BaseQueryGraph, code: str, session: Optional[BaseQuerySession | GraphSession | CodeGraphSession], bindings: Optional[Mapping[str, Any]], include_export: bool, export_kwargs: Optional[Mapping[str, Any]], limits: Optional[QueryLimits | Mapping[str, Any]], raise_on_error: bool) -> QueryRunResult\",\n        \"symbol_name\": \"run_python_query\"\n      }\n    ],\n    \"omitted_symbol_count\": 5379,\n    \"summary\": {\n      \"directories\": 0,\n      \"files\": 0,\n      \"hydrated_sources\": 0,\n      \"max_selected\": 48,\n      \"repositories\": 0,\n      \"selected\": 1,\n      \"symbols\": 1\n    },\n    \"total_selected_edges\": 0,\n    \"visible_node_count\": 1\n  },\n  \"limits\": {\n    \"max_operations\": 40,\n    \"max_seconds\": 2.0,\n    \"max_stdout_chars\": 4000,\n    \"max_trace_events\": 2000\n  },\n  \"ok\": true,\n  \"result\": \"symbol:crates/ucp-python/python/ucp/query.py::run_python_query\",\n  \"selected_block_ids\": [\n    \"blk_d0bc4d43952a282eb3ff9aa0\"\n  ],\n  \"stdout\": \"\",\n  \"summary\": {\n    \"directories\": 0,\n    \"files\": 0,\n    \"hydrated_sources\": 0,\n    \"max_selected\": 48,\n    \"repositories\": 0,\n    \"selected\": 1,\n    \"symbols\": 1\n  },\n  \"tool_name\": \"run_python_query\",\n  \"usage\": {\n    \"elapsed_seconds\": 0.542173,\n    \"operation_count\": 2,\n    \"stdout_chars\": 0,\n    \"trace_events\": 99\n  }\n}",
  "role": "tool",
  "tool_call_id": "call_demo_1"
}
```

## Anthropic tool result

```json
{
  "content": [
    {
      "text": "{\n  \"error\": null,\n  \"export\": {\n    \"edges\": [],\n    \"export_mode\": \"compact\",\n    \"frontier\": [],\n    \"heuristics\": {\n      \"hidden_candidate_count\": 0,\n      \"low_value_candidate_count\": 0,\n      \"reasons\": [\n        \"set focus to a file or symbol before continuing expansion\"\n      ],\n      \"should_stop\": false\n    },\n    \"hidden_unreachable_count\": 0,\n    \"nodes\": [\n      {\n        \"block_id\": \"blk_d0bc4d43952a282eb3ff9aa0\",\n        \"coderef\": {\n          \"display\": \"crates/ucp-python/python/ucp/query.py#L368-L432\",\n          \"end_line\": 432,\n          \"path\": \"crates/ucp-python/python/ucp/query.py\",\n          \"start_line\": 368\n        },\n        \"detail_level\": \"symbol_card\",\n        \"label\": \"symbol:crates/ucp-python/python/ucp/query.py::run_python_query\",\n        \"logical_key\": \"symbol:crates/ucp-python/python/ucp/query.py::run_python_query\",\n        \"node_class\": \"symbol\",\n        \"origin\": {\n          \"kind\": \"manual\"\n        },\n        \"path\": \"crates/ucp-python/python/ucp/query.py\",\n        \"pinned\": false,\n        \"relevance_score\": 76,\n        \"short_id\": \"S1\",\n        \"signature\": \"function run_python_query(graph: Graph | CodeGraph | BaseQueryGraph, code: str, session: Optional[BaseQuerySession | GraphSession | CodeGraphSession], bindings: Optional[Mapping[str, Any]], include_export: bool, export_kwargs: Optional[Mapping[str, Any]], limits: Optional[QueryLimits | Mapping[str, Any]], raise_on_error: bool) -> QueryRunResult\",\n        \"symbol_name\": \"run_python_query\"\n      }\n    ],\n    \"omitted_symbol_count\": 5379,\n    \"summary\": {\n      \"directories\": 0,\n      \"files\": 0,\n      \"hydrated_sources\": 0,\n      \"max_selected\": 48,\n      \"repositories\": 0,\n      \"selected\": 1,\n      \"symbols\": 1\n    },\n    \"total_selected_edges\": 0,\n    \"visible_node_count\": 1\n  },\n  \"limits\": {\n    \"max_operations\": 40,\n    \"max_seconds\": 2.0,\n    \"max_stdout_chars\": 4000,\n    \"max_trace_events\": 2000\n  },\n  \"ok\": true,\n  \"result\": {\n    \"selected\": 1\n  },\n  \"selected_block_ids\": [\n    \"blk_d0bc4d43952a282eb3ff9aa0\"\n  ],\n  \"stdout\": \"hello from tool\\n\",\n  \"summary\": {\n    \"directories\": 0,\n    \"files\": 0,\n    \"hydrated_sources\": 0,\n    \"max_selected\": 48,\n    \"repositories\": 0,\n    \"selected\": 1,\n    \"symbols\": 1\n  },\n  \"tool_name\": \"run_python_query\",\n  \"usage\": {\n    \"elapsed_seconds\": 0.672412,\n    \"operation_count\": 3,\n    \"stdout_chars\": 16,\n    \"trace_events\": 148\n  }\n}",
      "type": "text"
    }
  ],
  "is_error": false,
  "tool_use_id": "toolu_demo_1",
  "type": "tool_result"
}
```
