/**
 * Typed error system for UCM Editor.
 *
 * Uses discriminated unions for exhaustive error handling.
 * Each error has a unique code, category, and structured data.
 */

// =============================================================================
// ERROR CODES
// =============================================================================

export const ErrorCode = {
  // Block errors (E001-E009)
  BLOCK_NOT_FOUND: 'UCM_E001',
  INVALID_BLOCK_CONTENT: 'UCM_E002',
  BLOCK_TYPE_MISMATCH: 'UCM_E003',
  BLOCK_ALREADY_EXISTS: 'UCM_E004',
  INVALID_BLOCK_ID: 'UCM_E005',

  // Selection errors (E010-E019)
  INVALID_SELECTION: 'UCM_E010',
  SELECTION_TARGET_NOT_FOUND: 'UCM_E011',
  SELECTION_OUT_OF_BOUNDS: 'UCM_E012',

  // Edit errors (E020-E029)
  EDIT_FAILED: 'UCM_E020',
  CONCURRENT_EDIT_CONFLICT: 'UCM_E021',
  EDIT_VALIDATION_FAILED: 'UCM_E022',
  READONLY_BLOCK: 'UCM_E023',

  // Drag/Drop errors (E030-E039)
  INVALID_DROP_TARGET: 'UCM_E030',
  CYCLE_WOULD_BE_CREATED: 'UCM_E031',
  DROP_NOT_ALLOWED: 'UCM_E032',

  // History errors (E040-E049)
  NO_UNDO_AVAILABLE: 'UCM_E040',
  NO_REDO_AVAILABLE: 'UCM_E041',
  HISTORY_CORRUPTED: 'UCM_E042',

  // Diff errors (E050-E059)
  SNAPSHOT_NOT_FOUND: 'UCM_E050',
  INCOMPATIBLE_SNAPSHOTS: 'UCM_E051',
  DIFF_COMPUTATION_FAILED: 'UCM_E052',

  // Graph errors (E060-E069)
  LAYOUT_FAILED: 'UCM_E060',
  INVALID_EDGE: 'UCM_E061',
  NODE_NOT_FOUND: 'UCM_E062',

  // Document errors (E070-E079)
  DOCUMENT_NOT_LOADED: 'UCM_E070',
  DOCUMENT_VALIDATION_FAILED: 'UCM_E071',
  DOCUMENT_TOO_LARGE: 'UCM_E072',

  // Internal errors (E090-E099)
  INTERNAL_ERROR: 'UCM_E090',
  INVALID_STATE: 'UCM_E091',
  NOT_IMPLEMENTED: 'UCM_E092',
} as const

export type ErrorCode = (typeof ErrorCode)[keyof typeof ErrorCode]

// =============================================================================
// ERROR CATEGORIES
// =============================================================================

export type ErrorCategory =
  | 'block'
  | 'selection'
  | 'edit'
  | 'drag'
  | 'history'
  | 'diff'
  | 'graph'
  | 'document'
  | 'internal'

export type ErrorSeverity = 'error' | 'warning' | 'info'

// =============================================================================
// ERROR DATA TYPES
// =============================================================================

export interface BlockErrorData {
  blockId?: string
  expectedType?: string
  actualType?: string
  content?: string
}

export interface SelectionErrorData {
  targetId?: string
  startOffset?: number
  endOffset?: number
}

export interface EditErrorData {
  blockId?: string
  operation?: string
  reason?: string
  conflictingVersion?: number
}

export interface DragErrorData {
  sourceId?: string
  targetId?: string
  reason?: string
}

export interface HistoryErrorData {
  action?: 'undo' | 'redo'
  stackSize?: number
}

export interface DiffErrorData {
  snapshotId1?: string
  snapshotId2?: string
  reason?: string
}

export interface GraphErrorData {
  nodeId?: string
  edgeId?: string
  layout?: string
  reason?: string
}

export interface DocumentErrorData {
  documentId?: string
  blockCount?: number
  maxBlocks?: number
  validationErrors?: string[]
}

export interface InternalErrorData {
  component?: string
  operation?: string
  stack?: string
}

// =============================================================================
// EDITOR ERROR CLASS
// =============================================================================

export type EditorErrorData =
  | BlockErrorData
  | SelectionErrorData
  | EditErrorData
  | DragErrorData
  | HistoryErrorData
  | DiffErrorData
  | GraphErrorData
  | DocumentErrorData
  | InternalErrorData

export interface EditorErrorOptions {
  code: ErrorCode
  message: string
  category: ErrorCategory
  severity?: ErrorSeverity
  data?: EditorErrorData
  cause?: Error
  suggestion?: string
}

/**
 * Structured error for the UCM Editor.
 *
 * Provides rich error information for debugging and user feedback.
 *
 * @example
 * ```typescript
 * throw new EditorError({
 *   code: ErrorCode.BLOCK_NOT_FOUND,
 *   message: 'Block not found in document',
 *   category: 'block',
 *   data: { blockId: 'blk_123' },
 *   suggestion: 'Ensure the block exists before editing',
 * })
 * ```
 */
export class EditorError extends Error {
  readonly code: ErrorCode
  readonly category: ErrorCategory
  readonly severity: ErrorSeverity
  readonly data: EditorErrorData
  readonly suggestion?: string
  readonly timestamp: Date

  constructor(options: EditorErrorOptions) {
    super(options.message)
    this.name = 'EditorError'
    this.code = options.code
    this.category = options.category
    this.severity = options.severity ?? 'error'
    this.data = options.data ?? {}
    this.suggestion = options.suggestion
    this.timestamp = new Date()

    if (options.cause) {
      this.cause = options.cause
    }

    // Capture stack trace
    type CaptureStackTraceFn = (
      target: object,
      constructorOpt: new (...args: unknown[]) => unknown
    ) => void
    const captureStackTrace = (Error as ErrorConstructor & {
      captureStackTrace?: CaptureStackTraceFn
    }).captureStackTrace

    if (captureStackTrace) {
      captureStackTrace(
        this,
        EditorError as unknown as new (...args: unknown[]) => unknown
      )
    }
  }

  /**
   * Create a JSON representation for logging/serialization.
   */
  toJSON(): Record<string, unknown> {
    return {
      name: this.name,
      code: this.code,
      message: this.message,
      category: this.category,
      severity: this.severity,
      data: this.data,
      suggestion: this.suggestion,
      timestamp: this.timestamp.toISOString(),
      stack: this.stack,
    }
  }

  /**
   * Format for display to users.
   */
  toUserMessage(): string {
    const parts = [`[${this.code}] ${this.message}`]
    if (this.suggestion) {
      parts.push(`Suggestion: ${this.suggestion}`)
    }
    return parts.join('\n')
  }
}

// =============================================================================
// ERROR FACTORIES
// =============================================================================

/**
 * Factory functions for creating common errors.
 */
export const Errors = {
  // Block errors
  blockNotFound(blockId: string): EditorError {
    return new EditorError({
      code: ErrorCode.BLOCK_NOT_FOUND,
      message: `Block not found: ${blockId}`,
      category: 'block',
      data: { blockId },
      suggestion: 'Verify the block ID exists in the document',
    })
  },

  invalidBlockContent(blockId: string, reason: string): EditorError {
    return new EditorError({
      code: ErrorCode.INVALID_BLOCK_CONTENT,
      message: `Invalid content for block ${blockId}: ${reason}`,
      category: 'block',
      data: { blockId },
      suggestion: 'Check the content format matches the block type',
    })
  },

  blockTypeMismatch(blockId: string, expected: string, actual: string): EditorError {
    return new EditorError({
      code: ErrorCode.BLOCK_TYPE_MISMATCH,
      message: `Block ${blockId} type mismatch: expected ${expected}, got ${actual}`,
      category: 'block',
      data: { blockId, expectedType: expected, actualType: actual },
      suggestion: 'Use the correct editor for this block type',
    })
  },

  // Selection errors
  invalidSelection(reason: string): EditorError {
    return new EditorError({
      code: ErrorCode.INVALID_SELECTION,
      message: `Invalid selection: ${reason}`,
      category: 'selection',
      suggestion: 'Click on a block to select it',
    })
  },

  selectionTargetNotFound(targetId: string): EditorError {
    return new EditorError({
      code: ErrorCode.SELECTION_TARGET_NOT_FOUND,
      message: `Selection target not found: ${targetId}`,
      category: 'selection',
      data: { targetId },
    })
  },

  // Edit errors
  editFailed(blockId: string, reason: string): EditorError {
    return new EditorError({
      code: ErrorCode.EDIT_FAILED,
      message: `Edit failed for block ${blockId}: ${reason}`,
      category: 'edit',
      data: { blockId, reason },
    })
  },

  concurrentEditConflict(blockId: string, version: number): EditorError {
    return new EditorError({
      code: ErrorCode.CONCURRENT_EDIT_CONFLICT,
      message: `Concurrent edit conflict on block ${blockId}`,
      category: 'edit',
      data: { blockId, conflictingVersion: version },
      suggestion: 'Refresh the document and try again',
    })
  },

  readonlyBlock(blockId: string): EditorError {
    return new EditorError({
      code: ErrorCode.READONLY_BLOCK,
      message: `Block ${blockId} is read-only`,
      category: 'edit',
      data: { blockId },
      severity: 'warning',
    })
  },

  // Drag/Drop errors
  invalidDropTarget(sourceId: string, targetId: string, reason: string): EditorError {
    return new EditorError({
      code: ErrorCode.INVALID_DROP_TARGET,
      message: `Cannot drop ${sourceId} onto ${targetId}: ${reason}`,
      category: 'drag',
      data: { sourceId, targetId, reason },
    })
  },

  cycleWouldBeCreated(sourceId: string, targetId: string): EditorError {
    return new EditorError({
      code: ErrorCode.CYCLE_WOULD_BE_CREATED,
      message: `Moving ${sourceId} to ${targetId} would create a cycle`,
      category: 'drag',
      data: { sourceId, targetId },
      suggestion: 'Choose a different drop target',
    })
  },

  // History errors
  noUndoAvailable(): EditorError {
    return new EditorError({
      code: ErrorCode.NO_UNDO_AVAILABLE,
      message: 'No undo action available',
      category: 'history',
      severity: 'info',
      data: { action: 'undo' },
    })
  },

  noRedoAvailable(): EditorError {
    return new EditorError({
      code: ErrorCode.NO_REDO_AVAILABLE,
      message: 'No redo action available',
      category: 'history',
      severity: 'info',
      data: { action: 'redo' },
    })
  },

  // Diff errors
  snapshotNotFound(snapshotId: string): EditorError {
    return new EditorError({
      code: ErrorCode.SNAPSHOT_NOT_FOUND,
      message: `Snapshot not found: ${snapshotId}`,
      category: 'diff',
      data: { snapshotId1: snapshotId },
    })
  },

  incompatibleSnapshots(id1: string, id2: string, reason: string): EditorError {
    return new EditorError({
      code: ErrorCode.INCOMPATIBLE_SNAPSHOTS,
      message: `Snapshots ${id1} and ${id2} are incompatible: ${reason}`,
      category: 'diff',
      data: { snapshotId1: id1, snapshotId2: id2, reason },
    })
  },

  // Graph errors
  layoutFailed(layout: string, reason: string): EditorError {
    return new EditorError({
      code: ErrorCode.LAYOUT_FAILED,
      message: `Layout computation failed (${layout}): ${reason}`,
      category: 'graph',
      data: { layout, reason },
    })
  },

  // Document errors
  documentNotLoaded(): EditorError {
    return new EditorError({
      code: ErrorCode.DOCUMENT_NOT_LOADED,
      message: 'No document is currently loaded',
      category: 'document',
      suggestion: 'Load a document before performing this action',
    })
  },

  documentTooLarge(blockCount: number, maxBlocks: number): EditorError {
    return new EditorError({
      code: ErrorCode.DOCUMENT_TOO_LARGE,
      message: `Document has ${blockCount} blocks, maximum is ${maxBlocks}`,
      category: 'document',
      data: { blockCount, maxBlocks },
      suggestion: 'Split the document into smaller parts',
    })
  },

  // Internal errors
  internalError(message: string, component?: string): EditorError {
    return new EditorError({
      code: ErrorCode.INTERNAL_ERROR,
      message: `Internal error: ${message}`,
      category: 'internal',
      data: { component },
    })
  },

  invalidState(message: string): EditorError {
    return new EditorError({
      code: ErrorCode.INVALID_STATE,
      message: `Invalid state: ${message}`,
      category: 'internal',
    })
  },

  notImplemented(feature: string): EditorError {
    return new EditorError({
      code: ErrorCode.NOT_IMPLEMENTED,
      message: `Not implemented: ${feature}`,
      category: 'internal',
      severity: 'warning',
    })
  },
} as const

// =============================================================================
// RESULT TYPE
// =============================================================================

/**
 * Result type for operations that can fail.
 *
 * @example
 * ```typescript
 * function getBlock(id: string): Result<Block, EditorError> {
 *   const block = doc.blocks.get(id)
 *   if (!block) {
 *     return { ok: false, error: Errors.blockNotFound(id) }
 *   }
 *   return { ok: true, value: block }
 * }
 *
 * const result = getBlock('blk_123')
 * if (result.ok) {
 *   console.log(result.value.content)
 * } else {
 *   console.error(result.error.message)
 * }
 * ```
 */
export type Result<T, E = EditorError> = { ok: true; value: T } | { ok: false; error: E }

/**
 * Create a successful result.
 */
export function ok<T>(value: T): Result<T, never> {
  return { ok: true, value }
}

/**
 * Create a failed result.
 */
export function err<E>(error: E): Result<never, E> {
  return { ok: false, error }
}

/**
 * Unwrap a result, throwing if it's an error.
 */
export function unwrap<T, E extends Error>(result: Result<T, E>): T {
  if (result.ok) {
    return result.value
  }
  throw result.error
}

/**
 * Unwrap a result or return a default value.
 */
export function unwrapOr<T, E>(result: Result<T, E>, defaultValue: T): T {
  return result.ok ? result.value : defaultValue
}

/**
 * Map a successful result.
 */
export function map<T, U, E>(result: Result<T, E>, fn: (value: T) => U): Result<U, E> {
  if (result.ok) {
    return ok(fn(result.value))
  }
  return result
}

/**
 * Chain results (flatMap).
 */
export function andThen<T, U, E>(
  result: Result<T, E>,
  fn: (value: T) => Result<U, E>
): Result<U, E> {
  if (result.ok) {
    return fn(result.value)
  }
  return result
}
