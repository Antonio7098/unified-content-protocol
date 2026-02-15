# Sprint 36 (UCP): UCM CodeGraph Readiness

**Goal:** Prepare the UCM model and UCP interfaces so codebase graph extraction (Sprint 37) can be deterministic, stable, and reusable without introducing a separate graph model.

> Strategy: keep a **single unified UCM `Document` model**, and add an optional **CodeGraph profile** with strict contracts.

---

## 36.0 Architectural Commitments

- [ ] No separate “code graph document type”; code graph remains a UCM `Document`.
- [ ] Code graph behavior is defined by a profile contract, not ad-hoc conventions.
- [ ] Deterministic serialization and hashing are first-class capabilities.
- [ ] Stable logical identity exists independently from content-addressed block IDs.

---

## 36.1 CodeGraph Profile v1 (UCM-Level Contract)

- [ ] Define a formal `CodeGraphProfile v1` specification for UCM documents.
- [ ] Standardize required node classes:
  - [ ] repository
  - [ ] directory
  - [ ] file
  - [ ] symbol (top-level declarations only)
- [ ] Standardize required edge semantics:
  - [ ] contains (via `Document.structure`)
  - [ ] imports/depends_on (explicit edge relationship)
  - [ ] optional exports (best effort)
- [ ] Define required metadata keys per node class (path/language/symbol kind/span/exported).
- [ ] Define profile version marker embedded in document metadata.

---

## 36.2 Deterministic Canonicalization and Fingerprinting

- [ ] Add canonical serialization rules for code-graph documents:
  - [ ] stable ordering of blocks
  - [ ] stable ordering of structure children
  - [ ] stable ordering of edges and metadata maps
- [ ] Define canonical fingerprint input and output:
  - [ ] include structural and semantic graph content
  - [ ] exclude volatile timestamps/non-semantic runtime fields
- [ ] Provide a reusable API to compute `canonical_fingerprint`.
- [ ] Add conformance tests ensuring byte-stable canonical output for fixed fixtures.

---

## 36.3 Stable Logical Identity Layer

- [ ] Introduce profile-required `logical_key` metadata for diff stability:
  - [ ] file example: `file:src/core/state.rs`
  - [ ] symbol example: `symbol:src/core/state.rs::AppState`
- [ ] Define uniqueness constraints for `logical_key` within a document.
- [ ] Add validator checks for duplicate/missing/invalid logical keys.
- [ ] Define how logical keys interact with content-derived `BlockId` churn.

---

## 36.4 Build Result Envelope (API Contract)

- [ ] Define a standard build result type for code-graph generation:
  - [ ] `document: ucm_core::Document`
  - [ ] `diagnostics: Vec<...>`
  - [ ] `stats` (node/edge/language counts)
  - [ ] `profile_version`
  - [ ] `canonical_fingerprint`
- [ ] Classify diagnostics severity (`error`, `warning`, `info`) with machine-readable codes.
- [ ] Ensure partial-success semantics are explicit and never silent.

---

## 36.5 Validation and Conformance Tooling

- [ ] Add `CodeGraphProfile` validator to verify structural/profile compliance.
- [ ] Add deterministic roundtrip tests (serialize -> deserialize -> canonicalize -> hash).
- [ ] Add fixture repositories for multi-language and edge-case coverage.
- [ ] Add performance smoke validation for medium-size fixtures.

---

## 36.6 API/CLI Readiness Hooks

- [ ] Expose readiness primitives in `ucp-api` (profile validation + fingerprint computation).
- [ ] Add CLI support to validate profile documents and print canonical fingerprint.
- [ ] Keep these APIs generic so Sprint 37 extractor and external orchestrators (e.g., Hivemind) can consume them directly.

---

## 36.7 Compatibility and Non-Regression

- [ ] Existing UCM document workflows remain valid with no required migration for non-code documents.
- [ ] Existing serialization/deserialization contracts remain backward-compatible for Rust, Python, and WASM SDKs.
- [ ] Existing `ucp-cli` document/block/edge operations continue to behave identically for non-codegraph usage.
- [ ] `ucp-llm` utilities (`IdMapper`, prompt projections) remain compatible with prior document structures.
- [ ] Translators and current engine validation/snapshot features remain operational.

---

## 36.8 Manual Testing (`@ucp-manual-test`)

- [ ] Create/update manual test checklist and fixtures under `@ucp-manual-test` (`/home/antonio/programming/Hivemind/ucp-manual-test`).
- [ ] Manually validate profile compliance on representative documents (valid + invalid cases).
- [ ] Manually validate canonical fingerprint stability across repeated runs and reload cycles.
- [ ] Manually validate backward compatibility of existing document/block/edge workflows against readiness changes.
- [ ] Record manual test outcomes and known gaps in a sprint-specific report artifact.

---

## 36.9 Exit Criteria

- [ ] `CodeGraphProfile v1` is specified and validated against fixtures.
- [ ] Canonical fingerprinting is deterministic and reusable.
- [ ] Stable logical keys are required and validated.
- [ ] Build result envelope is defined and consumable by external systems.
- [ ] UCM is ready for Sprint 37 extraction without model fragmentation.
- [ ] No breaking regression is introduced to existing UCP systems.
- [ ] Manual validation in `@ucp-manual-test` is completed and documented.
