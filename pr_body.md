# Agent Graph Traversal System (v0.1.7)

## Summary
This PR introduces the **Agent Graph Traversal System**, a powerful new capability for AI agents to navigate, search, and manage context within the Unified Content Protocol. It also completes the migration of the Python and JavaScript SDKs to use direct Rust bindings via PyO3 and wasm-bindgen, replacing the legacy pure-language implementations.

## Key Changes

### 1. New Crate: `ucp-agent`
- **Session Management**: Stateful agent sessions with cursor tracking, history, and safety limits.
- **Graph Traversal**: `navigate_to`, `expand` (BFS/DFS/Semantic), `find_path`, and neighborhood caching.
- **View Modes**: Flexible content views (`Full`, `Preview`, `Metadata`, `IdsOnly`, `Adaptive`) to manage token usage.
- **Safety**: Circuit breakers, depth guards, and operation budgets.
- **RAG Interface**: Pluggable `RagProvider` trait for semantic search integration.

### 2. UCL Parser Extensions
- Added traversal commands: `GOTO`, `BACK`, `EXPAND`, `FOLLOW`, `PATH`, `SEARCH`, `FIND`, `VIEW`.
- Added context commands: `CTX ADD`, `CTX REMOVE`, `CTX CLEAR`, `CTX EXPAND`, `CTX COMPRESS`, `CTX PRUNE`, `CTX RENDER`, `CTX STATS`, `CTX FOCUS`.

### 3. SDK Migration
- **Python**: Replaced `packages/ucp-python` with `crates/ucp-python` (PyO3 bindings).
- **WASM/JS**: Replaced `packages/ucp-js` with `crates/ucp-wasm` (wasm-bindgen).
- **Parity**: Ensure all core engine features (transactions, section writing, validation) are available in SDKs.

### 4. Cleanup
- Removed legacy `detailed_failure_analysis.md` and `proper.txt` notes.

## Tests & Coverage
- **Rust**: All crates passing (`cargo test --all-features`).
  - `ucp-agent`: 38 integration tests covering full session lifecycle.
- **WASM**: JS integration tests passing (`npm test` in `crates/ucp-wasm/tests`).
  - 124 tests covering Document, Content, Engine, and Agent operations.

## Documentation
- Added `docs/ucp-agent/` with:
  - `index.md`: System overview.
  - `api.md`: Rust/Python/JS API reference.
  - `ucl-commands.md`: UCL command syntax guide.
  - `examples.md`: Usage patterns.
  - `architecture.md`: Design decisions.

## Versioning
- Bumped workspace version to `0.1.7`.
- Changelog updated in `changelog.json`.
