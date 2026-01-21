/**
 * DiffEngine - Computes differences between document snapshots.
 *
 * Provides block-level and text-level diffing for UCM documents.
 */

import type { BlockId, Document, Block } from 'ucp-js'
import type {
  DocumentDiff,
  BlockDiff,
  TextDiff,
  StructuralChange,
  DiffChangeType,
  MetadataChange,
} from '../types/editor.js'
import { Logger } from './Logger.js'

const logger = new Logger({ context: 'DiffEngine' })

// =============================================================================
// TEXT DIFF ALGORITHM
// =============================================================================

type DiffOperation =
  | { type: 'equal'; text: string }
  | { type: 'insert'; text: string }
  | { type: 'delete'; text: string }

/**
 * Simple line-based diff algorithm.
 * For production, consider using a more sophisticated algorithm like Myers diff.
 */
function computeTextDiff(oldText: string, newText: string): TextDiff {
  if (oldText === newText) {
    return { operations: [{ type: 'equal', text: oldText }] }
  }

  const operations: DiffOperation[] = []

  // Simple character-based diff for short texts
  if (oldText.length < 1000 && newText.length < 1000) {
    const result = computeCharacterDiff(oldText, newText)
    return { operations: result }
  }

  // Line-based diff for longer texts
  const oldLines = oldText.split('\n')
  const newLines = newText.split('\n')

  const lcs = computeLCS(oldLines, newLines)
  let oldIdx = 0
  let newIdx = 0
  let lcsIdx = 0

  while (oldIdx < oldLines.length || newIdx < newLines.length) {
    if (lcsIdx < lcs.length && oldLines[oldIdx] === lcs[lcsIdx] && newLines[newIdx] === lcs[lcsIdx]) {
      // Common line
      operations.push({ type: 'equal', text: oldLines[oldIdx]! + '\n' })
      oldIdx++
      newIdx++
      lcsIdx++
    } else if (oldIdx < oldLines.length && (lcsIdx >= lcs.length || oldLines[oldIdx] !== lcs[lcsIdx])) {
      // Deleted line
      operations.push({ type: 'delete', text: oldLines[oldIdx]! + '\n' })
      oldIdx++
    } else if (newIdx < newLines.length) {
      // Inserted line
      operations.push({ type: 'insert', text: newLines[newIdx]! + '\n' })
      newIdx++
    }
  }

  // Merge consecutive operations of the same type
  return { operations: mergeOperations(operations) }
}

/**
 * Character-based diff for short strings.
 */
function computeCharacterDiff(oldText: string, newText: string): DiffOperation[] {
  const operations: DiffOperation[] = []

  // Find common prefix
  let prefixLen = 0
  while (prefixLen < oldText.length && prefixLen < newText.length && oldText[prefixLen] === newText[prefixLen]) {
    prefixLen++
  }

  // Find common suffix
  let suffixLen = 0
  while (
    suffixLen < oldText.length - prefixLen &&
    suffixLen < newText.length - prefixLen &&
    oldText[oldText.length - 1 - suffixLen] === newText[newText.length - 1 - suffixLen]
  ) {
    suffixLen++
  }

  // Add prefix
  if (prefixLen > 0) {
    operations.push({ type: 'equal', text: oldText.slice(0, prefixLen) })
  }

  // Add middle (deleted and inserted)
  const oldMiddle = oldText.slice(prefixLen, oldText.length - suffixLen)
  const newMiddle = newText.slice(prefixLen, newText.length - suffixLen)

  if (oldMiddle.length > 0) {
    operations.push({ type: 'delete', text: oldMiddle })
  }
  if (newMiddle.length > 0) {
    operations.push({ type: 'insert', text: newMiddle })
  }

  // Add suffix
  if (suffixLen > 0) {
    operations.push({ type: 'equal', text: oldText.slice(oldText.length - suffixLen) })
  }

  return operations
}

/**
 * Compute Longest Common Subsequence for line-based diff.
 */
function computeLCS(a: string[], b: string[]): string[] {
  const m = a.length
  const n = b.length

  // Use a space-optimized approach for large arrays
  if (m > 1000 || n > 1000) {
    return computeLCSOptimized(a, b)
  }

  const dp: number[][] = Array(m + 1)
    .fill(null)
    .map(() => Array(n + 1).fill(0))

  for (let i = 1; i <= m; i++) {
    for (let j = 1; j <= n; j++) {
      if (a[i - 1] === b[j - 1]) {
        dp[i]![j] = dp[i - 1]![j - 1]! + 1
      } else {
        dp[i]![j] = Math.max(dp[i - 1]![j]!, dp[i]![j - 1]!)
      }
    }
  }

  // Backtrack to find LCS
  const lcs: string[] = []
  let i = m
  let j = n

  while (i > 0 && j > 0) {
    if (a[i - 1] === b[j - 1]) {
      lcs.unshift(a[i - 1]!)
      i--
      j--
    } else if (dp[i - 1]![j]! > dp[i]![j - 1]!) {
      i--
    } else {
      j--
    }
  }

  return lcs
}

/**
 * Space-optimized LCS for large arrays.
 */
function computeLCSOptimized(a: string[], b: string[]): string[] {
  // For very large arrays, use a simpler heuristic
  // This trades accuracy for memory efficiency
  const commonElements: string[] = []
  const bSet = new Set(b)

  for (const item of a) {
    if (bSet.has(item)) {
      commonElements.push(item)
    }
  }

  return commonElements
}

/**
 * Merge consecutive operations of the same type.
 */
function mergeOperations(operations: DiffOperation[]): DiffOperation[] {
  if (operations.length === 0) return []

  const merged: DiffOperation[] = []
  let current = operations[0]!

  for (let i = 1; i < operations.length; i++) {
    const op = operations[i]!
    if (op.type === current.type) {
      current = { type: current.type, text: current.text + op.text }
    } else {
      merged.push(current)
      current = op
    }
  }

  merged.push(current)
  return merged
}

// =============================================================================
// STRUCTURE DIFF
// =============================================================================

interface BlockInfo {
  id: BlockId
  parentId?: BlockId
  index: number
  content: string
  type: string
}

function getBlockInfo(doc: Document, blockId: BlockId): BlockInfo | undefined {
  const block = doc.blocks.get(blockId)
  if (!block) return undefined

  // Find parent
  let parentId: BlockId | undefined
  let index = 0

  for (const [id, b] of doc.blocks) {
    const childIndex = b.children.indexOf(blockId)
    if (childIndex !== -1) {
      parentId = id
      index = childIndex
      break
    }
  }

  return {
    id: blockId,
    parentId,
    index,
    content: block.content,
    type: block.type,
  }
}

/**
 * Compute structural changes between two documents.
 */
function computeStructuralChanges(
  oldDoc: Document,
  newDoc: Document,
  commonBlockIds: Set<BlockId>
): StructuralChange[] {
  const changes: StructuralChange[] = []

  // Check for moved/reordered blocks
  for (const blockId of commonBlockIds) {
    const oldInfo = getBlockInfo(oldDoc, blockId)
    const newInfo = getBlockInfo(newDoc, blockId)

    if (!oldInfo || !newInfo) continue

    if (oldInfo.parentId !== newInfo.parentId) {
      changes.push({
        type: 'moved',
        blockId,
        oldParentId: oldInfo.parentId,
        newParentId: newInfo.parentId,
        oldIndex: oldInfo.index,
        newIndex: newInfo.index,
      })
    } else if (oldInfo.index !== newInfo.index) {
      changes.push({
        type: 'reordered',
        blockId,
        oldParentId: oldInfo.parentId,
        newParentId: newInfo.parentId,
        oldIndex: oldInfo.index,
        newIndex: newInfo.index,
      })
    }
  }

  // Check for added blocks (not in old doc)
  for (const blockId of newDoc.blocks.keys()) {
    if (!oldDoc.blocks.has(blockId)) {
      const info = getBlockInfo(newDoc, blockId)
      if (info) {
        changes.push({
          type: 'added',
          blockId,
          newParentId: info.parentId,
          newIndex: info.index,
        })
      }
    }
  }

  // Check for removed blocks (not in new doc)
  for (const blockId of oldDoc.blocks.keys()) {
    if (!newDoc.blocks.has(blockId)) {
      const info = getBlockInfo(oldDoc, blockId)
      if (info) {
        changes.push({
          type: 'removed',
          blockId,
          oldParentId: info.parentId,
          oldIndex: info.index,
        })
      }
    }
  }

  return changes
}

// =============================================================================
// METADATA DIFF
// =============================================================================

function computeMetadataChanges(oldBlock: Block, newBlock: Block): MetadataChange[] {
  const changes: MetadataChange[] = []

  // Check type
  if (oldBlock.type !== newBlock.type) {
    changes.push({ field: 'type', oldValue: oldBlock.type, newValue: newBlock.type })
  }

  // Check role
  if (oldBlock.role !== newBlock.role) {
    changes.push({ field: 'role', oldValue: oldBlock.role, newValue: newBlock.role })
  }

  // Check label
  if (oldBlock.label !== newBlock.label) {
    changes.push({ field: 'label', oldValue: oldBlock.label, newValue: newBlock.label })
  }

  // Check tags
  const oldTags = new Set(oldBlock.tags)
  const newTags = new Set(newBlock.tags)
  const addedTags = [...newTags].filter((t) => !oldTags.has(t))
  const removedTags = [...oldTags].filter((t) => !newTags.has(t))

  if (addedTags.length > 0 || removedTags.length > 0) {
    changes.push({
      field: 'tags',
      oldValue: [...oldTags],
      newValue: [...newTags],
    })
  }

  // Check edges count (detailed edge diff is complex)
  if (oldBlock.edges.length !== newBlock.edges.length) {
    changes.push({
      field: 'edges',
      oldValue: oldBlock.edges.length,
      newValue: newBlock.edges.length,
    })
  }

  return changes
}

// =============================================================================
// BLOCK DIFF
// =============================================================================

function computeBlockDiff(oldBlock: Block | undefined, newBlock: Block | undefined): BlockDiff {
  // Added block
  if (!oldBlock && newBlock) {
    return {
      blockId: newBlock.id,
      changeType: 'added',
      newBlock,
    }
  }

  // Removed block
  if (oldBlock && !newBlock) {
    return {
      blockId: oldBlock.id,
      changeType: 'removed',
      oldBlock,
    }
  }

  // Both exist - check for modifications
  if (oldBlock && newBlock) {
    const contentChanged = oldBlock.content !== newBlock.content
    const metadataChanges = computeMetadataChanges(oldBlock, newBlock)

    if (!contentChanged && metadataChanges.length === 0) {
      return {
        blockId: oldBlock.id,
        changeType: 'unchanged',
        oldBlock,
        newBlock,
      }
    }

    return {
      blockId: oldBlock.id,
      changeType: 'modified',
      oldBlock,
      newBlock,
      contentDiff: contentChanged ? computeTextDiff(oldBlock.content, newBlock.content) : undefined,
      metadataChanges: metadataChanges.length > 0 ? metadataChanges : undefined,
    }
  }

  // Should not reach here
  throw new Error('Invalid block diff state')
}

// =============================================================================
// DOCUMENT DIFF
// =============================================================================

/**
 * Compute a complete diff between two documents.
 *
 * @example
 * ```typescript
 * const oldDoc = parseMarkdown('# Hello\n\nWorld')
 * const newDoc = parseMarkdown('# Hello\n\nNew World')
 *
 * const diff = computeDocumentDiff(oldDoc, newDoc, 'snapshot1', 'snapshot2')
 *
 * console.log(diff.summary)
 * // { added: 0, removed: 0, modified: 1, moved: 0, unchanged: 2 }
 * ```
 */
export function computeDocumentDiff(
  oldDoc: Document,
  newDoc: Document,
  fromSnapshotId: string,
  toSnapshotId: string
): DocumentDiff {
  logger.debug('Computing document diff', {
    fromSnapshot: fromSnapshotId,
    toSnapshot: toSnapshotId,
    oldBlockCount: oldDoc.blocks.size,
    newBlockCount: newDoc.blocks.size,
  })

  const blockDiffs = new Map<BlockId, BlockDiff>()
  const allBlockIds = new Set([...oldDoc.blocks.keys(), ...newDoc.blocks.keys()])
  const commonBlockIds = new Set<BlockId>()

  const summary = {
    added: 0,
    removed: 0,
    modified: 0,
    moved: 0,
    unchanged: 0,
  }

  // Compute block diffs
  for (const blockId of allBlockIds) {
    const oldBlock = oldDoc.blocks.get(blockId)
    const newBlock = newDoc.blocks.get(blockId)

    if (oldBlock && newBlock) {
      commonBlockIds.add(blockId)
    }

    const diff = computeBlockDiff(oldBlock, newBlock)
    blockDiffs.set(blockId, diff)

    switch (diff.changeType) {
      case 'added':
        summary.added++
        break
      case 'removed':
        summary.removed++
        break
      case 'modified':
        summary.modified++
        break
      case 'unchanged':
        summary.unchanged++
        break
    }
  }

  // Compute structural changes
  const structuralChanges = computeStructuralChanges(oldDoc, newDoc, commonBlockIds)
  summary.moved = structuralChanges.filter((c) => c.type === 'moved').length

  logger.debug('Diff computed', summary)

  return {
    fromSnapshotId,
    toSnapshotId,
    blockDiffs,
    structuralChanges,
    summary,
  }
}

// =============================================================================
// DIFF UTILITIES
// =============================================================================

/**
 * Get blocks that have changes.
 */
export function getChangedBlocks(diff: DocumentDiff): BlockDiff[] {
  return Array.from(diff.blockDiffs.values()).filter((d) => d.changeType !== 'unchanged')
}

/**
 * Get blocks by change type.
 */
export function getBlocksByChangeType(diff: DocumentDiff, changeType: DiffChangeType): BlockDiff[] {
  return Array.from(diff.blockDiffs.values()).filter((d) => d.changeType === changeType)
}

/**
 * Check if a specific block has changes.
 */
export function hasBlockChanged(diff: DocumentDiff, blockId: BlockId): boolean {
  const blockDiff = diff.blockDiffs.get(blockId)
  return blockDiff !== undefined && blockDiff.changeType !== 'unchanged'
}

/**
 * Get the text diff for a specific block.
 */
export function getBlockTextDiff(diff: DocumentDiff, blockId: BlockId): TextDiff | undefined {
  const blockDiff = diff.blockDiffs.get(blockId)
  return blockDiff?.contentDiff
}

/**
 * Format text diff as a string with markers.
 */
export function formatTextDiff(textDiff: TextDiff): string {
  return textDiff.operations
    .map((op) => {
      switch (op.type) {
        case 'equal':
          return op.text
        case 'insert':
          return `[+${op.text}+]`
        case 'delete':
          return `[-${op.text}-]`
      }
    })
    .join('')
}

/**
 * Check if diff has any changes.
 */
export function hasDiffChanges(diff: DocumentDiff): boolean {
  return diff.summary.added > 0 || diff.summary.removed > 0 || diff.summary.modified > 0 || diff.summary.moved > 0
}
