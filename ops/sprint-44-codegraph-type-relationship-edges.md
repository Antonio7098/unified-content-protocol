# Sprint 44 (UCP): CodeGraph Type Relationship Edges

## Goal

Push the supported-language graph further toward semantic usefulness by linking explicit inheritance and implementation syntax to concrete symbol targets when resolution is safe.

## Scope

- TS/JS class `extends` / `implements` edges.
- Python class base edges.
- Rust `impl Trait for Type` semantic edges.
- Preserve determinism, warning quality, and existing import/export semantics.

## Checklist

- [x] Extend file analysis to capture extracted symbol relationships.
- [x] Resolve relationship targets against same-file symbols and explicit imported-name bindings.
- [x] Emit custom symbol-level semantic edges for `extends`, `implements`, and Rust `for_type`.
- [x] Support TS/JS import alias binding resolution for relationship targets.
- [x] Support Python `from ... import ... as ...` binding resolution for relationship targets.
- [x] Avoid spurious self-edges when Rust impl symbols share the type name they implement for.
- [x] Add focused regression coverage spanning TS, Python, and Rust.
- [x] Re-run API and CLI codegraph checks.
- [x] Re-validate on the real UCP repo and a semantic probe repo.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Real UCP repo build after this sprint:
  - 130 file nodes
  - 4,441 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Outcome

- The graph now carries explicit type-relationship edges across supported languages:
  - TS/JS classes point to resolved base classes and interfaces.
  - Python classes point to resolved base classes.
  - Rust impl symbols point to resolved traits and implemented types.
- Relationship resolution remains conservative: only same-file and explicit imported-name targets are linked.

## Status

Completed in this branch.