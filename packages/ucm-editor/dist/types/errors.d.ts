/**
 * Typed error system for UCM Editor.
 *
 * Uses discriminated unions for exhaustive error handling.
 * Each error has a unique code, category, and structured data.
 */
export declare const ErrorCode: {
    readonly BLOCK_NOT_FOUND: "UCM_E001";
    readonly INVALID_BLOCK_CONTENT: "UCM_E002";
    readonly BLOCK_TYPE_MISMATCH: "UCM_E003";
    readonly BLOCK_ALREADY_EXISTS: "UCM_E004";
    readonly INVALID_BLOCK_ID: "UCM_E005";
    readonly INVALID_SELECTION: "UCM_E010";
    readonly SELECTION_TARGET_NOT_FOUND: "UCM_E011";
    readonly SELECTION_OUT_OF_BOUNDS: "UCM_E012";
    readonly EDIT_FAILED: "UCM_E020";
    readonly CONCURRENT_EDIT_CONFLICT: "UCM_E021";
    readonly EDIT_VALIDATION_FAILED: "UCM_E022";
    readonly READONLY_BLOCK: "UCM_E023";
    readonly INVALID_DROP_TARGET: "UCM_E030";
    readonly CYCLE_WOULD_BE_CREATED: "UCM_E031";
    readonly DROP_NOT_ALLOWED: "UCM_E032";
    readonly NO_UNDO_AVAILABLE: "UCM_E040";
    readonly NO_REDO_AVAILABLE: "UCM_E041";
    readonly HISTORY_CORRUPTED: "UCM_E042";
    readonly SNAPSHOT_NOT_FOUND: "UCM_E050";
    readonly INCOMPATIBLE_SNAPSHOTS: "UCM_E051";
    readonly DIFF_COMPUTATION_FAILED: "UCM_E052";
    readonly LAYOUT_FAILED: "UCM_E060";
    readonly INVALID_EDGE: "UCM_E061";
    readonly NODE_NOT_FOUND: "UCM_E062";
    readonly DOCUMENT_NOT_LOADED: "UCM_E070";
    readonly DOCUMENT_VALIDATION_FAILED: "UCM_E071";
    readonly DOCUMENT_TOO_LARGE: "UCM_E072";
    readonly INTERNAL_ERROR: "UCM_E090";
    readonly INVALID_STATE: "UCM_E091";
    readonly NOT_IMPLEMENTED: "UCM_E092";
};
export type ErrorCode = (typeof ErrorCode)[keyof typeof ErrorCode];
export type ErrorCategory = 'block' | 'selection' | 'edit' | 'drag' | 'history' | 'diff' | 'graph' | 'document' | 'internal';
export type ErrorSeverity = 'error' | 'warning' | 'info';
export interface BlockErrorData {
    blockId?: string;
    expectedType?: string;
    actualType?: string;
    content?: string;
}
export interface SelectionErrorData {
    targetId?: string;
    startOffset?: number;
    endOffset?: number;
}
export interface EditErrorData {
    blockId?: string;
    operation?: string;
    reason?: string;
    conflictingVersion?: number;
}
export interface DragErrorData {
    sourceId?: string;
    targetId?: string;
    reason?: string;
}
export interface HistoryErrorData {
    action?: 'undo' | 'redo';
    stackSize?: number;
}
export interface DiffErrorData {
    snapshotId1?: string;
    snapshotId2?: string;
    reason?: string;
}
export interface GraphErrorData {
    nodeId?: string;
    edgeId?: string;
    layout?: string;
    reason?: string;
}
export interface DocumentErrorData {
    documentId?: string;
    blockCount?: number;
    maxBlocks?: number;
    validationErrors?: string[];
}
export interface InternalErrorData {
    component?: string;
    operation?: string;
    stack?: string;
}
export type EditorErrorData = BlockErrorData | SelectionErrorData | EditErrorData | DragErrorData | HistoryErrorData | DiffErrorData | GraphErrorData | DocumentErrorData | InternalErrorData;
export interface EditorErrorOptions {
    code: ErrorCode;
    message: string;
    category: ErrorCategory;
    severity?: ErrorSeverity;
    data?: EditorErrorData;
    cause?: Error;
    suggestion?: string;
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
export declare class EditorError extends Error {
    readonly code: ErrorCode;
    readonly category: ErrorCategory;
    readonly severity: ErrorSeverity;
    readonly data: EditorErrorData;
    readonly suggestion?: string;
    readonly timestamp: Date;
    constructor(options: EditorErrorOptions);
    /**
     * Create a JSON representation for logging/serialization.
     */
    toJSON(): Record<string, unknown>;
    /**
     * Format for display to users.
     */
    toUserMessage(): string;
}
/**
 * Factory functions for creating common errors.
 */
export declare const Errors: {
    readonly blockNotFound: (blockId: string) => EditorError;
    readonly invalidBlockContent: (blockId: string, reason: string) => EditorError;
    readonly blockTypeMismatch: (blockId: string, expected: string, actual: string) => EditorError;
    readonly invalidSelection: (reason: string) => EditorError;
    readonly selectionTargetNotFound: (targetId: string) => EditorError;
    readonly editFailed: (blockId: string, reason: string) => EditorError;
    readonly concurrentEditConflict: (blockId: string, version: number) => EditorError;
    readonly readonlyBlock: (blockId: string) => EditorError;
    readonly invalidDropTarget: (sourceId: string, targetId: string, reason: string) => EditorError;
    readonly cycleWouldBeCreated: (sourceId: string, targetId: string) => EditorError;
    readonly noUndoAvailable: () => EditorError;
    readonly noRedoAvailable: () => EditorError;
    readonly snapshotNotFound: (snapshotId: string) => EditorError;
    readonly incompatibleSnapshots: (id1: string, id2: string, reason: string) => EditorError;
    readonly layoutFailed: (layout: string, reason: string) => EditorError;
    readonly documentNotLoaded: () => EditorError;
    readonly documentTooLarge: (blockCount: number, maxBlocks: number) => EditorError;
    readonly internalError: (message: string, component?: string) => EditorError;
    readonly invalidState: (message: string) => EditorError;
    readonly notImplemented: (feature: string) => EditorError;
};
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
export type Result<T, E = EditorError> = {
    ok: true;
    value: T;
} | {
    ok: false;
    error: E;
};
/**
 * Create a successful result.
 */
export declare function ok<T>(value: T): Result<T, never>;
/**
 * Create a failed result.
 */
export declare function err<E>(error: E): Result<never, E>;
/**
 * Unwrap a result, throwing if it's an error.
 */
export declare function unwrap<T, E extends Error>(result: Result<T, E>): T;
/**
 * Unwrap a result or return a default value.
 */
export declare function unwrapOr<T, E>(result: Result<T, E>, defaultValue: T): T;
/**
 * Map a successful result.
 */
export declare function map<T, U, E>(result: Result<T, E>, fn: (value: T) => U): Result<U, E>;
/**
 * Chain results (flatMap).
 */
export declare function andThen<T, U, E>(result: Result<T, E>, fn: (value: T) => Result<U, E>): Result<U, E>;
