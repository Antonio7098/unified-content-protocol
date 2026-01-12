# ucp-content

Unified Content Protocol SDK for Python.

Build LLM-powered content manipulation with minimal code.

## Installation

```bash
pip install ucp-content
```

## Quick Start

```python
import ucp

# 1. Parse markdown into a document
doc = ucp.parse("""
# My Article

This is the introduction.

## Section 1

Some content here.
""")

# 2. Create an ID mapper for token efficiency
mapper = ucp.map_ids(doc)

# 3. Get a compact document description for the LLM
description = mapper.describe(doc)
# Output:
# Document Structure:
#   [2] heading1 - My Article
#     [3] paragraph - This is the introduction.
#     [4] heading2 - Section 1
#       [5] paragraph - Some content here.

# 4. Build a prompt with only the capabilities you need
system_prompt = (ucp.prompt()
    .edit()
    .append()
    .with_short_ids()
    .build())

# 5. After LLM responds, expand short IDs back to full IDs
llm_response = 'EDIT 3 SET text = "Updated intro"'
expanded_ucl = mapper.expand(llm_response)
# Result: 'EDIT blk_000000000003 SET text = "Updated intro"'
```

## API Reference

### Document Operations

```python
# Parse markdown
doc = ucp.parse('# Hello\n\nWorld')

# Render back to markdown
md = ucp.render(doc)

# Create empty document
doc = ucp.create()
```

### Prompt Builder

Build prompts with only the capabilities your agent needs:

```python
prompt = (ucp.prompt()
    .edit()           # Enable EDIT command
    .append()         # Enable APPEND command
    .move()           # Enable MOVE command
    .delete()         # Enable DELETE command
    .link()           # Enable LINK/UNLINK commands
    .snapshot()       # Enable SNAPSHOT commands
    .transaction()    # Enable ATOMIC transactions
    .all()            # Enable all capabilities
    .with_short_ids() # Use short numeric IDs
    .with_rule('Keep responses concise')
    .build())
```

### ID Mapper

Save tokens by using short numeric IDs:

```python
mapper = ucp.map_ids(doc)

# Shorten IDs in any text
short = mapper.shorten('Block blk_000000000003 has content')
# Result: 'Block 3 has content'

# Expand IDs in UCL commands
expanded = mapper.expand('EDIT 3 SET text = "hello"')
# Result: 'EDIT blk_000000000003 SET text = "hello"'

# Get document description with short IDs
desc = mapper.describe(doc)
```

### UCL Builder

Build UCL commands programmatically:

```python
commands = (ucp.ucl()
    .edit(3, 'Updated content')
    .append(2, 'New paragraph')
    .delete(5)
    .atomic()  # Wrap in ATOMIC block
    .build())
```

## Token Efficiency

Using short IDs can significantly reduce token usage:

| ID Format | Example | Tokens |
|-----------|---------|--------|
| Long | `blk_000000000003` | ~6 |
| Short | `3` | 1 |

For a document with 50 blocks referenced 3 times each, this saves ~750 tokens.

## Type Hints

Full type hint support:

```python
from ucp import Document, Block, ContentType, SemanticRole, Capability
```

## Deterministic Block IDs

Block identifiers now follow the canonical `blk_XXXXXXXXXXXX` pattern (root is
always `blk_000000000000`). IDs are computed as:

1. Normalize content to NFC.
2. SHA-256 hash `content || semantic_role || namespace`.
3. Take the first 12 hex characters and prefix with `blk_`.

This matches the reference implementation, ensuring Python documents can round
trip with Rust/TypeScript.

```python
from ucp import Block, SemanticRole

heading = Block.new("Overview", role=SemanticRole.HEADING2)
print(heading.id)  # e.g. blk_a1c3b8f1d2e4
```

## Error Handling

The SDK raises descriptive exceptions for invalid operations:

```python
import ucp
from ucp import Document

doc = ucp.create()

# Block not found
try:
    doc.delete_block("blk_nonexistent")
except KeyError as e:
    print(e)  # "Block not found: blk_nonexistent"

# Cannot delete root
try:
    doc.delete_block(doc.root_id)
except ValueError as e:
    print(e)  # "Cannot delete the root block"

# Cannot move into self
try:
    block_id = doc.add_block(doc.root_id, "Test")
    doc.move_block(block_id, block_id)
except ValueError as e:
    print(e)  # "Cannot move a block into itself or its descendants"
```

### Validation

```python
result = doc.validate()

if not result.valid:
    for issue in result.issues:
        print(f"[{issue.severity}] {issue.code}: {issue.message}")
        # [error] E201: Document structure contains a cycle
        # [warning] E203: Block blk_123 is unreachable from root
```

### Error Codes

| Code | Severity | Description |
|------|----------|-------------|
| E001 | Error | Block not found |
| E201 | Error | Cycle detected in document |
| E203 | Warning | Orphaned/unreachable block |
| E400 | Error | Block count limit exceeded |
| E402 | Error | Block size limit exceeded |
| E403 | Error | Nesting depth limit exceeded |
| E404 | Error | Edge count limit exceeded |

## Observability

The SDK includes built-in observability features:

```python
import ucp
from ucp import get_logger, on_event, trace, record_metric

# Logging
logger = get_logger()
logger.info("Starting document processing")

# Event subscription
@on_event("block_added")
def handle_block_added(event):
    print(f"Block {event.block_id} added")

# Tracing
with trace("parse_document") as span:
    doc = ucp.parse(markdown_content)
    span.set_attribute("block_count", len(doc.blocks))

# Metrics
record_metric("documents_parsed", 1)
```

## Snapshots and Transactions

```python
import ucp
from ucp import transaction

doc = ucp.create()

# Create a snapshot
snapshot_mgr = ucp.SnapshotManager()
snapshot_mgr.create("before_changes", doc)

# Use transactions for atomic operations
with transaction(doc) as txn:
    doc.add_block(doc.root_id, "Block 1")
    doc.add_block(doc.root_id, "Block 2")
    # Commits automatically on success
    # Rolls back on exception

# Restore from snapshot
restored_doc = snapshot_mgr.restore("before_changes")
```

## Async Support

For async applications:

```python
import asyncio
import ucp

async def process_documents():
    # Most operations are synchronous but can be wrapped
    doc = await asyncio.to_thread(ucp.parse, large_markdown)
    return doc
```

## Conformance

This SDK implements the UCP specification. See `docs/conformance/README.md` for the full specification and test vectors.

Run the Python conformance suite with:

```bash
PYTHONPATH=src python3 -m pytest tests/conformance/test_conformance.py
```

All 26 reference tests pass against the current SDK.

## UCL Execution Summary

`execute_ucl` now returns an `ExecutionSummary` that exposes the aggregated status
and affected blocks:

```python
import ucp

doc = ucp.create()
summary = ucp.execute_ucl(doc, 'APPEND blk_000000000000 text :: "Hello"')

if summary.success:
    print("Blocks touched:", summary.affected_blocks)
```

Individual `ExecutionResult` objects remain available via `summary.results`.

## Development

```bash
# Install dev dependencies
pip install -e ".[dev]"

# Run tests
pytest

# Run conformance tests
pytest tests/conformance/
```

## License

MIT
