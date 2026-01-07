# JavaScript SDK

The JavaScript/TypeScript SDK (`@ucp-core/core`) gives developers a fluent API for working with UCP on servers, edge runtimes, and browsers.

## Installation

```bash
npm install @ucp-core/core
# or
bun add @ucp-core/core
```

## Core Imports

```typescript
import { ucp, PromptBuilder, IdMapper, UclBuilder } from '@ucp-core/core'
```

## Creating Documents

```typescript
const doc = ucp.parse(`# Title\n\nParagraph text\n\n## Section\n\nMore text`)
const markdown = ucp.render(doc)
const empty = ucp.create()
```

## Prompt Builder

```typescript
const prompt = ucp.prompt()
  .edit()
  .append()
  .move()
  .withShortIds()
  .withRule('Keep responses under 50 tokens')
  .build()
```

## ID Mapper

```typescript
const mapper = ucp.mapIds(doc)
const summary = mapper.describe(doc)
const shortened = mapper.shorten('EDIT blk_00000000000c SET text = "Hi"')
const expanded = mapper.expand('EDIT 12 SET text = "Hi"')
```

## UCL Builder

```typescript
const commands = ucp.ucl()
  .edit(4, 'Updated intro')
  .append(2, 'New paragraph')
  .delete(7, true)
  .atomic()
  .build()
```

## Type Definitions

```typescript
import type {
  Document,
  Block,
  BlockId,
  ContentType,
  SemanticRole,
  Capability,
} from '@ucp-core/core'
```

## Testing

```bash
npm install
npx vitest run
```
