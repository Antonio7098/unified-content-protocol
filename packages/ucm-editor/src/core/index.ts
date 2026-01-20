/**
 * Core module exports for UCM Editor.
 */

// Logger
export {
  type LogLevel,
  type LogEntry,
  type LogHandler,
  type LoggerConfig,
  Logger,
  logger,
  configureLogger,
  consoleHandler,
  createBufferHandler,
} from './Logger.js'

// Editor Store
export { createEditorStore, type EditorStoreInstance } from './EditorStore.js'

// Selection Manager
export {
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
  SelectionManager,
  type SelectionManagerConfig,
} from './SelectionManager.js'

// Diff Engine
export {
  computeDocumentDiff,
  getChangedBlocks,
  getBlocksByChangeType,
  hasBlockChanged,
  getBlockTextDiff,
  formatTextDiff,
  hasDiffChanges,
} from './DiffEngine.js'
