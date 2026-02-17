# Sprint 36 (UCP): UCM CodeGraph Readiness

**Goal:** Prepare the UCM model and UCP interfaces so codebase graph extraction (Sprint 37) can be deterministic, stable, and reusable without introducing a separate graph model.

> Strategy: keep a **single unified UCM `Document` model**, and add an optional **CodeGraph profile** with strict contracts.

---

## 36.0 Architectural Commitments

- [x] No separate “code graph document type”; code graph remains a UCM `Document`.
- [x] Code graph behavior is defined by a profile contract, not ad-hoc conventions.
- [x] Deterministic serialization and hashing are first-class capabilities.
- [x] Stable logical identity exists independently from content-addressed block IDs.

---

## 36.1 CodeGraph Profile v1 (UCM-Level Contract)

- [x] Define a formal `CodeGraphProfile v1` specification for UCM documents.
- [x] Standardize required node classes:
  - [x] repository
  - [x] directory
  - [x] file
  - [x] symbol (top-level declarations only)
- [x] Standardize required edge semantics:
  - [x] contains (via `Document.structure`)
  - [x] imports/depends_on (explicit edge relationship)
  - [x] optional exports (best effort)
- [x] Define required metadata keys per node class (path/language/symbol kind/span/exported).
- [x] Define profile version marker embedded in document metadata.

---

## 36.2 Deterministic Canonicalization and Fingerprinting

- [x] Add canonical serialization rules for code-graph documents:
  - [x] stable ordering of blocks
  - [x] stable ordering of structure children
  - [x] stable ordering of edges and metadata maps
- [x] Define canonical fingerprint input and output:
  - [x] include structural and semantic graph content
  - [x] exclude volatile timestamps/non-semantic runtime fields
- [x] Provide a reusable API to compute `canonical_fingerprint`.
- [x] Add conformance tests ensuring byte-stable canonical output for fixed fixtures.

---

## 36.3 Stable Logical Identity Layer

- [x] Introduce profile-required `logical_key` metadata for diff stability:
  - [x] file example: `file:src/core/state.rs`
  - [x] symbol example: `symbol:src/core/state.rs::AppState`
- [x] Define uniqueness constraints for `logical_key` within a document.
- [x] Add validator checks for duplicate/missing/invalid logical keys.
- [x] Define how logical keys interact with content-derived `BlockId` churn.

---

## 36.4 Build Result Envelope (API Contract)

- [x] Define a standard build result type for code-graph generation:
  - [x] `document: ucm_core::Document`
  - [x] `diagnostics: Vec<...>`
  - [x] `stats` (node/edge/language counts)
  - [x] `profile_version`
  - [x] `canonical_fingerprint`
- [x] Classify diagnostics severity (`error`, `warning`, `info`) with machine-readable codes.
- [x] Ensure partial-success semantics are explicit and never silent.

---

## 36.5 Validation and Conformance Tooling

- [x] Add `CodeGraphProfile` validator to verify structural/profile compliance.
- [x] Add deterministic roundtrip tests (serialize -> deserialize -> canonicalize -> hash).
- [x] Add fixture repositories for multi-language and edge-case coverage.
- [x] Add performance smoke validation for medium-size fixtures.

---

## 36.6 API/CLI Readiness Hooks

- [x] Expose readiness primitives in `ucp-api` (profile validation + fingerprint computation).
- [x] Add CLI support to validate profile documents and print canonical fingerprint.
- [x] Keep these APIs generic so Sprint 37 extractor and external orchestrators (e.g., Hivemind) can consume them directly.

---

## 36.7 Compatibility and Non-Regression

- [x] Existing UCM document workflows remain valid with no required migration for non-code documents.
- [x] Existing serialization/deserialization contracts remain backward-compatible for Rust, Python, and WASM SDKs.
- [x] Existing `ucp-cli` document/block/edge operations continue to behave identically for non-codegraph usage.
- [x] `ucp-llm` utilities (`IdMapper`, prompt projections) remain compatible with prior document structures.
- [x] Translators and current engine validation/snapshot features remain operational.

---

## 36.8 Manual Testing (`@ucp-manual-test`)

- [x] Create/update manual test checklist and fixtures under `@ucp-manual-test` (`/home/antonio/programming/Hivemind/ucp-manual-test`).
- [x] Manually validate profile compliance on representative documents (valid + invalid cases).
- [x] Manually validate canonical fingerprint stability across repeated runs and reload cycles.
- [x] Manually validate backward compatibility of existing document/block/edge workflows against readiness changes.
- [x] Record manual test outcomes and known gaps in a sprint-specific report artifact.

---

## 36.9 Exit Criteria

- [x] `CodeGraphProfile v1` is specified and validated against fixtures.
- [x] Canonical fingerprinting is deterministic and reusable.
- [x] Stable logical keys are required and validated.
- [x] Build result envelope is defined and consumable by external systems.
- [x] UCM is ready for Sprint 37 extraction without model fragmentation.
- [x] No breaking regression is introduced to existing UCP systems.
- [x] Manual validation in `@ucp-manual-test` is completed and documented.
