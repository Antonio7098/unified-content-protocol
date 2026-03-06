## Codegraph context demo transcript

Chosen refactor candidate: deduplicate codegraph context/session helper logic across `agent.rs` and `codegraph.rs`, using bounded depth and selected-edge traversal.

## Build a codegraph for the current repository

`$ cargo run -q -p ucp-cli -- codegraph build /home/antonio/programming/Hivemind/unified-content-protocol --commit 01fb33c5 --output /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --allow-partial --format json`

```text
{
  "status": "partial_success",
  "profile_version": "codegraph.v1",
  "canonical_fingerprint": "7e6a75f0dc8ae09718b27237ea121e7f5ce5ec76ad2ff2e802569eb2c6580434",
  "stats": {
    "total_nodes": 5085,
    "repository_nodes": 1,
    "directory_nodes": 51,
    "file_nodes": 146,
    "symbol_nodes": 4887,
    "total_edges": 5286,
    "reference_edges": 606,
    "export_edges": 1556,
    "languages": {
      "javascript": 2,
      "python": 20,
      "rust": 119,
      "typescript": 5
    }
  },
  "diagnostics": [
    {
      "severity": "info",
      "code": "CG2001",
      "message": "no symbols extracted for crates/ucp-python/python/ucp/__init__.py",
      "path": "crates/ucp-python/python/ucp/__init__.py"
    },
    {
      "severity": "warning",
      "code": "CG2006",
      "message": "unresolved import '.missing'",
      "path": "crates/ucp-api/tests/fixtures/edge-cases/py/main.py"
    },
    {
      "severity": "warning",
      "code": "CG2006",
      "message": "unresolved import 'crate::missing::Thing'",
      "path": "crates/ucp-api/tests/fixtures/edge-cases/src/lib.rs"
    },
    {
      "severity": "warning",
      "code": "CG2006",
      "message": "unresolved import './not-here'",
      "path": "crates/ucp-api/tests/fixtures/edge-cases/web.ts"
    },
    {
      "severity": "warning",
      "code": "CG2006",
      "message": "unresolved import 'mod:analyze'",
      "path": "crates/ucp-codegraph/src/legacy.rs"
    },
    {
      "severity": "warning",
      "code": "CG2006",
      "message": "unresolved import 'mod:build'",
      "path": "crates/ucp-codegraph/src/legacy.rs"
    },
    {
      "severity": "warning",
      "code": "CG2006",
      "message": "unresolved import 'mod:canonical'",
      "path": "crates/ucp-codegraph/src/legacy.rs"
    },
    {
      "severity": "warning",
      "code": "CG2006",
      "message": "unresolved import 'mod:extract'",
      "path": "crates/ucp-codegraph/src/legacy.rs"
    },
    {
      "severity": "warning",
      "code": "CG2006",
      "message": "unresolved import 'mod:filesystem'",
      "path": "crates/ucp-codegraph/src/legacy.rs"
    },
    {
      "severity": "warning",
      "code": "CG2006",
      "message": "unresolved import 'mod:resolve'",
      "path": "crates/ucp-codegraph/src/legacy.rs"
... clipped 355986 more lines ...
```

## Initialize a stateful codegraph context session from a shallow structural overview

`$ cargo run -q -p ucp-cli -- codegraph context init --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --name demo_context_walk --max-selected 512 --initial-depth 1 --format json`

```text
{
  "initial_depth": 1,
  "name": "demo_context_walk",
  "rendered": "CodeGraph working set\nfocus: [R1] .\nsummary: selected=1/512 repositories=1 directories=0 files=0 symbols=0 hydrated=0\n\nfilesystem:\n- [R1] .\n\nomissions:\n- symbols omitted from working set: 4887\n- prune policy: max_selected=512 demote_before_remove=true protect_focus=true\n\nfrontier:\n- set focus to a file or symbol to expand the working set",
  "session_id": "cgctx_2954b48f",
  "success": true,
  "summary": {
    "directories": 0,
    "files": 0,
    "hydrated_sources": 0,
    "max_selected": 512,
    "repositories": 1,
    "selected": 1,
    "symbols": 0
  }
}
```

Session: `cgctx_2954b48f`

## Show the compact initial working set

`$ cargo run -q -p ucp-cli -- codegraph context show --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --format json`

```text
{
  "edges": [],
  "export_mode": "compact",
  "focus": "cc437144455b1eb51ad2b4fb",
  "focus_label": ".",
  "focus_short_id": "R1",
  "frontier": [],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "should_stop": true
  },
  "hidden_unreachable_count": 0,
  "nodes": [
    {
      "block_id": "cc437144455b1eb51ad2b4fb",
      "coderef": {
        "display": "unified-content-protocol",
        "path": "."
      },
      "detail_level": "skeleton",
      "distance_from_focus": 0,
      "label": ".",
      "logical_key": "repository:unified-content-protocol",
      "node_class": "repository",
      "origin": {
        "kind": "overview"
      },
      "pinned": false,
      "relevance_score": 162,
      "short_id": "R1"
    }
  ],
  "omitted_symbol_count": 4887,
  "session": "cgctx_2954b48f",
  "summary": {
    "directories": 0,
    "files": 0,
    "hydrated_sources": 0,
    "max_selected": 512,
    "repositories": 1,
    "selected": 1,
    "symbols": 0
  },
  "total_selected_edges": 0,
  "visible_node_count": 1
}
```

## Expand file symbols for crates/ucp-cli/src/commands/codegraph.rs with nested depth

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f crates/ucp-cli/src/commands/codegraph.rs --mode file --depth 2 --format json`

```text
{
  "added": [
    "blk_e4fc6ac96af7e1f5b261fe1f",
    "blk_01a6bcfd6027e7c8448f86c9",
    "blk_4eab2a70e654d15901773975",
    "blk_e7430e47ba61f5d6c6a7d1c4",
    "blk_51a2efe3948f185fe290a8bf",
    "blk_aabc66a9e95dbf452d4f4613",
    "blk_837e09bad6550e46bace4a0c",
    "blk_b8e7c31e596f118dc2f8d083",
    "blk_dafefbc02143660b373ee955",
    "blk_c79890c85fe5faf91c8a7bf7",
    "blk_cceb4cc74ec1ce320bae469d",
    "blk_36c6cd2279755163100bf577",
    "blk_3d83acf3c51e902b8087654d",
    "blk_1f5ae3e27b5b0f20e0f9b5ee",
    "blk_4575c49867d0bcbc45988dcd",
    "blk_b140afd9b657deaf709f6e5d",
    "blk_e73115ef157f15828c2ebafc",
    "blk_e9c19d163d3cf8ad8aba5ac5",
    "blk_b247ea26ba584a905ca58359",
    "blk_051222f35bcc94c4be0537f6",
    "blk_1a8b9ab804e1f7020acb63f7",
    "blk_26ef3c0fa7c9a4a7784f90d6",
    "blk_27f4ccceeb41ed54a5107271",
    "blk_a7a2ed9003c74392c2ebf001",
    "blk_fab7f9f02d8794949b02cdac",
    "blk_0ca661b39f87b6f1b753c066",
    "blk_589955bff6b8e0e6b508ec75",
    "blk_7e87a17494fe023f695509e7",
    "blk_b8b98240323d547c1b12566b",
    "blk_ba40fb7f92e5bd19f4e30563",
    "blk_2996b03d1a9ed53635e7a627"
  ],
  "changed": [],
  "focus": "blk_e4fc6ac96af7e1f5b261fe1f",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 32,
  "warnings": []
}
```

## Expand file symbols for crates/ucp-cli/src/commands/agent.rs with nested depth

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f crates/ucp-cli/src/commands/agent.rs --mode file --depth 2 --format json`

```text
{
  "added": [
    "blk_421333df5881c48ef6b4be16",
    "blk_6235791ed9ac4d7a14aac319",
    "blk_a0f46eea06efb92ade78319a",
    "blk_68ac17cf0b72cc53bcdd5398",
    "blk_90c791014a3ff4d7147bc4ae",
    "blk_999218ae981f6f86a4e6c555",
    "blk_b7e3d6028d7afb93335e0c0e",
    "blk_2d14aa6d8516a3c73a19a37b",
    "blk_4dc709203576a0c9a19d147e",
    "blk_748c018c0ff255072e8400a4",
    "blk_6bdfa08751d5a549d77dfeae",
    "blk_ea22c75b78e8959e70d7f4cc",
    "blk_b2bc11f492b1d0c47f16e483",
    "blk_33c464f1fa016a2a873ce023",
    "blk_f97e4beae70a52e156496818",
    "blk_c9c42d87b5039c025547ddd3",
    "blk_caa0b63234a21634ac29a13c",
    "blk_388c43cdf82cf3be4c3a2de3",
    "blk_d26b84666d6b90b00dc8abb9",
    "blk_ce4deb0d861f1c377b1fb833",
    "blk_b418c0b106a76e2a816dc89d",
    "blk_4ad997efb31de3ed7b077bf8",
    "blk_97aed61a3a17944ab15efac8",
    "blk_4e5834618eba57dc2705bf21",
    "blk_2f2ff11e7823297e4a3a58ca",
    "blk_038a0d36c4bfe68f61ae207d",
    "blk_dabdd7fa1f031b5b15d50adf",
    "blk_62e8cfd89f05690ea589c053",
    "blk_4d6beda7db26a12be6f96d3e",
    "blk_36297514b84ae49ca30429cf",
    "blk_76e903e84d62e60151a2636f",
    "blk_2067cb932c99686289c1870c",
    "blk_4ebeb74d6c99a6cdd8434440",
    "blk_e1461da0092b93ba436329f5",
    "blk_e82bc834beb1e93b6beb9320",
    "blk_1c2377fdb5c6e58265653ec2",
    "blk_7573ba527b717f92b1f9ff37",
    "blk_e804bec6a3d59ded23434300",
    "blk_a03f786951e9056adeb7d847",
    "blk_9cb55c80be4d1e6e672635b5",
    "blk_fcbaff9c723b3c545893b460",
    "blk_252a08abfd351262785f1af7",
    "blk_83fe331988dd1204ee617928",
    "blk_83c18aaef4cd71a1c2471814"
  ],
  "changed": [],
  "focus": "blk_421333df5881c48ef6b4be16",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 76,
  "warnings": []
}
```

## Expand file symbols for crates/ucp-cli/src/state.rs with nested depth

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f crates/ucp-cli/src/state.rs --mode file --depth 2 --format json`

```text
{
  "added": [
    "blk_b93da718e1d46a0927bc4074",
    "blk_8200e68767cfb0ef6ab7ba79",
    "blk_188d0a5a3eecda3b89bd6663",
    "blk_bff602b27a6fde8e2ffa5c83",
    "blk_8ba9a2e4b16e2822a83ba3ee",
    "blk_c781cee8feef9b724dc5267b",
    "blk_701d4a158570aac82367aa83",
    "blk_e226efa27ee49766852deed2",
    "blk_e171de84c619ae784f69d034",
    "blk_3454f118d408655205b7a81f",
    "blk_94facbc10fc4336f57a1edfa",
    "blk_6bef18c64e4b48ec6b16199d",
    "blk_d70999acf47b26fb7b615535",
    "blk_8812e3cd9c2deda7699c27ff",
    "blk_935b8bb38a66bb3fea2a47ba",
    "blk_e91699dcdbeccecf2e3faacc",
    "blk_70389eb140a9e9e83cee012f",
    "blk_01b4e5ae4df4fbc19afe29a5",
    "blk_43ae50f0ee2859a5b67c342f",
    "blk_76fa95f3966e235ded59deed",
    "blk_0e56da010393704a8f24cfa0",
    "blk_3782dd2be141e3c55305983d",
    "blk_0341e8199fff1053a0ca9a7b",
    "blk_9299e9547b8d0efa11291bb1",
    "blk_512af7304477aedf697093ad",
    "blk_bb9d761faf75e2521a2bed72",
    "blk_98cecc35fdf3a26f89ae1e7d",
    "blk_5fdffd711aa011e9e9dd39f5",
    "blk_4b61c9e588579964859333a2",
    "blk_12d1227a2646bf2521e28c13",
    "blk_62e3b255e283d9c6baa7b484",
    "blk_9361af0ca23f02a772c132a2",
    "blk_abe9f9fe7bff58e60a927f86",
    "blk_de8bedf545b1c01ca7b6b53c",
    "blk_2101ee03c207a0ddda697bad",
    "blk_49300b9d09e65882cc0d62c2",
    "blk_be7bebb2d9258b0b192a1428",
    "blk_7fc9bfbca39635630447d5c3",
    "blk_185ce4556c9cf59fe54872f7",
    "blk_22f7c81a012a56168d1cf22c",
    "blk_170b3ff154cc6352ec177cc3",
    "blk_8ffe45e5aae6a3fbb5854f5c",
    "blk_ac4e71d14344917a11ee62d3",
    "blk_4766ffa34f47f97e704daea8"
  ],
  "changed": [],
  "focus": "blk_b93da718e1d46a0927bc4074",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 120,
  "warnings": []
}
```

## Export the structured working set after file expansion

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "388c43cdf82cf3be4c3a2de3",
      "target_short_id": "S27"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S105"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "e91699dcdbeccecf2e3faacc",
      "target_short_id": "S116"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8200e68767cfb0ef6ab7ba79",
      "target_short_id": "S74"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "188d0a5a3eecda3b89bd6663",
      "target_short_id": "S75"
    },
    {
      "multiplicity": 3,
      "relation": "references",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "b93da718e1d46a0927bc4074",
      "target_short_id": "F3"
    },
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "e9c19d163d3cf8ad8aba5ac5",
      "target_short_id": "S61"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S105"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "e91699dcdbeccecf2e3faacc",
      "target_short_id": "S116"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8200e68767cfb0ef6ab7ba79",
... clipped 3982 more lines ...
```

### Seed symbols

- `symbol:crates/ucp-cli/src/commands/agent.rs::context_show`
- `symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut`
- `symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update`
- `symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector`
- `symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show`
- `symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut`
- `symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update`
- `symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector`

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::context_show

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/agent.rs::context_show --format json`

```text
{
  "added": [],
  "changed": [
    "blk_ea22c75b78e8959e70d7f4cc"
  ],
  "focus": "blk_ea22c75b78e8959e70d7f4cc",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 120,
  "warnings": []
}
```

## Show +1 levels around symbol:crates/ucp-cli/src/commands/agent.rs::context_show

`$ cargo run -q -p ucp-cli -- codegraph context show --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --levels 1 --format text`

```text
visible nodes: 5 of 120 selected
focus window: +1 levels
+ 41 nodes at level 2
+ 50 nodes at level 3
+ 23 nodes at level 4
+ 1 selected nodes disconnected from focus
next: hydrate_source *
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::context_show (+1 visible level)

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --levels 1 --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S105"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "ea22c75b78e8959e70d7f4cc",
      "source_short_id": "S14",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S105"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "d26b84666d6b90b00dc8abb9",
      "source_short_id": "S28",
      "target": "ea22c75b78e8959e70d7f4cc",
      "target_short_id": "S14"
    }
  ],
  "export_mode": "compact",
  "focus": "ea22c75b78e8959e70d7f4cc",
  "focus_label": "symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
  "focus_short_id": "S14",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "ea22c75b78e8959e70d7f4cc",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
      "priority": 121,
      "short_id": "S14"
    },
    {
      "action": "expand_dependencies",
      "block_id": "ea22c75b78e8959e70d7f4cc",
      "candidate_count": 1,
      "description": "expand_dependencies outgoing neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
      "direction": "outgoing",
      "priority": 94,
      "relation": "uses_symbol",
      "short_id": "S14"
    },
    {
      "action": "collapse",
      "block_id": "ea22c75b78e8959e70d7f4cc",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/commands/agent.rs::context_show from working set",
      "priority": 6,
      "short_id": "S14"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 1,
    "low_value_candidate_count": 0,
    "recommended_actions": [
      {
        "action": "hydrate_source",
        "block_id": "ea22c75b78e8959e70d7f4cc",
        "candidate_count": 1,
        "description": "Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
        "priority": 121,
        "short_id": "S14"
      },
      {
        "action": "expand_dependencies",
        "block_id": "ea22c75b78e8959e70d7f4cc",
        "candidate_count": 1,
        "description": "expand_dependencies outgoing neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/agent.rs::context_show",
        "direction": "outgoing",
        "priority": 94,
        "relation": "uses_symbol",
        "short_id": "S14"
... clipped 159 more lines ...
```

## Expand dependencies for symbol:crates/ucp-cli/src/commands/agent.rs::context_show via uses_symbol (+2 hops)

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/agent.rs::context_show --mode dependencies --relations uses_symbol --depth 2 --format json`

```text
{
  "added": [
    "blk_6557d3b244263e4971245831"
  ],
  "changed": [
    "blk_ea22c75b78e8959e70d7f4cc"
  ],
  "focus": "blk_ea22c75b78e8959e70d7f4cc",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 121,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::context_show

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/agent.rs::context_show --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_ea22c75b78e8959e70d7f4cc",
    "blk_ea22c75b78e8959e70d7f4cc"
  ],
  "focus": "blk_ea22c75b78e8959e70d7f4cc",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 121,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::context_show

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "388c43cdf82cf3be4c3a2de3",
      "target_short_id": "S27"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S106"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "e91699dcdbeccecf2e3faacc",
      "target_short_id": "S117"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "6557d3b244263e4971245831",
      "target_short_id": "S74"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8200e68767cfb0ef6ab7ba79",
      "target_short_id": "S75"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "188d0a5a3eecda3b89bd6663",
      "target_short_id": "S76"
    },
    {
      "multiplicity": 3,
      "relation": "references",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "b93da718e1d46a0927bc4074",
      "target_short_id": "F3"
    },
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "e9c19d163d3cf8ad8aba5ac5",
      "target_short_id": "S61"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S106"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "e91699dcdbeccecf2e3faacc",
... clipped 4035 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut --format json`

```text
{
  "added": [],
  "changed": [
    "blk_c9c42d87b5039c025547ddd3"
  ],
  "focus": "blk_c9c42d87b5039c025547ddd3",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 121,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut (+1 visible level)

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --levels 1 --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "4dc709203576a0c9a19d147e",
      "source_short_id": "S10",
      "target": "c9c42d87b5039c025547ddd3",
      "target_short_id": "S24"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "748c018c0ff255072e8400a4",
      "source_short_id": "S11",
      "target": "c9c42d87b5039c025547ddd3",
      "target_short_id": "S24"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "6bdfa08751d5a549d77dfeae",
      "source_short_id": "S13",
      "target": "c9c42d87b5039c025547ddd3",
      "target_short_id": "S24"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "a0f46eea06efb92ade78319a",
      "source_short_id": "S3",
      "target": "c9c42d87b5039c025547ddd3",
      "target_short_id": "S24"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "68ac17cf0b72cc53bcdd5398",
      "source_short_id": "S5",
      "target": "c9c42d87b5039c025547ddd3",
      "target_short_id": "S24"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "90c791014a3ff4d7147bc4ae",
      "source_short_id": "S6",
      "target": "c9c42d87b5039c025547ddd3",
      "target_short_id": "S24"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "999218ae981f6f86a4e6c555",
      "source_short_id": "S7",
      "target": "c9c42d87b5039c025547ddd3",
      "target_short_id": "S24"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "b7e3d6028d7afb93335e0c0e",
      "source_short_id": "S8",
      "target": "c9c42d87b5039c025547ddd3",
      "target_short_id": "S24"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "2d14aa6d8516a3c73a19a37b",
      "source_short_id": "S9",
      "target": "c9c42d87b5039c025547ddd3",
      "target_short_id": "S24"
    }
  ],
  "export_mode": "compact",
  "focus": "c9c42d87b5039c025547ddd3",
  "focus_label": "symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut",
  "focus_short_id": "S24",
  "frontier": [
... clipped 322 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_c9c42d87b5039c025547ddd3",
    "blk_c9c42d87b5039c025547ddd3"
  ],
  "focus": "blk_c9c42d87b5039c025547ddd3",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 121,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "388c43cdf82cf3be4c3a2de3",
      "target_short_id": "S27"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S106"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "e91699dcdbeccecf2e3faacc",
      "target_short_id": "S117"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "6557d3b244263e4971245831",
      "target_short_id": "S74"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8200e68767cfb0ef6ab7ba79",
      "target_short_id": "S75"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "188d0a5a3eecda3b89bd6663",
      "target_short_id": "S76"
    },
    {
      "multiplicity": 3,
      "relation": "references",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "b93da718e1d46a0927bc4074",
      "target_short_id": "F3"
    },
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "e9c19d163d3cf8ad8aba5ac5",
      "target_short_id": "S61"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S106"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "e91699dcdbeccecf2e3faacc",
... clipped 4032 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update --format json`

```text
{
  "added": [],
  "changed": [
    "blk_b418c0b106a76e2a816dc89d"
  ],
  "focus": "blk_b418c0b106a76e2a816dc89d",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 121,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update (+1 visible level)

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --levels 1 --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "6bdfa08751d5a549d77dfeae",
      "source_short_id": "S13",
      "target": "b418c0b106a76e2a816dc89d",
      "target_short_id": "S30"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "90c791014a3ff4d7147bc4ae",
      "source_short_id": "S6",
      "target": "b418c0b106a76e2a816dc89d",
      "target_short_id": "S30"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "999218ae981f6f86a4e6c555",
      "source_short_id": "S7",
      "target": "b418c0b106a76e2a816dc89d",
      "target_short_id": "S30"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "b7e3d6028d7afb93335e0c0e",
      "source_short_id": "S8",
      "target": "b418c0b106a76e2a816dc89d",
      "target_short_id": "S30"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "2d14aa6d8516a3c73a19a37b",
      "source_short_id": "S9",
      "target": "b418c0b106a76e2a816dc89d",
      "target_short_id": "S30"
    }
  ],
  "export_mode": "compact",
  "focus": "b418c0b106a76e2a816dc89d",
  "focus_label": "symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update",
  "focus_short_id": "S30",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "b418c0b106a76e2a816dc89d",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update",
      "priority": 121,
      "short_id": "S30"
    },
    {
      "action": "expand_dependencies",
      "block_id": "b418c0b106a76e2a816dc89d",
      "candidate_count": 1,
      "description": "expand_dependencies outgoing neighbors via uses_symbol for symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update",
      "direction": "outgoing",
      "priority": 94,
      "relation": "uses_symbol",
      "short_id": "S30"
    },
    {
      "action": "collapse",
      "block_id": "b418c0b106a76e2a816dc89d",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update from working set",
      "priority": 6,
      "short_id": "S30"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 1,
    "low_value_candidate_count": 0,
    "recommended_actions": [
      {
... clipped 222 more lines ...
```

## Expand dependencies for symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update via uses_symbol (+2 hops)

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update --mode dependencies --relations uses_symbol --depth 2 --format json`

```text
{
  "added": [
    "blk_298cd57cedbe61b98d446788"
  ],
  "changed": [
    "blk_b418c0b106a76e2a816dc89d"
  ],
  "focus": "blk_b418c0b106a76e2a816dc89d",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 122,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_b418c0b106a76e2a816dc89d",
    "blk_b418c0b106a76e2a816dc89d"
  ],
  "focus": "blk_b418c0b106a76e2a816dc89d",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 122,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "388c43cdf82cf3be4c3a2de3",
      "target_short_id": "S27"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S107"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "e91699dcdbeccecf2e3faacc",
      "target_short_id": "S118"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "6557d3b244263e4971245831",
      "target_short_id": "S74"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "298cd57cedbe61b98d446788",
      "target_short_id": "S75"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8200e68767cfb0ef6ab7ba79",
      "target_short_id": "S76"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "188d0a5a3eecda3b89bd6663",
      "target_short_id": "S77"
    },
    {
      "multiplicity": 3,
      "relation": "references",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "b93da718e1d46a0927bc4074",
      "target_short_id": "F3"
    },
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "e9c19d163d3cf8ad8aba5ac5",
      "target_short_id": "S61"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8812e3cd9c2deda7699c27ff",
... clipped 4183 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector --format json`

```text
{
  "added": [],
  "changed": [
    "blk_4ad997efb31de3ed7b077bf8"
  ],
  "focus": "blk_4ad997efb31de3ed7b077bf8",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 122,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector (+1 visible level)

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --levels 1 --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "4dc709203576a0c9a19d147e",
      "source_short_id": "S10",
      "target": "4ad997efb31de3ed7b077bf8",
      "target_short_id": "S31"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "97aed61a3a17944ab15efac8",
      "source_short_id": "S32",
      "target": "4ad997efb31de3ed7b077bf8",
      "target_short_id": "S31"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "90c791014a3ff4d7147bc4ae",
      "source_short_id": "S6",
      "target": "4ad997efb31de3ed7b077bf8",
      "target_short_id": "S31"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "999218ae981f6f86a4e6c555",
      "source_short_id": "S7",
      "target": "4ad997efb31de3ed7b077bf8",
      "target_short_id": "S31"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "b7e3d6028d7afb93335e0c0e",
      "source_short_id": "S8",
      "target": "4ad997efb31de3ed7b077bf8",
      "target_short_id": "S31"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "2d14aa6d8516a3c73a19a37b",
      "source_short_id": "S9",
      "target": "4ad997efb31de3ed7b077bf8",
      "target_short_id": "S31"
    }
  ],
  "export_mode": "compact",
  "focus": "4ad997efb31de3ed7b077bf8",
  "focus_label": "symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector",
  "focus_short_id": "S31",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "4ad997efb31de3ed7b077bf8",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector",
      "priority": 121,
      "short_id": "S31"
    },
    {
      "action": "collapse",
      "block_id": "4ad997efb31de3ed7b077bf8",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector from working set",
      "priority": 6,
      "short_id": "S31"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "recommended_actions": [
      {
        "action": "hydrate_source",
        "block_id": "4ad997efb31de3ed7b077bf8",
... clipped 232 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_4ad997efb31de3ed7b077bf8",
    "blk_4ad997efb31de3ed7b077bf8"
  ],
  "focus": "blk_4ad997efb31de3ed7b077bf8",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 122,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "388c43cdf82cf3be4c3a2de3",
      "target_short_id": "S27"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S107"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "e91699dcdbeccecf2e3faacc",
      "target_short_id": "S118"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "6557d3b244263e4971245831",
      "target_short_id": "S74"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "298cd57cedbe61b98d446788",
      "target_short_id": "S75"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8200e68767cfb0ef6ab7ba79",
      "target_short_id": "S76"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "188d0a5a3eecda3b89bd6663",
      "target_short_id": "S77"
    },
    {
      "multiplicity": 3,
      "relation": "references",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "b93da718e1d46a0927bc4074",
      "target_short_id": "F3"
    },
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "e9c19d163d3cf8ad8aba5ac5",
      "target_short_id": "S61"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8812e3cd9c2deda7699c27ff",
... clipped 4181 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show --format json`

```text
{
  "added": [],
  "changed": [
    "blk_3d83acf3c51e902b8087654d"
  ],
  "focus": "blk_3d83acf3c51e902b8087654d",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 122,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show (+1 visible level)

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --levels 1 --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S107"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "4eab2a70e654d15901773975",
      "source_short_id": "S46",
      "target": "3d83acf3c51e902b8087654d",
      "target_short_id": "S56"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "3d83acf3c51e902b8087654d",
      "source_short_id": "S56",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S107"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "3d83acf3c51e902b8087654d",
      "source_short_id": "S56",
      "target": "4575c49867d0bcbc45988dcd",
      "target_short_id": "S58"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "3d83acf3c51e902b8087654d",
      "source_short_id": "S56",
      "target": "b140afd9b657deaf709f6e5d",
      "target_short_id": "S59"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "3d83acf3c51e902b8087654d",
      "source_short_id": "S56",
      "target": "051222f35bcc94c4be0537f6",
      "target_short_id": "S64"
    }
  ],
  "export_mode": "compact",
  "focus": "3d83acf3c51e902b8087654d",
  "focus_label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
  "focus_short_id": "S56",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "3d83acf3c51e902b8087654d",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show",
      "priority": 121,
      "short_id": "S56"
    },
    {
      "action": "collapse",
      "block_id": "3d83acf3c51e902b8087654d",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show from working set",
      "priority": 6,
      "short_id": "S56"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "recommended_actions": [
      {
        "action": "hydrate_source",
        "block_id": "3d83acf3c51e902b8087654d",
... clipped 206 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_3d83acf3c51e902b8087654d",
    "blk_3d83acf3c51e902b8087654d"
  ],
  "focus": "blk_3d83acf3c51e902b8087654d",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 122,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "388c43cdf82cf3be4c3a2de3",
      "target_short_id": "S27"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S107"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "e91699dcdbeccecf2e3faacc",
      "target_short_id": "S118"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "6557d3b244263e4971245831",
      "target_short_id": "S74"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "298cd57cedbe61b98d446788",
      "target_short_id": "S75"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8200e68767cfb0ef6ab7ba79",
      "target_short_id": "S76"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "188d0a5a3eecda3b89bd6663",
      "target_short_id": "S77"
    },
    {
      "multiplicity": 3,
      "relation": "references",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "b93da718e1d46a0927bc4074",
      "target_short_id": "F3"
    },
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "e9c19d163d3cf8ad8aba5ac5",
      "target_short_id": "S61"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8812e3cd9c2deda7699c27ff",
... clipped 4180 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut --format json`

```text
{
  "added": [],
  "changed": [
    "blk_e73115ef157f15828c2ebafc"
  ],
  "focus": "blk_e73115ef157f15828c2ebafc",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 122,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut (+1 visible level)

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --levels 1 --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "e7430e47ba61f5d6c6a7d1c4",
      "source_short_id": "S47",
      "target": "e73115ef157f15828c2ebafc",
      "target_short_id": "S60"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "51a2efe3948f185fe290a8bf",
      "source_short_id": "S48",
      "target": "e73115ef157f15828c2ebafc",
      "target_short_id": "S60"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "aabc66a9e95dbf452d4f4613",
      "source_short_id": "S49",
      "target": "e73115ef157f15828c2ebafc",
      "target_short_id": "S60"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "b8e7c31e596f118dc2f8d083",
      "source_short_id": "S51",
      "target": "e73115ef157f15828c2ebafc",
      "target_short_id": "S60"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "dafefbc02143660b373ee955",
      "source_short_id": "S52",
      "target": "e73115ef157f15828c2ebafc",
      "target_short_id": "S60"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "cceb4cc74ec1ce320bae469d",
      "source_short_id": "S54",
      "target": "e73115ef157f15828c2ebafc",
      "target_short_id": "S60"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "36c6cd2279755163100bf577",
      "source_short_id": "S55",
      "target": "e73115ef157f15828c2ebafc",
      "target_short_id": "S60"
    }
  ],
  "export_mode": "compact",
  "focus": "e73115ef157f15828c2ebafc",
  "focus_label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut",
  "focus_short_id": "S60",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "e73115ef157f15828c2ebafc",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut",
      "priority": 121,
      "short_id": "S60"
    },
    {
      "action": "collapse",
      "block_id": "e73115ef157f15828c2ebafc",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut from working set",
      "priority": 6,
      "short_id": "S60"
    }
... clipped 261 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_e73115ef157f15828c2ebafc",
    "blk_e73115ef157f15828c2ebafc"
  ],
  "focus": "blk_e73115ef157f15828c2ebafc",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 122,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "388c43cdf82cf3be4c3a2de3",
      "target_short_id": "S27"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S107"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "e91699dcdbeccecf2e3faacc",
      "target_short_id": "S118"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "6557d3b244263e4971245831",
      "target_short_id": "S74"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "298cd57cedbe61b98d446788",
      "target_short_id": "S75"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8200e68767cfb0ef6ab7ba79",
      "target_short_id": "S76"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "188d0a5a3eecda3b89bd6663",
      "target_short_id": "S77"
    },
    {
      "multiplicity": 3,
      "relation": "references",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "b93da718e1d46a0927bc4074",
      "target_short_id": "F3"
    },
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "e9c19d163d3cf8ad8aba5ac5",
      "target_short_id": "S61"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8812e3cd9c2deda7699c27ff",
... clipped 4178 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update --format json`

```text
{
  "added": [],
  "changed": [
    "blk_27f4ccceeb41ed54a5107271"
  ],
  "focus": "blk_27f4ccceeb41ed54a5107271",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 122,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update (+1 visible level)

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --levels 1 --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "298cd57cedbe61b98d446788",
      "target_short_id": "S75"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "e7430e47ba61f5d6c6a7d1c4",
      "source_short_id": "S47",
      "target": "27f4ccceeb41ed54a5107271",
      "target_short_id": "S67"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "51a2efe3948f185fe290a8bf",
      "source_short_id": "S48",
      "target": "27f4ccceeb41ed54a5107271",
      "target_short_id": "S67"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "aabc66a9e95dbf452d4f4613",
      "source_short_id": "S49",
      "target": "27f4ccceeb41ed54a5107271",
      "target_short_id": "S67"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "b8e7c31e596f118dc2f8d083",
      "source_short_id": "S51",
      "target": "27f4ccceeb41ed54a5107271",
      "target_short_id": "S67"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "dafefbc02143660b373ee955",
      "source_short_id": "S52",
      "target": "27f4ccceeb41ed54a5107271",
      "target_short_id": "S67"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "cceb4cc74ec1ce320bae469d",
      "source_short_id": "S54",
      "target": "27f4ccceeb41ed54a5107271",
      "target_short_id": "S67"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "36c6cd2279755163100bf577",
      "source_short_id": "S55",
      "target": "27f4ccceeb41ed54a5107271",
      "target_short_id": "S67"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "27f4ccceeb41ed54a5107271",
      "source_short_id": "S67",
      "target": "298cd57cedbe61b98d446788",
      "target_short_id": "S75"
    }
  ],
  "export_mode": "compact",
  "focus": "27f4ccceeb41ed54a5107271",
  "focus_label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update",
  "focus_short_id": "S67",
  "frontier": [
... clipped 317 more lines ...
```

## Expand dependencies for symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update via uses_symbol (+2 hops)

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update --mode dependencies --relations uses_symbol --depth 2 --format json`

```text
{
  "added": [
    "blk_823f109791f59cbd2455992b"
  ],
  "changed": [
    "blk_27f4ccceeb41ed54a5107271"
  ],
  "focus": "blk_27f4ccceeb41ed54a5107271",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 123,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_27f4ccceeb41ed54a5107271",
    "blk_27f4ccceeb41ed54a5107271"
  ],
  "focus": "blk_27f4ccceeb41ed54a5107271",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 123,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "388c43cdf82cf3be4c3a2de3",
      "target_short_id": "S27"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S108"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "e91699dcdbeccecf2e3faacc",
      "target_short_id": "S119"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "6557d3b244263e4971245831",
      "target_short_id": "S74"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "298cd57cedbe61b98d446788",
      "target_short_id": "S75"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8200e68767cfb0ef6ab7ba79",
      "target_short_id": "S77"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "188d0a5a3eecda3b89bd6663",
      "target_short_id": "S78"
    },
    {
      "multiplicity": 3,
      "relation": "references",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "b93da718e1d46a0927bc4074",
      "target_short_id": "F3"
    },
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "e9c19d163d3cf8ad8aba5ac5",
      "target_short_id": "S61"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8812e3cd9c2deda7699c27ff",
... clipped 4226 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector --format json`

```text
{
  "added": [],
  "changed": [
    "blk_0ca661b39f87b6f1b753c066"
  ],
  "focus": "blk_0ca661b39f87b6f1b753c066",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 123,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector (+1 visible level)

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --levels 1 --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "51a2efe3948f185fe290a8bf",
      "source_short_id": "S48",
      "target": "0ca661b39f87b6f1b753c066",
      "target_short_id": "S71"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "aabc66a9e95dbf452d4f4613",
      "source_short_id": "S49",
      "target": "0ca661b39f87b6f1b753c066",
      "target_short_id": "S71"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "b8e7c31e596f118dc2f8d083",
      "source_short_id": "S51",
      "target": "0ca661b39f87b6f1b753c066",
      "target_short_id": "S71"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "dafefbc02143660b373ee955",
      "source_short_id": "S52",
      "target": "0ca661b39f87b6f1b753c066",
      "target_short_id": "S71"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "cceb4cc74ec1ce320bae469d",
      "source_short_id": "S54",
      "target": "0ca661b39f87b6f1b753c066",
      "target_short_id": "S71"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "589955bff6b8e0e6b508ec75",
      "source_short_id": "S72",
      "target": "0ca661b39f87b6f1b753c066",
      "target_short_id": "S71"
    }
  ],
  "export_mode": "compact",
  "focus": "0ca661b39f87b6f1b753c066",
  "focus_label": "symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector",
  "focus_short_id": "S71",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "0ca661b39f87b6f1b753c066",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector",
      "priority": 121,
      "short_id": "S71"
    },
    {
      "action": "collapse",
      "block_id": "0ca661b39f87b6f1b753c066",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector from working set",
      "priority": 6,
      "short_id": "S71"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "recommended_actions": [
      {
        "action": "hydrate_source",
        "block_id": "0ca661b39f87b6f1b753c066",
... clipped 231 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_0ca661b39f87b6f1b753c066",
    "blk_0ca661b39f87b6f1b753c066"
  ],
  "focus": "blk_0ca661b39f87b6f1b753c066",
  "removed": [],
  "session": "cgctx_2954b48f",
  "success": true,
  "total": 123,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "388c43cdf82cf3be4c3a2de3",
      "target_short_id": "S27"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S108"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "e91699dcdbeccecf2e3faacc",
      "target_short_id": "S119"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "6557d3b244263e4971245831",
      "target_short_id": "S74"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "298cd57cedbe61b98d446788",
      "target_short_id": "S75"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8200e68767cfb0ef6ab7ba79",
      "target_short_id": "S77"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "188d0a5a3eecda3b89bd6663",
      "target_short_id": "S78"
    },
    {
      "multiplicity": 3,
      "relation": "references",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "b93da718e1d46a0927bc4074",
      "target_short_id": "F3"
    },
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "e9c19d163d3cf8ad8aba5ac5",
      "target_short_id": "S61"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8812e3cd9c2deda7699c27ff",
... clipped 4223 more lines ...
```

## Export the final structured context

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-zcwszmuy/ucp-codegraph.json --session cgctx_2954b48f --compact --no-rendered --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "388c43cdf82cf3be4c3a2de3",
      "target_short_id": "S27"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8812e3cd9c2deda7699c27ff",
      "target_short_id": "S108"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "e91699dcdbeccecf2e3faacc",
      "target_short_id": "S119"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "6557d3b244263e4971245831",
      "target_short_id": "S74"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "298cd57cedbe61b98d446788",
      "target_short_id": "S75"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8200e68767cfb0ef6ab7ba79",
      "target_short_id": "S77"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "188d0a5a3eecda3b89bd6663",
      "target_short_id": "S78"
    },
    {
      "multiplicity": 3,
      "relation": "references",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "b93da718e1d46a0927bc4074",
      "target_short_id": "F3"
    },
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "e9c19d163d3cf8ad8aba5ac5",
      "target_short_id": "S61"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8812e3cd9c2deda7699c27ff",
... clipped 4223 more lines ...
```

## Read coderef-backed excerpts from the final working set

### S71 `symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:740-745`

```rust
 738 }
 739 
 740 fn resolve_selector(doc: &Document, selector: &str) -> Result<BlockId> {
 741     BlockId::from_str(selector)
 742         .ok()
 743         .or_else(|| resolve_codegraph_selector(doc, selector))
 744         .ok_or_else(|| anyhow!("Could not resolve selector: {}", selector))
 745 }
 746 
 747 fn get_session<'a>(stateful: &'a StatefulDocument, session: &str) -> Result<&'a AgentSessionState> {
```
### S56 `symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:402-434`

```rust
 400 }
 401 
 402 fn context_show(
 403     input: Option<String>,
 404     session: String,
 405     max_tokens: usize,
 406     compact: bool,
 407     no_rendered: bool,
 408     levels: Option<usize>,
 409     format: OutputFormat,
 410 ) -> Result<()> {
 411     let stateful = read_stateful_document(input)?;
 412     ensure_codegraph_document(&stateful.document)?;
 413     let sess = get_session(&stateful, &session)?;
 414     let context = sess
 415         .codegraph_context
 416         .as_ref()
 417         .ok_or_else(|| anyhow!("Session has no codegraph context: {}", session))?;
 418     let config = CodeGraphRenderConfig::for_max_tokens(max_tokens);
 419     let export_config = make_export_config(compact, no_rendered, levels);
 420     let export = export_codegraph_context_with_config(&stateful.document, context, &config, &export_config);
 421 
 422     match format {
 423         OutputFormat::Json => {
 424             let mut value = serde_json::to_value(&export)?;
 425             if let Some(object) = value.as_object_mut() {
 426                 object.insert("session".to_string(), serde_json::Value::String(session));
 427             }
 428             println!("{}", serde_json::to_string_pretty(&value)?);
 429         }
 430         OutputFormat::Text => println!("{}", render_context_show_text(&stateful.document, context, &config, &export)),
 431     }
 432 
 433     Ok(())
 434 }
 435 
 436 fn context_export(
```
### S60 `symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:755-764`

```rust
 753 }
 754 
 755 fn get_session_mut<'a>(
 756     stateful: &'a mut StatefulDocument,
 757     session: &str,
 758 ) -> Result<&'a mut AgentSessionState> {
 759     stateful
 760         .state_mut()
 761         .sessions
 762         .get_mut(session)
 763         .ok_or_else(|| anyhow!("Session not found: {}", session))
 764 }
 765 
 766 fn merge_updates(into: &mut CodeGraphContextUpdate, next: CodeGraphContextUpdate) {
```
### S67 `symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:774-811`

```rust
 772 }
 773 
 774 fn print_context_update(
 775     format: OutputFormat,
 776     session_id: &str,
 777     update: &CodeGraphContextUpdate,
 778     session: &AgentSessionState,
 779 ) -> Result<()> {
 780     match format {
 781         OutputFormat::Json => {
 782             println!(
 783                 "{}",
 784                 serde_json::to_string_pretty(&serde_json::json!({
 785                     "success": true,
 786                     "session": session_id,
 787                     "added": update.added.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
 788                     "removed": update.removed.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
 789                     "changed": update.changed.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
 790                     "focus": update.focus.map(|id| id.to_string()),
 791                     "warnings": update.warnings,
 792                     "total": session.context_blocks.len(),
 793                 }))?
 794             );
 795         }
 796         OutputFormat::Text => {
 797             print_success(&format!(
 798                 "Updated codegraph context {} (added {}, removed {}, changed {}, total {})",
 799                 session_id,
 800                 update.added.len(),
 801                 update.removed.len(),
 802                 update.changed.len(),
 803                 session.context_blocks.len()
 804             ));
 805             for warning in &update.warnings {
 806                 print_warning(warning);
 807             }
 808         }
 809     }
 810     Ok(())
 811 }
 812 
 813 fn uuid_short() -> String {
```
### S14 `symbol:crates/ucp-cli/src/commands/agent.rs::context_show`

- ref: `crates/ucp-cli/src/commands/agent.rs:952-1022`

```rust
 950 }
 951 
 952 fn context_show(input: Option<String>, session: String, format: OutputFormat) -> Result<()> {
 953     let stateful = read_stateful_document(input)?;
 954 
 955     let sess = stateful
 956         .state()
 957         .sessions
 958         .get(&session)
 959         .ok_or_else(|| anyhow!("Session not found: {}", session))?;
 960 
 961     if is_codegraph_document(&stateful.document) {
 962         if let Some(context) = sess.codegraph_context.as_ref() {
 963             let rendered = render_codegraph_context_prompt(
 964                 &stateful.document,
 965                 context,
 966                 &CodeGraphRenderConfig::default(),
 967             );
 968             match format {
 969                 OutputFormat::Json => {
 970                     println!("{}", serde_json::to_string_pretty(&serde_json::json!({
 971                         "session": session,
 972                         "focus": context.focus.map(|id| id.to_string()),
 973                         "summary": context.summary(&stateful.document),
 974                         "blocks": sess.context_blocks,
 975                         "rendered": rendered
 976                     }))?);
 977                 }
 978                 OutputFormat::Text => {
 979                     println!("{}", rendered);
 980                 }
 981             }
 982             return Ok(());
 983         }
 984     }
 985 
 986     match format {
 987         OutputFormat::Json => {
 988             #[derive(Serialize)]
 989             struct ContextInfo {
 990                 session: String,
 991                 blocks: Vec<String>,
 992                 count: usize,
 993             }
 994             let result = ContextInfo {
 995                 session,
 996                 blocks: sess.context_blocks.clone(),
 997                 count: sess.context_blocks.len(),
 998             };
 999             println!("{}", serde_json::to_string_pretty(&result)?);
1000         }
1001         OutputFormat::Text => {
1002             println!("{}", "Context Window:".cyan().bold());
1003             if sess.context_blocks.is_empty() {
1004                 println!("  (empty)");
1005             } else {
1006                 for id in &sess.context_blocks {
1007                     if let Ok(block_id) = BlockId::from_str(id) {
1008                         if let Some(block) = stateful.document.get_block(&block_id) {
1009                             let preview = content_preview(&block.content, 60);
1010                             let preview_line = preview.lines().next().unwrap_or("");
1011                             println!("  [{}] {}", id.yellow(), preview_line.dimmed());
1012                         } else {
1013                             println!("  [{}] (block not found)", id.yellow());
1014                         }
1015                     }
1016                 }
1017             }
1018         }
1019     }
1020 
1021     Ok(())
1022 }
1023 
1024 fn resolve_selectors(doc: &ucm_core::Document, selectors: &str) -> Result<Vec<BlockId>> {
```
### S30 `symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update`

- ref: `crates/ucp-cli/src/commands/agent.rs:1055-1092`

```rust
1053 }
1054 
1055 fn print_context_update(
1056     format: OutputFormat,
1057     session: &str,
1058     update: &ucp_api::CodeGraphContextUpdate,
1059     total: usize,
1060     text_message: &str,
1061 ) -> Result<()> {
1062     match format {
1063         OutputFormat::Json => {
1064             println!("{}", serde_json::to_string_pretty(&serde_json::json!({
1065                 "success": true,
1066                 "session": session,
1067                 "added": update.added.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
1068                 "removed": update.removed.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
1069                 "changed": update.changed.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
1070                 "focus": update.focus.map(|id| id.to_string()),
1071                 "warnings": update.warnings,
1072                 "total": total
1073             }))?);
1074         }
1075         OutputFormat::Text => {
1076             print_success(&format!(
1077                 "{} (added {}, removed {}, changed {}, total {})",
1078                 text_message,
1079                 update.added.len(),
1080                 update.removed.len(),
1081                 update.changed.len(),
1082                 total
1083             ));
1084             if !update.warnings.is_empty() {
1085                 for warning in &update.warnings {
1086                     eprintln!("warning: {}", warning);
1087                 }
1088             }
1089         }
1090     }
1091     Ok(())
1092 }
1093 
1094 fn view(input: Option<String>, session: String, mode: String, format: OutputFormat) -> Result<()> {
```
### S24 `symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut`

- ref: `crates/ucp-cli/src/commands/agent.rs:1044-1053`

```rust
1042 }
1043 
1044 fn get_session_mut<'a>(
1045     stateful: &'a mut crate::state::StatefulDocument,
1046     session: &str,
1047 ) -> Result<&'a mut AgentSessionState> {
1048     stateful
1049         .state_mut()
1050         .sessions
1051         .get_mut(session)
1052         .ok_or_else(|| anyhow!("Session not found: {}", session))
1053 }
1054 
1055 fn print_context_update(
```
### S31 `symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector`

- ref: `crates/ucp-cli/src/commands/agent.rs:1031-1042`

```rust
1029 }
1030 
1031 fn resolve_selector(doc: &ucm_core::Document, selector: &str) -> Result<BlockId> {
1032     BlockId::from_str(selector)
1033         .ok()
1034         .or_else(|| {
1035             if is_codegraph_document(doc) {
1036                 resolve_codegraph_selector(doc, selector)
1037             } else {
1038                 None
1039             }
1040         })
1041         .ok_or_else(|| anyhow!("Could not resolve selector: {}", selector))
1042 }
1043 
1044 fn get_session_mut<'a>(
```
### F2 `file:crates/ucp-cli/src/commands/codegraph.rs`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:None-None`

```rust
   1 use anyhow::{anyhow, Context, Result};
   2 use colored::Colorize;
   3 use serde::Serialize;
```
### F3 `file:crates/ucp-cli/src/state.rs`

- ref: `crates/ucp-cli/src/state.rs:None-None`

```rust
   1 //! State management for CLI sessions
   2 //!
   3 //! This module provides state persistence for agent sessions, transactions,
```
### F1 `file:crates/ucp-cli/src/commands/agent.rs`

- ref: `crates/ucp-cli/src/commands/agent.rs:None-None`

```rust
   1 //! Agent traversal commands
   2 
   3 use anyhow::{anyhow, Result};
```

## Final summary

- selected nodes: 123
- frontier actions remaining: 2
- transcript file: `artifacts/codegraph-context-demo-transcript.md`
