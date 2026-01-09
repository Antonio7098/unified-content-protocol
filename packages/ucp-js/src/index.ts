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
export type ContentType = 'text' | 'code' | 'table' | 'math' | 'json' | 'media' | 'binary' | 'composite'

/** Semantic roles for blocks */
export type SemanticRole =
  | 'heading1' | 'heading2' | 'heading3' | 'heading4' | 'heading5' | 'heading6'
  | 'paragraph' | 'quote' | 'list' | 'code' | 'table'
  | 'title' | 'subtitle' | 'abstract' | 'intro' | 'body' | 'conclusion'

/** Edge types for block relationships */
export type EdgeType =
  // Derivation relationships
  | 'derived_from' | 'supersedes' | 'transformed_from'
  // Reference relationships
  | 'references' | 'cited_by' | 'links_to'
  // Semantic relationships
  | 'supports' | 'contradicts' | 'elaborates' | 'summarizes'
  // Structural relationships
  | 'parent_of' | 'child_of' | 'sibling_of' | 'previous_sibling' | 'next_sibling'
  // Version relationships
  | 'version_of' | 'alternative_of' | 'translation_of'

/** Edge metadata */
export interface EdgeMetadata {
  confidence?: number
  description?: string
  custom?: Record<string, unknown>
}

/** An edge represents a relationship between blocks */
export interface Edge {
  edgeType: EdgeType
  target: BlockId
  metadata: EdgeMetadata
  createdAt: Date
}

/** Validation severity levels */
export type ValidationSeverity = 'error' | 'warning' | 'info'

/** A validation issue */
export interface ValidationIssue {
  severity: ValidationSeverity
  code: string
  message: string
  blockId?: BlockId
}

/** Validation result */
export interface ValidationResult {
  valid: boolean
  issues: ValidationIssue[]
}

/** Resource limits for validation */
export interface ResourceLimits {
  maxDocumentSize: number
  maxBlockCount: number
  maxBlockSize: number
  maxNestingDepth: number
  maxEdgesPerBlock: number
}

/** Transaction states */
export type TransactionState = 'active' | 'committed' | 'rolled_back' | 'timed_out'

/** Block metadata */
export interface BlockMetadata {
  semanticRole?: SemanticRole
  label?: string
  tags: string[]
  summary?: string
  createdAt: Date
  modifiedAt: Date
  custom: Record<string, unknown>
}

/** A content block in the document */
export interface Block {
  id: BlockId
  content: string
  type: ContentType
  role?: SemanticRole
  label?: string
  tags: string[]
  children: BlockId[]
  edges: Edge[]
  metadata?: BlockMetadata
}

/** Document metadata */
export interface DocumentMetadata {
  title?: string
  description?: string
  authors: string[]
  language?: string
  createdAt: Date
  modifiedAt: Date
  custom: Record<string, unknown>
}

/** A UCM document */
export interface Document {
  id: string
  root: BlockId
  blocks: Map<BlockId, Block>
  metadata?: DocumentMetadata
  version: number
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
export function createDocument(title?: string): Document {
  const rootId = generateId()
  const now = new Date()
  const root: Block = {
    id: rootId,
    content: '',
    type: 'text',
    tags: [],
    children: [],
    edges: [],
  }

  return {
    id: `doc_${Date.now().toString(16)}`,
    root: rootId,
    blocks: new Map([[rootId, root]]),
    metadata: {
      title,
      authors: [],
      createdAt: now,
      modifiedAt: now,
      custom: {},
    },
    version: 1,
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
    tags: [],
    children: [],
    edges: [],
  }

  doc.blocks.set(id, block)
  parent.children.push(id)

  return id
}

/** Get a block by ID */
export function getBlock(doc: Document, id: BlockId): Block | undefined {
  return doc.blocks.get(id)
}

/** Edit a block's textual content */
export function editBlock(doc: Document, id: BlockId, content: string): void {
  const block = doc.blocks.get(id)
  if (!block) throw new Error(`Block not found: ${id}`)
  block.content = content
}

/** Move a block (and its subtree) to a new parent */
export function moveBlock(doc: Document, id: BlockId, newParentId: BlockId, index?: number): void {
  if (id === doc.root) throw new Error('Cannot move the root block')

  const block = doc.blocks.get(id)
  if (!block) throw new Error(`Block not found: ${id}`)

  const newParent = doc.blocks.get(newParentId)
  if (!newParent) throw new Error(`Parent block not found: ${newParentId}`)

  if (newParentId === id || isDescendant(doc, id, newParentId)) {
    throw new Error('Cannot move a block into itself or its descendants')
  }

  const oldParentId = findParent(doc, id)
  if (!oldParentId) throw new Error(`Parent block not found for: ${id}`)
  const oldParent = doc.blocks.get(oldParentId)!

  const childIndex = oldParent.children.indexOf(id)
  if (childIndex === -1) throw new Error(`Block ${id} not linked to parent ${oldParentId}`)
  oldParent.children.splice(childIndex, 1)

  if (index === undefined || index < 0 || index > newParent.children.length) {
    newParent.children.push(id)
  } else {
    newParent.children.splice(index, 0, id)
  }
}

/** Delete a block (with optional cascade to children) */
export function deleteBlock(doc: Document, id: BlockId, options: { cascade?: boolean } = {}): void {
  if (id === doc.root) throw new Error('Cannot delete the root block')

  const block = doc.blocks.get(id)
  if (!block) throw new Error(`Block not found: ${id}`)

  const parentId = findParent(doc, id)
  if (!parentId) throw new Error(`Parent block not found for: ${id}`)
  const parent = doc.blocks.get(parentId)!

  const idx = parent.children.indexOf(id)
  if (idx === -1) throw new Error(`Block ${id} not linked to parent ${parentId}`)
  parent.children.splice(idx, 1)

  if (options.cascade ?? true) {
    for (const childId of block.children) {
      deleteSubtree(doc, childId)
    }
  } else {
    parent.children.splice(idx, 0, ...block.children)
  }

  doc.blocks.delete(id)
}

function findParent(doc: Document, id: BlockId): BlockId | undefined {
  for (const [candidateId, block] of doc.blocks.entries()) {
    if (block.children.includes(id)) {
      return candidateId
    }
  }
  return undefined
}

function isDescendant(doc: Document, ancestorId: BlockId, candidateId: BlockId): boolean {
  const ancestor = doc.blocks.get(ancestorId)
  if (!ancestor) return false

  const queue = [...ancestor.children]
  while (queue.length > 0) {
    const current = queue.shift()!
    if (current === candidateId) return true
    const block = doc.blocks.get(current)
    if (block) queue.push(...block.children)
  }
  return false
}

function deleteSubtree(doc: Document, id: BlockId): void {
  const block = doc.blocks.get(id)
  if (!block) return
  for (const child of block.children) {
    deleteSubtree(doc, child)
  }
  doc.blocks.delete(id)
}

/** Add a tag to a block */
export function addTag(doc: Document, id: BlockId, tag: string): void {
  const block = doc.blocks.get(id)
  if (!block) throw new Error(`Block not found: ${id}`)
  if (!block.tags.includes(tag)) {
    block.tags.push(tag)
  }
}

/** Remove a tag from a block */
export function removeTag(doc: Document, id: BlockId, tag: string): void {
  const block = doc.blocks.get(id)
  if (!block) throw new Error(`Block not found: ${id}`)
  block.tags = block.tags.filter(t => t !== tag)
}

/** Check if a block has a tag */
export function blockHasTag(doc: Document, id: BlockId, tag: string): boolean {
  const block = doc.blocks.get(id)
  if (!block) throw new Error(`Block not found: ${id}`)
  return block.tags.includes(tag)
}

/** Find block IDs with a tag */
export function findBlocksByTag(doc: Document, tag: string): BlockId[] {
  const ids: BlockId[] = []
  for (const [id, block] of doc.blocks.entries()) {
    if (block.tags.includes(tag)) {
      ids.push(id)
    }
  }
  return ids
}

// =============================================================================
// EDGE MANAGEMENT
// =============================================================================

/** Create a new edge */
export function createEdge(edgeType: EdgeType, target: BlockId, metadata: EdgeMetadata = {}): Edge {
  return {
    edgeType,
    target,
    metadata,
    createdAt: new Date(),
  }
}

/** Add an edge to a block */
export function addEdge(
  doc: Document,
  sourceId: BlockId,
  edgeType: EdgeType,
  targetId: BlockId,
  metadata: EdgeMetadata = {}
): void {
  const source = doc.blocks.get(sourceId)
  if (!source) throw new Error(`Source block not found: ${sourceId}`)
  if (!doc.blocks.has(targetId)) throw new Error(`Target block not found: ${targetId}`)

  const edge = createEdge(edgeType, targetId, metadata)
  source.edges.push(edge)
  touchDocument(doc)
}

/** Remove an edge from a block */
export function removeEdge(
  doc: Document,
  sourceId: BlockId,
  edgeType: EdgeType,
  targetId: BlockId
): boolean {
  const source = doc.blocks.get(sourceId)
  if (!source) throw new Error(`Source block not found: ${sourceId}`)

  const initialLength = source.edges.length
  source.edges = source.edges.filter(
    e => !(e.edgeType === edgeType && e.target === targetId)
  )
  const removed = source.edges.length < initialLength
  if (removed) touchDocument(doc)
  return removed
}

/** Check if an edge exists */
export function hasEdge(
  doc: Document,
  sourceId: BlockId,
  targetId: BlockId,
  edgeType?: EdgeType
): boolean {
  const source = doc.blocks.get(sourceId)
  if (!source) return false
  return source.edges.some(
    e => e.target === targetId && (edgeType === undefined || e.edgeType === edgeType)
  )
}

/** Get outgoing edges from a block */
export function getOutgoingEdges(doc: Document, sourceId: BlockId): Edge[] {
  const source = doc.blocks.get(sourceId)
  return source ? [...source.edges] : []
}

/** Get incoming edges to a block */
export function getIncomingEdges(doc: Document, targetId: BlockId): Array<{ source: BlockId; edge: Edge }> {
  const result: Array<{ source: BlockId; edge: Edge }> = []
  for (const [sourceId, block] of doc.blocks.entries()) {
    for (const edge of block.edges) {
      if (edge.target === targetId) {
        result.push({ source: sourceId, edge })
      }
    }
  }
  return result
}

/** Touch document to update modification time and version */
function touchDocument(doc: Document): void {
  if (doc.metadata) {
    doc.metadata.modifiedAt = new Date()
  }
  doc.version++
}

// =============================================================================
// VALIDATION
// =============================================================================

/** Default resource limits */
export const DEFAULT_LIMITS: ResourceLimits = {
  maxDocumentSize: 50 * 1024 * 1024, // 50MB
  maxBlockCount: 100_000,
  maxBlockSize: 5 * 1024 * 1024, // 5MB
  maxNestingDepth: 50,
  maxEdgesPerBlock: 1000,
}

/** Validate a document */
export function validateDocument(doc: Document, limits: ResourceLimits = DEFAULT_LIMITS): ValidationResult {
  const issues: ValidationIssue[] = []

  // Check block count
  if (doc.blocks.size > limits.maxBlockCount) {
    issues.push({
      severity: 'error',
      code: 'E400',
      message: `Document has ${doc.blocks.size} blocks, max is ${limits.maxBlockCount}`,
    })
  }

  // Check for cycles
  if (hasCycles(doc)) {
    issues.push({
      severity: 'error',
      code: 'E201',
      message: 'Document structure contains a cycle',
    })
  }

  // Check nesting depth
  const maxDepth = computeMaxDepth(doc)
  if (maxDepth > limits.maxNestingDepth) {
    issues.push({
      severity: 'error',
      code: 'E403',
      message: `Max nesting depth is ${limits.maxNestingDepth}, document has ${maxDepth}`,
    })
  }

  // Validate each block
  for (const [id, block] of doc.blocks.entries()) {
    const blockIssues = validateBlock(doc, block, limits)
    issues.push(...blockIssues)
  }

  // Check for orphans (warning)
  const orphans = findOrphans(doc)
  for (const orphan of orphans) {
    issues.push({
      severity: 'warning',
      code: 'E203',
      message: `Block ${orphan} is unreachable from root`,
      blockId: orphan,
    })
  }

  const hasErrors = issues.some(i => i.severity === 'error')
  return { valid: !hasErrors, issues }
}

function validateBlock(doc: Document, block: Block, limits: ResourceLimits): ValidationIssue[] {
  const issues: ValidationIssue[] = []

  // Check block size
  const size = new TextEncoder().encode(block.content).length
  if (size > limits.maxBlockSize) {
    issues.push({
      severity: 'error',
      code: 'E402',
      message: `Block ${block.id} has size ${size}, max is ${limits.maxBlockSize}`,
      blockId: block.id,
    })
  }

  // Check edge count
  if (block.edges.length > limits.maxEdgesPerBlock) {
    issues.push({
      severity: 'error',
      code: 'E404',
      message: `Block ${block.id} has ${block.edges.length} edges, max is ${limits.maxEdgesPerBlock}`,
      blockId: block.id,
    })
  }

  // Check edge targets exist
  for (const edge of block.edges) {
    if (!doc.blocks.has(edge.target)) {
      issues.push({
        severity: 'error',
        code: 'E001',
        message: `Block ${block.id} has edge to non-existent block ${edge.target}`,
        blockId: block.id,
      })
    }
  }

  return issues
}

function hasCycles(doc: Document): boolean {
  const visited = new Set<BlockId>()
  const recStack = new Set<BlockId>()

  function dfs(nodeId: BlockId): boolean {
    visited.add(nodeId)
    recStack.add(nodeId)

    const node = doc.blocks.get(nodeId)
    if (node) {
      for (const child of node.children) {
        if (!visited.has(child)) {
          if (dfs(child)) return true
        } else if (recStack.has(child)) {
          return true
        }
      }
    }

    recStack.delete(nodeId)
    return false
  }

  return dfs(doc.root)
}

function computeMaxDepth(doc: Document): number {
  function depthFrom(nodeId: BlockId, current: number): number {
    const node = doc.blocks.get(nodeId)
    if (!node || node.children.length === 0) return current
    return Math.max(...node.children.map(c => depthFrom(c, current + 1)))
  }
  return depthFrom(doc.root, 1)
}

/** Find orphaned blocks (unreachable from root) */
export function findOrphans(doc: Document): BlockId[] {
  const reachable = new Set<BlockId>([doc.root])
  const queue = [doc.root]

  while (queue.length > 0) {
    const current = queue.shift()!
    const block = doc.blocks.get(current)
    if (block) {
      for (const child of block.children) {
        if (!reachable.has(child)) {
          reachable.add(child)
          queue.push(child)
        }
      }
    }
  }

  return Array.from(doc.blocks.keys()).filter(id => !reachable.has(id))
}

/** Remove all orphaned blocks */
export function pruneOrphans(doc: Document): BlockId[] {
  const orphans = findOrphans(doc)
  for (const id of orphans) {
    doc.blocks.delete(id)
  }
  if (orphans.length > 0) touchDocument(doc)
  return orphans
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
// SNAPSHOT MANAGEMENT
// =============================================================================

/** Snapshot data */
export interface Snapshot {
  id: string
  description?: string
  createdAt: Date
  documentVersion: number
  data: string // JSON serialized document
}

/** Serialize a document to JSON string */
export function serializeDocument(doc: Document): string {
  const serializable = {
    id: doc.id,
    root: doc.root,
    blocks: Array.from(doc.blocks.entries()).map(([id, block]) => [id, {
      ...block,
      edges: block.edges.map(e => ({ ...e, createdAt: e.createdAt.toISOString() })),
    }]),
    metadata: doc.metadata ? {
      ...doc.metadata,
      createdAt: doc.metadata.createdAt.toISOString(),
      modifiedAt: doc.metadata.modifiedAt.toISOString(),
    } : undefined,
    version: doc.version,
  }
  return JSON.stringify(serializable)
}

/** Deserialize a document from JSON string */
export function deserializeDocument(data: string): Document {
  const parsed = JSON.parse(data)
  const blocks = new Map<BlockId, Block>()
  
  for (const [id, blockData] of parsed.blocks) {
    blocks.set(id, {
      ...blockData,
      edges: blockData.edges.map((e: { createdAt: string } & Edge) => ({
        ...e,
        createdAt: new Date(e.createdAt),
      })),
    })
  }

  return {
    id: parsed.id,
    root: parsed.root,
    blocks,
    metadata: parsed.metadata ? {
      ...parsed.metadata,
      createdAt: new Date(parsed.metadata.createdAt),
      modifiedAt: new Date(parsed.metadata.modifiedAt),
    } : undefined,
    version: parsed.version,
  }
}

/** Snapshot manager for document versioning */
export class SnapshotManager {
  private snapshots = new Map<string, Snapshot>()
  private maxSnapshots: number

  constructor(maxSnapshots = 100) {
    this.maxSnapshots = maxSnapshots
  }

  /** Create a snapshot of the document */
  create(name: string, doc: Document, description?: string): string {
    if (this.snapshots.size >= this.maxSnapshots) {
      this.evictOldest()
    }

    const snapshot: Snapshot = {
      id: name,
      description,
      createdAt: new Date(),
      documentVersion: doc.version,
      data: serializeDocument(doc),
    }

    this.snapshots.set(name, snapshot)
    return name
  }

  /** Restore a document from a snapshot */
  restore(name: string): Document {
    const snapshot = this.snapshots.get(name)
    if (!snapshot) throw new Error(`Snapshot not found: ${name}`)
    return deserializeDocument(snapshot.data)
  }

  /** Get a snapshot by name */
  get(name: string): Snapshot | undefined {
    return this.snapshots.get(name)
  }

  /** List all snapshots (newest first) */
  list(): Snapshot[] {
    return Array.from(this.snapshots.values())
      .sort((a, b) => b.createdAt.getTime() - a.createdAt.getTime())
  }

  /** Delete a snapshot */
  delete(name: string): boolean {
    return this.snapshots.delete(name)
  }

  /** Check if a snapshot exists */
  exists(name: string): boolean {
    return this.snapshots.has(name)
  }

  /** Get snapshot count */
  count(): number {
    return this.snapshots.size
  }

  private evictOldest(): void {
    let oldest: Snapshot | undefined
    for (const snapshot of this.snapshots.values()) {
      if (!oldest || snapshot.createdAt < oldest.createdAt) {
        oldest = snapshot
      }
    }
    if (oldest) this.snapshots.delete(oldest.id)
  }
}

// =============================================================================
// TRANSACTION MANAGEMENT
// =============================================================================

/** Transaction for atomic operations */
export class Transaction {
  readonly id: string
  readonly name?: string
  private _state: TransactionState = 'active'
  private _startTime: number
  private _timeout: number
  private _initialState: string
  private _doc: Document

  constructor(doc: Document, timeoutMs = 30000, name?: string) {
    this.id = name ?? `txn_${Date.now().toString(16)}`
    this.name = name
    this._doc = doc
    this._startTime = Date.now()
    this._timeout = timeoutMs
    this._initialState = serializeDocument(doc)
  }

  get state(): TransactionState {
    if (this._state === 'active' && this.isTimedOut()) {
      this._state = 'timed_out'
    }
    return this._state
  }

  isActive(): boolean {
    return this.state === 'active'
  }

  isTimedOut(): boolean {
    return Date.now() - this._startTime > this._timeout
  }

  elapsedMs(): number {
    return Date.now() - this._startTime
  }

  commit(): void {
    if (!this.isActive()) {
      throw new Error(`Cannot commit ${this._state} transaction`)
    }
    this._state = 'committed'
  }

  rollback(): void {
    if (this._state === 'committed') {
      throw new Error('Cannot rollback a committed transaction')
    }
    
    // Restore initial state
    const restored = deserializeDocument(this._initialState)
    this._doc.blocks = restored.blocks
    this._doc.metadata = restored.metadata
    this._doc.version = restored.version
    
    this._state = 'rolled_back'
  }
}

/** Transaction manager */
export class TransactionManager {
  private transactions = new Map<string, Transaction>()
  private defaultTimeout: number

  constructor(defaultTimeoutMs = 30000) {
    this.defaultTimeout = defaultTimeoutMs
  }

  begin(doc: Document, name?: string, timeoutMs?: number): Transaction {
    const txn = new Transaction(doc, timeoutMs ?? this.defaultTimeout, name)
    this.transactions.set(txn.id, txn)
    return txn
  }

  get(id: string): Transaction | undefined {
    return this.transactions.get(id)
  }

  commit(id: string): void {
    const txn = this.transactions.get(id)
    if (!txn) throw new Error(`Transaction not found: ${id}`)
    txn.commit()
  }

  rollback(id: string): void {
    const txn = this.transactions.get(id)
    if (!txn) throw new Error(`Transaction not found: ${id}`)
    txn.rollback()
  }

  activeCount(): number {
    return Array.from(this.transactions.values()).filter(t => t.isActive()).length
  }

  cleanup(): number {
    const toRemove: string[] = []
    for (const [id, txn] of this.transactions) {
      if (!txn.isActive()) toRemove.push(id)
    }
    toRemove.forEach(id => this.transactions.delete(id))
    return toRemove.length
  }
}

// =============================================================================
// UCL EXECUTOR
// =============================================================================

/** UCL execution error */
export class UclExecutionError extends Error {
  constructor(message: string, public command?: string) {
    super(command ? `[${command}] ${message}` : message)
    this.name = 'UclExecutionError'
  }
}

/** UCL parse error */
export class UclParseError extends Error {
  constructor(message: string, public line?: number) {
    super(line !== undefined ? `Line ${line}: ${message}` : message)
    this.name = 'UclParseError'
  }
}

/** Execute UCL commands on a document */
export function executeUcl(doc: Document, uclText: string): BlockId[] {
  const affectedBlocks: BlockId[] = []
  const lines = extractUclLines(uclText)

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i]
    try {
      const result = executeUclLine(doc, line)
      affectedBlocks.push(...result)
    } catch (e) {
      if (e instanceof Error) {
        throw new UclExecutionError(e.message, line)
      }
      throw e
    }
  }

  return affectedBlocks
}

function extractUclLines(ucl: string): string[] {
  const lines: string[] = []
  let atomicDepth = 0

  for (const raw of ucl.split('\n')) {
    const stripped = raw.trim()
    if (!stripped || stripped.startsWith('#')) continue
    if (stripped.toUpperCase() === 'ATOMIC {') {
      atomicDepth++
      continue
    }
    if (stripped === '}' && atomicDepth > 0) {
      atomicDepth--
      continue
    }
    lines.push(stripped)
  }

  return lines
}

function executeUclLine(doc: Document, line: string): BlockId[] {
  const upper = line.toUpperCase()

  if (upper.startsWith('EDIT ')) {
    return executeEdit(doc, line)
  } else if (upper.startsWith('APPEND ')) {
    return executeAppend(doc, line)
  } else if (upper.startsWith('MOVE ')) {
    return executeMove(doc, line)
  } else if (upper.startsWith('DELETE ')) {
    return executeDelete(doc, line)
  } else if (upper.startsWith('LINK ')) {
    return executeLink(doc, line)
  } else if (upper.startsWith('UNLINK ')) {
    return executeUnlink(doc, line)
  } else if (upper.startsWith('PRUNE')) {
    return executePrune(doc, line)
  } else {
    throw new UclParseError(`Unknown command: ${line}`)
  }
}

function executeEdit(doc: Document, line: string): BlockId[] {
  const match = line.match(/^EDIT\s+(\S+)\s+SET\s+(\S+)\s*=\s*"((?:\\.|[^"])*)"/i)
  if (!match) throw new UclParseError(`Malformed EDIT command: ${line}`)
  
  const [, blockId, path, value] = match
  const unescaped = value.replace(/\\(.)/g, '$1')

  if (!doc.blocks.has(blockId)) {
    throw new UclExecutionError(`Block not found: ${blockId}`)
  }

  if (path.toLowerCase() === 'text') {
    editBlock(doc, blockId, unescaped)
  } else {
    throw new UclExecutionError(`Unsupported edit path: ${path}`)
  }

  return [blockId]
}

function executeAppend(doc: Document, line: string): BlockId[] {
  const match = line.match(/^APPEND\s+(\S+)\s+(\w+)(?:\s+WITH\s+([^:]+))?\s*::\s*(.+)$/i)
  if (!match) throw new UclParseError(`Malformed APPEND command: ${line}`)

  const [, parentId, contentType, propsStr, content] = match
  
  if (!doc.blocks.has(parentId)) {
    throw new UclExecutionError(`Parent block not found: ${parentId}`)
  }

  const props: Record<string, string> = {}
  if (propsStr) {
    const propMatches = propsStr.matchAll(/(\w+)\s*=\s*"([^"]*)"/g)
    for (const m of propMatches) {
      props[m[1].toLowerCase()] = m[2]
    }
  }

  const newId = addBlock(doc, parentId, content, {
    type: contentType.toLowerCase() as ContentType,
    role: props.role as SemanticRole | undefined,
    label: props.label,
  })

  return [newId]
}

function executeMove(doc: Document, line: string): BlockId[] {
  const match = line.match(/^MOVE\s+(\S+)\s+(TO|BEFORE|AFTER)\s+(\S+)(?:\s+INDEX\s+(\d+))?$/i)
  if (!match) throw new UclParseError(`Malformed MOVE command: ${line}`)

  const [, blockId, mode, targetId, indexStr] = match
  const index = indexStr ? parseInt(indexStr, 10) : undefined

  if (!doc.blocks.has(blockId)) {
    throw new UclExecutionError(`Block not found: ${blockId}`)
  }
  if (!doc.blocks.has(targetId)) {
    throw new UclExecutionError(`Target block not found: ${targetId}`)
  }

  if (mode.toUpperCase() === 'TO') {
    moveBlock(doc, blockId, targetId, index)
  } else {
    const parentId = findParent(doc, targetId)
    if (!parentId) throw new UclExecutionError(`Target ${targetId} has no parent`)
    
    const parent = doc.blocks.get(parentId)!
    const siblingIndex = parent.children.indexOf(targetId)
    if (siblingIndex === -1) {
      throw new UclExecutionError(`Target ${targetId} not found in parent's children`)
    }
    
    const insertIndex = mode.toUpperCase() === 'AFTER' ? siblingIndex + 1 : siblingIndex
    moveBlock(doc, blockId, parentId, insertIndex)
  }

  return [blockId]
}

function executeDelete(doc: Document, line: string): BlockId[] {
  const match = line.match(/^DELETE\s+(\S+)(\s+CASCADE)?$/i)
  if (!match) throw new UclParseError(`Malformed DELETE command: ${line}`)

  const [, blockId, cascade] = match
  
  if (!doc.blocks.has(blockId)) {
    throw new UclExecutionError(`Block not found: ${blockId}`)
  }

  deleteBlock(doc, blockId, { cascade: !!cascade })
  return [blockId]
}

function executeLink(doc: Document, line: string): BlockId[] {
  const match = line.match(/^LINK\s+(\S+)\s+(\w+)\s+(\S+)$/i)
  if (!match) throw new UclParseError(`Malformed LINK command: ${line}`)

  const [, sourceId, edgeTypeStr, targetId] = match
  
  if (!doc.blocks.has(sourceId)) {
    throw new UclExecutionError(`Source block not found: ${sourceId}`)
  }
  if (!doc.blocks.has(targetId)) {
    throw new UclExecutionError(`Target block not found: ${targetId}`)
  }

  addEdge(doc, sourceId, edgeTypeStr.toLowerCase() as EdgeType, targetId)
  return [sourceId, targetId]
}

function executeUnlink(doc: Document, line: string): BlockId[] {
  const match = line.match(/^UNLINK\s+(\S+)\s+(\w+)\s+(\S+)$/i)
  if (!match) throw new UclParseError(`Malformed UNLINK command: ${line}`)

  const [, sourceId, edgeTypeStr, targetId] = match
  
  if (!doc.blocks.has(sourceId)) {
    throw new UclExecutionError(`Source block not found: ${sourceId}`)
  }

  removeEdge(doc, sourceId, edgeTypeStr.toLowerCase() as EdgeType, targetId)
  return [sourceId, targetId]
}

function executePrune(doc: Document, _line: string): BlockId[] {
  return pruneOrphans(doc)
}

// =============================================================================
// OBSERVABILITY
// =============================================================================

/** Event types for observability */
export type EventType =
  | 'document.created' | 'document.modified'
  | 'block.added' | 'block.edited' | 'block.moved' | 'block.deleted'
  | 'edge.added' | 'edge.removed'
  | 'tag.added' | 'tag.removed'
  | 'ucl.parsed' | 'ucl.executed' | 'ucl.error'
  | 'validation.started' | 'validation.completed'
  | 'transaction.started' | 'transaction.committed' | 'transaction.rolled_back'
  | 'snapshot.created' | 'snapshot.restored'

/** UCP event */
export interface UcpEvent {
  type: EventType
  timestamp: Date
  data: Record<string, unknown>
}

/** Event handler type */
export type EventHandler = (event: UcpEvent) => void

/** Simple event bus for observability */
export class EventBus {
  private static instance: EventBus
  private handlers = new Map<string, EventHandler[]>()
  private globalHandlers: EventHandler[] = []

  static getInstance(): EventBus {
    if (!EventBus.instance) {
      EventBus.instance = new EventBus()
    }
    return EventBus.instance
  }

  subscribe(eventType: EventType, handler: EventHandler): void {
    const handlers = this.handlers.get(eventType) ?? []
    handlers.push(handler)
    this.handlers.set(eventType, handlers)
  }

  subscribeAll(handler: EventHandler): void {
    this.globalHandlers.push(handler)
  }

  unsubscribe(eventType: EventType, handler: EventHandler): void {
    const handlers = this.handlers.get(eventType) ?? []
    this.handlers.set(eventType, handlers.filter(h => h !== handler))
  }

  unsubscribeAll(handler: EventHandler): void {
    this.globalHandlers = this.globalHandlers.filter(h => h !== handler)
  }

  publish(event: UcpEvent): void {
    const handlers = this.handlers.get(event.type) ?? []
    handlers.forEach(h => {
      try { h(event) } catch (e) { console.error('Event handler error:', e) }
    })
    this.globalHandlers.forEach(h => {
      try { h(event) } catch (e) { console.error('Global handler error:', e) }
    })
  }

  clear(): void {
    this.handlers.clear()
    this.globalHandlers = []
  }
}

/** Emit an event to the global event bus */
export function emitEvent(type: EventType, data: Record<string, unknown> = {}): void {
  EventBus.getInstance().publish({
    type,
    timestamp: new Date(),
    data,
  })
}

/** Simple metrics collector */
export class Metrics {
  private static instance: Metrics
  private counters = new Map<string, number>()
  private gauges = new Map<string, number>()
  private histograms = new Map<string, number[]>()

  static getInstance(): Metrics {
    if (!Metrics.instance) {
      Metrics.instance = new Metrics()
    }
    return Metrics.instance
  }

  increment(name: string, value = 1): void {
    this.counters.set(name, (this.counters.get(name) ?? 0) + value)
  }

  setGauge(name: string, value: number): void {
    this.gauges.set(name, value)
  }

  recordHistogram(name: string, value: number): void {
    const values = this.histograms.get(name) ?? []
    values.push(value)
    this.histograms.set(name, values)
  }

  getCounter(name: string): number {
    return this.counters.get(name) ?? 0
  }

  getGauge(name: string): number | undefined {
    return this.gauges.get(name)
  }

  getHistogram(name: string): number[] {
    return [...(this.histograms.get(name) ?? [])]
  }

  getAll(): { counters: Record<string, number>; gauges: Record<string, number>; histograms: Record<string, number[]> } {
    return {
      counters: Object.fromEntries(this.counters),
      gauges: Object.fromEntries(this.gauges),
      histograms: Object.fromEntries(this.histograms),
    }
  }

  reset(): void {
    this.counters.clear()
    this.gauges.clear()
    this.histograms.clear()
  }
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

  /** Execute UCL commands */
  execute: executeUcl,

  /** Validate a document */
  validate: validateDocument,

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
