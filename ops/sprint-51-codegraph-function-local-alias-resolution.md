# Sprint 51 (UCP): CodeGraph Function-Local Alias Resolution

## Goal

Extend conservative alias-based call resolution into function-local scope.

## Scope

- TypeScript function-local aliases like `const alias = util; alias()`.
- Python function-local aliases like `alias = helper; alias()`.
- Preserve prior top-level alias, alias-chain, member-call, import/export, and relationship semantics.

## Checklist

- [x] Make alias tracking scope-aware by recording the owning enclosing symbol.
- [x] Resolve local aliases with fallback to same-file symbols, imports, top-level aliases, and module-member targets.
- [x] Reuse scope-aware alias maps when resolving `uses_symbol` edges for calls.
- [x] Add focused regression coverage for TS and Python function-local alias calls.
- [x] Re-run API and CLI codegraph checks.
- [x] Re-validate on the real UCP repo and a function-local alias probe repo.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Real UCP repo build after this sprint:
  - 130 file nodes
  - 4,469 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Outcome

- Calls through simple function-local aliases now resolve to underlying symbols rather than stopping at the local alias name.
- Alias handling now respects enclosing-symbol scope instead of treating all aliases as file-global.

## Status

Completed in this branch.