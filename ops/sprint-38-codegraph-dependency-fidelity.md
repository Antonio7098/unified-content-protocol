# Sprint 38 (UCP): CodeGraph Dependency Fidelity

## Goal

Reduce false-positive import diagnostics and improve real file-to-file dependency capture on real repositories.

## Scope

- Rust workspace-aware crate-root resolution.
- Rust `use`-tree expansion for grouped imports.
- Better external-vs-internal import classification.
- Preserve deterministic fingerprints and CLI behavior.

## Checklist

- [x] Resolve Rust `crate::`, `self::`, `super::`, and bare module imports relative to the nearest crate `src/` root.
- [x] Expand grouped Rust imports like `use crate::{a::b, c::{self, d}}` into resolvable import paths.
- [x] Stop warning on clearly external Rust/TS/Python imports when no local file edge exists.
- [x] Keep unresolved warnings for likely-internal missing imports.
- [x] Add focused regression tests for grouped imports and nested-workspace crate roots.
- [x] Re-run `ucp-api` codegraph tests and `ucp-cli` codegraph workflow tests.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Manual build on the real UCP repo:
  - before: 468 unresolved-import warnings
  - after: 17 unresolved-import warnings
  - deterministic fingerprint preserved across repeated runs

## Outcome

- Real UCP repository reference edges increased materially while warning noise dropped sharply.
- Remaining warnings are now concentrated in actual edge cases, fixture failures, and a smaller set of unresolved re-export-style imports.

## Status

Completed in this branch.