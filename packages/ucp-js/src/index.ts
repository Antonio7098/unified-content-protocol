/**
 * @ucp/core - Unified Content Protocol SDK
 *
 * A developer-friendly SDK for building LLM-powered content manipulation.
 *
 * @example
 * ```typescript
 * import { ucp } from '@anthropic/ucp'
 *
 * // Parse markdown into a document
 * const doc = ucp.parse('# Hello\n\nWorld')
 *
 * // Get a prompt builder for your LLM
 * const prompt = ucp.prompt()
 *   .edit()
 *   .append()
 *   .withShortIds()
 *   .build()
 *
 * // Map IDs for token efficiency
 * const mapper = ucp.mapIds(doc)
 * const shortPrompt = mapper.shorten(docDescription)
 * const expandedUcl = mapper.expand(llmResponse)
 * ```
 */

// =============================================================================
// TYPES
// =============================================================================

/** Block ID - unique identifier for content blocks */
export type BlockId = string

/** Content types supported by UCM */
export type ContentType = 'text' | 'code' | 'table' | 'math' | 'json' | 'media'

/** Semantic roles for blocks */
export type SemanticRole =
  | 'heading1' | 'heading2' | 'heading3' | 'heading4' | 'heading5' | 'heading6'
  | 'paragraph' | 'quote' | 'list' | 'code' | 'table'
  | 'title' | 'subtitle' | 'abstract'

/** A content block in the document */
export interface Block {
  id: BlockId
  content: string
  type: ContentType
  role?: SemanticRole
  label?: string
  children: BlockId[]
}

/** A UCM document */
export interface Document {
  id: string
  root: BlockId
  blocks: Map<BlockId, Block>
}

/** UCL command capabilities */
export type Capability = 'edit' | 'append' | 'move' | 'delete' | 'link' | 'snapshot' | 'transaction'

// =============================================================================
// DOCUMENT OPERATIONS
// =============================================================================

let blockCounter = 0

function generateId(): BlockId {
  blockCounter++
  return `blk_${blockCounter.toString(16).padStart(12, '0')}`
}

/** Create a new empty document */
export function createDocument(): Document {
  const rootId = generateId()
  const root: Block = {
    id: rootId,
    content: '',
    type: 'text',
    children: [],
  }

  return {
    id: `doc_${Date.now().toString(16)}`,
    root: rootId,
    blocks: new Map([[rootId, root]]),
  }
}

/** Add a block to a document */
export function addBlock(
  doc: Document,
  parentId: BlockId,
  content: string,
  options: { type?: ContentType; role?: SemanticRole; label?: string } = {}
): BlockId {
  const parent = doc.blocks.get(parentId)
  if (!parent) throw new Error(`Parent block not found: ${parentId}`)

  const id = generateId()
  const block: Block = {
    id,
    content,
    type: options.type ?? 'text',
    role: options.role,
    label: options.label,
    children: [],
  }

  doc.blocks.set(id, block)
  parent.children.push(id)

  return id
}

/** Get a block by ID */
export function getBlock(doc: Document, id: BlockId): Block | undefined {
  return doc.blocks.get(id)
}

// =============================================================================
// MARKDOWN PARSING
// =============================================================================

/** Parse markdown into a UCM document */
export function parseMarkdown(markdown: string): Document {
  const doc = createDocument()
  const lines = markdown.split('\n')

  let currentParent = doc.root
  const headingStack: { level: number; id: BlockId }[] = [{ level: 0, id: doc.root }]

  let i = 0
  while (i < lines.length) {
    const line = lines[i]

    // Skip empty lines
    if (line.trim() === '') {
      i++
      continue
    }

    // Heading
    const headingMatch = line.match(/^(#{1,6})\s+(.+)$/)
    if (headingMatch) {
      const level = headingMatch[1].length
      const text = headingMatch[2]

      // Find parent (closest heading with lower level)
      while (headingStack.length > 0 && headingStack[headingStack.length - 1].level >= level) {
        headingStack.pop()
      }
      currentParent = headingStack[headingStack.length - 1]?.id ?? doc.root

      const id = addBlock(doc, currentParent, text, {
        role: `heading${level}` as SemanticRole,
      })

      headingStack.push({ level, id })
      currentParent = id
      i++
      continue
    }

    // Code block
    if (line.startsWith('```')) {
      const lang = line.slice(3).trim()
      const codeLines: string[] = []
      i++
      while (i < lines.length && !lines[i].startsWith('```')) {
        codeLines.push(lines[i])
        i++
      }
      addBlock(doc, currentParent, codeLines.join('\n'), { type: 'code', role: 'code' })
      i++ // skip closing ```
      continue
    }

    // Quote
    if (line.startsWith('> ')) {
      const quoteLines: string[] = []
      while (i < lines.length && lines[i].startsWith('> ')) {
        quoteLines.push(lines[i].slice(2))
        i++
      }
      addBlock(doc, currentParent, quoteLines.join('\n'), { role: 'quote' })
      continue
    }

    // Paragraph
    const paragraphLines: string[] = []
    while (i < lines.length && lines[i].trim() !== '' && !lines[i].startsWith('#') && !lines[i].startsWith('```')) {
      paragraphLines.push(lines[i])
      i++
    }
    if (paragraphLines.length > 0) {
      addBlock(doc, currentParent, paragraphLines.join('\n'), { role: 'paragraph' })
    }
  }

  return doc
}

/** Render a document to markdown */
export function renderMarkdown(doc: Document): string {
  const lines: string[] = []

  function renderBlock(id: BlockId) {
    const block = doc.blocks.get(id)
    if (!block) return

    // Skip root block content
    if (id !== doc.root) {
      if (block.role?.startsWith('heading')) {
        const level = parseInt(block.role.slice(7), 10)
        lines.push('#'.repeat(level) + ' ' + block.content)
        lines.push('')
      } else if (block.role === 'code') {
        lines.push('```')
        lines.push(block.content)
        lines.push('```')
        lines.push('')
      } else if (block.role === 'quote') {
        block.content.split('\n').forEach(l => lines.push('> ' + l))
        lines.push('')
      } else {
        lines.push(block.content)
        lines.push('')
      }
    }

    // Render children
    for (const childId of block.children) {
      renderBlock(childId)
    }
  }

  renderBlock(doc.root)
  return lines.join('\n').trim() + '\n'
}

// =============================================================================
// PROMPT BUILDER
// =============================================================================

const CAPABILITY_DOCS: Record<Capability, string> = {
  edit: `### EDIT - Modify block content
\`\`\`
EDIT <block_id> SET text = "<new_content>"
\`\`\``,

  append: `### APPEND - Add new blocks
\`\`\`
APPEND <parent_id> text :: <content>
APPEND <parent_id> code WITH label = "name" :: <content>
\`\`\``,

  move: `### MOVE - Relocate blocks
\`\`\`
MOVE <block_id> TO <new_parent_id>
MOVE <block_id> BEFORE <sibling_id>
MOVE <block_id> AFTER <sibling_id>
\`\`\``,

  delete: `### DELETE - Remove blocks
\`\`\`
DELETE <block_id>
DELETE <block_id> CASCADE
\`\`\``,

  link: `### LINK - Manage relationships
\`\`\`
LINK <source_id> references <target_id>
UNLINK <source_id> references <target_id>
\`\`\``,

  snapshot: `### SNAPSHOT - Version control
\`\`\`
SNAPSHOT CREATE "name"
SNAPSHOT RESTORE "name"
\`\`\``,

  transaction: `### TRANSACTION - Atomic operations
\`\`\`
ATOMIC { <commands> }
\`\`\``,
}

/** Fluent prompt builder for LLM agents */
export class PromptBuilder {
  private capabilities = new Set<Capability>()
  private shortIds = false
  private customRules: string[] = []
  private context?: string

  /** Enable EDIT capability */
  edit(): this {
    this.capabilities.add('edit')
    return this
  }

  /** Enable APPEND capability */
  append(): this {
    this.capabilities.add('append')
    return this
  }

  /** Enable MOVE capability */
  move(): this {
    this.capabilities.add('move')
    return this
  }

  /** Enable DELETE capability */
  delete(): this {
    this.capabilities.add('delete')
    return this
  }

  /** Enable LINK capability */
  link(): this {
    this.capabilities.add('link')
    return this
  }

  /** Enable SNAPSHOT capability */
  snapshot(): this {
    this.capabilities.add('snapshot')
    return this
  }

  /** Enable TRANSACTION capability */
  transaction(): this {
    this.capabilities.add('transaction')
    return this
  }

  /** Enable all capabilities */
  all(): this {
    Object.keys(CAPABILITY_DOCS).forEach(c => this.capabilities.add(c as Capability))
    return this
  }

  /** Use short numeric IDs (1, 2, 3) instead of full block IDs */
  withShortIds(): this {
    this.shortIds = true
    return this
  }

  /** Add a custom rule */
  withRule(rule: string): this {
    this.customRules.push(rule)
    return this
  }

  /** Add context to the prompt */
  withContext(ctx: string): this {
    this.context = ctx
    return this
  }

  /** Build the system prompt */
  build(): string {
    const caps = Array.from(this.capabilities)
    if (caps.length === 0) {
      throw new Error('At least one capability must be enabled')
    }

    const parts: string[] = []

    // Header
    parts.push('You are a UCL (Unified Content Language) command generator.')
    parts.push('Generate valid UCL commands to manipulate documents.')
    parts.push('')

    // Context
    if (this.context) {
      parts.push(this.context)
      parts.push('')
    }

    // Command reference
    parts.push('## UCL Commands')
    parts.push('')
    for (const cap of caps) {
      parts.push(CAPABILITY_DOCS[cap])
      parts.push('')
    }

    // Rules
    parts.push('## Rules')
    parts.push('1. Output ONLY UCL commands, no explanations')
    parts.push('2. Use exact block IDs as provided')
    parts.push('3. String values must be quoted')

    if (this.shortIds) {
      parts.push('4. Block IDs are short numbers (1, 2, 3, etc.)')
    } else {
      parts.push('4. Block IDs have format: blk_XXXXXXXXXXXX')
    }

    // Custom rules
    this.customRules.forEach((rule, i) => {
      parts.push(`${5 + i}. ${rule}`)
    })

    return parts.join('\n')
  }

  /** Build a complete prompt with document and task */
  buildPrompt(documentDescription: string, task: string): string {
    return `${documentDescription}

## Task
${task}

Generate the UCL command:`
  }
}

/** Create a new prompt builder */
export function prompt(): PromptBuilder {
  return new PromptBuilder()
}

// =============================================================================
// ID MAPPER
// =============================================================================

/** Maps long block IDs to short numbers for token efficiency */
export class IdMapper {
  private toShort = new Map<BlockId, number>()
  private toLong = new Map<number, BlockId>()
  private nextId = 1

  /** Create a mapper from a document */
  static fromDocument(doc: Document): IdMapper {
    const mapper = new IdMapper()

    // Add root first
    mapper.register(doc.root)

    // Add all blocks in sorted order for determinism
    const ids = Array.from(doc.blocks.keys()).sort()
    for (const id of ids) {
      if (id !== doc.root) {
        mapper.register(id)
      }
    }

    return mapper
  }

  /** Register a block ID */
  register(id: BlockId): number {
    if (this.toShort.has(id)) {
      return this.toShort.get(id)!
    }

    const shortId = this.nextId++
    this.toShort.set(id, shortId)
    this.toLong.set(shortId, id)
    return shortId
  }

  /** Get short ID for a block */
  getShort(id: BlockId): number | undefined {
    return this.toShort.get(id)
  }

  /** Get long ID from short */
  getLong(shortId: number): BlockId | undefined {
    return this.toLong.get(shortId)
  }

  /** Shorten all block IDs in text */
  shorten(text: string): string {
    let result = text
    // Process longer IDs first to avoid partial matches
    const entries = Array.from(this.toShort.entries())
      .sort((a, b) => b[0].length - a[0].length)

    for (const [longId, shortId] of entries) {
      result = result.replaceAll(longId, shortId.toString())
    }
    return result
  }

  /** Expand short IDs back to long IDs in UCL commands */
  expand(ucl: string): string {
    let result = ucl

    // Match UCL command patterns with numbers
    const patterns = [
      /\b(EDIT|APPEND|MOVE|DELETE|LINK|UNLINK|TO|BEFORE|AFTER)\s+(\d+)/g,
      /\b(references|elaborates|summarizes|supports|requires)\s+(\d+)/g,
    ]

    for (const pattern of patterns) {
      result = result.replace(pattern, (match, prefix, num) => {
        const longId = this.toLong.get(parseInt(num, 10))
        return longId ? `${prefix} ${longId}` : match
      })
    }

    return result
  }

  /** Generate a compact document description */
  describe(doc: Document): string {
    const lines: string[] = ['Document Structure:']

    const describe = (id: BlockId, depth: number) => {
      const block = doc.blocks.get(id)
      if (!block) return

      const indent = '  '.repeat(depth)
      const shortId = this.toShort.get(id)
      const role = block.role ?? 'block'
      const preview = block.content.slice(0, 40) + (block.content.length > 40 ? '...' : '')

      if (id !== doc.root || block.content) {
        lines.push(`${indent}[${shortId}] ${role} - ${preview}`)
      }

      for (const childId of block.children) {
        describe(childId, depth + 1)
      }
    }

    describe(doc.root, 0)
    return lines.join('\n')
  }

  /** Get the mapping table (for debugging) */
  getMappings(): Array<{ short: number; long: BlockId }> {
    return Array.from(this.toLong.entries())
      .map(([short, long]) => ({ short, long }))
      .sort((a, b) => a.short - b.short)
  }
}

/** Create an ID mapper from a document */
export function mapIds(doc: Document): IdMapper {
  return IdMapper.fromDocument(doc)
}

// =============================================================================
// UCL BUILDER
// =============================================================================

/** Fluent builder for UCL commands */
export class UclBuilder {
  private commands: string[] = []

  /** Add an EDIT command */
  edit(blockId: string | number, content: string): this {
    this.commands.push(`EDIT ${blockId} SET text = "${this.escape(content)}"`)
    return this
  }

  /** Add an APPEND command */
  append(parentId: string | number, content: string, options?: { type?: ContentType; label?: string }): this {
    const type = options?.type ?? 'text'
    const label = options?.label ? ` WITH label = "${options.label}"` : ''
    this.commands.push(`APPEND ${parentId} ${type}${label} :: ${content}`)
    return this
  }

  /** Add a MOVE command */
  moveTo(blockId: string | number, newParent: string | number): this {
    this.commands.push(`MOVE ${blockId} TO ${newParent}`)
    return this
  }

  /** Add a MOVE BEFORE command */
  moveBefore(blockId: string | number, sibling: string | number): this {
    this.commands.push(`MOVE ${blockId} BEFORE ${sibling}`)
    return this
  }

  /** Add a MOVE AFTER command */
  moveAfter(blockId: string | number, sibling: string | number): this {
    this.commands.push(`MOVE ${blockId} AFTER ${sibling}`)
    return this
  }

  /** Add a DELETE command */
  delete(blockId: string | number, cascade = false): this {
    this.commands.push(`DELETE ${blockId}${cascade ? ' CASCADE' : ''}`)
    return this
  }

  /** Add a LINK command */
  link(source: string | number, edgeType: string, target: string | number): this {
    this.commands.push(`LINK ${source} ${edgeType} ${target}`)
    return this
  }

  /** Wrap all commands in ATOMIC block */
  atomic(): this {
    if (this.commands.length > 0) {
      const inner = this.commands.map(c => '  ' + c).join('\n')
      this.commands = [`ATOMIC {\n${inner}\n}`]
    }
    return this
  }

  /** Build the final UCL string */
  build(): string {
    return this.commands.join('\n')
  }

  /** Get commands as array */
  toArray(): string[] {
    return [...this.commands]
  }

  private escape(s: string): string {
    return s.replace(/"/g, '\\"').replace(/\n/g, '\\n')
  }
}

/** Create a new UCL builder */
export function ucl(): UclBuilder {
  return new UclBuilder()
}

// =============================================================================
// MAIN API - Simple unified interface
// =============================================================================

/** Main UCP API - simple, unified interface */
export const ucp = {
  /** Parse markdown into a document */
  parse: parseMarkdown,

  /** Render document to markdown */
  render: renderMarkdown,

  /** Create an empty document */
  create: createDocument,

  /** Create a prompt builder */
  prompt,

  /** Create an ID mapper from a document */
  mapIds,

  /** Create a UCL command builder */
  ucl,
}

export default ucp
