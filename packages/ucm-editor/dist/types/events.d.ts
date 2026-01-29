/**
 * Event types for UCM Editor.
 *
 * Events are used for observability, logging, and extensibility.
 */
import type { BlockId, ContentType, EdgeType } from 'ucp-content';
import type { EditorView, EditorMode, DiffChangeType, GraphLayout } from './editor.js';
export type EditorEventType = 'document:loaded' | 'document:created' | 'document:saved' | 'document:modified' | 'document:validated' | 'block:added' | 'block:deleted' | 'block:edited' | 'block:moved' | 'block:type_changed' | 'selection:changed' | 'selection:cleared' | 'edit:started' | 'edit:saved' | 'edit:cancelled' | 'drag:started' | 'drag:moved' | 'drag:ended' | 'drag:cancelled' | 'history:undo' | 'history:redo' | 'history:snapshot_created' | 'edge:added' | 'edge:removed' | 'view:changed' | 'mode:changed' | 'graph:layout_changed' | 'graph:node_expanded' | 'graph:node_collapsed' | 'graph:viewport_changed' | 'diff:started' | 'diff:ended' | 'diff:change_applied' | 'diff:change_rejected' | 'error:occurred' | 'error:cleared';
export interface DocumentEventData {
    documentId: string;
    title?: string;
    blockCount?: number;
    version?: number;
}
export interface BlockEventData {
    blockId: BlockId;
    parentId?: BlockId;
    content?: string;
    type?: ContentType;
    oldContent?: string;
    oldType?: ContentType;
    oldParentId?: BlockId;
    newParentId?: BlockId;
    position?: 'before' | 'after' | 'inside';
}
export interface SelectionEventData {
    blockIds: BlockId[];
    focusedBlockId?: BlockId;
    anchor?: BlockId;
    textSelection?: {
        blockId: BlockId;
        start: number;
        end: number;
    };
}
export interface EditEventData {
    blockId: BlockId;
    content?: string;
    previousContent?: string;
}
export interface DragEventData {
    sourceId: BlockId;
    targetId?: BlockId;
    position?: 'before' | 'after' | 'inside';
    x?: number;
    y?: number;
}
export interface HistoryEventData {
    entryId: string;
    description: string;
    snapshotId: string;
    operationCount?: number;
}
export interface EdgeEventData {
    sourceId: BlockId;
    targetId: BlockId;
    edgeType: EdgeType;
}
export interface ViewEventData {
    view?: EditorView;
    previousView?: EditorView;
    mode?: EditorMode;
    previousMode?: EditorMode;
}
export interface GraphEventData {
    layout?: GraphLayout;
    nodeId?: BlockId;
    viewport?: {
        x: number;
        y: number;
        zoom: number;
    };
}
export interface DiffEventData {
    leftSnapshotId?: string;
    rightSnapshotId?: string;
    blockId?: BlockId;
    changeType?: DiffChangeType;
}
export interface ErrorEventData {
    code: string;
    message: string;
    category: string;
    severity: string;
    data?: Record<string, unknown>;
}
export type EditorEventData = DocumentEventData | BlockEventData | SelectionEventData | EditEventData | DragEventData | HistoryEventData | EdgeEventData | ViewEventData | GraphEventData | DiffEventData | ErrorEventData;
export interface EditorEvent<T extends EditorEventType = EditorEventType> {
    type: T;
    timestamp: Date;
    data: EditorEventData;
    source?: string;
}
export type EditorEventHandler<T extends EditorEventType = EditorEventType> = (event: EditorEvent<T>) => void;
export interface EditorEventEmitter {
    on<T extends EditorEventType>(type: T, handler: EditorEventHandler<T>): () => void;
    once<T extends EditorEventType>(type: T, handler: EditorEventHandler<T>): () => void;
    off<T extends EditorEventType>(type: T, handler: EditorEventHandler<T>): void;
    emit<T extends EditorEventType>(type: T, data: EditorEventData): void;
    clear(): void;
}
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
export declare class SimpleEventEmitter implements EditorEventEmitter {
    private handlers;
    private onceHandlers;
    on<T extends EditorEventType>(type: T, handler: EditorEventHandler<T>): () => void;
    once<T extends EditorEventType>(type: T, handler: EditorEventHandler<T>): () => void;
    off<T extends EditorEventType>(type: T, handler: EditorEventHandler<T>): void;
    emit<T extends EditorEventType>(type: T, data: EditorEventData): void;
    clear(): void;
    /**
     * Get the number of handlers for a specific event type.
     */
    listenerCount(type: EditorEventType): number;
    /**
     * Get all registered event types.
     */
    eventTypes(): EditorEventType[];
}
