## CodeGraph agent benchmark suite demo

This transcript runs a small benchmark/evaluation suite of canonical agent workflows against the UCP codebase graph.

## Suite summary

```json
{
  "cases": 4,
  "failed": 0,
  "ok": 4,
  "total_elapsed_seconds": 9.993398,
  "total_operations": 48,
  "total_trace_events": 2992
}
```

## Benchmark: rank-tests-for-run_python_query

```json
{
  "description": "Rank the most likely Python tests for run_python_query via lightweight name/path heuristics.",
  "error": null,
  "export": null,
  "limits": {
    "max_operations": 120,
    "max_seconds": 8.0,
    "max_stdout_chars": null,
    "max_trace_events": 8000
  },
  "name": "rank-tests-for-run_python_query",
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
        "path": "crates/ucp-python/tests/test_query_tools.py",
        "score": 6,
        "test": "symbol:crates/ucp-python/tests/test_query_tools.py::test_run_python_query_enforces_max_operations"
      },
      {
        "path": "crates/ucp-python/tests/test_query_tools.py",
        "score": 6,
        "test": "symbol:crates/ucp-python/tests/test_query_tools.py::test_run_python_query_enforces_max_seconds_via_monkeypatched_clock"
      },
      {
        "path": "crates/ucp-python/tests/test_query_tools.py",
        "score": 6,
        "test": "symbol:crates/ucp-python/tests/test_query_tools.py::test_run_python_query_enforces_max_stdout_chars"
      },
      {
        "path": "crates/ucp-python/tests/test_query_tools.py",
        "score": 6,
        "test": "symbol:crates/ucp-python/tests/test_query_tools.py::test_run_python_query_enforces_max_trace_events"
      },
      {
        "path": "crates/ucp-python/tests/test_query_tools.py",
        "score": 6,
        "test": "symbol:crates/ucp-python/tests/test_query_tools.py::test_run_python_query_reports_usage_and_limits"
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
    "elapsed_seconds": 0.264111,
    "operation_count": 2,
    "stdout_chars": 0,
    "trace_events": 1098
  }
}
```

## Benchmark: trace-context-show-to-render-config

```json
{
  "description": "Connect CLI context_show handlers to render/export helpers using path-based explanation.",
  "error": null,
  "export": null,
  "limits": {
    "max_operations": 120,
    "max_seconds": 8.0,
    "max_stdout_chars": null,
    "max_trace_events": 8000
  },
  "name": "trace-context-show-to-render-config",
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
    "elapsed_seconds": 4.24049,
    "operation_count": 12,
    "stdout_chars": 0,
    "trace_events": 482
  }
}
```

## Benchmark: rank-before-hydrate-context-symbols

```json
{
  "description": "Score candidate context/render symbols, then only hydrate the top branch.",
  "error": null,
  "export": {
    "edges": [
      {
        "multiplicity": 1,
        "relation": "uses_symbol",
        "source": "blk_047a6b142bd13e2db299bf35",
        "source_short_id": "S1",
        "target": "blk_53fa15e7333e649f4eb74d9c",
        "target_short_id": "S2"
      },
      {
        "multiplicity": 1,
        "relation": "uses_symbol",
        "source": "blk_047a6b142bd13e2db299bf35",
        "source_short_id": "S1",
        "target": "blk_d1fab9b7f09d11d524cad890",
        "target_short_id": "S3"
      },
      {
        "multiplicity": 1,
        "relation": "uses_symbol",
        "source": "blk_047a6b142bd13e2db299bf35",
        "source_short_id": "S1",
        "target": "blk_384b68c8c1725eba6a9eb9da",
        "target_short_id": "S4"
      },
      {
        "multiplicity": 1,
        "relation": "uses_symbol",
        "source": "blk_047a6b142bd13e2db299bf35",
        "source_short_id": "S1",
        "target": "blk_0c6c13995a670d18a116596b",
        "target_short_id": "S5"
      }
    ],
    "export_mode": "compact",
    "focus": "blk_047a6b142bd13e2db299bf35",
    "focus_label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export",
    "focus_short_id": "S1",
    "frontier": [
      {
        "action": "hydrate_source",
        "block_id": "blk_047a6b142bd13e2db299bf35",
        "candidate_count": 0,
        "description": "Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export",
        "priority": 120,
        "short_id": "S1"
      },
      {
        "action": "expand_dependents",
        "block_id": "blk_047a6b142bd13e2db299bf35",
        "candidate_count": 1,
        "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export",
        "direction": "incoming",
        "priority": 69,
        "relation": "uses_symbol",
        "short_id": "S1"
      },
      {
        "action": "collapse",
        "block_id": "blk_047a6b142bd13e2db299bf35",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export from working set",
        "priority": 6,
        "short_id": "S1"
      }
    ],
    "heuristics": {
      "hidden_candidate_count": 1,
      "low_value_candidate_count": 0,
      "recommended_actions": [
        {
          "action": "expand_dependents",
          "block_id": "blk_047a6b142bd13e2db299bf35",
          "candidate_count": 1,
          "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export",
          "direction": "incoming",
          "priority": 69,
          "relation": "uses_symbol",
          "short_id": "S1"
        },
        {
          "action": "collapse",
          "block_id": "blk_047a6b142bd13e2db299bf35",
          "candidate_count": 1,
          "description": "Collapse symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export from working set",
          "priority": 6,
          "short_id": "S1"
        }
      ],
      "recommended_next_action": {
        "action": "expand_dependents",
        "block_id": "blk_047a6b142bd13e2db299bf35",
        "candidate_count": 1,
        "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export",
        "direction": "incoming",
        "priority": 69,
        "relation": "uses_symbol",
        "short_id": "S1"
      },
      "should_stop": false
    },
    "hidden_unreachable_count": 0,
    "nodes": [
      {
        "block_id": "blk_047a6b142bd13e2db299bf35",
        "coderef": {
          "display": "crates/ucp-cli/src/commands/codegraph.rs#L704-L746",
          "end_line": 746,
          "path": "crates/ucp-cli/src/commands/codegraph.rs",
          "start_line": 704
        },
        "detail_level": "source",
        "distance_from_focus": 0,
        "hydrated_source": {
          "display": "crates/ucp-cli/src/commands/codegraph.rs#L704-L746",
          "end_line": 746,
          "path": "crates/ucp-cli/src/commands/codegraph.rs",
          "snippet": " 702 | }\n 703 | \n 704 | fn context_export(\n 705 |     input: Option<String>,\n 706 |     session: String,\n 707 |     max_tokens: usize,\n 708 |     compact: bool,\n 709 |     no_rendered: bool,\n 710 |     levels: Option<usize>,\n 711 |     only: Option<String>,\n 712 |     exclude: Option<String>,\n 713 |     format: OutputFormat,\n 714 | ) -> Result<()> {\n 715 |     let stateful = read_stateful_document(input)?;\n 716 |     ensure_codegraph_document(&stateful.document)?;\n 717 |     let sess = get_session(&stateful, &session)?;\n 718 |     let context = sess\n 719 |         .codegraph_context\n 720 |         .as_ref()\n 721 |         .ok_or_else(|| anyhow!(\"Session has no codegraph context: {}\", session))?;\n 722 |     let config = CodeGraphRenderConfig::for_max_tokens(max_tokens);\n 723 |     let export_config = make_export_config(\n 724 |         &sess.codegraph_preferences,\n 725 |         compact,\n 726 |         no_rendered,\n 727 |         levels,\n 728 |         only.as_deref(),\n 729 |         exclude.as_deref(),\n 730 |     )?;\n 731 |     let export =\n 732 |         export_codegraph_context_with_config(&stateful.document, context, &config, &export_config);\n 733 | \n 734 |     match format {\n 735 |         OutputFormat::Json => {\n 736 |             let mut value = serde_json::to_value(&export)?;\n 737 |             if let Some(object) = value.as_object_mut() {\n 738 |                 object.insert(\"session\".to_string(), serde_json::Value::String(session));\n 739 |             }\n 740 |             println!(\"{}\", serde_json::to_string_pretty(&value)?);\n 741 |         }\n 742 |         OutputFormat::Text => println!(\"{}\", serde_json::to_string_pretty(&export)?),\n 743 |     }\n 744 | \n 745 |     Ok(())\n 746 | }\n 747 | \n 748 | #[allow(clippy::too_many_arguments)]",
          "start_line": 704
        },
        "label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export",
        "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export",
        "node_class": "symbol",
        "origin": {
          "kind": "manual"
        },
        "path": "crates/ucp-cli/src/commands/codegraph.rs",
        "pinned": false,
        "relevance_score": 224,
        "short_id": "S1",
        "signature": "function context_export(input: Option<String>, session: String, max_tokens: usize, compact: bool, no_rendered: bool, levels: Option<usize>, only: Option<String>, exclude: Option<String>, format: OutputFormat) -> Result<()>",
        "symbol_name": "context_export"
      },
      {
        "block_id": "blk_53fa15e7333e649f4eb74d9c",
        "coderef": {
          "display": "crates/ucp-cli/src/commands/codegraph.rs#L1599-L1605",
          "end_line": 1605,
          "path": "crates/ucp-cli/src/commands/codegraph.rs",
          "start_line": 1599
        },
        "detail_level": "symbol_card",
        "distance_from_focus": 1,
        "label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::ensure_codegraph_document",
        "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::ensure_codegraph_document",
        "node_class": "symbol",
        "origin": {
          "anchor": "047a6b142bd13e2db299bf35",
          "kind": "dependencies",
          "relation": "uses_symbol"
        },
        "path": "crates/ucp-cli/src/commands/codegraph.rs",
        "pinned": false,
        "relevance_score": 84,
        "short_id": "S2",
        "signature": "function ensure_codegraph_document(doc: &Document) -> Result<()>",
        "symbol_name": "ensure_codegraph_document"
      },
      {
        "block_id": "blk_d1fab9b7f09d11d524cad890",
        "coderef": {
          "display": "crates/ucp-cli/src/commands/codegraph.rs#L1621-L1627",
          "end_line": 1627,
          "path": "crates/ucp-cli/src/commands/codegraph.rs",
          "start_line": 1621
        },
        "detail_level": "symbol_card",
        "distance_from_focus": 1,
        "label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session",
        "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session",
        "node_class": "symbol",
        "origin": {
          "anchor": "047a6b142bd13e2db299bf35",
          "kind": "dependencies",
          "relation": "uses_symbol"
        },
        "path": "crates/ucp-cli/src/commands/codegraph.rs",
        "pinned": false,
        "relevance_score": 84,
        "short_id": "S3",
        "signature": "function get_session(stateful: &'a StatefulDocument, session: &str) -> Result<&'a AgentSessionState>",
        "symbol_name": "get_session"
      },
      {
        "block_id": "blk_384b68c8c1725eba6a9eb9da",
        "coderef": {
          "display": "crates/ucp-cli/src/commands/codegraph.rs#L822-L851",
          "end_line": 851,
          "path": "crates/ucp-cli/src/commands/codegraph.rs",
          "start_line": 822
        },
        "detail_level": "symbol_card",
        "distance_from_focus": 1,
        "label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::make_export_config",
        "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::make_export_config",
        "node_class": "symbol",
        "origin": {
          "anchor": "047a6b142bd13e2db299bf35",
          "kind": "dependencies",
          "relation": "uses_symbol"
        },
        "path": "crates/ucp-cli/src/commands/codegraph.rs",
        "pinned": false,
        "relevance_score": 84,
        "short_id": "S4",
        "signature": "function make_export_config(preferences: &CodeGraphSessionPreferences, compact: bool, no_rendered: bool, levels: Option<usize>, only: Option<&str>, exclude: Option<&str>) -> Result<CodeGraphExportConfig>",
        "symbol_name": "make_export_config"
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
          "anchor": "047a6b142bd13e2db299bf35",
          "kind": "dependencies",
          "relation": "uses_symbol"
        },
        "path": "crates/ucp-cli/src/state.rs",
        "pinned": false,
        "relevance_score": 84,
        "short_id": "S5",
        "signature": "function read_stateful_document(input: Option<String>) -> anyhow::Result<StatefulDocument> [public]",
        "symbol_name": "read_stateful_document"
      }
    ],
    "omitted_symbol_count": 5375,
    "summary": {
      "directories": 0,
      "files": 0,
      "hydrated_sources": 1,
      "max_selected": 48,
      "repositories": 0,
      "selected": 5,
      "symbols": 5
    },
    "total_selected_edges": 4,
    "visible_node_count": 5
  },
  "limits": {
    "max_operations": 120,
    "max_seconds": 8.0,
    "max_stdout_chars": null,
    "max_trace_events": 8000
  },
  "name": "rank-before-hydrate-context-symbols",
  "ok": true,
  "result": {
    "best": {
      "score": 14,
      "target": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export"
    },
    "export": {
      "edges": [
        {
          "multiplicity": 1,
          "relation": "uses_symbol",
          "source": "blk_047a6b142bd13e2db299bf35",
          "source_short_id": "S1",
          "target": "blk_53fa15e7333e649f4eb74d9c",
          "target_short_id": "S2"
        },
        {
          "multiplicity": 1,
          "relation": "uses_symbol",
          "source": "blk_047a6b142bd13e2db299bf35",
          "source_short_id": "S1",
          "target": "blk_d1fab9b7f09d11d524cad890",
          "target_short_id": "S3"
        },
        {
          "multiplicity": 1,
          "relation": "uses_symbol",
          "source": "blk_047a6b142bd13e2db299bf35",
          "source_short_id": "S1",
          "target": "blk_384b68c8c1725eba6a9eb9da",
          "target_short_id": "S4"
        },
        {
          "multiplicity": 1,
          "relation": "uses_symbol",
          "source": "blk_047a6b142bd13e2db299bf35",
          "source_short_id": "S1",
          "target": "blk_0c6c13995a670d18a116596b",
          "target_short_id": "S5"
        }
      ],
      "export_mode": "compact",
      "focus": "blk_047a6b142bd13e2db299bf35",
      "focus_label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export",
      "focus_short_id": "S1",
      "frontier": [
        {
          "action": "hydrate_source",
          "block_id": "blk_047a6b142bd13e2db299bf35",
          "candidate_count": 0,
          "description": "Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export",
          "priority": 120,
          "short_id": "S1"
        },
        {
          "action": "expand_dependents",
          "block_id": "blk_047a6b142bd13e2db299bf35",
          "candidate_count": 1,
          "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export",
          "direction": "incoming",
          "priority": 69,
          "relation": "uses_symbol",
          "short_id": "S1"
        },
        {
          "action": "collapse",
          "block_id": "blk_047a6b142bd13e2db299bf35",
          "candidate_count": 1,
          "description": "Collapse symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export from working set",
          "priority": 6,
          "short_id": "S1"
        }
      ],
      "heuristics": {
        "hidden_candidate_count": 1,
        "low_value_candidate_count": 0,
        "recommended_actions": [
          {
            "action": "expand_dependents",
            "block_id": "blk_047a6b142bd13e2db299bf35",
            "candidate_count": 1,
            "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export",
            "direction": "incoming",
            "priority": 69,
            "relation": "uses_symbol",
            "short_id": "S1"
          },
          {
            "action": "collapse",
            "block_id": "blk_047a6b142bd13e2db299bf35",
            "candidate_count": 1,
            "description": "Collapse symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export from working set",
            "priority": 6,
            "short_id": "S1"
          }
        ],
        "recommended_next_action": {
          "action": "expand_dependents",
          "block_id": "blk_047a6b142bd13e2db299bf35",
          "candidate_count": 1,
          "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export",
          "direction": "incoming",
          "priority": 69,
          "relation": "uses_symbol",
          "short_id": "S1"
        },
        "should_stop": false
      },
      "hidden_unreachable_count": 0,
      "nodes": [
        {
          "block_id": "blk_047a6b142bd13e2db299bf35",
          "coderef": {
            "display": "crates/ucp-cli/src/commands/codegraph.rs#L704-L746",
            "end_line": 746,
            "path": "crates/ucp-cli/src/commands/codegraph.rs",
            "start_line": 704
          },
          "detail_level": "source",
          "distance_from_focus": 0,
          "hydrated_source": {
            "display": "crates/ucp-cli/src/commands/codegraph.rs#L704-L746",
            "end_line": 746,
            "path": "crates/ucp-cli/src/commands/codegraph.rs",
            "snippet": " 702 | }\n 703 | \n 704 | fn context_export(\n 705 |     input: Option<String>,\n 706 |     session: String,\n 707 |     max_tokens: usize,\n 708 |     compact: bool,\n 709 |     no_rendered: bool,\n 710 |     levels: Option<usize>,\n 711 |     only: Option<String>,\n 712 |     exclude: Option<String>,\n 713 |     format: OutputFormat,\n 714 | ) -> Result<()> {\n 715 |     let stateful = read_stateful_document(input)?;\n 716 |     ensure_codegraph_document(&stateful.document)?;\n 717 |     let sess = get_session(&stateful, &session)?;\n 718 |     let context = sess\n 719 |         .codegraph_context\n 720 |         .as_ref()\n 721 |         .ok_or_else(|| anyhow!(\"Session has no codegraph context: {}\", session))?;\n 722 |     let config = CodeGraphRenderConfig::for_max_tokens(max_tokens);\n 723 |     let export_config = make_export_config(\n 724 |         &sess.codegraph_preferences,\n 725 |         compact,\n 726 |         no_rendered,\n 727 |         levels,\n 728 |         only.as_deref(),\n 729 |         exclude.as_deref(),\n 730 |     )?;\n 731 |     let export =\n 732 |         export_codegraph_context_with_config(&stateful.document, context, &config, &export_config);\n 733 | \n 734 |     match format {\n 735 |         OutputFormat::Json => {\n 736 |             let mut value = serde_json::to_value(&export)?;\n 737 |             if let Some(object) = value.as_object_mut() {\n 738 |                 object.insert(\"session\".to_string(), serde_json::Value::String(session));\n 739 |             }\n 740 |             println!(\"{}\", serde_json::to_string_pretty(&value)?);\n 741 |         }\n 742 |         OutputFormat::Text => println!(\"{}\", serde_json::to_string_pretty(&export)?),\n 743 |     }\n 744 | \n 745 |     Ok(())\n 746 | }\n 747 | \n 748 | #[allow(clippy::too_many_arguments)]",
            "start_line": 704
          },
          "label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export",
          "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export",
          "node_class": "symbol",
          "origin": {
            "kind": "manual"
          },
          "path": "crates/ucp-cli/src/commands/codegraph.rs",
          "pinned": false,
          "relevance_score": 224,
          "short_id": "S1",
          "signature": "function context_export(input: Option<String>, session: String, max_tokens: usize, compact: bool, no_rendered: bool, levels: Option<usize>, only: Option<String>, exclude: Option<String>, format: OutputFormat) -> Result<()>",
          "symbol_name": "context_export"
        },
        {
          "block_id": "blk_53fa15e7333e649f4eb74d9c",
          "coderef": {
            "display": "crates/ucp-cli/src/commands/codegraph.rs#L1599-L1605",
            "end_line": 1605,
            "path": "crates/ucp-cli/src/commands/codegraph.rs",
            "start_line": 1599
          },
          "detail_level": "symbol_card",
          "distance_from_focus": 1,
          "label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::ensure_codegraph_document",
          "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::ensure_codegraph_document",
          "node_class": "symbol",
          "origin": {
            "anchor": "047a6b142bd13e2db299bf35",
            "kind": "dependencies",
            "relation": "uses_symbol"
          },
          "path": "crates/ucp-cli/src/commands/codegraph.rs",
          "pinned": false,
          "relevance_score": 84,
          "short_id": "S2",
          "signature": "function ensure_codegraph_document(doc: &Document) -> Result<()>",
          "symbol_name": "ensure_codegraph_document"
        },
        {
          "block_id": "blk_d1fab9b7f09d11d524cad890",
          "coderef": {
            "display": "crates/ucp-cli/src/commands/codegraph.rs#L1621-L1627",
            "end_line": 1627,
            "path": "crates/ucp-cli/src/commands/codegraph.rs",
            "start_line": 1621
          },
          "detail_level": "symbol_card",
          "distance_from_focus": 1,
          "label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session",
          "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session",
          "node_class": "symbol",
          "origin": {
            "anchor": "047a6b142bd13e2db299bf35",
            "kind": "dependencies",
            "relation": "uses_symbol"
          },
          "path": "crates/ucp-cli/src/commands/codegraph.rs",
          "pinned": false,
          "relevance_score": 84,
          "short_id": "S3",
          "signature": "function get_session(stateful: &'a StatefulDocument, session: &str) -> Result<&'a AgentSessionState>",
          "symbol_name": "get_session"
        },
        {
          "block_id": "blk_384b68c8c1725eba6a9eb9da",
          "coderef": {
            "display": "crates/ucp-cli/src/commands/codegraph.rs#L822-L851",
            "end_line": 851,
            "path": "crates/ucp-cli/src/commands/codegraph.rs",
            "start_line": 822
          },
          "detail_level": "symbol_card",
          "distance_from_focus": 1,
          "label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::make_export_config",
          "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::make_export_config",
          "node_class": "symbol",
          "origin": {
            "anchor": "047a6b142bd13e2db299bf35",
            "kind": "dependencies",
            "relation": "uses_symbol"
          },
          "path": "crates/ucp-cli/src/commands/codegraph.rs",
          "pinned": false,
          "relevance_score": 84,
          "short_id": "S4",
          "signature": "function make_export_config(preferences: &CodeGraphSessionPreferences, compact: bool, no_rendered: bool, levels: Option<usize>, only: Option<&str>, exclude: Option<&str>) -> Result<CodeGraphExportConfig>",
          "symbol_name": "make_export_config"
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
            "anchor": "047a6b142bd13e2db299bf35",
            "kind": "dependencies",
            "relation": "uses_symbol"
          },
          "path": "crates/ucp-cli/src/state.rs",
          "pinned": false,
          "relevance_score": 84,
          "short_id": "S5",
          "signature": "function read_stateful_document(input: Option<String>) -> anyhow::Result<StatefulDocument> [public]",
          "symbol_name": "read_stateful_document"
        }
      ],
      "omitted_symbol_count": 5375,
      "summary": {
        "directories": 0,
        "files": 0,
        "hydrated_sources": 1,
        "max_selected": 48,
        "repositories": 0,
        "selected": 5,
        "symbols": 5
      },
      "total_selected_edges": 4,
      "visible_node_count": 5
    }
  },
  "selected_block_ids": [
    "blk_047a6b142bd13e2db299bf35",
    "blk_0c6c13995a670d18a116596b",
    "blk_384b68c8c1725eba6a9eb9da",
    "blk_53fa15e7333e649f4eb74d9c",
    "blk_d1fab9b7f09d11d524cad890"
  ],
  "stdout": "",
  "summary": {
    "directories": 0,
    "files": 0,
    "hydrated_sources": 1,
    "max_selected": 48,
    "repositories": 0,
    "selected": 5,
    "symbols": 5
  },
  "usage": {
    "elapsed_seconds": 4.724658,
    "operation_count": 29,
    "stdout_chars": 0,
    "trace_events": 1177
  }
}
```

## Benchmark: find-public-wrappers-around-run_python_query

```json
{
  "description": "Expand one hop to dependents and find lightweight public wrappers before hydrating deeper helpers.",
  "error": null,
  "export": null,
  "limits": {
    "max_operations": 120,
    "max_seconds": 8.0,
    "max_stdout_chars": null,
    "max_trace_events": 8000
  },
  "name": "find-public-wrappers-around-run_python_query",
  "ok": true,
  "result": {
    "target": "symbol:crates/ucp-python/python/ucp/query.py::run_python_query",
    "wrappers": [
      {
        "logical_key": "symbol:crates/ucp-python/python/ucp/query.py::BaseQueryGraph::run",
        "path": "crates/ucp-python/python/ucp/query.py",
        "symbol_name": "run"
      },
      {
        "logical_key": "symbol:crates/ucp-python/python/ucp/query.py::BaseQuerySession::run",
        "path": "crates/ucp-python/python/ucp/query.py",
        "symbol_name": "run"
      },
      {
        "logical_key": "file:crates/ucp-python/python/ucp/query.py",
        "path": "crates/ucp-python/python/ucp/query.py",
        "symbol_name": null
      }
    ]
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
    "elapsed_seconds": 0.764139,
    "operation_count": 5,
    "stdout_chars": 0,
    "trace_events": 235
  }
}
```
