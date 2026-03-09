# Sprint 56 (UCP): CodeGraph Alias Shadowing and Scope Isolation Coverage

## Goal

Lock in conservative alias precedence and scope isolation semantics.

## Scope

- TypeScript and Python cases where a function-local alias shadows a top-level alias of the same name.
- TypeScript and Python cases where a function-local alias remains isolated to its own enclosing symbol and does not affect sibling functions.
- No extractor behavior change was required; this sprint adds regression coverage for behavior already enabled by scope-aware alias resolution.

## Checklist

- [x] Add targeted TS and Python regression tests for local-over-top-level alias shadowing.
- [x] Add targeted TS and Python regression tests for alias scope isolation across sibling functions.
- [x] Re-run the `ucp-api` codegraph suite.
- [x] Re-check deterministic real-repo codegraph output.
- [x] Re-check focused shadowing and isolation probe repos.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- Real UCP repo build after this sprint:
  - 130 file nodes
  - 4,477 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Outcome

- Alias precedence and scope isolation are now explicitly covered by regression tests, lowering the risk of regressions in local-vs-top-level alias handling.

## Status

Completed in this branch.