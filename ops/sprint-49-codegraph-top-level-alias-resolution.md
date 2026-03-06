# Sprint 49 (UCP): CodeGraph Top-Level Alias Resolution

## Goal

Improve semantic call resolution by following conservative top-level alias bindings before emitting `uses_symbol` edges.

## Scope

- TypeScript top-level aliases like `const alias = util`.
- Python top-level aliases like `alias = helper`.
- Preserve prior direct call, namespace/module member-call, import/export, wildcard, and relationship semantics.

## Checklist

- [x] Extend file analysis to capture extracted top-level aliases.
- [x] Resolve alias targets against same-file symbols, imported symbol bindings, and imported module-member targets.
- [x] Prefer resolved alias targets for direct call-site lookups like `alias()`.
- [x] Add focused regression coverage for TS and Python alias-call cases.
- [x] Re-run API and CLI codegraph checks.
- [x] Re-validate on the real UCP repo and an alias probe repo.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Real UCP repo build after this sprint:
  - 130 file nodes
  - 4,466 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Outcome

- Direct calls through simple top-level aliases now resolve to the underlying symbols instead of stopping at the alias name.
- The graph stays conservative by only following simple, unambiguous top-level alias patterns.

## Status

Completed in this branch.