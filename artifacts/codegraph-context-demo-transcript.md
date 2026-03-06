## Codegraph context demo transcript

Chosen refactor candidate: deduplicate codegraph context/session helper logic across `agent.rs` and `codegraph.rs`.

## Build a codegraph for the current repository

`$ cargo run -q -p ucp-cli -- codegraph build /home/antonio/programming/Hivemind/unified-content-protocol --commit c54e7241 --output /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --allow-partial --format json`

```text
{
  "status": "partial_success",
  "profile_version": "codegraph.v1",
  "canonical_fingerprint": "3641d2e39635c634d8ac97a37bec779e98a879b85d74a7e3e2c4e3fe30dfd1d7",
  "stats": {
    "total_nodes": 5046,
    "repository_nodes": 1,
    "directory_nodes": 51,
    "file_nodes": 146,
    "symbol_nodes": 4848,
    "total_edges": 5238,
    "reference_edges": 601,
    "export_edges": 1543,
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
... clipped 352899 more lines ...
```

## Initialize a stateful codegraph context session from the root overview

`$ cargo run -q -p ucp-cli -- codegraph context init --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --name demo_context_walk --max-selected 512 --format json`

```text
{
  "name": "demo_context_walk",
  "rendered": "CodeGraph working set\nfocus: [D1] crates\nsummary: selected=198/512 repositories=1 directories=51 files=146 symbols=0 hydrated=0\n\nfilesystem:\n- [R1] .\n- [D4] crates/translators/html/src\n- [D7] crates/translators/markdown/tests\n- [D2] crates/translators\n- [D11] crates/ucm-core/src\n- [D10] crates/ucm-core\n- [D28] crates/ucp-api/tests/fixtures/single-language-rust\n- [D49] crates/ucp-wasm/src\n- [D36] crates/ucp-codegraph/src/legacy\n- [D34] crates/ucp-codegraph\n- [D26] crates/ucp-api/tests/fixtures/multi-language/src\n- [D18] crates/ucp-api/src\n- [D29] crates/ucp-api/tests/fixtures/single-language-rust/src\n- [D30] crates/ucp-cli\n- [D37] crates/ucp-codegraph/src/legacy/languages\n- [D33] crates/ucp-cli/tests\n- [D41] crates/ucp-observe/src\n- [D42] crates/ucp-python\n- [D39] crates/ucp-llm/src\n- [D3] crates/translators/html\n- [D20] crates/ucp-api/tests/fixtures\n- [D14] crates/ucp-agent\n- [D38] crates/ucp-llm\n- [D12] crates/ucm-engine\n- [D51] scripts\n- [D6] crates/translators/markdown/src\n- [D27] crates/ucp-api/tests/fixtures/multi-language/web\n- [D8] crates/ucl-parser\n- [D25] crates/ucp-api/tests/fixtures/multi-language/py\n- [D16] crates/ucp-agent/tests\n- [D45] crates/ucp-python/src\n- [D17] crates/ucp-api\n- [D24] crates/ucp-api/tests/fixtures/multi-language\n- [D23] crates/ucp-api/tests/fixtures/edge-cases/src\n- [D1] crates\n- [D44] crates/ucp-python/python/ucp\n- [D48] crates/ucp-wasm/pkg\n- [D22] crates/ucp-api/tests/fixtures/edge-cases/py\n- [D43] crates/ucp-python/python\n- [D50] crates/ucp-wasm/tests\n- [D19] crates/ucp-api/tests\n- [D13] crates/ucm-engine/src\n- [D40] crates/ucp-observe\n- [D5] crates/translators/markdown\n- [D32] crates/ucp-cli/src/commands\n- [D31] crates/ucp-cli/src\n- [D15] crates/ucp-agent/src\n- [D47] crates/ucp-wasm\n- [D35] crates/ucp-codegraph/src\n- [D46] crates/ucp-python/tests\n- [D21] crates/ucp-api/tests/fixtures/edge-cases\n- [D9] crates/ucl-parser/src\n- [F114] crates/ucp-python/tests/conftest.py [python]\n- [F106] crates/ucp-python/src/engine.rs [rust]\n- [F46] crates/ucp-api/tests/fixtures/edge-cases/web.ts [typescript]\n- [F138] crates/ucp-wasm/src/observe.rs [rust]\n- [F26] crates/ucm-engine/src/operation.rs [rust]\n- [F34] crates/ucp-agent/src/executor.rs [rust]\n- [F74] crates/ucp-cli/src/main.rs [rust]\n- [F13] crates/ucm-core/src/content.rs [rust]\n- [F105] crates/ucp-python/src/edge.rs [rust]\n- [F134] crates/ucp-wasm/src/engine.rs [rust]\n- [F21] crates/ucm-core/src/version.rs [rust]\n- [F32] crates/ucp-agent/src/cursor.rs [rust]\n- [F1] crates/translators/html/src/error.rs [rust]\n- [F49] crates/ucp-api/tests/fixtures/multi-language/src/lib.rs [rust]\n- [F129] crates/ucp-wasm/src/agent.rs [rust]\n- [F65] crates/ucp-cli/src/commands/mod.rs [rust]\n- [F66] crates/ucp-cli/src/commands/nav.rs [rust]\n- [F142] crates/ucp-wasm/tests/ucp.test.js [javascript]\n- [F144] scripts/demo_codegraph_context_walk.py [python]\n- [F93] crates/ucp-codegraph/src/projection.rs [rust]\n- [F5] crates/translators/markdown/src/lib.rs [rust]\n- [F68] crates/ucp-cli/src/commands/snapshot.rs [rust]\n- [F73] crates/ucp-cli/src/error.rs [rust]\n- [F20] crates/ucm-core/src/normalize.rs [rust]\n- [F10] crates/ucl-parser/src/lib.rs [rust]\n- [F115] crates/ucp-python/tests/test_agent.py [python]\n- [F37] crates/ucp-agent/src/operations.rs [rust]\n- [F82] crates/ucp-codegraph/src/legacy/canonical.rs [rust]\n- [F14] crates/ucm-core/src/document.rs [rust]\n- [F128] crates/ucp-wasm/pkg/ucp_wasm_bg.wasm.d.ts [typescript]\n- [F62] crates/ucp-cli/src/commands/find.rs [rust]\n- [F55] crates/ucp-cli/src/commands/agent.rs [rust]\n- [F59] crates/ucp-cli/src/commands/document.rs [rust]\n- [F16] crates/ucm-core/src/error.rs [rust]\n- [F85] crates/ucp-codegraph/src/legacy/languages/python.rs [rust]\n- [F23] crates/ucm-engine/src/engine.rs [rust]\n- [F89] crates/ucp-codegraph/src/legacy/tests.rs [rust]\n- [F67] crates/ucp-cli/src/commands/prune.rs [rust]\n- [F18] crates/ucm-core/src/lib.rs [rust]\n- [F132] crates/ucp-wasm/src/document.rs [rust]\n- [F71] crates/ucp-cli/src/commands/ucl.rs [rust]\n- [F52] crates/ucp-api/tests/fixtures/single-language-rust/src/lib.rs [rust]\n- [F19] crates/ucm-core/src/metadata.rs [rust]\n- [F113] crates/ucp-python/src/types.rs [rust]\n- [F117] crates/ucp-python/tests/test_content.py [python]\n- [F110] crates/ucp-python/src/observe.rs [rust]\n- [F87] crates/ucp-codegraph/src/legacy/languages/ts_js.rs [rust]\n- [F47] crates/ucp-api/tests/fixtures/multi-language/py/helper.py [python]\n- [F127] crates/ucp-wasm/pkg/ucp_wasm.js [javascript]\n- [F38] crates/ucp-agent/src/rag.rs [rust]\n- [F130] crates/ucp-wasm/src/block.rs [rust]\n- [F80] crates/ucp-codegraph/src/legacy/analyze.rs [rust]\n- [F79] crates/ucp-codegraph/src/legacy.rs [rust]\n- [F99] crates/ucp-python/build.rs [rust]\n- [F94] crates/ucp-llm/src/context.rs [rust]\n- [F140] crates/ucp-wasm/src/snapshot.rs [rust]\n- [F135] crates/ucp-wasm/src/errors.rs [rust]\n- [F43] crates/ucp-api/tests/codegraph_fixture_tests.rs [rust]\n- [F112] crates/ucp-python/src/snapshot.rs [rust]\n- [F12] crates/ucm-core/src/block.rs [rust]\n- [F58] crates/ucp-cli/src/commands/completions.rs [rust]\n- [F124] crates/ucp-python/tests/test_section.py [python]\n- [F88] crates/ucp-codegraph/src/legacy/resolve.rs [rust]\n- [F116] crates/ucp-python/tests/test_comprehensive_failures.py [python]\n- [F53] crates/ucp-api/tests/fixtures/single-language-rust/src/util.rs [rust]\n- [F90] crates/ucp-codegraph/src/legacy/validate.rs [rust]\n- [F28] crates/ucm-engine/src/snapshot.rs [rust]\n- [F145] scripts/log_helper.py [python]\n- [F8] crates/ucl-parser/src/ast.rs [rust]\n- [F86] crates/ucp-codegraph/src/legacy/languages/rust.rs [rust]\n- [F2] crates/translators/html/src/lib.rs [rust]\n- [F51] crates/ucp-api/tests/fixtures/multi-language/web/util.ts [typescript]\n- [F133] crates/ucp-wasm/src/edge.rs [rust]\n- [F104] crates/ucp-python/src/document.rs [rust]\n- [F119] crates/ucp-python/tests/test_edges.py [python]\n- [F81] crates/ucp-codegraph/src/legacy/build.rs [rust]\n- [F4] crates/translators/markdown/src/from_markdown.rs [rust]\n- [F45] crates/ucp-api/tests/fixtures/edge-cases/src/lib.rs [rust]\n- [F118] crates/ucp-python/tests/test_document.py [python]\n- [F136] crates/ucp-wasm/src/lib.rs [rust]\n- [F126] crates/ucp-wasm/pkg/ucp_wasm.d.ts [typescript]\n- [F103] crates/ucp-python/src/content.rs [rust]\n- [F77] crates/ucp-cli/tests/integration_tests.rs [rust]\n- [F31] crates/ucm-engine/src/validate.rs [rust]\n- [F29] crates/ucm-engine/src/transaction.rs [rust]\n- [F120] crates/ucp-python/tests/test_engine.py [python]\n- [F33] crates/ucp-agent/src/error.rs [rust]\n- [F56] crates/ucp-cli/src/commands/block.rs [rust]\n- [F137] crates/ucp-wasm/src/llm.rs [rust]\n- [F78] crates/ucp-codegraph/src/context.rs [rust]\n- [F11] crates/ucl-parser/src/parser.rs [rust]\n- [F39] crates/ucp-agent/src/safety.rs [rust]\n- [F36] crates/ucp-agent/src/metrics.rs [rust]\n- [F123] crates/ucp-python/tests/test_llm.py [python]\n- [F131] crates/ucp-wasm/src/content.rs [rust]\n- [F84] crates/ucp-codegraph/src/legacy/filesystem.rs [rust]\n- [F143] scripts/check_version_sync.py [python]\n- [F108] crates/ucp-python/src/lib.rs [rust]\n- [F40] crates/ucp-agent/src/session.rs [rust]\n- [F17] crates/ucm-core/src/id.rs [rust]\n- [F83] crates/ucp-codegraph/src/legacy/extract.rs [rust]\n- [F48] crates/ucp-api/tests/fixtures/multi-language/py/main.py [python]\n- [F76] crates/ucp-cli/src/state.rs [rust]\n- [F25] crates/ucm-engine/src/lib.rs [rust]\n- [F95] crates/ucp-llm/src/id_mapper.rs [rust]\n- [F122] crates/ucp-python/tests/test_integration.py [python]\n- [F141] crates/ucp-wasm/src/types.rs [rust]\n- [F125] crates/ucp-python/tests/test_snapshots.py [python]\n- [F22] crates/ucm-engine/src/config.rs [rust]\n- [F41] crates/ucp-agent/tests/integration_tests.rs [rust]\n- [F111] crates/ucp-python/src/section.rs [rust]\n- [F44] crates/ucp-api/tests/fixtures/edge-cases/py/main.py [python]\n- [F42] crates/ucp-api/src/lib.rs [rust]\n- [F9] crates/ucl-parser/src/lexer.rs [rust]\n- [F98] crates/ucp-observe/src/lib.rs [rust]\n- [F102] crates/ucp-python/src/block.rs [rust]\n- [F121] crates/ucp-python/tests/test_failure_regression.py [python]\n- [F61] crates/ucp-cli/src/commands/export.rs [rust]\n- [F30] crates/ucm-engine/src/traversal.rs [rust]\n- [F107] crates/ucp-python/src/errors.rs [rust]\n- [F91] crates/ucp-codegraph/src/lib.rs [rust]\n- [F7] crates/translators/markdown/tests/golden_tests.rs [rust]\n- [F54] crates/ucp-cli/src/cli.rs [rust]\n- [F96] crates/ucp-llm/src/lib.rs [rust]\n- [F146] scripts/release.py [python]\n- [F69] crates/ucp-cli/src/commands/tree.rs [rust]\n- [F97] crates/ucp-llm/src/prompt_builder.rs [rust]\n- [F60] crates/ucp-cli/src/commands/edge.rs [rust]\n- [F101] crates/ucp-python/src/agent.rs [rust]\n- [F50] crates/ucp-api/tests/fixtures/multi-language/web/app.ts [typescript]\n- [F15] crates/ucm-core/src/edge.rs [rust]\n- [F24] crates/ucm-engine/src/error.rs [rust]\n- [F57] crates/ucp-cli/src/commands/codegraph.rs [rust]\n- [F35] crates/ucp-agent/src/lib.rs [rust]\n- [F100] crates/ucp-python/python/ucp/__init__.py [python]\n- [F72] crates/ucp-cli/src/commands/validate.rs [rust]\n- [F6] crates/translators/markdown/src/to_markdown.rs [rust]\n- [F63] crates/ucp-cli/src/commands/import.rs [rust]\n- [F3] crates/translators/html/src/parser.rs [rust]\n- [F75] crates/ucp-cli/src/output.rs [rust]\n- [F27] crates/ucm-engine/src/section.rs [rust]\n- [F109] crates/ucp-python/src/llm.rs [rust]\n- [F139] crates/ucp-wasm/src/section.rs [rust]\n- [F64] crates/ucp-cli/src/commands/llm.rs [rust]\n- [F70] crates/ucp-cli/src/commands/tx.rs [rust]\n- [F92] crates/ucp-codegraph/src/model.rs [rust]\n\nomissions:\n- symbols omitted from working set: 4848\n- prune policy: max_selected=512 demote_before_remove=true protect_focus=true\n\nfrontier:\n- set focus to a file or symbol to expand the working set",
  "session_id": "cgctx_b882aa45",
  "success": true,
  "summary": {
    "directories": 51,
    "files": 146,
    "hydrated_sources": 0,
    "max_selected": 512,
    "repositories": 1,
    "selected": 198,
    "symbols": 0
  }
}
```

Session: `cgctx_b882aa45`

## Show the initial root working set

`$ cargo run -q -p ucp-cli -- codegraph context show --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 7431 more lines ...
```

## Expand file symbols for crates/ucp-cli/src/commands/codegraph.rs

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 crates/ucp-cli/src/commands/codegraph.rs --mode file --format json`

```text
{
  "added": [
    "blk_ee2bd5d66010f7ca9ff03751",
    "blk_64e06ba0432fb20a3d983c9e",
    "blk_ecc7305aa5537bc4ba194f70",
    "blk_ccd60e7e0204e6c070b49b3a",
    "blk_a1024e521fbcbc03beb54a91",
    "blk_6e236e367cc0078a78fa2bb4",
    "blk_f47e88a42839ca7aecf15597",
    "blk_28c88ee3ea39d0e262f6b2a1",
    "blk_60ab5b57aaa6697047e885f7",
    "blk_cd6ef8e63c1b57f336baa2c3",
    "blk_bab6e0b3b0d86000edbf4749",
    "blk_9535be6a8e7162c109a6e579",
    "blk_e974dc99ca25af3da241c794",
    "blk_5080e04b8929051c62f21332",
    "blk_9ca3a1a6369f1c436d3bba00",
    "blk_5e097ddbd88ce747c9fc94e7",
    "blk_329692a490f409a0651ea892",
    "blk_4ab1409d012f04700dfae70c",
    "blk_9144e5729789e34ecae9f578",
    "blk_ca55094fc900efb2de1ec7f6",
    "blk_e6b2d1db3b0de2651d0f96f2",
    "blk_90d1d70429cbf4ca29a5bbd0",
    "blk_e0d658f88c35822f46fe7f89",
    "blk_bca85a9fddb9c1cf661e95ab"
  ],
  "changed": [
    "blk_e4fc6ac96af7e1f5b261fe1f"
  ],
  "focus": "blk_e4fc6ac96af7e1f5b261fe1f",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 222,
  "warnings": []
}
```

## Expand file symbols for crates/ucp-cli/src/commands/agent.rs

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 crates/ucp-cli/src/commands/agent.rs --mode file --format json`

```text
{
  "added": [
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
    "blk_4d6beda7db26a12be6f96d3e"
  ],
  "changed": [
    "blk_421333df5881c48ef6b4be16"
  ],
  "focus": "blk_421333df5881c48ef6b4be16",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 250,
  "warnings": []
}
```

## Expand file symbols for crates/ucp-cli/src/state.rs

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 crates/ucp-cli/src/state.rs --mode file --format json`

```text
{
  "added": [
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
    "blk_e91699dcdbeccecf2e3faacc"
  ],
  "changed": [
    "blk_b93da718e1d46a0927bc4074"
  ],
  "focus": "blk_b93da718e1d46a0927bc4074",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 265,
  "warnings": []
}
```

## Export the structured working set after file expansion

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10007 more lines ...
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

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_show --format json`

```text
{
  "added": [],
  "changed": [
    "blk_ea22c75b78e8959e70d7f4cc"
  ],
  "focus": "blk_ea22c75b78e8959e70d7f4cc",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 265,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::context_show

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10022 more lines ...
```

## Expand dependencies for symbol:crates/ucp-cli/src/commands/agent.rs::context_show via uses_symbol

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_show --mode dependencies --relation uses_symbol --format json`

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
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 266,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::context_show

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_show --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_ea22c75b78e8959e70d7f4cc",
    "blk_ea22c75b78e8959e70d7f4cc"
  ],
  "focus": "blk_ea22c75b78e8959e70d7f4cc",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 266,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::context_show

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10091 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut --format json`

```text
{
  "added": [],
  "changed": [
    "blk_c9c42d87b5039c025547ddd3"
  ],
  "focus": "blk_c9c42d87b5039c025547ddd3",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 266,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10090 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_c9c42d87b5039c025547ddd3",
    "blk_c9c42d87b5039c025547ddd3"
  ],
  "focus": "blk_c9c42d87b5039c025547ddd3",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 266,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10097 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update --format json`

```text
{
  "added": [],
  "changed": [
    "blk_b418c0b106a76e2a816dc89d"
  ],
  "focus": "blk_b418c0b106a76e2a816dc89d",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 266,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10105 more lines ...
```

## Expand dependencies for symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update via uses_symbol

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update --mode dependencies --relation uses_symbol --format json`

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
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 267,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_b418c0b106a76e2a816dc89d",
    "blk_b418c0b106a76e2a816dc89d"
  ],
  "focus": "blk_b418c0b106a76e2a816dc89d",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 267,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10307 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector --format json`

```text
{
  "added": [],
  "changed": [
    "blk_4ad997efb31de3ed7b077bf8"
  ],
  "focus": "blk_4ad997efb31de3ed7b077bf8",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 267,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10306 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_4ad997efb31de3ed7b077bf8",
    "blk_4ad997efb31de3ed7b077bf8"
  ],
  "focus": "blk_4ad997efb31de3ed7b077bf8",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 267,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10313 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show --format json`

```text
{
  "added": [],
  "changed": [
    "blk_9535be6a8e7162c109a6e579"
  ],
  "focus": "blk_9535be6a8e7162c109a6e579",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 267,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10312 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_9535be6a8e7162c109a6e579",
    "blk_9535be6a8e7162c109a6e579"
  ],
  "focus": "blk_9535be6a8e7162c109a6e579",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 267,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10319 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut --format json`

```text
{
  "added": [],
  "changed": [
    "blk_5e097ddbd88ce747c9fc94e7"
  ],
  "focus": "blk_5e097ddbd88ce747c9fc94e7",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 267,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10318 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_5e097ddbd88ce747c9fc94e7",
    "blk_5e097ddbd88ce747c9fc94e7"
  ],
  "focus": "blk_5e097ddbd88ce747c9fc94e7",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 267,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10325 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update --format json`

```text
{
  "added": [],
  "changed": [
    "blk_ca55094fc900efb2de1ec7f6"
  ],
  "focus": "blk_ca55094fc900efb2de1ec7f6",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 267,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10333 more lines ...
```

## Expand dependencies for symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update via uses_symbol

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update --mode dependencies --relation uses_symbol --format json`

```text
{
  "added": [
    "blk_823f109791f59cbd2455992b"
  ],
  "changed": [
    "blk_ca55094fc900efb2de1ec7f6"
  ],
  "focus": "blk_ca55094fc900efb2de1ec7f6",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_ca55094fc900efb2de1ec7f6",
    "blk_ca55094fc900efb2de1ec7f6"
  ],
  "focus": "blk_ca55094fc900efb2de1ec7f6",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10388 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector --format json`

```text
{
  "added": [],
  "changed": [
    "blk_90d1d70429cbf4ca29a5bbd0"
  ],
  "focus": "blk_90d1d70429cbf4ca29a5bbd0",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10387 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_90d1d70429cbf4ca29a5bbd0",
    "blk_90d1d70429cbf4ca29a5bbd0"
  ],
  "focus": "blk_90d1d70429cbf4ca29a5bbd0",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10394 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::back

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::back --format json`

```text
{
  "added": [],
  "changed": [
    "blk_6235791ed9ac4d7a14aac319"
  ],
  "focus": "blk_6235791ed9ac4d7a14aac319",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::back

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10393 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::back

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::back --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_6235791ed9ac4d7a14aac319",
    "blk_6235791ed9ac4d7a14aac319"
  ],
  "focus": "blk_6235791ed9ac4d7a14aac319",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::back

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10400 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::context_add

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_add --format json`

```text
{
  "added": [],
  "changed": [
    "blk_a0f46eea06efb92ade78319a"
  ],
  "focus": "blk_a0f46eea06efb92ade78319a",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::context_add

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10399 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::context_add

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_add --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_a0f46eea06efb92ade78319a",
    "blk_a0f46eea06efb92ade78319a"
  ],
  "focus": "blk_a0f46eea06efb92ade78319a",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::context_add

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10406 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::context_clear

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_clear --format json`

```text
{
  "added": [],
  "changed": [
    "blk_68ac17cf0b72cc53bcdd5398"
  ],
  "focus": "blk_68ac17cf0b72cc53bcdd5398",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::context_clear

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10405 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::context_clear

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_clear --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_68ac17cf0b72cc53bcdd5398",
    "blk_68ac17cf0b72cc53bcdd5398"
  ],
  "focus": "blk_68ac17cf0b72cc53bcdd5398",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::context_clear

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10412 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::context_collapse

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_collapse --format json`

```text
{
  "added": [],
  "changed": [
    "blk_90c791014a3ff4d7147bc4ae"
  ],
  "focus": "blk_90c791014a3ff4d7147bc4ae",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::context_collapse

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10411 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::context_collapse

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_collapse --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_90c791014a3ff4d7147bc4ae",
    "blk_90c791014a3ff4d7147bc4ae"
  ],
  "focus": "blk_90c791014a3ff4d7147bc4ae",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::context_collapse

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10418 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::context_expand

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_expand --format json`

```text
{
  "added": [],
  "changed": [
    "blk_999218ae981f6f86a4e6c555"
  ],
  "focus": "blk_999218ae981f6f86a4e6c555",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::context_expand

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10417 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::context_expand

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_expand --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_999218ae981f6f86a4e6c555",
    "blk_999218ae981f6f86a4e6c555"
  ],
  "focus": "blk_999218ae981f6f86a4e6c555",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::context_expand

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10424 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::context_focus

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_focus --format json`

```text
{
  "added": [],
  "changed": [
    "blk_b7e3d6028d7afb93335e0c0e"
  ],
  "focus": "blk_b7e3d6028d7afb93335e0c0e",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::context_focus

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10423 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::context_focus

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_focus --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_b7e3d6028d7afb93335e0c0e",
    "blk_b7e3d6028d7afb93335e0c0e"
  ],
  "focus": "blk_b7e3d6028d7afb93335e0c0e",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::context_focus

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10430 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::context_hydrate

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_hydrate --format json`

```text
{
  "added": [],
  "changed": [
    "blk_2d14aa6d8516a3c73a19a37b"
  ],
  "focus": "blk_2d14aa6d8516a3c73a19a37b",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::context_hydrate

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10429 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::context_hydrate

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_hydrate --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_2d14aa6d8516a3c73a19a37b",
    "blk_2d14aa6d8516a3c73a19a37b"
  ],
  "focus": "blk_2d14aa6d8516a3c73a19a37b",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::context_hydrate

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10436 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::context_pin

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_pin --format json`

```text
{
  "added": [],
  "changed": [
    "blk_4dc709203576a0c9a19d147e"
  ],
  "focus": "blk_4dc709203576a0c9a19d147e",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::context_pin

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10435 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::context_pin

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_pin --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_4dc709203576a0c9a19d147e",
    "blk_4dc709203576a0c9a19d147e"
  ],
  "focus": "blk_4dc709203576a0c9a19d147e",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::context_pin

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10442 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::context_remove

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_remove --format json`

```text
{
  "added": [],
  "changed": [
    "blk_748c018c0ff255072e8400a4"
  ],
  "focus": "blk_748c018c0ff255072e8400a4",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::context_remove

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10441 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::context_remove

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_remove --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_748c018c0ff255072e8400a4",
    "blk_748c018c0ff255072e8400a4"
  ],
  "focus": "blk_748c018c0ff255072e8400a4",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::context_remove

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10448 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::context_seed

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_seed --format json`

```text
{
  "added": [],
  "changed": [
    "blk_6bdfa08751d5a549d77dfeae"
  ],
  "focus": "blk_6bdfa08751d5a549d77dfeae",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::context_seed

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10447 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::context_seed

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::context_seed --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_6bdfa08751d5a549d77dfeae",
    "blk_6bdfa08751d5a549d77dfeae"
  ],
  "focus": "blk_6bdfa08751d5a549d77dfeae",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::context_seed

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10454 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::expand

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::expand --format json`

```text
{
  "added": [],
  "changed": [
    "blk_b2bc11f492b1d0c47f16e483"
  ],
  "focus": "blk_b2bc11f492b1d0c47f16e483",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::expand

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10453 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::expand

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::expand --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_b2bc11f492b1d0c47f16e483",
    "blk_b2bc11f492b1d0c47f16e483"
  ],
  "focus": "blk_b2bc11f492b1d0c47f16e483",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::expand

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10460 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::find

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::find --format json`

```text
{
  "added": [],
  "changed": [
    "blk_33c464f1fa016a2a873ce023"
  ],
  "focus": "blk_33c464f1fa016a2a873ce023",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 268,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::find

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10468 more lines ...
```

## Expand dependencies for symbol:crates/ucp-cli/src/commands/agent.rs::find via uses_symbol

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::find --mode dependencies --relation uses_symbol --format json`

```text
{
  "added": [
    "blk_c66b2666f79b0469d4d25144"
  ],
  "changed": [
    "blk_33c464f1fa016a2a873ce023"
  ],
  "focus": "blk_33c464f1fa016a2a873ce023",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 269,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::find

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::find --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_33c464f1fa016a2a873ce023",
    "blk_33c464f1fa016a2a873ce023"
  ],
  "focus": "blk_33c464f1fa016a2a873ce023",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 269,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::find

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10614 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::follow

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::follow --format json`

```text
{
  "added": [],
  "changed": [
    "blk_f97e4beae70a52e156496818"
  ],
  "focus": "blk_f97e4beae70a52e156496818",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 269,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::follow

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10613 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::follow

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::follow --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_f97e4beae70a52e156496818",
    "blk_f97e4beae70a52e156496818"
  ],
  "focus": "blk_f97e4beae70a52e156496818",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 269,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::follow

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10620 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::goto

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::goto --format json`

```text
{
  "added": [],
  "changed": [
    "blk_caa0b63234a21634ac29a13c"
  ],
  "focus": "blk_caa0b63234a21634ac29a13c",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 269,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::goto

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10628 more lines ...
```

## Expand dependencies for symbol:crates/ucp-cli/src/commands/agent.rs::goto via uses_symbol

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::goto --mode dependencies --relation uses_symbol --format json`

```text
{
  "added": [
    "blk_edb7449d8c5d39eddca9cabc"
  ],
  "changed": [
    "blk_caa0b63234a21634ac29a13c"
  ],
  "focus": "blk_caa0b63234a21634ac29a13c",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::goto

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::goto --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_caa0b63234a21634ac29a13c",
    "blk_caa0b63234a21634ac29a13c"
  ],
  "focus": "blk_caa0b63234a21634ac29a13c",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::goto

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10697 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::handle

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::handle --format json`

```text
{
  "added": [],
  "changed": [
    "blk_388c43cdf82cf3be4c3a2de3"
  ],
  "focus": "blk_388c43cdf82cf3be4c3a2de3",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::handle

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10696 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::handle

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::handle --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_388c43cdf82cf3be4c3a2de3",
    "blk_388c43cdf82cf3be4c3a2de3"
  ],
  "focus": "blk_388c43cdf82cf3be4c3a2de3",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::handle

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10703 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::handle_context

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::handle_context --format json`

```text
{
  "added": [],
  "changed": [
    "blk_d26b84666d6b90b00dc8abb9"
  ],
  "focus": "blk_d26b84666d6b90b00dc8abb9",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::handle_context

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10702 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::handle_context

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::handle_context --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_d26b84666d6b90b00dc8abb9",
    "blk_d26b84666d6b90b00dc8abb9"
  ],
  "focus": "blk_d26b84666d6b90b00dc8abb9",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::handle_context

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10709 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::handle_session

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::handle_session --format json`

```text
{
  "added": [],
  "changed": [
    "blk_ce4deb0d861f1c377b1fb833"
  ],
  "focus": "blk_ce4deb0d861f1c377b1fb833",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::handle_session

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10708 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::handle_session

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::handle_session --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_ce4deb0d861f1c377b1fb833",
    "blk_ce4deb0d861f1c377b1fb833"
  ],
  "focus": "blk_ce4deb0d861f1c377b1fb833",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::handle_session

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10715 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selectors

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selectors --format json`

```text
{
  "added": [],
  "changed": [
    "blk_97aed61a3a17944ab15efac8"
  ],
  "focus": "blk_97aed61a3a17944ab15efac8",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selectors

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10714 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selectors

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selectors --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_97aed61a3a17944ab15efac8",
    "blk_97aed61a3a17944ab15efac8"
  ],
  "focus": "blk_97aed61a3a17944ab15efac8",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selectors

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10721 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::search

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::search --format json`

```text
{
  "added": [],
  "changed": [
    "blk_4e5834618eba57dc2705bf21"
  ],
  "focus": "blk_4e5834618eba57dc2705bf21",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::search

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10720 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::search

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::search --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_4e5834618eba57dc2705bf21",
    "blk_4e5834618eba57dc2705bf21"
  ],
  "focus": "blk_4e5834618eba57dc2705bf21",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::search

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10727 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::session_close

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::session_close --format json`

```text
{
  "added": [],
  "changed": [
    "blk_2f2ff11e7823297e4a3a58ca"
  ],
  "focus": "blk_2f2ff11e7823297e4a3a58ca",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::session_close

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10726 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::session_close

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::session_close --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_2f2ff11e7823297e4a3a58ca",
    "blk_2f2ff11e7823297e4a3a58ca"
  ],
  "focus": "blk_2f2ff11e7823297e4a3a58ca",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::session_close

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10733 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::session_create

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::session_create --format json`

```text
{
  "added": [],
  "changed": [
    "blk_038a0d36c4bfe68f61ae207d"
  ],
  "focus": "blk_038a0d36c4bfe68f61ae207d",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::session_create

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10732 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::session_create

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::session_create --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_038a0d36c4bfe68f61ae207d",
    "blk_038a0d36c4bfe68f61ae207d"
  ],
  "focus": "blk_038a0d36c4bfe68f61ae207d",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::session_create

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10739 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::session_list

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::session_list --format json`

```text
{
  "added": [],
  "changed": [
    "blk_dabdd7fa1f031b5b15d50adf"
  ],
  "focus": "blk_dabdd7fa1f031b5b15d50adf",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::session_list

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10738 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::session_list

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::session_list --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_dabdd7fa1f031b5b15d50adf",
    "blk_dabdd7fa1f031b5b15d50adf"
  ],
  "focus": "blk_dabdd7fa1f031b5b15d50adf",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::session_list

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10745 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::uuid_short

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::uuid_short --format json`

```text
{
  "added": [],
  "changed": [
    "blk_62e8cfd89f05690ea589c053"
  ],
  "focus": "blk_62e8cfd89f05690ea589c053",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::uuid_short

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10744 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::uuid_short

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::uuid_short --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_62e8cfd89f05690ea589c053",
    "blk_62e8cfd89f05690ea589c053"
  ],
  "focus": "blk_62e8cfd89f05690ea589c053",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::uuid_short

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10751 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/agent.rs::view

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::view --format json`

```text
{
  "added": [],
  "changed": [
    "blk_4d6beda7db26a12be6f96d3e"
  ],
  "focus": "blk_4d6beda7db26a12be6f96d3e",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/agent.rs::view

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10750 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/agent.rs::view

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/agent.rs::view --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_4d6beda7db26a12be6f96d3e",
    "blk_4d6beda7db26a12be6f96d3e"
  ],
  "focus": "blk_4d6beda7db26a12be6f96d3e",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/agent.rs::view

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10757 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::build

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::build --format json`

```text
{
  "added": [],
  "changed": [
    "blk_ee2bd5d66010f7ca9ff03751"
  ],
  "focus": "blk_ee2bd5d66010f7ca9ff03751",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::build

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10756 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::build

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::build --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_ee2bd5d66010f7ca9ff03751",
    "blk_ee2bd5d66010f7ca9ff03751"
  ],
  "focus": "blk_ee2bd5d66010f7ca9ff03751",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::build

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10763 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::context

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context --format json`

```text
{
  "added": [],
  "changed": [
    "blk_64e06ba0432fb20a3d983c9e"
  ],
  "focus": "blk_64e06ba0432fb20a3d983c9e",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::context

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10762 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_64e06ba0432fb20a3d983c9e",
    "blk_64e06ba0432fb20a3d983c9e"
  ],
  "focus": "blk_64e06ba0432fb20a3d983c9e",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::context

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10769 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::context_add

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_add --format json`

```text
{
  "added": [],
  "changed": [
    "blk_ecc7305aa5537bc4ba194f70"
  ],
  "focus": "blk_ecc7305aa5537bc4ba194f70",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_add

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10768 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_add

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_add --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_ecc7305aa5537bc4ba194f70",
    "blk_ecc7305aa5537bc4ba194f70"
  ],
  "focus": "blk_ecc7305aa5537bc4ba194f70",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_add

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10775 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::context_collapse

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_collapse --format json`

```text
{
  "added": [],
  "changed": [
    "blk_ccd60e7e0204e6c070b49b3a"
  ],
  "focus": "blk_ccd60e7e0204e6c070b49b3a",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_collapse

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10774 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_collapse

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_collapse --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_ccd60e7e0204e6c070b49b3a",
    "blk_ccd60e7e0204e6c070b49b3a"
  ],
  "focus": "blk_ccd60e7e0204e6c070b49b3a",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_collapse

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10781 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::context_expand

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_expand --format json`

```text
{
  "added": [],
  "changed": [
    "blk_a1024e521fbcbc03beb54a91"
  ],
  "focus": "blk_a1024e521fbcbc03beb54a91",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_expand

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10780 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_expand

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_expand --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_a1024e521fbcbc03beb54a91",
    "blk_a1024e521fbcbc03beb54a91"
  ],
  "focus": "blk_a1024e521fbcbc03beb54a91",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_expand

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10787 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export --format json`

```text
{
  "added": [],
  "changed": [
    "blk_6e236e367cc0078a78fa2bb4"
  ],
  "focus": "blk_6e236e367cc0078a78fa2bb4",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10786 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_6e236e367cc0078a78fa2bb4",
    "blk_6e236e367cc0078a78fa2bb4"
  ],
  "focus": "blk_6e236e367cc0078a78fa2bb4",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10793 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::context_focus

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_focus --format json`

```text
{
  "added": [],
  "changed": [
    "blk_f47e88a42839ca7aecf15597"
  ],
  "focus": "blk_f47e88a42839ca7aecf15597",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_focus

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10792 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_focus

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_focus --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_f47e88a42839ca7aecf15597",
    "blk_f47e88a42839ca7aecf15597"
  ],
  "focus": "blk_f47e88a42839ca7aecf15597",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_focus

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10799 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::context_hydrate

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_hydrate --format json`

```text
{
  "added": [],
  "changed": [
    "blk_28c88ee3ea39d0e262f6b2a1"
  ],
  "focus": "blk_28c88ee3ea39d0e262f6b2a1",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_hydrate

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10798 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_hydrate

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_hydrate --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_28c88ee3ea39d0e262f6b2a1",
    "blk_28c88ee3ea39d0e262f6b2a1"
  ],
  "focus": "blk_28c88ee3ea39d0e262f6b2a1",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_hydrate

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10805 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::context_init

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_init --format json`

```text
{
  "added": [],
  "changed": [
    "blk_60ab5b57aaa6697047e885f7"
  ],
  "focus": "blk_60ab5b57aaa6697047e885f7",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_init

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10804 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_init

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_init --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_60ab5b57aaa6697047e885f7",
    "blk_60ab5b57aaa6697047e885f7"
  ],
  "focus": "blk_60ab5b57aaa6697047e885f7",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_init

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10811 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::context_pin

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_pin --format json`

```text
{
  "added": [],
  "changed": [
    "blk_cd6ef8e63c1b57f336baa2c3"
  ],
  "focus": "blk_cd6ef8e63c1b57f336baa2c3",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_pin

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10810 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_pin

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_pin --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_cd6ef8e63c1b57f336baa2c3",
    "blk_cd6ef8e63c1b57f336baa2c3"
  ],
  "focus": "blk_cd6ef8e63c1b57f336baa2c3",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_pin

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10817 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::context_prune

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_prune --format json`

```text
{
  "added": [],
  "changed": [
    "blk_bab6e0b3b0d86000edbf4749"
  ],
  "focus": "blk_bab6e0b3b0d86000edbf4749",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_prune

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10816 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_prune

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::context_prune --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_bab6e0b3b0d86000edbf4749",
    "blk_bab6e0b3b0d86000edbf4749"
  ],
  "focus": "blk_bab6e0b3b0d86000edbf4749",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::context_prune

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10823 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::detect_commit_hash

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::detect_commit_hash --format json`

```text
{
  "added": [],
  "changed": [
    "blk_e974dc99ca25af3da241c794"
  ],
  "focus": "blk_e974dc99ca25af3da241c794",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::detect_commit_hash

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10822 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::detect_commit_hash

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::detect_commit_hash --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_e974dc99ca25af3da241c794",
    "blk_e974dc99ca25af3da241c794"
  ],
  "focus": "blk_e974dc99ca25af3da241c794",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::detect_commit_hash

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10829 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::ensure_codegraph_document

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::ensure_codegraph_document --format json`

```text
{
  "added": [],
  "changed": [
    "blk_5080e04b8929051c62f21332"
  ],
  "focus": "blk_5080e04b8929051c62f21332",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::ensure_codegraph_document

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10828 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::ensure_codegraph_document

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::ensure_codegraph_document --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_5080e04b8929051c62f21332",
    "blk_5080e04b8929051c62f21332"
  ],
  "focus": "blk_5080e04b8929051c62f21332",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::ensure_codegraph_document

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10835 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session --format json`

```text
{
  "added": [],
  "changed": [
    "blk_9ca3a1a6369f1c436d3bba00"
  ],
  "focus": "blk_9ca3a1a6369f1c436d3bba00",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10834 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_9ca3a1a6369f1c436d3bba00",
    "blk_9ca3a1a6369f1c436d3bba00"
  ],
  "focus": "blk_9ca3a1a6369f1c436d3bba00",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10841 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::handle

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::handle --format json`

```text
{
  "added": [],
  "changed": [
    "blk_329692a490f409a0651ea892"
  ],
  "focus": "blk_329692a490f409a0651ea892",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::handle

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10840 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::handle

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::handle --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_329692a490f409a0651ea892",
    "blk_329692a490f409a0651ea892"
  ],
  "focus": "blk_329692a490f409a0651ea892",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::handle

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10847 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::inspect

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::inspect --format json`

```text
{
  "added": [],
  "changed": [
    "blk_4ab1409d012f04700dfae70c"
  ],
  "focus": "blk_4ab1409d012f04700dfae70c",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 270,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::inspect

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10855 more lines ...
```

## Expand dependencies for symbol:crates/ucp-cli/src/commands/codegraph.rs::inspect via uses_symbol

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::inspect --mode dependencies --relation uses_symbol --format json`

```text
{
  "added": [
    "blk_9568dce7e06e7b9a39c9507c"
  ],
  "changed": [
    "blk_4ab1409d012f04700dfae70c"
  ],
  "focus": "blk_4ab1409d012f04700dfae70c",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 271,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::inspect

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::inspect --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_4ab1409d012f04700dfae70c",
    "blk_4ab1409d012f04700dfae70c"
  ],
  "focus": "blk_4ab1409d012f04700dfae70c",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 271,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::inspect

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10931 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::merge_updates

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::merge_updates --format json`

```text
{
  "added": [],
  "changed": [
    "blk_9144e5729789e34ecae9f578"
  ],
  "focus": "blk_9144e5729789e34ecae9f578",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 271,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::merge_updates

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10930 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::merge_updates

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::merge_updates --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_9144e5729789e34ecae9f578",
    "blk_9144e5729789e34ecae9f578"
  ],
  "focus": "blk_9144e5729789e34ecae9f578",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 271,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::merge_updates

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10937 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::prompt

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::prompt --format json`

```text
{
  "added": [],
  "changed": [
    "blk_e6b2d1db3b0de2651d0f96f2"
  ],
  "focus": "blk_e6b2d1db3b0de2651d0f96f2",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 271,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::prompt

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 10945 more lines ...
```

## Expand dependencies for symbol:crates/ucp-cli/src/commands/codegraph.rs::prompt via uses_symbol

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::prompt --mode dependencies --relation uses_symbol --format json`

```text
{
  "added": [
    "blk_58e7f489bddb3c7b220a5ff6"
  ],
  "changed": [
    "blk_e6b2d1db3b0de2651d0f96f2"
  ],
  "focus": "blk_e6b2d1db3b0de2651d0f96f2",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::prompt

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::prompt --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_e6b2d1db3b0de2651d0f96f2",
    "blk_e6b2d1db3b0de2651d0f96f2"
  ],
  "focus": "blk_e6b2d1db3b0de2651d0f96f2",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::prompt

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11007 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selectors

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selectors --format json`

```text
{
  "added": [],
  "changed": [
    "blk_e0d658f88c35822f46fe7f89"
  ],
  "focus": "blk_e0d658f88c35822f46fe7f89",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selectors

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11006 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selectors

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selectors --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_e0d658f88c35822f46fe7f89",
    "blk_e0d658f88c35822f46fe7f89"
  ],
  "focus": "blk_e0d658f88c35822f46fe7f89",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selectors

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11013 more lines ...
```

## Focus symbol:crates/ucp-cli/src/commands/codegraph.rs::uuid_short

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::uuid_short --format json`

```text
{
  "added": [],
  "changed": [
    "blk_bca85a9fddb9c1cf661e95ab"
  ],
  "focus": "blk_bca85a9fddb9c1cf661e95ab",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/commands/codegraph.rs::uuid_short

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11012 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/commands/codegraph.rs::uuid_short

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/commands/codegraph.rs::uuid_short --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_bca85a9fddb9c1cf661e95ab",
    "blk_bca85a9fddb9c1cf661e95ab"
  ],
  "focus": "blk_bca85a9fddb9c1cf661e95ab",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/commands/codegraph.rs::uuid_short

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11019 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::AgentSessionState

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::AgentSessionState --format json`

```text
{
  "added": [],
  "changed": [
    "blk_8200e68767cfb0ef6ab7ba79"
  ],
  "focus": "blk_8200e68767cfb0ef6ab7ba79",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::AgentSessionState

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11018 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::AgentSessionState --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_8200e68767cfb0ef6ab7ba79",
    "blk_8200e68767cfb0ef6ab7ba79"
  ],
  "focus": "blk_8200e68767cfb0ef6ab7ba79",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::AgentSessionState

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11025 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::AgentSessionState#49

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::AgentSessionState#49 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_188d0a5a3eecda3b89bd6663"
  ],
  "focus": "blk_188d0a5a3eecda3b89bd6663",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::AgentSessionState#49

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11024 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::AgentSessionState#49

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::AgentSessionState#49 --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_188d0a5a3eecda3b89bd6663",
    "blk_188d0a5a3eecda3b89bd6663"
  ],
  "focus": "blk_188d0a5a3eecda3b89bd6663",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::AgentSessionState#49

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11031 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::CliState

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::CliState --format json`

```text
{
  "added": [],
  "changed": [
    "blk_bff602b27a6fde8e2ffa5c83"
  ],
  "focus": "blk_bff602b27a6fde8e2ffa5c83",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::CliState

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11030 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::CliState

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::CliState --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_bff602b27a6fde8e2ffa5c83",
    "blk_bff602b27a6fde8e2ffa5c83"
  ],
  "focus": "blk_bff602b27a6fde8e2ffa5c83",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::CliState

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11037 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::CliState#29

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::CliState#29 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_8ba9a2e4b16e2822a83ba3ee"
  ],
  "focus": "blk_8ba9a2e4b16e2822a83ba3ee",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::CliState#29

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11036 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::CliState#29

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::CliState#29 --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_8ba9a2e4b16e2822a83ba3ee",
    "blk_8ba9a2e4b16e2822a83ba3ee"
  ],
  "focus": "blk_8ba9a2e4b16e2822a83ba3ee",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::CliState#29

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11043 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::SavepointInfo

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::SavepointInfo --format json`

```text
{
  "added": [],
  "changed": [
    "blk_c781cee8feef9b724dc5267b"
  ],
  "focus": "blk_c781cee8feef9b724dc5267b",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::SavepointInfo

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11042 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::SavepointInfo

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::SavepointInfo --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_c781cee8feef9b724dc5267b",
    "blk_c781cee8feef9b724dc5267b"
  ],
  "focus": "blk_c781cee8feef9b724dc5267b",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::SavepointInfo

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11049 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::SnapshotInfo

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::SnapshotInfo --format json`

```text
{
  "added": [],
  "changed": [
    "blk_701d4a158570aac82367aa83"
  ],
  "focus": "blk_701d4a158570aac82367aa83",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::SnapshotInfo

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11048 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::SnapshotInfo

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::SnapshotInfo --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_701d4a158570aac82367aa83",
    "blk_701d4a158570aac82367aa83"
  ],
  "focus": "blk_701d4a158570aac82367aa83",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::SnapshotInfo

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11055 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::SnapshotInfo#128

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::SnapshotInfo#128 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_e226efa27ee49766852deed2"
  ],
  "focus": "blk_e226efa27ee49766852deed2",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::SnapshotInfo#128

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11054 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::SnapshotInfo#128

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::SnapshotInfo#128 --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_e226efa27ee49766852deed2",
    "blk_e226efa27ee49766852deed2"
  ],
  "focus": "blk_e226efa27ee49766852deed2",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::SnapshotInfo#128

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11061 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::StatefulDocument

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::StatefulDocument --format json`

```text
{
  "added": [],
  "changed": [
    "blk_e171de84c619ae784f69d034"
  ],
  "focus": "blk_e171de84c619ae784f69d034",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::StatefulDocument

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11060 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::StatefulDocument

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::StatefulDocument --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_e171de84c619ae784f69d034",
    "blk_e171de84c619ae784f69d034"
  ],
  "focus": "blk_e171de84c619ae784f69d034",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::StatefulDocument

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11067 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::StatefulDocument#222

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::StatefulDocument#222 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_3454f118d408655205b7a81f"
  ],
  "focus": "blk_3454f118d408655205b7a81f",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::StatefulDocument#222

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11066 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::StatefulDocument#222

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::StatefulDocument#222 --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_3454f118d408655205b7a81f",
    "blk_3454f118d408655205b7a81f"
  ],
  "focus": "blk_3454f118d408655205b7a81f",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::StatefulDocument#222

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11073 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::StatefulDocumentJson

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::StatefulDocumentJson --format json`

```text
{
  "added": [],
  "changed": [
    "blk_94facbc10fc4336f57a1edfa"
  ],
  "focus": "blk_94facbc10fc4336f57a1edfa",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::StatefulDocumentJson

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11072 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::StatefulDocumentJson

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::StatefulDocumentJson --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_94facbc10fc4336f57a1edfa",
    "blk_94facbc10fc4336f57a1edfa"
  ],
  "focus": "blk_94facbc10fc4336f57a1edfa",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::StatefulDocumentJson

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11079 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::TransactionState

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::TransactionState --format json`

```text
{
  "added": [],
  "changed": [
    "blk_6bef18c64e4b48ec6b16199d"
  ],
  "focus": "blk_6bef18c64e4b48ec6b16199d",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::TransactionState

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11078 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::TransactionState

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::TransactionState --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_6bef18c64e4b48ec6b16199d",
    "blk_6bef18c64e4b48ec6b16199d"
  ],
  "focus": "blk_6bef18c64e4b48ec6b16199d",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::TransactionState

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11085 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::TransactionState#160

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::TransactionState#160 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_d70999acf47b26fb7b615535"
  ],
  "focus": "blk_d70999acf47b26fb7b615535",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::TransactionState#160

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11084 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::TransactionState#160

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::TransactionState#160 --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_d70999acf47b26fb7b615535",
    "blk_d70999acf47b26fb7b615535"
  ],
  "focus": "blk_d70999acf47b26fb7b615535",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::TransactionState#160

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11091 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::read_stateful_document

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::read_stateful_document --format json`

```text
{
  "added": [],
  "changed": [
    "blk_8812e3cd9c2deda7699c27ff"
  ],
  "focus": "blk_8812e3cd9c2deda7699c27ff",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 272,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::read_stateful_document

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11099 more lines ...
```

## Expand dependents for symbol:crates/ucp-cli/src/state.rs::read_stateful_document via uses_symbol

`$ cargo run -q -p ucp-cli -- codegraph context expand --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::read_stateful_document --mode dependents --relation uses_symbol --format json`

```text
{
  "added": [
    "blk_3fde28b45f03c338c5b64a98",
    "blk_c354450f499781aae98067b8",
    "blk_7061689c85a7207c38183598",
    "blk_09b86753f44c655f15d3b2e8",
    "blk_2777f85b37a199a8ad19ab09",
    "blk_0ea00e5efeceb6787cb4c648",
    "blk_e007a89608a674ff3448b23f",
    "blk_8dfcd52700c99e1e29e0a098",
    "blk_55b91ce2213714a9ba8aaa39",
    "blk_3ef0dda59fee0db87fd50fd2"
  ],
  "changed": [
    "blk_8812e3cd9c2deda7699c27ff"
  ],
  "focus": "blk_8812e3cd9c2deda7699c27ff",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 282,
  "warnings": []
}
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::read_stateful_document

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::read_stateful_document --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_8812e3cd9c2deda7699c27ff",
    "blk_8812e3cd9c2deda7699c27ff"
  ],
  "focus": "blk_8812e3cd9c2deda7699c27ff",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 282,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::read_stateful_document

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11524 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::tests

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::tests --format json`

```text
{
  "added": [],
  "changed": [
    "blk_935b8bb38a66bb3fea2a47ba"
  ],
  "focus": "blk_935b8bb38a66bb3fea2a47ba",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 282,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::tests

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11523 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::tests

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::tests --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_935b8bb38a66bb3fea2a47ba",
    "blk_935b8bb38a66bb3fea2a47ba"
  ],
  "focus": "blk_935b8bb38a66bb3fea2a47ba",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 282,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::tests

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11530 more lines ...
```

## Focus symbol:crates/ucp-cli/src/state.rs::write_stateful_document

`$ cargo run -q -p ucp-cli -- codegraph context focus --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::write_stateful_document --format json`

```text
{
  "added": [],
  "changed": [
    "blk_e91699dcdbeccecf2e3faacc"
  ],
  "focus": "blk_e91699dcdbeccecf2e3faacc",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 282,
  "warnings": []
}
```

## Export frontier for symbol:crates/ucp-cli/src/state.rs::write_stateful_document

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11529 more lines ...
```

## Hydrate source for symbol:crates/ucp-cli/src/state.rs::write_stateful_document

`$ cargo run -q -p ucp-cli -- codegraph context hydrate --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 symbol:crates/ucp-cli/src/state.rs::write_stateful_document --padding 2 --format json`

```text
{
  "added": [],
  "changed": [
    "blk_e91699dcdbeccecf2e3faacc",
    "blk_e91699dcdbeccecf2e3faacc"
  ],
  "focus": "blk_e91699dcdbeccecf2e3faacc",
  "removed": [],
  "session": "cgctx_b882aa45",
  "success": true,
  "total": 282,
  "warnings": []
}
```

## Export updated working set for symbol:crates/ucp-cli/src/state.rs::write_stateful_document

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11536 more lines ...
```

## Export the final structured context

`$ cargo run -q -p ucp-cli -- codegraph context export --input /tmp/ucp-codegraph-demo-j0r2kbqe/ucp-codegraph.json --session cgctx_b882aa45 --format json`

```text
{
  "edges": [
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "a57c62e171851759fa0e4e4d",
      "target_short_id": "F11"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "77e09f54fd35c514549687ae",
      "target_short_id": "F8"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "39da0589489463612b2d49e0",
      "source_short_id": "F10",
      "target": "c688870a246f46d42e576de2",
      "target_short_id": "F9"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "83e09d4445d4a857dcf00bcd",
      "target_short_id": "F104"
    },
    {
      "relation": "references",
      "source": "dfbac060b9a6772b06f21def",
      "source_short_id": "F101",
      "target": "5470c69ddc19b49e181404fd",
      "target_short_id": "F113"
    },
    {
... clipped 11536 more lines ...
```

## Read coderef-backed excerpts from the final working set

### S25 `symbol:crates/ucp-cli/src/commands/agent.rs::session_create`

- ref: `crates/ucp-cli/src/commands/agent.rs:73-119`

```rust
  71 }
  72 
  73 fn session_create(
  74     input: Option<String>,
  75     name: Option<String>,
  76     start: Option<String>,
  77     format: OutputFormat,
  78 ) -> Result<()> {
  79     let mut stateful = read_stateful_document(input.clone())?;
  80 
  81     let start_block = if let Some(s) = start {
  82         Some(BlockId::from_str(&s).map_err(|_| anyhow!("Invalid start block ID: {}", s))?)
  83     } else {
  84         None
  85     };
  86 
  87     // Generate session ID
  88     let session_id = format!("sess_{}", uuid_short());
  89 
  90     let session = AgentSessionState::new(session_id.clone(), name.clone(), start_block);
  91     stateful
  92         .state_mut()
  93         .sessions
  94         .insert(session_id.clone(), session);
  95 
  96     match format {
  97         OutputFormat::Json => {
  98             #[derive(Serialize)]
  99             struct SessionResult {
 100                 success: bool,
 101                 session_id: String,
 102                 name: Option<String>,
 103             }
 104             let result = SessionResult {
 105                 success: true,
 106                 session_id,
 107                 name,
 108             };
 109             println!("{}", serde_json::to_string_pretty(&result)?);
 110         }
 111         OutputFormat::Text => {
 112             print_success(&format!("Created session: {}", session_id));
 113         }
 114     }
 115 
 116     // Write back to same input file or stdout
 117     write_stateful_document(&stateful, input)?;
 118     Ok(())
 119 }
 120 
 121 fn session_list(input: Option<String>, format: OutputFormat) -> Result<()> {
```
### S71 `symbol:crates/ucp-cli/src/state.rs::AgentSessionState#49`

- ref: `crates/ucp-cli/src/state.rs:49-116`

```rust
  47 }
  48 
  49 impl AgentSessionState {
  50     pub fn new(id: String, name: Option<String>, start_block: Option<BlockId>) -> Self {
  51         Self {
  52             id,
  53             name,
  54             current_block: start_block.map(|b| b.to_string()),
  55             history: Vec::new(),
  56             context_blocks: Vec::new(),
  57             codegraph_context: None,
  58             state: "active".to_string(),
  59             created_at: chrono::Utc::now().to_rfc3339(),
  60         }
  61     }
  62 
  63     pub fn goto(&mut self, block_id: &BlockId) {
  64         if let Some(current) = &self.current_block {
  65             self.history.push(current.clone());
  66         }
  67         self.current_block = Some(block_id.to_string());
  68     }
  69 
  70     pub fn back(&mut self, steps: usize) -> Option<BlockId> {
  71         use std::str::FromStr;
  72         for _ in 0..steps {
  73             if let Some(prev) = self.history.pop() {
  74                 self.current_block = Some(prev);
  75             }
  76         }
  77         self.current_block
  78             .as_ref()
  79             .and_then(|s| BlockId::from_str(s).ok())
  80     }
  81 
  82     pub fn add_to_context(&mut self, block_id: &BlockId) {
  83         let id_str = block_id.to_string();
  84         if !self.context_blocks.contains(&id_str) {
  85             self.context_blocks.push(id_str);
  86         }
  87     }
  88 
  89     pub fn remove_from_context(&mut self, block_id: &BlockId) {
  90         let id_str = block_id.to_string();
  91         self.context_blocks.retain(|b| b != &id_str);
  92     }
  93 
  94     #[allow(dead_code)]
  95     pub fn clear_context(&mut self) {
  96         self.context_blocks.clear();
  97         if let Some(context) = self.codegraph_context.as_mut() {
  98             context.clear();
  99         }
 100     }
 101 
 102     pub fn ensure_codegraph_context(&mut self) -> &mut CodeGraphContextSession {
 103         self.codegraph_context
 104             .get_or_insert_with(CodeGraphContextSession::new)
 105     }
 106 
 107     pub fn sync_context_blocks_from_codegraph(&mut self) {
 108         if let Some(context) = self.codegraph_context.as_ref() {
 109             self.context_blocks = context
 110                 .selected_block_ids()
 111                 .into_iter()
 112                 .map(|block_id| block_id.to_string())
 113                 .collect();
 114         }
 115     }
 116 }
 117 
 118 /// Serializable snapshot info
```
### S36 `symbol:crates/ucp-cli/src/commands/codegraph.rs::context_hydrate`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:536-558`

```rust
 534 }
 535 
 536 fn context_hydrate(
 537     input: Option<String>,
 538     session: String,
 539     target: String,
 540     padding: usize,
 541     format: OutputFormat,
 542 ) -> Result<()> {
 543     let mut stateful = read_stateful_document(input.clone())?;
 544     ensure_codegraph_document(&stateful.document)?;
 545     let document = stateful.document.clone();
 546     let block_id = resolve_selector(&document, &target)?;
 547 
 548     let sess = get_session_mut(&mut stateful, &session)?;
 549     let update = sess
 550         .ensure_codegraph_context()
 551         .hydrate_source(&document, block_id, padding);
 552     sess.sync_context_blocks_from_codegraph();
 553     sess.current_block = update.focus.map(|id| id.to_string());
 554 
 555     print_context_update(format, &session, &update, sess)?;
 556     write_stateful_document(&stateful, input)?;
 557     Ok(())
 558 }
 559 
 560 fn context_collapse(
```
### S7 `symbol:crates/ucp-cli/src/commands/agent.rs::context_hydrate`

- ref: `crates/ucp-cli/src/commands/agent.rs:844-868`

```rust
 842 }
 843 
 844 fn context_hydrate(
 845     input: Option<String>,
 846     session: String,
 847     target: String,
 848     padding: usize,
 849     format: OutputFormat,
 850 ) -> Result<()> {
 851     let mut stateful = read_stateful_document(input.clone())?;
 852     let document = stateful.document.clone();
 853     if !is_codegraph_document(&document) {
 854         return Err(anyhow!("context hydrate currently requires a codegraph document"));
 855     }
 856     let block_id = resolve_selector(&document, &target)?;
 857 
 858     let sess = get_session_mut(&mut stateful, &session)?;
 859     let update = sess
 860         .ensure_codegraph_context()
 861         .hydrate_source(&document, block_id, padding);
 862     sess.sync_context_blocks_from_codegraph();
 863     sess.current_block = update.focus.map(|id| id.to_string());
 864 
 865     print_context_update(format, &session, &update, sess.context_blocks.len(), "Hydrated source")?;
 866     write_stateful_document(&stateful, input)?;
 867     Ok(())
 868 }
 869 
 870 fn context_collapse(
```
### S24 `symbol:crates/ucp-cli/src/commands/agent.rs::session_close`

- ref: `crates/ucp-cli/src/commands/agent.rs:172-193`

```rust
 170 }
 171 
 172 fn session_close(session: String, format: OutputFormat) -> Result<()> {
 173     // Note: Without input file, we can't actually persist this change
 174     // In practice, the user should provide an input file
 175     match format {
 176         OutputFormat::Json => {
 177             #[derive(Serialize)]
 178             struct SessionResult {
 179                 success: bool,
 180                 session_id: String,
 181             }
 182             let result = SessionResult {
 183                 success: true,
 184                 session_id: session,
 185             };
 186             println!("{}", serde_json::to_string_pretty(&result)?);
 187         }
 188         OutputFormat::Text => {
 189             print_success(&format!("Session {} closed", session));
 190         }
 191     }
 192     Ok(())
 193 }
 194 
 195 fn goto(
```
### S45 `symbol:crates/ucp-cli/src/commands/codegraph.rs::handle`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:25-53`

```rust
  23 };
  24 
  25 pub fn handle(cmd: CodegraphCommands, format: OutputFormat) -> Result<()> {
  26     match cmd {
  27         CodegraphCommands::Build {
  28             repo,
  29             commit,
  30             output,
  31             extensions,
  32             include_hidden,
  33             no_export_edges,
  34             fail_on_parse_error,
  35             max_file_bytes,
  36             allow_partial,
  37         } => build(
  38             repo,
  39             commit,
  40             output,
  41             extensions,
  42             include_hidden,
  43             no_export_edges,
  44             fail_on_parse_error,
  45             max_file_bytes,
  46             allow_partial,
  47             format,
  48         ),
  49         CodegraphCommands::Inspect { input } => inspect(input, format),
  50         CodegraphCommands::Prompt { input, output } => prompt(input, output, format),
  51         CodegraphCommands::Context(cmd) => context(cmd, format),
  52     }
  53 }
  54 
  55 #[allow(clippy::too_many_arguments)]
```
### S13 `symbol:crates/ucp-cli/src/commands/agent.rs::find`

- ref: `crates/ucp-cli/src/commands/agent.rs:542-600`

```rust
 540 }
 541 
 542 fn find(
 543     input: Option<String>,
 544     _session: String,
 545     role: Option<String>,
 546     tag: Option<String>,
 547     format: OutputFormat,
 548 ) -> Result<()> {
 549     let doc = read_document(input)?;
 550 
 551     let matches: Vec<_> = doc
 552         .blocks
 553         .values()
 554         .filter(|block| {
 555             if let Some(ref r) = role {
 556                 if let Some(ref block_role) = block.metadata.semantic_role {
 557                     let role_str = block_role.to_string();
 558                     if !role_str.to_lowercase().contains(&r.to_lowercase()) {
 559                         return false;
 560                     }
 561                 } else {
 562                     return false;
 563                 }
 564             }
 565             if let Some(ref t) = tag {
 566                 if !block.metadata.tags.iter().any(|bt| bt.contains(t)) {
 567                     return false;
 568                 }
 569             }
 570             true
 571         })
 572         .collect();
 573 
 574     match format {
 575         OutputFormat::Json => {
 576             #[derive(Serialize)]
 577             struct FindResult {
 578                 count: usize,
 579                 blocks: Vec<String>,
 580             }
 581             let result = FindResult {
 582                 count: matches.len(),
 583                 blocks: matches.iter().map(|b| b.id.to_string()).collect(),
 584             };
 585             println!("{}", serde_json::to_string_pretty(&result)?);
 586         }
 587         OutputFormat::Text => {
 588             if matches.is_empty() {
 589                 println!("No matching blocks found");
 590             } else {
 591                 println!("Found {} blocks:", matches.len());
 592                 for block in matches {
 593                     println!("  {}", block.id);
 594                 }
 595             }
 596         }
 597     }
 598 
 599     Ok(())
 600 }
 601 
 602 fn handle_context(cmd: AgentContextCommands, format: OutputFormat) -> Result<()> {
```
### S78 `symbol:crates/ucp-cli/src/state.rs::StatefulDocument#222`

- ref: `crates/ucp-cli/src/state.rs:222-245`

```rust
 220 }
 221 
 222 impl StatefulDocument {
 223     #[allow(dead_code)]
 224     pub fn new(document: Document) -> Self {
 225         Self {
 226             document,
 227             cli_state: CliState::new(),
 228         }
 229     }
 230 
 231     pub fn from_document(document: Document) -> Self {
 232         Self {
 233             document,
 234             cli_state: CliState::new(),
 235         }
 236     }
 237 
 238     pub fn state(&self) -> &CliState {
 239         &self.cli_state
 240     }
 241 
 242     pub fn state_mut(&mut self) -> &mut CliState {
 243         &mut self.cli_state
 244     }
 245 }
 246 
 247 /// Read a stateful document from file or stdin
```
### S17 `symbol:crates/ucp-cli/src/commands/agent.rs::handle`

- ref: `crates/ucp-cli/src/commands/agent.rs:17-61`

```rust
  15 use crate::state::{read_stateful_document, write_stateful_document, AgentSessionState};
  16 
  17 pub fn handle(cmd: AgentCommands, format: OutputFormat) -> Result<()> {
  18     match cmd {
  19         AgentCommands::Session(subcmd) => handle_session(subcmd, format),
  20         AgentCommands::Goto {
  21             input,
  22             session,
  23             target,
  24         } => goto(input, session, target, format),
  25         AgentCommands::Back {
  26             input,
  27             session,
  28             steps,
  29         } => back(input, session, steps, format),
  30         AgentCommands::Expand {
  31             input,
  32             session,
  33             id,
  34             direction,
  35             depth,
  36         } => expand(input, session, id, direction, depth, format),
  37         AgentCommands::Follow {
  38             input,
  39             session,
  40             edge_type,
  41         } => follow(input, session, edge_type, format),
  42         AgentCommands::Search {
  43             input,
  44             session,
  45             query,
  46             limit,
  47         } => search(input, session, query, limit, format),
  48         AgentCommands::Find {
  49             input,
  50             session,
  51             role,
  52             tag,
  53         } => find(input, session, role, tag, format),
  54         AgentCommands::Context(subcmd) => handle_context(subcmd, format),
  55         AgentCommands::View {
  56             input,
  57             session,
  58             mode,
  59         } => view(input, session, mode, format),
  60     }
  61 }
  62 
  63 fn handle_session(cmd: AgentSessionCommands, format: OutputFormat) -> Result<()> {
```
### F55 `file:crates/ucp-cli/src/commands/agent.rs`

- ref: `crates/ucp-cli/src/commands/agent.rs:None-None`

```rust
   1 //! Agent traversal commands
   2 
   3 use anyhow::{anyhow, Result};
```
### S46 `symbol:crates/ucp-cli/src/commands/codegraph.rs::inspect`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:199-243`

```rust
 197 }
 198 
 199 fn inspect(input: Option<String>, format: OutputFormat) -> Result<()> {
 200     let doc = read_document(input)?;
 201     let validation = validate_code_graph_profile(&doc);
 202     let fingerprint = canonical_fingerprint(&doc)?;
 203 
 204     match format {
 205         OutputFormat::Json => {
 206             #[derive(Serialize)]
 207             struct InspectResult {
 208                 valid: bool,
 209                 canonical_fingerprint: String,
 210                 diagnostics: Vec<ucp_api::CodeGraphDiagnostic>,
 211             }
 212 
 213             let result = InspectResult {
 214                 valid: validation.valid,
 215                 canonical_fingerprint: fingerprint,
 216                 diagnostics: validation.diagnostics,
 217             };
 218             println!("{}", serde_json::to_string_pretty(&result)?);
 219         }
 220         OutputFormat::Text => {
 221             if validation.valid {
 222                 print_success("CodeGraph profile validation passed");
 223             } else {
 224                 print_error("CodeGraph profile validation failed");
 225             }
 226             println!("canonical_fingerprint: {}", fingerprint);
 227 
 228             if !validation.diagnostics.is_empty() {
 229                 println!("{}", "diagnostics:".yellow().bold());
 230                 for diag in validation.diagnostics {
 231                     let level = match diag.severity {
 232                         CodeGraphSeverity::Error => "ERROR".red().bold(),
 233                         CodeGraphSeverity::Warning => "WARN".yellow().bold(),
 234                         CodeGraphSeverity::Info => "INFO".blue().bold(),
 235                     };
 236                     println!("  {} {} ({})", level, diag.message, diag.code);
 237                 }
 238             }
 239         }
 240     }
 241 
 242     Ok(())
 243 }
 244 
 245 fn prompt(input: Option<String>, output: Option<String>, format: OutputFormat) -> Result<()> {
```
### S21 `symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selector`

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
### S28 `symbol:crates/ucp-cli/src/commands/agent.rs::view`

- ref: `crates/ucp-cli/src/commands/agent.rs:1094-1132`

```rust
1092 }
1093 
1094 fn view(input: Option<String>, session: String, mode: String, format: OutputFormat) -> Result<()> {
1095     let stateful = read_stateful_document(input)?;
1096 
1097     let sess = stateful
1098         .state()
1099         .sessions
1100         .get(&session)
1101         .ok_or_else(|| anyhow!("Session not found: {}", session))?;
1102 
1103     let current = sess
1104         .current_block
1105         .as_ref()
1106         .ok_or_else(|| anyhow!("No current position"))?;
1107     let block_id = BlockId::from_str(current)?;
1108 
1109     let block = stateful
1110         .document
1111         .get_block(&block_id)
1112         .ok_or_else(|| anyhow!("Current block not found"))?;
1113 
1114     match format {
1115         OutputFormat::Json => match mode.as_str() {
1116             "metadata" => {
1117                 println!("{}", serde_json::to_string_pretty(&block.metadata)?);
1118             }
1119             "ids" => {
1120                 println!("{}", serde_json::to_string(&block.id)?);
1121             }
1122             _ => {
1123                 println!("{}", serde_json::to_string_pretty(block)?);
1124             }
1125         },
1126         OutputFormat::Text => {
1127             print_block(block, mode != "metadata");
1128         }
1129     }
1130 
1131     Ok(())
1132 }
1133 
1134 /// Generate a short UUID-like string
```
### S8 `symbol:crates/ucp-cli/src/commands/agent.rs::context_pin`

- ref: `crates/ucp-cli/src/commands/agent.rs:896-931`

```rust
 894 }
 895 
 896 fn context_pin(
 897     input: Option<String>,
 898     session: String,
 899     target: String,
 900     pinned: bool,
 901     format: OutputFormat,
 902 ) -> Result<()> {
 903     let mut stateful = read_stateful_document(input.clone())?;
 904     let document = stateful.document.clone();
 905     if !is_codegraph_document(&document) {
 906         return Err(anyhow!("context pin currently requires a codegraph document"));
 907     }
 908     let block_id = resolve_selector(&document, &target)?;
 909 
 910     let sess = get_session_mut(&mut stateful, &session)?;
 911     sess.ensure_codegraph_context().pin(block_id, pinned);
 912     sess.sync_context_blocks_from_codegraph();
 913 
 914     match format {
 915         OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&serde_json::json!({
 916             "success": true,
 917             "session": session,
 918             "target": block_id.to_string(),
 919             "pinned": pinned,
 920             "total": sess.context_blocks.len()
 921         }))?),
 922         OutputFormat::Text => print_success(&format!(
 923             "{} {}",
 924             if pinned { "Pinned" } else { "Unpinned" },
 925             block_id
 926         )),
 927     }
 928 
 929     write_stateful_document(&stateful, input)?;
 930     Ok(())
 931 }
 932 
 933 fn context_clear(input: Option<String>, session: String, format: OutputFormat) -> Result<()> {
```
### S23 `symbol:crates/ucp-cli/src/commands/agent.rs::search`

- ref: `crates/ucp-cli/src/commands/agent.rs:478-540`

```rust
 476 }
 477 
 478 fn search(
 479     input: Option<String>,
 480     _session: String,
 481     query: String,
 482     limit: usize,
 483     format: OutputFormat,
 484 ) -> Result<()> {
 485     let doc = read_document(input)?;
 486 
 487     // Simple text search (in a real implementation, this would use RAG)
 488     let query_lower = query.to_lowercase();
 489     let matches: Vec<_> = doc
 490         .blocks
 491         .values()
 492         .filter(|block| {
 493             let content_str = content_preview(&block.content, 10000).to_lowercase();
 494             content_str.contains(&query_lower)
 495         })
 496         .take(limit)
 497         .collect();
 498 
 499     match format {
 500         OutputFormat::Json => {
 501             #[derive(Serialize)]
 502             struct SearchResult {
 503                 query: String,
 504                 matches: Vec<SearchMatch>,
 505             }
 506             #[derive(Serialize)]
 507             struct SearchMatch {
 508                 id: String,
 509                 content_type: String,
 510                 preview: String,
 511             }
 512             let result = SearchResult {
 513                 query,
 514                 matches: matches
 515                     .iter()
 516                     .map(|b| SearchMatch {
 517                         id: b.id.to_string(),
 518                         content_type: b.content.type_tag().to_string(),
 519                         preview: content_preview(&b.content, 100),
 520                     })
 521                     .collect(),
 522             };
 523             println!("{}", serde_json::to_string_pretty(&result)?);
 524         }
 525         OutputFormat::Text => {
 526             if matches.is_empty() {
 527                 println!("No matches found for '{}'", query);
 528             } else {
 529                 println!("Found {} matches for '{}':", matches.len(), query.green());
 530                 for block in matches {
 531                     let preview = content_preview(&block.content, 80);
 532                     let preview_line = preview.lines().next().unwrap_or("");
 533                     println!("  [{}] {}", block.id.to_string().yellow(), preview_line);
 534                 }
 535             }
 536         }
 537     }
 538 
 539     Ok(())
 540 }
 541 
 542 fn find(
```
### S42 `symbol:crates/ucp-cli/src/commands/codegraph.rs::ensure_codegraph_document`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:625-631`

```rust
 623 }
 624 
 625 fn ensure_codegraph_document(doc: &Document) -> Result<()> {
 626     if is_codegraph_document(doc) {
 627         Ok(())
 628     } else {
 629         Err(anyhow!("document is not a codegraph"))
 630     }
 631 }
 632 
 633 fn resolve_selectors(doc: &Document, selectors: &str) -> Result<Vec<BlockId>> {
```
### S44 `symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session_mut`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:655-664`

```rust
 653 }
 654 
 655 fn get_session_mut<'a>(
 656     stateful: &'a mut StatefulDocument,
 657     session: &str,
 658 ) -> Result<&'a mut AgentSessionState> {
 659     stateful
 660         .state_mut()
 661         .sessions
 662         .get_mut(session)
 663         .ok_or_else(|| anyhow!("Session not found: {}", session))
 664 }
 665 
 666 fn merge_updates(into: &mut CodeGraphContextUpdate, next: CodeGraphContextUpdate) {
```
### S37 `symbol:crates/ucp-cli/src/commands/codegraph.rs::context_init`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:337-388`

```rust
 335 }
 336 
 337 fn context_init(
 338     input: Option<String>,
 339     name: Option<String>,
 340     max_selected: usize,
 341     format: OutputFormat,
 342 ) -> Result<()> {
 343     let mut stateful = read_stateful_document(input.clone())?;
 344     ensure_codegraph_document(&stateful.document)?;
 345 
 346     let session_id = format!("cgctx_{}", uuid_short());
 347     let mut session = AgentSessionState::new(session_id.clone(), name.clone(), None);
 348     {
 349         let context = session.ensure_codegraph_context();
 350         context.set_prune_policy(CodeGraphPrunePolicy {
 351             max_selected: max_selected.max(1),
 352             ..CodeGraphPrunePolicy::default()
 353         });
 354         let update = context.seed_overview(&stateful.document);
 355         session.current_block = update.focus.map(|id| id.to_string());
 356         session.sync_context_blocks_from_codegraph();
 357     }
 358     stateful
 359         .state_mut()
 360         .sessions
 361         .insert(session_id.clone(), session.clone());
 362 
 363     match format {
 364         OutputFormat::Json => {
 365             let rendered = render_codegraph_context_prompt(
 366                 &stateful.document,
 367                 session.codegraph_context.as_ref().expect("context seeded"),
 368                 &CodeGraphRenderConfig::default(),
 369             );
 370             println!(
 371                 "{}",
 372                 serde_json::to_string_pretty(&serde_json::json!({
 373                     "success": true,
 374                     "session_id": session_id,
 375                     "name": name,
 376                     "summary": session.codegraph_context.as_ref().map(|ctx| ctx.summary(&stateful.document)),
 377                     "rendered": rendered,
 378                 }))?
 379             );
 380         }
 381         OutputFormat::Text => {
 382             print_success(&format!("Initialized codegraph context session: {}", session_id));
 383         }
 384     }
 385 
 386     write_stateful_document(&stateful, input)?;
 387     Ok(())
 388 }
 389 
 390 fn context_show(
```
### S1 `symbol:crates/ucp-cli/src/commands/agent.rs::back`

- ref: `crates/ucp-cli/src/commands/agent.rs:248-288`

```rust
 246 }
 247 
 248 fn back(input: Option<String>, session: String, steps: usize, format: OutputFormat) -> Result<()> {
 249     let mut stateful = read_stateful_document(input.clone())?;
 250     let document = stateful.document.clone();
 251 
 252     let sess = stateful
 253         .state_mut()
 254         .sessions
 255         .get_mut(&session)
 256         .ok_or_else(|| anyhow!("Session not found: {}", session))?;
 257 
 258     let new_pos = sess.back(steps);
 259     if let Some(context) = sess.codegraph_context.as_mut() {
 260         let focus = new_pos;
 261         context.set_focus(&document, focus);
 262     }
 263 
 264     match format {
 265         OutputFormat::Json => {
 266             #[derive(Serialize)]
 267             struct BackResult {
 268                 success: bool,
 269                 position: Option<String>,
 270             }
 271             let result = BackResult {
 272                 success: new_pos.is_some(),
 273                 position: new_pos.map(|p| p.to_string()),
 274             };
 275             println!("{}", serde_json::to_string_pretty(&result)?);
 276         }
 277         OutputFormat::Text => {
 278             if let Some(pos) = new_pos {
 279                 print_success(&format!("Moved back to {}", pos));
 280             } else {
 281                 println!("No more history");
 282             }
 283         }
 284     }
 285 
 286     write_stateful_document(&stateful, input)?;
 287     Ok(())
 288 }
 289 
 290 fn expand(
```
### S27 `symbol:crates/ucp-cli/src/commands/agent.rs::uuid_short`

- ref: `crates/ucp-cli/src/commands/agent.rs:1135-1142`

```rust
1133 
1134 /// Generate a short UUID-like string
1135 fn uuid_short() -> String {
1136     use std::time::{SystemTime, UNIX_EPOCH};
1137     let now = SystemTime::now()
1138         .duration_since(UNIX_EPOCH)
1139         .unwrap()
1140         .as_nanos();
1141     format!("{:x}", now % 0xFFFFFFFF)
1142 }
```
### S30 `symbol:crates/ucp-cli/src/commands/codegraph.rs::context`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:273-335`

```rust
 271 }
 272 
 273 fn context(cmd: CodegraphContextCommands, format: OutputFormat) -> Result<()> {
 274     match cmd {
 275         CodegraphContextCommands::Init {
 276             input,
 277             name,
 278             max_selected,
 279         } => context_init(input, name, max_selected, format),
 280         CodegraphContextCommands::Show {
 281             input,
 282             session,
 283             max_tokens,
 284         } => context_show(input, session, max_tokens, format),
 285         CodegraphContextCommands::Export {
 286             input,
 287             session,
 288             max_tokens,
 289         } => context_export(input, session, max_tokens, format),
 290         CodegraphContextCommands::Add {
 291             input,
 292             session,
 293             selectors,
 294         } => context_add(input, session, selectors, format),
 295         CodegraphContextCommands::Focus {
 296             input,
 297             session,
 298             target,
 299         } => context_focus(input, session, target, format),
 300         CodegraphContextCommands::Expand {
 301             input,
 302             session,
 303             target,
 304             mode,
 305             relation,
 306         } => context_expand(input, session, target, mode, relation, format),
 307         CodegraphContextCommands::Hydrate {
 308             input,
 309             session,
 310             target,
 311             padding,
 312         } => context_hydrate(input, session, target, padding, format),
 313         CodegraphContextCommands::Collapse {
 314             input,
 315             session,
 316             target,
 317             descendants,
 318         } => context_collapse(input, session, target, descendants, format),
 319         CodegraphContextCommands::Pin {
 320             input,
 321             session,
 322             target,
 323         } => context_pin(input, session, target, true, format),
 324         CodegraphContextCommands::Unpin {
 325             input,
 326             session,
 327             target,
 328         } => context_pin(input, session, target, false, format),
 329         CodegraphContextCommands::Prune {
 330             input,
 331             session,
 332             max_selected,
 333         } => context_prune(input, session, max_selected, format),
 334     }
 335 }
 336 
 337 fn context_init(
```
### S3 `symbol:crates/ucp-cli/src/commands/agent.rs::context_clear`

- ref: `crates/ucp-cli/src/commands/agent.rs:933-950`

```rust
 931 }
 932 
 933 fn context_clear(input: Option<String>, session: String, format: OutputFormat) -> Result<()> {
 934     let mut stateful = read_stateful_document(input.clone())?;
 935     let sess = get_session_mut(&mut stateful, &session)?;
 936     sess.clear_context();
 937     sess.current_block = None;
 938 
 939     match format {
 940         OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&serde_json::json!({
 941             "success": true,
 942             "session": session,
 943             "count": 0
 944         }))?),
 945         OutputFormat::Text => print_success("Context cleared"),
 946     }
 947 
 948     write_stateful_document(&stateful, input)?;
 949     Ok(())
 950 }
 951 
 952 fn context_show(input: Option<String>, session: String, format: OutputFormat) -> Result<()> {
```
### S10 `symbol:crates/ucp-cli/src/commands/agent.rs::context_seed`

- ref: `crates/ucp-cli/src/commands/agent.rs:654-669`

```rust
 652 }
 653 
 654 fn context_seed(input: Option<String>, session: String, format: OutputFormat) -> Result<()> {
 655     let mut stateful = read_stateful_document(input.clone())?;
 656     let document = stateful.document.clone();
 657     if !is_codegraph_document(&document) {
 658         return Err(anyhow!("context seed currently requires a codegraph document"));
 659     }
 660 
 661     let sess = get_session_mut(&mut stateful, &session)?;
 662     let update = sess.ensure_codegraph_context().seed_overview(&document);
 663     sess.sync_context_blocks_from_codegraph();
 664     sess.current_block = update.focus.map(|id| id.to_string());
 665 
 666     print_context_update(format, &session, &update, sess.context_blocks.len(), "Seeded overview")?;
 667     write_stateful_document(&stateful, input)?;
 668     Ok(())
 669 }
 670 
 671 fn context_add(
```
### S80 `symbol:crates/ucp-cli/src/state.rs::TransactionState`

- ref: `crates/ucp-cli/src/state.rs:152-158`

```rust
 150 /// Transaction state
 151 #[derive(Debug, Clone, Serialize, Deserialize)]
 152 pub struct TransactionState {
 153     pub name: Option<String>,
 154     pub started_at: String,
 155     pub savepoints: Vec<SavepointInfo>,
 156     /// Document state at transaction start
 157     pub original_document: String,
 158 }
 159 
 160 impl TransactionState {
```
### S34 `symbol:crates/ucp-cli/src/commands/codegraph.rs::context_export`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:420-448`

```rust
 418 }
 419 
 420 fn context_export(
 421     input: Option<String>,
 422     session: String,
 423     max_tokens: usize,
 424     format: OutputFormat,
 425 ) -> Result<()> {
 426     let stateful = read_stateful_document(input)?;
 427     ensure_codegraph_document(&stateful.document)?;
 428     let sess = get_session(&stateful, &session)?;
 429     let context = sess
 430         .codegraph_context
 431         .as_ref()
 432         .ok_or_else(|| anyhow!("Session has no codegraph context: {}", session))?;
 433     let config = CodeGraphRenderConfig::for_max_tokens(max_tokens);
 434     let export = export_codegraph_context(&stateful.document, context, &config);
 435 
 436     match format {
 437         OutputFormat::Json => {
 438             let mut value = serde_json::to_value(&export)?;
 439             if let Some(object) = value.as_object_mut() {
 440                 object.insert("session".to_string(), serde_json::Value::String(session));
 441             }
 442             println!("{}", serde_json::to_string_pretty(&value)?);
 443         }
 444         OutputFormat::Text => println!("{}", serde_json::to_string_pretty(&export)?),
 445     }
 446 
 447     Ok(())
 448 }
 449 
 450 fn context_add(
```
### S75 `symbol:crates/ucp-cli/src/state.rs::SnapshotInfo`

- ref: `crates/ucp-cli/src/state.rs:120-126`

```rust
 118 /// Serializable snapshot info
 119 #[derive(Debug, Clone, Serialize, Deserialize)]
 120 pub struct SnapshotInfo {
 121     pub name: String,
 122     pub description: Option<String>,
 123     pub created_at: String,
 124     pub block_count: usize,
 125     pub document_json: String,
 126 }
 127 
 128 impl SnapshotInfo {
```
### S9 `symbol:crates/ucp-cli/src/commands/agent.rs::context_remove`

- ref: `crates/ucp-cli/src/commands/agent.rs:724-774`

```rust
 722 }
 723 
 724 fn context_remove(
 725     input: Option<String>,
 726     session: String,
 727     ids: String,
 728     format: OutputFormat,
 729 ) -> Result<()> {
 730     let mut stateful = read_stateful_document(input.clone())?;
 731 
 732     let document = stateful.document.clone();
 733     let codegraph = is_codegraph_document(&document);
 734     let block_ids = resolve_selectors(&document, &ids)?;
 735 
 736     let sess = get_session_mut(&mut stateful, &session)?;
 737 
 738     for id in &block_ids {
 739         sess.remove_from_context(id);
 740         if let Some(context) = sess.codegraph_context.as_mut() {
 741             context.remove_block(*id);
 742         }
 743     }
 744     if codegraph {
 745         sess.sync_context_blocks_from_codegraph();
 746     }
 747 
 748     match format {
 749         OutputFormat::Json => {
 750             #[derive(Serialize)]
 751             struct ContextResult {
 752                 success: bool,
 753                 removed: usize,
 754                 total: usize,
 755             }
 756             let result = ContextResult {
 757                 success: true,
 758                 removed: block_ids.len(),
 759                 total: sess.context_blocks.len(),
 760             };
 761             println!("{}", serde_json::to_string_pretty(&result)?);
 762         }
 763         OutputFormat::Text => {
 764             print_success(&format!(
 765                 "Removed {} blocks from context ({} remaining)",
 766                 block_ids.len(),
 767                 sess.context_blocks.len()
 768             ));
 769         }
 770     }
 771 
 772     write_stateful_document(&stateful, input)?;
 773     Ok(())
 774 }
 775 
 776 fn context_focus(
```
### S70 `symbol:crates/ucp-cli/src/state.rs::AgentSessionState`

- ref: `crates/ucp-cli/src/state.rs:37-47`

```rust
  35 /// Serializable agent session state
  36 #[derive(Debug, Clone, Serialize, Deserialize)]
  37 pub struct AgentSessionState {
  38     pub id: String,
  39     pub name: Option<String>,
  40     pub current_block: Option<String>,
  41     pub history: Vec<String>,
  42     pub context_blocks: Vec<String>,
  43     #[serde(default, skip_serializing_if = "Option::is_none")]
  44     pub codegraph_context: Option<CodeGraphContextSession>,
  45     pub state: String,
  46     pub created_at: String,
  47 }
  48 
  49 impl AgentSessionState {
```
### S82 `symbol:crates/ucp-cli/src/state.rs::read_stateful_document`

- ref: `crates/ucp-cli/src/state.rs:248-275`

```rust
 246 
 247 /// Read a stateful document from file or stdin
 248 pub fn read_stateful_document(input: Option<String>) -> anyhow::Result<StatefulDocument> {
 249     let json = if let Some(path) = input {
 250         std::fs::read_to_string(&path)?
 251     } else {
 252         use std::io::Read;
 253         let mut buffer = String::new();
 254         std::io::stdin().read_to_string(&mut buffer)?;
 255         buffer
 256     };
 257 
 258     // Try to parse as StatefulDocumentJson first
 259     if let Ok(parsed) = serde_json::from_str::<StatefulDocumentJson>(&json) {
 260         let cli_state = parsed.cli_state.unwrap_or_default();
 261         let doc = parsed.document.to_document()?;
 262         return Ok(StatefulDocument {
 263             document: doc,
 264             cli_state,
 265         });
 266     }
 267 
 268     // Try as plain DocumentJson
 269     if let Ok(doc_json) = serde_json::from_str::<DocumentJson>(&json) {
 270         let doc = doc_json.to_document()?;
 271         return Ok(StatefulDocument::from_document(doc));
 272     }
 273 
 274     Err(anyhow::anyhow!("Failed to parse document JSON"))
 275 }
 276 
 277 /// Write a stateful document to file or stdout
```
### S73 `symbol:crates/ucp-cli/src/state.rs::CliState#29`

- ref: `crates/ucp-cli/src/state.rs:29-33`

```rust
  27 }
  28 
  29 impl CliState {
  30     pub fn new() -> Self {
  31         Self::default()
  32     }
  33 }
  34 
  35 /// Serializable agent session state
```
### S4 `symbol:crates/ucp-cli/src/commands/agent.rs::context_collapse`

- ref: `crates/ucp-cli/src/commands/agent.rs:870-894`

```rust
 868 }
 869 
 870 fn context_collapse(
 871     input: Option<String>,
 872     session: String,
 873     target: String,
 874     descendants: bool,
 875     format: OutputFormat,
 876 ) -> Result<()> {
 877     let mut stateful = read_stateful_document(input.clone())?;
 878     let document = stateful.document.clone();
 879     if !is_codegraph_document(&document) {
 880         return Err(anyhow!("context collapse currently requires a codegraph document"));
 881     }
 882     let block_id = resolve_selector(&document, &target)?;
 883 
 884     let sess = get_session_mut(&mut stateful, &session)?;
 885     let update = sess
 886         .ensure_codegraph_context()
 887         .collapse(&document, block_id, descendants);
 888     sess.sync_context_blocks_from_codegraph();
 889     sess.current_block = update.focus.map(|id| id.to_string());
 890 
 891     print_context_update(format, &session, &update, sess.context_blocks.len(), "Collapsed context")?;
 892     write_stateful_document(&stateful, input)?;
 893     Ok(())
 894 }
 895 
 896 fn context_pin(
```
### S50 `symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selector`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:640-645`

```rust
 638 }
 639 
 640 fn resolve_selector(doc: &Document, selector: &str) -> Result<BlockId> {
 641     BlockId::from_str(selector)
 642         .ok()
 643         .or_else(|| resolve_codegraph_selector(doc, selector))
 644         .ok_or_else(|| anyhow!("Could not resolve selector: {}", selector))
 645 }
 646 
 647 fn get_session<'a>(stateful: &'a StatefulDocument, session: &str) -> Result<&'a AgentSessionState> {
```
### S47 `symbol:crates/ucp-cli/src/commands/codegraph.rs::merge_updates`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:666-672`

```rust
 664 }
 665 
 666 fn merge_updates(into: &mut CodeGraphContextUpdate, next: CodeGraphContextUpdate) {
 667     into.added.extend(next.added);
 668     into.removed.extend(next.removed);
 669     into.changed.extend(next.changed);
 670     into.focus = next.focus.or(into.focus);
 671     into.warnings.extend(next.warnings);
 672 }
 673 
 674 fn print_context_update(
```
### S83 `symbol:crates/ucp-cli/src/state.rs::tests`

- ref: `crates/ucp-cli/src/state.rs:309-419`

```rust
 307 
 308 #[cfg(test)]
 309 mod tests {
 310     use super::*;
 311     use ucm_core::Document;
 312 
 313     #[test]
 314     fn test_cli_state_new() {
 315         let state = CliState::new();
 316         assert!(state.sessions.is_empty());
 317         assert!(state.snapshots.is_empty());
 318         assert!(state.transaction.is_none());
 319     }
 320 
 321     #[test]
 322     fn test_agent_session_goto() {
 323         let mut session = AgentSessionState::new("test-session".to_string(), None, None);
 324         let block_id = ucm_core::BlockId::root();
 325 
 326         session.goto(&block_id);
 327 
 328         assert_eq!(session.current_block, Some(block_id.to_string()));
 329         assert_eq!(session.history.len(), 0); // First goto doesn't add to history
 330     }
 331 
 332     #[test]
 333     fn test_agent_session_back() {
 334         let mut session = AgentSessionState::new("test-session".to_string(), None, None);
 335         let block1 = ucm_core::BlockId::root();
 336         let block2 = ucm_core::BlockId::from_hex("aabbccddeeff001122334455").unwrap();
 337 
 338         session.goto(&block1);
 339         session.goto(&block2);
 340 
 341         let result = session.back(1);
 342         assert_eq!(result, Some(block1));
 343         assert_eq!(session.current_block, Some(block1.to_string()));
 344     }
 345 
 346     #[test]
 347     fn test_agent_session_back_empty() {
 348         let mut session = AgentSessionState::new("test-session".to_string(), None, None);
 349         let result = session.back(1);
 350         assert!(result.is_none());
 351     }
 352 
 353     #[test]
 354     fn test_agent_session_context() {
 355         let mut session = AgentSessionState::new("test-session".to_string(), None, None);
 356         let block1 = ucm_core::BlockId::root();
 357         let block2 = ucm_core::BlockId::from_hex("aabbccddeeff001122334455").unwrap();
 358 
 359         session.add_to_context(&block1);
 360         session.add_to_context(&block2);
 361 
 362         assert_eq!(session.context_blocks.len(), 2);
 363 
 364         session.remove_from_context(&block1);
 365         assert_eq!(session.context_blocks.len(), 1);
 366         assert!(!session.context_blocks.contains(&block1.to_string()));
 367         assert!(session.context_blocks.contains(&block2.to_string()));
 368     }
 369 
 370     #[test]
 371     fn test_snapshot_info_create_restore() {
 372         let doc = Document::create();
 373         let snapshot = SnapshotInfo::create(
 374             "test-snapshot".to_string(),
 375             Some("Test description".to_string()),
 376             &doc,
 377         )
 378         .expect("Should create snapshot");
 379 
 380         assert_eq!(snapshot.name, "test-snapshot");
 381         assert_eq!(snapshot.description, Some("Test description".to_string()));
 382         assert_eq!(snapshot.block_count, doc.block_count());
 383 
 384         let restored = snapshot.restore().expect("Should restore");
 385         assert_eq!(restored.block_count(), doc.block_count());
 386     }
 387 
 388     #[test]
 389     fn test_transaction_state_new() {
 390         let doc = Document::create();
 391         let tx = TransactionState::new(Some("test-tx".to_string()), &doc)
 392             .expect("Should create transaction");
 393 
 394         assert_eq!(tx.name, Some("test-tx".to_string()));
 395         assert!(tx.savepoints.is_empty());
 396     }
 397 
 398     #[test]
 399     fn test_transaction_state_savepoint() {
 400         let doc = Document::create();
 401         let mut tx = TransactionState::new(None, &doc).expect("Should create transaction");
 402 
 403         tx.create_savepoint("sp1".to_string(), &doc)
 404             .expect("Should create savepoint");
 405 
 406         assert_eq!(tx.savepoints.len(), 1);
 407         assert_eq!(tx.savepoints[0].name, "sp1");
 408     }
 409 
 410     #[test]
 411     fn test_stateful_document_from_document() {
 412         let doc = Document::create();
 413         let stateful = StatefulDocument::from_document(doc);
 414 
 415         assert!(stateful.cli_state.sessions.is_empty());
 416         assert!(stateful.cli_state.snapshots.is_empty());
 417         assert!(stateful.cli_state.transaction.is_none());
 418     }
 419 }
```
### S79 `symbol:crates/ucp-cli/src/state.rs::StatefulDocumentJson`

- ref: `crates/ucp-cli/src/state.rs:215-220`

```rust
 213 /// JSON representation for stateful document
 214 #[derive(Debug, Clone, Serialize, Deserialize)]
 215 struct StatefulDocumentJson {
 216     #[serde(flatten)]
 217     document: DocumentJson,
 218     #[serde(default, skip_serializing_if = "Option::is_none")]
 219     cli_state: Option<CliState>,
 220 }
 221 
 222 impl StatefulDocument {
```
### S40 `symbol:crates/ucp-cli/src/commands/codegraph.rs::context_show`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:390-418`

```rust
 388 }
 389 
 390 fn context_show(
 391     input: Option<String>,
 392     session: String,
 393     max_tokens: usize,
 394     format: OutputFormat,
 395 ) -> Result<()> {
 396     let stateful = read_stateful_document(input)?;
 397     ensure_codegraph_document(&stateful.document)?;
 398     let sess = get_session(&stateful, &session)?;
 399     let context = sess
 400         .codegraph_context
 401         .as_ref()
 402         .ok_or_else(|| anyhow!("Session has no codegraph context: {}", session))?;
 403     let config = CodeGraphRenderConfig::for_max_tokens(max_tokens);
 404     let export = export_codegraph_context(&stateful.document, context, &config);
 405 
 406     match format {
 407         OutputFormat::Json => {
 408             let mut value = serde_json::to_value(&export)?;
 409             if let Some(object) = value.as_object_mut() {
 410                 object.insert("session".to_string(), serde_json::Value::String(session));
 411             }
 412             println!("{}", serde_json::to_string_pretty(&value)?);
 413         }
 414         OutputFormat::Text => println!("{}", export.rendered),
 415     }
 416 
 417     Ok(())
 418 }
 419 
 420 fn context_export(
```
### S22 `symbol:crates/ucp-cli/src/commands/agent.rs::resolve_selectors`

- ref: `crates/ucp-cli/src/commands/agent.rs:1024-1029`

```rust
1022 }
1023 
1024 fn resolve_selectors(doc: &ucm_core::Document, selectors: &str) -> Result<Vec<BlockId>> {
1025     selectors
1026         .split(',')
1027         .map(|selector| resolve_selector(doc, selector.trim()))
1028         .collect()
1029 }
1030 
1031 fn resolve_selector(doc: &ucm_core::Document, selector: &str) -> Result<BlockId> {
```
### S5 `symbol:crates/ucp-cli/src/commands/agent.rs::context_expand`

- ref: `crates/ucp-cli/src/commands/agent.rs:810-842`

```rust
 808 }
 809 
 810 fn context_expand(
 811     input: Option<String>,
 812     session: String,
 813     target: String,
 814     mode: String,
 815     relation: Option<String>,
 816     format: OutputFormat,
 817 ) -> Result<()> {
 818     let mut stateful = read_stateful_document(input.clone())?;
 819     let document = stateful.document.clone();
 820     if !is_codegraph_document(&document) {
 821         return Err(anyhow!("context expand currently requires a codegraph document"));
 822     }
 823     let block_id = resolve_selector(&document, &target)?;
 824 
 825     let sess = get_session_mut(&mut stateful, &session)?;
 826     let update = match mode.as_str() {
 827         "file" => sess.ensure_codegraph_context().expand_file(&document, block_id),
 828         "dependencies" => sess
 829             .ensure_codegraph_context()
 830             .expand_dependencies(&document, block_id, relation.as_deref()),
 831         "dependents" => sess
 832             .ensure_codegraph_context()
 833             .expand_dependents(&document, block_id, relation.as_deref()),
 834         other => return Err(anyhow!("Unsupported expand mode: {}", other)),
 835     };
 836     sess.sync_context_blocks_from_codegraph();
 837     sess.current_block = update.focus.map(|id| id.to_string());
 838 
 839     print_context_update(format, &session, &update, sess.context_blocks.len(), "Expanded context")?;
 840     write_stateful_document(&stateful, input)?;
 841     Ok(())
 842 }
 843 
 844 fn context_hydrate(
```
### S43 `symbol:crates/ucp-cli/src/commands/codegraph.rs::get_session`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:647-653`

```rust
 645 }
 646 
 647 fn get_session<'a>(stateful: &'a StatefulDocument, session: &str) -> Result<&'a AgentSessionState> {
 648     stateful
 649         .state()
 650         .sessions
 651         .get(session)
 652         .ok_or_else(|| anyhow!("Session not found: {}", session))
 653 }
 654 
 655 fn get_session_mut<'a>(
```
### S2 `symbol:crates/ucp-cli/src/commands/agent.rs::context_add`

- ref: `crates/ucp-cli/src/commands/agent.rs:671-722`

```rust
 669 }
 670 
 671 fn context_add(
 672     input: Option<String>,
 673     session: String,
 674     ids: String,
 675     format: OutputFormat,
 676 ) -> Result<()> {
 677     let mut stateful = read_stateful_document(input.clone())?;
 678 
 679     let document = stateful.document.clone();
 680     let codegraph = is_codegraph_document(&document);
 681     let block_ids = resolve_selectors(&document, &ids)?;
 682 
 683     let sess = get_session_mut(&mut stateful, &session)?;
 684 
 685     for id in &block_ids {
 686         sess.add_to_context(id);
 687         if codegraph {
 688             sess.ensure_codegraph_context()
 689                 .select_block(&document, *id, CodeGraphDetailLevel::SymbolCard);
 690         }
 691     }
 692     if codegraph {
 693         sess.sync_context_blocks_from_codegraph();
 694     }
 695 
 696     match format {
 697         OutputFormat::Json => {
 698             #[derive(Serialize)]
 699             struct ContextResult {
 700                 success: bool,
 701                 added: usize,
 702                 total: usize,
 703             }
 704             let result = ContextResult {
 705                 success: true,
 706                 added: block_ids.len(),
 707                 total: sess.context_blocks.len(),
 708             };
 709             println!("{}", serde_json::to_string_pretty(&result)?);
 710         }
 711         OutputFormat::Text => {
 712             print_success(&format!(
 713                 "Added {} blocks to context ({} total)",
 714                 block_ids.len(),
 715                 sess.context_blocks.len()
 716             ));
 717         }
 718     }
 719 
 720     write_stateful_document(&stateful, input)?;
 721     Ok(())
 722 }
 723 
 724 fn context_remove(
```
### S33 `symbol:crates/ucp-cli/src/commands/codegraph.rs::context_expand`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:504-534`

```rust
 502 }
 503 
 504 fn context_expand(
 505     input: Option<String>,
 506     session: String,
 507     target: String,
 508     mode: String,
 509     relation: Option<String>,
 510     format: OutputFormat,
 511 ) -> Result<()> {
 512     let mut stateful = read_stateful_document(input.clone())?;
 513     ensure_codegraph_document(&stateful.document)?;
 514     let document = stateful.document.clone();
 515     let block_id = resolve_selector(&document, &target)?;
 516 
 517     let sess = get_session_mut(&mut stateful, &session)?;
 518     let update = match mode.as_str() {
 519         "file" => sess.ensure_codegraph_context().expand_file(&document, block_id),
 520         "dependencies" => sess
 521             .ensure_codegraph_context()
 522             .expand_dependencies(&document, block_id, relation.as_deref()),
 523         "dependents" => sess
 524             .ensure_codegraph_context()
 525             .expand_dependents(&document, block_id, relation.as_deref()),
 526         other => return Err(anyhow!("Unsupported expand mode: {}", other)),
 527     };
 528     sess.sync_context_blocks_from_codegraph();
 529     sess.current_block = update.focus.map(|id| id.to_string());
 530 
 531     print_context_update(format, &session, &update, sess)?;
 532     write_stateful_document(&stateful, input)?;
 533     Ok(())
 534 }
 535 
 536 fn context_hydrate(
```
### F78 `file:crates/ucp-codegraph/src/context.rs`

- ref: `crates/ucp-codegraph/src/context.rs:None-None`

```rust
   1 use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
   2 use std::fmt::Write;
   3 use std::path::PathBuf;
```
### S12 `symbol:crates/ucp-cli/src/commands/agent.rs::expand`

- ref: `crates/ucp-cli/src/commands/agent.rs:290-413`

```rust
 288 }
 289 
 290 fn expand(
 291     input: Option<String>,
 292     session: String,
 293     id: Option<String>,
 294     direction: String,
 295     depth: usize,
 296     format: OutputFormat,
 297 ) -> Result<()> {
 298     let stateful = read_stateful_document(input)?;
 299 
 300     let sess = stateful
 301         .state()
 302         .sessions
 303         .get(&session)
 304         .ok_or_else(|| anyhow!("Session not found: {}", session))?;
 305 
 306     let block_id = if let Some(id_str) = id {
 307         BlockId::from_str(&id_str).map_err(|_| anyhow!("Invalid block ID: {}", id_str))?
 308     } else if let Some(curr) = &sess.current_block {
 309         BlockId::from_str(curr).map_err(|_| anyhow!("Invalid current block: {}", curr))?
 310     } else {
 311         stateful.document.root
 312     };
 313 
 314     // Collect blocks based on direction and depth
 315     fn collect_expansion(
 316         doc: &ucm_core::Document,
 317         block_id: &BlockId,
 318         direction: &str,
 319         depth: usize,
 320         current_depth: usize,
 321         results: &mut Vec<(BlockId, usize)>,
 322     ) {
 323         if current_depth > depth {
 324             return;
 325         }
 326 
 327         results.push((*block_id, current_depth));
 328 
 329         if direction == "down" || direction == "both" {
 330             for child_id in doc.children(block_id) {
 331                 collect_expansion(doc, child_id, direction, depth, current_depth + 1, results);
 332             }
 333         }
 334 
 335         if direction == "up" || direction == "both" {
 336             if let Some(parent_id) = doc.parent(block_id) {
 337                 if current_depth < depth {
 338                     collect_expansion(doc, parent_id, "up", depth, current_depth + 1, results);
 339                 }
 340             }
 341         }
 342     }
 343 
 344     let mut results = Vec::new();
 345     collect_expansion(
 346         &stateful.document,
 347         &block_id,
 348         &direction,
 349         depth,
 350         0,
 351         &mut results,
 352     );
 353 
 354     match format {
 355         OutputFormat::Json => {
 356             #[derive(Serialize)]
 357             struct ExpandResult {
 358                 root: String,
 359                 direction: String,
 360                 depth: usize,
 361                 blocks: Vec<ExpandedBlock>,
 362             }
 363             #[derive(Serialize)]
 364             struct ExpandedBlock {
 365                 id: String,
 366                 level: usize,
 367                 content_type: String,
 368                 preview: String,
 369             }
 370             let blocks: Vec<ExpandedBlock> = results
 371                 .iter()
 372                 .filter_map(|(id, level)| {
 373                     stateful.document.get_block(id).map(|b| ExpandedBlock {
 374                         id: id.to_string(),
 375                         level: *level,
 376                         content_type: b.content.type_tag().to_string(),
 377                         preview: content_preview(&b.content, 100),
 378                     })
 379                 })
 380                 .collect();
 381             let result = ExpandResult {
 382                 root: block_id.to_string(),
 383                 direction,
 384                 depth,
 385                 blocks,
 386             };
 387             println!("{}", serde_json::to_string_pretty(&result)?);
 388         }
 389         OutputFormat::Text => {
 390             println!(
 391                 "Expanded {} {} (depth {})",
 392                 block_id,
 393                 direction.cyan(),
 394                 depth
 395             );
 396             for (id, level) in &results {
 397                 if let Some(block) = stateful.document.get_block(id) {
 398                     let indent = "  ".repeat(*level);
 399                     let preview = content_preview(&block.content, 60);
 400                     let preview_line = preview.lines().next().unwrap_or("");
 401                     println!(
 402                         "{}[{}] {}",
 403                         indent,
 404                         id.to_string().yellow(),
 405                         preview_line.dimmed()
 406                     );
 407                 }
 408             }
 409         }
 410     }
 411 
 412     Ok(())
 413 }
 414 
 415 fn follow(
```
### S20 `symbol:crates/ucp-cli/src/commands/agent.rs::print_context_update`

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
### S6 `symbol:crates/ucp-cli/src/commands/agent.rs::context_focus`

- ref: `crates/ucp-cli/src/commands/agent.rs:776-808`

```rust
 774 }
 775 
 776 fn context_focus(
 777     input: Option<String>,
 778     session: String,
 779     target: Option<String>,
 780     format: OutputFormat,
 781 ) -> Result<()> {
 782     let mut stateful = read_stateful_document(input.clone())?;
 783     let document = stateful.document.clone();
 784     let codegraph = is_codegraph_document(&document);
 785     let target_id = target
 786         .as_deref()
 787         .map(|selector| resolve_selector(&document, selector))
 788         .transpose()?;
 789 
 790     let sess = get_session_mut(&mut stateful, &session)?;
 791     sess.current_block = target_id.map(|id| id.to_string());
 792     if codegraph {
 793         let update = sess.ensure_codegraph_context().set_focus(&document, target_id);
 794         sess.sync_context_blocks_from_codegraph();
 795         print_context_update(format, &session, &update, sess.context_blocks.len(), "Updated focus")?;
 796     } else {
 797         match format {
 798             OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&serde_json::json!({
 799                 "success": true,
 800                 "focus": target_id.map(|id| id.to_string())
 801             }))?),
 802             OutputFormat::Text => print_success("Updated focus"),
 803         }
 804     }
 805 
 806     write_stateful_document(&stateful, input)?;
 807     Ok(())
 808 }
 809 
 810 fn context_expand(
```
### F76 `file:crates/ucp-cli/src/state.rs`

- ref: `crates/ucp-cli/src/state.rs:None-None`

```rust
   1 //! State management for CLI sessions
   2 //!
   3 //! This module provides state persistence for agent sessions, transactions,
```
### S39 `symbol:crates/ucp-cli/src/commands/codegraph.rs::context_prune`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:605-623`

```rust
 603 }
 604 
 605 fn context_prune(
 606     input: Option<String>,
 607     session: String,
 608     max_selected: Option<usize>,
 609     format: OutputFormat,
 610 ) -> Result<()> {
 611     let mut stateful = read_stateful_document(input.clone())?;
 612     ensure_codegraph_document(&stateful.document)?;
 613     let document = stateful.document.clone();
 614 
 615     let sess = get_session_mut(&mut stateful, &session)?;
 616     let update = sess.ensure_codegraph_context().prune(&document, max_selected);
 617     sess.sync_context_blocks_from_codegraph();
 618     sess.current_block = update.focus.map(|id| id.to_string()).or_else(|| sess.current_block.clone());
 619 
 620     print_context_update(format, &session, &update, sess)?;
 621     write_stateful_document(&stateful, input)?;
 622     Ok(())
 623 }
 624 
 625 fn ensure_codegraph_document(doc: &Document) -> Result<()> {
```
### S52 `symbol:crates/ucp-cli/src/commands/codegraph.rs::uuid_short`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:713-720`

```rust
 711 }
 712 
 713 fn uuid_short() -> String {
 714     use std::time::{SystemTime, UNIX_EPOCH};
 715     let now = SystemTime::now()
 716         .duration_since(UNIX_EPOCH)
 717         .unwrap()
 718         .as_nanos();
 719     format!("{:x}", now % 0xFFFFFFFF)
 720 }
 721 
 722 fn detect_commit_hash(repo: &PathBuf) -> Result<String> {
```
### S72 `symbol:crates/ucp-cli/src/state.rs::CliState`

- ref: `crates/ucp-cli/src/state.rs:15-27`

```rust
  13 /// Complete CLI state that can be serialized with the document
  14 #[derive(Debug, Clone, Serialize, Deserialize, Default)]
  15 pub struct CliState {
  16     /// Active agent sessions
  17     #[serde(default)]
  18     pub sessions: HashMap<String, AgentSessionState>,
  19 
  20     /// Snapshots
  21     #[serde(default)]
  22     pub snapshots: Vec<SnapshotInfo>,
  23 
  24     /// Transaction state (if in a transaction)
  25     #[serde(default)]
  26     pub transaction: Option<TransactionState>,
  27 }
  28 
  29 impl CliState {
```
### S74 `symbol:crates/ucp-cli/src/state.rs::SavepointInfo`

- ref: `crates/ucp-cli/src/state.rs:200-204`

```rust
 198 /// Savepoint info
 199 #[derive(Debug, Clone, Serialize, Deserialize)]
 200 pub struct SavepointInfo {
 201     pub name: String,
 202     pub created_at: String,
 203     pub document_json: String,
 204 }
 205 
 206 /// Document with CLI state - stored as separate JSON fields
```
### S15 `symbol:crates/ucp-cli/src/commands/agent.rs::get_session_mut`

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
### S48 `symbol:crates/ucp-cli/src/commands/codegraph.rs::print_context_update`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:674-711`

```rust
 672 }
 673 
 674 fn print_context_update(
 675     format: OutputFormat,
 676     session_id: &str,
 677     update: &CodeGraphContextUpdate,
 678     session: &AgentSessionState,
 679 ) -> Result<()> {
 680     match format {
 681         OutputFormat::Json => {
 682             println!(
 683                 "{}",
 684                 serde_json::to_string_pretty(&serde_json::json!({
 685                     "success": true,
 686                     "session": session_id,
 687                     "added": update.added.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
 688                     "removed": update.removed.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
 689                     "changed": update.changed.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
 690                     "focus": update.focus.map(|id| id.to_string()),
 691                     "warnings": update.warnings,
 692                     "total": session.context_blocks.len(),
 693                 }))?
 694             );
 695         }
 696         OutputFormat::Text => {
 697             print_success(&format!(
 698                 "Updated codegraph context {} (added {}, removed {}, changed {}, total {})",
 699                 session_id,
 700                 update.added.len(),
 701                 update.removed.len(),
 702                 update.changed.len(),
 703                 session.context_blocks.len()
 704             ));
 705             for warning in &update.warnings {
 706                 print_warning(warning);
 707             }
 708         }
 709     }
 710     Ok(())
 711 }
 712 
 713 fn uuid_short() -> String {
```
### S16 `symbol:crates/ucp-cli/src/commands/agent.rs::goto`

- ref: `crates/ucp-cli/src/commands/agent.rs:195-246`

```rust
 193 }
 194 
 195 fn goto(
 196     input: Option<String>,
 197     session: String,
 198     target: String,
 199     format: OutputFormat,
 200 ) -> Result<()> {
 201     let mut stateful = read_stateful_document(input.clone())?;
 202     let document = stateful.document.clone();
 203 
 204     let target_id =
 205         BlockId::from_str(&target).map_err(|_| anyhow!("Invalid target block ID: {}", target))?;
 206 
 207     // Verify block exists
 208     if stateful.document.get_block(&target_id).is_none() {
 209         return Err(anyhow!("Block not found: {}", target));
 210     }
 211 
 212     let sess = stateful
 213         .state_mut()
 214         .sessions
 215         .get_mut(&session)
 216         .ok_or_else(|| anyhow!("Session not found: {}", session))?;
 217 
 218     sess.goto(&target_id);
 219     if let Some(context) = sess.codegraph_context.as_mut() {
 220         context.set_focus(&document, Some(target_id));
 221     }
 222 
 223     match format {
 224         OutputFormat::Json => {
 225             #[derive(Serialize)]
 226             struct GotoResult {
 227                 success: bool,
 228                 position: String,
 229             }
 230             let result = GotoResult {
 231                 success: true,
 232                 position: target,
 233             };
 234             println!("{}", serde_json::to_string_pretty(&result)?);
 235         }
 236         OutputFormat::Text => {
 237             print_success(&format!("Moved to {}", target));
 238             if let Some(block) = stateful.document.get_block(&target_id) {
 239                 print_block(block, true);
 240             }
 241         }
 242     }
 243 
 244     write_stateful_document(&stateful, input)?;
 245     Ok(())
 246 }
 247 
 248 fn back(input: Option<String>, session: String, steps: usize, format: OutputFormat) -> Result<()> {
```
### S32 `symbol:crates/ucp-cli/src/commands/codegraph.rs::context_collapse`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:560-582`

```rust
 558 }
 559 
 560 fn context_collapse(
 561     input: Option<String>,
 562     session: String,
 563     target: String,
 564     descendants: bool,
 565     format: OutputFormat,
 566 ) -> Result<()> {
 567     let mut stateful = read_stateful_document(input.clone())?;
 568     ensure_codegraph_document(&stateful.document)?;
 569     let document = stateful.document.clone();
 570     let block_id = resolve_selector(&document, &target)?;
 571 
 572     let sess = get_session_mut(&mut stateful, &session)?;
 573     let update = sess
 574         .ensure_codegraph_context()
 575         .collapse(&document, block_id, descendants);
 576     sess.sync_context_blocks_from_codegraph();
 577     sess.current_block = update.focus.map(|id| id.to_string());
 578 
 579     print_context_update(format, &session, &update, sess)?;
 580     write_stateful_document(&stateful, input)?;
 581     Ok(())
 582 }
 583 
 584 fn context_pin(
```
### S38 `symbol:crates/ucp-cli/src/commands/codegraph.rs::context_pin`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:584-603`

```rust
 582 }
 583 
 584 fn context_pin(
 585     input: Option<String>,
 586     session: String,
 587     target: String,
 588     pinned: bool,
 589     format: OutputFormat,
 590 ) -> Result<()> {
 591     let mut stateful = read_stateful_document(input.clone())?;
 592     ensure_codegraph_document(&stateful.document)?;
 593     let document = stateful.document.clone();
 594     let block_id = resolve_selector(&document, &target)?;
 595 
 596     let sess = get_session_mut(&mut stateful, &session)?;
 597     let update = sess.ensure_codegraph_context().pin(block_id, pinned);
 598     sess.sync_context_blocks_from_codegraph();
 599 
 600     print_context_update(format, &session, &update, sess)?;
 601     write_stateful_document(&stateful, input)?;
 602     Ok(())
 603 }
 604 
 605 fn context_prune(
```
### S19 `symbol:crates/ucp-cli/src/commands/agent.rs::handle_session`

- ref: `crates/ucp-cli/src/commands/agent.rs:63-71`

```rust
  61 }
  62 
  63 fn handle_session(cmd: AgentSessionCommands, format: OutputFormat) -> Result<()> {
  64     match cmd {
  65         AgentSessionCommands::Create { input, name, start } => {
  66             session_create(input, name, start, format)
  67         }
  68         AgentSessionCommands::List { input } => session_list(input, format),
  69         AgentSessionCommands::Close { session } => session_close(session, format),
  70     }
  71 }
  72 
  73 fn session_create(
```
### S18 `symbol:crates/ucp-cli/src/commands/agent.rs::handle_context`

- ref: `crates/ucp-cli/src/commands/agent.rs:602-652`

```rust
 600 }
 601 
 602 fn handle_context(cmd: AgentContextCommands, format: OutputFormat) -> Result<()> {
 603     match cmd {
 604         AgentContextCommands::Seed { input, session } => context_seed(input, session, format),
 605         AgentContextCommands::Add {
 606             input,
 607             session,
 608             ids,
 609         } => context_add(input, session, ids, format),
 610         AgentContextCommands::Remove {
 611             input,
 612             session,
 613             ids,
 614         } => context_remove(input, session, ids, format),
 615         AgentContextCommands::Focus {
 616             input,
 617             session,
 618             target,
 619         } => context_focus(input, session, target, format),
 620         AgentContextCommands::Expand {
 621             input,
 622             session,
 623             target,
 624             mode,
 625             relation,
 626         } => context_expand(input, session, target, mode, relation, format),
 627         AgentContextCommands::Hydrate {
 628             input,
 629             session,
 630             target,
 631             padding,
 632         } => context_hydrate(input, session, target, padding, format),
 633         AgentContextCommands::Collapse {
 634             input,
 635             session,
 636             target,
 637             descendants,
 638         } => context_collapse(input, session, target, descendants, format),
 639         AgentContextCommands::Pin {
 640             input,
 641             session,
 642             target,
 643         } => context_pin(input, session, target, true, format),
 644         AgentContextCommands::Unpin {
 645             input,
 646             session,
 647             target,
 648         } => context_pin(input, session, target, false, format),
 649         AgentContextCommands::Clear { input, session } => context_clear(input, session, format),
 650         AgentContextCommands::Show { input, session } => context_show(input, session, format),
 651     }
 652 }
 653 
 654 fn context_seed(input: Option<String>, session: String, format: OutputFormat) -> Result<()> {
```
### S81 `symbol:crates/ucp-cli/src/state.rs::TransactionState#160`

- ref: `crates/ucp-cli/src/state.rs:160-196`

```rust
 158 }
 159 
 160 impl TransactionState {
 161     pub fn new(name: Option<String>, doc: &Document) -> anyhow::Result<Self> {
 162         let doc_json = DocumentJson::from_document(doc);
 163         Ok(Self {
 164             name,
 165             started_at: chrono::Utc::now().to_rfc3339(),
 166             savepoints: Vec::new(),
 167             original_document: serde_json::to_string(&doc_json)?,
 168         })
 169     }
 170 
 171     pub fn create_savepoint(&mut self, name: String, doc: &Document) -> anyhow::Result<()> {
 172         let doc_json = DocumentJson::from_document(doc);
 173         self.savepoints.push(SavepointInfo {
 174             name,
 175             created_at: chrono::Utc::now().to_rfc3339(),
 176             document_json: serde_json::to_string(&doc_json)?,
 177         });
 178         Ok(())
 179     }
 180 
 181     #[allow(dead_code)]
 182     pub fn rollback_to_savepoint(&self, name: &str) -> anyhow::Result<Option<Document>> {
 183         for savepoint in self.savepoints.iter().rev() {
 184             if savepoint.name == name {
 185                 let doc_json: DocumentJson = serde_json::from_str(&savepoint.document_json)?;
 186                 return Ok(Some(doc_json.to_document()?));
 187             }
 188         }
 189         Ok(None)
 190     }
 191 
 192     pub fn get_original_document(&self) -> anyhow::Result<Document> {
 193         let doc_json: DocumentJson = serde_json::from_str(&self.original_document)?;
 194         doc_json.to_document()
 195     }
 196 }
 197 
 198 /// Savepoint info
```
### S26 `symbol:crates/ucp-cli/src/commands/agent.rs::session_list`

- ref: `crates/ucp-cli/src/commands/agent.rs:121-170`

```rust
 119 }
 120 
 121 fn session_list(input: Option<String>, format: OutputFormat) -> Result<()> {
 122     let stateful = read_stateful_document(input)?;
 123     let sessions = &stateful.state().sessions;
 124 
 125     match format {
 126         OutputFormat::Json => {
 127             #[derive(Serialize)]
 128             struct SessionInfo {
 129                 id: String,
 130                 name: Option<String>,
 131                 current_block: Option<String>,
 132                 context_blocks: usize,
 133                 state: String,
 134             }
 135             let list: Vec<SessionInfo> = sessions
 136                 .values()
 137                 .map(|s| SessionInfo {
 138                     id: s.id.clone(),
 139                     name: s.name.clone(),
 140                     current_block: s.current_block.clone(),
 141                     context_blocks: s.context_blocks.len(),
 142                     state: s.state.clone(),
 143                 })
 144                 .collect();
 145             println!("{}", serde_json::to_string_pretty(&list)?);
 146         }
 147         OutputFormat::Text => {
 148             if sessions.is_empty() {
 149                 println!("No active sessions");
 150             } else {
 151                 println!("{}", "Active Sessions:".cyan().bold());
 152                 for sess in sessions.values() {
 153                     let name_str = sess
 154                         .name
 155                         .as_ref()
 156                         .map(|n| format!(" ({})", n))
 157                         .unwrap_or_default();
 158                     println!(
 159                         "  {} {} - {} blocks in context",
 160                         sess.id.green(),
 161                         name_str,
 162                         sess.context_blocks.len()
 163                     );
 164                 }
 165             }
 166         }
 167     }
 168 
 169     Ok(())
 170 }
 171 
 172 fn session_close(session: String, format: OutputFormat) -> Result<()> {
```
### S51 `symbol:crates/ucp-cli/src/commands/codegraph.rs::resolve_selectors`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:633-638`

```rust
 631 }
 632 
 633 fn resolve_selectors(doc: &Document, selectors: &str) -> Result<Vec<BlockId>> {
 634     selectors
 635         .split(',')
 636         .map(|selector| resolve_selector(doc, selector.trim()))
 637         .collect()
 638 }
 639 
 640 fn resolve_selector(doc: &Document, selector: &str) -> Result<BlockId> {
```
### S77 `symbol:crates/ucp-cli/src/state.rs::StatefulDocument`

- ref: `crates/ucp-cli/src/state.rs:208-211`

```rust
 206 /// Document with CLI state - stored as separate JSON fields
 207 #[derive(Debug, Clone)]
 208 pub struct StatefulDocument {
 209     pub document: Document,
 210     pub cli_state: CliState,
 211 }
 212 
 213 /// JSON representation for stateful document
```
### S76 `symbol:crates/ucp-cli/src/state.rs::SnapshotInfo#128`

- ref: `crates/ucp-cli/src/state.rs:128-148`

```rust
 126 }
 127 
 128 impl SnapshotInfo {
 129     pub fn create(
 130         name: String,
 131         description: Option<String>,
 132         doc: &Document,
 133     ) -> anyhow::Result<Self> {
 134         let doc_json = DocumentJson::from_document(doc);
 135         Ok(Self {
 136             name,
 137             description,
 138             created_at: chrono::Utc::now().to_rfc3339(),
 139             block_count: doc.block_count(),
 140             document_json: serde_json::to_string(&doc_json)?,
 141         })
 142     }
 143 
 144     pub fn restore(&self) -> anyhow::Result<Document> {
 145         let doc_json: DocumentJson = serde_json::from_str(&self.document_json)?;
 146         doc_json.to_document()
 147     }
 148 }
 149 
 150 /// Transaction state
```
### F57 `file:crates/ucp-cli/src/commands/codegraph.rs`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:None-None`

```rust
   1 use anyhow::{anyhow, Context, Result};
   2 use colored::Colorize;
   3 use serde::Serialize;
```
### S49 `symbol:crates/ucp-cli/src/commands/codegraph.rs::prompt`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:245-271`

```rust
 243 }
 244 
 245 fn prompt(input: Option<String>, output: Option<String>, format: OutputFormat) -> Result<()> {
 246     let doc = read_document(input)?;
 247     let projection = codegraph_prompt_projection(&doc);
 248 
 249     match format {
 250         OutputFormat::Json => {
 251             #[derive(Serialize)]
 252             struct PromptResult {
 253                 projection: String,
 254             }
 255             println!(
 256                 "{}",
 257                 serde_json::to_string_pretty(&PromptResult {
 258                     projection: projection.clone(),
 259                 })?
 260             );
 261             if let Some(path) = output {
 262                 write_output(&projection, Some(path))?;
 263             }
 264         }
 265         OutputFormat::Text => {
 266             write_output(&projection, output)?;
 267         }
 268     }
 269 
 270     Ok(())
 271 }
 272 
 273 fn context(cmd: CodegraphContextCommands, format: OutputFormat) -> Result<()> {
```
### S84 `symbol:crates/ucp-cli/src/state.rs::write_stateful_document`

- ref: `crates/ucp-cli/src/state.rs:278-306`

```rust
 276 
 277 /// Write a stateful document to file or stdout
 278 pub fn write_stateful_document(
 279     doc: &StatefulDocument,
 280     output: Option<String>,
 281 ) -> anyhow::Result<()> {
 282     let doc_json = DocumentJson::from_document(&doc.document);
 283 
 284     // Create combined JSON
 285     let stateful_json = StatefulDocumentJson {
 286         document: doc_json,
 287         cli_state: if doc.cli_state.sessions.is_empty()
 288             && doc.cli_state.snapshots.is_empty()
 289             && doc.cli_state.transaction.is_none()
 290         {
 291             None
 292         } else {
 293             Some(doc.cli_state.clone())
 294         },
 295     };
 296 
 297     let json = serde_json::to_string_pretty(&stateful_json)?;
 298 
 299     if let Some(path) = output {
 300         std::fs::write(&path, &json)?;
 301     } else {
 302         println!("{}", json);
 303     }
 304 
 305     Ok(())
 306 }
 307 
 308 #[cfg(test)]
```
### S41 `symbol:crates/ucp-cli/src/commands/codegraph.rs::detect_commit_hash`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:722-740`

```rust
 720 }
 721 
 722 fn detect_commit_hash(repo: &PathBuf) -> Result<String> {
 723     let output = Command::new("git")
 724         .arg("-C")
 725         .arg(repo)
 726         .arg("rev-parse")
 727         .arg("HEAD")
 728         .output()
 729         .context("failed to run git rev-parse HEAD")?;
 730 
 731     if !output.status.success() {
 732         print_warning("could not detect commit hash; using 'unknown'");
 733         return Err(anyhow!(
 734             "git rev-parse failed with status {}",
 735             output.status
 736         ));
 737     }
 738 
 739     Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
 740 }
```
### S11 `symbol:crates/ucp-cli/src/commands/agent.rs::context_show`

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
### S31 `symbol:crates/ucp-cli/src/commands/codegraph.rs::context_add`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:450-478`

```rust
 448 }
 449 
 450 fn context_add(
 451     input: Option<String>,
 452     session: String,
 453     selectors: String,
 454     format: OutputFormat,
 455 ) -> Result<()> {
 456     let mut stateful = read_stateful_document(input.clone())?;
 457     ensure_codegraph_document(&stateful.document)?;
 458     let document = stateful.document.clone();
 459     let block_ids = resolve_selectors(&document, &selectors)?;
 460 
 461     let sess = get_session_mut(&mut stateful, &session)?;
 462     let mut update = CodeGraphContextUpdate::default();
 463     {
 464         let context = sess.ensure_codegraph_context();
 465         for block_id in block_ids {
 466             merge_updates(
 467                 &mut update,
 468                 context.select_block(&document, block_id, CodeGraphDetailLevel::SymbolCard),
 469             );
 470         }
 471     }
 472     sess.sync_context_blocks_from_codegraph();
 473     sess.current_block = update.focus.map(|id| id.to_string()).or_else(|| sess.current_block.clone());
 474 
 475     print_context_update(format, &session, &update, sess)?;
 476     write_stateful_document(&stateful, input)?;
 477     Ok(())
 478 }
 479 
 480 fn context_focus(
```
### S29 `symbol:crates/ucp-cli/src/commands/codegraph.rs::build`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:56-197`

```rust
  54 
  55 #[allow(clippy::too_many_arguments)]
  56 fn build(
  57     repo: String,
  58     commit: Option<String>,
  59     output: Option<String>,
  60     extensions: Option<String>,
  61     include_hidden: bool,
  62     no_export_edges: bool,
  63     fail_on_parse_error: bool,
  64     max_file_bytes: usize,
  65     allow_partial: bool,
  66     format: OutputFormat,
  67 ) -> Result<()> {
  68     let repository_path = PathBuf::from(&repo);
  69     let commit_hash = commit
  70         .or_else(|| detect_commit_hash(&repository_path).ok())
  71         .unwrap_or_else(|| "unknown".to_string());
  72 
  73     let mut config = CodeGraphExtractorConfig::default();
  74     if let Some(exts) = extensions {
  75         config.include_extensions = exts
  76             .split(',')
  77             .map(|e| e.trim().trim_start_matches('.').to_ascii_lowercase())
  78             .filter(|e| !e.is_empty())
  79             .collect();
  80     }
  81     config.include_hidden = include_hidden;
  82     config.emit_export_edges = !no_export_edges;
  83     config.continue_on_parse_error = !fail_on_parse_error;
  84     config.max_file_bytes = max_file_bytes;
  85 
  86     let result = build_code_graph(&CodeGraphBuildInput {
  87         repository_path,
  88         commit_hash,
  89         config,
  90     })
  91     .context("failed to build code graph")?;
  92 
  93     let doc_json = DocumentJson::from_document(&result.document);
  94 
  95     if let Some(path) = &output {
  96         let serialized = serde_json::to_string_pretty(&doc_json)?;
  97         std::fs::write(path, serialized)?;
  98     }
  99 
 100     match format {
 101         OutputFormat::Json => {
 102             #[derive(Serialize)]
 103             struct JsonBuildOutput {
 104                 status: CodeGraphBuildStatus,
 105                 profile_version: String,
 106                 canonical_fingerprint: String,
 107                 stats: ucp_api::CodeGraphStats,
 108                 diagnostics: Vec<ucp_api::CodeGraphDiagnostic>,
 109                 document: DocumentJson,
 110             }
 111 
 112             let payload = JsonBuildOutput {
 113                 status: result.status,
 114                 profile_version: result.profile_version,
 115                 canonical_fingerprint: result.canonical_fingerprint,
 116                 stats: result.stats,
 117                 diagnostics: result.diagnostics.clone(),
 118                 document: doc_json,
 119             };
 120 
 121             println!("{}", serde_json::to_string_pretty(&payload)?);
 122         }
 123         OutputFormat::Text => {
 124             println!("{}", "CodeGraph Build Summary".cyan().bold());
 125             println!("{}", "─".repeat(60));
 126             println!("status: {:?}", result.status);
 127             println!("profile_version: {}", result.profile_version);
 128             println!("canonical_fingerprint: {}", result.canonical_fingerprint);
 129             println!(
 130                 "nodes: total={} repo={} dir={} file={} symbol={}",
 131                 result.stats.total_nodes,
 132                 result.stats.repository_nodes,
 133                 result.stats.directory_nodes,
 134                 result.stats.file_nodes,
 135                 result.stats.symbol_nodes
 136             );
 137             println!(
 138                 "edges: total={} references={} exports={}",
 139                 result.stats.total_edges, result.stats.reference_edges, result.stats.export_edges
 140             );
 141 
 142             if !result.stats.languages.is_empty() {
 143                 let mut langs: Vec<_> = result.stats.languages.iter().collect();
 144                 langs.sort_by(|a, b| a.0.cmp(b.0));
 145                 let joined = langs
 146                     .into_iter()
 147                     .map(|(lang, count)| format!("{}:{}", lang, count))
 148                     .collect::<Vec<_>>()
 149                     .join(", ");
 150                 println!("languages: {joined}");
 151             }
 152 
 153             if !result.diagnostics.is_empty() {
 154                 println!("{}", "diagnostics:".yellow().bold());
 155                 for diag in &result.diagnostics {
 156                     let sev = match diag.severity {
 157                         CodeGraphSeverity::Error => "ERROR".red().bold(),
 158                         CodeGraphSeverity::Warning => "WARN".yellow().bold(),
 159                         CodeGraphSeverity::Info => "INFO".blue().bold(),
 160                     };
 161                     match (&diag.path, &diag.logical_key) {
 162                         (Some(path), _) => println!("  {} {} [{}]", sev, diag.message, path),
 163                         (None, Some(logical_key)) => {
 164                             println!("  {} {} [{}]", sev, diag.message, logical_key)
 165                         }
 166                         _ => println!("  {} {}", sev, diag.message),
 167                     }
 168                 }
 169             }
 170 
 171             if let Some(path) = output {
 172                 print_success(&format!("CodeGraph document written to {}", path));
 173             } else {
 174                 println!("\n{}", serde_json::to_string_pretty(&doc_json)?);
 175             }
 176         }
 177     }
 178 
 179     if !allow_partial {
 180         let has_errors = result
 181             .diagnostics
 182             .iter()
 183             .any(|d| d.severity == CodeGraphSeverity::Error);
 184         if result.status == CodeGraphBuildStatus::FailedValidation {
 185             return Err(anyhow!(
 186                 "code graph build failed profile validation; rerun with --allow-partial to keep output"
 187             ));
 188         }
 189         if has_errors {
 190             return Err(anyhow!(
 191                 "code graph build produced errors; rerun with --allow-partial to keep output"
 192             ));
 193         }
 194     }
 195 
 196     Ok(())
 197 }
 198 
 199 fn inspect(input: Option<String>, format: OutputFormat) -> Result<()> {
```
### S35 `symbol:crates/ucp-cli/src/commands/codegraph.rs::context_focus`

- ref: `crates/ucp-cli/src/commands/codegraph.rs:480-502`

```rust
 478 }
 479 
 480 fn context_focus(
 481     input: Option<String>,
 482     session: String,
 483     target: Option<String>,
 484     format: OutputFormat,
 485 ) -> Result<()> {
 486     let mut stateful = read_stateful_document(input.clone())?;
 487     ensure_codegraph_document(&stateful.document)?;
 488     let document = stateful.document.clone();
 489     let target_id = target
 490         .as_deref()
 491         .map(|selector| resolve_selector(&document, selector))
 492         .transpose()?;
 493 
 494     let sess = get_session_mut(&mut stateful, &session)?;
 495     let update = sess.ensure_codegraph_context().set_focus(&document, target_id);
 496     sess.sync_context_blocks_from_codegraph();
 497     sess.current_block = update.focus.map(|id| id.to_string());
 498 
 499     print_context_update(format, &session, &update, sess)?;
 500     write_stateful_document(&stateful, input)?;
 501     Ok(())
 502 }
 503 
 504 fn context_expand(
```
### S14 `symbol:crates/ucp-cli/src/commands/agent.rs::follow`

- ref: `crates/ucp-cli/src/commands/agent.rs:415-476`

```rust
 413 }
 414 
 415 fn follow(
 416     input: Option<String>,
 417     session: String,
 418     edge_type: String,
 419     format: OutputFormat,
 420 ) -> Result<()> {
 421     let stateful = read_stateful_document(input)?;
 422 
 423     let sess = stateful
 424         .state()
 425         .sessions
 426         .get(&session)
 427         .ok_or_else(|| anyhow!("Session not found: {}", session))?;
 428 
 429     let current = sess
 430         .current_block
 431         .as_ref()
 432         .ok_or_else(|| anyhow!("No current position"))?;
 433     let block_id = BlockId::from_str(current)?;
 434 
 435     let block = stateful
 436         .document
 437         .get_block(&block_id)
 438         .ok_or_else(|| anyhow!("Current block not found"))?;
 439 
 440     // Find edges of the specified type
 441     let matching_edges: Vec<_> = block
 442         .edges
 443         .iter()
 444         .filter(|e| format!("{:?}", e.edge_type).to_lowercase() == edge_type.to_lowercase())
 445         .collect();
 446 
 447     match format {
 448         OutputFormat::Json => {
 449             #[derive(Serialize)]
 450             struct FollowResult {
 451                 edge_type: String,
 452                 targets: Vec<String>,
 453             }
 454             let result = FollowResult {
 455                 edge_type,
 456                 targets: matching_edges
 457                     .iter()
 458                     .map(|e| e.target.to_string())
 459                     .collect(),
 460             };
 461             println!("{}", serde_json::to_string_pretty(&result)?);
 462         }
 463         OutputFormat::Text => {
 464             if matching_edges.is_empty() {
 465                 println!("No edges of type '{}' found", edge_type);
 466             } else {
 467                 println!("Following {} edges:", edge_type.cyan());
 468                 for edge in matching_edges {
 469                     println!("  → {}", edge.target);
 470                 }
 471             }
 472         }
 473     }
 474 
 475     Ok(())
 476 }
 477 
 478 fn search(
```

## Final summary

- selected nodes: 282
- frontier actions remaining: 2
- transcript file: `artifacts/codegraph-context-demo-transcript.md`
