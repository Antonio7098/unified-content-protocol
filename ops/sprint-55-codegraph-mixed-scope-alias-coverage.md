# Sprint 55 (UCP): CodeGraph Mixed-Scope Alias Coverage

## Goal

Lock in conservative resolution across alias scopes when a top-level alias is reused through a function-local alias.

## Scope

- TypeScript mixed-scope alias propagation like `const top = ns.util; function run() { const local = top; return local(); }`.
- Python mixed-scope alias propagation like `top = helper_mod.helper; def execute(): local = top; return local()`.
- No extractor behavior change was required; this sprint adds regression coverage for behavior already enabled by scope-aware alias and member resolution.

## Checklist

- [x] Add targeted TS and Python regression tests for mixed-scope alias propagation.
- [x] Re-run the `ucp-api` codegraph suite.
- [x] Re-check deterministic real-repo codegraph output.
- [x] Re-check a focused mixed-scope alias probe repo.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- Real UCP repo build after this sprint:
  - 130 file nodes
  - 4,475 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Outcome

- Mixed-scope alias propagation is now explicitly covered by regression tests, reducing the risk of future regressions where top-level aliases flow through local aliases before a call site.

## Status

Completed in this branch.