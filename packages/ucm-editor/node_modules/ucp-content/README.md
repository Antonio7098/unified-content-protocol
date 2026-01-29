# ucp-js

Unified Content Protocol SDK for JavaScript/TypeScript.

Build LLM-powered content manipulation with minimal code.

## Installation

```bash
npm install ucp-js
# or
bun add ucp-js
# or
yarn add ucp-js
```

## Features

- **Document Model** - Graph-based content representation with blocks and edges
- **Markdown Parsing** - Convert markdown to structured documents
- **LLM Integration** - Prompt builders and ID mapping for token efficiency
- **Validation** - Comprehensive document validation with error codes
- **Snapshots** - Version control for documents
- **Transactions** - Atomic operations with rollback support

## Quick Start

```typescript
import { parse, mapIds, prompt } from 'ucp-js'

// 1. Parse markdown into a document
const doc = parse(`
# My Article

This is the introduction.

## Section 1

Some content here.
`)

// 2. Create an ID mapper for token efficiency
const mapper = mapIds(doc)

// 3. Get a compact document description for the LLM
const description = mapper.describe(doc)
// Output:
// Document Structure:
//   [2] heading1 - My Article
//     [3] paragraph - This is the introduction.
//     [4] heading2 - Section 1
//       [5] paragraph - Some content here.

// 4. Build a prompt with only the capabilities you need
const systemPrompt = prompt()
  .edit()
  .append()
  .withShortIds()
  .build()

// 5. After LLM responds, expand short IDs back to full IDs
const llmResponse = 'EDIT 3 SET text = "Updated intro"'
const expandedUcl = mapper.expand(llmResponse)
// Result: 'EDIT blk_000000000003 SET text = "Updated intro"'
```

## API Reference

### Document Operations

```typescript
import { parse, render, createDocument } from '@ucp-core/core'

// Parse markdown
const doc = parse('# Hello\n\nWorld')

// Render back to markdown
const md = render(doc)

// Create empty document
const empty = createDocument()
```

### Prompt Builder

Build prompts with only the capabilities your agent needs:

```typescript
import { prompt } from '@ucp-core/core'

const systemPrompt = prompt()
  .edit()           // Enable EDIT command
  .append()         // Enable APPEND command
  .move()           // Enable MOVE command
  .delete()         // Enable DELETE command
  .link()           // Enable LINK/UNLINK commands
  .snapshot()       // Enable SNAPSHOT commands
  .transaction()    // Enable ATOMIC transactions
  .all()            // Enable all capabilities
  .withShortIds()   // Use short numeric IDs
  .withRule('Keep responses concise')
  .build()
```

### ID Mapper

Save tokens by using short numeric IDs:

```typescript
import { mapIds } from '@ucp-core/core'

const mapper = mapIds(doc)

// Shorten IDs in any text
const short = mapper.shorten('Block blk_000000000003 has content')
// Result: 'Block 3 has content'

// Expand IDs in UCL commands
const expanded = mapper.expand('EDIT 3 SET text = "hello"')
// Result: 'EDIT blk_000000000003 SET text = "hello"'

// Get document description with short IDs
const desc = mapper.describe(doc)
```

### UCL Builder

Build UCL commands programmatically:

```typescript
import { ucl } from '@ucp-core/core'

const commands = ucl()
  .edit(3, 'Updated content')
  .append(2, 'New paragraph')
  .delete(5)
  .atomic()  // Wrap in ATOMIC block
  .build()
```

## Token Efficiency

Using short IDs can significantly reduce token usage:

| ID Format | Example | Tokens |
|-----------|---------|--------|
| Long | `blk_000000000003` | ~6 |
| Short | `3` | 1 |

For a document with 50 blocks referenced 3 times each, this saves ~750 tokens.

## TypeScript Support

Full TypeScript support with exported types:

```typescript
import type { 
  Document, 
  Block, 
  BlockId,
  ContentType,
  SemanticRole,
  Capability 
} from '@ucp-core/core'
```

## Error Handling

The SDK throws descriptive errors for invalid operations:

```typescript
import { createDocument, addBlock, deleteBlock, moveBlock } from '@ucp-core/core'

const doc = createDocument()

try {
  // Block not found
  deleteBlock(doc, 'blk_nonexistent')
} catch (error) {
  console.error(error.message) // "Block not found: blk_nonexistent"
}

try {
  // Cannot delete root
  deleteBlock(doc, doc.root)
} catch (error) {
  console.error(error.message) // "Cannot delete the root block"
}

try {
  // Cannot move into self
  const id = addBlock(doc, doc.root, 'Test')
  moveBlock(doc, id, id)
} catch (error) {
  console.error(error.message) // "Cannot move a block into itself or its descendants"
}
```

### Validation Errors

```typescript
import { validateDocument, DEFAULT_LIMITS } from '@ucp-core/core'

const result = validateDocument(doc)

if (!result.valid) {
  for (const issue of result.issues) {
    console.log(`[${issue.severity}] ${issue.code}: ${issue.message}`)
    // [error] E201: Document structure contains a cycle
    // [warning] E203: Block blk_123 is unreachable from root
  }
}
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

## Bundling

### Webpack / Vite

The SDK works out of the box with modern bundlers:

```typescript
// ESM import
import { parse, render, createDocument } from '@ucp-core/core'
```

### Node.js

Requires Node.js 18+ for native ES modules:

```typescript
// package.json: "type": "module"
import { parse } from '@ucp-core/core'
```

Or use dynamic import:

```javascript
const ucp = await import('@ucp-core/core')
```

## Conformance

This SDK implements the UCP specification. See `docs/conformance/README.md` for the full specification and test vectors.

## License

MIT