# UCP CLI

Command-line interface for the Unified Content Protocol (UCP).

## Overview

The `ucp` CLI provides comprehensive access to UCP functionality including document management, block operations, UCL execution, and agent traversal. It is ideal for automation, CI pipelines, regression testing, and rapid document inspection without writing host-language code.

## Installation

```bash
cargo install ucp-cli
```

## Usage

```bash
# Get help
ucp --help

# Create a new document
ucp create --title "My Document" --output doc.json

# Add a block
ucp block add --input doc.json --output doc.json --parent blk_root --content-type text --content "Hello World"

# View document structure
ucp tree --input doc.json

# Validate document
ucp validate --input doc.json
```

## Global Flags

- `-v, --verbose` - Enable debug-level logging
- `--trace` - Enable trace-level logging
- `-f, --format <text|json>` - Output format (default: text)

## Command Categories

### Document Management
- `create` - Create a new UCP document
- `info` - Display document information
- `validate` - Validate a document

### Block Operations
- `block add` - Add a new block
- `block get` - Get block by ID
- `block delete` - Delete a block
- `block move` - Move a block
- `block list` - List all blocks
- `block update` - Update block content

### Edge Operations
- `edge add` - Add an edge between blocks
- `edge remove` - Remove an edge
- `edge list` - List edges for a block

### Navigation
- `nav children` - Show child blocks
- `nav parent` - Show parent block
- `nav siblings` - Show sibling blocks
- `nav descendants` - Show all descendants

### Search & Structure
- `find` - Find blocks matching criteria
- `orphans` - Find orphaned blocks
- `tree` - Display document hierarchy
- `prune` - Prune orphaned or tagged blocks

### Transactions
- `tx begin` - Begin a transaction
- `tx commit` - Commit a transaction
- `tx rollback` - Rollback a transaction
- `tx savepoint` - Create a savepoint

### Snapshots
- `snapshot create` - Create a snapshot
- `snapshot restore` - Restore from snapshot
- `snapshot list` - List snapshots
- `snapshot delete` - Delete a snapshot
- `snapshot diff` - Compare two snapshots

### Import/Export
- `import markdown` - Import from Markdown
- `import html` - Import from HTML
- `export markdown` - Export to Markdown
- `export json` - Export to JSON

### UCL (Unified Content Language)
- `ucl exec` - Execute UCL commands
- `ucl parse` - Parse and validate UCL

### Agent
- `agent session create` - Create agent session
- `agent session list` - List sessions
- `agent session close` - Close session
- `agent goto` - Navigate to block
- `agent back` - Go back in history
- `agent expand` - Expand context
- `agent follow` - Follow edge type
- `agent search` - Search blocks
- `agent context add` - Add to context
- `agent view` - View current position

### LLM
- `llm id-map` - Create ID mapping
- `llm shorten-ucl` - Convert to short IDs
- `llm expand-ucl` - Expand short IDs
- `llm prompt` - Generate prompt docs
- `llm context` - Manage context window

## License

MIT
