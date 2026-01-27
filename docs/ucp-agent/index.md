# Agent Graph Traversal System

The Agent Graph Traversal System (`ucp-agent`) enables AI agents to intelligently navigate knowledge graphs, search content semantically, manage context windows, and coordinate with other agents. It provides both a powerful Rust API and language bindings for Python and JavaScript.

## Key Features

- **Graph Navigation**: GOTO, BACK, EXPAND, FOLLOW, and PATH commands for flexible graph traversal
- **Semantic Search**: RAG-powered semantic search with pluggable provider interface
- **Context Hooks**: CTX operations emit structured events for external context managers
- **Multi-agent Support**: Session isolation and coordination via shared discovery pool
- **Safety Mechanisms**: Depth limits, circuit breakers, operation budgets, and timeout protection
- **View Modes**: IdsOnly, Metadata, Preview, Full, and Adaptive display modes
- **Metrics & Observability**: Built-in session metrics tracking for monitoring

## Quick Start

=== "Rust"
    ```rust
    use ucp_agent::{AgentTraversal, SessionConfig};
    use ucm_core::Document;

    let doc = Document::create();
    let traversal = AgentTraversal::new(doc);

    // Create a session
    let session_id = traversal.create_session(SessionConfig::default())?;

    // Navigate to root
    let result = traversal.navigate_to(&session_id, doc.root)?;
    println!("At: {}", result.position);

    // Expand children
    let expansion = traversal.expand(
        &session_id,
        doc.root,
        ExpandDirection::Down,
        ExpandOptions::new().with_depth(2),
    )?;
    println!("Found {} blocks", expansion.total_blocks);

    // Add to context
    traversal.context_add(&session_id, doc.root, None, Some(0.9))?;

    // Clean up
    traversal.close_session(&session_id)?;
    ```

=== "Python"
    ```python
    from ucp import Document, AgentTraversal, SessionConfig, ExpandDirection

    doc = Document.create()
    traversal = AgentTraversal(doc)

    # Create a session
    session_id = traversal.create_session(SessionConfig())

    # Navigate to root
    result = traversal.navigate_to(session_id, doc.root)
    print(f"At: {result.position}")

    # Expand children
    expansion = traversal.expand(
        session_id,
        doc.root,
        ExpandDirection.Down,
        depth=2
    )
    print(f"Found {expansion.total_blocks} blocks")

    # Add to context
    traversal.context_add(session_id, doc.root, relevance=0.9)

    # Clean up
    traversal.close_session(session_id)
    ```

=== "JavaScript"
    ```javascript
    const { Document, AgentTraversal, SessionConfig, ExpandDirection } = require('ucp');

    const doc = Document.create();
    const traversal = new AgentTraversal(doc);

    // Create a session
    const sessionId = traversal.createSession(new SessionConfig());

    // Navigate to root
    const result = traversal.navigateTo(sessionId, doc.root);
    console.log(`At: ${result.position}`);

    // Expand children
    const expansion = traversal.expand(
        sessionId,
        doc.root,
        ExpandDirection.Down,
        null,
        new ExpandOptions().withDepth(2)
    );
    console.log(`Found ${expansion.totalBlocks} blocks`);

    // Add to context
    traversal.contextAdd(sessionId, doc.root, null, 0.9);

    // Clean up
    traversal.closeSession(sessionId);
    ```

## Core Concepts

### Sessions
Each agent operates within an isolated session. Sessions track:
- Current position (cursor) in the graph
- Navigation history (for BACK operations)
- Context window (blocks added for LLM processing)
- Metrics and statistics
- Capabilities and limits

### Cursor & Neighborhood
The cursor represents the agent's current position. The neighborhood provides fast access to adjacent blocks:
- Ancestors (parent chain)
- Children (direct descendants)
- Siblings (same parent)
- Connections (semantic edges)

### Context Window
The traversal crate does **not** maintain an in-memory context store. Instead, each
context command (CTX ADD, CTX REMOVE, etc.) validates inputs, updates metrics, and
emits structured events so that an embedding store, RAG cache, or host
application can maintain the actual window. A typical integration:

1. Subscribe to CTX events via the executor or API bindings.
2. Persist the block IDs, scores, and reasons inside your own datastore.
3. Apply pruning, token budgets, or streaming to the downstream LLM.

This division keeps the traversal core lightweight while still providing a
consistent interface for higher-level context managers.

### View Modes
Control how block content is displayed:
- **IdsOnly**: Just block identifiers (minimal overhead)
- **Metadata**: IDs + roles, tags, edge counts
- **Preview**: First N characters of content
- **Full**: Complete block content
- **Adaptive**: Auto-selects based on relevance score

## Architecture

The system is organized into several key modules:

```
ucp-agent/
├── session.rs        # Session and capability management
├── cursor.rs         # Traversal cursor and neighborhood
├── operations.rs     # Main traversal API (AgentTraversal)
├── executor.rs       # UCL command execution
├── expansion.rs      # Expansion strategies and view modes
├── rag.rs            # RAG provider trait and implementations
├── safety.rs         # Limits, budgets, circuit breakers
├── metrics.rs        # Session metrics and observability
└── error.rs          # Error types and session IDs
```

## Next Steps

- **[API Reference](api.md)** - Complete method documentation
- **[UCL Commands](ucl-commands.md)** - Traversal and context commands
- **[Examples](examples.md)** - Common usage patterns
- **[Architecture](architecture.md)** - System design details
