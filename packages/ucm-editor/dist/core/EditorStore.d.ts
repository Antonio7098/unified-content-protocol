/**
 * EditorStore - Core state management for UCM Editor.
 *
 * Uses a simple observable store pattern with immutable updates.
 * Implements SOLID principles with clear separation of concerns.
 */
import type { EditorStoreState, EditorStore, EditorConfig } from '../types/editor.js';
import type { EditorEventEmitter } from '../types/events.js';
type StoreListener = (state: EditorStoreState, prevState: EditorStoreState) => void;
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
export declare function createEditorStore(initialConfig?: Partial<EditorConfig>): EditorStore & {
    subscribe: (listener: StoreListener) => () => void;
    getState: () => EditorStoreState;
    events: EditorEventEmitter;
};
export type EditorStoreInstance = ReturnType<typeof createEditorStore>;
export {};
