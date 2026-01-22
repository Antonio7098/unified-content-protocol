/**
 * useEditor - Main React hook for the UCM Editor.
 *
 * Provides access to the editor store and reactive state updates.
 */

import { useRef, useEffect, useSyncExternalStore, useCallback } from 'react'
import type { Document } from 'ucp-content'
import { createEditorStore, type EditorStoreInstance } from '../core/EditorStore.js'
import type { EditorStoreState, EditorConfig } from '../types/editor.js'

type EditorEventTypeParam = Parameters<EditorStoreInstance['events']['on']>[0]
type EditorEventHandlerParam = Parameters<EditorStoreInstance['events']['on']>[1]

// =============================================================================
// EDITOR CONTEXT
// =============================================================================

/**
 * Hook to create and manage an editor store instance.
 *
 * @example
 * ```typescript
 * function MyEditor() {
 *   const editor = useEditorStore()
 *
 *   useEffect(() => {
 *     editor.loadDocument(myDocument)
 *   }, [])
 *
 *   return <Editor store={editor} />
 * }
 * ```
 */
export function useEditorStore(config?: Partial<EditorConfig>): EditorStoreInstance {
  const storeRef = useRef<EditorStoreInstance | null>(null)

  if (storeRef.current === null) {
    storeRef.current = createEditorStore(config)
  }

  return storeRef.current
}

// =============================================================================
// STATE SELECTOR HOOK
// =============================================================================

/**
 * Hook to subscribe to specific parts of the editor state.
 *
 * @example
 * ```typescript
 * function BlockList() {
 *   const document = useEditorState(store, (state) => state.document)
 *   const selection = useEditorState(store, (state) => state.selection)
 *
 *   // Component re-renders only when document or selection changes
 * }
 * ```
 */
export function useEditorState<T>(
  store: EditorStoreInstance,
  selector: (state: EditorStoreState) => T
): T {
  const subscribe = useCallback(
    (callback: () => void) => store.subscribe(callback),
    [store]
  )

  const getSnapshot = useCallback(() => selector(store.getState()), [store, selector])

  return useSyncExternalStore(subscribe, getSnapshot, getSnapshot)
}

// =============================================================================
// DOCUMENT HOOK
// =============================================================================

/**
 * Hook to access the current document.
 */
export function useDocument(store: EditorStoreInstance): Document | null {
  return useEditorState(store, (state) => state.document)
}

// =============================================================================
// SELECTION HOOK
// =============================================================================

/**
 * Hook to access selection state.
 */
export function useSelection(store: EditorStoreInstance) {
  const selection = useEditorState(store, (state) => state.selection)
  const editingBlockId = useEditorState(store, (state) => state.editingBlockId)

  return {
    selection,
    editingBlockId,
    isBlockSelected: (blockId: string) => {
      if (selection.type === 'block') {
        return selection.blocks?.blockIds.includes(blockId) ?? false
      }
      if (selection.type === 'text') {
        return selection.text?.blockId === blockId
      }
      return false
    },
    isBlockFocused: (blockId: string) => selection.focusedBlockId === blockId,
    isBlockEditing: (blockId: string) => editingBlockId === blockId,
  }
}

// =============================================================================
// HISTORY HOOK
// =============================================================================

/**
 * Hook to access history state and actions.
 */
export function useHistory(store: EditorStoreInstance) {
  const history = useEditorState(store, (state) => state.history)

  return {
    canUndo: history.canUndo,
    canRedo: history.canRedo,
    entries: history.entries,
    currentIndex: history.currentIndex,
    undo: store.undo,
    redo: store.redo,
  }
}

// =============================================================================
// DRAG HOOK
// =============================================================================

/**
 * Hook to access drag state.
 */
export function useDrag(store: EditorStoreInstance) {
  const drag = useEditorState(store, (state) => state.drag)

  return {
    isDragging: drag.isDragging,
    sourceId: drag.sourceId,
    targetId: drag.targetId,
    position: drag.position,
    startDrag: store.startDrag,
    updateTarget: store.updateDragTarget,
    endDrag: store.endDrag,
  }
}

// =============================================================================
// VIEW HOOK
// =============================================================================

/**
 * Hook to access view state.
 */
export function useView(store: EditorStoreInstance) {
  const view = useEditorState(store, (state) => state.view)
  const mode = useEditorState(store, (state) => state.mode)

  return {
    view,
    mode,
    setView: store.setView,
    setMode: store.setMode,
  }
}

// =============================================================================
// BLOCK ACTIONS HOOK
// =============================================================================

/**
 * Hook to access block manipulation actions.
 */
export function useBlockActions(store: EditorStoreInstance) {
  return {
    addBlock: store.addBlock,
    editBlock: store.editBlock,
    deleteBlock: store.deleteBlock,
    moveBlock: store.moveBlock,
    changeBlockType: store.changeBlockType,
  }
}

// =============================================================================
// EDIT ACTIONS HOOK
// =============================================================================

/**
 * Hook to access edit actions.
 */
export function useEditActions(store: EditorStoreInstance) {
  const editingBlockId = useEditorState(store, (state) => state.editingBlockId)
  const pendingContent = useEditorState(store, (state) => state.pendingContent)
  const editState = useEditorState(store, (state) => state.editState)

  return {
    editingBlockId,
    pendingContent,
    editState,
    startEditing: store.startEditing,
    stopEditing: store.stopEditing,
    updateContent: store.updatePendingContent,
  }
}

// =============================================================================
// EDITOR EVENTS HOOK
// =============================================================================

/**
 * Hook to subscribe to editor events.
 *
 * @example
 * ```typescript
 * function MyComponent({ store }) {
 *   useEditorEvent(store, 'block:added', (event) => {
 *     console.log('Block added:', event.data)
 *   })
 * }
 * ```
 */
export function useEditorEvent<T extends EditorEventTypeParam>(
  store: EditorStoreInstance,
  eventType: T,
  handler: EditorEventHandlerParam
): void {
  useEffect(() => {
    return store.events.on(eventType, handler)
  }, [store, eventType, handler])
}

// =============================================================================
// KEYBOARD SHORTCUTS HOOK
// =============================================================================

export interface KeyboardShortcuts {
  'mod+z': () => void
  'mod+shift+z': () => void
  'mod+y': () => void
  'mod+s': () => void
  'mod+a': () => void
  escape: () => void
  enter: () => void
  delete: () => void
  backspace: () => void
  arrowUp: () => void
  arrowDown: () => void
  arrowLeft: () => void
  arrowRight: () => void
  tab: () => void
  'shift+tab': () => void
}

/**
 * Hook to handle keyboard shortcuts.
 */
export function useKeyboardShortcuts(
  store: EditorStoreInstance,
  enabled = true
): void {
  useEffect(() => {
    if (!enabled) return

    const handleKeyDown = (event: KeyboardEvent) => {
      const isMod = event.metaKey || event.ctrlKey
      const isShift = event.shiftKey

      // Undo: Cmd/Ctrl+Z
      if (isMod && !isShift && event.key === 'z') {
        event.preventDefault()
        if (store.history.canUndo) {
          store.undo()
        }
        return
      }

      // Redo: Cmd/Ctrl+Shift+Z or Cmd/Ctrl+Y
      if ((isMod && isShift && event.key === 'z') || (isMod && event.key === 'y')) {
        event.preventDefault()
        if (store.history.canRedo) {
          store.redo()
        }
        return
      }

      // Save: Cmd/Ctrl+S
      if (isMod && event.key === 's') {
        event.preventDefault()
        store.saveDocument().catch(console.error)
        return
      }

      // Select all: Cmd/Ctrl+A
      if (isMod && event.key === 'a') {
        event.preventDefault()
        if (store.document) {
          const blockIds = Array.from(store.document.blocks.keys()).filter(
            (id) => id !== store.document!.root
          )
          store.selectMultiple(blockIds)
        }
        return
      }

      // Escape: Cancel editing or clear selection
      if (event.key === 'Escape') {
        if (store.editingBlockId) {
          store.stopEditing(false)
        } else if (store.drag.isDragging) {
          store.endDrag(false)
        } else {
          store.clearSelection()
        }
        return
      }

      // Enter: Start editing selected block
      if (event.key === 'Enter' && !isMod) {
        const focusedId = store.selection.focusedBlockId
        if (focusedId && !store.editingBlockId) {
          event.preventDefault()
          store.startEditing(focusedId)
        }
        return
      }

      // Delete/Backspace: Delete selected blocks
      if ((event.key === 'Delete' || event.key === 'Backspace') && !store.editingBlockId) {
        const selectedIds =
          store.selection.type === 'block' ? store.selection.blocks?.blockIds : undefined
        if (selectedIds && selectedIds.length > 0) {
          event.preventDefault()
          selectedIds.forEach((id) => {
            try {
              store.deleteBlock(id)
            } catch {
              // Ignore errors for already-deleted blocks
            }
          })
        }
        return
      }

      // Arrow keys: Navigation (only when not editing)
      if (!store.editingBlockId) {
        if (event.key === 'ArrowUp') {
          event.preventDefault()
          navigateUp(store, isShift)
          return
        }
        if (event.key === 'ArrowDown') {
          event.preventDefault()
          navigateDown(store, isShift)
          return
        }
      }
    }

    document.addEventListener('keydown', handleKeyDown)
    return () => document.removeEventListener('keydown', handleKeyDown)
  }, [store, enabled])
}

function navigateUp(store: EditorStoreInstance, expand: boolean): void {
  if (!store.document) return

  const currentId = store.selection.focusedBlockId
  if (!currentId) {
    // Select first non-root block
    const firstChild = store.document.blocks.get(store.document.root)?.children[0]
    if (firstChild) {
      store.select(firstChild)
    }
    return
  }

  // Find previous block in document order
  const order = getBlockOrder(store.document)
  const currentIndex = order.indexOf(currentId)
  if (currentIndex > 1) {
    // Skip root at index 0
    const prevId = order[currentIndex - 1]!
    if (expand) {
      const selectedIds = store.selection.blocks?.blockIds ?? [currentId]
      if (!selectedIds.includes(prevId)) {
        store.selectMultiple([...selectedIds, prevId])
      }
    } else {
      store.select(prevId)
    }
  }
}

function navigateDown(store: EditorStoreInstance, expand: boolean): void {
  if (!store.document) return

  const currentId = store.selection.focusedBlockId
  if (!currentId) {
    const firstChild = store.document.blocks.get(store.document.root)?.children[0]
    if (firstChild) {
      store.select(firstChild)
    }
    return
  }

  const order = getBlockOrder(store.document)
  const currentIndex = order.indexOf(currentId)
  if (currentIndex < order.length - 1) {
    const nextId = order[currentIndex + 1]!
    if (expand) {
      const selectedIds = store.selection.blocks?.blockIds ?? [currentId]
      if (!selectedIds.includes(nextId)) {
        store.selectMultiple([...selectedIds, nextId])
      }
    } else {
      store.select(nextId)
    }
  }
}

function getBlockOrder(doc: Document): string[] {
  const order: string[] = []

  function traverse(blockId: string): void {
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
