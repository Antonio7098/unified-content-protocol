# CodeGraph Guide
This guide covers the current CodeGraph surface in `ucp-cli`, `ucp-api`, and `ucp-codegraph`.

## What CodeGraph includes
CodeGraph converts a source repository into a UCP document with repository, directory, file, and symbol nodes plus semantic/reference edges.
Supported languages today:
- Rust
- Python
- TypeScript / TSX
- JavaScript / JSX
Primary workflows:
- build a graph from a repo
- validate and fingerprint it
- render an LLM-friendly projection
- manage a stateful working-set context
- use incremental rebuilds with persisted per-file state
- benchmark full vs incremental behavior

## CLI surface
### Build
```bash
ucp codegraph build /path/to/repo \
  --commit "$(git -C /path/to/repo rev-parse --short HEAD)" \
  --output /tmp/graph.json \
  --allow-partial \
  --format json
```
Important flags:
- `--extensions rs,py,ts,tsx,js,jsx`
- `--include-hidden`
- `--no-export-edges`
- `--fail-on-parse-error`
- `--max-file-bytes <N>`
- `--allow-partial`
- `--incremental`
- `--state-file /tmp/graph.state.json`
Incremental mode:
- persists per-file analysis snapshots
- reuses unchanged files
- rebuilds changed files
- propagates dependent rebuilds only when the changed file's exported API surface changes
- falls back to a full rebuild when state is missing, invalid, config-mismatched, extractor-version-mismatched, or repo-mismatched
Build JSON output includes `status`, `canonical_fingerprint`, `stats`, `diagnostics`, `incremental`, `incremental_state_file`, and `document`.
Current incremental metrics:
- `scanned_files`
- `state_entries`
- `direct_invalidated_files`
- `surface_changed_files`
- `rebuilt_files`
- `reused_files`
- `added_files`
- `changed_files`
- `deleted_files`
- `invalidated_files`
- `full_rebuild_reason`

### Inspect
```bash
ucp codegraph inspect --input /tmp/graph.json --format json
```
Use this to verify CodeGraphProfile conformance and recompute the canonical fingerprint.

### Prompt projection
```bash
ucp codegraph prompt --input /tmp/graph.json --output /tmp/graph-projection.txt
```
This renders a compact structure-plus-symbol summary for downstream LLM workflows.

## Stateful context workflow
The `ucp codegraph context` family manages a focused working set over an existing graph.
Key subcommands:
- `init`, `show`, `export`, `defaults`, `add`, `focus`, `expand`, `expand-recommended`
- `hydrate`, `collapse`, `pin`, `unpin`, `prune`
Useful selectors resolve in this order:
- block ID
- logical key like `symbol:src/lib.rs::add`
- repo path like `src/lib.rs`
- coderef display like `src/lib.rs:2-2`
- unique symbol name when unambiguous
Representative session flow:
```bash
ucp codegraph context init --input /tmp/graph.json --focus src/lib.rs --focus-mode file --format json
ucp codegraph context expand --input /tmp/graph.json --session cgctx_x --target src/lib.rs --mode file --depth 2 --format json
ucp codegraph context show --input /tmp/graph.json --session cgctx_x --max-tokens 4000 --format json
ucp codegraph context hydrate --input /tmp/graph.json --session cgctx_x --target symbol:src/lib.rs::add --padding 2 --format json
ucp codegraph context export --input /tmp/graph.json --session cgctx_x --compact --no-rendered --format json
```
Context-specific knobs worth testing:
- relation presets: `semantic`, `imports`, `reverse-impact`, `references`
- explicit `--relations`
- bounded `--depth`
- `--max-add`
- `--priority-threshold`
- `--levels`, `--only`, `--exclude`
- prune behavior with and without pins
- frontier recommendations from `expand-recommended`

## CLI and traversal capability review
Testers should explicitly judge whether the current CLI and traversal controls are enough for real debugging, navigation, and change-planning workflows.
Review whether these are sufficient and intuitive:
- context lifecycle commands and session persistence
- focus, add, expand, collapse, hydrate, pin, unpin, and prune operations
- relation presets vs explicit relation filters
- depth limits, `--max-add`, and `--priority-threshold`
- export controls like compact mode, rendered text suppression, visible levels, and class filtering
Look for missing capabilities that would help, such as:
- better ambiguity inspection and selector-disambiguation commands
- richer traversal previews before mutating a session
- direct "why is this selected?" or provenance introspection
- bulk operations over many matched nodes
- stronger diff / compare commands for two sessions or two graphs
- more task-oriented shortcuts for "show callers", "show callees", "show imports", and edit-impact exploration

## Rust API surface
Main entry points in `ucp-api`:
- `build_code_graph(...)`
- `build_code_graph_incremental(...)`
- `validate_code_graph_profile(...)`
- `canonical_fingerprint(...)`
- `codegraph_prompt_projection(...)`
- `resolve_codegraph_selector(...)`
- `export_codegraph_context_with_config(...)`
- `render_codegraph_context_prompt(...)`

## Programmatic agent API
CodeGraph also exposes a first-class programmatic surface for agent scripting in Rust and Python.

That surface now sits on top of the generic UCP graph runtime:

- `ucp-graph` handles generic document graph traversal, sessions, JSON persistence, and SQLite persistence
- `ucp-codegraph` handles repository extraction, code selectors, coderef hydration, and code-aware traversal policies

Key Rust types:
- `CodeGraphNavigator`
- `CodeGraphNavigatorSession`
- `CodeGraphFindQuery`
- `CodeGraphExpandMode`

Key Python types:
- `ucp.CodeGraph`
- `ucp.CodeGraphSession`

High-value operations now available without shelling out to repeated CLI calls:
- regex-driven `find_nodes(...)`
- `path_between(...)` for concise dependency chains
- `why_selected(...)` provenance/explainability
- `apply_recommended(...)` / `apply_recommended_actions(...)`
- `fork()` / `diff(...)` for hypothesis branching
- JSON round-tripping with `to_json()`, `from_json(...)`, `save(...)`, and `load(...)`

This is the recommended interface when agents need loops, conditionals, ranking logic, or regex-driven traversal.

If you need graph traversal outside code intelligence, use the generic `GraphNavigator` / `GraphSession` API described in `docs/ucp-api/graph-runtime.md`.

## Benchmark and demo assets
Useful repo assets for testing:
- `crates/ucp-codegraph/examples/incremental_benchmark.rs`
- `scripts/demo_codegraph_context_walk.py`
- `crates/ucp-cli/tests/integration_tests.rs`
- `crates/ucp-codegraph/src/legacy/tests.rs`
- `crates/ucp-codegraph/src/context.rs`
- `docs/ucp-api/codegraph-programmatic.md`
- `docs/ucp-api/graph-runtime.md`
Example benchmark run:
```bash
cargo run -p ucp-codegraph --example incremental_benchmark -- --consumers 200 --format json
```

## Agent optimization
Testers should also role-play as coding agents using CodeGraph to inspect, debug, refactor, and extend a real codebase.
Evaluate which display form works best for agent workflows:
- raw graph document
- prompt projection
- rendered context session
- compact context export JSON
Look for the best agent-facing representation of:
- file and symbol hierarchy
- dependency and dependent paths
- source hydration excerpts
- next-step recommendations and pruning behavior
- ambiguity handling when selectors or matches are not unique
- whether the CLI exposes enough traversal power without forcing low-level graph reasoning

## High-value bug hunts and improvement ideas
Focus testers on:
- fingerprint equivalence between full and incremental rebuilds
- bad incremental-state recovery and fallback reasons
- selector ambiguity and surprising resolution precedence
- context pruning removing high-value nodes too aggressively
- hidden diagnostics when `--allow-partial` is used
- large-file, parse-error, and hidden-file handling
- relation-filter correctness in multi-hop traversal
- output stability across repeated builds and session mutations
- perf cliffs when many dependents are invalidated
- agent-facing display quality for code understanding and edit planning
- missing CLI commands or traversal controls that block efficient investigation
Likely improvement themes:
- clearer selector ambiguity errors
- richer diagnostic grouping in text output
- persisted benchmark baselines
- more visible fallback explanations for incremental rebuilds
- broader language/edge coverage and fixture diversity
- better agent-oriented graph summaries, frontier hints, and compact exports
- more discoverable traversal and session-inspection capabilities

