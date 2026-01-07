# @ucp-core/core

Unified Content Protocol SDK for JavaScript/TypeScript.

Build LLM-powered content manipulation with minimal code.

## Installation

```bash
npm install @ucp-core/core
# or
bun add @ucp-core/core
```

## Quick Start

```typescript
import { ucp } from '@ucp-core/core'

// 1. Parse markdown into a document
const doc = ucp.parse(`
# My Article

This is the introduction.

## Section 1

Some content here.
`)

// 2. Create an ID mapper for token efficiency
const mapper = ucp.mapIds(doc)

// 3. Get a compact document description for the LLM
const description = mapper.describe(doc)
// Output:
// Document Structure:
//   [2] heading1 - My Article
//     [3] paragraph - This is the introduction.
//     [4] heading2 - Section 1
//       [5] paragraph - Some content here.

// 4. Build a prompt with only the capabilities you need
const systemPrompt = ucp.prompt()
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
// Parse markdown
const doc = ucp.parse('# Hello\n\nWorld')

// Render back to markdown
const md = ucp.render(doc)

// Create empty document
const doc = ucp.create()
```

### Prompt Builder

Build prompts with only the capabilities your agent needs:

```typescript
const prompt = ucp.prompt()
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
const mapper = ucp.mapIds(doc)

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
const commands = ucp.ucl()
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

## License

MIT