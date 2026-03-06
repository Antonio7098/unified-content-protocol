# Sprint 65 (UCP): CodeGraph TS/Python Module-Alias Alias Resolution

## Goal

Resolve TS/Python member calls through aliases that point at imported modules or namespaces.

## Scope

- Top-level aliases like `const alias = ns; alias.util()` and `alias = helper_mod; alias.helper()`.
- Function-local alias chains like `const first = ns; const second = first; second.util()` and the Python equivalent.
- Conservative scope-aware behavior only; unresolved or unsupported aliases remain unresolved.

## Outcome

- TS/Python aliases that point to imported module or namespace aliases now resolve member call sites to underlying symbols.
- Local and top-level module-alias chains now participate in the same scope/shadowing rules as symbol aliases.
- Regression coverage now protects both top-level and function-local module-alias chain behavior.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Real UCP repo after this sprint:
  - 130 file nodes
  - 4,491 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Status

Completed in this branch.