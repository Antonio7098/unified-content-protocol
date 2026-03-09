# Sprint 57 (UCP): CodeGraph Rust Import Aliases and Constructor/Path Coverage

## Goal

Close the Rust imported-call alias gap and lock in nearby constructor/path call behaviors that were already resolving correctly.

## Scope

- Rust `use ... as ...` imported-call aliases like `use util::greet as hello; hello()`.
- Rust nested path calls like `nested::util::wave()`.
- TypeScript constructor aliases like `const Ctor = Thing; new Ctor()` and namespace/member constructor alias chains.

## Checklist

- [x] Preserve local `as` bindings when expanding Rust `use` trees.
- [x] Emit imported symbol bindings for Rust symbol aliases so aliased calls resolve to the target symbol.
- [x] Add regression coverage for Rust import aliases and nested path calls.
- [x] Add regression coverage for TS constructor aliases and member-constructor alias chains.
- [x] Re-run API and CLI codegraph checks.
- [x] Re-check deterministic real-repo output and focused probe repos.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Real UCP repo build after this sprint:
  - 130 file nodes
  - 4,480 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Outcome

- Rust aliased symbol imports now participate in `uses_symbol` call resolution.
- Rust nested path calls and TS constructor-alias patterns are now explicitly protected by regression tests.

## Status

Completed in this branch.