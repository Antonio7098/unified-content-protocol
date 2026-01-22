/**
 * Event types for UCM Editor.
 *
 * Events are used for observability, logging, and extensibility.
 */

import type { ContentType, EdgeType } from 'ucp-content'

// Type alias for BlockId since it's not exported
type BlockId = string
import type { EditorView, EditorMode, DiffChangeType, GraphLayout } from './editor.js'

// =============================================================================
// EVENT TYPES
// =============================================================================

export type EditorEventType =
  // Document events
  | 'document:loaded'
  | 'document:created'
  | 'document:saved'
  | 'document:modified'
  | 'document:validated'
  // Block events
  | 'block:added'
  | 'block:deleted'
  | 'block:edited'
  | 'block:moved'
  | 'block:type_changed'
  // Selection events
  | 'selection:changed'
  | 'selection:cleared'
  // Edit events
  | 'edit:started'
  | 'edit:saved'
  | 'edit:cancelled'
  // Drag events
  | 'drag:started'
  | 'drag:moved'
  | 'drag:ended'
  | 'drag:cancelled'
  // History events
  | 'history:undo'
  | 'history:redo'
  | 'history:snapshot_created'
  // Edge events
  | 'edge:added'
  | 'edge:removed'
  // View events
  | 'view:changed'
  | 'mode:changed'
  // Graph events
  | 'graph:layout_changed'
  | 'graph:node_expanded'
  | 'graph:node_collapsed'
  | 'graph:viewport_changed'
  // Diff events
  | 'diff:started'
  | 'diff:ended'
  | 'diff:change_applied'
  | 'diff:change_rejected'
  // Error events
  | 'error:occurred'
  | 'error:cleared'

// =============================================================================
// EVENT DATA INTERFACES
// =============================================================================

export interface DocumentEventData {
  documentId: string
  title?: string
  blockCount?: number
  version?: number
}

export interface BlockEventData {
  blockId: BlockId
  parentId?: BlockId
  content?: string
  type?: ContentType
  oldContent?: string
  oldType?: ContentType
  oldParentId?: BlockId
  newParentId?: BlockId
  position?: 'before' | 'after' | 'inside'
}

export interface SelectionEventData {
  blockIds: BlockId[]
  focusedBlockId?: BlockId
  anchor?: BlockId
  textSelection?: {
    blockId: BlockId
    start: number
    end: number
  }
}

export interface EditEventData {
  blockId: BlockId
  content?: string
  previousContent?: string
}

export interface DragEventData {
  sourceId: BlockId
  targetId?: BlockId
  position?: 'before' | 'after' | 'inside'
  x?: number
  y?: number
}

export interface HistoryEventData {
  entryId: string
  description: string
  snapshotId: string
  operationCount?: number
}

export interface EdgeEventData {
  sourceId: BlockId
  targetId: BlockId
  edgeType: EdgeType
}

export interface ViewEventData {
  view?: EditorView
  previousView?: EditorView
  mode?: EditorMode
  previousMode?: EditorMode
}

export interface GraphEventData {
  layout?: GraphLayout
  nodeId?: BlockId
  viewport?: {
    x: number
    y: number
    zoom: number
  }
}

export interface DiffEventData {
  leftSnapshotId?: string
  rightSnapshotId?: string
  blockId?: BlockId
  changeType?: DiffChangeType
}

export interface ErrorEventData {
  code: string
  message: string
  category: string
  severity: string
  data?: Record<string, unknown>
}

// =============================================================================
// EVENT INTERFACE
// =============================================================================

export type EditorEventData =
  | DocumentEventData
  | BlockEventData
  | SelectionEventData
  | EditEventData
  | DragEventData
  | HistoryEventData
  | EdgeEventData
  | ViewEventData
  | GraphEventData
  | DiffEventData
  | ErrorEventData

export interface EditorEvent<T extends EditorEventType = EditorEventType> {
  type: T
  timestamp: Date
  data: EditorEventData
  source?: string
}

// =============================================================================
// EVENT HANDLER
// =============================================================================

export type EditorEventHandler<T extends EditorEventType = EditorEventType> = (
  event: EditorEvent<T>
) => void

// =============================================================================
// EVENT EMITTER INTERFACE
// =============================================================================

export interface EditorEventEmitter {
  on<T extends EditorEventType>(type: T, handler: EditorEventHandler<T>): () => void
  once<T extends EditorEventType>(type: T, handler: EditorEventHandler<T>): () => void
  off<T extends EditorEventType>(type: T, handler: EditorEventHandler<T>): void
  emit<T extends EditorEventType>(type: T, data: EditorEventData): void
  clear(): void
}

// =============================================================================
// SIMPLE EVENT EMITTER IMPLEMENTATION
// =============================================================================

/**
 * Simple event emitter for editor events.
 *
 * @example
 * ```typescript
 * const emitter = new SimpleEventEmitter()
 *
 * const unsubscribe = emitter.on('block:added', (event) => {
 *   console.log('Block added:', event.data.blockId)
 * })
 *
 * emitter.emit('block:added', { blockId: 'blk_123' })
 *
 * unsubscribe()
 * ```
 */
export class SimpleEventEmitter implements EditorEventEmitter {
  private handlers = new Map<EditorEventType, Set<EditorEventHandler>>()
  private onceHandlers = new Map<EditorEventType, Set<EditorEventHandler>>()

  on<T extends EditorEventType>(type: T, handler: EditorEventHandler<T>): () => void {
    if (!this.handlers.has(type)) {
      this.handlers.set(type, new Set())
    }
    this.handlers.get(type)!.add(handler as EditorEventHandler)
    return () => this.off(type, handler)
  }

  once<T extends EditorEventType>(type: T, handler: EditorEventHandler<T>): () => void {
    if (!this.onceHandlers.has(type)) {
      this.onceHandlers.set(type, new Set())
    }
    this.onceHandlers.get(type)!.add(handler as EditorEventHandler)
    return () => {
      const handlers = this.onceHandlers.get(type)
      if (handlers) {
        handlers.delete(handler as EditorEventHandler)
      }
    }
  }

  off<T extends EditorEventType>(type: T, handler: EditorEventHandler<T>): void {
    const handlers = this.handlers.get(type)
    if (handlers) {
      handlers.delete(handler as EditorEventHandler)
    }
    const onceHandlers = this.onceHandlers.get(type)
    if (onceHandlers) {
      onceHandlers.delete(handler as EditorEventHandler)
    }
  }

  emit<T extends EditorEventType>(type: T, data: EditorEventData): void {
    const event: EditorEvent<T> = {
      type,
      timestamp: new Date(),
      data,
    }

    const handlers = this.handlers.get(type)
    if (handlers) {
      handlers.forEach((handler) => {
        try {
          handler(event)
        } catch (error) {
          console.error(`Error in event handler for ${type}:`, error)
        }
      })
    }

    const onceHandlers = this.onceHandlers.get(type)
    if (onceHandlers) {
      onceHandlers.forEach((handler) => {
        try {
          handler(event)
        } catch (error) {
          console.error(`Error in once handler for ${type}:`, error)
        }
      })
      this.onceHandlers.delete(type)
    }
  }

  clear(): void {
    this.handlers.clear()
    this.onceHandlers.clear()
  }

  /**
   * Get the number of handlers for a specific event type.
   */
  listenerCount(type: EditorEventType): number {
    const handlers = this.handlers.get(type)?.size ?? 0
    const onceHandlers = this.onceHandlers.get(type)?.size ?? 0
    return handlers + onceHandlers
  }

  /**
   * Get all registered event types.
   */
  eventTypes(): EditorEventType[] {
    const types = new Set<EditorEventType>()
    this.handlers.forEach((_, type) => types.add(type))
    this.onceHandlers.forEach((_, type) => types.add(type))
    return Array.from(types)
  }
}
