# Post-Sprint 37 (UCP): CodeGraph Follow-On Sprint Sequence

## 1. Overview

- This plan proposes the next practical sprint documents after `ops/sprint-37-codebase-ucm-graph.md`.
- Goal: improve current CodeGraph usefulness before broadening scope too aggressively.
- Success means the next sprint sequence reduces noisy diagnostics, hardens traversal/perf, expands symbol depth safely, adds more language coverage on a better architecture, and closes the bindings compatibility gap.
- Included: sprint names, execution order, goals, scope, exit criteria, likely file targets.
- Excluded: Hivemind runtime/eventing work already called out as outside UCP ownership in Sprint 37.

## 2. Prerequisites

- `ops/sprint-37-codebase-ucm-graph.md` is the baseline and should be treated as complete-but-follow-up-needed.
- Main implementation seam remains `crates/ucp-api/src/codegraph.rs`.
- Likely dependency additions in follow-on execution:
  - `ignore` in `crates/ucp-api/Cargo.toml`
  - `rayon` in `crates/ucp-api/Cargo.toml`
  - additional `tree-sitter-*` grammars in `crates/ucp-api/Cargo.toml` for new languages
- No data migrations are required.
- Binding validation work will need Python/WASM test runs, but should not block the first extractor-hardening sprint.

## 3. Implementation Steps

### Step 1: Execute first — Sprint 38 (UCP): CodeGraph Signal Quality and Traversal Hardening
- **Create:** `ops/sprint-38-codegraph-signal-quality-and-traversal-hardening.md`
- **Goal:** reduce false unresolved-import noise and replace brittle traversal/ignore behavior with production-grade repository walking.
- **Why first / ROI:** current value is most limited by noisy Rust import diagnostics and custom ignore handling. Fixing this improves every supported repo immediately.
- **Scope:**
  - replace the custom `GitignoreMatcher` in `crates/ucp-api/src/codegraph.rs` with `ignore`
  - normalize Rust `use` trees into resolvable internal paths instead of warning on every non-local form
  - classify Rust imports into `stdlib`, `external crate`, `workspace-internal`, `relative internal`, `unresolved internal`
  - only emit `CG2006`-style warnings for likely internal unresolved imports
  - move import-extraction logic toward per-language query/assets, closing Sprint 37 item `37.5 / Per-language Tree-sitter queries stored under extractor assets`
- **Key files to modify:**
  - `crates/ucp-api/src/codegraph.rs`
  - `crates/ucp-api/Cargo.toml`
  - `crates/ucp-api/tests/codegraph_fixture_tests.rs`
  - `crates/ucp-api/tests/fixtures/edge-cases/**`
  - `docs/ucp-api/README.md`
  - `docs/ucp-cli/README.md`
- **Exit criteria:**
  - Rust-heavy fixtures show materially fewer unresolved warnings without reducing real internal-edge coverage
  - `.gitignore` semantics come from `ignore`, not custom regex logic
  - import diagnostics distinguish external dependencies from true unresolved internal references
  - determinism/fingerprint tests still pass unchanged
- **Testing considerations:**
  - add fixtures for nested `use {a::{b,c}, d}` patterns and external crate imports
  - compare diagnostic counts before/after on `edge-cases` and at least one larger manual-test repo

### Step 2: Sprint 39 (UCP): CodeGraph Deterministic Scale and Parallel Extraction
- **Create:** `ops/sprint-39-codegraph-deterministic-scale.md`
- **Goal:** meet the unfinished Sprint 37 scale target without regressing determinism.
- **Scope:**
  - add `rayon`-backed parse/extract parallelism with deterministic merge ordering
  - separate traversal order from processing order so fingerprints stay stable
  - add explicit perf counters for file count, parse time, edge resolution time, and memory-sensitive smoke coverage
  - lock down the `~2k files under 3 seconds` target from Sprint 37.8 with repeatable benchmark fixtures
- **Key files to modify:**
  - `crates/ucp-api/src/codegraph.rs`
  - `crates/ucp-api/Cargo.toml`
  - `crates/ucp-api/tests/codegraph_fixture_tests.rs`
  - new or expanded fixtures under `crates/ucp-api/tests/fixtures/`
- **Exit criteria:**
  - a medium polyglot fixture consistently stays under the defined wall-clock target on a dev machine baseline
  - canonical fingerprints remain stable across repeated parallel runs
  - no ordering flake is introduced in unit/integration tests
- **Testing considerations:**
  - add a repeated-run determinism test under parallel mode
  - preserve a serial fallback for debugging and bisecting

### Step 3: Sprint 40 (UCP): CodeGraph Profile v1.1 — Nested Symbol Capture
- **Create:** `ops/sprint-40-codegraph-profile-v1_1-nested-symbol-capture.md`
- **Goal:** move beyond top-level-only capture without breaking the current orientation-first profile by accident.
- **Why after Sprint 39:** nested capture increases node counts and logical-key complexity, so it should land after scale and determinism are hardened.
- **Scope:**
  - define whether nested symbol capture is a new profile version (`codegraph.v1.1`) or an opt-in extractor mode
  - add bounded nested extraction for the highest-value forms first:
    - Rust `impl` methods / associated items
    - Python class methods
    - TS/JS class methods and exported nested declarations
  - extend logical-key rules to include parent symbol ancestry
  - keep deeper body-level local variables and call graphs out of scope
- **Key files to modify:**
  - `crates/ucp-api/src/codegraph.rs`
  - `crates/ucp-api/src/lib.rs`
  - `crates/ucp-cli/src/commands/codegraph.rs`
  - `crates/ucp-cli/src/cli.rs`
  - `crates/ucp-api/tests/codegraph_fixture_tests.rs`
  - `docs/ucp-api/README.md`
  - `docs/ucp-cli/README.md`
- **Exit criteria:**
  - nested symbols are emitted with stable parent-aware logical keys
  - existing `codegraph.v1` consumers do not silently change behavior unless they opt in or upgrade profile version
  - symbol density improves on representative fixtures without exploding graph size
- **Testing considerations:**
  - add fixtures with classes/impls/methods in all currently supported languages
  - add regression tests for logical-key stability across reordering of methods

### Step 4: Sprint 41 (UCP): CodeGraph Language Pack 1 (Go + Java)
- **Create:** `ops/sprint-41-codegraph-language-pack-1-go-java.md`
- **Goal:** expand supported-language coverage only after import/traversal and symbol semantics are on a firmer base.
- **Scope:**
  - add `go` and `java` extension/language mapping to the extractor
  - add tree-sitter grammars and top-level symbol/import extraction for those languages
  - wire import resolution rules that are explicitly best-effort and internal-only for warning generation
  - keep language support shallow and structural; no type resolution or build-tool integration
- **Key files to modify:**
  - `crates/ucp-api/src/codegraph.rs`
  - `crates/ucp-api/Cargo.toml`
  - `crates/ucp-api/tests/codegraph_fixture_tests.rs`
  - new fixtures under `crates/ucp-api/tests/fixtures/`
  - `docs/ucp-api/README.md`
  - `docs/ucp-cli/README.md`
- **Exit criteria:**
  - `extension_language()` and default extension config cover the new languages
  - each new language has fixture-backed symbol/import extraction and deterministic fingerprints
  - unsupported-language diagnostics become the exception rather than the default on mixed-language repos
- **Testing considerations:**
  - add one clean and one edge-case fixture per new language
  - verify mixed-language repos still produce stable counts and diagnostics

### Step 5: Sprint 42 (UCP): CodeGraph Bindings Compatibility and Cross-SDK Inspection
- **Create:** `ops/sprint-42-codegraph-bindings-compatibility.md`
- **Goal:** close the unchecked Sprint 37.12 item for Rust/Python/WASM inspection of generated codegraph documents.
- **Why last:** bindings should validate the stabilized codegraph surface, not a moving target.
- **Scope:**
  - add or expose `from_json`/load helpers for full document round-trip where missing
  - ensure codegraph metadata, structure, and reference edges are inspectable from Python and WASM
  - add codegraph-specific tests in binding suites, not just generic document serialization tests
  - update binding docs with a codegraph inspection example
- **Key files to modify:**
  - `crates/ucp-python/src/document.rs`
  - `crates/ucp-python/tests/test_document.py`
  - `crates/ucp-python/tests/test_integration.py` or new codegraph-specific tests
  - `crates/ucp-wasm/src/document.rs`
  - `crates/ucp-wasm/tests/ucp.test.js`
  - `docs/ucm-engine/validation.md`
  - `docs/ucp-api/README.md`
- **Exit criteria:**
  - Python and WASM can load a generated codegraph document and inspect metadata/blocks/edges without compatibility gaps
  - codegraph examples are covered by automated tests in both bindings
  - Sprint 37.12 compatibility item can be checked off with evidence
- **Testing considerations:**
  - use a real codegraph fixture JSON emitted by `ucp-api` rather than hand-built generic documents

## 4. File Changes Summary

### Created
- `ops/post-sprint-37-codegraph-follow-on-sequence.md`
- Proposed sprint docs:
  - `ops/sprint-38-codegraph-signal-quality-and-traversal-hardening.md`
  - `ops/sprint-39-codegraph-deterministic-scale.md`
  - `ops/sprint-40-codegraph-profile-v1_1-nested-symbol-capture.md`
  - `ops/sprint-41-codegraph-language-pack-1-go-java.md`
  - `ops/sprint-42-codegraph-bindings-compatibility.md`

### Expected Modified During Execution
- `crates/ucp-api/src/codegraph.rs`
- `crates/ucp-api/src/lib.rs`
- `crates/ucp-api/Cargo.toml`
- `crates/ucp-api/tests/codegraph_fixture_tests.rs`
- `crates/ucp-cli/src/commands/codegraph.rs`
- `crates/ucp-cli/src/cli.rs`
- `crates/ucp-python/src/document.rs`
- `crates/ucp-python/tests/*.py`
- `crates/ucp-wasm/src/document.rs`
- `crates/ucp-wasm/tests/ucp.test.js`
- `docs/ucp-api/README.md`
- `docs/ucp-cli/README.md`

## 5. Testing Strategy

- Unit tests: import classification, language detection, nested symbol logical keys, binding JSON round-trip.
- Integration tests: CLI `codegraph build/inspect/prompt`, mixed-language fixture builds, Python/WASM codegraph inspection.
- Performance tests: repeated-run fingerprint stability and medium-repo timing under parallel mode.
- Manual tests: rerun the Sprint 37 manual matrix plus at least one Rust-heavy repo with external dependencies and one mixed-language repo with new languages.

## 6. Rollback Plan

- Land each sprint independently so it can be reverted without unwinding later work.
- Keep nested symbol capture behind an explicit profile/version or config switch.
- Preserve serial extraction path while introducing `rayon`.
- If new language packs are noisy, remove them from default extension lists until analyzers stabilize.

## 7. Estimated Effort

- Sprint 38: medium, highest ROI, ~1 sprint.
- Sprint 39: medium/high due to determinism + perf validation, ~1 sprint.
- Sprint 40: high because it touches profile semantics and logical keys, ~1 sprint.
- Sprint 41: medium/high depending on grammar quirks, ~1 sprint.
- Sprint 42: medium, mostly SDK/test/documentation hardening, ~0.5-1 sprint.
- Overall complexity: medium/high, but decomposed cleanly enough to execute in order without blocking the entire roadmap.