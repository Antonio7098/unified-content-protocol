# Sprint 54 (UCP): CodeGraph Top-Level Member-Alias Coverage

## Goal

Lock in conservative resolution for top-level aliases that point at imported module members.

## Scope

- TypeScript top-level member aliases like `const f = ns.util; f()` and `const a = ns.util; const b = a; b()`.
- Python top-level member aliases like `alias = helper_mod.helper; alias()` and `first = helper_mod.helper; second = first; second()`.
- No extractor behavior change was required; this sprint adds regression coverage for behavior already enabled by alias/member resolution.

## Checklist

- [x] Add targeted TS and Python regression tests for top-level member-alias calls.
- [x] Add targeted TS and Python regression tests for top-level member-alias chains.
- [x] Re-run the `ucp-api` codegraph suite.
- [x] Re-check deterministic real-repo codegraph output.
- [x] Re-check a focused top-level member-alias probe repo.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- Real UCP repo build after this sprint:
  - 130 file nodes
  - 4,474 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Outcome

- Top-level member aliases and top-level member-alias chains are now explicitly covered by regression tests, lowering the risk of future regressions in alias/member call resolution.

## Status

Completed in this branch.