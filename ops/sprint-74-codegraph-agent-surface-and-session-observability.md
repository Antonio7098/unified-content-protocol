# Sprint 74: CodeGraph Agent Surface and Session Observability

## Goal

Capture the reusable CodeGraph improvements surfaced by GraphBench work and put them in `ucp-codegraph` / the Python query surface rather than reimplementing them in benchmark-specific code.

The main principle is:

- graph semantics belong in graphcode
- benchmark semantics belong in GraphBench
- runtime semantics belong in the harness

## Current State

Today the system already has two distinct layers:

1. Rust graph/programmatic API
   - `CodeGraphNavigator`
   - `CodeGraphNavigatorSession`
   - `find_nodes`
   - `path_between`
   - `expand`
   - `hydrate_source`
   - `why_selected`
   - `fork`
   - `diff`
   - `apply_recommended_actions`

2. Python agent façade
   - `ucp.query(...)`
   - `graph.find(...)`
   - `graph.describe(...)`
   - `graph.resolve(...)`
   - `graph.path(...)`
   - `graph.session()`
   - `session.add(...)`
   - `session.walk(...)`
   - `session.focus(...)`
   - `session.why(...)`
   - `session.export(...)`
   - `session.fork()` / `session.diff(...)`
   - `session.hydrate(...)`
   - `run_python_query(...)`
   - `PythonQueryTool(...)`

The Python façade is the intended agent-facing API. It is primarily a thin ergonomic layer plus a guarded Python query runner. It is not a separate semantics layer.

## Decision

Do not build a second Rust "agent API" that only renames existing Rust methods to match the Python façade.

That would duplicate wrappers without adding reusable capability.

Instead:

- keep the Rust graph/programmatic API as the core implementation
- keep the Python façade as the intended agent-facing surface
- add missing reusable graph/session capabilities to `ucp-codegraph`
- expose those new capabilities through thin Python bindings and the façade when useful

## Improvements That Belong in CodeGraph

### 1. Session mutation telemetry

Add first-class structured telemetry for graph/session mutations.

Every mutation should be able to emit a typed record for operations such as:

- select
- focus
- expand
- hydrate
- collapse
- pin
- unpin
- prune
- apply recommended actions

Each record should preserve:

- operation name
- selector or block id target
- resolved block ids
- traversal parameters
- nodes added
- nodes removed
- nodes changed
- focus change
- elapsed time
- mutation reason

This is reusable for any graph-driven agent, not just GraphBench.

### 2. Better provenance and negative explanations

`why_selected(...)` exists, but it is not enough.

Add richer explainability for:

- full provenance chain for a selected node
- why a node was omitted from export/rendering
- why a candidate was pruned
- why a recommendation was generated
- why a selector resolved the way it did when ambiguity exists

GraphBench needs this for scoring and audit, but the capability is broadly useful.

### 3. First-class session persistence

Graph persistence exists. Session export exists. The missing step is stable session persistence as a first-class concept.

Add:

- session save/load APIs
- stable session schema/version
- session identity and snapshot hashes
- replayable mutation history or mutation log references

This would help debugging, reproducibility, UI integration, and agent resume flows.

### 4. Recommendation API with structured reasons

`apply_recommended_actions(...)` currently returns applied action strings plus an update.

It should instead expose recommendation objects with fields like:

- action kind
- target node
- relation set
- priority
- candidate count
- estimated evidence gain
- estimated token or hydration cost
- explanation / rationale

This would make recommendations inspectable, rankable, and benchmarkable.

### 5. Pre-mutation cost estimation

Add APIs that estimate likely cost before mutating a session.

Examples:

- estimated nodes added by an expansion
- estimated rendered bytes or tokens for hydration
- estimated frontier width
- estimated export growth

This supports budget-aware agents and GraphBench strategy evaluation.

### 6. Bounded traversal and operation budgets in Rust

The Python query runner already has limits around:

- max seconds
- max operations
- max trace events
- max stdout chars

The Rust graph core should have analogous budget objects for graph operations themselves, such as:

- max depth
- max nodes visited
- max nodes added
- max hydrated bytes
- max elapsed time
- max emitted telemetry events

This should live in graphcode, not in benchmark-specific wrappers.

### 7. Session-level event sink / observer hooks

Add observer hooks so callers can subscribe to structured graph/session events without patching graphcode internals.

Use cases:

- benchmark tracing
- UI live updates
- debugging
- usage analytics
- offline replay support

This is likely the cleanest way to support GraphBench observability without contaminating graphcode with benchmark policy.

### 8. Export/render omission reporting

Current export is useful, but agents and evaluators need explicit omission information.

Add omission details such as:

- nodes hidden by visible-level limits
- nodes excluded by class filters
- nodes dropped by token or byte budgets
- nodes suppressed because a hydrated excerpt superseded a summary node

This is highly reusable for prompt construction systems.

## Improvements That Do Not Belong in CodeGraph

These should stay outside graphcode:

- prompt assembly
- LLM/provider integration
- runtime turn loop
- tool execution
- benchmark evidence matching
- benchmark scoring
- run orchestration

Those belong in the harness or GraphBench.

## API Direction

The desired long-term shape is:

- Rust core remains the semantic source of truth
- Python bindings remain thin wrappers over the Rust core
- Python façade remains the intended agent-facing API

If new reusable graph semantics are added, the order should be:

1. implement in Rust core
2. expose in Python bindings
3. expose through the Python façade where it improves agent ergonomics

Do not add Python-only semantics that the Rust core cannot represent.

## Why This Matters

GraphBench is going to pressure-test graph traversal, working-set management, provenance, and observability very hard.

If that pressure reveals reusable graph capabilities, they should be pushed into CodeGraph so they can benefit:

- agent workflows
- local debugging tools
- UI sessions
- non-benchmark prompt construction systems
- future runtime integrations
