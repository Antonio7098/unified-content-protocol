# Sprint 60 (UCP): CodeGraph Rust Function-Local Alias Resolution

## Goal

Extend scope-aware alias resolution to simple Rust `let` bindings inside functions.

## Scope

- Support `let hello = greet; hello()`.
- Support `let wave_alias = nested::util::wave; wave_alias()`.
- Reuse existing alias-resolution machinery conservatively and only resolve unambiguous targets.

## Outcome

- Rust function-local `let` aliases now participate in `uses_symbol` call resolution.
- Rust path aliases inside functions resolve through the same import/path machinery as direct call sites.
- Regression coverage now protects imported-symbol and path-based local Rust aliases.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Real UCP repo after this sprint set:
  - 130 file nodes
  - 4,484 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Status

Completed in this branch.