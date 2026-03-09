# Sprint 40 (UCP): CodeGraph External Dependencies and Language Pack

## Goal

Extend the graph from local-file dependency edges toward a fuller dependency model while broadening language coverage.

## Scope

- Represent external package/crate dependencies explicitly instead of silently classifying them as non-file imports.
- Add at least one new language pack (recommended: Go first, then Java).
- Add fixtures and real-repo validation for mixed local + external dependency graphs.

## Checklist

- [ ] Define profile additions for external dependency nodes/edges.
- [ ] Emit external dependency nodes for Rust crates, Python packages, and TS/JS packages.
- [ ] Add Go extraction support with file/symbol/import coverage.
- [ ] Add Java extraction support with file/symbol/import coverage.
- [ ] Add mixed-language fixtures proving external dependency capture.
- [ ] Validate on at least one real codebase per new language.

## Exit Criteria

- External dependencies are visible in the graph rather than only inferred from diagnostics.
- Unsupported-language sample repos no longer collapse to repository-only graphs for the newly added languages.
- Deterministic fingerprints remain stable.

## Status

Planned next.