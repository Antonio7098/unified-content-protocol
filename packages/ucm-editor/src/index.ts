/**
 * UCM Editor - Notion-like editor for UCM documents
 *
 * A comprehensive editor for viewing and editing UCM (Unified Content Model)
 * documents with support for:
 * - Block-based editing with Notion-like UX
 * - Graph visualization of document structure
 * - Diff viewing for comparing snapshots
 * - Drag-and-drop block reordering
 * - Keyboard shortcuts and accessibility
 *
 * @example
 * ```tsx
 * import { Editor } from 'ucm-editor'
 * import { parseMarkdown } from 'ucp-js'
 *
 * const doc = parseMarkdown('# Hello World\n\nWelcome to UCM Editor!')
 *
 * function App() {
 *   return (
 *     <Editor
 *       document={doc}
 *       onChange={(doc) => console.log('Document changed:', doc)}
 *     />
 *   )
 * }
 * ```
 *
 * @packageDocumentation
 */

// =============================================================================
// TYPES
// =============================================================================

export type {
  // Error types
  ErrorCode,
  ErrorCategory,
  ErrorSeverity,
  EditorErrorData,
  EditorErrorOptions,
  Result,

  // Editor types
  EditorView,
  EditorMode,
  BlockEditState,
  TextSelection,
  BlockSelection,
  SelectionState,
  DragState,
  DragPreview,
  DropZone,
  ContentEditorProps,
  GraphLayout,
  GraphNode,
  GraphEdge,
  GraphViewState,
  DiffChangeType,
  BlockDiff,
  TextDiff,
  MetadataChange,
  StructuralChange,
  DocumentDiff,
  DiffViewState,
  HistoryEntry,
  HistoryOperation,
  HistoryState,
  BlockMetadataDisplay,
  EditorConfig,
  EditorStoreState,
  EditorStoreActions,
  EditorStore,

  // Event types
  EditorEventType,
  EditorEventData,
  EditorEvent,
  EditorEventHandler,
  EditorEventEmitter,
} from './types/index.js'

export {
  EditorError,
  Errors,
  ok,
  err,
  unwrap,
  unwrapOr,
  map,
  andThen,
  DEFAULT_EDITOR_CONFIG,
  SimpleEventEmitter,
} from './types/index.js'

// =============================================================================
// CORE
// =============================================================================

export type { LogLevel, LogEntry, LogHandler, LoggerConfig } from './core/index.js'

export {
  Logger,
  logger,
  configureLogger,
  consoleHandler,
  createBufferHandler,
  createEditorStore,
  type EditorStoreInstance,
  SelectionManager,
  type SelectionManagerConfig,
  createEmptySelection,
  createBlockSelection,
  createMultiBlockSelection,
  createTextSelection,
  isBlockSelected,
  isBlockFocused,
  getSelectedBlockIds,
  getPrimarySelectedBlock,
  getTextSelection,
  isSelectionEmpty,
  isTextSelection,
  isBlockSelectionType,
  getBlockOrder,
  getNextBlock,
  getPreviousBlock,
  getParentBlock,
  getFirstChildBlock,
  getSiblingBlocks,
  getNextSibling,
  getPreviousSibling,
  expandSelection,
  computeDocumentDiff,
  getChangedBlocks,
  getBlocksByChangeType,
  hasBlockChanged,
  getBlockTextDiff,
  formatTextDiff,
  hasDiffChanges,
} from './core/index.js'

// =============================================================================
// HOOKS
// =============================================================================

export {
  useEditorStore,
  useEditorState,
  useDocument,
  useSelection,
  useHistory,
  useDrag,
  useView,
  useBlockActions,
  useEditActions,
  useEditorEvent,
  useKeyboardShortcuts,
  type KeyboardShortcuts,
} from './hooks/index.js'

// =============================================================================
// COMPONENTS
// =============================================================================

export { Editor, type EditorProps } from './components/index.js'
export { BlockRenderer, type BlockRendererProps } from './components/index.js'
export { BlockEditor, type BlockEditorProps } from './components/index.js'
export { MetadataTooltip, type MetadataTooltipProps } from './components/index.js'

// =============================================================================
// GRAPH
// =============================================================================

export { GraphView, type GraphViewProps } from './graph/index.js'

// =============================================================================
// DIFF
// =============================================================================

export { DiffViewer, type DiffViewerProps } from './diff/index.js'
