# API Reference

## Session Management

### AgentTraversal

The main interface for agent graph traversal.

=== "Rust"
    ```rust
    pub struct AgentTraversal {
        // Internal session storage and document
    }

    impl AgentTraversal {
        /// Create a new traversal system from a document
        pub fn new(doc: Document) -> Self

        /// Create with custom global limits
        pub fn with_global_limits(self, limits: GlobalLimits) -> Self

        /// Set a RAG provider for semantic search
        pub fn with_rag_provider(self, provider: Arc<dyn RagProvider>) -> Self

        /// Update the internal document snapshot
        pub fn update_document(&self, doc: Document) -> Result<()>
    }
    ```

=== "Python"
    ```python
    class AgentTraversal:
        def __init__(self, doc: Document) -> None:
            """Create a new traversal system from a document"""

        def update_document(self, doc: Document) -> None:
            """Update the internal document snapshot"""
    ```

=== "JavaScript"
    ```javascript
    class WasmAgentTraversal {
        constructor(doc: Document)

        withGlobalLimits(limits: WasmGlobalLimits): WasmAgentTraversal

        updateDocument(doc: Document): void
    }
    ```

### Session Operations

#### Create Session

=== "Rust"
    ```rust
    pub fn create_session(&self, config: SessionConfig) -> Result<AgentSessionId>
    ```

=== "Python"
    ```python
    def create_session(self, config: SessionConfig | None = None) -> AgentSessionId
    ```

=== "JavaScript"
    ```javascript
    createSession(config: WasmSessionConfig | null): WasmAgentSessionId
    ```

Creates a new agent session with the given configuration.

**Parameters:**
- `config`: Session configuration (optional, uses defaults if not provided)

**Returns:** Unique session identifier

**Errors (Rust only):**
- `MaxSessionsReached`: Too many active sessions

#### Close Session

=== "Rust"
    ```rust
    pub fn close_session(&self, session_id: &AgentSessionId) -> Result<()>
    ```

=== "Python"
    ```python
    def close_session(self, session_id: AgentSessionId) -> None
    ```

=== "JavaScript"
    ```javascript
    closeSession(session_id: WasmAgentSessionId): void
    ```

Closes a session and releases its resources.

### Python API reminders (Jan 2026)

> Several "fixes not deployed" alerts actually came from automation calling the bindings incorrectly. Double-check the following before opening an issue:
>
> 1. `expand(session_id, block_id, direction="down", ...)` — the `block_id` argument is mandatory and must be positional; there is no overload that infers it from the session.
> 2. `context_add(session_id, block_id, relevance=0.8)` — the keyword parameter is `relevance`, not `relevance_score`.
> 3. `SessionConfig(name="...")` — the constructor only accepts `name` and `start_block`. Use the builder helpers (`with_view_mode`, `with_capabilities`, etc.) for any other option.

#### Update Document

=== "Rust"
    ```rust
    pub fn update_document(&self, doc: Document) -> Result<()>
    ```

=== "Python"
    ```python
    def update_document(self, doc: Document) -> None
    ```

=== "JavaScript"
    ```javascript
    updateDocument(doc: Document): void
    ```

Sync document changes to the traversal. Call this after adding blocks to the original document if you want the traversal to see those new blocks.

**Parameters:**
- `doc`: Updated document

**Important:** The traversal creates a snapshot of the document at construction time. Any blocks added to the document after constructing the traversal will not be visible until you call `update_document()`.

**Example Use Case:**
```python
doc = Document.create()
traversal = AgentTraversal(doc)

# Add blocks after creating traversal
block_id = doc.add_block(doc.root_id, "New content")

# Sync changes to traversal
traversal.update_document(doc)

# Now you can navigate to the new block
traversal.navigate_to(session, block_id)
```

## Navigation

### Navigate To

=== "Rust"
    ```rust
    pub fn navigate_to(&self, session_id: &AgentSessionId, target: BlockId)
        -> Result<NavigationResult>
    ```

=== "Python"
    ```python
    def navigate_to(self, session_id: AgentSessionId, block_id: BlockId) -> NavigationResult
    ```

=== "JavaScript"
    ```javascript
    navigateTo(sessionId: WasmAgentSessionId, blockId: string): object
    ```

Move the cursor to a specific block.

**Parameters:**
- `session_id`: Target session
- `target`/`block_id`: Block to navigate to

**Returns:** Navigation result with new position and refresh status

**Errors (Rust only):**
- `BlockNotFound`: Target block doesn't exist

### Go Back

=== "Rust"
    ```rust
    pub fn go_back(&self, session_id: &AgentSessionId, steps: usize)
        -> Result<NavigationResult>
    ```

=== "Python"
    ```python
    def go_back(self, session_id: AgentSessionId, steps: int = 1) -> NavigationResult
    ```

=== "JavaScript"
    ```javascript
    goBack(sessionId: WasmAgentSessionId, steps?: number): object
    ```

Go back in navigation history.

**Parameters:**
- `session_id`: Target session
- `steps`: Number of steps to go back (default: 1)

**Returns:** Navigation result with previous position

**Errors (Rust only):**
- `EmptyHistory`: No navigation history available

## Expansion

### Expand

=== "Rust"
    ```rust
    pub fn expand(&self,
        session_id: &AgentSessionId,
        block_id: BlockId,
        direction: ExpandDirection,
        options: ExpandOptions
    ) -> Result<ExpansionResult>
    ```

=== "Python"
    ```python
    def expand(self,
        session_id: AgentSessionId,
        block_id: BlockId,
        direction: str = "down",  # "down", "up", "both", "semantic"
        depth: int = 3,
        view_mode: ViewMode | None = None
    ) -> ExpansionResult
    ```

=== "JavaScript"
    ```javascript
    expand(sessionId: WasmAgentSessionId,
        blockId: string,
        direction: WasmExpandDirection,
        options?: WasmExpandOptions
    ): object
    ```

Expand the graph in a given direction with specified depth.

**Parameters:**
- `session_id`: Target session
- `block_id`: Block to expand from
- `direction`: Expansion direction (Down, Up, Both, Semantic)
- `options`: Expansion options (depth, view mode, filters)

**Returns:** Expansion result with block tree organized by levels

**Errors (Rust only):**
- `DepthLimitExceeded`: Exceeds session depth limit
- `BlockNotFound`: Source block doesn't exist

## Search

### Search (Semantic)

=== "Rust"
    ```rust
    pub async fn search(&self,
        session_id: &AgentSessionId,
        query: &str,
        options: SearchOptions
    ) -> Result<RagSearchResults>
    ```

=== "Python"
    ```python
    async def search(self,
        session_id: AgentSessionId,
        query: str,
        options: SearchOptions | None = None
    ) -> SearchResult
    ```

=== "JavaScript"
    ```javascript
    search(sessionId: WasmAgentSessionId,
        query: string,
        options?: WasmSearchOptions
    ): Promise<object>
    ```

Perform semantic search via RAG provider.

**Parameters:**
- `session_id`: Target session
- `query`: Search query
- `options`: Search options (limit, min_similarity, filters)

**Returns:** Search results with matching blocks and relevance scores

**Errors:**
- `RagNotConfigured`: No RAG provider set up
- `RagSearchFailed`: Search provider error

### Find (Pattern-based)

=== "Rust"
    ```rust
    pub fn find_by_pattern(&self,
        session_id: &AgentSessionId,
        role: Option<&str>,
        tag: Option<&str>,
        label: Option<&str>,
        pattern: Option<&str>
    ) -> Result<FindResult>
    ```

=== "Python"
    ```python
    def find(self,
        session_id: AgentSessionId,
        role: str | None = None,
        tag: str | None = None,
        tags: list[str] | None = None,
        label: str | None = None,
        pattern: str | None = None
    ) -> FindResult
    ```

=== "JavaScript"
    ```javascript
    findByPattern(sessionId: WasmAgentSessionId,
        role?: string,
        tag?: string,
        tags?: string,
        label?: string,
        pattern?: string
    ): object
    ```

Find blocks by role, tag, label, or content pattern (no RAG required).

**Parameters:**
- `session_id`: Target session
- `role`: Semantic role to match (e.g., "paragraph", "heading1")
- `tag`: Single tag to match (or use `tags` parameter)
- `tags`: Multiple tags to match (Python: list, JavaScript: comma-separated string)
- `label`: Label to match
- `pattern`: Regex pattern for content matching

**Returns:** Find result with matching block IDs

**Note:** Both `tag` (singular) and `tags` (plural) parameters are supported. The `tags` parameter takes precedence if both are provided.

## View

### View Block

=== "Rust"
    ```rust
    pub fn view_block(&self,
        session_id: &AgentSessionId,
        block_id: BlockId,
        mode: ViewMode
    ) -> Result<BlockView>
    ```

=== "Python"
    ```python
    def view_block(self,
        session_id: AgentSessionId,
        block_id: BlockId,
        view_mode: ViewMode | None = None
    ) -> BlockView
    ```

=== "JavaScript"
    ```javascript
    viewBlock(sessionId: WasmAgentSessionId,
        blockId: string,
        viewMode?: string
    ): object
    ```

View a specific block with the given display mode.

**Parameters:**
- `session_id`: Target session
- `block_id`: Block to view
- `mode`: View mode (IdsOnly, Metadata, Preview, Full, Adaptive)

**Returns:** Block view with content and metadata

### View Neighborhood

=== "Rust"
    ```rust
    pub fn view_neighborhood(&self, session_id: &AgentSessionId)
        -> Result<NeighborhoodView>
    ```

=== "Python"
    ```python
    def view_neighborhood(self, session_id: AgentSessionId) -> object
    ```

=== "JavaScript"
    ```javascript
    viewNeighborhood(sessionId: WasmAgentSessionId): object
    ```

View the neighborhood around the current cursor position.

**Returns:** Neighborhood with ancestors, children, siblings, and connections

## Path Finding

### Find Path

=== "Rust"
    ```rust
    pub fn find_path(&self,
        session_id: &AgentSessionId,
        from_id: BlockId,
        to_id: BlockId,
        max_length: Option<usize>
    ) -> Result<Vec<BlockId>>
    ```

=== "Python"
    ```python
    def find_path(self,
        session_id: AgentSessionId,
        from_block: BlockId,
        to_block: BlockId,
        max_length: int | None = None
    ) -> list[BlockId]
    ```

=== "JavaScript"
    ```javascript
    findPath(sessionId: WasmAgentSessionId,
        fromId: string,
        toId: string,
        maxLength?: number
    ): string[]
    ```

Find a path between two blocks.

**Parameters:**
- `session_id`: Target session
- `from_id`/`from_block`: Starting block
- `to_id`/`to_block`: Ending block
- `max_length`: Maximum path length

**Returns:** Ordered list of block IDs forming the path

**Errors (Rust only):**
- `NoPathExists`: No path found between blocks

## Context Operations

> **Important:** The traversal crate does not maintain an internal context
> window. Every `context_*` method validates inputs, updates metrics, and emits
> a structured event (`ExecutionResult::Context` in Rust or the equivalent in
> Python/WASM). Your application is responsible for persisting the block IDs,
> relevance scores, and reasons in its own datastore (vector DB, cache, prompt
> buffer, etc.).

### Add to Context

=== "Rust"
    ```rust
    pub fn context_add(&self,
        session_id: &AgentSessionId,
        block_id: BlockId,
        reason: Option<String>,
        relevance: Option<f32>
    ) -> Result<()>
    ```

=== "Python"
    ```python
    def context_add(self,
        session_id: AgentSessionId,
        block_id: BlockId,
        reason: str | None = None,
        relevance: float | None = None
    ) -> None
    ```

=== "JavaScript"
    ```javascript
    contextAdd(sessionId: WasmAgentSessionId,
        blockId: string,
        reason?: string,
        relevance?: number
    ): void
    ```

Emit an event indicating that a block should be added to the host-managed context store.

**Parameters:**
- `session_id`: Target session
- `block_id`: Block to add
- `reason`: Optional inclusion reason (for tracking)
- `relevance`: Optional relevance score (0.0-1.0)

### Remove from Context

=== "Rust"
    ```rust
    pub fn context_remove(&self,
        session_id: &AgentSessionId,
        block_id: BlockId
    ) -> Result<()>
    ```

=== "Python"
    ```python
    def context_remove(self,
        session_id: AgentSessionId,
        block_id: BlockId
    ) -> None
    ```

=== "JavaScript"
    ```javascript
    contextRemove(sessionId: WasmAgentSessionId, blockId: string): void
    ```

Emit an event indicating that a block should be removed from the host-managed context store.

### Clear Context

=== "Rust"
    ```rust
    pub fn context_clear(&self, session_id: &AgentSessionId) -> Result<()>
    ```

=== "Python"
    ```python
    def context_clear(self, session_id: AgentSessionId) -> None
    ```

=== "JavaScript"
    ```javascript
    contextClear(sessionId: WasmAgentSessionId): void
    ```

Signal that the external context store should clear any tracked blocks for the session.

### Add Results to Context

=== "Rust"
    ```rust
    pub fn context_add_results(&self, session_id: &AgentSessionId)
        -> Result<Vec<BlockId>>
    ```

=== "Python"
    ```python
    def context_add_results(self, session_id: AgentSessionId) -> list[BlockId]
    ```

=== "JavaScript"
    ```javascript
    contextAddResults(sessionId: WasmAgentSessionId): string[]
    ```

Emit the list of block IDs from the last search/find so the host can add them in bulk.

**Returns:** List of block IDs added

**Errors (Rust only):**
- `NoResultsAvailable`: No search/find results available

### Set Focus Block

=== "Rust"
    ```rust
    pub fn context_focus(&self,
        session_id: &AgentSessionId,
        block_id: Option<BlockId>
    ) -> Result<()>
    ```

=== "Python"
    ```python
    def context_focus(self,
        session_id: AgentSessionId,
        block_id: BlockId | None = None
    ) -> None
    ```

=== "JavaScript"
    ```javascript
    contextFocus(sessionId: WasmAgentSessionId, blockId?: string): void
    ```

Update the focus block metadata in the host-managed context. Pass `None`/`null` to clear.

## UCL Execution

### Execute UCL

=== "Rust"
    ```rust
    pub async fn execute_ucl(
        traversal: &AgentTraversal,
        session_id: &AgentSessionId,
        ucl_input: &str
    ) -> Result<Vec<ExecutionResult>>
    ```

=== "Python"
    ```python
    async def execute_ucl(self,
        session_id: AgentSessionId,
        ucl_input: str
    ) -> list[str]
    ```

=== "JavaScript"
    ```javascript
    executeUcl(sessionId: WasmAgentSessionId, uclInput: string): Promise<string[]>
    ```

Execute UCL commands from a string. Multiple commands separated by newlines.

**Parameters:**
- `traversal`/`self`: Traversal instance
- `session_id`: Target session
- `ucl_input`: UCL command(s)

**Returns:** Execution results as JSON strings

## Configuration Types

### SessionConfig

Configuration for creating new sessions.

=== "Rust"
    ```rust
    pub struct SessionConfig {
        pub name: Option<String>,
        pub start_block: Option<BlockId>,
        pub limits: SessionLimits,
        pub capabilities: AgentCapabilities,
        pub view_mode: ViewMode,
    }

    impl SessionConfig {
        pub fn new() -> Self
        pub fn with_name(self, name: &str) -> Self
        pub fn with_start_block(self, block: BlockId) -> Self
        pub fn with_limits(self, limits: SessionLimits) -> Self
        pub fn with_capabilities(self, caps: AgentCapabilities) -> Self
        pub fn with_view_mode(self, mode: ViewMode) -> Self
    }
    ```

=== "Python"
    ```python
    class SessionConfig:
        def __init__(self,
            name: str | None = None,
            start_block: BlockId | None = None
        ) -> None

        def with_name(self, name: str) -> SessionConfig
        def with_view_mode(self, mode: ViewMode) -> SessionConfig
        def with_capabilities(self, caps: AgentCapabilities) -> SessionConfig
    ```

=== "JavaScript"
    ```javascript
    class WasmSessionConfig {
        constructor()

        withName(name: string): WasmSessionConfig
        withStartBlock(blockId: string): WasmSessionConfig
        withLimits(limits: WasmSessionLimits): WasmSessionConfig
        withCapabilities(caps: WasmAgentCapabilities): WasmSessionConfig

        // View mode builders
        withViewModeIds(): WasmSessionConfig
        withViewModePreview(length: number): WasmSessionConfig
        withViewModeFull(): WasmSessionConfig
        withViewModeMetadata(): WasmSessionConfig
    }
    ```

### AgentCapabilities

Defines what operations a session is allowed to perform.

=== "Rust"
    ```rust
    pub struct AgentCapabilities {
        pub can_traverse: bool,
        pub can_search: bool,
        pub can_modify_context: bool,
        pub can_coordinate: bool,
        pub allowed_edge_types: HashSet<EdgeType>,
        pub max_expand_depth: usize,
    }

    impl AgentCapabilities {
        pub fn full() -> Self  // All permissions
        pub fn read_only() -> Self  // Traverse & search only
    }
    ```

=== "Python"
    ```python
    class AgentCapabilities:
        can_traverse: bool
        can_search: bool
        can_modify_context: bool
        can_coordinate: bool
        max_expand_depth: int

        @staticmethod
        def full() -> AgentCapabilities

        @staticmethod
        def read_only() -> AgentCapabilities
    ```

=== "JavaScript"
    ```javascript
    class WasmAgentCapabilities {
        static full(): WasmAgentCapabilities
        static readOnly(): WasmAgentCapabilities

        get canTraverse(): boolean
        get canSearch(): boolean
        get canModifyContext(): boolean
        get canCoordinate(): boolean
        get maxExpandDepth(): number
    }
    ```
