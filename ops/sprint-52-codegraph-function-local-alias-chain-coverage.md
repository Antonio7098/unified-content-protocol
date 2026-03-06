# Sprint 52 (UCP): CodeGraph Function-Local Alias-Chain Coverage

## Goal

Lock in the new scope-aware alias behavior for function-local alias chains with explicit regression coverage.

## Scope

- TypeScript local alias chains like `const a = util; const b = a; b()` inside a function.
- Python local alias chains like `a = helper; b = a; b()` inside a function.
- No extractor behavior change was required; this sprint captures and validates behavior already enabled by the prior scope-aware alias work.

## Checklist

- [x] Add targeted TS and Python regression tests for function-local alias-chain call resolution.
- [x] Re-run the `ucp-api` codegraph suite.
- [x] Re-check deterministic real-repo codegraph output.
- [x] Re-check a focused local alias-chain probe repo.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- Real UCP repo build after this sprint:
  - 130 file nodes
  - 4,470 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Outcome

- Function-local alias chains are now covered by regression tests, reducing the risk of future semantic regressions in the scope-aware alias resolver.

## Status

Completed in this branch.