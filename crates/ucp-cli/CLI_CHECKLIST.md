# UCP CLI Feature Checklist

This checklist tracks the implementation status of CLI features based on the Rust, Python, and JavaScript SDKs.

## Rust SDK Features

### Document Management
- [x] `create` - Create new document
- [x] `open` - Load document from file (JSON) - via `--input` flag
- [x] `save` - Save document to file (JSON) - via `--output` flag
- [x] `info` - Document statistics and metadata
- [x] `validate` - Run validation pipeline

### Block Operations
- [x] `block add` - Create new block (with content type selection)
- [x] `block get` - Retrieve block by ID
- [x] `block delete` - Remove block(s) (with cascade option)
- [x] `block move` - Relocate block (to parent, before, after)
- [x] `block list` - List blocks in document
- [x] `block update` - Modify block content

### Content Types
- [x] `block add --type text` - Add text block
- [x] `block add --type markdown` - Add markdown text block
- [x] `block add --type code` - Add code block with language
- [x] `block add --type table` - Create table
- [x] `block add --type json` - Add JSON block
- [x] `block add --type math` - Add math expression
- [x] `block add --type media` - Add media reference

### Metadata & Tags
- [x] `block update --add-tag` - Add tag to block
- [x] `block update --remove-tag` - Remove tag from block
- [x] `block update --label` - Set block label
- [x] `block update --role` - Set semantic role
- [x] `block update --summary` - Set block summary

### Relationships (Edges)
- [x] `edge add` - Create edge between blocks
- [x] `edge remove` - Remove edge
- [x] `edge list` - List edges for a block (incoming/outgoing)

### Navigation & Query
- [x] `nav children` - Show child blocks
- [x] `nav parent` - Show parent block
- [x] `nav siblings` - Show sibling blocks
- [x] `nav descendants` - Show all descendants
- [x] `find` - Search blocks by criteria (role, tag, content pattern)
- [x] `orphans` - Find unreachable blocks

### Structure Operations
- [x] `prune` - Remove orphaned/tagged blocks
- [x] `tree` - Display document hierarchy as tree

### Transactions
- [x] `tx begin` - Start transaction
- [x] `tx commit` - Complete transaction
- [x] `tx rollback` - Abort transaction
- [x] `tx savepoint` - Create transaction savepoint

### Snapshots
- [x] `snapshot create` - Save version
- [x] `snapshot restore` - Load version
- [x] `snapshot list` - Show snapshots
- [x] `snapshot delete` - Remove snapshot
- [ ] `snapshot diff` - Compare versions (not implemented - needs snapshot storage)

### Translators
- [x] `import markdown` - Parse markdown file to document
- [x] `import html` - Parse HTML file to document
- [x] `export markdown` - Render document to markdown
- [x] `export json` - Serialize document to JSON

### UCL Execution
- [x] `ucl exec` - Execute UCL commands from string or file
- [x] `ucl parse` - Parse and validate UCL string

### Agent Traversal
- [x] `agent session create` - Start agent session
- [x] `agent session list` - Show active sessions
- [x] `agent session close` - Close session
- [x] `agent goto` - Navigate to block
- [x] `agent back` - Go back in history
- [x] `agent expand` - Expand from current position
- [x] `agent follow` - Follow edge type
- [x] `agent search` - Search from current position
- [x] `agent find` - Find blocks with conditions
- [x] `agent context add` - Add blocks to context window
- [x] `agent context remove` - Remove blocks from context
- [x] `agent context clear` - Clear context window
- [x] `agent view` - View current block/context

### LLM Integration
- [x] `llm id-map` - Create ID mapping for token efficiency
- [x] `llm shorten-ucl` - Convert UCL to use short IDs
- [x] `llm expand-ucl` - Convert UCL from short to full IDs
- [x] `llm prompt` - Generate capability-based prompt documentation
- [x] `llm context` - Manage context window for LLM

### Observability
- [x] `--verbose` flag - Enable detailed output
- [x] `--trace` flag - Enable tracing output
- [x] JSON output mode for all commands (`--format json`)

---

## Python SDK Features

### Core Types (leave blank - user will fill in)
- [ ]
- [ ]
- [ ]

### Document Operations
- [ ]
- [ ]
- [ ]

### Block Operations
- [ ]
- [ ]
- [ ]

### Engine Operations
- [ ]
- [ ]
- [ ]

### Traversal
- [ ]
- [ ]
- [ ]

### LLM Utilities
- [ ]
- [ ]
- [ ]

---

## JavaScript/TypeScript SDK Features

### Core Types (leave blank - user will fill in)
- [ ]
- [ ]
- [ ]

### Document Operations
- [ ]
- [ ]
- [ ]

### Block Operations
- [ ]
- [ ]
- [ ]

### Section API
- [ ]
- [ ]
- [ ]

### Traversal
- [ ]
- [ ]
- [ ]

### Context Management
- [ ]
- [ ]
- [ ]

---

## Implementation Progress

| Category | Rust | Python | JS/TS |
|----------|------|--------|-------|
| Document Management | 5/5 | _/_ | _/_ |
| Block Operations | 6/6 | _/_ | _/_ |
| Content Types | 7/7 | _/_ | _/_ |
| Metadata & Tags | 5/5 | _/_ | _/_ |
| Edges | 3/3 | _/_ | _/_ |
| Navigation | 6/6 | _/_ | _/_ |
| Structure | 2/2 | _/_ | _/_ |
| Transactions | 4/4 | _/_ | _/_ |
| Snapshots | 4/5 | _/_ | _/_ |
| Translators | 4/4 | _/_ | _/_ |
| UCL | 2/2 | _/_ | _/_ |
| Agent | 13/13 | _/_ | _/_ |
| LLM | 5/5 | _/_ | _/_ |
| Observability | 3/3 | _/_ | _/_ |
| **Total** | **69/70** | **_/_** | **_/_** |
