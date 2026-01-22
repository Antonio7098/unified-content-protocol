/**
 * SelectionManager - Handles selection state and navigation.
 *
 * Manages block selection, text selection, and keyboard navigation.
 */

import type { Document, ContentType, EdgeType } from 'ucp-content'

// Type alias for BlockId since it's not exported
type BlockId = string
import type { SelectionState, TextSelection } from '../types/editor.js'
import { Logger } from './Logger.js'

const logger = new Logger({ context: 'SelectionManager' })

// =============================================================================
// SELECTION HELPERS
// =============================================================================

/**
 * Create an empty selection state.
 */
export function createEmptySelection(): SelectionState {
  return {
    type: 'none',
    blocks: undefined,
    text: undefined,
    focusedBlockId: undefined,
  }
}

/**
 * Create a single block selection.
 */
export function createBlockSelection(blockId: BlockId): SelectionState {
  return {
    type: 'block',
    blocks: {
      blockIds: [blockId],
      anchor: blockId,
      focus: blockId,
    },
    focusedBlockId: blockId,
  }
}

/**
 * Create a multi-block selection.
 */
export function createMultiBlockSelection(
  blockIds: BlockId[],
  anchor?: BlockId,
  focus?: BlockId
): SelectionState {
  if (blockIds.length === 0) {
    return createEmptySelection()
  }

  return {
    type: 'block',
    blocks: {
      blockIds,
      anchor: anchor ?? blockIds[0],
      focus: focus ?? blockIds[blockIds.length - 1],
    },
    focusedBlockId: focus ?? blockIds[blockIds.length - 1],
  }
}

/**
 * Create a text selection within a block.
 */
export function createTextSelection(blockId: BlockId, start: number, end: number): SelectionState {
  return {
    type: 'text',
    text: {
      blockId,
      start: Math.min(start, end),
      end: Math.max(start, end),
    },
    focusedBlockId: blockId,
  }
}

// =============================================================================
// SELECTION QUERIES
// =============================================================================

/**
 * Check if a block is selected.
 */
export function isBlockSelected(selection: SelectionState, blockId: BlockId): boolean {
  if (selection.type === 'none') return false
  if (selection.type === 'text') return selection.text?.blockId === blockId
  if (selection.type === 'block') return selection.blocks?.blockIds.includes(blockId) ?? false
  return false
}

/**
 * Check if a block is the focused block.
 */
export function isBlockFocused(selection: SelectionState, blockId: BlockId): boolean {
  return selection.focusedBlockId === blockId
}

/**
 * Get all selected block IDs.
 */
export function getSelectedBlockIds(selection: SelectionState): BlockId[] {
  if (selection.type === 'none') return []
  if (selection.type === 'text') return selection.text ? [selection.text.blockId] : []
  if (selection.type === 'block') return selection.blocks?.blockIds ?? []
  return []
}

/**
 * Get the primary selected block (anchor).
 */
export function getPrimarySelectedBlock(selection: SelectionState): BlockId | undefined {
  if (selection.type === 'none') return undefined
  if (selection.type === 'text') return selection.text?.blockId
  if (selection.type === 'block') return selection.blocks?.anchor
  return undefined
}

/**
 * Get text selection details.
 */
export function getTextSelection(selection: SelectionState): TextSelection | undefined {
  return selection.type === 'text' ? selection.text : undefined
}

/**
 * Check if selection is empty.
 */
export function isSelectionEmpty(selection: SelectionState): boolean {
  return selection.type === 'none'
}

/**
 * Check if selection is a text selection.
 */
export function isTextSelection(selection: SelectionState): boolean {
  return selection.type === 'text'
}

/**
 * Check if selection is a block selection.
 */
export function isBlockSelectionType(selection: SelectionState): boolean {
  return selection.type === 'block'
}

// =============================================================================
// NAVIGATION
// =============================================================================

/**
 * Get the block order for navigation.
 */
export function getBlockOrder(doc: Document): BlockId[] {
  const order: BlockId[] = []

  function traverse(blockId: BlockId): void {
    order.push(blockId)
    const block = doc.blocks.get(blockId)
    if (block) {
      for (const childId of block.children) {
        traverse(childId)
      }
    }
  }

  traverse(doc.root)
  return order
}

/**
 * Get the next block in document order.
 */
export function getNextBlock(doc: Document, currentId: BlockId): BlockId | undefined {
  const order = getBlockOrder(doc)
  const currentIndex = order.indexOf(currentId)
  if (currentIndex === -1 || currentIndex === order.length - 1) {
    return undefined
  }
  return order[currentIndex + 1]
}

/**
 * Get the previous block in document order.
 */
export function getPreviousBlock(doc: Document, currentId: BlockId): BlockId | undefined {
  const order = getBlockOrder(doc)
  const currentIndex = order.indexOf(currentId)
  if (currentIndex <= 0) {
    return undefined
  }
  return order[currentIndex - 1]
}

/**
 * Get the parent block.
 */
export function getParentBlock(doc: Document, blockId: BlockId): BlockId | undefined {
  for (const [id, block] of doc.blocks) {
    if (block.children.includes(blockId)) {
      return id
    }
  }
  return undefined
}

/**
 * Get the first child block.
 */
export function getFirstChildBlock(doc: Document, blockId: BlockId): BlockId | undefined {
  const block = doc.blocks.get(blockId)
  return block?.children[0]
}

/**
 * Get sibling blocks.
 */
export function getSiblingBlocks(doc: Document, blockId: BlockId): BlockId[] {
  const parentId = getParentBlock(doc, blockId)
  if (!parentId) return []
  const parent = doc.blocks.get(parentId)
  return parent?.children ?? []
}

/**
 * Get the next sibling.
 */
export function getNextSibling(doc: Document, blockId: BlockId): BlockId | undefined {
  const siblings = getSiblingBlocks(doc, blockId)
  const index = siblings.indexOf(blockId)
  if (index === -1 || index === siblings.length - 1) {
    return undefined
  }
  return siblings[index + 1]
}

/**
 * Get the previous sibling.
 */
export function getPreviousSibling(doc: Document, blockId: BlockId): BlockId | undefined {
  const siblings = getSiblingBlocks(doc, blockId)
  const index = siblings.indexOf(blockId)
  if (index <= 0) {
    return undefined
  }
  return siblings[index - 1]
}

// =============================================================================
// SELECTION EXPANSION
// =============================================================================

/**
 * Expand selection to include another block.
 */
export function expandSelection(
  doc: Document,
  selection: SelectionState,
  targetId: BlockId
): SelectionState {
  if (selection.type === 'none') {
    return createBlockSelection(targetId)
  }

  if (selection.type === 'text') {
    // Convert to block selection
    const currentBlockId = selection.text?.blockId
    if (!currentBlockId) return createBlockSelection(targetId)

    return expandBlockRange(doc, currentBlockId, targetId)
  }

  if (selection.type === 'block') {
    const anchor = selection.blocks?.anchor
    if (!anchor) return createBlockSelection(targetId)

    return expandBlockRange(doc, anchor, targetId)
  }

  return selection
}

/**
 * Expand selection between two blocks.
 */
function expandBlockRange(doc: Document, anchorId: BlockId, focusId: BlockId): SelectionState {
  const order = getBlockOrder(doc)
  const anchorIndex = order.indexOf(anchorId)
  const focusIndex = order.indexOf(focusId)

  if (anchorIndex === -1 || focusIndex === -1) {
    return createBlockSelection(focusId)
  }

  const start = Math.min(anchorIndex, focusIndex)
  const end = Math.max(anchorIndex, focusIndex)
  const selectedIds = order.slice(start, end + 1)

  return createMultiBlockSelection(selectedIds, anchorId, focusId)
}

// =============================================================================
// SELECTION MANAGER CLASS
// =============================================================================

export interface SelectionManagerConfig {
  onSelectionChange?: (selection: SelectionState) => void
}

/**
 * Selection manager for handling selection state and navigation.
 *
 * @example
 * ```typescript
 * const manager = new SelectionManager(doc, {
 *   onSelectionChange: (selection) => {
 *     console.log('Selection changed:', selection)
 *   }
 * })
 *
 * manager.select('blk_123')
 * manager.moveNext()
 * manager.expandToBlock('blk_456')
 * ```
 */
export class SelectionManager {
  private doc: Document
  private selection: SelectionState
  private config: SelectionManagerConfig

  constructor(doc: Document, config: SelectionManagerConfig = {}) {
    this.doc = doc
    this.selection = createEmptySelection()
    this.config = config
  }

  /**
   * Update the document reference.
   */
  setDocument(doc: Document): void {
    this.doc = doc
    // Clear selection if document changed significantly
    if (this.selection.focusedBlockId && !doc.blocks.has(this.selection.focusedBlockId)) {
      this.clear()
    }
  }

  /**
   * Get current selection.
   */
  getSelection(): SelectionState {
    return this.selection
  }

  /**
   * Update selection and notify.
   */
  private setSelection(selection: SelectionState): void {
    this.selection = selection
    this.config.onSelectionChange?.(selection)
  }

  /**
   * Select a single block.
   */
  select(blockId: BlockId): void {
    if (!this.doc.blocks.has(blockId)) {
      logger.warn('Attempted to select non-existent block', { blockId })
      return
    }
    this.setSelection(createBlockSelection(blockId))
  }

  /**
   * Select multiple blocks.
   */
  selectMultiple(blockIds: BlockId[]): void {
    const validIds = blockIds.filter((id) => this.doc.blocks.has(id))
    if (validIds.length === 0) {
      this.clear()
      return
    }
    this.setSelection(createMultiBlockSelection(validIds))
  }

  /**
   * Clear selection.
   */
  clear(): void {
    this.setSelection(createEmptySelection())
  }

  /**
   * Select text within a block.
   */
  selectText(blockId: BlockId, start: number, end: number): void {
    if (!this.doc.blocks.has(blockId)) {
      logger.warn('Attempted to select text in non-existent block', { blockId })
      return
    }
    this.setSelection(createTextSelection(blockId, start, end))
  }

  /**
   * Expand selection to include another block.
   */
  expandTo(blockId: BlockId): void {
    if (!this.doc.blocks.has(blockId)) return
    this.setSelection(expandSelection(this.doc, this.selection, blockId))
  }

  /**
   * Move selection to next block.
   */
  moveNext(): boolean {
    const currentId = this.selection.focusedBlockId
    if (!currentId) {
      const order = getBlockOrder(this.doc)
      if (order.length > 1) {
        this.select(order[1]!) // Skip root
        return true
      }
      return false
    }

    const nextId = getNextBlock(this.doc, currentId)
    if (nextId) {
      this.select(nextId)
      return true
    }
    return false
  }

  /**
   * Move selection to previous block.
   */
  movePrevious(): boolean {
    const currentId = this.selection.focusedBlockId
    if (!currentId) return false

    const prevId = getPreviousBlock(this.doc, currentId)
    if (prevId && prevId !== this.doc.root) {
      this.select(prevId)
      return true
    }
    return false
  }

  /**
   * Move selection to parent block.
   */
  moveToParent(): boolean {
    const currentId = this.selection.focusedBlockId
    if (!currentId) return false

    const parentId = getParentBlock(this.doc, currentId)
    if (parentId && parentId !== this.doc.root) {
      this.select(parentId)
      return true
    }
    return false
  }

  /**
   * Move selection to first child.
   */
  moveToFirstChild(): boolean {
    const currentId = this.selection.focusedBlockId
    if (!currentId) return false

    const childId = getFirstChildBlock(this.doc, currentId)
    if (childId) {
      this.select(childId)
      return true
    }
    return false
  }

  /**
   * Expand selection to next block.
   */
  expandNext(): boolean {
    const currentId = this.selection.focusedBlockId
    if (!currentId) return this.moveNext()

    const nextId = getNextBlock(this.doc, currentId)
    if (nextId) {
      this.expandTo(nextId)
      return true
    }
    return false
  }

  /**
   * Expand selection to previous block.
   */
  expandPrevious(): boolean {
    const currentId = this.selection.focusedBlockId
    if (!currentId) return this.movePrevious()

    const prevId = getPreviousBlock(this.doc, currentId)
    if (prevId && prevId !== this.doc.root) {
      this.expandTo(prevId)
      return true
    }
    return false
  }

  /**
   * Select all blocks.
   */
  selectAll(): void {
    const order = getBlockOrder(this.doc)
    // Exclude root
    const blockIds = order.slice(1)
    if (blockIds.length > 0) {
      this.setSelection(createMultiBlockSelection(blockIds))
    }
  }

  /**
   * Check if block is selected.
   */
  isSelected(blockId: BlockId): boolean {
    return isBlockSelected(this.selection, blockId)
  }

  /**
   * Check if block is focused.
   */
  isFocused(blockId: BlockId): boolean {
    return isBlockFocused(this.selection, blockId)
  }
}
