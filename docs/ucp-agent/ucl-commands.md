# UCL Traversal & Context Commands

## Overview

The Agent Traversal System extends UCL with new commands for graph navigation and context management. Commands are case-insensitive but typically written in uppercase.

### Troubleshooting note (Jan 2026)

> Earlier automated reports claiming "fixes not deployed" were traced back to the test harness issuing invalid UCL, not a runtime regression. Double-check the following syntax rules before filing a bug:
>
> 1. `EXPAND blk_target DOWN …` — the block ID must come first, followed by the direction keyword. `EXPAND DOWN blk_target` will be rejected by the parser.
> 2. `PATH blk_start TO blk_end` — always include the `TO` keyword between the two block IDs.
> 3. `FIND PATTERN=".*"` — every filter (`ROLE`, `TAG`, `LABEL`, `PATTERN`) uses `=` between the key and value.
> 4. `CTX ADD blk_id RELEVANCE=0.8` — contextual metadata such as `RELEVANCE` and `REASON` also require `=`.

These constraints already existed in the engine; the improved errors below simply call them out explicitly.

## Traversal Commands

### GOTO

Navigate cursor to a specific block.

```
GOTO <block_id>
```

**Parameters:**
- `block_id`: Target block ID (e.g., `blk_abc123`)

**Example:**
```
GOTO blk_introduction
```

### BACK

Go back in navigation history.

```
BACK [steps]
```

**Parameters:**
- `steps`: Number of steps to go back (default: 1)

**Examples:**
```
BACK           # Go back 1 step
BACK 3         # Go back 3 steps
```

### EXPAND

Expand the graph from a block in a given direction.

```
EXPAND <block_id> <direction> [DEPTH=N] [MODE=mode] [ROLES=r1,r2,...] [TAGS=t1,t2,...]
```

**Parameters:**
- `block_id`: Block to expand from
- `direction`: DOWN, UP, BOTH, or SEMANTIC
- `DEPTH`: Maximum depth (default: 3)
- `MODE`: View mode (FULL, PREVIEW, METADATA, IDS, ADAPTIVE)
- `ROLES`: Comma-separated roles to include (optional filter)
- `TAGS`: Comma-separated tags to include (optional filter)

**Examples:**
```
EXPAND blk_intro DOWN DEPTH=2
EXPAND blk_intro UP DEPTH=1
EXPAND blk_intro SEMANTIC DEPTH=2
EXPAND blk_intro DOWN DEPTH=3 MODE=PREVIEW
EXPAND blk_intro DOWN DEPTH=2 ROLES=heading1,heading2
```

### FOLLOW

Follow edges from a block to connected blocks.

```
FOLLOW <block_id> <edge_types> [TO <target_id>]
```

**Parameters:**
- `block_id`: Source block
- `edge_types`: Edge type(s) to follow (e.g., REFERENCES, ELABORATES)
- `target_id`: Specific target (optional)

**Examples:**
```
FOLLOW blk_auth REFERENCES
FOLLOW blk_login SEMANTIC_RELATED
```

### PATH

Find a path between two blocks.

```
PATH <from_id> TO <to_id> [MAX=N]
```

**Parameters:**
- `from_id`: Starting block
- `to_id`: Ending block
- `MAX`: Maximum path length (optional)

**Examples:**
```
PATH blk_start TO blk_end
PATH blk_intro TO blk_conclusion MAX=5
```

### SEARCH

Perform semantic search (requires RAG provider).

```
SEARCH "<query>" [LIMIT=N] [MIN_SIMILARITY=score] [ROLES=r1,r2] [TAGS=t1,t2]
```

**Parameters:**
- `query`: Search query (must be quoted)
- `LIMIT`: Max results (default: 10)
- `MIN_SIMILARITY`: Minimum similarity score 0.0-1.0 (default: 0.0)
- `ROLES`: Filter by semantic roles
- `TAGS`: Filter by tags

**Examples:**
```
SEARCH "how to authenticate users"
SEARCH "encryption" LIMIT=5 MIN_SIMILARITY=0.7
SEARCH "database" ROLES=code LIMIT=10
```

### FIND

Find blocks by pattern (no RAG required).

```
FIND [ROLE=role] [TAG=tag] [LABEL=label] [PATTERN=regex]
```

**Parameters:**
- `ROLE`: Semantic role to match (e.g., heading1, paragraph, code)
- `TAG`: Tag to match
- `LABEL`: Label to match
- `PATTERN`: Regex pattern for content

**Examples:**
```
FIND ROLE=paragraph
FIND TAG=important
FIND ROLE=code TAG=python
FIND PATTERN="TODO.*"
```

### VIEW

View a block or neighborhood with specific display mode.

```
VIEW <target> [MODE=mode] [DEPTH=N]
VIEW NEIGHBORHOOD [DEPTH=N]
```

**Parameters:**
- `target`: Block ID or NEIGHBORHOOD
- `MODE`: FULL, PREVIEW, METADATA, IDS (default: FULL)
- `DEPTH`: Depth for neighborhood (default: 1)

**Examples:**
```
VIEW blk_section
VIEW blk_intro MODE=PREVIEW
VIEW NEIGHBORHOOD
VIEW NEIGHBORHOOD DEPTH=2
```

## Context Commands

> **Note:** The traversal crate does not maintain a context window. Every `CTX`
> command validates inputs, updates metrics, and emits a structured event so
> that a host application (vector store, cache, prompt builder, etc.) can manage
> the actual context state. Some helpers (such as `CTX ADD RESULTS`) return
> explicit block IDs, while others (e.g., `CTX ADD CHILDREN`) only report counts,
> so the host should fetch the corresponding IDs via regular API calls before
> persisting them.

### CTX ADD

Emit an event instructing the host-managed context store to include additional
blocks.

```
CTX ADD <target> [REASON=reason] [RELEVANCE=score]
CTX ADD RESULTS
CTX ADD CHILDREN <parent_id>
CTX ADD PATH <from_id> TO <to_id>
```

**Parameters:**
- `target`: Block ID
- `RESULTS`: Add all blocks from last SEARCH/FIND
- `CHILDREN`: Add all children of a block
- `PATH`: Add all blocks in path between two blocks
- `REASON`: Inclusion reason (for tracking)
- `RELEVANCE`: Relevance score 0.0-1.0

**Examples:**
```
CTX ADD blk_intro
CTX ADD blk_auth REASON=semantic_relevance RELEVANCE=0.9
CTX ADD RESULTS
CTX ADD CHILDREN blk_root
CTX ADD PATH blk_intro TO blk_conclusion
```

### CTX REMOVE

Emit an event indicating that a block should be removed from the external
context store.

```
CTX REMOVE <block_id>
```

**Examples:**
```
CTX REMOVE blk_irrelevant
```

### CTX CLEAR

Signal that the host-managed context should be cleared for the session.

```
CTX CLEAR
```

### CTX EXPAND

Request that the host fetch additional blocks (via its own expand/find logic)
from the focus block in the specified direction and consider them for context.

```
CTX EXPAND <direction> [DEPTH=N] [TOKENS=N]
```

**Parameters:**
- `direction`: DOWN, UP, SEMANTIC
- `DEPTH`: Expansion depth
- `TOKENS`: Token budget for expansion

**Examples:**
```
CTX EXPAND DOWN DEPTH=2
CTX EXPAND SEMANTIC DEPTH=1
CTX EXPAND DOWN TOKENS=2000
```

### CTX COMPRESS

Inform the host to apply a compression strategy (truncate, summarize, etc.) to
its stored context representation.

```
CTX COMPRESS METHOD=method
```

**Parameters:**
- `method`: TRUNCATE, SUMMARIZE, STRUCTURE_ONLY

**Examples:**
```
CTX COMPRESS METHOD=TRUNCATE
CTX COMPRESS METHOD=STRUCTURE_ONLY
```

### CTX PRUNE

Ask the host context manager to drop blocks matching the given criteria.

```
CTX PRUNE [MIN_RELEVANCE=score] [MAX_AGE=seconds]
```

**Parameters:**
- `MIN_RELEVANCE`: Remove blocks below this threshold
- `MAX_AGE`: Remove blocks not accessed in this many seconds

**Examples:**
```
CTX PRUNE MIN_RELEVANCE=0.3
CTX PRUNE MAX_AGE=300
```

### CTX RENDER

Request that the host produce a rendered view (markdown, short IDs, etc.) of
its stored context.

```
CTX RENDER [FORMAT=format]
```

**Parameters:**
- `FORMAT`: DEFAULT, SHORT_IDS, MARKDOWN

**Examples:**
```
CTX RENDER
CTX RENDER FORMAT=MARKDOWN
CTX RENDER FORMAT=SHORT_IDS
```

### CTX STATS

Return host-managed context metrics (counts, tokens, average relevance, etc.).

```
CTX STATS
```

Returns information about current context:
- Total blocks
- Total tokens
- Average relevance
- Inclusion reasons breakdown

### CTX FOCUS

Set or clear focus block (protected from pruning).

```
CTX FOCUS <block_id>
CTX FOCUS CLEAR
```

**Examples:**
```
CTX FOCUS blk_main_topic
CTX FOCUS CLEAR
```

## Multi-Command Execution

Commands can be executed together by separating with newlines:

=== "Rust"
    ```rust
    let ucl = r#"
    GOTO blk_root
    EXPAND blk_root DOWN DEPTH=2
    FIND ROLE=paragraph
    CTX ADD RESULTS
    CTX EXPAND DOWN DEPTH=1
    CTX RENDER FORMAT=MARKDOWN
    "#;

    let results = ucp_agent::execute_ucl(&traversal, &session, ucl).await?;
    ```

=== "Python"
    ```python
    ucl = """
    GOTO blk_root
    EXPAND blk_root DOWN DEPTH=2
    FIND ROLE=paragraph
    CTX ADD RESULTS
    CTX EXPAND DOWN DEPTH=1
    CTX RENDER FORMAT=MARKDOWN
    """

    results = asyncio.run(traversal.execute_ucl(session, ucl))
    ```

=== "JavaScript"
    ```javascript
    const ucl = `
    GOTO blk_root
    EXPAND blk_root DOWN DEPTH=2
    FIND ROLE=paragraph
    CTX ADD RESULTS
    CTX EXPAND DOWN DEPTH=1
    CTX RENDER FORMAT=MARKDOWN
    `;

    const results = await traversal.executeUcl(session, ucl);
    ```

## Common Patterns

### Explore and Contextualize

```
SEARCH "key topic"
CTX ADD RESULTS
CTX EXPAND DOWN DEPTH=2
CTX RENDER FORMAT=MARKDOWN
```

### Navigate by Structure

```
GOTO blk_root
EXPAND blk_root DOWN DEPTH=3 MODE=PREVIEW
FIND ROLE=heading1
CTX ADD RESULTS
```

### Semantic Exploration

```
EXPAND blk_current SEMANTIC DEPTH=2
VIEW NEIGHBORHOOD
CTX FOCUS blk_current
```

### Deep Investigation

```
GOTO blk_topic
EXPAND blk_topic DOWN DEPTH=2
FIND ROLE=paragraph
CTX ADD RESULTS
PATH blk_topic TO blk_conclusion
CTX ADD PATH blk_topic TO blk_conclusion
CTX STATS
```

## Error Handling

Commands may fail with errors that vary by language:

=== "Rust"
    ```rust
    match ucp_agent::execute_ucl(&traversal, &session, ucl).await {
        Ok(results) => { /* success */ },
        Err(AgentError::BlockNotFound(id)) => { /* block doesn't exist */ },
        Err(AgentError::DepthLimitExceeded { .. }) => { /* depth too deep */ },
        Err(AgentError::ParseError(msg)) => { /* UCL syntax error */ },
        Err(e) => { /* other error */ },
    }
    ```

=== "Python"
    ```python
    try:
        results = asyncio.run(traversal.execute_ucl(session, ucl))
    except RuntimeError as e:
        if "BlockNotFound" in str(e):
            # Handle missing block
        elif "DepthLimitExceeded" in str(e):
            # Handle depth limit
        elif "ParseError" in str(e):
            # Handle syntax error
    ```

=== "JavaScript"
    ```javascript
    try {
        const results = await traversal.executeUcl(session, ucl);
    } catch (error) {
        if (error.includes("BlockNotFound")) {
            // Handle missing block
        } else if (error.includes("DepthLimitExceeded")) {
            // Handle depth limit
        } else if (error.includes("ParseError")) {
            // Handle syntax error
        }
    }
    ```
