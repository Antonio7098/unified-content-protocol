# Sprint 59 (UCP): CodeGraph Shadowing Without Invalid Fallback

## Goal

Prevent incorrect fallback when a shadowing alias declaration exists but cannot be resolved.

## Scope

- Local aliases like `const alias = missing; alias()` must not fall back to an outer alias of the same name.
- Unsupported alias expressions like conditional RHS forms must still count as shadowing declarations.
- Applies to TS and Python shadowing behavior already covered by the existing alias machinery.

## Outcome

- Alias declarations are now recorded even when their RHS is unsupported.
- Unresolved or unsupported shadowing aliases now block fallback to outer-scope aliases or other same-name targets.
- Negative regression coverage now protects both unresolved and unsupported shadowing cases.

## Validation

- `cargo test -p ucp-api codegraph -- --nocapture`
- `cargo test -p ucp-cli test_codegraph_build_inspect_prompt_workflow -- --nocapture`
- Deterministic real-UCP build remained stable with only the 3 deliberate edge-case warnings.

## Status

Completed in this branch.
