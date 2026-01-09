# Examples

Ready-to-run snippets for common tasks.

## 1. Summarize Section via LLM

```typescript
import { parse, mapIds } from '@ucp-core/core'
import OpenAI from 'openai'

const doc = parse(`# Postmortem\n\n## Timeline\n- 10:32 alert triggered`)
const mapper = mapIds(doc)

const client = new OpenAI()
const prompt = `${mapper.describe(doc)}\n\nSummarize the timeline.`
const response = await client.responses.create({
  model: 'gpt-4.1-mini',
  input: prompt,
})
```

## 2. Replay UCL Commands

```python
import ucp

doc = ucp.parse("""# Outline

## Intro

Paragraph
""")

ucl = """EDIT 3 SET text = "Edited"
APPEND 3 text :: New paragraph"""

# Use your own executor to apply commands
```

## 3. Token-Efficient Editing

```typescript
import { prompt, mapIds } from '@ucp-core/core'

const systemPrompt = prompt().edit().append().withShortIds().build()
const mapper = mapIds(doc)
const shortDoc = mapper.describe(doc)
```
