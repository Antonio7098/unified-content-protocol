# Sprint 67 (UCP): CodeGraph TS Namespace/Module-Member Constructor Coverage

## Goal

Lock in constructor resolution through TS namespace/module-member access and namespace aliases.

## Scope

- Direct namespace member constructors like `new ns.Thing()`.
- Constructors through top-level or function-local aliases to the imported namespace.

## Outcome

- TS namespace-member constructors are now explicitly covered by regression tests.
- Constructor calls through namespace aliases and local namespace-alias chains are now explicitly covered.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- Deterministic real-UCP build remained stable with:
  - 130 file nodes
  - 4,494 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)

## Status

Completed in this branch.
