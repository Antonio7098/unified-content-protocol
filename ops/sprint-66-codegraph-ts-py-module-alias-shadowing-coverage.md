# Sprint 66 (UCP): CodeGraph TS/Python Module-Alias Shadowing Coverage

## Goal

Lock in shadowing and scope-isolation semantics for TS/Python aliases that point at imported modules or namespaces.

## Scope

- Unresolved or unsupported shadowing aliases must not fall back to outer module aliases.
- Function-local module aliases must remain isolated to their own enclosing symbol.

## Outcome

- TS/Python module-alias shadowing now has explicit no-fallback regression coverage.
- TS/Python module-alias scope isolation now has explicit regression coverage.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- Deterministic real-UCP build remained stable with:
  - 130 file nodes
  - 4,493 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)

## Status

Completed in this branch.
