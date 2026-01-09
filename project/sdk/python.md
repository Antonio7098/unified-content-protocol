# Python SDK

The Python SDK (`ucp-content`) mirrors the JavaScript API for backend workflows, agents, and research pipelines.

## Installation

```bash
pip install ucp-content
```

## Core Imports

```python
import ucp
from ucp import PromptBuilder, IdMapper, UclBuilder
```

## Creating Documents

```python
doc = ucp.parse("""# Title

Paragraph text

## Section

More text
""")
markdown = ucp.render(doc)
empty = ucp.create()
```

## Prompt Builder

```python
prompt = (
    ucp.prompt()
    .edit()
    .append()
    .move()
    .with_short_ids()
    .with_rule("Keep responses under 50 tokens")
    .build()
)
```

## ID Mapper

```python
mapper = ucp.map_ids(doc)
summary = mapper.describe(doc)
shortened = mapper.shorten('EDIT blk_00000000000c SET text = "Hi"')
expanded = mapper.expand('EDIT 12 SET text = "Hi"')
```

## UCL Builder

```python
commands = (
    ucp.ucl()
    .edit(4, 'Updated intro')
    .append(2, 'New paragraph')
    .delete(7, cascade=True)
    .atomic()
    .build()
)
```

## Type Hints

```python
from ucp import Document, Block, ContentType, SemanticRole, Capability
```

## Testing

```bash
PYTHONPATH=src python -m pytest tests/ -v
```
