# Sprint 71 (UCP): CodeGraph TS Re-Export Follow-Up Coverage

## Goal

Lock in the next layer of TS barrel-file behavior after adding re-export-aware call resolution.

## Scope

- Wildcard TS barrel exports like `export * from './util'`.
- Multi-hop barrel chains like `index2 -> index1 -> util`.
- Consumer-side aliasing over multi-hop re-export imports.

## Outcome

- Wildcard TS barrel calls are now explicitly covered by regression tests.
- Multi-hop TS re-export chains are now explicitly covered by regression tests.
- Consumer-side aliasing over re-export imports is now explicitly covered.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- Deterministic real-UCP build remained stable with:
  - 130 file nodes
  - 4,499 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)

## Status

Completed in this branch.
