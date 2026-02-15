# Sprint 37 (UCP): Codebase -> UCM Structural Graph Engine

**Goal:** Implement the codebase extractor that generates `CodeGraphProfile v1` UCM documents from Git repositories using the Sprint 36 readiness contracts.

> **Prerequisite:** `ops/sprint-36-ucm-codegraph-readiness.md` completed.  
> This sprint is intentionally scoped to the **core graph creation engine** in UCP.  
> Hivemind-owned concerns (eventing, lifecycle triggers, governance gates, snapshot policy/versioning) are handled in Hivemind Phase 3 Sprint 37 integration.

---

## 37.0 Precondition and Scope Commitments

- [ ] Treat Sprint 36 contracts as hard prerequisites:
  - [ ] `CodeGraphProfile v1`
  - [ ] canonical fingerprint API
  - [ ] stable `logical_key` requirements
  - [ ] build result envelope + diagnostics schema
- [ ] Graph output is a pure projection of repository state at a commit.
- [ ] Output is deterministic for identical `(commit_hash, extractor_version, config, profile_version)`.
- [ ] Graph is orientation-first, not semantic-static-analysis.
- [ ] Engine reuses existing UCP primitives (`ucm-core`, `ucm-engine`, `ucp-llm`) as first choice.
- [ ] No hidden mutable caches that change output semantics.

---

## 37.1 Product Scope (Orientation, Not Validation)

### In Scope

- [ ] Repository node
- [ ] Directory nodes (filtered, meaningful only)
- [ ] File nodes (supported code files)
- [ ] Top-level symbol nodes (declarations only; no internals)
- [ ] Contains edges (hierarchy)
- [ ] Import/dependency edges (file -> file)
- [ ] Optional export edges (language-dependent, best effort)

### Explicitly Out of Scope

- [ ] Function/method call graph
- [ ] Control-flow graph
- [ ] Type-flow/inheritance graph
- [ ] Runtime execution inference
- [ ] Compiler-level semantic validation

---

## 37.2 Reuse of Existing UCP Primitives

### UCM Data Model Reuse

- [ ] Represent the graph as `ucm_core::Document`
- [ ] Emit profile marker and metadata conforming to `CodeGraphProfile v1`
- [ ] Use `Document.structure` as the canonical hierarchy (`repository -> directory -> file -> symbol`)
- [ ] Use `EdgeType::References` for dependency/import relationships
- [ ] Populate required per-node metadata keys including stable `logical_key`
- [ ] Store extractor metadata in document/block metadata (not side channels)

### Engine/Traversal Reuse

- [ ] Use `ucm-engine` validation/snapshot/traversal utilities where applicable
- [ ] Reuse existing index behavior (`DocumentIndices`, `EdgeIndex`) for querying

### Prompt Projection Reuse

- [ ] Provide a graph-to-prompt projection compatible with `ucp-llm` style:
  - [ ] `Document structure:` section
  - [ ] `Blocks:` section
- [ ] Reuse `IdMapper::from_document()` and deterministic block ordering semantics
- [ ] Ensure structure+blocks output is stable across runs for same profile input

---

## 37.3 Extraction Technology and Parsing Strategy

### Parsing Layer

- [ ] Use Tree-sitter as the primary parser strategy:
  - [ ] `tree-sitter`
  - [ ] language grammars (`tree-sitter-rust`, `tree-sitter-typescript`, `tree-sitter-python`)
- [ ] Use `ignore` crate for deterministic filesystem traversal with `.gitignore` support
- [ ] Use `rayon` for parallel parsing where safe

### Non-Goals for Parser Layer

- [ ] No LSP daemons
- [ ] No regex-only parsing fallback for supported languages
- [ ] No runtime/compiler plugin dependency for v1

---

## 37.4 Canonical UCM Graph Taxonomy

### Node Representation (UCM Blocks, Profile-Conformant)

- [ ] Repository block:
  - [ ] semantic role: `custom.repository`
  - [ ] metadata label: repo name
- [ ] Directory block:
  - [ ] semantic role: `custom.directory`
  - [ ] metadata contains normalized path
- [ ] File block:
  - [ ] semantic role: `custom.file`
  - [ ] metadata contains normalized path + language hint
- [ ] Symbol block (top-level only):
  - [ ] semantic role: `custom.symbol`
  - [ ] metadata contains symbol kind/name/span/exported flag

### Edge Representation

- [ ] Hierarchy expressed through `Document.structure` (contains relation)
- [ ] Import dependencies as explicit `EdgeType::References`
- [ ] Optional exports as explicit edges (custom metadata-tagged relation)
- [ ] Validate all emitted nodes/edges against `CodeGraphProfile v1`

---

## 37.5 Filesystem and Import Extraction Pipeline

### Traversal

- [ ] Root at a repository worktree path
- [ ] Deterministic sorted path iteration
- [ ] Path normalization (`/` separators, no platform-specific drift)
- [ ] Extension/language filtering via explicit extractor config

### Import Extraction

- [ ] Per-language Tree-sitter queries stored under extractor assets
- [ ] Best-effort resolution from import text -> target file node
- [ ] Unresolved imports are captured as diagnostics, not hard failures

### Symbol Extraction

- [ ] Capture only top-level declarations (function/class/struct/interface/type/module)
- [ ] Do not descend into nested/internal function bodies
- [ ] Preserve source span metadata for later navigation

---

## 37.6 Determinism and Fingerprint Contract

- [ ] Stable traversal order for files and extracted entities
- [ ] Stable block creation order before document finalization
- [ ] Stable dependency edge sorting before insertion
- [ ] Stable prompt projection ordering (`Document structure`, then `Blocks`)
- [ ] Compute canonical fingerprint using Sprint 36 canonicalization API (no ad-hoc hash logic)
- [ ] Determinism test suite validates stable canonical fingerprint for fixed fixtures

---

## 37.7 API and CLI Surface (UCP-owned)

### Rust API

- [ ] Add a public API in `ucp-api` for building codebase graphs:
  - [ ] input: repository path + commit hash + extractor config
  - [ ] output: Sprint 36 build result envelope (`document`, `diagnostics`, `stats`, `profile_version`, `canonical_fingerprint`)
- [ ] Ensure output UCM document passes `CodeGraphProfile v1` validator

### CLI

- [ ] Add `ucp` CLI command group for code graph extraction (build/inspect/prompt)
- [ ] `build` outputs serialized UCM document
- [ ] `build` can emit profile version + canonical fingerprint + diagnostics summary
- [ ] `prompt` outputs structure+blocks projection for LLM context

---

## 37.8 Performance and Scale Targets

- [ ] Medium repo target (~2k files): generation in under 3 seconds on modern dev machine
- [ ] Typical memory bounded for medium repos (document + indices + parser state)
- [ ] Graph scale expectation remains practical for orientation use:
  - [ ] directory/file/symbol node counts remain tractable
  - [ ] no explosion from deep semantic analysis

---

## 37.9 Failure Semantics

- [ ] Structured diagnostics for:
  - [ ] unsupported language
  - [ ] parse failures
  - [ ] unreadable files/directories
  - [ ] unresolved imports
- [ ] Catastrophic extractor failure returns explicit error, never silent partial success
- [ ] Non-catastrophic per-file parse failures can continue with diagnostics if configured
- [ ] Profile validation failures are surfaced as first-class errors before output is marked successful

---

## 37.10 UCP Exit Criteria

- [ ] UCP produces a deterministic UCM structural graph from a repository snapshot
- [ ] Output is valid `CodeGraphProfile v1` and includes stable logical keys
- [ ] Graph includes repository/directory/file/top-level-symbol layers
- [ ] Import/dependency edges are extracted and queryable
- [ ] Structure+blocks prompt projection is available and stable
- [ ] Canonical fingerprint is stable for identical extractor inputs
- [ ] API + CLI surfaces are usable by external orchestrators (including Hivemind)
- [ ] Determinism/performance/failure-mode tests are in place

---

## 37.11 Integration Boundary with Hivemind

UCP owns:

- [ ] Core extraction engine and UCM graph projection
- [ ] Node/edge taxonomy and deterministic output contract
- [ ] Prompt-style structure+blocks projection from UCM graph

Hivemind owns:

- [ ] When extraction runs (attach/checkpoint/merge/manual refresh)
- [ ] Event emission and observability around extraction
- [ ] Governance/constitution checks using extracted graph
- [ ] Snapshot storage policy and lifecycle in Hivemind data model

---

## 37.12 Compatibility and Legacy-System Assurance

- [ ] New extractor paths do not break existing UCP document APIs and CLI commands.
- [ ] Generated codegraph documents remain valid UCM documents consumable by existing engine/traversal utilities.
- [ ] Rust/Python/WASM bindings can load and inspect generated codegraph documents without compatibility breaks.
- [ ] Existing `ucp-llm` structure+blocks rendering works for codegraph documents without custom forks.
- [ ] Regression suite covers representative pre-existing UCP workflows alongside codegraph workflows.

---

## 37.13 Manual Testing (`@ucp-manual-test`)

- [ ] Build and run a manual extraction matrix from `@ucp-manual-test` (`/home/antonio/programming/Hivemind/ucp-manual-test`) across at least:
  - [ ] single-language repository
  - [ ] multi-language repository
  - [ ] repository with unresolved imports and parser edge cases
- [ ] Manually inspect generated UCM graph documents for hierarchy correctness and import edge quality.
- [ ] Manually verify structure+blocks prompt projection output stability for repeated runs.
- [ ] Manually verify API/CLI behavior for diagnostics severity and partial-success handling.
- [ ] Record a manual sprint test report in `@ucp-manual-test` with pass/fail outcomes and follow-up fixes.
