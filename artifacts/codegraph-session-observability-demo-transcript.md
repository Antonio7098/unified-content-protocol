## CodeGraph Session Observability Demo

This transcript exercises selector explanations, mutation telemetry, recommendations, pre-mutation estimates, omission reporting, prune explanations, and session persistence.

## Selector Explanation

```json
{
  "ambiguous": false,
  "candidates": [
    {
      "block_id": "blk_183a3c18b3cd680dbd76c053",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L20-L189",
        "end_line": 189,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 20
      },
      "exported": false,
      "label": "PyCodeGraph",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph#20",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "PyCodeGraph"
    },
    {
      "block_id": "blk_4db23b2c636638e099ba1716",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L9-L11",
        "end_line": 11,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 9
      },
      "exported": true,
      "label": "PyCodeGraph",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "PyCodeGraph"
    },
    {
      "block_id": "blk_b244fc9d04b2363f1f16775e",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L15-L17",
        "end_line": 17,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 15
      },
      "exported": true,
      "label": "PyCodeGraphSession",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "PyCodeGraphSession"
    },
    {
      "block_id": "blk_c65bb9fcd9a2fb78fe727e73",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L192-L574",
        "end_line": 574,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 192
      },
      "exported": false,
      "label": "PyCodeGraphSession",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession#192",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "PyCodeGraphSession"
    },
    {
      "block_id": "blk_034bfb7cc344199a27e369b8",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L568-L573",
        "end_line": 573,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 568
      },
      "exported": false,
      "label": "__repr__",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::__repr__",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "__repr__"
    },
    {
      "block_id": "blk_76411991300998183044734c",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L186-L188",
        "end_line": 188,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 186
      },
      "exported": false,
      "label": "__repr__",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph::__repr__",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "__repr__"
    },
    {
      "block_id": "blk_d25655ca10b46913a4589e25",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L381-L397",
        "end_line": 397,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 381
      },
      "exported": false,
      "label": "apply_recommended",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::apply_recommended",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "apply_recommended"
    },
    {
      "block_id": "blk_4ab2809f7f8ed88ade5348cb",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L24-L57",
        "end_line": 57,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 24
      },
      "exported": false,
      "label": "build",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph::build",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "build"
    },
    {
      "block_id": "blk_ea6deba2d66fdae85f758184",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L653-L678",
        "end_line": 678,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 653
      },
      "exported": false,
      "label": "build_budget",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::build_budget",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "build_budget"
    },
    {
      "block_id": "blk_4251423dff95f312138c2607",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L576-L594",
        "end_line": 594,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 576
      },
      "exported": false,
      "label": "build_find_query",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::build_find_query",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "build_find_query"
    },
    {
      "block_id": "blk_4ac20768c8331bfa0dfb0ebf",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L596-L615",
        "end_line": 615,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 596
      },
      "exported": false,
      "label": "build_traversal",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "build_traversal"
    },
    {
      "block_id": "blk_11721ce840f9f170ca9910b9",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L321-L334",
        "end_line": 334,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 321
      },
      "exported": false,
      "label": "collapse",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::collapse",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "collapse"
    },
    {
      "block_id": "blk_ca19df6efb93453bc3518777",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs",
        "path": "crates/ucp-python/src/codegraph.rs"
      },
      "exported": false,
      "label": "crates/ucp-python/src/codegraph.rs",
      "logical_key": "file:crates/ucp-python/src/codegraph.rs",
      "node_class": "file",
      "path": "crates/ucp-python/src/codegraph.rs"
    },
    {
      "block_id": "blk_5ccb717843fd62bd2d1f1fc0",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L130-L136",
        "end_line": 136,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 130
      },
      "exported": false,
      "label": "describe",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph::describe",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "describe"
    },
    {
      "block_id": "blk_178bdc2b3cac7ce4854fa926",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L521-L523",
        "end_line": 523,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 521
      },
      "exported": false,
      "label": "diff",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::diff",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "diff"
    },
    {
      "block_id": "blk_4052e54817e42f30e8e9df40",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L454-L489",
        "end_line": 489,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 454
      },
      "exported": false,
      "label": "estimate_expand",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::estimate_expand",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "estimate_expand"
    },
    {
      "block_id": "blk_cc412522185ca26452f8e6d9",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L492-L518",
        "end_line": 518,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 492
      },
      "exported": false,
      "label": "estimate_hydrate",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::estimate_hydrate",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "estimate_hydrate"
    },
    {
      "block_id": "blk_3662e18a4ab66439a7084c66",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L221-L223",
        "end_line": 223,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 221
      },
      "exported": false,
      "label": "event_log",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::event_log",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "event_log"
    },
    {
      "block_id": "blk_582ba056ba0d669d17a8d058",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L253-L289",
        "end_line": 289,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 253
      },
      "exported": false,
      "label": "expand",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::expand",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "expand"
    },
    {
      "block_id": "blk_8ab6f78cbc4ae3e05a8a3d63",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L417-L446",
        "end_line": 446,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 417
      },
      "exported": false,
      "label": "explain_export_omission",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::explain_export_omission",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "explain_export_omission"
    },
    {
      "block_id": "blk_34d3c639dc5225b3978af393",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L411-L413",
        "end_line": 413,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 411
      },
      "exported": false,
      "label": "explain_selector",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::explain_selector",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "explain_selector"
    },
    {
      "block_id": "blk_c4270095b18fa8268e33d0ce",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L107-L109",
        "end_line": 109,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 107
      },
      "exported": false,
      "label": "explain_selector",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph::explain_selector",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "explain_selector"
    },
    {
      "block_id": "blk_6f5c6c1765f36ce9b0b2229e",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L351-L373",
        "end_line": 373,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 351
      },
      "exported": false,
      "label": "export",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::export",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "export"
    },
    {
      "block_id": "blk_4e63644d5a83a5582e5cc1f1",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L628-L651",
        "end_line": 651,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 628
      },
      "exported": false,
      "label": "export_config",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::export_config",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "export_config"
    },
    {
      "block_id": "blk_793d30fc3e9330701f925bec",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L140-L164",
        "end_line": 164,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 140
      },
      "exported": false,
      "label": "find_nodes",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph::find_nodes",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "find_nodes"
    },
    {
      "block_id": "blk_85878cf700470f12816ab54c",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L542-L566",
        "end_line": 566,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 542
      },
      "exported": false,
      "label": "find_nodes",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::find_nodes",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "find_nodes"
    },
    {
      "block_id": "blk_0ed29f1be18e3ea3eb6a9c76",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L235-L237",
        "end_line": 237,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 235
      },
      "exported": false,
      "label": "focus",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::focus",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "focus"
    },
    {
      "block_id": "blk_c0e211bff9bdf7a3964b485d",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L193-L195",
        "end_line": 195,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 193
      },
      "exported": false,
      "label": "fork",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::fork",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "fork"
    },
    {
      "block_id": "blk_6840f423b652bfaccf16f716",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L60-L69",
        "end_line": 69,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 60
      },
      "exported": false,
      "label": "from_document",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph::from_document",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "from_document"
    },
    {
      "block_id": "blk_f86d5d4eb9f3d5e013bc730a",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L72-L79",
        "end_line": 79,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 72
      },
      "exported": false,
      "label": "from_json",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph::from_json",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "from_json"
    },
    {
      "block_id": "blk_892a22ee82a584820e2fe412",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L292-L318",
        "end_line": 318,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 292
      },
      "exported": false,
      "label": "hydrate",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::hydrate",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "hydrate"
    },
    {
      "block_id": "blk_63e24b4127af7e3656cfc294",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L82-L84",
        "end_line": 84,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 82
      },
      "exported": false,
      "label": "load",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph::load",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "load"
    },
    {
      "block_id": "blk_45926201ae4783db12cf3005",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L120-L124",
        "end_line": 124,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 120
      },
      "exported": false,
      "label": "load_session",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph::load_session",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "load_session"
    },
    {
      "block_id": "blk_03f96380badb4c7a9ccbad80",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L111-L118",
        "end_line": 118,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 111
      },
      "exported": false,
      "label": "load_session_json",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph::load_session_json",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "load_session_json"
    },
    {
      "block_id": "blk_be0d1ae3e9d34106926c6f07",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L217-L219",
        "end_line": 219,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 217
      },
      "exported": false,
      "label": "mutation_log",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::mutation_log",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "mutation_log"
    },
    {
      "block_id": "blk_2a608d5b737669d0236977ca",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L680-L691",
        "end_line": 691,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 680
      },
      "exported": false,
      "label": "parse_detail_level",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::parse_detail_level",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "parse_detail_level"
    },
    {
      "block_id": "blk_2ed28bc0ecf94a98a3abf8d8",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L693-L703",
        "end_line": 703,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 693
      },
      "exported": false,
      "label": "parse_expand_mode",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::parse_expand_mode",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "parse_expand_mode"
    },
    {
      "block_id": "blk_21f4a5661777ccf212b9f8e4",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L526-L538",
        "end_line": 538,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 526
      },
      "exported": false,
      "label": "path_between",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::path_between",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "path_between"
    },
    {
      "block_id": "blk_784a35d353a7d823c1b99894",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L167-L184",
        "end_line": 184,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 167
      },
      "exported": false,
      "label": "path_between",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph::path_between",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "path_between"
    },
    {
      "block_id": "blk_693117591acd95a13eac805b",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L337-L342",
        "end_line": 342,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 337
      },
      "exported": false,
      "label": "pin",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::pin",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "pin"
    },
    {
      "block_id": "blk_3c251425db60e648a729cc79",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L345-L347",
        "end_line": 347,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 345
      },
      "exported": false,
      "label": "prune",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::prune",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "prune"
    },
    {
      "block_id": "blk_3441ccb7868c65f1827afbfa",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L400-L402",
        "end_line": 402,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 400
      },
      "exported": false,
      "label": "recommendations",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::recommendations",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "recommendations"
    },
    {
      "block_id": "blk_8c1fa717865c666ef220e78f",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L617-L626",
        "end_line": 626,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 617
      },
      "exported": false,
      "label": "render_config",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::render_config",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "render_config"
    },
    {
      "block_id": "blk_a08afb9e5715f06b8ad193d7",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L376-L378",
        "end_line": 378,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 376
      },
      "exported": false,
      "label": "render_prompt",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::render_prompt",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "render_prompt"
    },
    {
      "block_id": "blk_5f3705479d11adecb7d1088c",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L126-L128",
        "end_line": 128,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 126
      },
      "exported": false,
      "label": "resolve",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph::resolve",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "resolve"
    },
    {
      "block_id": "blk_830f9a92137632e6f54664d1",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L86-L88",
        "end_line": 88,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 86
      },
      "exported": false,
      "label": "save",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph::save",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "save"
    },
    {
      "block_id": "blk_eea27e6ce6c5edbde5c910c1",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L213-L215",
        "end_line": 215,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 213
      },
      "exported": false,
      "label": "save",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::save",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "save"
    },
    {
      "block_id": "blk_e848a83c5c067cd4d1f8ad6b",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L230-L232",
        "end_line": 232,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 230
      },
      "exported": false,
      "label": "seed_overview",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::seed_overview",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "seed_overview"
    },
    {
      "block_id": "blk_787f1a2fff8e3dddbee70ce8",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L240-L249",
        "end_line": 249,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 240
      },
      "exported": false,
      "label": "select",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::select",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "select"
    },
    {
      "block_id": "blk_ec36abcffb9678a3cf128766",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L197-L203",
        "end_line": 203,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 197
      },
      "exported": false,
      "label": "selected_block_ids",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::selected_block_ids",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "selected_block_ids"
    },
    {
      "block_id": "blk_5b489477d0f998dd479e7da4",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L101-L105",
        "end_line": 105,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 101
      },
      "exported": false,
      "label": "session",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph::session",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "session"
    },
    {
      "block_id": "blk_dc9715f257a1cb61d904d46b",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L205-L207",
        "end_line": 207,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 205
      },
      "exported": false,
      "label": "session_id",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::session_id",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "session_id"
    },
    {
      "block_id": "blk_a8f0390566b4fa73e9f5e61a",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L225-L227",
        "end_line": 227,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 225
      },
      "exported": false,
      "label": "summary",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::summary",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "summary"
    },
    {
      "block_id": "blk_8b275cb61f4cb153561a46db",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L97-L99",
        "end_line": 99,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 97
      },
      "exported": false,
      "label": "to_document",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph::to_document",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "to_document"
    },
    {
      "block_id": "blk_28770004b1629472361f2e77",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L90-L95",
        "end_line": 95,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 90
      },
      "exported": false,
      "label": "to_json",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph::to_json",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "to_json"
    },
    {
      "block_id": "blk_2b0cd303659222ea6470a04b",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L209-L211",
        "end_line": 211,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 209
      },
      "exported": false,
      "label": "to_json",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::to_json",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "to_json"
    },
    {
      "block_id": "blk_1b4e8cc985e3edb9b2df44f4",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L705-L707",
        "end_line": 707,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 705
      },
      "exported": false,
      "label": "to_runtime_error",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::to_runtime_error",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "to_runtime_error"
    },
    {
      "block_id": "blk_a1838d0e8bb1bdaa7f359010",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L709-L711",
        "end_line": 711,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 709
      },
      "exported": false,
      "label": "to_value_error",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::to_value_error",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "to_value_error"
    },
    {
      "block_id": "blk_01879ee26c040df0e808a035",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L448-L450",
        "end_line": 450,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 448
      },
      "exported": false,
      "label": "why_pruned",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::why_pruned",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "why_pruned"
    },
    {
      "block_id": "blk_0453c0395bb420a86f180509",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L404-L409",
        "end_line": 409,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 404
      },
      "exported": false,
      "label": "why_selected",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession::why_selected",
      "node_class": "symbol",
      "path": "crates/ucp-python/src/codegraph.rs",
      "symbol_name": "why_selected"
    }
  ],
  "explanation": "Selector resolved via path to blk_d25655ca10b46913a4589e25.",
  "match_kind": "path",
  "resolved_block_id": "d25655ca10b46913a4589e25",
  "selector": "crates/ucp-python/src/codegraph.rs"
}
```

## Expand File Update

```json
{
  "added": [
    "blk_ca19df6efb93453bc3518777",
    "blk_4db23b2c636638e099ba1716",
    "blk_183a3c18b3cd680dbd76c053",
    "blk_b244fc9d04b2363f1f16775e",
    "blk_c65bb9fcd9a2fb78fe727e73",
    "blk_ea6deba2d66fdae85f758184",
    "blk_4251423dff95f312138c2607",
    "blk_4ac20768c8331bfa0dfb0ebf",
    "blk_4e63644d5a83a5582e5cc1f1",
    "blk_2a608d5b737669d0236977ca",
    "blk_2ed28bc0ecf94a98a3abf8d8",
    "blk_8c1fa717865c666ef220e78f",
    "blk_1b4e8cc985e3edb9b2df44f4",
    "blk_a1838d0e8bb1bdaa7f359010"
  ],
  "changed": [],
  "focus": "blk_ca19df6efb93453bc3518777",
  "removed": [],
  "telemetry": [
    {
      "budget": {
        "max_nodes_visited": 16
      },
      "elapsed_ms": 168,
      "focus_after": "blk_ca19df6efb93453bc3518777",
      "focus_before": "blk_99b62cfb483779db26d3be75",
      "kind": "expand_file",
      "nodes_added": [
        "ca19df6efb93453bc3518777",
        "4db23b2c636638e099ba1716",
        "183a3c18b3cd680dbd76c053",
        "b244fc9d04b2363f1f16775e",
        "c65bb9fcd9a2fb78fe727e73",
        "ea6deba2d66fdae85f758184",
        "4251423dff95f312138c2607",
        "4ac20768c8331bfa0dfb0ebf",
        "4e63644d5a83a5582e5cc1f1",
        "2a608d5b737669d0236977ca",
        "2ed28bc0ecf94a98a3abf8d8",
        "8c1fa717865c666ef220e78f",
        "1b4e8cc985e3edb9b2df44f4",
        "a1838d0e8bb1bdaa7f359010"
      ],
      "operation": "expand_file",
      "reason": "Expanded blk_ca19df6efb93453bc3518777 via File traversal.",
      "resolved_block_ids": [
        "ca19df6efb93453bc3518777"
      ],
      "selector": "crates/ucp-python/src/codegraph.rs",
      "sequence": 2,
      "target_block_id": "ca19df6efb93453bc3518777",
      "traversal": {
        "budget": {
          "max_nodes_visited": 16
        },
        "depth": 1
      }
    }
  ],
  "warnings": []
}
```

## Expansion Estimate

```json
{
  "budget": {
    "max_nodes_visited": 8
  },
  "estimated_export_growth": 0,
  "estimated_frontier_width": 3,
  "estimated_nodes_added": 0,
  "estimated_nodes_changed": 1,
  "estimated_nodes_visited": 1,
  "estimated_rendered_bytes": 0,
  "estimated_rendered_tokens": 0,
  "explanation": "Estimated Dependencies expansion for symbol:crates/ucp-python/src/codegraph.rs::build_traversal by simulating the mutation on a forked session.",
  "operation": "dependencies",
  "resolved_block_ids": [
    "4ac20768c8331bfa0dfb0ebf"
  ],
  "selector": "symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
  "target_block_id": "4ac20768c8331bfa0dfb0ebf"
}
```

## Dependency Walk Update

```json
{
  "added": [],
  "changed": [
    "blk_4ac20768c8331bfa0dfb0ebf"
  ],
  "focus": "blk_4ac20768c8331bfa0dfb0ebf",
  "removed": [],
  "telemetry": [
    {
      "elapsed_ms": 169,
      "focus_after": "blk_4ac20768c8331bfa0dfb0ebf",
      "focus_before": "blk_ca19df6efb93453bc3518777",
      "kind": "expand_dependencies",
      "nodes_changed": [
        "4ac20768c8331bfa0dfb0ebf"
      ],
      "operation": "expand_dependencies",
      "reason": "Expanded blk_4ac20768c8331bfa0dfb0ebf via Dependencies traversal.",
      "resolved_block_ids": [
        "4ac20768c8331bfa0dfb0ebf"
      ],
      "selector": "symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
      "sequence": 3,
      "target_block_id": "4ac20768c8331bfa0dfb0ebf",
      "traversal": {
        "depth": 1
      }
    }
  ],
  "warnings": []
}
```

## Compact Export

```json
{
  "edges": [],
  "export_mode": "compact",
  "focus": "blk_4ac20768c8331bfa0dfb0ebf",
  "focus_label": "symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
  "focus_short_id": "S7",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "blk_4ac20768c8331bfa0dfb0ebf",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
      "explanation": "Hydrating symbol:crates/ucp-python/src/codegraph.rs::build_traversal will surface an anchored source excerpt for the focused symbol",
      "priority": 121,
      "short_id": "S7"
    },
    {
      "action": "expand_dependents",
      "block_id": "blk_4ac20768c8331bfa0dfb0ebf",
      "candidate_count": 2,
      "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
      "direction": "incoming",
      "explanation": "2 hidden incoming candidates remain for symbol:crates/ucp-python/src/codegraph.rs::build_traversal via uses_symbol",
      "priority": 70,
      "relation": "uses_symbol",
      "short_id": "S7"
    },
    {
      "action": "collapse",
      "block_id": "blk_4ac20768c8331bfa0dfb0ebf",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-python/src/codegraph.rs::build_traversal from working set",
      "explanation": "Collapse removes symbol:crates/ucp-python/src/codegraph.rs::build_traversal from the active working set when the current branch is no longer useful",
      "priority": 6,
      "short_id": "S7"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 2,
    "low_value_candidate_count": 0,
    "recommendations": [
      {
        "action_kind": "hydrate_source",
        "candidate_count": 1,
        "estimated_evidence_gain": 4,
        "estimated_hydration_bytes": 576,
        "estimated_token_cost": 144,
        "explanation": "Hydrate source for symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
        "priority": 121,
        "rationale": "Hydrating symbol:crates/ucp-python/src/codegraph.rs::build_traversal will surface an anchored source excerpt for the focused symbol",
        "target_block_id": "4ac20768c8331bfa0dfb0ebf",
        "target_label": "symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
        "target_short_id": "S7"
      },
      {
        "action_kind": "expand_dependents",
        "candidate_count": 2,
        "estimated_evidence_gain": 2,
        "estimated_hydration_bytes": 0,
        "estimated_token_cost": 48,
        "explanation": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
        "priority": 70,
        "rationale": "2 hidden incoming candidates remain for symbol:crates/ucp-python/src/codegraph.rs::build_traversal via uses_symbol",
        "relation_set": [
          "uses_symbol"
        ],
        "target_block_id": "4ac20768c8331bfa0dfb0ebf",
        "target_label": "symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
        "target_short_id": "S7"
      },
      {
        "action_kind": "collapse",
        "candidate_count": 1,
        "estimated_evidence_gain": 1,
        "estimated_hydration_bytes": 0,
        "estimated_token_cost": 24,
        "explanation": "Collapse symbol:crates/ucp-python/src/codegraph.rs::build_traversal from working set",
        "priority": 6,
        "rationale": "Collapse removes symbol:crates/ucp-python/src/codegraph.rs::build_traversal from the active working set when the current branch is no longer useful",
        "target_block_id": "4ac20768c8331bfa0dfb0ebf",
        "target_label": "symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
        "target_short_id": "S7"
      }
    ],
    "recommended_actions": [
      {
        "action": "hydrate_source",
        "block_id": "blk_4ac20768c8331bfa0dfb0ebf",
        "candidate_count": 1,
        "description": "Hydrate source for symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
        "explanation": "Hydrating symbol:crates/ucp-python/src/codegraph.rs::build_traversal will surface an anchored source excerpt for the focused symbol",
        "priority": 121,
        "short_id": "S7"
      },
      {
        "action": "expand_dependents",
        "block_id": "blk_4ac20768c8331bfa0dfb0ebf",
        "candidate_count": 2,
        "description": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
        "direction": "incoming",
        "explanation": "2 hidden incoming candidates remain for symbol:crates/ucp-python/src/codegraph.rs::build_traversal via uses_symbol",
        "priority": 70,
        "relation": "uses_symbol",
        "short_id": "S7"
      },
      {
        "action": "collapse",
        "block_id": "blk_4ac20768c8331bfa0dfb0ebf",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-python/src/codegraph.rs::build_traversal from working set",
        "explanation": "Collapse removes symbol:crates/ucp-python/src/codegraph.rs::build_traversal from the active working set when the current branch is no longer useful",
        "priority": 6,
        "short_id": "S7"
      }
    ],
    "recommended_next_action": {
      "action": "hydrate_source",
      "block_id": "blk_4ac20768c8331bfa0dfb0ebf",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
      "explanation": "Hydrating symbol:crates/ucp-python/src/codegraph.rs::build_traversal will surface an anchored source excerpt for the focused symbol",
      "priority": 121,
      "short_id": "S7"
    },
    "should_stop": false
  },
  "hidden_levels": [
    {
      "count": 1,
      "direction": "manual",
      "level": 1
    },
    {
      "count": 12,
      "direction": "structural",
      "level": 2,
      "relation": "contains_symbol"
    }
  ],
  "hidden_unreachable_count": 28,
  "nodes": [
    {
      "block_id": "blk_4ac20768c8331bfa0dfb0ebf",
      "coderef": {
        "display": "crates/ucp-python/src/codegraph.rs#L596-L615",
        "end_line": 615,
        "path": "crates/ucp-python/src/codegraph.rs",
        "start_line": 596
      },
      "detail_level": "neighborhood",
      "distance_from_focus": 0,
      "label": "symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
      "logical_key": "symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
      "node_class": "symbol",
      "origin": {
        "kind": "manual"
      },
      "path": "crates/ucp-python/src/codegraph.rs",
      "pinned": false,
      "relevance_score": 214,
      "short_id": "S7",
      "signature": "function build_traversal(relation: Option<String>, relations: Option<Vec<String>>, depth: usize, max_add: Option<usize>, priority_threshold: Option<u16>, budget: Option<ucp_api::CodeGraphOperationBudget>) -> ucp_api::CodeGraphTraversalConfig",
      "symbol_name": "build_traversal"
    }
  ],
  "omissions": {
    "details": [
      {
        "block_id": "blk_d5685d95a7229a5fb2b8a2e3",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "scripts/demo_ucp_graph_runtime.py",
        "reason": "visible_level_limit",
        "short_id": "F10"
      },
      {
        "block_id": "blk_27dd45b9f4f423b3da20d031",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "crates/ucp-codegraph",
        "reason": "visible_level_limit",
        "short_id": "D9"
      },
      {
        "block_id": "blk_d6f3398b3d61e14dd798624a",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "scripts/demo_codegraph_query_recipes.py",
        "reason": "visible_level_limit",
        "short_id": "F7"
      },
      {
        "block_id": "blk_35d1be9f87753900056d83e5",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": ".",
        "reason": "visible_level_limit",
        "short_id": "R1"
      },
      {
        "block_id": "blk_4e63644d5a83a5582e5cc1f1",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "symbol:crates/ucp-python/src/codegraph.rs::export_config",
        "reason": "visible_level_limit",
        "short_id": "S8"
      },
      {
        "block_id": "blk_99b62cfb483779db26d3be75",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "crates",
        "reason": "visible_level_limit",
        "short_id": "D1"
      },
      {
        "block_id": "blk_c65bb9fcd9a2fb78fe727e73",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession#192",
        "reason": "visible_level_limit",
        "short_id": "S4"
      },
      {
        "block_id": "blk_54c55b45f69468fdb717d1f2",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "crates/ucp-agent",
        "reason": "visible_level_limit",
        "short_id": "D6"
      },
      {
        "block_id": "blk_3febc0ada88477685bc61d98",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "crates/ucp-python",
        "reason": "visible_level_limit",
        "short_id": "D13"
      },
      {
        "block_id": "blk_d559ae668b1892ddb93331ff",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "scripts/demo_codegraph_query_benchmarks.py",
        "reason": "visible_level_limit",
        "short_id": "F5"
      },
      {
        "block_id": "blk_73114ea8e67a3c5edd51420d",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "scripts/log_helper.py",
        "reason": "visible_level_limit",
        "short_id": "F12"
      },
      {
        "block_id": "blk_e75297e577b337ca2fb68a7a",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "crates/ucp-wasm",
        "reason": "visible_level_limit",
        "short_id": "D14"
      },
      {
        "block_id": "blk_8c1fa717865c666ef220e78f",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "symbol:crates/ucp-python/src/codegraph.rs::render_config",
        "reason": "visible_level_limit",
        "short_id": "S11"
      },
      {
        "block_id": "blk_1463cfacf2bd8c2604b76bde",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "crates/ucm-core",
        "reason": "visible_level_limit",
        "short_id": "D4"
      },
      {
        "block_id": "blk_6c752b79da406a1c951e3a8b",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "scripts",
        "reason": "visible_level_limit",
        "short_id": "D15"
      },
      {
        "block_id": "blk_ca19df6efb93453bc3518777",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "crates/ucp-python/src/codegraph.rs",
        "reason": "visible_level_limit",
        "short_id": "F1"
      },
      {
        "block_id": "blk_dcc23fc72145f5f411e3743a",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "scripts/demo_codegraph_query_tool_wrapper.py",
        "reason": "visible_level_limit",
        "short_id": "F8"
      },
      {
        "block_id": "blk_b07c00f9cb6f24a8023c3cf2",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "scripts/check_version_sync.py",
        "reason": "visible_level_limit",
        "short_id": "F2"
      },
      {
        "block_id": "blk_a1838d0e8bb1bdaa7f359010",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "symbol:crates/ucp-python/src/codegraph.rs::to_value_error",
        "reason": "visible_level_limit",
        "short_id": "S13"
      },
      {
        "block_id": "blk_2a608d5b737669d0236977ca",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "symbol:crates/ucp-python/src/codegraph.rs::parse_detail_level",
        "reason": "visible_level_limit",
        "short_id": "S9"
      },
      {
        "block_id": "blk_d2cef3475e8ab2b5ed3ec587",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "crates/ucp-observe",
        "reason": "visible_level_limit",
        "short_id": "D12"
      },
      {
        "block_id": "blk_a0430a0079b41b34257572b6",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "scripts/demo_ucp_python_query.py",
        "reason": "visible_level_limit",
        "short_id": "F11"
      },
      {
        "block_id": "blk_7cf2a27244897a4b9b334414",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "crates/ucl-parser",
        "reason": "visible_level_limit",
        "short_id": "D3"
      },
      {
        "block_id": "blk_2ed28bc0ecf94a98a3abf8d8",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "symbol:crates/ucp-python/src/codegraph.rs::parse_expand_mode",
        "reason": "visible_level_limit",
        "short_id": "S10"
      },
      {
        "block_id": "blk_4bc1ec7b7ec959cf48f09aba",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "scripts/demo_codegraph_query_edge_cases.py",
        "reason": "visible_level_limit",
        "short_id": "F6"
      },
      {
        "block_id": "blk_3043405a7c5239bd734b2298",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "crates/ucp-cli",
        "reason": "visible_level_limit",
        "short_id": "D8"
      },
      {
        "block_id": "blk_183a3c18b3cd680dbd76c053",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph#20",
        "reason": "visible_level_limit",
        "short_id": "S2"
      },
      {
        "block_id": "blk_ea6deba2d66fdae85f758184",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "symbol:crates/ucp-python/src/codegraph.rs::build_budget",
        "reason": "visible_level_limit",
        "short_id": "S5"
      },
      {
        "block_id": "blk_3eebb1e9a0bdbbe648be878e",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "scripts/demo_codegraph_session_observability.py",
        "reason": "visible_level_limit",
        "short_id": "F9"
      },
      {
        "block_id": "blk_4251423dff95f312138c2607",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "symbol:crates/ucp-python/src/codegraph.rs::build_find_query",
        "reason": "visible_level_limit",
        "short_id": "S6"
      },
      {
        "block_id": "blk_6a0e503cbb91ad4b20bf7083",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "crates/ucm-engine",
        "reason": "visible_level_limit",
        "short_id": "D5"
      },
      {
        "block_id": "blk_b244fc9d04b2363f1f16775e",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraphSession",
        "reason": "visible_level_limit",
        "short_id": "S3"
      },
      {
        "block_id": "blk_7fd21c1a401565bdaa0db690",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "scripts/demo_codegraph_python_query.py",
        "reason": "visible_level_limit",
        "short_id": "F4"
      },
      {
        "block_id": "blk_da1e5784051bb4cf30e6d66b",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "scripts/release.py",
        "reason": "visible_level_limit",
        "short_id": "F13"
      },
      {
        "block_id": "blk_123d8e68e78ebec442607b86",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "crates/ucp-graph",
        "reason": "visible_level_limit",
        "short_id": "D10"
      },
      {
        "block_id": "blk_11f2222350db8eee5bc5ef71",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "crates/translators",
        "reason": "visible_level_limit",
        "short_id": "D2"
      },
      {
        "block_id": "blk_1b4e8cc985e3edb9b2df44f4",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "symbol:crates/ucp-python/src/codegraph.rs::to_runtime_error",
        "reason": "visible_level_limit",
        "short_id": "S12"
      },
      {
        "block_id": "blk_686b241bbbc1ebd533b4fb9c",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "crates/ucp-llm",
        "reason": "visible_level_limit",
        "short_id": "D11"
      },
      {
        "block_id": "blk_4db23b2c636638e099ba1716",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "symbol:crates/ucp-python/src/codegraph.rs::PyCodeGraph",
        "reason": "visible_level_limit",
        "short_id": "S1"
      },
      {
        "block_id": "blk_8deca5103a3b10e631b1fa62",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "crates/ucp-api",
        "reason": "visible_level_limit",
        "short_id": "D7"
      },
      {
        "block_id": "blk_27181c8b52e4b06023cc9774",
        "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
        "label": "scripts/demo_codegraph_context_walk.py",
        "reason": "visible_level_limit",
        "short_id": "F3"
      }
    ],
    "dropped_by_render_budget": 0,
    "excluded_by_class_filters": 0,
    "hidden_by_visible_levels": 41,
    "suppressed_by_hydrated_excerpt": 0
  },
  "omitted_symbol_count": 5454,
  "summary": {
    "directories": 15,
    "files": 13,
    "hydrated_sources": 0,
    "max_selected": 48,
    "repositories": 1,
    "selected": 42,
    "symbols": 13
  },
  "total_selected_edges": 0,
  "visible_levels": 0,
  "visible_node_count": 1
}
```

## Export Omission Explanation

```json
{
  "block_id": "blk_8c1fa717865c666ef220e78f",
  "detail": {
    "block_id": "blk_8c1fa717865c666ef220e78f",
    "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
    "label": "symbol:crates/ucp-python/src/codegraph.rs::render_config",
    "reason": "visible_level_limit",
    "short_id": "S11"
  },
  "explanation": "Node is outside the visible level budget from the current focus (visible_levels=0).",
  "omitted": true,
  "selector": "symbol:crates/ucp-python/src/codegraph.rs::render_config"
}
```

## Prune Explanation

```json
{
  "block_id": "blk_8c1fa717865c666ef220e78f",
  "explanation": "Node was removed while applying prune policy after prune.",
  "pruned": true,
  "selector": "symbol:crates/ucp-python/src/codegraph.rs::render_config"
}
```

## Recommendations

```json
[
  {
    "action_kind": "hydrate_source",
    "candidate_count": 1,
    "estimated_evidence_gain": 4,
    "estimated_hydration_bytes": 576,
    "estimated_token_cost": 144,
    "explanation": "Hydrate source for symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
    "priority": 121,
    "rationale": "Hydrating symbol:crates/ucp-python/src/codegraph.rs::build_traversal will surface an anchored source excerpt for the focused symbol",
    "target_block_id": "4ac20768c8331bfa0dfb0ebf",
    "target_label": "symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
    "target_short_id": "S1"
  },
  {
    "action_kind": "expand_dependents",
    "candidate_count": 2,
    "estimated_evidence_gain": 2,
    "estimated_hydration_bytes": 0,
    "estimated_token_cost": 48,
    "explanation": "expand_dependents incoming neighbors via uses_symbol for symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
    "priority": 70,
    "rationale": "2 hidden incoming candidates remain for symbol:crates/ucp-python/src/codegraph.rs::build_traversal via uses_symbol",
    "relation_set": [
      "uses_symbol"
    ],
    "target_block_id": "4ac20768c8331bfa0dfb0ebf",
    "target_label": "symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
    "target_short_id": "S1"
  },
  {
    "action_kind": "collapse",
    "candidate_count": 1,
    "estimated_evidence_gain": 1,
    "estimated_hydration_bytes": 0,
    "estimated_token_cost": 24,
    "explanation": "Collapse symbol:crates/ucp-python/src/codegraph.rs::build_traversal from working set",
    "priority": 6,
    "rationale": "Collapse removes symbol:crates/ucp-python/src/codegraph.rs::build_traversal from the active working set when the current branch is no longer useful",
    "target_block_id": "4ac20768c8331bfa0dfb0ebf",
    "target_label": "symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
    "target_short_id": "S1"
  }
]
```

## Mutation Log

```json
[
  {
    "elapsed_ms": 82,
    "focus_after": "blk_99b62cfb483779db26d3be75",
    "kind": "seed_overview",
    "nodes_added": [
      "99b62cfb483779db26d3be75",
      "11f2222350db8eee5bc5ef71",
      "7cf2a27244897a4b9b334414",
      "1463cfacf2bd8c2604b76bde",
      "6a0e503cbb91ad4b20bf7083",
      "54c55b45f69468fdb717d1f2",
      "8deca5103a3b10e631b1fa62",
      "3043405a7c5239bd734b2298",
      "27dd45b9f4f423b3da20d031",
      "123d8e68e78ebec442607b86",
      "686b241bbbc1ebd533b4fb9c",
      "d2cef3475e8ab2b5ed3ec587",
      "3febc0ada88477685bc61d98",
      "e75297e577b337ca2fb68a7a",
      "6c752b79da406a1c951e3a8b",
      "b07c00f9cb6f24a8023c3cf2",
      "27181c8b52e4b06023cc9774",
      "7fd21c1a401565bdaa0db690",
      "d559ae668b1892ddb93331ff",
      "4bc1ec7b7ec959cf48f09aba",
      "d6f3398b3d61e14dd798624a",
      "dcc23fc72145f5f411e3743a",
      "3eebb1e9a0bdbbe648be878e",
      "d5685d95a7229a5fb2b8a2e3",
      "a0430a0079b41b34257572b6",
      "73114ea8e67a3c5edd51420d",
      "da1e5784051bb4cf30e6d66b",
      "35d1be9f87753900056d83e5"
    ],
    "operation": "seed_overview",
    "reason": "Seeded structural overview up to depth 3",
    "resolved_block_ids": [
      "99b62cfb483779db26d3be75",
      "11f2222350db8eee5bc5ef71",
      "7cf2a27244897a4b9b334414",
      "1463cfacf2bd8c2604b76bde",
      "6a0e503cbb91ad4b20bf7083",
      "54c55b45f69468fdb717d1f2",
      "8deca5103a3b10e631b1fa62",
      "3043405a7c5239bd734b2298",
      "27dd45b9f4f423b3da20d031",
      "123d8e68e78ebec442607b86",
      "686b241bbbc1ebd533b4fb9c",
      "d2cef3475e8ab2b5ed3ec587",
      "3febc0ada88477685bc61d98",
      "e75297e577b337ca2fb68a7a",
      "6c752b79da406a1c951e3a8b",
      "b07c00f9cb6f24a8023c3cf2",
      "27181c8b52e4b06023cc9774",
      "7fd21c1a401565bdaa0db690",
      "d559ae668b1892ddb93331ff",
      "4bc1ec7b7ec959cf48f09aba",
      "d6f3398b3d61e14dd798624a",
      "dcc23fc72145f5f411e3743a",
      "3eebb1e9a0bdbbe648be878e",
      "d5685d95a7229a5fb2b8a2e3",
      "a0430a0079b41b34257572b6",
      "73114ea8e67a3c5edd51420d",
      "da1e5784051bb4cf30e6d66b",
      "35d1be9f87753900056d83e5"
    ],
    "sequence": 1
  },
  {
    "budget": {
      "max_nodes_visited": 16
    },
    "elapsed_ms": 168,
    "focus_after": "blk_ca19df6efb93453bc3518777",
    "focus_before": "blk_99b62cfb483779db26d3be75",
    "kind": "expand_file",
    "nodes_added": [
      "ca19df6efb93453bc3518777",
      "4db23b2c636638e099ba1716",
      "183a3c18b3cd680dbd76c053",
      "b244fc9d04b2363f1f16775e",
      "c65bb9fcd9a2fb78fe727e73",
      "ea6deba2d66fdae85f758184",
      "4251423dff95f312138c2607",
      "4ac20768c8331bfa0dfb0ebf",
      "4e63644d5a83a5582e5cc1f1",
      "2a608d5b737669d0236977ca",
      "2ed28bc0ecf94a98a3abf8d8",
      "8c1fa717865c666ef220e78f",
      "1b4e8cc985e3edb9b2df44f4",
      "a1838d0e8bb1bdaa7f359010"
    ],
    "operation": "expand_file",
    "reason": "Expanded blk_ca19df6efb93453bc3518777 via File traversal.",
    "resolved_block_ids": [
      "ca19df6efb93453bc3518777"
    ],
    "selector": "crates/ucp-python/src/codegraph.rs",
    "sequence": 2,
    "target_block_id": "ca19df6efb93453bc3518777",
    "traversal": {
      "budget": {
        "max_nodes_visited": 16
      },
      "depth": 1
    }
  },
  {
    "elapsed_ms": 169,
    "focus_after": "blk_4ac20768c8331bfa0dfb0ebf",
    "focus_before": "blk_ca19df6efb93453bc3518777",
    "kind": "expand_dependencies",
    "nodes_changed": [
      "4ac20768c8331bfa0dfb0ebf"
    ],
    "operation": "expand_dependencies",
    "reason": "Expanded blk_4ac20768c8331bfa0dfb0ebf via Dependencies traversal.",
    "resolved_block_ids": [
      "4ac20768c8331bfa0dfb0ebf"
    ],
    "selector": "symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
    "sequence": 3,
    "target_block_id": "4ac20768c8331bfa0dfb0ebf",
    "traversal": {
      "depth": 1
    }
  },
  {
    "elapsed_ms": 85,
    "focus_after": "blk_4ac20768c8331bfa0dfb0ebf",
    "focus_before": "blk_4ac20768c8331bfa0dfb0ebf",
    "kind": "prune",
    "nodes_changed": [
      "ca19df6efb93453bc3518777"
    ],
    "nodes_removed": [
      "ea6deba2d66fdae85f758184",
      "c65bb9fcd9a2fb78fe727e73",
      "b244fc9d04b2363f1f16775e",
      "a1838d0e8bb1bdaa7f359010",
      "8c1fa717865c666ef220e78f",
      "4e63644d5a83a5582e5cc1f1",
      "4db23b2c636638e099ba1716",
      "4251423dff95f312138c2607",
      "2ed28bc0ecf94a98a3abf8d8",
      "2a608d5b737669d0236977ca",
      "1b4e8cc985e3edb9b2df44f4",
      "183a3c18b3cd680dbd76c053",
      "dcc23fc72145f5f411e3743a",
      "da1e5784051bb4cf30e6d66b",
      "d6f3398b3d61e14dd798624a",
      "d5685d95a7229a5fb2b8a2e3",
      "d559ae668b1892ddb93331ff",
      "b07c00f9cb6f24a8023c3cf2",
      "a0430a0079b41b34257572b6",
      "7fd21c1a401565bdaa0db690",
      "73114ea8e67a3c5edd51420d",
      "4bc1ec7b7ec959cf48f09aba",
      "3eebb1e9a0bdbbe648be878e",
      "27181c8b52e4b06023cc9774",
      "e75297e577b337ca2fb68a7a",
      "d2cef3475e8ab2b5ed3ec587",
      "99b62cfb483779db26d3be75",
      "8deca5103a3b10e631b1fa62",
      "7cf2a27244897a4b9b334414",
      "6c752b79da406a1c951e3a8b",
      "6a0e503cbb91ad4b20bf7083",
      "686b241bbbc1ebd533b4fb9c",
      "54c55b45f69468fdb717d1f2",
      "3febc0ada88477685bc61d98"
    ],
    "operation": "prune",
    "reason": "Applied prune policy with max_selected=8.",
    "sequence": 4
  }
]
```

## Event Log

```json
[
  {
    "event": "mutation",
    "mutation": {
      "elapsed_ms": 82,
      "focus_after": "blk_99b62cfb483779db26d3be75",
      "kind": "seed_overview",
      "nodes_added": [
        "99b62cfb483779db26d3be75",
        "11f2222350db8eee5bc5ef71",
        "7cf2a27244897a4b9b334414",
        "1463cfacf2bd8c2604b76bde",
        "6a0e503cbb91ad4b20bf7083",
        "54c55b45f69468fdb717d1f2",
        "8deca5103a3b10e631b1fa62",
        "3043405a7c5239bd734b2298",
        "27dd45b9f4f423b3da20d031",
        "123d8e68e78ebec442607b86",
        "686b241bbbc1ebd533b4fb9c",
        "d2cef3475e8ab2b5ed3ec587",
        "3febc0ada88477685bc61d98",
        "e75297e577b337ca2fb68a7a",
        "6c752b79da406a1c951e3a8b",
        "b07c00f9cb6f24a8023c3cf2",
        "27181c8b52e4b06023cc9774",
        "7fd21c1a401565bdaa0db690",
        "d559ae668b1892ddb93331ff",
        "4bc1ec7b7ec959cf48f09aba",
        "d6f3398b3d61e14dd798624a",
        "dcc23fc72145f5f411e3743a",
        "3eebb1e9a0bdbbe648be878e",
        "d5685d95a7229a5fb2b8a2e3",
        "a0430a0079b41b34257572b6",
        "73114ea8e67a3c5edd51420d",
        "da1e5784051bb4cf30e6d66b",
        "35d1be9f87753900056d83e5"
      ],
      "operation": "seed_overview",
      "reason": "Seeded structural overview up to depth 3",
      "resolved_block_ids": [
        "99b62cfb483779db26d3be75",
        "11f2222350db8eee5bc5ef71",
        "7cf2a27244897a4b9b334414",
        "1463cfacf2bd8c2604b76bde",
        "6a0e503cbb91ad4b20bf7083",
        "54c55b45f69468fdb717d1f2",
        "8deca5103a3b10e631b1fa62",
        "3043405a7c5239bd734b2298",
        "27dd45b9f4f423b3da20d031",
        "123d8e68e78ebec442607b86",
        "686b241bbbc1ebd533b4fb9c",
        "d2cef3475e8ab2b5ed3ec587",
        "3febc0ada88477685bc61d98",
        "e75297e577b337ca2fb68a7a",
        "6c752b79da406a1c951e3a8b",
        "b07c00f9cb6f24a8023c3cf2",
        "27181c8b52e4b06023cc9774",
        "7fd21c1a401565bdaa0db690",
        "d559ae668b1892ddb93331ff",
        "4bc1ec7b7ec959cf48f09aba",
        "d6f3398b3d61e14dd798624a",
        "dcc23fc72145f5f411e3743a",
        "3eebb1e9a0bdbbe648be878e",
        "d5685d95a7229a5fb2b8a2e3",
        "a0430a0079b41b34257572b6",
        "73114ea8e67a3c5edd51420d",
        "da1e5784051bb4cf30e6d66b",
        "35d1be9f87753900056d83e5"
      ],
      "sequence": 1
    }
  },
  {
    "event": "mutation",
    "mutation": {
      "budget": {
        "max_nodes_visited": 16
      },
      "elapsed_ms": 168,
      "focus_after": "blk_ca19df6efb93453bc3518777",
      "focus_before": "blk_99b62cfb483779db26d3be75",
      "kind": "expand_file",
      "nodes_added": [
        "ca19df6efb93453bc3518777",
        "4db23b2c636638e099ba1716",
        "183a3c18b3cd680dbd76c053",
        "b244fc9d04b2363f1f16775e",
        "c65bb9fcd9a2fb78fe727e73",
        "ea6deba2d66fdae85f758184",
        "4251423dff95f312138c2607",
        "4ac20768c8331bfa0dfb0ebf",
        "4e63644d5a83a5582e5cc1f1",
        "2a608d5b737669d0236977ca",
        "2ed28bc0ecf94a98a3abf8d8",
        "8c1fa717865c666ef220e78f",
        "1b4e8cc985e3edb9b2df44f4",
        "a1838d0e8bb1bdaa7f359010"
      ],
      "operation": "expand_file",
      "reason": "Expanded blk_ca19df6efb93453bc3518777 via File traversal.",
      "resolved_block_ids": [
        "ca19df6efb93453bc3518777"
      ],
      "selector": "crates/ucp-python/src/codegraph.rs",
      "sequence": 2,
      "target_block_id": "ca19df6efb93453bc3518777",
      "traversal": {
        "budget": {
          "max_nodes_visited": 16
        },
        "depth": 1
      }
    }
  },
  {
    "event": "mutation",
    "mutation": {
      "elapsed_ms": 169,
      "focus_after": "blk_4ac20768c8331bfa0dfb0ebf",
      "focus_before": "blk_ca19df6efb93453bc3518777",
      "kind": "expand_dependencies",
      "nodes_changed": [
        "4ac20768c8331bfa0dfb0ebf"
      ],
      "operation": "expand_dependencies",
      "reason": "Expanded blk_4ac20768c8331bfa0dfb0ebf via Dependencies traversal.",
      "resolved_block_ids": [
        "4ac20768c8331bfa0dfb0ebf"
      ],
      "selector": "symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
      "sequence": 3,
      "target_block_id": "4ac20768c8331bfa0dfb0ebf",
      "traversal": {
        "depth": 1
      }
    }
  },
  {
    "event": "mutation",
    "mutation": {
      "elapsed_ms": 85,
      "focus_after": "blk_4ac20768c8331bfa0dfb0ebf",
      "focus_before": "blk_4ac20768c8331bfa0dfb0ebf",
      "kind": "prune",
      "nodes_changed": [
        "ca19df6efb93453bc3518777"
      ],
      "nodes_removed": [
        "ea6deba2d66fdae85f758184",
        "c65bb9fcd9a2fb78fe727e73",
        "b244fc9d04b2363f1f16775e",
        "a1838d0e8bb1bdaa7f359010",
        "8c1fa717865c666ef220e78f",
        "4e63644d5a83a5582e5cc1f1",
        "4db23b2c636638e099ba1716",
        "4251423dff95f312138c2607",
        "2ed28bc0ecf94a98a3abf8d8",
        "2a608d5b737669d0236977ca",
        "1b4e8cc985e3edb9b2df44f4",
        "183a3c18b3cd680dbd76c053",
        "dcc23fc72145f5f411e3743a",
        "da1e5784051bb4cf30e6d66b",
        "d6f3398b3d61e14dd798624a",
        "d5685d95a7229a5fb2b8a2e3",
        "d559ae668b1892ddb93331ff",
        "b07c00f9cb6f24a8023c3cf2",
        "a0430a0079b41b34257572b6",
        "7fd21c1a401565bdaa0db690",
        "73114ea8e67a3c5edd51420d",
        "4bc1ec7b7ec959cf48f09aba",
        "3eebb1e9a0bdbbe648be878e",
        "27181c8b52e4b06023cc9774",
        "e75297e577b337ca2fb68a7a",
        "d2cef3475e8ab2b5ed3ec587",
        "99b62cfb483779db26d3be75",
        "8deca5103a3b10e631b1fa62",
        "7cf2a27244897a4b9b334414",
        "6c752b79da406a1c951e3a8b",
        "6a0e503cbb91ad4b20bf7083",
        "686b241bbbc1ebd533b4fb9c",
        "54c55b45f69468fdb717d1f2",
        "3febc0ada88477685bc61d98"
      ],
      "operation": "prune",
      "reason": "Applied prune policy with max_selected=8.",
      "sequence": 4
    }
  },
  {
    "event": "session_saved",
    "metadata": {
      "graph_snapshot_hash": "972c90ea9d6130e2de0e03eb6e7a76b429618a29178c006cf5e7be8a61be20a1",
      "mutation_count": 4,
      "schema_version": "codegraph_session.v1",
      "session_id": "cgs_a218099ca065ed81",
      "session_snapshot_hash": "e48cb020dc1eff0588b43e6f8d74acefd34db05dbe51f0691e190ad5a8b19545"
    }
  }
]
```

## Restored Session Summary

```json
{
  "selected_block_ids": [
    "blk_11f2222350db8eee5bc5ef71",
    "blk_123d8e68e78ebec442607b86",
    "blk_1463cfacf2bd8c2604b76bde",
    "blk_27dd45b9f4f423b3da20d031",
    "blk_3043405a7c5239bd734b2298",
    "blk_35d1be9f87753900056d83e5",
    "blk_4ac20768c8331bfa0dfb0ebf",
    "blk_ca19df6efb93453bc3518777"
  ],
  "session_id": "cgs_a218099ca065ed81",
  "summary": {
    "directories": 5,
    "files": 1,
    "hydrated_sources": 0,
    "max_selected": 8,
    "repositories": 1,
    "selected": 8,
    "symbols": 1
  }
}
```

## Final summary

- session id: `cgs_a218099ca065ed81`
- restored session id: `cgs_a218099ca065ed81`
- transcript: `/home/antonio/programming/Hivemind/unified-content-protocol/artifacts/codegraph-session-observability-demo-transcript.md`
