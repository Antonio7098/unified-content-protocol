/**
 * Section-based operations for UCM documents.
 *
 * This module provides utilities for section-based markdown writing,
 * allowing efficient bulk updates to document sections.
 */

import type { BlockId, Document, Block } from './index.js'
import { parseMarkdown, addBlock } from './index.js'

/** Result of a section write operation */
export interface SectionWriteResult {
  success: boolean
  sectionId: BlockId
  blocksRemoved: BlockId[]
  blocksAdded: BlockId[]
  error?: string
}

/** Deleted content snapshot for undo operations */
export interface DeletedSectionContent {
  parentId: BlockId
  blocks: Record<BlockId, Block>
  structure: Record<BlockId, BlockId[]>
  deletedAt: number
}

/** Result of clearing a section with undo support */
export interface ClearSectionResult {
  removedIds: BlockId[]
  deletedContent: DeletedSectionContent
}

/**
 * Write markdown content to a section, replacing all children.
 *
 * @param doc - The document to modify
 * @param sectionId - The block ID of the section heading
 * @param markdown - Markdown content to write
 * @param baseHeadingLevel - Optional base level for heading adjustment
 * @returns SectionWriteResult with details of the operation
 *
 * @example
 * ```typescript
 * const result = writeSection(doc, 'blk_123', '## New Section\n\nContent here')
 * console.log(`Added ${result.blocksAdded.length} blocks`)
 * ```
 */
export function writeSection(
  doc: Document,
  sectionId: BlockId,
  markdown: string,
  baseHeadingLevel?: number
): SectionWriteResult {
  // Verify section exists
  if (!doc.blocks.has(sectionId)) {
    return {
      success: false,
      sectionId,
      blocksRemoved: [],
      blocksAdded: [],
      error: `Section not found: ${sectionId}`,
    }
  }

  // Clear existing children
  const blocksRemoved = clearSectionChildren(doc, sectionId)

  // Parse markdown into temporary structure
  const parsed = parseMarkdown(markdown)

  // Integrate parsed blocks
  const blocksAdded = integrateBlocks(doc, sectionId, parsed, baseHeadingLevel)

  return {
    success: true,
    sectionId,
    blocksRemoved,
    blocksAdded,
  }
}

/**
 * Clear a section while preserving content for undo.
 */
export function clearSectionWithUndo(doc: Document, sectionId: BlockId): ClearSectionResult {
  if (!doc.blocks.has(sectionId)) {
    throw new Error(`Section not found: ${sectionId}`)
  }

  const deletedContent = captureSectionContent(doc, sectionId)
  const removedIds = clearSectionChildren(doc, sectionId)
  return { removedIds, deletedContent }
}

/**
 * Restore section content from a previously captured snapshot.
 */
export function restoreDeletedSection(doc: Document, content: DeletedSectionContent): BlockId[] {
  if (!doc.blocks.has(content.parentId)) {
    throw new Error(`Parent section not found: ${content.parentId}`)
  }

  // Remove current content under the parent
  const parent = doc.blocks.get(content.parentId)!
  for (const childId of [...parent.children]) {
    removeSubtree(doc, childId)
  }
  parent.children = []

  const restoredIds: BlockId[] = []

  // Restore blocks
  for (const [blockId, block] of Object.entries(content.blocks)) {
    doc.blocks.set(blockId, cloneBlock(block))
    restoredIds.push(blockId)
  }

  // Restore structure for deleted blocks
  for (const [blockId, children] of Object.entries(content.structure)) {
    if (blockId === content.parentId) {
      continue
    }
    const block = doc.blocks.get(blockId)
    if (block) {
      block.children = [...children]
    }
  }

  // Restore parent's children list
  const parentChildren = content.structure[content.parentId] ?? []
  parent.children = [...parentChildren]

  return restoredIds
}

/**
 * Clear all children of a section recursively.
 */
function clearSectionChildren(doc: Document, sectionId: BlockId): BlockId[] {
  const removed: BlockId[] = []
  const sectionBlock = doc.blocks.get(sectionId)
  if (!sectionBlock) {
    return removed
  }

  for (const childId of [...sectionBlock.children]) {
    removed.push(...removeSubtree(doc, childId))
  }

  sectionBlock.children = []
  return removed
}

/**
 * Integrate blocks from source document into target parent.
 */
function integrateBlocks(
  doc: Document,
  parentId: BlockId,
  sourceDoc: Document,
  baseHeadingLevel?: number
): BlockId[] {
  const added: BlockId[] = []
  const rootBlock = sourceDoc.blocks.get(sourceDoc.root)
  const rootChildren = rootBlock?.children || []

  for (const childId of rootChildren) {
    const integrated = integrateSubtree(doc, parentId, sourceDoc, childId, baseHeadingLevel, 0)
    added.push(...integrated)
  }

  return added
}

function captureSectionContent(doc: Document, sectionId: BlockId): DeletedSectionContent {
  const deleted: DeletedSectionContent = {
    parentId: sectionId,
    blocks: {},
    structure: {},
    deletedAt: Date.now(),
  }

  const queue: BlockId[] = []
  const parent = doc.blocks.get(sectionId)
  const parentChildren = parent ? [...parent.children] : []
  deleted.structure[sectionId] = [...parentChildren]
  queue.push(...parentChildren)

  while (queue.length > 0) {
    const current = queue.shift()!
    const block = doc.blocks.get(current)
    if (block) {
      deleted.blocks[current] = cloneBlock(block)
      deleted.structure[current] = [...block.children]
      queue.push(...block.children)
    }
  }

  return deleted
}

function cloneBlock(block: Block): Block {
  return {
    id: block.id,
    content: block.content,
    type: block.type,
    role: block.role,
    label: block.label,
    tags: [...block.tags],
    children: [...block.children],
    edges: block.edges.map(edge => ({
      edgeType: edge.edgeType,
      target: edge.target,
      metadata: edge.metadata
        ? {
            confidence: edge.metadata.confidence,
            description: edge.metadata.description,
            custom: edge.metadata.custom ? { ...edge.metadata.custom } : {},
          }
        : { custom: {} },
      createdAt: edge.createdAt instanceof Date ? new Date(edge.createdAt) : new Date(edge.createdAt),
    })),
    metadata: block.metadata
      ? {
          ...block.metadata,
          tags: [...block.metadata.tags],
          custom: block.metadata.custom ? { ...block.metadata.custom } : {},
          createdAt:
            block.metadata.createdAt instanceof Date
              ? new Date(block.metadata.createdAt)
              : new Date(block.metadata.createdAt ?? Date.now()),
          modifiedAt:
            block.metadata.modifiedAt instanceof Date
              ? new Date(block.metadata.modifiedAt)
              : new Date(block.metadata.modifiedAt ?? Date.now()),
        }
      : undefined,
  }
}

function removeSubtree(doc: Document, blockId: BlockId): BlockId[] {
  const removed: BlockId[] = []
  const block = doc.blocks.get(blockId)
  if (!block) {
    return removed
  }

  for (const childId of [...block.children]) {
    removed.push(...removeSubtree(doc, childId))
  }

  const parentId = findParentId(doc, blockId)
  if (parentId) {
    const parent = doc.blocks.get(parentId)
    if (parent) {
      parent.children = parent.children.filter(child => child !== blockId)
    }
  }

  doc.blocks.delete(blockId)
  removed.push(blockId)
  return removed
}

function findParentId(doc: Document, childId: BlockId): BlockId | undefined {
  for (const [id, block] of doc.blocks.entries()) {
    if (block.children.includes(childId)) {
      return id
    }
  }
  return undefined
}

/**
 * Recursively integrate a subtree.
 */
function integrateSubtree(
  doc: Document,
  parentId: BlockId,
  sourceDoc: Document,
  sourceBlockId: BlockId,
  baseHeadingLevel: number | undefined,
  depth: number
): BlockId[] {
  const added: BlockId[] = []
  const sourceBlock = sourceDoc.blocks.get(sourceBlockId)

  if (!sourceBlock) {
    return added
  }

  // Get content and role
  let content = sourceBlock.content
  let role = sourceBlock.metadata?.semanticRole

  // Adjust heading level if specified
  if (baseHeadingLevel && role && role.startsWith('heading')) {
    const currentLevel = parseInt(role.substring(7), 10)
    if (!isNaN(currentLevel)) {
      const newLevel = Math.min(6, Math.max(1, baseHeadingLevel + currentLevel - 1))
      role = `heading${newLevel}` as any
    }
  }

  // Add block to document
  const newId = addBlock(doc, parentId, content, { role })
  added.push(newId)

  // Process children
  const sourceBlockData = sourceDoc.blocks.get(sourceBlockId)
  const children = sourceBlockData?.children || []
  for (const childId of children) {
    const childAdded = integrateSubtree(doc, newId, sourceDoc, childId, baseHeadingLevel, depth + 1)
    added.push(...childAdded)
  }

  return added
}

/**
 * Find a section by its path in the document hierarchy.
 *
 * @param doc - The document to search
 * @param path - Path like "Introduction > Getting Started"
 * @returns Block ID of the section, or undefined if not found
 *
 * @example
 * ```typescript
 * const sectionId = findSectionByPath(doc, 'Chapter 1 > Section 1.1')
 * ```
 */
export function findSectionByPath(doc: Document, path: string): BlockId | undefined {
  const parts = path.split(' > ').map(p => p.trim())
  if (parts.length === 0) {
    return undefined
  }

  let currentId = doc.root

  for (const part of parts) {
    const currentBlock = doc.blocks.get(currentId)
  const children = currentBlock?.children || []
    let found: BlockId | undefined

    for (const childId of children) {
      const block = doc.blocks.get(childId)
      if (!block) continue

      const role = block.metadata?.semanticRole || ''
      if (role.startsWith('heading')) {
        const content = typeof block.content === 'string' ? block.content : ''
        if (content.trim() === part) {
          found = childId
          break
        }
      }
    }

    if (!found) {
      return undefined
    }
    currentId = found
  }

  return currentId !== doc.root ? currentId : undefined
}

/**
 * Get all sections (heading blocks) in the document.
 *
 * @param doc - The document to search
 * @returns Array of [blockId, headingLevel] tuples
 *
 * @example
 * ```typescript
 * const sections = getAllSections(doc)
 * for (const [blockId, level] of sections) {
 *   console.log(`H${level}: ${doc.blocks.get(blockId)?.content}`)
 * }
 * ```
 */
export function getAllSections(doc: Document): Array<[BlockId, number]> {
  const sections: Array<[BlockId, number]> = []

  for (const [blockId, block] of doc.blocks) {
    const role = block.metadata?.semanticRole || ''
    if (role.startsWith('heading')) {
      const level = parseInt(role.substring(7), 10)
      if (!isNaN(level)) {
        sections.push([blockId, level])
      }
    }
  }

  return sections
}
