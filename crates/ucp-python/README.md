# UCP - Unified Content Protocol

Python bindings for the Rust UCP implementation.

## Installation

```bash
pip install ucp-content
```

## Usage

```python
import ucp

# Create a document
doc = ucp.create("My Document")

# Add blocks
root = doc.root_id
block1 = doc.add_block(root, "Hello, World!", role="paragraph")

# Edit blocks
doc.edit_block(block1, "Updated content")

# Render to markdown
md = ucp.render(doc)
print(md)
```

## Features

- **Document Operations**: Create, edit, move, delete blocks
- **Traversal**: Children, parent, ancestors, descendants, siblings
- **Finding**: By tag, label, role, content type
- **Edges**: Create relationships between blocks
- **LLM Utilities**: IdMapper for token-efficient prompts, PromptBuilder for UCL generation
- **Snapshots**: Version control for documents
- **UCL Execution**: Execute UCL commands on documents

## API Reference

See the [documentation](https://github.com/your-org/ucp) for full API reference.
