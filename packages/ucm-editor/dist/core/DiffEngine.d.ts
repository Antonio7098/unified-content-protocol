/**
 * DiffEngine - Computes differences between document snapshots.
 *
 * Provides block-level and text-level diffing for UCM documents.
 */
import type { BlockId, Document } from 'ucp-content';
import type { DocumentDiff, BlockDiff, TextDiff, DiffChangeType } from '../types/editor.js';
/**
 * Compute a complete diff between two documents.
 *
 * @example
 * ```typescript
 * const oldDoc = parseMarkdown('# Hello\n\nWorld')
 * const newDoc = parseMarkdown('# Hello\n\nNew World')
 *
 * const diff = computeDocumentDiff(oldDoc, newDoc, 'snapshot1', 'snapshot2')
 *
 * console.log(diff.summary)
 * // { added: 0, removed: 0, modified: 1, moved: 0, unchanged: 2 }
 * ```
 */
export declare function computeDocumentDiff(oldDoc: Document, newDoc: Document, fromSnapshotId: string, toSnapshotId: string): DocumentDiff;
/**
 * Get blocks that have changes.
 */
export declare function getChangedBlocks(diff: DocumentDiff): BlockDiff[];
/**
 * Get blocks by change type.
 */
export declare function getBlocksByChangeType(diff: DocumentDiff, changeType: DiffChangeType): BlockDiff[];
/**
 * Check if a specific block has changes.
 */
export declare function hasBlockChanged(diff: DocumentDiff, blockId: BlockId): boolean;
/**
 * Get the text diff for a specific block.
 */
export declare function getBlockTextDiff(diff: DocumentDiff, blockId: BlockId): TextDiff | undefined;
/**
 * Format text diff as a string with markers.
 */
export declare function formatTextDiff(textDiff: TextDiff): string;
/**
 * Check if diff has any changes.
 */
export declare function hasDiffChanges(diff: DocumentDiff): boolean;
