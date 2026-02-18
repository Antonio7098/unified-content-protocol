# UCP Agent

**ucp-agent** provides an agent graph traversal system for UCP knowledge graphs.

## Overview

This crate provides powerful, flexible graph traversal capabilities enabling AI agents to navigate knowledge graphs, search semantically, manage context windows, and coordinate parallel exploration.

## Features

- **UCL Commands**: Traversal commands (GOTO, EXPAND, FOLLOW, etc.) and context commands (CTX ADD, CTX REMOVE, etc.)
- **Session Management**: Track agent position, history, and state across operations
- **RAG Integration**: Pluggable semantic search providers for knowledge retrieval
- **Safety Mechanisms**: Limits, circuit breakers, and depth guards for robust operation
- **Observability**: Comprehensive metrics and telemetry for monitoring

## Installation

```toml
[dependencies]
ucp-agent = "0.1"
```

## Quick Start

```rust
use ucp_agent::{AgentSession, SessionConfig};
use ucm_core::Document;

// Create a document
let doc = Document::create();

// Create an agent session
let session = AgentSession::new(doc, SessionConfig::default());

// Navigate the graph
session.goto(block_id)?;
session.expand(ExpandDirection::Down, 3)?;

// Manage context
session.add_to_context(block_id, InclusionReason::DirectReference);
```

## UCL Commands

The agent system responds to UCL (Unified Content Language) commands:

```
GOTO blk_abc123           # Navigate to a block
EXPAND blk_abc DOWN 3     # Expand children up to depth 3
FOLLOW references         # Follow edges of a specific type
CTX ADD blk_xyz           # Add block to context window
CTX RENDER format=SHORT   # Render context for LLM
```

## See Also

- [UCM Core](../ucm-core) - Core types and traits
- [UCL Parser](../ucl-parser) - UCL parsing and execution
- [UCP LLM](../ucp-llm) - LLM integration utilities
