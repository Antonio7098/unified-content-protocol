# Sprint 48 (UCP): CodeGraph Namespace and Module Member Calls

## Goal

Extend conservative call-site resolution to safe namespace/module member-call patterns.

## Scope

- TypeScript namespace-import member calls like `ns.fn()`.
- Python module-alias member calls like `mod.fn()`.
- Preserve prior direct call-site, import/export, wildcard, and relationship semantics.

## Checklist

- [x] Track imported module aliases alongside imported symbol bindings.
- [x] Emit `uses_symbol` edges for TS namespace-import member calls when the target module and member resolve unambiguously.
- [x] Emit `uses_symbol` edges for Python module-alias member calls when the target module and member resolve unambiguously.
- [x] Add focused regression coverage for TS and Python member-call cases.
- [x] Re-run API and CLI codegraph checks.
- [x] Re-validate on the real UCP repo and a member-call probe repo.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Real UCP repo build after this sprint:
  - 130 file nodes
  - 4,457 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Outcome

- The graph now resolves a wider set of member-call usages, not just direct identifier calls.
- This adds a higher-confidence semantic layer for namespace/module mediated calls without attempting full dataflow or alias analysis.

## Status

Completed in this branch.