/**
 * Core module exports for UCM Editor.
 */
export { type LogLevel, type LogEntry, type LogHandler, type LoggerConfig, Logger, logger, configureLogger, consoleHandler, createBufferHandler, } from './Logger.js';
export { createEditorStore, type EditorStoreInstance } from './EditorStore.js';
export { createEmptySelection, createBlockSelection, createMultiBlockSelection, createTextSelection, isBlockSelected, isBlockFocused, getSelectedBlockIds, getPrimarySelectedBlock, getTextSelection, isSelectionEmpty, isTextSelection, isBlockSelectionType, getBlockOrder, getNextBlock, getPreviousBlock, getParentBlock, getFirstChildBlock, getSiblingBlocks, getNextSibling, getPreviousSibling, expandSelection, SelectionManager, type SelectionManagerConfig, } from './SelectionManager.js';
export { computeDocumentDiff, getChangedBlocks, getBlocksByChangeType, hasBlockChanged, getBlockTextDiff, formatTextDiff, hasDiffChanges, } from './DiffEngine.js';
