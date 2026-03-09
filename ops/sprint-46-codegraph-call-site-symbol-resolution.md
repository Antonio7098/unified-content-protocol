# Sprint 46 (UCP): CodeGraph Call-Site Symbol Resolution

## Goal

Move the supported-language graph closer to semantic usefulness by linking conservative call sites to concrete symbol targets.

## Scope

- Rust call-site resolution for direct identifiers and clear path calls like `util::greet()`.
- Python call-site resolution for direct identifier calls.
- TS/JS call-site resolution for direct identifier calls and constructors.
- Preserve determinism and prior import/export/type-relationship semantics.

## Checklist

- [x] Extend file analysis to capture extracted symbol usages.
- [x] Attribute call sites to the current enclosing symbol during traversal.
- [x] Resolve usage targets against same-file top-level symbols and explicit imported-name bindings.
- [x] Resolve Rust path calls like `util::greet()` to target files and symbols when unambiguous.
- [x] Emit custom symbol-level `uses_symbol` edges.
- [x] Add focused regression coverage spanning Rust, Python, and TypeScript call sites.
- [x] Re-run API and CLI codegraph checks.
- [x] Re-validate on the real UCP repo and a call-site probe repo.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Real UCP repo build after this sprint:
  - 130 file nodes
  - 4,451 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Outcome

- Symbols can now point directly at other symbols they call in conservative, high-confidence cases.
- The graph now models three semantic layers beyond file dependencies:
  - import/re-export symbol links
  - type relationship edges
  - call-site symbol usage edges

## Status

Completed in this branch.