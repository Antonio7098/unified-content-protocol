/**
 * Tests for EditorStore.
 */

import { describe, it, expect, beforeEach, vi } from 'vitest'
import { createEditorStore, type EditorStoreInstance } from '../../src/core/EditorStore.js'
import { parseMarkdown, createDocument } from 'ucp-content'

describe('EditorStore', () => {
  let store: EditorStoreInstance

  beforeEach(() => {
    store = createEditorStore({ logLevel: 'error' })
  })

  describe('Document Operations', () => {
    it('creates a new document', () => {
      store.createDocument('Test Document')

      expect(store.document).not.toBeNull()
      expect(store.document?.metadata?.title).toBe('Test Document')
      expect(store.documentId).not.toBeNull()
    })

    it('loads an existing document', () => {
      const doc = parseMarkdown('# Hello\n\nWorld')
      store.loadDocument(doc)

      expect(store.document).toBe(doc)
      expect(store.documentId).toBe(doc.id)
      expect(store.isDirty).toBe(false)
    })

    it('marks document as dirty after changes', () => {
      store.createDocument()
      expect(store.isDirty).toBe(false)

      store.addBlock(store.document!.root, 'Test content')
      expect(store.isDirty).toBe(true)
    })
  })

  describe('Block Operations', () => {
    beforeEach(() => {
      store.createDocument()
    })

    it('adds a block', () => {
      const initialSize = store.document!.blocks.size
      const blockId = store.addBlock(store.document!.root, 'New block')

      expect(store.document!.blocks.size).toBe(initialSize + 1)
      expect(store.document!.blocks.get(blockId)?.content).toBe('New block')
    })

    it('edits a block', () => {
      const blockId = store.addBlock(store.document!.root, 'Original')
      store.editBlock(blockId, 'Updated')

      expect(store.document!.blocks.get(blockId)?.content).toBe('Updated')
    })

    it('deletes a block', () => {
      const blockId = store.addBlock(store.document!.root, 'To delete')
      expect(store.document!.blocks.has(blockId)).toBe(true)

      store.deleteBlock(blockId)
      expect(store.document!.blocks.has(blockId)).toBe(false)
    })

    it('moves a block', () => {
      const block1 = store.addBlock(store.document!.root, 'Block 1')
      const block2 = store.addBlock(store.document!.root, 'Block 2')

      // Move block2 before block1
      store.moveBlock(block2, block1, 'before')

      const rootChildren = store.document!.blocks.get(store.document!.root)?.children
      expect(rootChildren?.[0]).toBe(block2)
      expect(rootChildren?.[1]).toBe(block1)
    })

    it('changes block type', () => {
      const blockId = store.addBlock(store.document!.root, 'Code block')
      store.changeBlockType(blockId, 'code')

      expect(store.document!.blocks.get(blockId)?.type).toBe('code')
    })
  })

  describe('Selection', () => {
    beforeEach(() => {
      const doc = parseMarkdown('# Title\n\nParagraph 1\n\nParagraph 2')
      store.loadDocument(doc)
    })

    it('selects a single block', () => {
      const blocks = Array.from(store.document!.blocks.values())
      const paragraph = blocks.find((b) => b.role === 'paragraph')!

      store.select(paragraph.id)

      expect(store.selection.type).toBe('block')
      expect(store.selection.focusedBlockId).toBe(paragraph.id)
    })

    it('selects multiple blocks', () => {
      const blocks = Array.from(store.document!.blocks.values())
      const paragraphs = blocks.filter((b) => b.role === 'paragraph')
      const ids = paragraphs.map((p) => p.id)

      store.selectMultiple(ids)

      expect(store.selection.type).toBe('block')
      expect(store.selection.blocks?.blockIds).toEqual(ids)
    })

    it('clears selection', () => {
      const blocks = Array.from(store.document!.blocks.values())
      const paragraph = blocks.find((b) => b.role === 'paragraph')!

      store.select(paragraph.id)
      expect(store.selection.type).toBe('block')

      store.clearSelection()
      expect(store.selection.type).toBe('none')
    })
  })

  describe('Editing', () => {
    beforeEach(() => {
      store.createDocument()
    })

    it('starts and stops editing', () => {
      const blockId = store.addBlock(store.document!.root, 'Edit me')

      store.startEditing(blockId)
      expect(store.editingBlockId).toBe(blockId)
      expect(store.editState).toBe('editing')
      expect(store.mode).toBe('edit')

      store.stopEditing(false)
      expect(store.editingBlockId).toBeNull()
      expect(store.editState).toBe('idle')
      expect(store.mode).toBe('view')
    })

    it('saves content when stopping edit', () => {
      const blockId = store.addBlock(store.document!.root, 'Original')

      store.startEditing(blockId)
      store.updatePendingContent('Modified')
      store.stopEditing(true)

      expect(store.document!.blocks.get(blockId)?.content).toBe('Modified')
    })

    it('discards content when canceling edit', () => {
      const blockId = store.addBlock(store.document!.root, 'Original')

      store.startEditing(blockId)
      store.updatePendingContent('Modified')
      store.stopEditing(false)

      expect(store.document!.blocks.get(blockId)?.content).toBe('Original')
    })
  })

  describe('History (Undo/Redo)', () => {
    beforeEach(() => {
      store.createDocument()
    })

    it('tracks history for block operations', () => {
      expect(store.history.canUndo).toBe(false)

      store.addBlock(store.document!.root, 'Block 1')
      expect(store.history.canUndo).toBe(true)
    })

    it('undoes block addition', () => {
      const initialSize = store.document!.blocks.size
      store.addBlock(store.document!.root, 'Block 1')
      expect(store.document!.blocks.size).toBe(initialSize + 1)

      store.undo()
      expect(store.document!.blocks.size).toBe(initialSize)
    })

    it('redoes undone operation', () => {
      const initialSize = store.document!.blocks.size
      store.addBlock(store.document!.root, 'Block 1')
      store.undo()
      expect(store.document!.blocks.size).toBe(initialSize)

      store.redo()
      expect(store.document!.blocks.size).toBe(initialSize + 1)
    })
  })

  describe('Drag and Drop', () => {
    beforeEach(() => {
      store.createDocument()
      store.addBlock(store.document!.root, 'Block 1')
      store.addBlock(store.document!.root, 'Block 2')
    })

    it('starts and ends drag', () => {
      const blocks = Array.from(store.document!.blocks.values())
      const block = blocks.find((b) => b.content === 'Block 1')!

      store.startDrag(block.id)
      expect(store.drag.isDragging).toBe(true)
      expect(store.drag.sourceId).toBe(block.id)

      store.endDrag(false)
      expect(store.drag.isDragging).toBe(false)
    })

    it('updates drag target', () => {
      const blocks = Array.from(store.document!.blocks.values())
      const block1 = blocks.find((b) => b.content === 'Block 1')!
      const block2 = blocks.find((b) => b.content === 'Block 2')!

      store.startDrag(block1.id)
      store.updateDragTarget(block2.id, 'after')

      expect(store.drag.targetId).toBe(block2.id)
      expect(store.drag.position).toBe('after')
    })
  })

  describe('View and Mode', () => {
    it('changes view', () => {
      expect(store.view).toBe('document')

      store.setView('graph')
      expect(store.view).toBe('graph')

      store.setView('diff')
      expect(store.view).toBe('diff')
    })

    it('changes mode', () => {
      expect(store.mode).toBe('view')

      store.setMode('edit')
      expect(store.mode).toBe('edit')

      store.setMode('view')
      expect(store.mode).toBe('view')
    })
  })

  describe('Edge Operations', () => {
    beforeEach(() => {
      store.createDocument()
    })

    it('adds an edge between blocks', () => {
      const block1 = store.addBlock(store.document!.root, 'Block 1')
      const block2 = store.addBlock(store.document!.root, 'Block 2')

      store.addEdge(block1, block2, 'references')

      const sourceBlock = store.document!.blocks.get(block1)
      expect(sourceBlock?.edges.some((e) => e.target === block2 && e.edgeType === 'references')).toBe(
        true
      )
    })

    it('removes an edge', () => {
      const block1 = store.addBlock(store.document!.root, 'Block 1')
      const block2 = store.addBlock(store.document!.root, 'Block 2')

      store.addEdge(block1, block2, 'references')
      store.removeEdge(block1, block2, 'references')

      const sourceBlock = store.document!.blocks.get(block1)
      expect(sourceBlock?.edges.some((e) => e.target === block2)).toBe(false)
    })
  })

  describe('Events', () => {
    it('emits events on document load', () => {
      const handler = vi.fn()
      store.events.on('document:loaded', handler)

      const doc = parseMarkdown('# Test')
      store.loadDocument(doc)

      expect(handler).toHaveBeenCalledTimes(1)
      expect(handler.mock.calls[0][0].data.documentId).toBe(doc.id)
    })

    it('emits events on block add', () => {
      store.createDocument()
      const handler = vi.fn()
      store.events.on('block:added', handler)

      const blockId = store.addBlock(store.document!.root, 'New block')

      expect(handler).toHaveBeenCalledTimes(1)
      expect(handler.mock.calls[0][0].data.blockId).toBe(blockId)
    })
  })

  describe('State Subscription', () => {
    it('notifies subscribers on state change', () => {
      const listener = vi.fn()
      store.subscribe(listener)

      store.createDocument()

      expect(listener).toHaveBeenCalled()
    })

    it('allows unsubscribing', () => {
      const listener = vi.fn()
      const unsubscribe = store.subscribe(listener)

      store.createDocument()
      expect(listener).toHaveBeenCalledTimes(1)

      unsubscribe()
      store.addBlock(store.document!.root, 'Test')

      // Should not be called again after unsubscribe
      expect(listener).toHaveBeenCalledTimes(1)
    })
  })

  describe('Error Handling', () => {
    it('throws when editing without document', () => {
      expect(() => store.editBlock('fake_id', 'content')).toThrow()
    })

    it('throws when block not found', () => {
      store.createDocument()
      expect(() => store.editBlock('fake_id', 'content')).toThrow('Block not found')
    })

    it('stores last error', () => {
      store.createDocument()
      try {
        store.editBlock('fake_id', 'content')
      } catch {
        // Expected
      }

      expect(store.lastError).not.toBeNull()
    })

    it('clears error', () => {
      store.createDocument()
      try {
        store.editBlock('fake_id', 'content')
      } catch {
        // Expected
      }

      store.clearError()
      expect(store.lastError).toBeNull()
    })
  })
})
