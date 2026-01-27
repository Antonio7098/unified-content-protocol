# UCP CLI Feature Checklist

This checklist tracks the implementation status of CLI features based on the Rust, Python, and JavaScript SDKs.

## Rust SDK Features

### Document Management
- [ ] `create` - Create new document
- [ ] `open` - Load document from file (JSON)
- [ ] `save` - Save document to file (JSON)
- [ ] `info` - Document statistics and metadata
- [ ] `validate` - Run validation pipeline

### Block Operations
- [ ] `block add` - Create new block (with content type selection)
- [ ] `block get` - Retrieve block by ID
- [ ] `block delete` - Remove block(s) (with cascade option)
- [ ] `block move` - Relocate block (to parent, before, after)
- [ ] `block list` - List blocks in document
- [ ] `block update` - Modify block content

### Content Types
- [ ] `block add --type text` - Add text block
- [ ] `block add --type markdown` - Add markdown text block
- [ ] `block add --type code` - Add code block with language
- [ ] `block add --type table` - Create table
- [ ] `block add --type json` - Add JSON block
- [ ] `block add --type math` - Add math expression
- [ ] `block add --type media` - Add media reference

### Metadata & Tags
- [ ] `block tag add` - Add tag to block
- [ ] `block tag remove` - Remove tag from block
- [ ] `block label` - Set block label
- [ ] `block role` - Set semantic role
- [ ] `block summary` - Set block summary

### Relationships (Edges)
- [ ] `edge add` - Create edge between blocks
- [ ] `edge remove` - Remove edge
- [ ] `edge list` - List edges for a block (incoming/outgoing)

### Navigation & Query
- [ ] `nav children` - Show child blocks
- [ ] `nav parent` - Show parent block
- [ ] `nav siblings` - Show sibling blocks
- [ ] `nav descendants` - Show all descendants
- [ ] `find` - Search blocks by criteria (role, tag, content pattern)
- [ ] `orphans` - Find unreachable blocks

### Structure Operations
- [ ] `prune` - Remove orphaned/tagged blocks
- [ ] `tree` - Display document hierarchy as tree

### Transactions
- [ ] `tx begin` - Start transaction
- [ ] `tx commit` - Complete transaction
- [ ] `tx rollback` - Abort transaction
- [ ] `tx savepoint` - Create transaction savepoint

### Snapshots
- [ ] `snapshot create` - Save version
- [ ] `snapshot restore` - Load version
- [ ] `snapshot list` - Show snapshots
- [ ] `snapshot delete` - Remove snapshot
- [ ] `snapshot diff` - Compare versions

### Translators
- [ ] `import markdown` - Parse markdown file to document
- [ ] `import html` - Parse HTML file to document
- [ ] `export markdown` - Render document to markdown
- [ ] `export json` - Serialize document to JSON

### UCL Execution
- [ ] `ucl exec` - Execute UCL commands from string or file
- [ ] `ucl parse` - Parse and validate UCL string

### Agent Traversal
- [ ] `agent session create` - Start agent session
- [ ] `agent session list` - Show active sessions
- [ ] `agent session close` - Close session
- [ ] `agent goto` - Navigate to block
- [ ] `agent back` - Go back in history
- [ ] `agent expand` - Expand from current position
- [ ] `agent follow` - Follow edge type
- [ ] `agent search` - Search from current position
- [ ] `agent find` - Find blocks with conditions
- [ ] `agent context add` - Add blocks to context window
- [ ] `agent context remove` - Remove blocks from context
- [ ] `agent context clear` - Clear context window
- [ ] `agent view` - View current block/context

### LLM Integration
- [ ] `llm id-map` - Create ID mapping for token efficiency
- [ ] `llm shorten-ucl` - Convert UCL to use short IDs
- [ ] `llm expand-ucl` - Convert UCL from short to full IDs
- [ ] `llm prompt` - Generate capability-based prompt documentation
- [ ] `llm context` - Manage context window for LLM

### Observability
- [ ] `--verbose` flag - Enable detailed output
- [ ] `--trace` flag - Enable tracing output
- [ ] JSON output mode for all commands

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
| Document Management | 0/5 | _/_ | _/_ |
| Block Operations | 0/6 | _/_ | _/_ |
| Content Types | 0/7 | _/_ | _/_ |
| Metadata & Tags | 0/5 | _/_ | _/_ |
| Edges | 0/3 | _/_ | _/_ |
| Navigation | 0/6 | _/_ | _/_ |
| Structure | 0/2 | _/_ | _/_ |
| Transactions | 0/4 | _/_ | _/_ |
| Snapshots | 0/5 | _/_ | _/_ |
| Translators | 0/4 | _/_ | _/_ |
| UCL | 0/2 | _/_ | _/_ |
| Agent | 0/13 | _/_ | _/_ |
| LLM | 0/5 | _/_ | _/_ |
| Observability | 0/3 | _/_ | _/_ |
| **Total** | **0/70** | **_/_** | **_/_** |
