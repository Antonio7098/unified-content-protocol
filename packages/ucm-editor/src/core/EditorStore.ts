/**
 * EditorStore - Core state management for UCM Editor.
 *
 * Uses a simple observable store pattern with immutable updates.
 * Implements SOLID principles with clear separation of concerns.
 */

import type { BlockId, Document, ContentType, EdgeType } from 'ucp-js'
import {
  createDocument as createUcpDocument,
  addBlock as addUcpBlock,
  editBlock as editUcpBlock,
  moveBlock as moveUcpBlock,
  deleteBlock as deleteUcpBlock,
  addEdge as addUcpEdge,
  removeEdge as removeUcpEdge,
  serializeDocument,
  SnapshotManager,
  validateDocument,
} from 'ucp-js'

import type {
  EditorStoreState,
  EditorStore,
  EditorView,
  EditorMode,
  SelectionState,
  DragState,
  GraphViewState,
  DiffViewState,
  HistoryState,
  HistoryEntry,
  HistoryOperation,
  EditorConfig,
  GraphLayout,
} from '../types/editor.js'
import { DEFAULT_EDITOR_CONFIG } from '../types/editor.js'
import { Errors, EditorError } from '../types/errors.js'
import type { EditorEventEmitter, EditorEventData } from '../types/events.js'
import { SimpleEventEmitter } from '../types/events.js'
import { Logger } from './Logger.js'

// =============================================================================
// INITIAL STATE
// =============================================================================

function createInitialSelection(): SelectionState {
  return {
    type: 'none',
  }
}

function createInitialDragState(): DragState {
  return {
    isDragging: false,
  }
}

function createInitialGraphState(): GraphViewState {
  return {
    layout: 'hierarchical',
    nodes: new Map(),
    edges: [],
    viewport: { x: 0, y: 0, zoom: 1 },
    selectedNodeId: undefined,
    hoveredNodeId: undefined,
    showEdgeLabels: true,
    edgeFilter: [],
  }
}

function createInitialDiffState(): DiffViewState {
  return {
    isComparing: false,
    leftSnapshotId: undefined,
    rightSnapshotId: undefined,
    diff: undefined,
    selectedChangeId: undefined,
    showUnchanged: false,
    viewMode: 'unified',
  }
}

function createInitialHistoryState(maxEntries: number): HistoryState {
  return {
    entries: [],
    currentIndex: -1,
    maxEntries,
    canUndo: false,
    canRedo: false,
  }
}

function createInitialState(config: EditorConfig): EditorStoreState {
  return {
    document: null,
    documentId: null,
    isLoading: false,
    isDirty: false,
    lastSaved: undefined,
    view: 'document',
    mode: 'view',
    selection: createInitialSelection(),
    editingBlockId: null,
    editState: 'idle',
    pendingContent: null,
    drag: createInitialDragState(),
    graph: createInitialGraphState(),
    diff: createInitialDiffState(),
    history: createInitialHistoryState(config.maxHistoryEntries),
    config,
    lastError: null,
  }
}

// =============================================================================
// STORE SUBSCRIBER
// =============================================================================

type StoreListener = (state: EditorStoreState, prevState: EditorStoreState) => void

// =============================================================================
// EDITOR STORE IMPLEMENTATION
// =============================================================================

/**
 * Create an editor store instance.
 *
 * @example
 * ```typescript
 * const store = createEditorStore()
 *
 * // Subscribe to state changes
 * const unsubscribe = store.subscribe((state, prevState) => {
 *   console.log('State changed:', state)
 * })
 *
 * // Load a document
 * store.loadDocument(myDocument)
 *
 * // Edit a block
 * store.startEditing('blk_123')
 * store.updatePendingContent('New content')
 * store.stopEditing(true) // save
 *
 * // Undo
 * store.undo()
 * ```
 */
export function createEditorStore(
  initialConfig: Partial<EditorConfig> = {}
): EditorStore & {
  subscribe: (listener: StoreListener) => () => void
  getState: () => EditorStoreState
  events: EditorEventEmitter
} {
  const config = { ...DEFAULT_EDITOR_CONFIG, ...initialConfig }
  const logger = new Logger({ context: 'EditorStore', level: config.logLevel })
  const events: EditorEventEmitter = new SimpleEventEmitter()
  const snapshotManager = new SnapshotManager(config.maxHistoryEntries)

  let state = createInitialState(config)
  const listeners = new Set<StoreListener>()

  // ==========================================================================
  // STATE MANAGEMENT
  // ==========================================================================

  function setState(updater: Partial<EditorStoreState> | ((s: EditorStoreState) => Partial<EditorStoreState>)): void {
    const prevState = state
    const updates = typeof updater === 'function' ? updater(state) : updater
    state = { ...state, ...updates }

    listeners.forEach((listener) => {
      try {
        listener(state, prevState)
      } catch (error) {
        logger.error('Listener error', error instanceof Error ? error : undefined)
      }
    })
  }

  function emitEvent(type: Parameters<typeof events.emit>[0], data: EditorEventData): void {
    events.emit(type, data)
  }

  function handleError(error: Error | EditorError): void {
    logger.error(error.message, error)
    setState({ lastError: error })
    if (error instanceof EditorError) {
      emitEvent('error:occurred', {
        code: error.code,
        message: error.message,
        category: error.category,
        severity: error.severity,
        data: error.data as Record<string, unknown>,
      })
    }
  }

  // ==========================================================================
  // HISTORY HELPERS
  // ==========================================================================

  function pushHistory(description: string, operations: HistoryOperation[]): void {
    if (!state.document) return

    const snapshotId = `snapshot_${Date.now()}`
    snapshotManager.create(snapshotId, state.document, description)

    const entry: HistoryEntry = {
      id: `entry_${Date.now()}`,
      timestamp: new Date(),
      description,
      snapshotId,
      operations,
    }

    const newEntries = [...state.history.entries.slice(0, state.history.currentIndex + 1), entry]
    if (newEntries.length > state.history.maxEntries) {
      newEntries.shift()
    }

    setState({
      history: {
        ...state.history,
        entries: newEntries,
        currentIndex: newEntries.length - 1,
        canUndo: newEntries.length > 0,
        canRedo: false,
      },
      isDirty: true,
    })

    emitEvent('history:snapshot_created', {
      entryId: entry.id,
      description,
      snapshotId,
      operationCount: operations.length,
    })
  }

  // ==========================================================================
  // DOCUMENT OPERATIONS
  // ==========================================================================

  function loadDocument(doc: Document): void {
    logger.info('Loading document', { documentId: doc.id, blockCount: doc.blocks.size })

    setState({
      document: doc,
      documentId: doc.id,
      isLoading: false,
      isDirty: false,
      selection: createInitialSelection(),
      editingBlockId: null,
      editState: 'idle',
      pendingContent: null,
      history: createInitialHistoryState(state.config.maxHistoryEntries),
      lastError: null,
    })

    // Create initial snapshot
    snapshotManager.create('initial', doc, 'Initial state')

    emitEvent('document:loaded', {
      documentId: doc.id,
      title: doc.metadata?.title,
      blockCount: doc.blocks.size,
      version: doc.version,
    })
  }

  function createDocument(title?: string): void {
    logger.info('Creating new document', { title })
    const doc = createUcpDocument(title)
    loadDocument(doc)
    emitEvent('document:created', {
      documentId: doc.id,
      title,
    })
  }

  async function saveDocument(): Promise<void> {
    if (!state.document) {
      throw Errors.documentNotLoaded()
    }

    logger.info('Saving document', { documentId: state.documentId })
    setState({ isLoading: true })

    try {
      // Validate before saving
      const validation = validateDocument(state.document)
      if (!validation.valid) {
        const errors = validation.issues.filter((i) => i.severity === 'error')
        if (errors.length > 0) {
          throw new EditorError({
            code: 'UCM_E071',
            message: 'Document validation failed',
            category: 'document',
            data: { validationErrors: errors.map((e) => e.message) },
          })
        }
      }

      // In a real implementation, this would persist to storage
      // For now, we just update the state
      setState({
        isLoading: false,
        isDirty: false,
        lastSaved: new Date(),
      })

      emitEvent('document:saved', {
        documentId: state.documentId!,
        version: state.document.version,
      })
    } catch (error) {
      setState({ isLoading: false })
      handleError(error instanceof Error ? error : Errors.internalError(String(error)))
      throw error
    }
  }

  // ==========================================================================
  // VIEW OPERATIONS
  // ==========================================================================

  function setView(view: EditorView): void {
    const previousView = state.view
    logger.debug('Setting view', { view, previousView })
    setState({ view })
    emitEvent('view:changed', { view, previousView })
  }

  function setMode(mode: EditorMode): void {
    const previousMode = state.mode
    logger.debug('Setting mode', { mode, previousMode })
    setState({ mode })
    emitEvent('mode:changed', { mode, previousMode })
  }

  // ==========================================================================
  // BLOCK OPERATIONS
  // ==========================================================================

  function addBlock(parentId: BlockId, content: string, type?: ContentType): BlockId {
    if (!state.document) {
      throw Errors.documentNotLoaded()
    }

    logger.debug('Adding block', { parentId, contentLength: content.length, type })

    const id = addUcpBlock(state.document, parentId, content, { type })

    pushHistory(`Add block`, [{ type: 'add_block', blockId: id, data: { parentId, content, type } }])

    emitEvent('block:added', { blockId: id, parentId, content, type })

    // Trigger re-render
    setState({ document: { ...state.document } })

    return id
  }

  function editBlockContent(blockId: BlockId, content: string): void {
    if (!state.document) {
      const error = Errors.documentNotLoaded()
      handleError(error)
      throw error
    }

    const block = state.document.blocks.get(blockId)
    if (!block) {
      const error = Errors.blockNotFound(blockId)
      handleError(error)
      throw error
    }

    const oldContent = block.content
    logger.debug('Editing block', { blockId, oldLength: oldContent.length, newLength: content.length })

    editUcpBlock(state.document, blockId, content)

    pushHistory(`Edit block`, [{ type: 'edit_block', blockId, data: { oldContent, newContent: content } }])

    emitEvent('block:edited', { blockId, content, oldContent })

    setState({ document: { ...state.document } })
  }

  function deleteBlockById(blockId: BlockId, cascade = true): void {
    if (!state.document) {
      throw Errors.documentNotLoaded()
    }

    const block = state.document.blocks.get(blockId)
    if (!block) {
      throw Errors.blockNotFound(blockId)
    }

    logger.debug('Deleting block', { blockId, cascade })

    // Capture block data for history
    const blockData = serializeDocument(state.document)

    deleteUcpBlock(state.document, blockId, { cascade })

    pushHistory(`Delete block`, [{ type: 'delete_block', blockId, data: { cascade, snapshot: blockData } }])

    // Clear selection if deleted block was selected
    if (state.selection.focusedBlockId === blockId) {
      setState({
        selection: createInitialSelection(),
        document: { ...state.document },
      })
    } else {
      setState({ document: { ...state.document } })
    }

    emitEvent('block:deleted', { blockId })
  }

  function moveBlockTo(
    blockId: BlockId,
    targetId: BlockId,
    position: 'before' | 'after' | 'inside'
  ): void {
    if (!state.document) {
      throw Errors.documentNotLoaded()
    }

    const block = state.document.blocks.get(blockId)
    if (!block) {
      throw Errors.blockNotFound(blockId)
    }

    const target = state.document.blocks.get(targetId)
    if (!target) {
      throw Errors.blockNotFound(targetId)
    }

    logger.debug('Moving block', { blockId, targetId, position })

    // Find current parent for history
    let oldParentId: BlockId | undefined
    for (const [id, b] of state.document.blocks) {
      if (b.children.includes(blockId)) {
        oldParentId = id
        break
      }
    }

    // Determine new parent and index
    let newParentId: BlockId
    let index: number | undefined

    if (position === 'inside') {
      newParentId = targetId
    } else {
      // Find target's parent
      for (const [id, b] of state.document.blocks) {
        if (b.children.includes(targetId)) {
          newParentId = id
          const targetIndex = b.children.indexOf(targetId)
          index = position === 'before' ? targetIndex : targetIndex + 1
          break
        }
      }
      if (!newParentId!) {
        throw Errors.invalidDropTarget(blockId, targetId, 'Target has no parent')
      }
    }

    moveUcpBlock(state.document, blockId, newParentId, index)

    pushHistory(`Move block`, [
      {
        type: 'move_block',
        blockId,
        data: { oldParentId, newParentId, position },
      },
    ])

    emitEvent('block:moved', { blockId, oldParentId, newParentId, position })

    setState({ document: { ...state.document } })
  }

  function changeBlockType(blockId: BlockId, type: ContentType): void {
    if (!state.document) {
      throw Errors.documentNotLoaded()
    }

    const block = state.document.blocks.get(blockId)
    if (!block) {
      throw Errors.blockNotFound(blockId)
    }

    const oldType = block.type
    logger.debug('Changing block type', { blockId, oldType, newType: type })

    block.type = type

    pushHistory(`Change block type`, [
      { type: 'change_type', blockId, data: { oldType, newType: type } },
    ])

    emitEvent('block:type_changed', { blockId, type, oldType })

    setState({ document: { ...state.document } })
  }

  // ==========================================================================
  // SELECTION OPERATIONS
  // ==========================================================================

  function select(blockId: BlockId): void {
    if (!state.document) return

    const block = state.document.blocks.get(blockId)
    if (!block) {
      logger.warn('Attempted to select non-existent block', { blockId })
      return
    }

    logger.debug('Selecting block', { blockId })

    setState({
      selection: {
        type: 'block',
        blocks: { blockIds: [blockId], anchor: blockId, focus: blockId },
        focusedBlockId: blockId,
      },
    })

    emitEvent('selection:changed', { blockIds: [blockId], focusedBlockId: blockId })
  }

  function selectMultiple(blockIds: BlockId[]): void {
    if (!state.document) return

    const validIds = blockIds.filter((id) => state.document!.blocks.has(id))
    if (validIds.length === 0) {
      clearSelection()
      return
    }

    logger.debug('Selecting multiple blocks', { count: validIds.length })

    setState({
      selection: {
        type: 'block',
        blocks: {
          blockIds: validIds,
          anchor: validIds[0],
          focus: validIds[validIds.length - 1],
        },
        focusedBlockId: validIds[0],
      },
    })

    emitEvent('selection:changed', { blockIds: validIds, focusedBlockId: validIds[0] })
  }

  function clearSelection(): void {
    logger.debug('Clearing selection')
    setState({ selection: createInitialSelection() })
    emitEvent('selection:cleared', { blockIds: [] })
  }

  function selectText(blockId: BlockId, start: number, end: number): void {
    if (!state.document) return

    const block = state.document.blocks.get(blockId)
    if (!block) return

    logger.debug('Selecting text', { blockId, start, end })

    setState({
      selection: {
        type: 'text',
        text: { blockId, start, end },
        focusedBlockId: blockId,
      },
    })

    emitEvent('selection:changed', {
      blockIds: [blockId],
      focusedBlockId: blockId,
      textSelection: { blockId, start, end },
    })
  }

  // ==========================================================================
  // EDIT OPERATIONS
  // ==========================================================================

  function startEditing(blockId: BlockId): void {
    if (!state.document) {
      throw Errors.documentNotLoaded()
    }

    const block = state.document.blocks.get(blockId)
    if (!block) {
      throw Errors.blockNotFound(blockId)
    }

    logger.debug('Starting edit', { blockId })

    setState({
      editingBlockId: blockId,
      editState: 'editing',
      pendingContent: block.content,
      mode: 'edit',
    })

    emitEvent('edit:started', { blockId, content: block.content })
  }

  function stopEditing(save = true): void {
    if (!state.editingBlockId || !state.document) {
      return
    }

    const blockId = state.editingBlockId
    const content = state.pendingContent

    logger.debug('Stopping edit', { blockId, save })

    if (save && content !== null) {
      const block = state.document.blocks.get(blockId)
      if (block && block.content !== content) {
        editBlockContent(blockId, content)
        emitEvent('edit:saved', { blockId, content })
      }
    } else {
      emitEvent('edit:cancelled', { blockId })
    }

    setState({
      editingBlockId: null,
      editState: 'idle',
      pendingContent: null,
      mode: 'view',
    })
  }

  function updatePendingContent(content: string): void {
    if (state.editingBlockId) {
      setState({ pendingContent: content })
    }
  }

  // ==========================================================================
  // DRAG OPERATIONS
  // ==========================================================================

  function startDrag(blockId: BlockId): void {
    if (!state.document) return

    const block = state.document.blocks.get(blockId)
    if (!block) return

    logger.debug('Starting drag', { blockId })

    setState({
      drag: {
        isDragging: true,
        sourceId: blockId,
        targetId: undefined,
        position: undefined,
      },
      mode: 'drag',
    })

    emitEvent('drag:started', { sourceId: blockId })
  }

  function updateDragTarget(targetId: BlockId, position: 'before' | 'after' | 'inside'): void {
    if (!state.drag.isDragging) return

    setState({
      drag: {
        ...state.drag,
        targetId,
        position,
      },
    })

    emitEvent('drag:moved', { sourceId: state.drag.sourceId!, targetId, position })
  }

  function endDrag(drop = true): void {
    if (!state.drag.isDragging) return

    const { sourceId, targetId, position } = state.drag

    logger.debug('Ending drag', { sourceId, targetId, position, drop })

    if (drop && sourceId && targetId && position) {
      try {
        moveBlockTo(sourceId, targetId, position)
        emitEvent('drag:ended', { sourceId, targetId, position })
      } catch (error) {
        handleError(error instanceof Error ? error : Errors.internalError(String(error)))
        emitEvent('drag:cancelled', { sourceId })
      }
    } else {
      emitEvent('drag:cancelled', { sourceId: sourceId! })
    }

    setState({
      drag: createInitialDragState(),
      mode: 'view',
    })
  }

  // ==========================================================================
  // HISTORY OPERATIONS
  // ==========================================================================

  function undo(): void {
    if (!state.history.canUndo || state.history.currentIndex < 0) {
      throw Errors.noUndoAvailable()
    }

    const entry = state.history.entries[state.history.currentIndex]
    if (!entry) return

    logger.debug('Undoing', { entryId: entry.id, description: entry.description })

    // Restore previous snapshot
    const previousIndex = state.history.currentIndex - 1
    const previousEntry = state.history.entries[previousIndex]
    if (previousEntry) {
      const doc = snapshotManager.restore(previousEntry.snapshotId)
      setState({
        document: doc,
        history: {
          ...state.history,
          currentIndex: previousIndex,
          canUndo: previousIndex >= 0,
          canRedo: true,
        },
        isDirty: true,
      })
    } else {
      // Restore initial state
      const doc = snapshotManager.restore('initial')
      setState({
        document: doc,
        history: {
          ...state.history,
          currentIndex: -1,
          canUndo: false,
          canRedo: true,
        },
        isDirty: true,
      })
    }

    emitEvent('history:undo', {
      entryId: entry.id,
      description: entry.description,
      snapshotId: entry.snapshotId,
    })
  }

  function redo(): void {
    if (!state.history.canRedo) {
      throw Errors.noRedoAvailable()
    }

    const nextIndex = state.history.currentIndex + 1
    const entry = state.history.entries[nextIndex]
    if (!entry) return

    logger.debug('Redoing', { entryId: entry.id, description: entry.description })

    const doc = snapshotManager.restore(entry.snapshotId)
    setState({
      document: doc,
      history: {
        ...state.history,
        currentIndex: nextIndex,
        canUndo: true,
        canRedo: nextIndex < state.history.entries.length - 1,
      },
      isDirty: true,
    })

    emitEvent('history:redo', {
      entryId: entry.id,
      description: entry.description,
      snapshotId: entry.snapshotId,
    })
  }

  function createSnapshot(description: string): void {
    if (!state.document) {
      throw Errors.documentNotLoaded()
    }

    const snapshotId = `snapshot_${Date.now()}`
    snapshotManager.create(snapshotId, state.document, description)

    logger.debug('Created snapshot', { snapshotId, description })

    emitEvent('history:snapshot_created', {
      entryId: snapshotId,
      description,
      snapshotId,
    })
  }

  // ==========================================================================
  // GRAPH OPERATIONS
  // ==========================================================================

  function setGraphLayout(layout: GraphLayout): void {
    logger.debug('Setting graph layout', { layout })
    setState({
      graph: {
        ...state.graph,
        layout,
      },
    })
    emitEvent('graph:layout_changed', { layout })
  }

  function setGraphViewport(x: number, y: number, zoom: number): void {
    setState({
      graph: {
        ...state.graph,
        viewport: { x, y, zoom },
      },
    })
    emitEvent('graph:viewport_changed', { viewport: { x, y, zoom } })
  }

  function toggleNodeExpansion(nodeId: BlockId): void {
    const node = state.graph.nodes.get(nodeId)
    if (!node) return

    const nodes = new Map(state.graph.nodes)
    nodes.set(nodeId, { ...node, isExpanded: !node.isExpanded })

    setState({
      graph: {
        ...state.graph,
        nodes,
      },
    })

    emitEvent(node.isExpanded ? 'graph:node_collapsed' : 'graph:node_expanded', { nodeId })
  }

  // ==========================================================================
  // DIFF OPERATIONS
  // ==========================================================================

  function startCompare(leftSnapshotId: string, rightSnapshotId: string): void {
    logger.debug('Starting diff comparison', { leftSnapshotId, rightSnapshotId })

    // TODO: Implement diff computation using DiffEngine
    setState({
      diff: {
        ...state.diff,
        isComparing: true,
        leftSnapshotId,
        rightSnapshotId,
      },
    })

    emitEvent('diff:started', { leftSnapshotId, rightSnapshotId })
  }

  function stopCompare(): void {
    logger.debug('Stopping diff comparison')
    setState({
      diff: createInitialDiffState(),
    })
    emitEvent('diff:ended', {})
  }

  function applyChange(blockId: BlockId): void {
    logger.debug('Applying diff change', { blockId })
    // TODO: Implement change application
    emitEvent('diff:change_applied', { blockId, changeType: 'modified' })
  }

  function rejectChange(blockId: BlockId): void {
    logger.debug('Rejecting diff change', { blockId })
    // TODO: Implement change rejection
    emitEvent('diff:change_rejected', { blockId, changeType: 'modified' })
  }

  // ==========================================================================
  // EDGE OPERATIONS
  // ==========================================================================

  function addEdgeToBlock(sourceId: BlockId, targetId: BlockId, edgeType: EdgeType): void {
    if (!state.document) {
      throw Errors.documentNotLoaded()
    }

    logger.debug('Adding edge', { sourceId, targetId, edgeType })

    addUcpEdge(state.document, sourceId, edgeType, targetId)

    pushHistory(`Add edge`, [{ type: 'add_edge', data: { sourceId, targetId, edgeType } }])

    emitEvent('edge:added', { sourceId, targetId, edgeType })

    setState({ document: { ...state.document } })
  }

  function removeEdgeFromBlock(sourceId: BlockId, targetId: BlockId, edgeType: EdgeType): void {
    if (!state.document) {
      throw Errors.documentNotLoaded()
    }

    logger.debug('Removing edge', { sourceId, targetId, edgeType })

    removeUcpEdge(state.document, sourceId, edgeType, targetId)

    pushHistory(`Remove edge`, [{ type: 'remove_edge', data: { sourceId, targetId, edgeType } }])

    emitEvent('edge:removed', { sourceId, targetId, edgeType })

    setState({ document: { ...state.document } })
  }

  // ==========================================================================
  // CONFIG
  // ==========================================================================

  function updateConfig(configUpdate: Partial<EditorConfig>): void {
    logger.debug('Updating config', configUpdate)
    setState({
      config: { ...state.config, ...configUpdate },
    })
  }

  // ==========================================================================
  // ERROR HANDLING
  // ==========================================================================

  function clearError(): void {
    setState({ lastError: null })
    emitEvent('error:cleared', { code: '', message: '', category: '', severity: '' })
  }

  // ==========================================================================
  // STORE API
  // ==========================================================================

  return {
    // State (getters)
    get document() {
      return state.document
    },
    get documentId() {
      return state.documentId
    },
    get isLoading() {
      return state.isLoading
    },
    get isDirty() {
      return state.isDirty
    },
    get lastSaved() {
      return state.lastSaved
    },
    get view() {
      return state.view
    },
    get mode() {
      return state.mode
    },
    get selection() {
      return state.selection
    },
    get editingBlockId() {
      return state.editingBlockId
    },
    get editState() {
      return state.editState
    },
    get pendingContent() {
      return state.pendingContent
    },
    get drag() {
      return state.drag
    },
    get graph() {
      return state.graph
    },
    get diff() {
      return state.diff
    },
    get history() {
      return state.history
    },
    get config() {
      return state.config
    },
    get lastError() {
      return state.lastError
    },

    // Actions
    loadDocument,
    createDocument,
    saveDocument,
    setView,
    setMode,
    addBlock,
    editBlock: editBlockContent,
    deleteBlock: deleteBlockById,
    moveBlock: moveBlockTo,
    changeBlockType,
    select,
    selectMultiple,
    clearSelection,
    selectText,
    startEditing,
    stopEditing,
    updatePendingContent,
    startDrag,
    updateDragTarget,
    endDrag,
    undo,
    redo,
    createSnapshot,
    setGraphLayout,
    setGraphViewport,
    toggleNodeExpansion,
    startCompare,
    stopCompare,
    applyChange,
    rejectChange,
    addEdge: addEdgeToBlock,
    removeEdge: removeEdgeFromBlock,
    updateConfig,
    clearError,

    // Store utilities
    subscribe: (listener: StoreListener) => {
      listeners.add(listener)
      return () => listeners.delete(listener)
    },
    getState: () => state,
    events,
  }
}

// =============================================================================
// STORE TYPE
// =============================================================================

export type EditorStoreInstance = ReturnType<typeof createEditorStore>
