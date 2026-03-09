## CodeGraph Python query façade demo

This transcript demonstrates agent-style repository querying through the thin Python façade and query runner, using regex discovery, loops, branch-and-compare, and targeted hydration.

## Raw CodeGraph summary

```json
{
  "nodes": 5548,
  "repr": "CodeGraph(nodes=5548)"
}
```

## Facade graph.find(...) seed candidates

```json
[
  {
    "block_id": "blk_0cb4f27ad738e059268f66dc",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/agent.rs#L1012-L1085",
      "end_line": 1085,
      "path": "crates/ucp-cli/src/commands/agent.rs",
      "start_line": 1012
    },
    "exported": false,
    "label": "context_show",
    "logical_key": "symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
    "node_class": "symbol",
    "path": "crates/ucp-cli/src/commands/agent.rs",
    "symbol_name": "context_show"
  },
  {
    "block_id": "blk_417c1f9bfe4c790b8f40b26c",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/codegraph.rs#L657-L702",
      "end_line": 702,
      "path": "crates/ucp-cli/src/commands/codegraph.rs",
      "start_line": 657
    },
    "exported": false,
    "label": "context_show",
    "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
    "node_class": "symbol",
    "path": "crates/ucp-cli/src/commands/codegraph.rs",
    "symbol_name": "context_show"
  },
  {
    "block_id": "blk_32c2f79214c85e15ed7111ca",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/agent.rs#L1107-L1116",
      "end_line": 1116,
      "path": "crates/ucp-cli/src/commands/agent.rs",
      "start_line": 1107
    },
    "exported": false,
    "label": "get_session_mut",
    "logical_key": "symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut",
    "node_class": "symbol",
    "path": "crates/ucp-cli/src/commands/agent.rs",
    "symbol_name": "get_session_mut"
  },
  {
    "block_id": "blk_f47adb3aefb90cb841185e13",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/codegraph.rs#L1629-L1638",
      "end_line": 1638,
      "path": "crates/ucp-cli/src/commands/codegraph.rs",
      "start_line": 1629
    },
    "exported": false,
    "label": "get_session_mut",
    "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut",
    "node_class": "symbol",
    "path": "crates/ucp-cli/src/commands/codegraph.rs",
    "symbol_name": "get_session_mut"
  },
  {
    "block_id": "blk_89a19cc0e721bb3337a34809",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/codegraph.rs#L1648-L1685",
      "end_line": 1685,
      "path": "crates/ucp-cli/src/commands/codegraph.rs",
      "start_line": 1648
    },
    "exported": false,
    "label": "print_context_update",
    "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update",
    "node_class": "symbol",
    "path": "crates/ucp-cli/src/commands/codegraph.rs",
    "symbol_name": "print_context_update"
  },
  {
    "block_id": "blk_ec5990bce475a7f5e5b35c8f",
    "coderef": {
      "display": "crates/ucp-cli/src/commands/agent.rs#L1118-L1158",
      "end_line": 1158,
      "path": "crates/ucp-cli/src/commands/agent.rs",
      "start_line": 1118
    },
    "exported": false,
    "label": "print_context_update",
    "logical_key": "symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update",
    "node_class": "symbol",
    "path": "crates/ucp-cli/src/commands/agent.rs",
    "symbol_name": "print_context_update"
  }
]
```

## Python query runner result

```json
{
  "error": null,
  "export": {
    "edges": [
      {
        "multiplicity": 1,
        "relation": "uses_symbol",
        "source": "blk_417c1f9bfe4c790b8f40b26c",
        "source_short_id": "S1",
        "target": "blk_53fa15e7333e649f4eb74d9c",
        "target_short_id": "S2"
      },
      {
        "multiplicity": 1,
        "relation": "uses_symbol",
        "source": "blk_417c1f9bfe4c790b8f40b26c",
        "source_short_id": "S1",
        "target": "blk_d1fab9b7f09d11d524cad890",
        "target_short_id": "S3"
      },
      {
        "multiplicity": 1,
        "relation": "uses_symbol",
        "source": "blk_417c1f9bfe4c790b8f40b26c",
        "source_short_id": "S1",
        "target": "blk_384b68c8c1725eba6a9eb9da",
        "target_short_id": "S4"
      },
      {
        "multiplicity": 1,
        "relation": "uses_symbol",
        "source": "blk_417c1f9bfe4c790b8f40b26c",
        "source_short_id": "S1",
        "target": "blk_0c6c13995a670d18a116596b",
        "target_short_id": "S5"
      }
    ],
    "export_mode": "compact",
    "focus": "blk_417c1f9bfe4c790b8f40b26c",
    "focus_label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
    "focus_short_id": "S1",
    "frontier": [
      {
        "action": "hydrate_source",
        "block_id": "blk_417c1f9bfe4c790b8f40b26c",
        "candidate_count": 0,
        "description": "Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
        "priority": 120,
        "short_id": "S1"
      },
      {
        "action": "expand_dependents",
        "block_id": "blk_417c1f9bfe4c790b8f40b26c",
        "candidate_count": 1,
        "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
        "direction": "incoming",
        "priority": 69,
        "relation": "uses_symbol",
        "short_id": "S1"
      },
      {
        "action": "collapse",
        "block_id": "blk_417c1f9bfe4c790b8f40b26c",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show from working set",
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
          "block_id": "blk_417c1f9bfe4c790b8f40b26c",
          "candidate_count": 1,
          "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
          "direction": "incoming",
          "priority": 69,
          "relation": "uses_symbol",
          "short_id": "S1"
        },
        {
          "action": "collapse",
          "block_id": "blk_417c1f9bfe4c790b8f40b26c",
          "candidate_count": 1,
          "description": "Collapse symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show from working set",
          "priority": 6,
          "short_id": "S1"
        }
      ],
      "recommended_next_action": {
        "action": "expand_dependents",
        "block_id": "blk_417c1f9bfe4c790b8f40b26c",
        "candidate_count": 1,
        "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
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
        "block_id": "blk_417c1f9bfe4c790b8f40b26c",
        "coderef": {
          "display": "crates/ucp-cli/src/commands/codegraph.rs#L657-L702",
          "end_line": 702,
          "path": "crates/ucp-cli/src/commands/codegraph.rs",
          "start_line": 657
        },
        "detail_level": "source",
        "distance_from_focus": 0,
        "hydrated_source": {
          "display": "crates/ucp-cli/src/commands/codegraph.rs#L657-L702",
          "end_line": 702,
          "path": "crates/ucp-cli/src/commands/codegraph.rs",
          "snippet": " 655 | }\n 656 | \n 657 | fn context_show(\n 658 |     input: Option<String>,\n 659 |     session: String,\n 660 |     max_tokens: usize,\n 661 |     compact: bool,\n 662 |     no_rendered: bool,\n 663 |     levels: Option<usize>,\n 664 |     only: Option<String>,\n 665 |     exclude: Option<String>,\n 666 |     format: OutputFormat,\n 667 | ) -> Result<()> {\n 668 |     let stateful = read_stateful_document(input)?;\n 669 |     ensure_codegraph_document(&stateful.document)?;\n 670 |     let sess = get_session(&stateful, &session)?;\n 671 |     let context = sess\n 672 |         .codegraph_context\n 673 |         .as_ref()\n 674 |         .ok_or_else(|| anyhow!(\"Session has no codegraph context: {}\", session))?;\n 675 |     let config = CodeGraphRenderConfig::for_max_tokens(max_tokens);\n 676 |     let export_config = make_export_config(\n 677 |         &sess.codegraph_preferences,\n 678 |         compact,\n 679 |         no_rendered,\n 680 |         levels,\n 681 |         only.as_deref(),\n 682 |         exclude.as_deref(),\n 683 |     )?;\n 684 |     let export =\n 685 |         export_codegraph_context_with_config(&stateful.document, context, &config, &export_config);\n 686 | \n 687 |     match format {\n 688 |         OutputFormat::Json => {\n 689 |             let mut value = serde_json::to_value(&export)?;\n 690 |             if let Some(object) = value.as_object_mut() {\n 691 |                 object.insert(\"session\".to_string(), serde_json::Value::String(session));\n 692 |             }\n 693 |             println!(\"{}\", serde_json::to_string_pretty(&value)?);\n 694 |         }\n 695 |         OutputFormat::Text => println!(\n 696 |             \"{}\",\n 697 |             render_context_show_text(&stateful.document, context, &config, &export)\n 698 |         ),\n 699 |     }\n 700 | \n 701 |     Ok(())\n 702 | }\n 703 | \n 704 | fn context_export(",
          "start_line": 657
        },
        "label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
        "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
        "node_class": "symbol",
        "origin": {
          "kind": "manual"
        },
        "pinned": false,
        "relevance_score": 224,
        "short_id": "S1",
        "signature": "function context_show(input: Option<String>, session: String, max_tokens: usize, compact: bool, no_rendered: bool, levels: Option<usize>, only: Option<String>, exclude: Option<String>, format: OutputFormat) -> Result<()>"
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
          "anchor": "417c1f9bfe4c790b8f40b26c",
          "kind": "dependencies",
          "relation": "uses_symbol"
        },
        "pinned": false,
        "relevance_score": 84,
        "short_id": "S2",
        "signature": "function ensure_codegraph_document(doc: &Document) -> Result<()>"
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
          "anchor": "417c1f9bfe4c790b8f40b26c",
          "kind": "dependencies",
          "relation": "uses_symbol"
        },
        "pinned": false,
        "relevance_score": 84,
        "short_id": "S3",
        "signature": "function get_session(stateful: &'a StatefulDocument, session: &str) -> Result<&'a AgentSessionState>"
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
          "anchor": "417c1f9bfe4c790b8f40b26c",
          "kind": "dependencies",
          "relation": "uses_symbol"
        },
        "pinned": false,
        "relevance_score": 84,
        "short_id": "S4",
        "signature": "function make_export_config(preferences: &CodeGraphSessionPreferences, compact: bool, no_rendered: bool, levels: Option<usize>, only: Option<&str>, exclude: Option<&str>) -> Result<CodeGraphExportConfig>"
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
          "anchor": "417c1f9bfe4c790b8f40b26c",
          "kind": "dependencies",
          "relation": "uses_symbol"
        },
        "pinned": false,
        "relevance_score": 84,
        "short_id": "S5",
        "signature": "function read_stateful_document(input: Option<String>) -> anyhow::Result<StatefulDocument> [public]"
      }
    ],
    "omitted_symbol_count": 5309,
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
  "ok": true,
  "result": {
    "best": {
      "diff": {
        "added": [
          {
            "block_id": "blk_417c1f9bfe4c790b8f40b26c",
            "coderef": {
              "display": "crates/ucp-cli/src/commands/codegraph.rs#L657-L702",
              "end_line": 702,
              "path": "crates/ucp-cli/src/commands/codegraph.rs",
              "start_line": 657
            },
            "exported": false,
            "label": "context_show",
            "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
            "node_class": "symbol",
            "path": "crates/ucp-cli/src/commands/codegraph.rs",
            "symbol_name": "context_show"
          },
          {
            "block_id": "blk_53fa15e7333e649f4eb74d9c",
            "coderef": {
              "display": "crates/ucp-cli/src/commands/codegraph.rs#L1599-L1605",
              "end_line": 1605,
              "path": "crates/ucp-cli/src/commands/codegraph.rs",
              "start_line": 1599
            },
            "exported": false,
            "label": "ensure_codegraph_document",
            "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::ensure_codegraph_document",
            "node_class": "symbol",
            "path": "crates/ucp-cli/src/commands/codegraph.rs",
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
            "exported": false,
            "label": "get_session",
            "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session",
            "node_class": "symbol",
            "path": "crates/ucp-cli/src/commands/codegraph.rs",
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
            "exported": false,
            "label": "make_export_config",
            "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::make_export_config",
            "node_class": "symbol",
            "path": "crates/ucp-cli/src/commands/codegraph.rs",
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
            "exported": true,
            "label": "read_stateful_document",
            "logical_key": "symbol:crates/ucp-cli/src/state.rs::read_stateful_document",
            "node_class": "symbol",
            "path": "crates/ucp-cli/src/state.rs",
            "symbol_name": "read_stateful_document"
          }
        ],
        "changed_focus": true,
        "focus_after": "blk_417c1f9bfe4c790b8f40b26c",
        "removed": []
      },
      "has_frontier": true,
      "selected": 5,
      "target": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show"
    },
    "branches": [
      {
        "diff": {
          "added": [
            {
              "block_id": "blk_6557d3b244263e4971245831",
              "coderef": {
                "display": "crates/ucp-cli/src/output.rs#L245-L272",
                "end_line": 272,
                "path": "crates/ucp-cli/src/output.rs",
                "start_line": 245
              },
              "exported": true,
              "label": "content_preview",
              "logical_key": "symbol:crates/ucp-cli/src/output.rs::content_preview",
              "node_class": "symbol",
              "path": "crates/ucp-cli/src/output.rs",
              "symbol_name": "content_preview"
            },
            {
              "block_id": "blk_0cb4f27ad738e059268f66dc",
              "coderef": {
                "display": "crates/ucp-cli/src/commands/agent.rs#L1012-L1085",
                "end_line": 1085,
                "path": "crates/ucp-cli/src/commands/agent.rs",
                "start_line": 1012
              },
              "exported": false,
              "label": "context_show",
              "logical_key": "symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
              "node_class": "symbol",
              "path": "crates/ucp-cli/src/commands/agent.rs",
              "symbol_name": "context_show"
            },
            {
              "block_id": "blk_0c6c13995a670d18a116596b",
              "coderef": {
                "display": "crates/ucp-cli/src/state.rs#L269-L296",
                "end_line": 296,
                "path": "crates/ucp-cli/src/state.rs",
                "start_line": 269
              },
              "exported": true,
              "label": "read_stateful_document",
              "logical_key": "symbol:crates/ucp-cli/src/state.rs::read_stateful_document",
              "node_class": "symbol",
              "path": "crates/ucp-cli/src/state.rs",
              "symbol_name": "read_stateful_document"
            }
          ],
          "changed_focus": true,
          "focus_after": "blk_0cb4f27ad738e059268f66dc",
          "removed": []
        },
        "has_frontier": true,
        "selected": 3,
        "target": "symbol:crates/ucp-cli/src/commands/agent.rs::context_show"
      },
      {
        "diff": {
          "added": [
            {
              "block_id": "blk_417c1f9bfe4c790b8f40b26c",
              "coderef": {
                "display": "crates/ucp-cli/src/commands/codegraph.rs#L657-L702",
                "end_line": 702,
                "path": "crates/ucp-cli/src/commands/codegraph.rs",
                "start_line": 657
              },
              "exported": false,
              "label": "context_show",
              "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
              "node_class": "symbol",
              "path": "crates/ucp-cli/src/commands/codegraph.rs",
              "symbol_name": "context_show"
            },
            {
              "block_id": "blk_53fa15e7333e649f4eb74d9c",
              "coderef": {
                "display": "crates/ucp-cli/src/commands/codegraph.rs#L1599-L1605",
                "end_line": 1605,
                "path": "crates/ucp-cli/src/commands/codegraph.rs",
                "start_line": 1599
              },
              "exported": false,
              "label": "ensure_codegraph_document",
              "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::ensure_codegraph_document",
              "node_class": "symbol",
              "path": "crates/ucp-cli/src/commands/codegraph.rs",
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
              "exported": false,
              "label": "get_session",
              "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session",
              "node_class": "symbol",
              "path": "crates/ucp-cli/src/commands/codegraph.rs",
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
              "exported": false,
              "label": "make_export_config",
              "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::make_export_config",
              "node_class": "symbol",
              "path": "crates/ucp-cli/src/commands/codegraph.rs",
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
              "exported": true,
              "label": "read_stateful_document",
              "logical_key": "symbol:crates/ucp-cli/src/state.rs::read_stateful_document",
              "node_class": "symbol",
              "path": "crates/ucp-cli/src/state.rs",
              "symbol_name": "read_stateful_document"
            }
          ],
          "changed_focus": true,
          "focus_after": "blk_417c1f9bfe4c790b8f40b26c",
          "removed": []
        },
        "has_frontier": true,
        "selected": 5,
        "target": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show"
      },
      {
        "diff": {
          "added": [
            {
              "block_id": "blk_32c2f79214c85e15ed7111ca",
              "coderef": {
                "display": "crates/ucp-cli/src/commands/agent.rs#L1107-L1116",
                "end_line": 1116,
                "path": "crates/ucp-cli/src/commands/agent.rs",
                "start_line": 1107
              },
              "exported": false,
              "label": "get_session_mut",
              "logical_key": "symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut",
              "node_class": "symbol",
              "path": "crates/ucp-cli/src/commands/agent.rs",
              "symbol_name": "get_session_mut"
            }
          ],
          "changed_focus": true,
          "focus_after": "blk_32c2f79214c85e15ed7111ca",
          "removed": []
        },
        "has_frontier": true,
        "selected": 1,
        "target": "symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut"
      }
    ],
    "candidates": [
      {
        "block_id": "blk_0cb4f27ad738e059268f66dc",
        "coderef": {
          "display": "crates/ucp-cli/src/commands/agent.rs#L1012-L1085",
          "end_line": 1085,
          "path": "crates/ucp-cli/src/commands/agent.rs",
          "start_line": 1012
        },
        "exported": false,
        "label": "context_show",
        "logical_key": "symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
        "node_class": "symbol",
        "path": "crates/ucp-cli/src/commands/agent.rs",
        "symbol_name": "context_show"
      },
      {
        "block_id": "blk_417c1f9bfe4c790b8f40b26c",
        "coderef": {
          "display": "crates/ucp-cli/src/commands/codegraph.rs#L657-L702",
          "end_line": 702,
          "path": "crates/ucp-cli/src/commands/codegraph.rs",
          "start_line": 657
        },
        "exported": false,
        "label": "context_show",
        "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
        "node_class": "symbol",
        "path": "crates/ucp-cli/src/commands/codegraph.rs",
        "symbol_name": "context_show"
      },
      {
        "block_id": "blk_32c2f79214c85e15ed7111ca",
        "coderef": {
          "display": "crates/ucp-cli/src/commands/agent.rs#L1107-L1116",
          "end_line": 1116,
          "path": "crates/ucp-cli/src/commands/agent.rs",
          "start_line": 1107
        },
        "exported": false,
        "label": "get_session_mut",
        "logical_key": "symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut",
        "node_class": "symbol",
        "path": "crates/ucp-cli/src/commands/agent.rs",
        "symbol_name": "get_session_mut"
      },
      {
        "block_id": "blk_f47adb3aefb90cb841185e13",
        "coderef": {
          "display": "crates/ucp-cli/src/commands/codegraph.rs#L1629-L1638",
          "end_line": 1638,
          "path": "crates/ucp-cli/src/commands/codegraph.rs",
          "start_line": 1629
        },
        "exported": false,
        "label": "get_session_mut",
        "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut",
        "node_class": "symbol",
        "path": "crates/ucp-cli/src/commands/codegraph.rs",
        "symbol_name": "get_session_mut"
      },
      {
        "block_id": "blk_89a19cc0e721bb3337a34809",
        "coderef": {
          "display": "crates/ucp-cli/src/commands/codegraph.rs#L1648-L1685",
          "end_line": 1685,
          "path": "crates/ucp-cli/src/commands/codegraph.rs",
          "start_line": 1648
        },
        "exported": false,
        "label": "print_context_update",
        "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update",
        "node_class": "symbol",
        "path": "crates/ucp-cli/src/commands/codegraph.rs",
        "symbol_name": "print_context_update"
      },
      {
        "block_id": "blk_ec5990bce475a7f5e5b35c8f",
        "coderef": {
          "display": "crates/ucp-cli/src/commands/agent.rs#L1118-L1158",
          "end_line": 1158,
          "path": "crates/ucp-cli/src/commands/agent.rs",
          "start_line": 1118
        },
        "exported": false,
        "label": "print_context_update",
        "logical_key": "symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update",
        "node_class": "symbol",
        "path": "crates/ucp-cli/src/commands/agent.rs",
        "symbol_name": "print_context_update"
      }
    ],
    "final_export": {
      "edges": [
        {
          "multiplicity": 1,
          "relation": "uses_symbol",
          "source": "blk_417c1f9bfe4c790b8f40b26c",
          "source_short_id": "S1",
          "target": "blk_53fa15e7333e649f4eb74d9c",
          "target_short_id": "S2"
        },
        {
          "multiplicity": 1,
          "relation": "uses_symbol",
          "source": "blk_417c1f9bfe4c790b8f40b26c",
          "source_short_id": "S1",
          "target": "blk_d1fab9b7f09d11d524cad890",
          "target_short_id": "S3"
        },
        {
          "multiplicity": 1,
          "relation": "uses_symbol",
          "source": "blk_417c1f9bfe4c790b8f40b26c",
          "source_short_id": "S1",
          "target": "blk_384b68c8c1725eba6a9eb9da",
          "target_short_id": "S4"
        },
        {
          "multiplicity": 1,
          "relation": "uses_symbol",
          "source": "blk_417c1f9bfe4c790b8f40b26c",
          "source_short_id": "S1",
          "target": "blk_0c6c13995a670d18a116596b",
          "target_short_id": "S5"
        }
      ],
      "export_mode": "compact",
      "focus": "blk_417c1f9bfe4c790b8f40b26c",
      "focus_label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
      "focus_short_id": "S1",
      "frontier": [
        {
          "action": "hydrate_source",
          "block_id": "blk_417c1f9bfe4c790b8f40b26c",
          "candidate_count": 0,
          "description": "Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
          "priority": 120,
          "short_id": "S1"
        },
        {
          "action": "expand_dependents",
          "block_id": "blk_417c1f9bfe4c790b8f40b26c",
          "candidate_count": 1,
          "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
          "direction": "incoming",
          "priority": 69,
          "relation": "uses_symbol",
          "short_id": "S1"
        },
        {
          "action": "collapse",
          "block_id": "blk_417c1f9bfe4c790b8f40b26c",
          "candidate_count": 1,
          "description": "Collapse symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show from working set",
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
            "block_id": "blk_417c1f9bfe4c790b8f40b26c",
            "candidate_count": 1,
            "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
            "direction": "incoming",
            "priority": 69,
            "relation": "uses_symbol",
            "short_id": "S1"
          },
          {
            "action": "collapse",
            "block_id": "blk_417c1f9bfe4c790b8f40b26c",
            "candidate_count": 1,
            "description": "Collapse symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show from working set",
            "priority": 6,
            "short_id": "S1"
          }
        ],
        "recommended_next_action": {
          "action": "expand_dependents",
          "block_id": "blk_417c1f9bfe4c790b8f40b26c",
          "candidate_count": 1,
          "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
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
          "block_id": "blk_417c1f9bfe4c790b8f40b26c",
          "coderef": {
            "display": "crates/ucp-cli/src/commands/codegraph.rs#L657-L702",
            "end_line": 702,
            "path": "crates/ucp-cli/src/commands/codegraph.rs",
            "start_line": 657
          },
          "detail_level": "source",
          "distance_from_focus": 0,
          "hydrated_source": {
            "display": "crates/ucp-cli/src/commands/codegraph.rs#L657-L702",
            "end_line": 702,
            "path": "crates/ucp-cli/src/commands/codegraph.rs",
            "snippet": " 655 | }\n 656 | \n 657 | fn context_show(\n 658 |     input: Option<String>,\n 659 |     session: String,\n 660 |     max_tokens: usize,\n 661 |     compact: bool,\n 662 |     no_rendered: bool,\n 663 |     levels: Option<usize>,\n 664 |     only: Option<String>,\n 665 |     exclude: Option<String>,\n 666 |     format: OutputFormat,\n 667 | ) -> Result<()> {\n 668 |     let stateful = read_stateful_document(input)?;\n 669 |     ensure_codegraph_document(&stateful.document)?;\n 670 |     let sess = get_session(&stateful, &session)?;\n 671 |     let context = sess\n 672 |         .codegraph_context\n 673 |         .as_ref()\n 674 |         .ok_or_else(|| anyhow!(\"Session has no codegraph context: {}\", session))?;\n 675 |     let config = CodeGraphRenderConfig::for_max_tokens(max_tokens);\n 676 |     let export_config = make_export_config(\n 677 |         &sess.codegraph_preferences,\n 678 |         compact,\n 679 |         no_rendered,\n 680 |         levels,\n 681 |         only.as_deref(),\n 682 |         exclude.as_deref(),\n 683 |     )?;\n 684 |     let export =\n 685 |         export_codegraph_context_with_config(&stateful.document, context, &config, &export_config);\n 686 | \n 687 |     match format {\n 688 |         OutputFormat::Json => {\n 689 |             let mut value = serde_json::to_value(&export)?;\n 690 |             if let Some(object) = value.as_object_mut() {\n 691 |                 object.insert(\"session\".to_string(), serde_json::Value::String(session));\n 692 |             }\n 693 |             println!(\"{}\", serde_json::to_string_pretty(&value)?);\n 694 |         }\n 695 |         OutputFormat::Text => println!(\n 696 |             \"{}\",\n 697 |             render_context_show_text(&stateful.document, context, &config, &export)\n 698 |         ),\n 699 |     }\n 700 | \n 701 |     Ok(())\n 702 | }\n 703 | \n 704 | fn context_export(",
            "start_line": 657
          },
          "label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
          "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
          "node_class": "symbol",
          "origin": {
            "kind": "manual"
          },
          "pinned": false,
          "relevance_score": 224,
          "short_id": "S1",
          "signature": "function context_show(input: Option<String>, session: String, max_tokens: usize, compact: bool, no_rendered: bool, levels: Option<usize>, only: Option<String>, exclude: Option<String>, format: OutputFormat) -> Result<()>"
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
            "anchor": "417c1f9bfe4c790b8f40b26c",
            "kind": "dependencies",
            "relation": "uses_symbol"
          },
          "pinned": false,
          "relevance_score": 84,
          "short_id": "S2",
          "signature": "function ensure_codegraph_document(doc: &Document) -> Result<()>"
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
            "anchor": "417c1f9bfe4c790b8f40b26c",
            "kind": "dependencies",
            "relation": "uses_symbol"
          },
          "pinned": false,
          "relevance_score": 84,
          "short_id": "S3",
          "signature": "function get_session(stateful: &'a StatefulDocument, session: &str) -> Result<&'a AgentSessionState>"
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
            "anchor": "417c1f9bfe4c790b8f40b26c",
            "kind": "dependencies",
            "relation": "uses_symbol"
          },
          "pinned": false,
          "relevance_score": 84,
          "short_id": "S4",
          "signature": "function make_export_config(preferences: &CodeGraphSessionPreferences, compact: bool, no_rendered: bool, levels: Option<usize>, only: Option<&str>, exclude: Option<&str>) -> Result<CodeGraphExportConfig>"
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
            "anchor": "417c1f9bfe4c790b8f40b26c",
            "kind": "dependencies",
            "relation": "uses_symbol"
          },
          "pinned": false,
          "relevance_score": 84,
          "short_id": "S5",
          "signature": "function read_stateful_document(input: Option<String>) -> anyhow::Result<StatefulDocument> [public]"
        }
      ],
      "omitted_symbol_count": 5309,
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
    "blk_0c6c13995a670d18a116596b",
    "blk_384b68c8c1725eba6a9eb9da",
    "blk_417c1f9bfe4c790b8f40b26c",
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
  }
}
```

## Final export

```json
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "blk_417c1f9bfe4c790b8f40b26c",
      "source_short_id": "S1",
      "target": "blk_53fa15e7333e649f4eb74d9c",
      "target_short_id": "S2"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "blk_417c1f9bfe4c790b8f40b26c",
      "source_short_id": "S1",
      "target": "blk_d1fab9b7f09d11d524cad890",
      "target_short_id": "S3"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "blk_417c1f9bfe4c790b8f40b26c",
      "source_short_id": "S1",
      "target": "blk_384b68c8c1725eba6a9eb9da",
      "target_short_id": "S4"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "blk_417c1f9bfe4c790b8f40b26c",
      "source_short_id": "S1",
      "target": "blk_0c6c13995a670d18a116596b",
      "target_short_id": "S5"
    }
  ],
  "export_mode": "compact",
  "focus": "blk_417c1f9bfe4c790b8f40b26c",
  "focus_label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
  "focus_short_id": "S1",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "blk_417c1f9bfe4c790b8f40b26c",
      "candidate_count": 0,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
      "priority": 120,
      "short_id": "S1"
    },
    {
      "action": "expand_dependents",
      "block_id": "blk_417c1f9bfe4c790b8f40b26c",
      "candidate_count": 1,
      "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
      "direction": "incoming",
      "priority": 69,
      "relation": "uses_symbol",
      "short_id": "S1"
    },
    {
      "action": "collapse",
      "block_id": "blk_417c1f9bfe4c790b8f40b26c",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show from working set",
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
        "block_id": "blk_417c1f9bfe4c790b8f40b26c",
        "candidate_count": 1,
        "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
        "direction": "incoming",
        "priority": 69,
        "relation": "uses_symbol",
        "short_id": "S1"
      },
      {
        "action": "collapse",
        "block_id": "blk_417c1f9bfe4c790b8f40b26c",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show from working set",
        "priority": 6,
        "short_id": "S1"
      }
    ],
    "recommended_next_action": {
      "action": "expand_dependents",
      "block_id": "blk_417c1f9bfe4c790b8f40b26c",
      "candidate_count": 1,
      "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
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
      "block_id": "blk_417c1f9bfe4c790b8f40b26c",
      "coderef": {
        "display": "crates/ucp-cli/src/commands/codegraph.rs#L657-L702",
        "end_line": 702,
        "path": "crates/ucp-cli/src/commands/codegraph.rs",
        "start_line": 657
      },
      "detail_level": "source",
      "distance_from_focus": 0,
      "hydrated_source": {
        "display": "crates/ucp-cli/src/commands/codegraph.rs#L657-L702",
        "end_line": 702,
        "path": "crates/ucp-cli/src/commands/codegraph.rs",
        "snippet": " 655 | }\n 656 | \n 657 | fn context_show(\n 658 |     input: Option<String>,\n 659 |     session: String,\n 660 |     max_tokens: usize,\n 661 |     compact: bool,\n 662 |     no_rendered: bool,\n 663 |     levels: Option<usize>,\n 664 |     only: Option<String>,\n 665 |     exclude: Option<String>,\n 666 |     format: OutputFormat,\n 667 | ) -> Result<()> {\n 668 |     let stateful = read_stateful_document(input)?;\n 669 |     ensure_codegraph_document(&stateful.document)?;\n 670 |     let sess = get_session(&stateful, &session)?;\n 671 |     let context = sess\n 672 |         .codegraph_context\n 673 |         .as_ref()\n 674 |         .ok_or_else(|| anyhow!(\"Session has no codegraph context: {}\", session))?;\n 675 |     let config = CodeGraphRenderConfig::for_max_tokens(max_tokens);\n 676 |     let export_config = make_export_config(\n 677 |         &sess.codegraph_preferences,\n 678 |         compact,\n 679 |         no_rendered,\n 680 |         levels,\n 681 |         only.as_deref(),\n 682 |         exclude.as_deref(),\n 683 |     )?;\n 684 |     let export =\n 685 |         export_codegraph_context_with_config(&stateful.document, context, &config, &export_config);\n 686 | \n 687 |     match format {\n 688 |         OutputFormat::Json => {\n 689 |             let mut value = serde_json::to_value(&export)?;\n 690 |             if let Some(object) = value.as_object_mut() {\n 691 |                 object.insert(\"session\".to_string(), serde_json::Value::String(session));\n 692 |             }\n 693 |             println!(\"{}\", serde_json::to_string_pretty(&value)?);\n 694 |         }\n 695 |         OutputFormat::Text => println!(\n 696 |             \"{}\",\n 697 |             render_context_show_text(&stateful.document, context, &config, &export)\n 698 |         ),\n 699 |     }\n 700 | \n 701 |     Ok(())\n 702 | }\n 703 | \n 704 | fn context_export(",
        "start_line": 657
      },
      "label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
      "logical_key": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
      "node_class": "symbol",
      "origin": {
        "kind": "manual"
      },
      "pinned": false,
      "relevance_score": 224,
      "short_id": "S1",
      "signature": "function context_show(input: Option<String>, session: String, max_tokens: usize, compact: bool, no_rendered: bool, levels: Option<usize>, only: Option<String>, exclude: Option<String>, format: OutputFormat) -> Result<()>"
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
        "anchor": "417c1f9bfe4c790b8f40b26c",
        "kind": "dependencies",
        "relation": "uses_symbol"
      },
      "pinned": false,
      "relevance_score": 84,
      "short_id": "S2",
      "signature": "function ensure_codegraph_document(doc: &Document) -> Result<()>"
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
        "anchor": "417c1f9bfe4c790b8f40b26c",
        "kind": "dependencies",
        "relation": "uses_symbol"
      },
      "pinned": false,
      "relevance_score": 84,
      "short_id": "S3",
      "signature": "function get_session(stateful: &'a StatefulDocument, session: &str) -> Result<&'a AgentSessionState>"
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
        "anchor": "417c1f9bfe4c790b8f40b26c",
        "kind": "dependencies",
        "relation": "uses_symbol"
      },
      "pinned": false,
      "relevance_score": 84,
      "short_id": "S4",
      "signature": "function make_export_config(preferences: &CodeGraphSessionPreferences, compact: bool, no_rendered: bool, levels: Option<usize>, only: Option<&str>, exclude: Option<&str>) -> Result<CodeGraphExportConfig>"
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
        "anchor": "417c1f9bfe4c790b8f40b26c",
        "kind": "dependencies",
        "relation": "uses_symbol"
      },
      "pinned": false,
      "relevance_score": 84,
      "short_id": "S5",
      "signature": "function read_stateful_document(input: Option<String>) -> anyhow::Result<StatefulDocument> [public]"
    }
  ],
  "omitted_symbol_count": 5309,
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
```

## Final summary

- ok: True
- selected nodes: 5
- transcript: `artifacts/codegraph-python-query-demo-transcript.md`
