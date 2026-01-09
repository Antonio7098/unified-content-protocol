# Getting Started

This guide walks you through installing the SDKs, creating your first document, and integrating UCP into an LLM workflow.

## 1. Install the SDKs

### JavaScript / TypeScript
```bash
npm install @ucp-core/core
# or
bun add @ucp-core/core
```

### Python
```bash
pip install ucp-content
```

## 2. Parse Markdown into UCM

```typescript
import { parse } from '@ucp-core/core'

const doc = parse(`# Welcome\n\nIntro paragraph\n\n## Section\n\nMore text`)
console.log(doc.blocks.size) // root + heading + paragraph + subheading + paragraph
```

```python
import ucp

doc = ucp.parse("""# Welcome

Intro paragraph

## Section

More text
""")
print(len(doc.blocks))
```

## 3. Build an LLM Prompt

```typescript
import { mapIds, prompt } from '@ucp-core/core'

const mapper = mapIds(doc)
const systemPrompt = prompt()
  .edit()
  .append()
  .withShortIds()
  .build()

const docSummary = mapper.describe(doc)
```

```python
mapper = ucp.map_ids(doc)
system_prompt = (
    ucp.prompt()
    .edit()
    .append()
    .with_short_ids()
    .build()
)

doc_summary = mapper.describe(doc)
```

## 4. Expand LLM Output

After the LLM returns UCL using short IDs:

```typescript
const expanded = mapper.expand('EDIT 3 SET text = "New intro"')
```

```python
expanded = mapper.expand('EDIT 3 SET text = "New intro"')
```

## 5. Next Steps

- Explore the [JavaScript SDK](sdk/javascript.md)
- Explore the [Python SDK](sdk/python.md)
- Browse [Examples](examples.md)
- Wire up CI to run SDK tests before publishing
