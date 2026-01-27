# Architecture

## System Overview

The Agent Traversal System is designed around the following key principles:

1. **Session Isolation**: Each agent operates in an isolated session with independent state
2. **Safety First**: Multiple safeguards prevent runaway operations and resource exhaustion
3. **Observability**: Built-in metrics enable monitoring and debugging
4. **Extensibility**: Pluggable RAG provider interface for custom semantic search
5. **Concurrency-Safe**: Uses atomic operations for thread-safe metrics
6. **Multi-language**: Rust core with Python and WASM bindings

## Core Components

### Session Management (`session.rs`)

Manages individual agent sessions with isolated state:

```
Session State:
├── Position: Current cursor location
├── History: Navigation breadcrumbs
├── Context: Active context blocks
├── Capabilities: Permission flags
├── Limits: Per-session constraints
└── Metrics: Operation tracking
```

**Key Types:**
- `AgentSession`: Per-session state container
- `SessionConfig`: Builder for session creation
- `AgentCapabilities`: Permission model
- `SessionLimits`: Per-session constraints

### Cursor & Navigation (`cursor.rs`)

Tracks the agent's position and provides fast neighborhood access:

```
TraversalCursor:
├── Position: Current block ID
├── Neighborhood: Cached visible area
│   ├── Ancestors: Parent chain
│   ├── Children: Direct children
│   ├── Siblings: Same-parent blocks
│   └── Connections: Semantic edges
└── Breadcrumbs: History for BACK
```

**Features:**
- Efficient neighborhood caching (lazily recomputed)
- Configurable history size
- View mode support for different display options

### Traversal Operations (`operations.rs`)

The main `AgentTraversal` interface orchestrating all operations:

```
AgentTraversal:
├── Session Management
│   ├── create_session()
│   └── close_session()
├── Navigation
│   ├── navigate_to()
│   └── go_back()
├── Expansion
│   └── expand()
├── Search
│   ├── search() [async]
│   └── find_by_pattern()
├── View
│   ├── view_block()
│   └── view_neighborhood()
├── Path Finding
│   └── find_path()
└── Context
    ├── context_add()
    ├── context_remove()
    ├── context_clear()
    ├── context_focus()
    └── context_add_results()
```

Uses RwLock for safe concurrent access to session storage.

### Context Integration (external)

The traversal crate does **not** keep an in-memory context window. All `CTX *`
operations perform validation, update metrics, and emit structured events so
that a host application can manage the actual datastore (vector DB, cache, or
LLM prompt buffer). A typical integration pipeline:

1. Subscribe to executor results or binding callbacks for `ExecutionResult::Context`.
2. Persist block IDs, relevance scores, and reasons in your own storage.
3. Apply pruning, budgeting, or rendering before handing data to the LLM.

This design keeps the core traversal engine lightweight while still exposing a
consistent control surface for higher-level context managers.

### UCL Execution (`executor.rs`)

Parses and executes UCL commands:

```
parse_ucl_input()
    ↓
Convert to parser AST
    ↓
For each command:
    ├── GOTO → navigate_to()
    ├── BACK → go_back()
    ├── EXPAND → expand()
    ├── SEARCH → search()
    ├── FIND → find_by_pattern()
    ├── VIEW → view_*()
    └── CTX * → context_*()
    ↓
Return ExecutionResult[]
```

Each command stores last results for `CTX ADD RESULTS`.

### Safety System (`safety.rs`)

Multiple layers of protection:

**1. Depth Guard (RAII Pattern)**
```rust
pub struct DepthGuard {
    current: AtomicUsize,
    max: usize,
}

// Usage: Prevents stack overflow
let _guard = depth_guard.try_enter()?;
```

Prevents infinite recursion with automatic cleanup.

**2. Circuit Breaker (State Machine)**
```
Closed ──[failures ≥ threshold]──> Open ──[timeout elapsed]──> HalfOpen
  ↑                                                                ↓
  └──[3 successes in HalfOpen]──────────────────────────────────┘
```

Stops operation during failure cascade, auto-recovers after timeout.

**3. Budget Tracking**
```
OperationBudget:
├── traversal_operations: Max traversal ops
├── search_operations: Max RAG searches
└── blocks_read: Max blocks accessed
```

Enforces per-session resource quotas.

**4. Session Limits**
```
SessionLimits:
├── max_context_tokens: Total tokens allowed
├── max_context_blocks: Max blocks in context
├── max_expand_depth: Max expansion depth
├── max_results_per_operation: Results limit
└── session_timeout: Inactivity timeout
```

### RAG Provider Integration (`rag.rs`)

Pluggable semantic search with async trait:

```rust
#[async_trait]
pub trait RagProvider: Send + Sync {
    async fn search(&self, query: &str, options: RagSearchOptions) -> Result<RagSearchResults>;
    async fn embed(&self, content: &str) -> Result<Vec<f32>>;  // Optional
    fn capabilities(&self) -> RagCapabilities;
    fn name(&self) -> &str;
}
```

**Built-in Implementations:**
- `NullRagProvider`: Always returns empty results
- `MockRagProvider`: Configurable test mock

**Integration Points:**
1. Initialize with `AgentTraversal::with_rag_provider()`
2. Check availability with `capabilities()`
3. Call `search()` from traversal operations

### Metrics & Observability (`metrics.rs`)

Thread-safe operation tracking:

```
SessionMetrics (using AtomicUsize):
├── navigation_count: GOTO/BACK operations
├── expansion_count: EXPAND operations
├── search_count: SEARCH/FIND operations
├── context_add_count: CTX ADD operations
├── context_remove_count: CTX REMOVE operations
├── blocks_visited: Total blocks traversed
├── edges_followed: Total edges followed
├── total_execution_time_us: Cumulative runtime
├── error_count: Failures encountered
└── budget_warnings: Limit approaching warnings

MetricsSnapshot (for serialization):
└── All above fields + captured_at timestamp
```

Snapshots created on-demand with `session.metrics.snapshot()`.

### Error Handling (`error.rs`)

Comprehensive error taxonomy:

```
AgentError:
├── Session Errors
│   ├── SessionNotFound
│   ├── SessionExpired
│   ├── MaxSessionsReached
│   └── SessionClosed
├── Navigation Errors
│   ├── BlockNotFound
│   ├── NoPathExists
│   └── EmptyHistory
├── Safety Errors
│   ├── DepthLimitExceeded
│   ├── ContextLimitExceeded
│   ├── BudgetExhausted
│   ├── RateLimitExceeded
│   └── OperationTimeout
├── Circuit Breaker
│   └── CircuitOpen
├── RAG Errors
│   ├── RagNotConfigured
│   └── RagSearchFailed
└── Operational
    ├── OperationNotPermitted
    ├── NoResultsAvailable
    └── ParseError
```

Automatic conversion from upstream errors.

## Data Flow

### Navigation Flow

```
navigate_to(session, block)
    │
    ├─> Check session active
    ├─> Check permissions
    ├─> Verify block exists
    ├─> Record breadcrumb
    ├─> Update cursor position
    ├─> Refresh neighborhood (BFS from new position)
    ├─> Record navigation metric
    └─> Return NavigationResult
```

### Expansion Flow

```
expand(session, block, direction, options)
    │
    ├─> Check depth limit
    ├─> Check permissions
    ├─> Check budget
    ├─> DepthGuard::try_enter()
    │   │
    │   ├─> Case DOWN: BFS children
    │   ├─> Case UP: Follow parent chain
    │   ├─> Case BOTH: Bidirectional
    │   └─> Case SEMANTIC: Follow semantic edges
    │
    ├─> Apply filters (roles, tags)
    ├─> Apply view mode (IdsOnly, Preview, Full, etc)
    ├─> Record metrics
    ├─> DepthGuard::drop() [auto cleanup]
    └─> Return ExpansionResult
```

### Search Flow

```
search(session, query, options)  // async
    │
    ├─> Check RAG provider configured
    ├─> Check search budget
    ├─> Call RAG provider
    │   └─> RagProvider::search(query, options)
    │       └─> Return RagSearchResults with matches
    │
    ├─> Store results in session (for CTX ADD RESULTS)
    ├─> Record search metric
    └─> Return results
```

### Context Management Flow

```
context_add(session, block, reason, relevance)
    │
    ├─> Check permissions (can_modify_context)
    ├─> Verify block exists
    ├─> Check context limits
    ├─> Add to ContextManager
    ├─> Record metric
    └─> Return result

context_add_results(session)
    │
    ├─> Get last search/find results
    ├─> For each result:
    │   └─> context_add(block)
    │
    └─> Return added blocks
```

## Memory Model

### Session Isolation

Each session maintains independent memory:
- Position & history (small, bounded)
- Context window (configurable limit)
- Metrics (atomic counters)

```
                 AgentTraversal
                      │
        ┌─────────────┼─────────────┐
        │             │             │
    Session 1     Session 2     Session N
    ├─ cursor      ├─ cursor      ├─ cursor
    ├─ context     ├─ context     ├─ context
    ├─ history     ├─ history     ├─ history
    └─ metrics     └─ metrics     └─ metrics
```

### Context Window Memory

Context is bounded by:
- **Block count limit**: max_context_blocks (default: 200)
- **Token limit**: max_context_tokens (default: 8,000)
- **Automatic pruning**: Remove low-relevance blocks when limits exceeded

## Concurrency Strategy

### Read-Write Locking

```rust
// AgentTraversal uses RwLock for session storage
sessions: RwLock<HashMap<AgentSessionId, AgentSession>>

// Multiple readers (navigation, view, search)
let session = self.sessions.read().unwrap().get(&session_id)?;

// Single writer (context modification)
let mut session = self.sessions.write().unwrap().get_mut(&session_id)?;
```

### Atomic Metrics

```rust
// SessionMetrics uses atomics for thread-safe counters
pub navigation_count: AtomicUsize
pub search_count: AtomicUsize

// No locks needed, no contention
metrics.navigation_count.fetch_add(1, Ordering::Relaxed);
```

## Performance Characteristics

| Operation | Time Complexity | Space | Notes |
|-----------|-----------------|-------|-------|
| GOTO | O(1) | O(neighborhood) | Neighborhood cached, refreshed on demand |
| EXPAND | O(B) | O(D²) | B = blocks, D = depth; BFS from root |
| SEARCH | O(N) | O(L) | N = searchable blocks, L = limit |
| FIND | O(N) | O(M) | N = searchable blocks, M = matches |
| PATH | O(N) | O(P) | N = blocks, P = path length |
| CTX ADD | O(1) | O(B) | B = context blocks |

## Extension Points

### Custom RAG Provider

```rust
pub struct MyRagProvider {
    // implementation
}

#[async_trait]
impl RagProvider for MyRagProvider {
    async fn search(&self, query: &str, options: RagSearchOptions) -> Result<RagSearchResults> {
        // Custom implementation
    }
}

// Use with traversal
let traversal = AgentTraversal::new(doc)
    .with_rag_provider(Arc::new(MyRagProvider::new()));
```

### Custom Limits

```rust
let limits = GlobalLimits {
    max_sessions: 50,
    max_total_context_blocks: 50_000,
    max_ops_per_second: 2000.0,
    operation_timeout: Duration::from_secs(60),
};

let traversal = AgentTraversal::new(doc)
    .with_global_limits(limits);
```

### Session Configuration

```rust
let config = SessionConfig::new()
    .with_name("custom-agent")
    .with_capabilities(AgentCapabilities {
        can_traverse: true,
        can_search: false,  // Disable search
        can_modify_context: true,
        can_coordinate: false,
        ..Default::default()
    })
    .with_limits(SessionLimits {
        max_context_blocks: 100,
        max_expand_depth: 5,
        ..Default::default()
    });

let session = traversal.create_session(config)?;
```

## Testing Strategy

### Unit Tests
- Individual module functionality
- Error handling paths
- Boundary conditions

### Integration Tests (38 tests)
- Session lifecycle
- Multi-operation workflows
- Safety mechanisms
- Context management
- UCL command execution

### Test Utilities
- `create_test_document()`: Multi-level document structure
- `MockRagProvider`: Configurable mock with pre-defined results
- `fake_block_id()`: Deterministic fake IDs for "not found" scenarios

## Design Patterns

### RAII Pattern (Depth Guard)
```rust
let _guard = depth_guard.try_enter()?;
// Automatically decrements on drop
```

### State Machine (Circuit Breaker)
```rust
match state {
    Closed => { /* normal operation */ },
    Open => { /* reject requests */ },
    HalfOpen => { /* test recovery */ },
}
```

### Builder Pattern (Configuration)
```rust
SessionConfig::new()
    .with_name("agent")
    .with_limits(limits)
    .with_capabilities(caps)
```

### Plugin Pattern (RAG Provider)
```rust
#[async_trait]
pub trait RagProvider: Send + Sync {
    // Interface for pluggable implementations
}
```
