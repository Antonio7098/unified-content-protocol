# Sprint 70 (UCP): CodeGraph TS Re-Export-Aware Call Resolution

## Goal

Resolve TS calls and constructors through barrel files and re-export surfaces.

## Scope

- Named imports sourced from files that re-export default symbols under named exports.
- Conservative exported-symbol target tracking per file, including iterative re-export propagation.
- Focused call and constructor resolution through barrel files.

## Outcome

- TS imports from re-export surfaces now resolve to underlying symbols for calls and constructors.
- Re-export-aware exported-symbol tracking supports direct, wildcard, and multi-hop barrel propagation.
- Regression coverage now protects named re-exported calls and constructors.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Real UCP repo after this sprint set:
  - 130 file nodes
  - 4,499 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Status

Completed in this branch.