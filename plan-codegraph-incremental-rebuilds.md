## 1. Overview

Implement an opt-in incremental CodeGraph rebuild path that reuses cached per-file analysis, but still reassembles, validates, sorts, and fingerprints the final `Document` on every run.

Goals and success criteria:
- Keep the existing full-build path (`build_code_graph`) unchanged by default.
- Add an end-to-end incremental mode that is conservative, deterministic, and easy to disable.
- Ensure incremental output matches a clean full rebuild for the same repository state.
- Preserve current profile validation and canonical fingerprint behavior.
- Make the feature testable at unit, API, and CLI levels.

Scope boundaries:
- Included: cached per-file analysis reuse, conservative invalidation, CLI/API plumbing, tests, docs.
- Excluded: daemon/watch mode, background indexing, block-level in-place document patching, AST diffing, parallel scheduler work, speculative dependency inference.

## 2. Prerequisites

- No new external dependencies should be required; existing `serde`, `serde_json`, `sha2`, and temp-file test infrastructure are sufficient.
- Use a sidecar incremental state file, not CodeGraph profile metadata, so canonical output in `crates/ucp-codegraph/src/legacy/canonical.rs` stays stable.
- Reuse existing file discovery in `crates/ucp-codegraph/src/legacy/filesystem.rs`, analysis in `crates/ucp-codegraph/src/legacy/analyze.rs`, assembly in `crates/ucp-codegraph/src/legacy/build.rs`, and validation in `crates/ucp-codegraph/src/legacy/validate.rs`.
- No data migrations are needed; deleting the sidecar file should always force a safe full rebuild.

## 3. Implementation Steps

### Step 1: Refactor the monolithic builder into reusable stages
- Files to modify: `crates/ucp-codegraph/src/legacy/build.rs`, `crates/ucp-codegraph/src/legacy.rs`.
- Extract the current flow into helpers for: repo normalization, file collection, per-file analysis, and final document assembly.
- Keep assembly logic for directories, file/symbol blocks, edges, sorting, validation, and fingerprinting identical to today.
- The key outcome is a reusable "analyzed file snapshot -> final document" path that both full and incremental builds can call.
- Testing: add/adjust `crates/ucp-codegraph/src/legacy/tests.rs` so a normal full build still produces identical stats/fingerprints.

### Step 2: Introduce serializable incremental state and result types
- Files to modify: `crates/ucp-codegraph/src/model.rs`, `crates/ucp-codegraph/src/lib.rs`, `crates/ucp-api/src/lib.rs`.
- Files to create: `crates/ucp-codegraph/src/legacy/incremental.rs`.
- Add state types such as `CodeGraphIncrementalState`, `CodeGraphFileState`, and a dedicated incremental build input/result type.
- Persist, per file: relative path, language, content hash, skip/error status, serialized analysis snapshot, and a conservative list of resolved direct dependencies.
- Persist global invalidation inputs: extractor version, normalized extractor config, and repo identity fields needed to detect incompatible cache reuse.
- Testing: state serde round-trip and backward-safe behavior when no prior state exists.

### Step 3: Implement conservative invalidation and cache reuse
- Files to modify: `crates/ucp-codegraph/src/legacy/incremental.rs`, `crates/ucp-codegraph/src/legacy/build.rs`.
- On each incremental run, recollect files using existing ignore/config rules, hash file contents, and compare against prior state.
- Force full invalidation if extractor version or relevant config changes (`include_extensions`, `exclude_dirs`, `include_hidden`, `max_file_bytes`, `emit_export_edges`, parse-error behavior).
- Rebuild changed/added/deleted files plus the transitive reverse-dependency closure derived from cached direct dependencies.
- If the state file is missing, unreadable, or semantically incompatible, fall back to a full build and emit a warning rather than failing the build.
- Testing: targeted cases for changed file, added file, deleted file, config change, and corrupt state fallback.

### Step 4: Always rebuild the final document from snapshots
- Files to modify: `crates/ucp-codegraph/src/legacy/build.rs`, `crates/ucp-codegraph/src/legacy/incremental.rs`.
- Do not patch the previous `Document` in place.
- Instead, combine reused snapshots and freshly analyzed snapshots, then run the existing block/edge assembly pipeline over the full file set.
- Recompute structure ordering, edge ordering, normalized timestamps, indices, validation diagnostics, stats, and canonical fingerprint exactly as today.
- This keeps correctness high while still avoiding re-parsing/re-analyzing unchanged files.
- Testing: incremental build fingerprint/stat output must match a clean full rebuild for the same repo contents.

### Step 5: Add opt-in CLI and public API support
- Files to modify: `crates/ucp-cli/src/cli.rs`, `crates/ucp-cli/src/commands/codegraph.rs`, `crates/ucp-api/src/lib.rs`, `crates/ucp-codegraph/src/lib.rs`.
- Keep `build_code_graph` untouched for existing callers; add a separate incremental API and re-export it through `ucp-api`.
- Extend `ucp codegraph build` with explicit flags like `--incremental` and `--state-file <path>`.
- If `--incremental` is set and the state file does not exist, do a full build, then write the initial state for the next run.
- Emit small build metrics in CLI JSON/text output, e.g. `rebuilt_files`, `reused_files`, `full_rebuild_reason`.
- Testing: add CLI coverage in `crates/ucp-cli/tests/integration_tests.rs` for first build, second no-op incremental build, and changed-file rebuild.

### Step 6: Document the operating model and limits
- Files to modify: `docs/README.md`, `docs/ucp-cli/README.md`.
- Document that incremental mode is opt-in, sidecar-backed, and conservative.
- Document exactly when the cache is invalidated and that correctness is preserved by full document reassembly.
- Call out current non-goals: no watch mode, no partial graph mutation, no performance promises beyond reduced repeated analysis work.
- Testing: docs-only change; no extra runtime validation beyond the CLI/API tests above.

## 4. File Changes Summary

Created:
- `crates/ucp-codegraph/src/legacy/incremental.rs`
- `plan-codegraph-incremental-rebuilds.md`

Modified:
- `crates/ucp-codegraph/src/model.rs`
- `crates/ucp-codegraph/src/lib.rs`
- `crates/ucp-codegraph/src/legacy.rs`
- `crates/ucp-codegraph/src/legacy/build.rs`
- `crates/ucp-codegraph/src/legacy/tests.rs`
- `crates/ucp-api/src/lib.rs`
- `crates/ucp-api/tests/codegraph_fixture_tests.rs`
- `crates/ucp-cli/src/cli.rs`
- `crates/ucp-cli/src/commands/codegraph.rs`
- `crates/ucp-cli/tests/integration_tests.rs`
- `docs/README.md`
- `docs/ucp-cli/README.md`

Deleted:
- None.

## 5. Testing Strategy

- Unit tests in `crates/ucp-codegraph/src/legacy/tests.rs`:
  - state serde round-trip
  - invalidation on content/config/version change
  - reverse-dependency closure selection
  - corrupt-state fallback to full rebuild
  - incremental vs full-build fingerprint equality
- API tests in `crates/ucp-api/tests/codegraph_fixture_tests.rs`:
  - fixture-based incremental rebuild matches full rebuild
  - add/delete/change scenarios across Rust/Python/TS fixtures
- CLI tests in `crates/ucp-cli/tests/integration_tests.rs`:
  - `codegraph build --incremental --state-file ...` first-run bootstrap
  - second run reuses cache
  - edit one file and confirm only affected files are rebuilt
- Manual smoke tests:
  - build this repo twice with a sidecar state file
  - modify a single source file and compare incremental output fingerprint to a fresh full rebuild

## 6. Rollback Plan

- Remove the new CLI flags and incremental API exports.
- Delete `crates/ucp-codegraph/src/legacy/incremental.rs` and related model types.
- Restore `crates/ucp-codegraph/src/legacy/build.rs` to full-build-only behavior.
- Delete any generated sidecar state files; no repository data migration is involved.

## 7. Estimated Effort

- Rough estimate: 2-4 focused development days.
- Complexity: Medium.
- Primary risk: the refactor needed to separate per-file analysis from final assembly without changing current diagnostics or determinism.
- Why this is achievable now: the repo already has stable logical keys, deterministic canonicalization/fingerprinting, serializable document forms, and strong existing build/CLI tests to anchor the change.
