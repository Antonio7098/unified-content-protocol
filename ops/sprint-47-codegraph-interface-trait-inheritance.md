# Sprint 47 (UCP): CodeGraph Interface and Trait Inheritance

## Goal

Extend the explicit type-relationship layer to cover interface and trait inheritance in currently supported languages.

## Scope

- TypeScript interface inheritance.
- Rust trait inheritance.
- Preserve prior class/type relationship, import/export, wildcard, and call-site semantics.

## Checklist

- [x] Emit `extends` edges for TS `interface ... extends ...`.
- [x] Emit `extends` edges for Rust `trait Child: Parent` bounds.
- [x] Reuse the existing conservative symbol target resolution strategy.
- [x] Add focused regression coverage for TS interface and Rust trait inheritance.
- [x] Re-run API and CLI codegraph checks.
- [x] Re-validate on the real UCP repo and an inheritance probe repo.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Real UCP repo build after this sprint:
  - 130 file nodes
  - 4,452 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Outcome

- The graph now models inheritance for TS interfaces and Rust traits, complementing the earlier class/base and impl relationship coverage.
- Relationship resolution remains conservative and deterministic.

## Status

Completed in this branch.