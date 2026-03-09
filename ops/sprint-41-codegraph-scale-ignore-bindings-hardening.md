# Sprint 41 (UCP): CodeGraph Scale, Ignore Semantics, and Bindings Hardening

## Goal

Make the richer graph practical at scale and fully inspectable from the SDK bindings.

## Scope

- Replace the bespoke ignore walker with production-grade ignore semantics.
- Add deterministic parallel extraction for larger repos.
- Ensure Python and WASM bindings can load and inspect generated codegraph documents without lossy serialization.

## Checklist

- [ ] Adopt the `ignore` crate for repository traversal semantics.
- [ ] Add deterministic parallel extraction with stable merge order.
- [ ] Add perf coverage for medium/large fixtures and repeat-run fingerprint checks.
- [ ] Add Python binding tests that load generated codegraph JSON and inspect metadata/edges/symbols.
- [ ] Add WASM binding tests that load generated codegraph JSON and inspect metadata/edges/symbols.
- [ ] Update docs with binding-level codegraph inspection examples.

## Exit Criteria

- Richer graphs remain deterministic and performant on larger repos.
- SDK users can round-trip and inspect generated codegraph docs from Python and WASM.
- The remaining Sprint 37 bindings-compatibility gap is closed with automated evidence.

## Status

Planned after Sprint 40, unless bindings validation is pulled forward.