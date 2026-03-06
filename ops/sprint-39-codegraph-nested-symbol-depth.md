# Sprint 39 (UCP): CodeGraph Nested Symbol Depth

## Goal

Move beyond top-level-only extraction so the graph captures higher-value nested symbols across the currently supported languages.

## Scope

- Nested symbol traversal for Rust, Python, TypeScript, and JavaScript.
- Parent-aware symbol containment in the document structure.
- Stable logical keys for nested declarations.
- Keep export edges limited to top-level exports for profile stability.

## Checklist

- [x] Recursively traverse supported-language syntax trees for nested declarations.
- [x] Capture Rust impl methods and nested functions.
- [x] Capture Python class methods and nested functions.
- [x] Capture TS/JS class methods and nested function declarations.
- [x] Nest symbol blocks under parent symbols in the document structure.
- [x] Add focused regression tests covering mixed-language nested symbol capture.
- [x] Re-run API and CLI codegraph tests.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Manual nested-symbol sample:
  - before: only top-level symbols plus Rust `impl` blocks
  - after: 13 symbol nodes including methods and nested inner functions across Rust/Python/TS

## Outcome

- The real UCP repository now emits 4,331 symbol nodes with nested test functions and similar inner declarations represented.
- Prompt projections are significantly richer because symbols are no longer flattened to top-level-only declarations.

## Status

Completed in this branch.