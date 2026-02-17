# Sprint 37 (UCP): Codebase -> UCM Structural Graph Engine

**Goal:** Implement the codebase extractor that generates `CodeGraphProfile v1` UCM documents from Git repositories using the Sprint 36 readiness contracts.

> **Prerequisite:** `ops/sprint-36-ucm-codegraph-readiness.md` completed.  
> This sprint is intentionally scoped to the **core graph creation engine** in UCP.  
> Hivemind-owned concerns (eventing, lifecycle triggers, governance gates, snapshot policy/versioning) are handled in Hivemind Phase 3 Sprint 37 integration.

---

## 37.0 Precondition and Scope Commitments

- [x] Treat Sprint 36 contracts as hard prerequisites:
  - [x] `CodeGraphProfile v1`
  - [x] canonical fingerprint API
  - [x] stable `logical_key` requirements
  - [x] build result envelope + diagnostics schema
- [x] Graph output is a pure projection of repository state at a commit.
- [x] Output is deterministic for identical `(commit_hash, extractor_version, config, profile_version)`.
- [x] Graph is orientation-first, not semantic-static-analysis.
- [x] Engine reuses existing UCP primitives (`ucm-core`, `ucm-engine`, `ucp-llm`) as first choice.
- [x] No hidden mutable caches that change output semantics.

---

## 37.1 Product Scope (Orientation, Not Validation)

### In Scope

- [x] Repository node
- [x] Directory nodes (filtered, meaningful only)
- [x] File nodes (supported code files)
- [x] Top-level symbol nodes (declarations only; no internals)
- [x] Contains edges (hierarchy)
- [x] Import/dependency edges (file -> file)
- [x] Optional export edges (language-dependent, best effort)

### Explicitly Out of Scope

- [x] Function/method call graph
- [x] Control-flow graph
- [x] Type-flow/inheritance graph
- [x] Runtime execution inference
- [x] Compiler-level semantic validation

---

## 37.2 Reuse of Existing UCP Primitives

### UCM Data Model Reuse

- [x] Represent the graph as `ucm_core::Document`
- [x] Emit profile marker and metadata conforming to `CodeGraphProfile v1`
- [x] Use `Document.structure` as the canonical hierarchy (`repository -> directory -> file -> symbol`)
- [x] Use `EdgeType::References` for dependency/import relationships
- [x] Populate required per-node metadata keys including stable `logical_key`
- [x] Store extractor metadata in document/block metadata (not side channels)

### Engine/Traversal Reuse

- [x] Use `ucm-engine` validation/snapshot/traversal utilities where applicable
- [x] Reuse existing index behavior (`DocumentIndices`, `EdgeIndex`) for querying

### Prompt Projection Reuse

- [x] Provide a graph-to-prompt projection compatible with `ucp-llm` style:
  - [x] `Document structure:` section
  - [x] `Blocks:` section
- [x] Reuse `IdMapper::from_document()` and deterministic block ordering semantics
- [x] Ensure structure+blocks output is stable across runs for same profile input

---

## 37.3 Extraction Technology and Parsing Strategy

### Parsing Layer

- [x] Use Tree-sitter as the primary parser strategy:
  - [x] `tree-sitter`
  - [x] language grammars (`tree-sitter-rust`, `tree-sitter-typescript`, `tree-sitter-python`)
- [ ] Use `ignore` crate for deterministic filesystem traversal with `.gitignore` support
- [ ] Use `rayon` for parallel parsing where safe

### Non-Goals for Parser Layer

- [x] No LSP daemons
- [x] No regex-only parsing fallback for supported languages
- [x] No runtime/compiler plugin dependency for v1

---

## 37.4 Canonical UCM Graph Taxonomy

### Node Representation (UCM Blocks, Profile-Conformant)

- [x] Repository block:
  - [x] semantic role: `custom.repository`
  - [x] metadata label: repo name
- [x] Directory block:
  - [x] semantic role: `custom.directory`
  - [x] metadata contains normalized path
- [x] File block:
  - [x] semantic role: `custom.file`
  - [x] metadata contains normalized path + language hint
- [x] Symbol block (top-level only):
  - [x] semantic role: `custom.symbol`
  - [x] metadata contains symbol kind/name/span/exported flag

### Edge Representation

- [x] Hierarchy expressed through `Document.structure` (contains relation)
- [x] Import dependencies as explicit `EdgeType::References`
- [x] Optional exports as explicit edges (custom metadata-tagged relation)
- [x] Validate all emitted nodes/edges against `CodeGraphProfile v1`

---

## 37.5 Filesystem and Import Extraction Pipeline

### Traversal

- [x] Root at a repository worktree path
- [x] Deterministic sorted path iteration
- [x] Path normalization (`/` separators, no platform-specific drift)
- [x] Extension/language filtering via explicit extractor config

### Import Extraction

- [ ] Per-language Tree-sitter queries stored under extractor assets
- [x] Best-effort resolution from import text -> target file node
- [x] Unresolved imports are captured as diagnostics, not hard failures

### Symbol Extraction

- [x] Capture only top-level declarations (function/class/struct/interface/type/module)
- [x] Do not descend into nested/internal function bodies
- [x] Preserve source span metadata for later navigation

---

## 37.6 Determinism and Fingerprint Contract

- [x] Stable traversal order for files and extracted entities
- [x] Stable block creation order before document finalization
- [x] Stable dependency edge sorting before insertion
- [x] Stable prompt projection ordering (`Document structure`, then `Blocks`)
- [x] Compute canonical fingerprint using Sprint 36 canonicalization API (no ad-hoc hash logic)
- [x] Determinism test suite validates stable canonical fingerprint for fixed fixtures

---

## 37.7 API and CLI Surface (UCP-owned)

### Rust API

- [x] Add a public API in `ucp-api` for building codebase graphs:
  - [x] input: repository path + commit hash + extractor config
  - [x] output: Sprint 36 build result envelope (`document`, `diagnostics`, `stats`, `profile_version`, `canonical_fingerprint`)
- [x] Ensure output UCM document passes `CodeGraphProfile v1` validator

### CLI

- [x] Add `ucp` CLI command group for code graph extraction (build/inspect/prompt)
- [x] `build` outputs serialized UCM document
- [x] `build` can emit profile version + canonical fingerprint + diagnostics summary
- [x] `prompt` outputs structure+blocks projection for LLM context

---

## 37.8 Performance and Scale Targets

- [ ] Medium repo target (~2k files): generation in under 3 seconds on modern dev machine
- [ ] Typical memory bounded for medium repos (document + indices + parser state)
- [x] Graph scale expectation remains practical for orientation use:
  - [x] directory/file/symbol node counts remain tractable
  - [x] no explosion from deep semantic analysis

---

## 37.9 Failure Semantics

- [x] Structured diagnostics for:
  - [x] unsupported language
  - [x] parse failures
  - [x] unreadable files/directories
  - [x] unresolved imports
- [x] Catastrophic extractor failure returns explicit error, never silent partial success
- [x] Non-catastrophic per-file parse failures can continue with diagnostics if configured
- [x] Profile validation failures are surfaced as first-class errors before output is marked successful

---

## 37.10 UCP Exit Criteria

- [x] UCP produces a deterministic UCM structural graph from a repository snapshot
- [x] Output is valid `CodeGraphProfile v1` and includes stable logical keys
- [x] Graph includes repository/directory/file/top-level-symbol layers
- [x] Import/dependency edges are extracted and queryable
- [x] Structure+blocks prompt projection is available and stable
- [x] Canonical fingerprint is stable for identical extractor inputs
- [x] API + CLI surfaces are usable by external orchestrators (including Hivemind)
- [x] Determinism/performance/failure-mode tests are in place

---

## 37.11 Integration Boundary with Hivemind

UCP owns:

- [x] Core extraction engine and UCM graph projection
- [x] Node/edge taxonomy and deterministic output contract
- [x] Prompt-style structure+blocks projection from UCM graph

Hivemind owns:

- [ ] When extraction runs (attach/checkpoint/merge/manual refresh)
- [ ] Event emission and observability around extraction
- [ ] Governance/constitution checks using extracted graph
- [ ] Snapshot storage policy and lifecycle in Hivemind data model

---

## 37.12 Compatibility and Legacy-System Assurance

- [x] New extractor paths do not break existing UCP document APIs and CLI commands.
- [x] Generated codegraph documents remain valid UCM documents consumable by existing engine/traversal utilities.
- [ ] Rust/Python/WASM bindings can load and inspect generated codegraph documents without compatibility breaks.
- [x] Existing `ucp-llm` structure+blocks rendering works for codegraph documents without custom forks.
- [x] Regression suite covers representative pre-existing UCP workflows alongside codegraph workflows.

---

## 37.13 Manual Testing (`@ucp-manual-test`)

- [x] Build and run a manual extraction matrix from `@ucp-manual-test` (`/home/antonio/programming/Hivemind/ucp-manual-test`) across at least:
  - [x] single-language repository
  - [x] multi-language repository
  - [x] repository with unresolved imports and parser edge cases
- [x] Manually inspect generated UCM graph documents for hierarchy correctness and import edge quality.
- [x] Manually verify structure+blocks prompt projection output stability for repeated runs.
- [x] Manually verify API/CLI behavior for diagnostics severity and partial-success handling.
- [x] Record a manual sprint test report in `@ucp-manual-test` with pass/fail outcomes and follow-up fixes.
