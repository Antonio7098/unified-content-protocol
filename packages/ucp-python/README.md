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

## License

MIT
