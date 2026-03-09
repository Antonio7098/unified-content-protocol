# Sprint 43 (UCP): CodeGraph Semantic Import and Re-export Links

## Goal

Push the supported-language graph closer to semantic usefulness by linking imports and re-exports to concrete symbol targets when resolution is safe.

## Scope

- Symbol-aware import edges for Rust, Python, TypeScript, and JavaScript.
- Re-export edges for explicit Rust `pub use ...` and TS/JS `export { ... } from ...` forms.
- Better Python relative-import normalization for `from . import module` forms.
- Python package export semantics for explicit `__all__` lists.
- Preserve deterministic output and existing file-level dependency edges.

## Checklist

- [x] Extend extracted import records to carry imported symbol names and re-export intent.
- [x] Normalize Python `from . import name` into resolvable sibling-module imports.
- [x] Emit custom `imports_symbol` edges from files to resolved target symbols.
- [x] Emit `exports` edges with `relation=reexports` for explicit Rust/TS re-exports.
- [x] Honor Python `__all__` to re-export imported package symbols and refine explicit package exports.
- [x] Preserve existing file-level `references` edges alongside new symbol-aware edges.
- [x] Add focused regression tests for Python relative imports and Rust/TS re-export symbol links.
- [x] Re-run API and CLI codegraph tests.
- [x] Re-validate on the real UCP repo and a semantic probe repository.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Real UCP repo build after this sprint:
  - 130 file nodes
  - 4,423 symbol nodes
  - 541 reference edges
  - 1,194 export edges
  - 3 warnings (only deliberate edge-case fixture misses)
  - deterministic repeated fingerprint

## Outcome

- The graph now carries a second semantic layer beyond file dependencies: files can point directly at imported symbols they consume, and explicit re-exporting files can point at the symbols they re-export.
- Python relative module imports like `from . import helper` no longer degrade into unresolved `.` imports.
- Python package modules with explicit `__all__` now surface re-export edges to the symbols they intentionally expose.

## Status

Completed in this branch.