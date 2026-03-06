## Codegraph context demo transcript

Chosen refactor candidate: deduplicate codegraph context/session helper logic across `agent.rs` and `codegraph.rs`, using bounded depth and selected-edge traversal.

## Build a codegraph for the current repository

`$ cargo run -q -p ucp-cli -- codegraph build /home/antonio/programming/Hivemind/unified-content-protocol --commit c567b7ad --output /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --allow-partial --format json`

```text
{
  "status": "partial_success",
  "profile_version": "codegraph.v1",
  "canonical_fingerprint": "5f60d7709daf90b96af434318006995bb3b762bdc7a43986eb032bb43392a8d5",
  "stats": {
    "total_nodes": 5111,
    "repository_nodes": 1,
    "directory_nodes": 51,
    "file_nodes": 146,
    "symbol_nodes": 4913,
    "total_edges": 5337,
    "reference_edges": 608,
    "export_edges": 1561,
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
... clipped 358360 more lines ...
```

## Initialize a focus-first codegraph context session with preserved defaults

`$ cargo run -q -p ucp-cli -- codegraph context init --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --name demo_context_walk --max-selected 512 --focus crates/ucp-cli/src/commands/codegraph.rs --focus-mode file --focus-depth 2 --initial-depth 1 --default-compact --default-levels 1 --default-preset semantic --default-depth 2 --format json`

```text
{
  "focus": "crates/ucp-cli/src/commands/codegraph.rs",
  "focus_mode": "file",
  "initial_depth": 1,
  "name": "demo_context_walk",
  "preferences": {
    "compact": true,
    "expand_depth": 2,
    "levels": 1,
    "relation_preset": "semantic"
  },
  "rendered": "CodeGraph working set\nfocus: [F1] crates/ucp-cli/src/commands/codegraph.rs\nsummary: selected=40/512 repositories=1 directories=0 files=1 symbols=38 hydrated=0\n\nfilesystem:\n- [R1] .\n- [F1] crates/ucp-cli/src/commands/codegraph.rs [rust]\n\nopened symbols:\n- [S31] function parse_relation_filters(relation: Option<String>, relations: Option<String>) -> Result<Vec<String>> @ crates/ucp-cli/src/commands/codegraph.rs#L713-L722\n- [S14] function context_init(input: Option<String>, name: Option<String>, max_selected: usize, initial_depth: Option<usize>, focus: Option<String>, focus_mode: String, focus_depth: usize, preset: Option<String>, default_relations: Option<String>, default_compact: bool, default_levels: Option<usize>, default_preset: Option<String>, default_depth: Option<usize>, default_only: Option<String>, default_exclude: Option<String>, format: OutputFormat) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L433-L547\n- [S26] struct InspectResult @ crates/ucp-cli/src/commands/codegraph.rs#L209-L213\n- [S13] function context_hydrate(input: Option<String>, session: String, target: String, padding: usize, format: OutputFormat) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L1097-L1119\n- [S28] function merge_updates(into: &mut CodeGraphContextUpdate, next: CodeGraphContextUpdate) @ crates/ucp-cli/src/commands/codegraph.rs#L1227-L1233\n- [S6] function context(cmd: CodegraphContextCommands, format: OutputFormat) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L275-L431\n- [S16] function context_prune(input: Option<String>, session: String, max_selected: Option<usize>, format: OutputFormat) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L1166-L1184\n- [S20] function expand_relation_preset(name: &str) -> Result<Vec<String>> @ crates/ucp-cli/src/commands/codegraph.rs#L750-L759\n- [S19] function ensure_codegraph_document(doc: &Document) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L1186-L1192\n- [S4] function build_session_preferences(compact: bool, levels: Option<usize>, preset: Option<String>, relation_filters: Vec<String>, expand_depth: Option<usize>, only_node_classes: Vec<String>, exclude_node_classes: Vec<String>) -> Result<CodeGraphSessionPreferences> @ crates/ucp-cli/src/commands/codegraph.rs#L761-L785\n- [S27] function make_export_config(preferences: &CodeGraphSessionPreferences, compact: bool, no_rendered: bool, levels: Option<usize>, only: Option<&str>, exclude: Option<&str>) -> Result<CodeGraphExportConfig> @ crates/ucp-cli/src/commands/codegraph.rs#L635-L664\n- [S21] function get_session(stateful: &'a StatefulDocument, session: &str) -> Result<&'a AgentSessionState> @ crates/ucp-cli/src/commands/codegraph.rs#L1208-L1214\n- [S25] function inspect(input: Option<String>, format: OutputFormat) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L201-L245\n- [S22] function get_session_mut(stateful: &'a mut StatefulDocument, session: &str) -> Result<&'a mut AgentSessionState> @ crates/ucp-cli/src/commands/codegraph.rs#L1216-L1225\n- [S3] struct JsonBuildOutput @ crates/ucp-cli/src/commands/codegraph.rs#L105-L112\n- [S9] function context_expand(input: Option<String>, session: String, target: String, mode: String, relation: Option<String>, relations: Option<String>, preset: Option<String>, depth: Option<usize>, max_add: Option<usize>, priority_threshold: Option<u16>, format: OutputFormat) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L934-L980\n- [S18] function detect_commit_hash(repo: &PathBuf) -> Result<String> @ crates/ucp-cli/src/commands/codegraph.rs#L1283-L1301\n- [S38] function uuid_short() -> String @ crates/ucp-cli/src/commands/codegraph.rs#L1274-L1281\n- [S15] function context_pin(input: Option<String>, session: String, target: String, pinned: bool, format: OutputFormat) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L1145-L1164\n- [S7] function context_add(input: Option<String>, session: String, selectors: String, format: OutputFormat) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L880-L908\n- [S23] function handle(cmd: CodegraphCommands, format: OutputFormat) -> Result<()> [public] @ crates/ucp-cli/src/commands/codegraph.rs#L27-L55\n- [S8] function context_collapse(input: Option<String>, session: String, target: String, descendants: bool, format: OutputFormat) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L1121-L1143\n- [S17] function context_show(input: Option<String>, session: String, max_tokens: usize, compact: bool, no_rendered: bool, levels: Option<usize>, only: Option<String>, exclude: Option<String>, format: OutputFormat) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L549-L590\n- [S30] function parse_node_classes(raw: Option<&str>) -> Result<Vec<String>> @ crates/ucp-cli/src/commands/codegraph.rs#L739-L748\n- [S37] function resolve_selectors(doc: &Document, selectors: &str) -> Result<Vec<BlockId>> @ crates/ucp-cli/src/commands/codegraph.rs#L1194-L1199\n- [S5] function build_traversal_config(preferences: &CodeGraphSessionPreferences, relation: Option<String>, relations: Option<String>, preset: Option<String>, depth: Option<usize>, max_add: Option<usize>, priority_threshold: Option<u16>) -> Result<CodeGraphTraversalConfig> @ crates/ucp-cli/src/commands/codegraph.rs#L787-L820\n- [S1] function action_summary(action: &CodeGraphContextFrontierAction) -> String @ crates/ucp-cli/src/commands/codegraph.rs#L867-L878\n- [S35] function render_context_show_text(document: &Document, context: &ucp_api::CodeGraphContextSession, config: &CodeGraphRenderConfig, export: &CodeGraphContextExport) -> String @ crates/ucp-cli/src/commands/codegraph.rs#L666-L711\n- [S2] function build(repo: String, commit: Option<String>, output: Option<String>, extensions: Option<String>, include_hidden: bool, no_export_edges: bool, fail_on_parse_error: bool, max_file_bytes: usize, allow_partial: bool, format: OutputFormat) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L58-L199\n- [S32] function print_context_update(format: OutputFormat, session_id: &str, update: &CodeGraphContextUpdate, session: &AgentSessionState) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L1235-L1272\n- [S29] function parse_csv_arg(raw: Option<&str>) -> Result<Vec<String>> @ crates/ucp-cli/src/commands/codegraph.rs#L724-L737\n- [S12] function context_focus(input: Option<String>, session: String, target: Option<String>, format: OutputFormat) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L910-L932\n- [S11] function context_export(input: Option<String>, session: String, max_tokens: usize, compact: bool, no_rendered: bool, levels: Option<usize>, only: Option<String>, exclude: Option<String>, format: OutputFormat) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L592-L633\n- [S34] struct PromptResult @ crates/ucp-cli/src/commands/codegraph.rs#L254-L256\n- [S10] function context_expand_recommended(input: Option<String>, session: String, top: usize, padding: usize, depth: Option<usize>, max_add: Option<usize>, priority_threshold: Option<u16>, format: OutputFormat) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L982-L1095\n- [S24] function init_focus_first_context(context: &mut ucp_api::CodeGraphContextSession, document: &Document, block_id: BlockId, focus_mode: &str, traversal: &CodeGraphTraversalConfig) -> Result<CodeGraphContextUpdate> @ crates/ucp-cli/src/commands/codegraph.rs#L822-L865\n- [S36] function resolve_selector(doc: &Document, selector: &str) -> Result<BlockId> @ crates/ucp-cli/src/commands/codegraph.rs#L1201-L1206\n- [S33] function prompt(input: Option<String>, output: Option<String>, format: OutputFormat) -> Result<()> @ crates/ucp-cli/src/commands/codegraph.rs#L247-L273\n\nomissions:\n- symbols omitted from working set: 4875\n- prune policy: max_selected=512 demote_before_remove=true protect_focus=true\n\nfrontier:\n- [F1] expand file symbols\n- [F1] hydrate file source",
  "session_id": "cgctx_9f3b4e82",
  "success": true,
  "summary": {
    "directories": 0,
    "files": 1,
    "hydrated_sources": 0,
    "max_selected": 512,
    "repositories": 1,
    "selected": 40,
    "symbols": 38
  }
}
```

Session: `cgctx_9f3b4e82`

## Show the initial working set using session defaults

`$ cargo run -q -p ucp-cli -- codegraph context show --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F1",
      "target": "901076464023033c660b85dd",
      "target_short_id": "S23"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "dc7c514e09209d15c08ae3ec",
      "source_short_id": "S10",
      "target": "b3f11e914fd87b07282a683e",
      "target_short_id": "S1"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "dc7c514e09209d15c08ae3ec",
      "source_short_id": "S10",
      "target": "4450d2707a0b19c1e365db1e",
      "target_short_id": "S19"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "dc7c514e09209d15c08ae3ec",
      "source_short_id": "S10",
      "target": "541d1b88fef0ab934a94d8c4",
      "target_short_id": "S21"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "dc7c514e09209d15c08ae3ec",
      "source_short_id": "S10",
      "target": "60bd974c1a707fae06ae9bff",
      "target_short_id": "S22"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "dc7c514e09209d15c08ae3ec",
      "source_short_id": "S10",
      "target": "32de3bb2f25f16987903cf64",
      "target_short_id": "S28"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "d0c919f1477fc77181e8f5af",
      "source_short_id": "S11",
      "target": "4450d2707a0b19c1e365db1e",
      "target_short_id": "S19"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "d0c919f1477fc77181e8f5af",
      "source_short_id": "S11",
      "target": "541d1b88fef0ab934a94d8c4",
      "target_short_id": "S21"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "d0c919f1477fc77181e8f5af",
      "source_short_id": "S11",
      "target": "5417b6f5770223c411fd533e",
      "target_short_id": "S27"
    },
    {
      "multiplicity": 1,
      "relation": "uses_symbol",
      "source": "cdf346ff059bf7c77c035f1c",
      "source_short_id": "S12",
      "target": "4450d2707a0b19c1e365db1e",
... clipped 1361 more lines ...
```

## Expand file symbols for crates/ucp-cli/src/commands/codegraph.rs with nested depth

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 crates/ucp-cli/src/commands/codegraph.rs --mode file --depth 2 --format json`

```text
{
  "added": [],
  "changed": [],
  "focus": "blk_e4fc6ac96af7e1f5b261fe1f",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 40,
  "warnings": []
}
```

## Expand file symbols for crates/ucp-cli/src/commands/agent.rs with nested depth

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 crates/ucp-cli/src/commands/agent.rs --mode file --depth 2 --format json`

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
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 84,
  "warnings": []
}
```

## Expand file symbols for crates/ucp-cli/src/state.rs with nested depth

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 crates/ucp-cli/src/state.rs --mode file --depth 2 --format json`

```text
{
  "added": [
    "blk_b93da718e1d46a0927bc4074",
    "blk_b2687bfbc0028beb7be3e9e2",
    "blk_8fa37a9b28fc83a9dd03064a",
    "blk_bff602b27a6fde8e2ffa5c83",
    "blk_8ba9a2e4b16e2822a83ba3ee",
    "blk_b8185f6fe13c5fc5fea62f7a",
    "blk_893f9444f372a00bbb564ddc",
    "blk_474738725d905374a343b4f2",
    "blk_04df4e95dd82367359cc7c74",
    "blk_e935692387960b1285167912",
    "blk_8c9c75b6fb2bd8c01b77c082",
    "blk_86f9eb5f6a731caf654cfcfd",
    "blk_a46d60b85791ed274d33127c",
    "blk_2649414a6e90f853fdf95d58",
    "blk_0c6c13995a670d18a116596b",
    "blk_778fa2de138f69564248f2a6",
    "blk_ce3da84f4d25148890d96698",
    "blk_7ea20dd7acaccbccc7ae0908",
    "blk_b0bc4af0362ef61c14881c4e",
    "blk_fe1acf1e3e51fd5474b97f2b",
    "blk_520f075fcd694a0fd58dab95",
    "blk_4797cf720ffc4759c4487680",
    "blk_6aa14daf4da988d8e6db3fd4",
    "blk_00bf01bab39e0d8bfcbee58c",
    "blk_63507f8f088b182401b16d4e",
    "blk_512af7304477aedf697093ad",
    "blk_c402a2eb38a29780f8e3b2d4",
    "blk_3f5f26f70abc3a3f93287213",
    "blk_2bd2ad9d2bb8b0e03ee2dbdd",
    "blk_4aa934409ab8241ae4ced0f6",
    "blk_ebb0a31c425a9c20309e7905",
    "blk_4fa0ff7211cfe8d616905baf",
    "blk_c5ec9c271d30ea7b81985801",
    "blk_605f3691b1a46afed17ef87c",
    "blk_e33cb21ff66ab1035886e032",
    "blk_b6cdbf01af786b7f3f7914dd",
    "blk_310eda51a116db9f8773ff3a",
    "blk_722c7d869ccdb6370317d28c",
    "blk_4964cf7bc18da2e9c433bdc8",
    "blk_b30abd1436b1b1fc352c5937",
    "blk_5592eb870c96664176dcd3d8",
    "blk_2486dd3a3fcbdad081c13e19",
    "blk_f55151e975cd9f34166eaba3",
    "blk_27ae77bff49f32eb424eba84",
    "blk_43d700d07714cdae962ab70f"
  ],
  "changed": [],
  "focus": "blk_b93da718e1d46a0927bc4074",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Export the structured working set after file expansion

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "0c6c13995a670d18a116596b",
      "target_short_id": "S114"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "ce3da84f4d25148890d96698",
      "target_short_id": "S125"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8fa37a9b28fc83a9dd03064a",
      "target_short_id": "S83"
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
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "e935692387960b1285167912",
      "target_short_id": "S101"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8c9c75b6fb2bd8c01b77c082",
      "target_short_id": "S102"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "0c6c13995a670d18a116596b",
      "target_short_id": "S114"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "ce3da84f4d25148890d96698",
      "target_short_id": "S125"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "b2687bfbc0028beb7be3e9e2",
... clipped 636 more lines ...
```

### Seed symbols

- `symbol:crates/ucp-cli/src/state.rs::AgentSessionState`
- `symbol:crates/ucp-cli/src/state.rs::AgentSessionState#69`
- `symbol:crates/ucp-cli/src/state.rs::CliState`

## Focus symbol:crates/ucp-cli/src/state.rs::AgentSessionState

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 symbol:crates/ucp-cli/src/state.rs::AgentSessionState --format json`

```text
{
  "added": [],
  "changed": [
    "blk_b2687bfbc0028beb7be3e9e2"
  ],
  "focus": "blk_b2687bfbc0028beb7be3e9e2",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Show +1 levels around symbol:crates/ucp-cli/src/state.rs::AgentSessionState

`$ cargo run -q -p ucp-cli -- codegraph context show --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --levels 1 --format text`

```text
visible nodes: 5 of 129 selected
focus window: +1 levels
+ 85 nodes at level 2 via structural contains_symbol
+ 38 nodes at level 3 via structural contains_symbol
+ 1 selected nodes disconnected from focus
next: hydrate_source *
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::AgentSessionState (+1 visible level)

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --levels 1 --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8fa37a9b28fc83a9dd03064a",
      "target_short_id": "S83"
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
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8fa37a9b28fc83a9dd03064a",
      "target_short_id": "S83"
    },
    {
      "multiplicity": 5,
      "relation": "references",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "b93da718e1d46a0927bc4074",
      "target_short_id": "F3"
    },
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "b93da718e1d46a0927bc4074",
      "source_short_id": "F3",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    },
    {
      "multiplicity": 1,
      "relation": "for_type",
      "source": "8fa37a9b28fc83a9dd03064a",
      "source_short_id": "S83",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    }
  ],
  "export_mode": "compact",
  "focus": "b2687bfbc0028beb7be3e9e2",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::AgentSessionState",
  "focus_short_id": "S82",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "b2687bfbc0028beb7be3e9e2",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState",
      "priority": 121,
      "short_id": "S82"
    },
... clipped 171 more lines ...
```

## Apply the top recommended frontier action for symbol:crates/ucp-cli/src/state.rs::AgentSessionState

`$ cargo run -q -p ucp-cli -- codegraph context expand-recommended --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --top 1 --priority-threshold 60 --format json`

```text
{
  "added": [],
  "applied_actions": [
    "hydrate_source S82 (priority 121, 1 candidates)"
  ],
  "changed": [
    "blk_b2687bfbc0028beb7be3e9e2",
    "blk_b2687bfbc0028beb7be3e9e2"
  ],
  "focus": "blk_b2687bfbc0028beb7be3e9e2",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 symbol:crates/ucp-cli/src/state.rs::AgentSessionState --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_b2687bfbc0028beb7be3e9e2"
  ],
  "focus": "blk_b2687bfbc0028beb7be3e9e2",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::AgentSessionState

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8fa37a9b28fc83a9dd03064a",
      "target_short_id": "S83"
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
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8fa37a9b28fc83a9dd03064a",
      "target_short_id": "S83"
    },
    {
      "multiplicity": 5,
      "relation": "references",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "b93da718e1d46a0927bc4074",
      "target_short_id": "F3"
    },
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "b93da718e1d46a0927bc4074",
      "source_short_id": "F3",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    },
    {
      "multiplicity": 1,
      "relation": "for_type",
      "source": "8fa37a9b28fc83a9dd03064a",
      "source_short_id": "S83",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    }
  ],
  "export_mode": "compact",
  "focus": "b2687bfbc0028beb7be3e9e2",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::AgentSessionState",
  "focus_short_id": "S82",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "b2687bfbc0028beb7be3e9e2",
      "candidate_count": 0,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState",
      "priority": 120,
      "short_id": "S82"
    },
... clipped 173 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::AgentSessionState#69

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 symbol:crates/ucp-cli/src/state.rs::AgentSessionState#69 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_8fa37a9b28fc83a9dd03064a"
  ],
  "focus": "blk_8fa37a9b28fc83a9dd03064a",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::AgentSessionState#69 (+1 visible level)

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --levels 1 --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8fa37a9b28fc83a9dd03064a",
      "target_short_id": "S83"
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
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8fa37a9b28fc83a9dd03064a",
      "target_short_id": "S83"
    },
    {
      "multiplicity": 5,
      "relation": "references",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "b93da718e1d46a0927bc4074",
      "target_short_id": "F3"
    },
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "b93da718e1d46a0927bc4074",
      "source_short_id": "F3",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    },
    {
      "multiplicity": 1,
      "relation": "for_type",
      "source": "8fa37a9b28fc83a9dd03064a",
      "source_short_id": "S83",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    }
  ],
  "export_mode": "compact",
  "focus": "8fa37a9b28fc83a9dd03064a",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::AgentSessionState#69",
  "focus_short_id": "S83",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "8fa37a9b28fc83a9dd03064a",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState#69",
      "priority": 121,
      "short_id": "S83"
    },
... clipped 346 more lines ...
```

## Apply the top recommended frontier action for symbol:crates/ucp-cli/src/state.rs::AgentSessionState#69

`$ cargo run -q -p ucp-cli -- codegraph context expand-recommended --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --top 1 --priority-threshold 60 --format json`

```text
{
  "added": [],
  "applied_actions": [
    "hydrate_source S83 (priority 121, 1 candidates)"
  ],
  "changed": [
    "blk_8fa37a9b28fc83a9dd03064a",
    "blk_8fa37a9b28fc83a9dd03064a"
  ],
  "focus": "blk_8fa37a9b28fc83a9dd03064a",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState#69

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 symbol:crates/ucp-cli/src/state.rs::AgentSessionState#69 --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_8fa37a9b28fc83a9dd03064a"
  ],
  "focus": "blk_8fa37a9b28fc83a9dd03064a",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::AgentSessionState#69

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "421333df5881c48ef6b4be16",
      "source_short_id": "F1",
      "target": "8fa37a9b28fc83a9dd03064a",
      "target_short_id": "S83"
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
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    },
    {
      "multiplicity": 1,
      "relation": "imports_symbol",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "8fa37a9b28fc83a9dd03064a",
      "target_short_id": "S83"
    },
    {
      "multiplicity": 5,
      "relation": "references",
      "source": "e4fc6ac96af7e1f5b261fe1f",
      "source_short_id": "F2",
      "target": "b93da718e1d46a0927bc4074",
      "target_short_id": "F3"
    },
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "b93da718e1d46a0927bc4074",
      "source_short_id": "F3",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    },
    {
      "multiplicity": 1,
      "relation": "for_type",
      "source": "8fa37a9b28fc83a9dd03064a",
      "source_short_id": "S83",
      "target": "b2687bfbc0028beb7be3e9e2",
      "target_short_id": "S82"
    }
  ],
  "export_mode": "compact",
  "focus": "8fa37a9b28fc83a9dd03064a",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::AgentSessionState#69",
  "focus_short_id": "S83",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "8fa37a9b28fc83a9dd03064a",
      "candidate_count": 0,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState#69",
      "priority": 120,
      "short_id": "S83"
    },
... clipped 348 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::CliState

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 symbol:crates/ucp-cli/src/state.rs::CliState --format json`

```text
{
  "added": [],
  "changed": [
    "blk_bff602b27a6fde8e2ffa5c83"
  ],
  "focus": "blk_bff602b27a6fde8e2ffa5c83",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::CliState (+1 visible level)

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --levels 1 --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "b93da718e1d46a0927bc4074",
      "source_short_id": "F3",
      "target": "bff602b27a6fde8e2ffa5c83",
      "target_short_id": "S92"
    },
    {
      "multiplicity": 1,
      "relation": "for_type",
      "source": "8ba9a2e4b16e2822a83ba3ee",
      "source_short_id": "S93",
      "target": "bff602b27a6fde8e2ffa5c83",
      "target_short_id": "S92"
    }
  ],
  "export_mode": "compact",
  "focus": "bff602b27a6fde8e2ffa5c83",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::CliState",
  "focus_short_id": "S92",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "bff602b27a6fde8e2ffa5c83",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::CliState",
      "priority": 121,
      "short_id": "S92"
    },
    {
      "action": "collapse",
      "block_id": "bff602b27a6fde8e2ffa5c83",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::CliState from working set",
      "priority": 6,
      "short_id": "S92"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "recommended_actions": [
      {
        "action": "hydrate_source",
        "block_id": "bff602b27a6fde8e2ffa5c83",
        "candidate_count": 1,
        "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::CliState",
        "priority": 121,
        "short_id": "S92"
      },
      {
        "action": "collapse",
        "block_id": "bff602b27a6fde8e2ffa5c83",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/state.rs::CliState from working set",
        "priority": 6,
        "short_id": "S92"
      }
    ],
    "recommended_next_action": {
      "action": "hydrate_source",
      "block_id": "bff602b27a6fde8e2ffa5c83",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::CliState",
      "priority": 121,
      "short_id": "S92"
    },
    "should_stop": false
  },
  "hidden_levels": [
    {
      "count": 4,
      "direction": "manual",
      "level": 2
    },
    {
      "count": 13,
... clipped 98 more lines ...
```

## Apply the top recommended frontier action for symbol:crates/ucp-cli/src/state.rs::CliState

`$ cargo run -q -p ucp-cli -- codegraph context expand-recommended --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --top 1 --priority-threshold 60 --format json`

```text
{
  "added": [],
  "applied_actions": [
    "hydrate_source S92 (priority 121, 1 candidates)"
  ],
  "changed": [
    "blk_bff602b27a6fde8e2ffa5c83",
    "blk_bff602b27a6fde8e2ffa5c83"
  ],
  "focus": "blk_bff602b27a6fde8e2ffa5c83",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::CliState

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 symbol:crates/ucp-cli/src/state.rs::CliState --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_bff602b27a6fde8e2ffa5c83"
  ],
  "focus": "blk_bff602b27a6fde8e2ffa5c83",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::CliState

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --format json`

```text
{
  "edges": [
    {
      "multiplicity": 1,
      "relation": "exports",
      "source": "b93da718e1d46a0927bc4074",
      "source_short_id": "F3",
      "target": "bff602b27a6fde8e2ffa5c83",
      "target_short_id": "S92"
    },
    {
      "multiplicity": 1,
      "relation": "for_type",
      "source": "8ba9a2e4b16e2822a83ba3ee",
      "source_short_id": "S93",
      "target": "bff602b27a6fde8e2ffa5c83",
      "target_short_id": "S92"
    }
  ],
  "export_mode": "compact",
  "focus": "bff602b27a6fde8e2ffa5c83",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::CliState",
  "focus_short_id": "S92",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "bff602b27a6fde8e2ffa5c83",
      "candidate_count": 0,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::CliState",
      "priority": 120,
      "short_id": "S92"
    },
    {
      "action": "collapse",
      "block_id": "bff602b27a6fde8e2ffa5c83",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::CliState from working set",
      "priority": 6,
      "short_id": "S92"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "reasons": [
      "focus symbol is hydrated and no unselected dependency frontier remains"
    ],
    "recommended_actions": [
      {
        "action": "collapse",
        "block_id": "bff602b27a6fde8e2ffa5c83",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/state.rs::CliState from working set",
        "priority": 6,
        "short_id": "S92"
      }
    ],
    "recommended_next_action": {
      "action": "collapse",
      "block_id": "bff602b27a6fde8e2ffa5c83",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::CliState from working set",
      "priority": 6,
      "short_id": "S92"
    },
    "should_stop": true
  },
  "hidden_levels": [
    {
      "count": 4,
      "direction": "manual",
      "level": 2
    },
    {
      "count": 13,
      "direction": "structural",
      "level": 2,
      "relation": "contains_symbol"
    },
    {
... clipped 100 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context --format json`

```text
{
  "added": [],
  "changed": [
    "blk_7ea20dd7acaccbccc7ae0908"
  ],
  "focus": "blk_7ea20dd7acaccbccc7ae0908",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context (+1 visible level)

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --levels 1 --format json`

```text
{
  "edges": [],
  "export_mode": "compact",
  "focus": "7ea20dd7acaccbccc7ae0908",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context",
  "focus_short_id": "S84",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "7ea20dd7acaccbccc7ae0908",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context",
      "priority": 121,
      "short_id": "S84"
    },
    {
      "action": "collapse",
      "block_id": "7ea20dd7acaccbccc7ae0908",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context from working set",
      "priority": 6,
      "short_id": "S84"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "recommended_actions": [
      {
        "action": "hydrate_source",
        "block_id": "7ea20dd7acaccbccc7ae0908",
        "candidate_count": 1,
        "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context",
        "priority": 121,
        "short_id": "S84"
      },
      {
        "action": "collapse",
        "block_id": "7ea20dd7acaccbccc7ae0908",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context from working set",
        "priority": 6,
        "short_id": "S84"
      }
    ],
    "recommended_next_action": {
      "action": "hydrate_source",
      "block_id": "7ea20dd7acaccbccc7ae0908",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context",
      "priority": 121,
      "short_id": "S84"
    },
    "should_stop": false
  },
  "hidden_levels": [
    {
      "count": 4,
      "direction": "manual",
      "level": 2
    },
    {
      "count": 7,
      "direction": "structural",
      "level": 2,
      "relation": "contains_symbol"
    },
    {
      "count": 1,
      "direction": "manual",
      "level": 3
    },
    {
      "count": 76,
      "direction": "structural",
      "level": 3,
      "relation": "contains_symbol"
    },
    {
      "count": 38,
... clipped 65 more lines ...
```

## Apply the top recommended frontier action for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context

`$ cargo run -q -p ucp-cli -- codegraph context expand-recommended --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --top 1 --priority-threshold 60 --format json`

```text
{
  "added": [],
  "applied_actions": [
    "hydrate_source S84 (priority 121, 1 candidates)"
  ],
  "changed": [
    "blk_7ea20dd7acaccbccc7ae0908",
    "blk_7ea20dd7acaccbccc7ae0908"
  ],
  "focus": "blk_7ea20dd7acaccbccc7ae0908",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_7ea20dd7acaccbccc7ae0908"
  ],
  "focus": "blk_7ea20dd7acaccbccc7ae0908",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --format json`

```text
{
  "edges": [],
  "export_mode": "compact",
  "focus": "7ea20dd7acaccbccc7ae0908",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context",
  "focus_short_id": "S84",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "7ea20dd7acaccbccc7ae0908",
      "candidate_count": 0,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context",
      "priority": 120,
      "short_id": "S84"
    },
    {
      "action": "collapse",
      "block_id": "7ea20dd7acaccbccc7ae0908",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context from working set",
      "priority": 6,
      "short_id": "S84"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "reasons": [
      "focus symbol is hydrated and no unselected dependency frontier remains"
    ],
    "recommended_actions": [
      {
        "action": "collapse",
        "block_id": "7ea20dd7acaccbccc7ae0908",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context from working set",
        "priority": 6,
        "short_id": "S84"
      }
    ],
    "recommended_next_action": {
      "action": "collapse",
      "block_id": "7ea20dd7acaccbccc7ae0908",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::add_to_context from working set",
      "priority": 6,
      "short_id": "S84"
    },
    "should_stop": true
  },
  "hidden_levels": [
    {
      "count": 4,
      "direction": "manual",
      "level": 2
    },
    {
      "count": 7,
      "direction": "structural",
      "level": 2,
      "relation": "contains_symbol"
    },
    {
      "count": 1,
      "direction": "manual",
      "level": 3
    },
    {
      "count": 76,
      "direction": "structural",
      "level": 3,
      "relation": "contains_symbol"
    },
    {
      "count": 38,
      "direction": "structural",
      "level": 4,
      "relation": "contains_symbol"
    }
  ],
... clipped 67 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back --format json`

```text
{
  "added": [],
  "changed": [
    "blk_b0bc4af0362ef61c14881c4e"
  ],
  "focus": "blk_b0bc4af0362ef61c14881c4e",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back (+1 visible level)

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --levels 1 --format json`

```text
{
  "edges": [],
  "export_mode": "compact",
  "focus": "b0bc4af0362ef61c14881c4e",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back",
  "focus_short_id": "S85",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "b0bc4af0362ef61c14881c4e",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back",
      "priority": 121,
      "short_id": "S85"
    },
    {
      "action": "collapse",
      "block_id": "b0bc4af0362ef61c14881c4e",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back from working set",
      "priority": 6,
      "short_id": "S85"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "recommended_actions": [
      {
        "action": "hydrate_source",
        "block_id": "b0bc4af0362ef61c14881c4e",
        "candidate_count": 1,
        "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back",
        "priority": 121,
        "short_id": "S85"
      },
      {
        "action": "collapse",
        "block_id": "b0bc4af0362ef61c14881c4e",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back from working set",
        "priority": 6,
        "short_id": "S85"
      }
    ],
    "recommended_next_action": {
      "action": "hydrate_source",
      "block_id": "b0bc4af0362ef61c14881c4e",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back",
      "priority": 121,
      "short_id": "S85"
    },
    "should_stop": false
  },
  "hidden_levels": [
    {
      "count": 5,
      "direction": "manual",
      "level": 2
    },
    {
      "count": 6,
      "direction": "structural",
      "level": 2,
      "relation": "contains_symbol"
    },
    {
      "count": 1,
      "direction": "manual",
      "level": 3
    },
    {
      "count": 76,
      "direction": "structural",
      "level": 3,
      "relation": "contains_symbol"
    },
    {
      "count": 38,
... clipped 65 more lines ...
```

## Apply the top recommended frontier action for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back

`$ cargo run -q -p ucp-cli -- codegraph context expand-recommended --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --top 1 --priority-threshold 60 --format json`

```text
{
  "added": [],
  "applied_actions": [
    "hydrate_source S85 (priority 121, 1 candidates)"
  ],
  "changed": [
    "blk_b0bc4af0362ef61c14881c4e",
    "blk_b0bc4af0362ef61c14881c4e"
  ],
  "focus": "blk_b0bc4af0362ef61c14881c4e",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_b0bc4af0362ef61c14881c4e"
  ],
  "focus": "blk_b0bc4af0362ef61c14881c4e",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --format json`

```text
{
  "edges": [],
  "export_mode": "compact",
  "focus": "b0bc4af0362ef61c14881c4e",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back",
  "focus_short_id": "S85",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "b0bc4af0362ef61c14881c4e",
      "candidate_count": 0,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back",
      "priority": 120,
      "short_id": "S85"
    },
    {
      "action": "collapse",
      "block_id": "b0bc4af0362ef61c14881c4e",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back from working set",
      "priority": 6,
      "short_id": "S85"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "reasons": [
      "focus symbol is hydrated and no unselected dependency frontier remains"
    ],
    "recommended_actions": [
      {
        "action": "collapse",
        "block_id": "b0bc4af0362ef61c14881c4e",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back from working set",
        "priority": 6,
        "short_id": "S85"
      }
    ],
    "recommended_next_action": {
      "action": "collapse",
      "block_id": "b0bc4af0362ef61c14881c4e",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::back from working set",
      "priority": 6,
      "short_id": "S85"
    },
    "should_stop": true
  },
  "hidden_levels": [
    {
      "count": 5,
      "direction": "manual",
      "level": 2
    },
    {
      "count": 6,
      "direction": "structural",
      "level": 2,
      "relation": "contains_symbol"
    },
    {
      "count": 1,
      "direction": "manual",
      "level": 3
    },
    {
      "count": 76,
      "direction": "structural",
      "level": 3,
      "relation": "contains_symbol"
    },
    {
      "count": 38,
      "direction": "structural",
      "level": 4,
      "relation": "contains_symbol"
    }
  ],
... clipped 67 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context --format json`

```text
{
  "added": [],
  "changed": [
    "blk_fe1acf1e3e51fd5474b97f2b"
  ],
  "focus": "blk_fe1acf1e3e51fd5474b97f2b",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context (+1 visible level)

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --levels 1 --format json`

```text
{
  "edges": [],
  "export_mode": "compact",
  "focus": "fe1acf1e3e51fd5474b97f2b",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context",
  "focus_short_id": "S86",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "fe1acf1e3e51fd5474b97f2b",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context",
      "priority": 121,
      "short_id": "S86"
    },
    {
      "action": "collapse",
      "block_id": "fe1acf1e3e51fd5474b97f2b",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context from working set",
      "priority": 6,
      "short_id": "S86"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "recommended_actions": [
      {
        "action": "hydrate_source",
        "block_id": "fe1acf1e3e51fd5474b97f2b",
        "candidate_count": 1,
        "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context",
        "priority": 121,
        "short_id": "S86"
      },
      {
        "action": "collapse",
        "block_id": "fe1acf1e3e51fd5474b97f2b",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context from working set",
        "priority": 6,
        "short_id": "S86"
      }
    ],
    "recommended_next_action": {
      "action": "hydrate_source",
      "block_id": "fe1acf1e3e51fd5474b97f2b",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context",
      "priority": 121,
      "short_id": "S86"
    },
    "should_stop": false
  },
  "hidden_levels": [
    {
      "count": 6,
      "direction": "manual",
      "level": 2
    },
    {
      "count": 5,
      "direction": "structural",
      "level": 2,
      "relation": "contains_symbol"
    },
    {
      "count": 1,
      "direction": "manual",
      "level": 3
    },
    {
      "count": 76,
      "direction": "structural",
      "level": 3,
      "relation": "contains_symbol"
    },
    {
      "count": 38,
... clipped 65 more lines ...
```

## Apply the top recommended frontier action for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context

`$ cargo run -q -p ucp-cli -- codegraph context expand-recommended --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --top 1 --priority-threshold 60 --format json`

```text
{
  "added": [],
  "applied_actions": [
    "hydrate_source S86 (priority 121, 1 candidates)"
  ],
  "changed": [
    "blk_fe1acf1e3e51fd5474b97f2b",
    "blk_fe1acf1e3e51fd5474b97f2b"
  ],
  "focus": "blk_fe1acf1e3e51fd5474b97f2b",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_fe1acf1e3e51fd5474b97f2b"
  ],
  "focus": "blk_fe1acf1e3e51fd5474b97f2b",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --format json`

```text
{
  "edges": [],
  "export_mode": "compact",
  "focus": "fe1acf1e3e51fd5474b97f2b",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context",
  "focus_short_id": "S86",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "fe1acf1e3e51fd5474b97f2b",
      "candidate_count": 0,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context",
      "priority": 120,
      "short_id": "S86"
    },
    {
      "action": "collapse",
      "block_id": "fe1acf1e3e51fd5474b97f2b",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context from working set",
      "priority": 6,
      "short_id": "S86"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "reasons": [
      "focus symbol is hydrated and no unselected dependency frontier remains"
    ],
    "recommended_actions": [
      {
        "action": "collapse",
        "block_id": "fe1acf1e3e51fd5474b97f2b",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context from working set",
        "priority": 6,
        "short_id": "S86"
      }
    ],
    "recommended_next_action": {
      "action": "collapse",
      "block_id": "fe1acf1e3e51fd5474b97f2b",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::clear_context from working set",
      "priority": 6,
      "short_id": "S86"
    },
    "should_stop": true
  },
  "hidden_levels": [
    {
      "count": 6,
      "direction": "manual",
      "level": 2
    },
    {
      "count": 5,
      "direction": "structural",
      "level": 2,
      "relation": "contains_symbol"
    },
    {
      "count": 1,
      "direction": "manual",
      "level": 3
    },
    {
      "count": 76,
      "direction": "structural",
      "level": 3,
      "relation": "contains_symbol"
    },
    {
      "count": 38,
      "direction": "structural",
      "level": 4,
      "relation": "contains_symbol"
    }
  ],
... clipped 67 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context --format json`

```text
{
  "added": [],
  "changed": [
    "blk_520f075fcd694a0fd58dab95"
  ],
  "focus": "blk_520f075fcd694a0fd58dab95",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context (+1 visible level)

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --levels 1 --format json`

```text
{
  "edges": [],
  "export_mode": "compact",
  "focus": "520f075fcd694a0fd58dab95",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context",
  "focus_short_id": "S87",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "520f075fcd694a0fd58dab95",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context",
      "priority": 121,
      "short_id": "S87"
    },
    {
      "action": "collapse",
      "block_id": "520f075fcd694a0fd58dab95",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context from working set",
      "priority": 6,
      "short_id": "S87"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "recommended_actions": [
      {
        "action": "hydrate_source",
        "block_id": "520f075fcd694a0fd58dab95",
        "candidate_count": 1,
        "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context",
        "priority": 121,
        "short_id": "S87"
      },
      {
        "action": "collapse",
        "block_id": "520f075fcd694a0fd58dab95",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context from working set",
        "priority": 6,
        "short_id": "S87"
      }
    ],
    "recommended_next_action": {
      "action": "hydrate_source",
      "block_id": "520f075fcd694a0fd58dab95",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context",
      "priority": 121,
      "short_id": "S87"
    },
    "should_stop": false
  },
  "hidden_levels": [
    {
      "count": 7,
      "direction": "manual",
      "level": 2
    },
    {
      "count": 4,
      "direction": "structural",
      "level": 2,
      "relation": "contains_symbol"
    },
    {
      "count": 1,
      "direction": "manual",
      "level": 3
    },
    {
      "count": 76,
      "direction": "structural",
      "level": 3,
      "relation": "contains_symbol"
    },
    {
      "count": 38,
... clipped 65 more lines ...
```

## Apply the top recommended frontier action for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context

`$ cargo run -q -p ucp-cli -- codegraph context expand-recommended --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --top 1 --priority-threshold 60 --format json`

```text
{
  "added": [],
  "applied_actions": [
    "hydrate_source S87 (priority 121, 1 candidates)"
  ],
  "changed": [
    "blk_520f075fcd694a0fd58dab95",
    "blk_520f075fcd694a0fd58dab95"
  ],
  "focus": "blk_520f075fcd694a0fd58dab95",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_520f075fcd694a0fd58dab95"
  ],
  "focus": "blk_520f075fcd694a0fd58dab95",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --format json`

```text
{
  "edges": [],
  "export_mode": "compact",
  "focus": "520f075fcd694a0fd58dab95",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context",
  "focus_short_id": "S87",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "520f075fcd694a0fd58dab95",
      "candidate_count": 0,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context",
      "priority": 120,
      "short_id": "S87"
    },
    {
      "action": "collapse",
      "block_id": "520f075fcd694a0fd58dab95",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context from working set",
      "priority": 6,
      "short_id": "S87"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "reasons": [
      "focus symbol is hydrated and no unselected dependency frontier remains"
    ],
    "recommended_actions": [
      {
        "action": "collapse",
        "block_id": "520f075fcd694a0fd58dab95",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context from working set",
        "priority": 6,
        "short_id": "S87"
      }
    ],
    "recommended_next_action": {
      "action": "collapse",
      "block_id": "520f075fcd694a0fd58dab95",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::ensure_codegraph_context from working set",
      "priority": 6,
      "short_id": "S87"
    },
    "should_stop": true
  },
  "hidden_levels": [
    {
      "count": 7,
      "direction": "manual",
      "level": 2
    },
    {
      "count": 4,
      "direction": "structural",
      "level": 2,
      "relation": "contains_symbol"
    },
    {
      "count": 1,
      "direction": "manual",
      "level": 3
    },
    {
      "count": 76,
      "direction": "structural",
      "level": 3,
      "relation": "contains_symbol"
    },
    {
      "count": 38,
      "direction": "structural",
      "level": 4,
      "relation": "contains_symbol"
    }
  ],
... clipped 67 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto --format json`

```text
{
  "added": [],
  "changed": [
    "blk_4797cf720ffc4759c4487680"
  ],
  "focus": "blk_4797cf720ffc4759c4487680",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto (+1 visible level)

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --levels 1 --format json`

```text
{
  "edges": [],
  "export_mode": "compact",
  "focus": "4797cf720ffc4759c4487680",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto",
  "focus_short_id": "S88",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "4797cf720ffc4759c4487680",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto",
      "priority": 121,
      "short_id": "S88"
    },
    {
      "action": "collapse",
      "block_id": "4797cf720ffc4759c4487680",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto from working set",
      "priority": 6,
      "short_id": "S88"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "recommended_actions": [
      {
        "action": "hydrate_source",
        "block_id": "4797cf720ffc4759c4487680",
        "candidate_count": 1,
        "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto",
        "priority": 121,
        "short_id": "S88"
      },
      {
        "action": "collapse",
        "block_id": "4797cf720ffc4759c4487680",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto from working set",
        "priority": 6,
        "short_id": "S88"
      }
    ],
    "recommended_next_action": {
      "action": "hydrate_source",
      "block_id": "4797cf720ffc4759c4487680",
      "candidate_count": 1,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto",
      "priority": 121,
      "short_id": "S88"
    },
    "should_stop": false
  },
  "hidden_levels": [
    {
      "count": 8,
      "direction": "manual",
      "level": 2
    },
    {
      "count": 3,
      "direction": "structural",
      "level": 2,
      "relation": "contains_symbol"
    },
    {
      "count": 1,
      "direction": "manual",
      "level": 3
    },
    {
      "count": 76,
      "direction": "structural",
      "level": 3,
      "relation": "contains_symbol"
    },
    {
      "count": 38,
... clipped 65 more lines ...
```

## Apply the top recommended frontier action for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto

`$ cargo run -q -p ucp-cli -- codegraph context expand-recommended --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --top 1 --priority-threshold 60 --format json`

```text
{
  "added": [],
  "applied_actions": [
    "hydrate_source S88 (priority 121, 1 candidates)"
  ],
  "changed": [
    "blk_4797cf720ffc4759c4487680",
    "blk_4797cf720ffc4759c4487680"
  ],
  "focus": "blk_4797cf720ffc4759c4487680",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_4797cf720ffc4759c4487680"
  ],
  "focus": "blk_4797cf720ffc4759c4487680",
  "removed": [],
  "session": "cgctx_9f3b4e82",
  "success": true,
  "total": 129,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --format json`

```text
{
  "edges": [],
  "export_mode": "compact",
  "focus": "4797cf720ffc4759c4487680",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto",
  "focus_short_id": "S88",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "4797cf720ffc4759c4487680",
      "candidate_count": 0,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto",
      "priority": 120,
      "short_id": "S88"
    },
    {
      "action": "collapse",
      "block_id": "4797cf720ffc4759c4487680",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto from working set",
      "priority": 6,
      "short_id": "S88"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "reasons": [
      "focus symbol is hydrated and no unselected dependency frontier remains"
    ],
    "recommended_actions": [
      {
        "action": "collapse",
        "block_id": "4797cf720ffc4759c4487680",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto from working set",
        "priority": 6,
        "short_id": "S88"
      }
    ],
    "recommended_next_action": {
      "action": "collapse",
      "block_id": "4797cf720ffc4759c4487680",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto from working set",
      "priority": 6,
      "short_id": "S88"
    },
    "should_stop": true
  },
  "hidden_levels": [
    {
      "count": 8,
      "direction": "manual",
      "level": 2
    },
    {
      "count": 3,
      "direction": "structural",
      "level": 2,
      "relation": "contains_symbol"
    },
    {
      "count": 1,
      "direction": "manual",
      "level": 3
    },
    {
      "count": 76,
      "direction": "structural",
      "level": 3,
      "relation": "contains_symbol"
    },
    {
      "count": 38,
      "direction": "structural",
      "level": 4,
      "relation": "contains_symbol"
    }
  ],
... clipped 67 more lines ...
```

## Export the final structured context

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-6762vu2d/ucp-codegraph.json --session cgctx_9f3b4e82 --compact --no-rendered --format json`

```text
{
  "edges": [],
  "export_mode": "compact",
  "focus": "4797cf720ffc4759c4487680",
  "focus_label": "symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto",
  "focus_short_id": "S88",
  "frontier": [
    {
      "action": "hydrate_source",
      "block_id": "4797cf720ffc4759c4487680",
      "candidate_count": 0,
      "description": "Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto",
      "priority": 120,
      "short_id": "S88"
    },
    {
      "action": "collapse",
      "block_id": "4797cf720ffc4759c4487680",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto from working set",
      "priority": 6,
      "short_id": "S88"
    }
  ],
  "heuristics": {
    "hidden_candidate_count": 0,
    "low_value_candidate_count": 0,
    "reasons": [
      "focus symbol is hydrated and no unselected dependency frontier remains"
    ],
    "recommended_actions": [
      {
        "action": "collapse",
        "block_id": "4797cf720ffc4759c4487680",
        "candidate_count": 1,
        "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto from working set",
        "priority": 6,
        "short_id": "S88"
      }
    ],
    "recommended_next_action": {
      "action": "collapse",
      "block_id": "4797cf720ffc4759c4487680",
      "candidate_count": 1,
      "description": "Collapse symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto from working set",
      "priority": 6,
      "short_id": "S88"
    },
    "should_stop": true
  },
  "hidden_levels": [
    {
      "count": 8,
      "direction": "manual",
      "level": 2
    },
    {
      "count": 3,
      "direction": "structural",
      "level": 2,
      "relation": "contains_symbol"
    },
    {
      "count": 1,
      "direction": "manual",
      "level": 3
    },
    {
      "count": 76,
      "direction": "structural",
      "level": 3,
      "relation": "contains_symbol"
    },
    {
      "count": 38,
      "direction": "structural",
      "level": 4,
      "relation": "contains_symbol"
    }
  ],
... clipped 67 more lines ...
```

## Read coderef-backed excerpts from the final working set

### S88 `symbol:crates/ucp-cli/src/state.rs::AgentSessionState::goto`

- ref: `crates/ucp-cli/src/state.rs:84-89`

```rust
  82     }
  83 
  84     pub fn goto(&mut self, block_id: &BlockId) {
  85         if let Some(current) = &self.current_block {
  86             self.history.push(current.clone());
  87         }
  88         self.current_block = Some(block_id.to_string());
  89     }
  90 
  91     pub fn back(&mut self, steps: usize) -> Option<BlockId> {
```
### S83 `symbol:crates/ucp-cli/src/state.rs::AgentSessionState#69`

- ref: `crates/ucp-cli/src/state.rs:69-137`

```rust
  67 }
  68 
  69 impl AgentSessionState {
  70     pub fn new(id: String, name: Option<String>, start_block: Option<BlockId>) -> Self {
  71         Self {
  72             id,
  73             name,
  74             current_block: start_block.map(|b| b.to_string()),
  75             history: Vec::new(),
  76             context_blocks: Vec::new(),
  77             codegraph_context: None,
  78             codegraph_preferences: CodeGraphSessionPreferences::default(),
  79             state: "active".to_string(),
  80             created_at: chrono::Utc::now().to_rfc3339(),
  81         }
  82     }
  83 
  84     pub fn goto(&mut self, block_id: &BlockId) {
  85         if let Some(current) = &self.current_block {
  86             self.history.push(current.clone());
  87         }
  88         self.current_block = Some(block_id.to_string());
  89     }
  90 
  91     pub fn back(&mut self, steps: usize) -> Option<BlockId> {
  92         use std::str::FromStr;
  93         for _ in 0..steps {
  94             if let Some(prev) = self.history.pop() {
  95                 self.current_block = Some(prev);
  96             }
  97         }
  98         self.current_block
  99             .as_ref()
 100             .and_then(|s| BlockId::from_str(s).ok())
 101     }
 102 
 103     pub fn add_to_context(&mut self, block_id: &BlockId) {
 104         let id_str = block_id.to_string();
 105         if !self.context_blocks.contains(&id_str) {
 106             self.context_blocks.push(id_str);
 107         }
 108     }
 109 
 110     pub fn remove_from_context(&mut self, block_id: &BlockId) {
 111         let id_str = block_id.to_string();
 112         self.context_blocks.retain(|b| b != &id_str);
 113     }
 114 
 115     #[allow(dead_code)]
 116     pub fn clear_context(&mut self) {
 117         self.context_blocks.clear();
 118         if let Some(context) = self.codegraph_context.as_mut() {
 119             context.clear();
 120         }
 121     }
 122 
 123     pub fn ensure_codegraph_context(&mut self) -> &mut CodeGraphContextSession {
 124         self.codegraph_context
 125             .get_or_insert_with(CodeGraphContextSession::new)
 126     }
 127 
 128     pub fn sync_context_blocks_from_codegraph(&mut self) {
 129         if let Some(context) = self.codegraph_context.as_ref() {
 130             self.context_blocks = context
 131                 .selected_block_ids()
 132                 .into_iter()
 133                 .map(|block_id| block_id.to_string())
 134                 .collect();
 135         }
 136     }
 137 }
 138 
 139 /// Serializable snapshot info
```

## Final summary

- selected nodes: 129
- frontier actions remaining: 2
- transcript file: `artifacts/codegraph-context-demo-transcript.md`
