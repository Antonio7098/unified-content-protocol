# Sprint 53 (UCP): CodeGraph Function-Local Member-Alias Coverage

## Goal

Lock in conservative resolution for function-local aliases that point at imported module members.

## Scope

- TypeScript local member aliases like `const f = ns.util; f()` and `const a = ns.util; const b = a; b()`.
- Python local member aliases like `alias = helper_mod.helper; alias()` and `first = helper_mod.helper; second = first; second()`.
- No extractor behavior change was required; this sprint adds regression coverage for behavior already enabled by scope-aware alias and member resolution.

## Checklist

- [x] Add targeted TS and Python regression tests for function-local member-alias calls.
- [x] Add targeted TS and Python regression tests for function-local member-alias chains.
- [x] Re-run the `ucp-api` codegraph suite.
- [x] Re-check deterministic real-repo codegraph output.
- [x] Re-check a focused local member-alias probe repo.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- Real UCP repo build after this sprint:
  - 130 file nodes
  - 4,472 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Outcome

- Function-local member aliases and local member-alias chains are now explicitly covered by regression tests, lowering the risk of future regressions in scope-aware alias/member resolution.

## Status

Completed in this branch.