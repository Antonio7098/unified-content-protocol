# Sprint 72 (UCP): CodeGraph Python Package Re-Export Resolution

## Goal

Resolve Python calls through package re-export surfaces like `pkg/__init__.py`.

## Scope

- Direct imports from package barrels like `from .pkg import helper; helper()`.
- Aliased imports from package barrels like `from .pkg import helper as alias; alias()`.
- Reuse the exported-symbol target machinery by treating package `__init__.py` imports as exported package bindings.

## Outcome

- Python package-surface imports now resolve calls to underlying symbols.
- Direct and aliased imports from package barrels now participate in `uses_symbol` call resolution.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Real UCP repo after this sprint:
  - 130 file nodes
  - 4,500 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Status

Completed in this branch.
