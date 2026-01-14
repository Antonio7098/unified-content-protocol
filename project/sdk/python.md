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

The `UclBuilder` provides programmatic construction of UCL commands.

### edit() Method

```python
commands = (
    ucp.ucl()
    .edit(block_id, content, path="text")  # path defaults to "text"
    .build()
)
```

**Parameters:**
- `block_id` (str): The block ID to edit
- `content` (str): The new content value  
- `path` (str, optional): Property path (default: `"text"`)

**Note:** The `edit()` method does not support `label` as a parameter. To set a label, use APPEND with the `label` property or edit the block directly via `doc.blocks[id].metadata.label`.

### Other Commands

```python
commands = (
    ucp.ucl()
    .edit("blk_1", "Updated intro")
    .append("blk_2", "New paragraph", content_type="text", label="intro")
    .delete("blk_7", cascade=True)
    .move_to("blk_3", "blk_4")
    .link("blk_5", "references", "blk_6")
    .atomic()
    .build()
)
```

## Type Hints

```python
from ucp import Document, Block, ContentType, SemanticRole, Capability
```

## Markdown Parser Limitations

The markdown parser focuses on structural elements:

- **Tables**: Not parsed into structured table content. Tables appear as paragraph text.
- **Inline formatting**: Bold, italic, and inline code are preserved as raw text (not parsed into separate elements). This preserves round-trip fidelity.
- **List markers**: Both ordered (`1. item`) and unordered (`- item`) lists are supported with marker preservation.

## Testing

```bash
PYTHONPATH=src python -m pytest tests/ -v
```
