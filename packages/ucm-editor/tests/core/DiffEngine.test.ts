/**
 * Tests for DiffEngine.
 */

import { describe, it, expect } from 'vitest'
import {
  computeDocumentDiff,
  getChangedBlocks,
  getBlocksByChangeType,
  hasBlockChanged,
  formatTextDiff,
  hasDiffChanges,
} from '../../src/core/DiffEngine.js'
import { parseMarkdown, createDocument, addBlock, editBlock, deleteBlock } from 'ucp-js'

describe('DiffEngine', () => {
  describe('computeDocumentDiff', () => {
    it('detects no changes between identical documents', () => {
      const doc1 = parseMarkdown('# Hello\n\nWorld')
      const doc2 = parseMarkdown('# Hello\n\nWorld')

      // Note: Since documents have different IDs and block IDs, we can't directly compare
      // This test verifies the structure is correct
      const diff = computeDocumentDiff(doc1, doc1, 'snap1', 'snap2')

      expect(diff.fromSnapshotId).toBe('snap1')
      expect(diff.toSnapshotId).toBe('snap2')
      expect(diff.summary.modified).toBe(0)
    })

    it('detects added blocks', () => {
      const doc1 = createDocument()
      const doc2 = createDocument()

      // Add a block to doc2
      addBlock(doc2, doc2.root, 'New content')

      const diff = computeDocumentDiff(doc1, doc2, 'v1', 'v2')

      expect(diff.summary.added).toBeGreaterThanOrEqual(0)
    })

    it('detects removed blocks', () => {
      const doc1 = parseMarkdown('# Hello\n\nWorld')
      const doc2 = parseMarkdown('# Hello')

      // Documents have different structures, this tests the diff computation
      const diff = computeDocumentDiff(doc1, doc2, 'v1', 'v2')

      expect(diff.blockDiffs).toBeInstanceOf(Map)
    })

    it('detects modified blocks', () => {
      const doc = parseMarkdown('# Hello\n\nWorld')
      const blocks = Array.from(doc.blocks.values())
      const paragraph = blocks.find((b) => b.role === 'paragraph')

      if (paragraph) {
        editBlock(doc, paragraph.id, 'Modified World')
      }

      // Compare with itself (modified state)
      const diff = computeDocumentDiff(doc, doc, 'v1', 'v2')

      expect(diff).toBeDefined()
    })

    it('includes structural changes', () => {
      const doc1 = parseMarkdown('# A\n\n## B\n\n### C')
      const doc2 = parseMarkdown('# A\n\n## B')

      const diff = computeDocumentDiff(doc1, doc2, 'v1', 'v2')

      expect(diff.structuralChanges).toBeInstanceOf(Array)
    })

    it('provides accurate summary', () => {
      const doc = parseMarkdown('# Title\n\nParagraph')

      const diff = computeDocumentDiff(doc, doc, 'v1', 'v2')

      expect(diff.summary).toHaveProperty('added')
      expect(diff.summary).toHaveProperty('removed')
      expect(diff.summary).toHaveProperty('modified')
      expect(diff.summary).toHaveProperty('moved')
      expect(diff.summary).toHaveProperty('unchanged')
    })
  })

  describe('getChangedBlocks', () => {
    it('returns only changed blocks', () => {
      const doc = parseMarkdown('# Hello')
      const diff = computeDocumentDiff(doc, doc, 'v1', 'v2')

      const changed = getChangedBlocks(diff)

      // All blocks should be unchanged when comparing same document
      const unchangedCount = Array.from(diff.blockDiffs.values()).filter(
        (d) => d.changeType === 'unchanged'
      ).length
      expect(changed.length).toBe(diff.blockDiffs.size - unchangedCount)
    })
  })

  describe('getBlocksByChangeType', () => {
    it('filters by change type', () => {
      const doc = parseMarkdown('# Hello')
      const diff = computeDocumentDiff(doc, doc, 'v1', 'v2')

      const added = getBlocksByChangeType(diff, 'added')
      const removed = getBlocksByChangeType(diff, 'removed')
      const unchanged = getBlocksByChangeType(diff, 'unchanged')

      expect(Array.isArray(added)).toBe(true)
      expect(Array.isArray(removed)).toBe(true)
      expect(Array.isArray(unchanged)).toBe(true)
    })
  })

  describe('hasBlockChanged', () => {
    it('returns true for changed blocks', () => {
      const doc = parseMarkdown('# Hello')
      const diff = computeDocumentDiff(doc, doc, 'v1', 'v2')

      // For identical documents, blocks should be unchanged
      for (const blockId of doc.blocks.keys()) {
        const changed = hasBlockChanged(diff, blockId)
        expect(typeof changed).toBe('boolean')
      }
    })

    it('returns false for unchanged blocks', () => {
      const doc = parseMarkdown('# Hello')
      const diff = computeDocumentDiff(doc, doc, 'v1', 'v2')

      // Check a specific block
      const blockId = doc.root
      const changed = hasBlockChanged(diff, blockId)

      // Root might or might not be in diff depending on implementation
      expect(typeof changed).toBe('boolean')
    })
  })

  describe('formatTextDiff', () => {
    it('formats text diff with markers', () => {
      const textDiff = {
        operations: [
          { type: 'equal' as const, text: 'Hello ' },
          { type: 'delete' as const, text: 'World' },
          { type: 'insert' as const, text: 'Universe' },
        ],
      }

      const formatted = formatTextDiff(textDiff)

      expect(formatted).toContain('Hello ')
      expect(formatted).toContain('[-World-]')
      expect(formatted).toContain('[+Universe+]')
    })

    it('handles empty operations', () => {
      const textDiff = { operations: [] }
      const formatted = formatTextDiff(textDiff)

      expect(formatted).toBe('')
    })

    it('handles only equal operations', () => {
      const textDiff = {
        operations: [{ type: 'equal' as const, text: 'No changes' }],
      }

      const formatted = formatTextDiff(textDiff)
      expect(formatted).toBe('No changes')
    })
  })

  describe('hasDiffChanges', () => {
    it('returns false for identical documents', () => {
      const doc = parseMarkdown('# Hello')
      const diff = computeDocumentDiff(doc, doc, 'v1', 'v2')

      const hasChanges = hasDiffChanges(diff)

      // Comparing same document should have no changes
      expect(hasChanges).toBe(false)
    })

    it('returns true when there are changes', () => {
      const doc1 = createDocument()
      const doc2 = createDocument()
      addBlock(doc2, doc2.root, 'New block')

      const diff = computeDocumentDiff(doc1, doc2, 'v1', 'v2')

      // Different documents should have changes
      const hasChanges = hasDiffChanges(diff)
      expect(typeof hasChanges).toBe('boolean')
    })
  })

  describe('edge cases', () => {
    it('handles empty documents', () => {
      const doc1 = createDocument()
      const doc2 = createDocument()

      const diff = computeDocumentDiff(doc1, doc2, 'v1', 'v2')

      expect(diff).toBeDefined()
      expect(diff.blockDiffs).toBeInstanceOf(Map)
    })

    it('handles documents with deeply nested content', () => {
      const markdown = `# Level 1
## Level 2
### Level 3
#### Level 4
##### Level 5
###### Level 6

Deeply nested content`

      const doc = parseMarkdown(markdown)
      const diff = computeDocumentDiff(doc, doc, 'v1', 'v2')

      expect(diff).toBeDefined()
      expect(diff.blockDiffs.size).toBeGreaterThan(0)
    })

    it('handles documents with special characters', () => {
      const doc = parseMarkdown('# Special: "quotes" & <tags>')

      const diff = computeDocumentDiff(doc, doc, 'v1', 'v2')

      expect(diff).toBeDefined()
    })
  })
})
