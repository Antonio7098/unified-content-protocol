import { describe, it, expect } from 'vitest'
import {
  ucp,
  parseMarkdown,
  renderMarkdown,
  createDocument,
  addBlock,
  PromptBuilder,
  IdMapper,
  UclBuilder,
} from './index.js'

describe('Document Operations', () => {
  it('creates an empty document', () => {
    const doc = createDocument()
    expect(doc.blocks.size).toBe(1) // root block
    expect(doc.blocks.get(doc.root)).toBeDefined()
  })

  it('adds blocks to a document', () => {
    const doc = createDocument()
    const id = addBlock(doc, doc.root, 'Hello', { role: 'paragraph' })

    expect(doc.blocks.size).toBe(2)
    expect(doc.blocks.get(id)?.content).toBe('Hello')
  })

  it('parses markdown', () => {
    const doc = parseMarkdown('# Hello\n\nWorld')

    expect(doc.blocks.size).toBe(3) // root + heading + paragraph
  })

  it('renders markdown', () => {
    const doc = parseMarkdown('# Hello\n\nWorld')
    const md = renderMarkdown(doc)

    expect(md).toContain('# Hello')
    expect(md).toContain('World')
  })

  it('handles code blocks', () => {
    const doc = parseMarkdown('# Title\n\n```js\nconsole.log("hi")\n```')

    const blocks = Array.from(doc.blocks.values())
    const codeBlock = blocks.find(b => b.type === 'code')

    expect(codeBlock).toBeDefined()
    expect(codeBlock?.content).toContain('console.log')
  })
})

describe('PromptBuilder', () => {
  it('builds a prompt with capabilities', () => {
    const prompt = new PromptBuilder().edit().append().build()

    expect(prompt).toContain('EDIT')
    expect(prompt).toContain('APPEND')
    expect(prompt).not.toContain('MOVE')
  })

  it('enables all capabilities', () => {
    const prompt = new PromptBuilder().all().build()

    expect(prompt).toContain('EDIT')
    expect(prompt).toContain('APPEND')
    expect(prompt).toContain('MOVE')
    expect(prompt).toContain('DELETE')
  })

  it('adds short ID instructions', () => {
    const prompt = new PromptBuilder().edit().withShortIds().build()

    expect(prompt).toContain('short numbers')
  })

  it('adds custom rules', () => {
    const prompt = new PromptBuilder()
      .edit()
      .withRule('Be concise')
      .build()

    expect(prompt).toContain('Be concise')
  })

  it('throws without capabilities', () => {
    expect(() => new PromptBuilder().build()).toThrow()
  })
})

describe('IdMapper', () => {
  it('creates mappings from document', () => {
    const doc = parseMarkdown('# Hello\n\nWorld')
    const mapper = IdMapper.fromDocument(doc)

    expect(mapper.getShort(doc.root)).toBe(1)
  })

  it('shortens text with block IDs', () => {
    const doc = parseMarkdown('# Hello')
    const mapper = IdMapper.fromDocument(doc)

    const blocks = Array.from(doc.blocks.values())
    const heading = blocks.find(b => b.role === 'heading1')!

    const text = `Edit block ${heading.id}`
    const short = mapper.shorten(text)

    expect(short).not.toContain('blk_')
    expect(short).toMatch(/Edit block \d+/)
  })

  it('expands UCL commands', () => {
    const doc = parseMarkdown('# Hello')
    const mapper = IdMapper.fromDocument(doc)

    const expanded = mapper.expand('EDIT 2 SET text = "hi"')

    expect(expanded).toContain('blk_')
  })

  it('generates document description', () => {
    const doc = parseMarkdown('# Hello\n\nWorld')
    const mapper = IdMapper.fromDocument(doc)

    const desc = mapper.describe(doc)

    expect(desc).toContain('[2]')
    expect(desc).toContain('heading1')
  })
})

describe('UclBuilder', () => {
  it('builds edit commands', () => {
    const ucl = new UclBuilder().edit(1, 'hello').build()

    expect(ucl).toBe('EDIT 1 SET text = "hello"')
  })

  it('builds append commands', () => {
    const ucl = new UclBuilder().append(1, 'content').build()

    expect(ucl).toContain('APPEND 1 text :: content')
  })

  it('builds delete commands', () => {
    const ucl = new UclBuilder().delete(1, true).build()

    expect(ucl).toBe('DELETE 1 CASCADE')
  })

  it('wraps in atomic block', () => {
    const ucl = new UclBuilder()
      .edit(1, 'a')
      .edit(2, 'b')
      .atomic()
      .build()

    expect(ucl).toContain('ATOMIC {')
    expect(ucl).toContain('EDIT 1')
    expect(ucl).toContain('EDIT 2')
  })
})

describe('Edge Cases', () => {
  it('handles empty markdown', () => {
    const doc = parseMarkdown('')
    expect(doc.blocks.size).toBe(1) // just root
  })

  it('handles deeply nested headings', () => {
    const doc = parseMarkdown(`# H1
## H2
### H3
#### H4
##### H5
###### H6

Paragraph under H6`)

    expect(doc.blocks.size).toBe(8) // root + 6 headings + 1 paragraph
  })

  it('preserves content with special characters', () => {
    const content = 'Text with "quotes" and \'apostrophes\' and `backticks`'
    const doc = parseMarkdown(`# Title\n\n${content}`)
    const blocks = Array.from(doc.blocks.values())
    const para = blocks.find(b => b.role === 'paragraph')
    expect(para?.content).toBe(content)
  })

  it('handles multiple code blocks', () => {
    const doc = parseMarkdown(`# Title

\`\`\`js
const a = 1
\`\`\`

\`\`\`python
x = 2
\`\`\``)

    const codeBlocks = Array.from(doc.blocks.values()).filter(b => b.type === 'code')
    expect(codeBlocks.length).toBe(2)
  })

  it('roundtrips markdown correctly', () => {
    const original = `# Hello

World

## Section

Content here
`
    const doc = parseMarkdown(original)
    const rendered = renderMarkdown(doc)

    expect(rendered).toContain('# Hello')
    expect(rendered).toContain('World')
    expect(rendered).toContain('## Section')
    expect(rendered).toContain('Content here')
  })
})

describe('Error Handling', () => {
  it('throws when adding to non-existent parent', () => {
    const doc = createDocument()
    expect(() => addBlock(doc, 'invalid_id', 'content')).toThrow()
  })

  it('PromptBuilder requires at least one capability', () => {
    expect(() => new PromptBuilder().build()).toThrow('At least one capability')
  })

  it('IdMapper returns undefined for unknown IDs', () => {
    const mapper = new IdMapper()
    expect(mapper.getShort('unknown')).toBeUndefined()
    expect(mapper.getLong(999)).toBeUndefined()
  })
})

describe('UclBuilder Advanced', () => {
  it('builds move commands', () => {
    expect(new UclBuilder().moveTo(1, 2).build()).toBe('MOVE 1 TO 2')
    expect(new UclBuilder().moveBefore(1, 2).build()).toBe('MOVE 1 BEFORE 2')
    expect(new UclBuilder().moveAfter(1, 2).build()).toBe('MOVE 1 AFTER 2')
  })

  it('builds link commands', () => {
    expect(new UclBuilder().link(1, 'references', 2).build()).toBe('LINK 1 references 2')
  })

  it('chains multiple commands', () => {
    const ucl = new UclBuilder()
      .edit(1, 'a')
      .append(1, 'b')
      .delete(2)
      .build()

    expect(ucl).toContain('EDIT 1')
    expect(ucl).toContain('APPEND 1')
    expect(ucl).toContain('DELETE 2')
  })

  it('escapes special characters in content', () => {
    const ucl = new UclBuilder().edit(1, 'line1\nline2').build()
    expect(ucl).toContain('\\n')
  })

  it('returns commands as array', () => {
    const builder = new UclBuilder().edit(1, 'a').edit(2, 'b')
    expect(builder.toArray()).toHaveLength(2)
  })
})

describe('IdMapper Advanced', () => {
  it('handles UCL with multiple ID references', () => {
    const doc = parseMarkdown('# A\n\n## B\n\n### C')
    const mapper = IdMapper.fromDocument(doc)

    const ucl = 'MOVE 3 TO 2\nLINK 2 references 4'
    const expanded = mapper.expand(ucl)

    expect(expanded).toContain('blk_')
    expect(expanded).not.toMatch(/\b[234]\b/)
  })

  it('provides accurate mappings list', () => {
    const doc = parseMarkdown('# Title\n\nPara')
    const mapper = IdMapper.fromDocument(doc)
    const mappings = mapper.getMappings()

    expect(mappings.length).toBe(3) // root + heading + para
    expect(mappings[0].short).toBe(1)
  })
})

describe('ucp API', () => {
  it('exposes parse function', () => {
    const doc = ucp.parse('# Hello')
    expect(doc.blocks.size).toBeGreaterThan(1)
  })

  it('exposes prompt function', () => {
    const builder = ucp.prompt()
    expect(builder).toBeInstanceOf(PromptBuilder)
  })

  it('exposes mapIds function', () => {
    const doc = ucp.parse('# Hello')
    const mapper = ucp.mapIds(doc)
    expect(mapper).toBeInstanceOf(IdMapper)
  })

  it('exposes ucl function', () => {
    const builder = ucp.ucl()
    expect(builder).toBeInstanceOf(UclBuilder)
  })
})
