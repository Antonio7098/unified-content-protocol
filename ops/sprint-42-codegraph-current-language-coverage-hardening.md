# Sprint 42 (UCP): CodeGraph Current-Language Coverage Hardening

## Goal

Improve graph quality and symbol/export coverage for the already supported languages without adding new language packs.

## Scope

- Rust crate-root symbol import quality.
- TypeScript/JavaScript export alias and default-export coverage.
- TypeScript/JavaScript generator and function-like member coverage.
- Preserve deterministic output on the real UCP repo.

## Checklist

- [x] Resolve single-segment `crate::Symbol` imports to crate entry files when module-file resolution does not apply.
- [x] Capture TS/JS generator function declarations as function symbols.
- [x] Capture JS `field_definition` and TS `public_field_definition` members, classifying function-valued fields as methods.
- [x] Classify function-valued TS/JS variable declarators as function symbols.
- [x] Mark local TS/JS symbols as exported when surfaced via `export { foo }` and `export default Foo` forms.
- [x] Add focused regression tests covering the above patterns.
- [x] Re-run API and CLI codegraph tests.
- [x] Re-validate on the real UCP repo and targeted TS/JS fixtures.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Real UCP repo build after this sprint:
  - 130 file nodes
  - 4,399 symbol nodes
  - 541 reference edges
  - 3 warnings (all intentional edge-case fixture misses)
  - deterministic repeated fingerprint

## Outcome

- Remaining unresolved-import noise on the UCP repo dropped to only the deliberate edge-case fixtures.
- TS/JS graphs now represent more realistic export styles and class-member/function patterns common in production code.

## Status

Completed in this branch.