/**
 * Core editor types for UCM Editor.
 */
import type { BlockId, Document, Block, ContentType, SemanticRole, EdgeType } from 'ucp-content';
/** Current view mode of the editor */
export type EditorView = 'document' | 'graph' | 'diff' | 'split';
/** Editor operation mode */
export type EditorMode = 'view' | 'edit' | 'select' | 'drag';
/** Block editing state */
export type BlockEditState = 'idle' | 'editing' | 'saving' | 'error';
/** Selection within a single block */
export interface TextSelection {
    blockId: BlockId;
    start: number;
    end: number;
}
/** Multi-block selection */
export interface BlockSelection {
    blockIds: BlockId[];
    anchor?: BlockId;
    focus?: BlockId;
}
/** Combined selection state */
export interface SelectionState {
    type: 'none' | 'text' | 'block';
    text?: TextSelection;
    blocks?: BlockSelection;
    focusedBlockId?: BlockId;
}
/** Drag operation state */
export interface DragState {
    isDragging: boolean;
    sourceId?: BlockId;
    targetId?: BlockId;
    position?: 'before' | 'after' | 'inside';
    preview?: DragPreview;
}
/** Visual preview during drag */
export interface DragPreview {
    x: number;
    y: number;
    width: number;
    height: number;
    ghostElement?: HTMLElement;
}
/** Drop zone configuration */
export interface DropZone {
    id: string;
    blockId: BlockId;
    position: 'before' | 'after' | 'inside';
    rect: DOMRect;
}
/** Props for block renderers */
export interface BlockRendererProps {
    block: Block;
    document: Document;
    isSelected: boolean;
    isEditing: boolean;
    isFocused: boolean;
    isDropTarget: boolean;
    depth: number;
    path: BlockId[];
    onSelect: (blockId: BlockId) => void;
    onEdit: (blockId: BlockId) => void;
    onContentChange: (blockId: BlockId, content: string) => void;
    onTypeChange: (blockId: BlockId, type: ContentType) => void;
    onDelete: (blockId: BlockId) => void;
    onAddChild: (parentId: BlockId) => void;
    onMoveBlock: (blockId: BlockId, targetId: BlockId, position: 'before' | 'after' | 'inside') => void;
}
/** Props for content-type-specific editors */
export interface ContentEditorProps {
    block: Block;
    isEditing: boolean;
    onChange: (content: string) => void;
    onSave: () => void;
    onCancel: () => void;
    onKeyDown?: (event: React.KeyboardEvent) => void;
}
/** Graph layout algorithm */
export type GraphLayout = 'hierarchical' | 'force' | 'dagre' | 'radial';
/** Graph node for visualization */
export interface GraphNode {
    id: BlockId;
    x: number;
    y: number;
    width: number;
    height: number;
    block: Block;
    depth: number;
    isExpanded: boolean;
    isSelected: boolean;
    isHighlighted: boolean;
}
/** Graph edge for visualization */
export interface GraphEdge {
    id: string;
    sourceId: BlockId;
    targetId: BlockId;
    edgeType: EdgeType | 'parent_child';
    points: Array<{
        x: number;
        y: number;
    }>;
    isHighlighted: boolean;
}
/** Graph view state */
export interface GraphViewState {
    layout: GraphLayout;
    nodes: Map<BlockId, GraphNode>;
    edges: GraphEdge[];
    viewport: {
        x: number;
        y: number;
        zoom: number;
    };
    selectedNodeId?: BlockId;
    hoveredNodeId?: BlockId;
    showEdgeLabels: boolean;
    edgeFilter: EdgeType[];
}
/** Type of change in a diff */
export type DiffChangeType = 'added' | 'removed' | 'modified' | 'moved' | 'unchanged';
/** Diff for a single block */
export interface BlockDiff {
    blockId: BlockId;
    changeType: DiffChangeType;
    oldBlock?: Block;
    newBlock?: Block;
    contentDiff?: TextDiff;
    metadataChanges?: MetadataChange[];
}
/** Text-level diff */
export interface TextDiff {
    operations: Array<{
        type: 'equal';
        text: string;
    } | {
        type: 'insert';
        text: string;
    } | {
        type: 'delete';
        text: string;
    }>;
}
/** Metadata change */
export interface MetadataChange {
    field: string;
    oldValue: unknown;
    newValue: unknown;
}
/** Structural change in the document tree */
export interface StructuralChange {
    type: 'added' | 'removed' | 'moved' | 'reordered';
    blockId: BlockId;
    oldParentId?: BlockId;
    newParentId?: BlockId;
    oldIndex?: number;
    newIndex?: number;
}
/** Complete document diff */
export interface DocumentDiff {
    fromSnapshotId: string;
    toSnapshotId: string;
    blockDiffs: Map<BlockId, BlockDiff>;
    structuralChanges: StructuralChange[];
    summary: {
        added: number;
        removed: number;
        modified: number;
        moved: number;
        unchanged: number;
    };
}
/** Diff view state */
export interface DiffViewState {
    isComparing: boolean;
    leftSnapshotId?: string;
    rightSnapshotId?: string;
    diff?: DocumentDiff;
    selectedChangeId?: BlockId;
    showUnchanged: boolean;
    viewMode: 'unified' | 'split';
}
/** A recorded action for undo/redo */
export interface HistoryEntry {
    id: string;
    timestamp: Date;
    description: string;
    snapshotId: string;
    operations: HistoryOperation[];
}
/** Individual operation in history */
export interface HistoryOperation {
    type: 'add_block' | 'delete_block' | 'edit_block' | 'move_block' | 'add_edge' | 'remove_edge' | 'change_type' | 'batch';
    blockId?: BlockId;
    data?: Record<string, unknown>;
}
/** History state */
export interface HistoryState {
    entries: HistoryEntry[];
    currentIndex: number;
    maxEntries: number;
    canUndo: boolean;
    canRedo: boolean;
}
/** Metadata to show in tooltip */
export interface BlockMetadataDisplay {
    id: BlockId;
    type: ContentType;
    role?: SemanticRole;
    label?: string;
    tags: string[];
    createdAt?: Date;
    modifiedAt?: Date;
    tokenEstimate?: number;
    edgeCount: number;
    childCount: number;
    custom: Record<string, unknown>;
}
/** Editor configuration options */
export interface EditorConfig {
    /** Maximum blocks to render before virtualization */
    virtualizationThreshold: number;
    /** Debounce delay for auto-save (ms) */
    autoSaveDelay: number;
    /** Maximum history entries to keep */
    maxHistoryEntries: number;
    /** Enable keyboard shortcuts */
    enableKeyboardShortcuts: boolean;
    /** Enable drag and drop */
    enableDragDrop: boolean;
    /** Show block IDs in UI */
    showBlockIds: boolean;
    /** Default graph layout */
    defaultGraphLayout: GraphLayout;
    /** Log level */
    logLevel: 'debug' | 'info' | 'warn' | 'error';
}
/** Default editor configuration */
export declare const DEFAULT_EDITOR_CONFIG: EditorConfig;
/** Complete editor store state */
export interface EditorStoreState {
    document: Document | null;
    documentId: string | null;
    isLoading: boolean;
    isDirty: boolean;
    lastSaved?: Date;
    view: EditorView;
    mode: EditorMode;
    selection: SelectionState;
    editingBlockId: BlockId | null;
    editState: BlockEditState;
    pendingContent: string | null;
    drag: DragState;
    graph: GraphViewState;
    diff: DiffViewState;
    history: HistoryState;
    config: EditorConfig;
    lastError: Error | null;
}
/** Actions for the editor store */
export interface EditorStoreActions {
    loadDocument: (doc: Document) => void;
    createDocument: (title?: string) => void;
    saveDocument: () => Promise<void>;
    setView: (view: EditorView) => void;
    setMode: (mode: EditorMode) => void;
    addBlock: (parentId: BlockId, content: string, type?: ContentType) => BlockId;
    editBlock: (blockId: BlockId, content: string) => void;
    deleteBlock: (blockId: BlockId, cascade?: boolean) => void;
    moveBlock: (blockId: BlockId, targetId: BlockId, position: 'before' | 'after' | 'inside') => void;
    changeBlockType: (blockId: BlockId, type: ContentType) => void;
    select: (blockId: BlockId) => void;
    selectMultiple: (blockIds: BlockId[]) => void;
    clearSelection: () => void;
    selectText: (blockId: BlockId, start: number, end: number) => void;
    startEditing: (blockId: BlockId) => void;
    stopEditing: (save?: boolean) => void;
    updatePendingContent: (content: string) => void;
    startDrag: (blockId: BlockId) => void;
    updateDragTarget: (targetId: BlockId, position: 'before' | 'after' | 'inside') => void;
    endDrag: (drop?: boolean) => void;
    undo: () => void;
    redo: () => void;
    createSnapshot: (description: string) => void;
    setGraphLayout: (layout: GraphLayout) => void;
    setGraphViewport: (x: number, y: number, zoom: number) => void;
    toggleNodeExpansion: (nodeId: BlockId) => void;
    startCompare: (leftSnapshotId: string, rightSnapshotId: string) => void;
    stopCompare: () => void;
    applyChange: (blockId: BlockId) => void;
    rejectChange: (blockId: BlockId) => void;
    addEdge: (sourceId: BlockId, targetId: BlockId, edgeType: EdgeType) => void;
    removeEdge: (sourceId: BlockId, targetId: BlockId, edgeType: EdgeType) => void;
    updateConfig: (config: Partial<EditorConfig>) => void;
    clearError: () => void;
}
/** Complete editor store */
export type EditorStore = EditorStoreState & EditorStoreActions;
