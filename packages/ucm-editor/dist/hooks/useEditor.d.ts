/**
 * useEditor - Main React hook for the UCM Editor.
 *
 * Provides access to the editor store and reactive state updates.
 */
import type { Document } from 'ucp-content';
import { type EditorStoreInstance } from '../core/EditorStore.js';
import type { EditorStoreState, EditorConfig } from '../types/editor.js';
type EditorEventTypeParam = Parameters<EditorStoreInstance['events']['on']>[0];
type EditorEventHandlerParam = Parameters<EditorStoreInstance['events']['on']>[1];
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
export declare function useEditorStore(config?: Partial<EditorConfig>): EditorStoreInstance;
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
export declare function useEditorState<T>(store: EditorStoreInstance, selector: (state: EditorStoreState) => T): T;
/**
 * Hook to access the current document.
 */
export declare function useDocument(store: EditorStoreInstance): Document | null;
/**
 * Hook to access selection state.
 */
export declare function useSelection(store: EditorStoreInstance): {
    selection: import("../index.js").SelectionState;
    editingBlockId: string | null;
    isBlockSelected: (blockId: string) => boolean;
    isBlockFocused: (blockId: string) => boolean;
    isBlockEditing: (blockId: string) => boolean;
};
/**
 * Hook to access history state and actions.
 */
export declare function useHistory(store: EditorStoreInstance): {
    canUndo: boolean;
    canRedo: boolean;
    entries: import("../index.js").HistoryEntry[];
    currentIndex: number;
    undo: () => void;
    redo: () => void;
};
/**
 * Hook to access drag state.
 */
export declare function useDrag(store: EditorStoreInstance): {
    isDragging: boolean;
    sourceId: string | undefined;
    targetId: string | undefined;
    position: "before" | "after" | "inside" | undefined;
    startDrag: (blockId: import("ucp-content").BlockId) => void;
    updateTarget: (targetId: import("ucp-content").BlockId, position: "before" | "after" | "inside") => void;
    endDrag: (drop?: boolean) => void;
};
/**
 * Hook to access view state.
 */
export declare function useView(store: EditorStoreInstance): {
    view: import("../index.js").EditorView;
    mode: import("../index.js").EditorMode;
    setView: (view: import("../index.js").EditorView) => void;
    setMode: (mode: import("../index.js").EditorMode) => void;
};
/**
 * Hook to access block manipulation actions.
 */
export declare function useBlockActions(store: EditorStoreInstance): {
    addBlock: (parentId: import("ucp-content").BlockId, content: string, type?: import("ucp-content").ContentType) => import("ucp-content").BlockId;
    editBlock: (blockId: import("ucp-content").BlockId, content: string) => void;
    deleteBlock: (blockId: import("ucp-content").BlockId, cascade?: boolean) => void;
    moveBlock: (blockId: import("ucp-content").BlockId, targetId: import("ucp-content").BlockId, position: "before" | "after" | "inside") => void;
    changeBlockType: (blockId: import("ucp-content").BlockId, type: import("ucp-content").ContentType) => void;
};
/**
 * Hook to access edit actions.
 */
export declare function useEditActions(store: EditorStoreInstance): {
    editingBlockId: string | null;
    pendingContent: string | null;
    editState: import("../index.js").BlockEditState;
    startEditing: (blockId: import("ucp-content").BlockId) => void;
    stopEditing: (save?: boolean) => void;
    updateContent: (content: string) => void;
};
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
export declare function useEditorEvent<T extends EditorEventTypeParam>(store: EditorStoreInstance, eventType: T, handler: EditorEventHandlerParam): void;
export interface KeyboardShortcuts {
    'mod+z': () => void;
    'mod+shift+z': () => void;
    'mod+y': () => void;
    'mod+s': () => void;
    'mod+a': () => void;
    escape: () => void;
    enter: () => void;
    delete: () => void;
    backspace: () => void;
    arrowUp: () => void;
    arrowDown: () => void;
    arrowLeft: () => void;
    arrowRight: () => void;
    tab: () => void;
    'shift+tab': () => void;
}
/**
 * Hook to handle keyboard shortcuts.
 */
export declare function useKeyboardShortcuts(store: EditorStoreInstance, enabled?: boolean): void;
export {};
