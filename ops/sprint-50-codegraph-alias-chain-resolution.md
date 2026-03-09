# Sprint 50 (UCP): CodeGraph Alias-Chain Resolution

## Goal

Extend conservative alias-based call resolution to simple top-level alias chains.

## Scope

- TypeScript alias chains like `const a = util; const b = a; b()`.
- Python alias chains like `a = helper; b = a; b()`.
- Preserve prior direct-call, member-call, and top-level alias semantics without adding noisy intermediate edges.

## Checklist

- [x] Resolve top-level aliases iteratively so aliases can depend on previously resolved aliases.
- [x] Prefer underlying resolved alias targets over intermediate alias symbols.
- [x] Add focused regression coverage for TS and Python alias-chain call cases.
- [x] Re-run API and CLI codegraph checks.
- [x] Re-validate on the real UCP repo and an alias-chain probe repo.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Real UCP repo build after this sprint:
  - 130 file nodes
  - 4,467 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Outcome

- Direct calls through simple alias chains now resolve to their underlying target symbols.
- Intermediate alias symbols are not kept as extra call targets when a deeper resolved target is available.

## Status

Completed in this branch.