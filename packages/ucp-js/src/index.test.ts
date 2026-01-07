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
