# Sprint 45 (UCP): CodeGraph Wildcard and Package-Surface Semantics

## Goal

Improve semantic fidelity for wildcard imports and re-exports so package/module surfaces are represented more accurately without sacrificing determinism.

## Scope

- Symbol-level handling for Rust and TS wildcard re-exports.
- Symbol-level handling for Python wildcard imports.
- Python `__all__` filtering for wildcard package re-exports.
- Preserve prior semantic import/export/type-relationship behavior.

## Checklist

- [x] Emit `imports_symbol` edges for wildcard imports when the target module's exported symbols are known.
- [x] Continue emitting `reexports` edges for wildcard re-export forms.
- [x] Filter Python wildcard package re-exports through explicit `__all__` when present.
- [x] Avoid duplicate wildcard edges when explicit export-name filtering already makes the relationship concrete.
- [x] Add focused regression coverage spanning Rust, TS, and Python wildcard behavior.
- [x] Re-run API and CLI codegraph checks.
- [x] Re-validate on the real UCP repo and a wildcard probe repo.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Real UCP repo build after this sprint:
  - 130 file nodes
  - 4,442 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Outcome

- Wildcard imports now contribute symbol-level semantic edges when exported target symbols are available.
- Python package wildcard surfaces now respect explicit `__all__` intent and avoid duplicate semantic edges.
- Rust and TS wildcard re-export semantics remain intact while now participating more consistently in symbol-aware import modeling.

## Status

Completed in this branch.