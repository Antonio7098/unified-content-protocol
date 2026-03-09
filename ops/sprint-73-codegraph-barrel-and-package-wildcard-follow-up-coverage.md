# Sprint 73 (UCP): CodeGraph Barrel and Package Wildcard Follow-Up Coverage

## Goal

Lock in the final high-signal wildcard follow-ups after the TS barrel and Python package re-export work.

## Scope

- Negative TS behavior: `export *` must not re-export default.
- Positive Python behavior: package wildcard re-exports like `from .helper import *` should surface callable names through the package.

## Outcome

- TS wildcard-barrel default exclusion now has explicit negative regression coverage.
- Python package wildcard re-exports now have explicit positive regression coverage.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- Deterministic real-UCP build remained stable with:
  - 130 file nodes
  - 4,501 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)

## Status

Completed in this branch.