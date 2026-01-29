/**
 * SelectionManager - Handles selection state and navigation.
 *
 * Manages block selection, text selection, and keyboard navigation.
 */
import type { BlockId, Document } from 'ucp-content';
import type { SelectionState, TextSelection } from '../types/editor.js';
/**
 * Create an empty selection state.
 */
export declare function createEmptySelection(): SelectionState;
/**
 * Create a single block selection.
 */
export declare function createBlockSelection(blockId: BlockId): SelectionState;
/**
 * Create a multi-block selection.
 */
export declare function createMultiBlockSelection(blockIds: BlockId[], anchor?: BlockId, focus?: BlockId): SelectionState;
/**
 * Create a text selection within a block.
 */
export declare function createTextSelection(blockId: BlockId, start: number, end: number): SelectionState;
/**
 * Check if a block is selected.
 */
export declare function isBlockSelected(selection: SelectionState, blockId: BlockId): boolean;
/**
 * Check if a block is the focused block.
 */
export declare function isBlockFocused(selection: SelectionState, blockId: BlockId): boolean;
/**
 * Get all selected block IDs.
 */
export declare function getSelectedBlockIds(selection: SelectionState): BlockId[];
/**
 * Get the primary selected block (anchor).
 */
export declare function getPrimarySelectedBlock(selection: SelectionState): BlockId | undefined;
/**
 * Get text selection details.
 */
export declare function getTextSelection(selection: SelectionState): TextSelection | undefined;
/**
 * Check if selection is empty.
 */
export declare function isSelectionEmpty(selection: SelectionState): boolean;
/**
 * Check if selection is a text selection.
 */
export declare function isTextSelection(selection: SelectionState): boolean;
/**
 * Check if selection is a block selection.
 */
export declare function isBlockSelectionType(selection: SelectionState): boolean;
/**
 * Get the block order for navigation.
 */
export declare function getBlockOrder(doc: Document): BlockId[];
/**
 * Get the next block in document order.
 */
export declare function getNextBlock(doc: Document, currentId: BlockId): BlockId | undefined;
/**
 * Get the previous block in document order.
 */
export declare function getPreviousBlock(doc: Document, currentId: BlockId): BlockId | undefined;
/**
 * Get the parent block.
 */
export declare function getParentBlock(doc: Document, blockId: BlockId): BlockId | undefined;
/**
 * Get the first child block.
 */
export declare function getFirstChildBlock(doc: Document, blockId: BlockId): BlockId | undefined;
/**
 * Get sibling blocks.
 */
export declare function getSiblingBlocks(doc: Document, blockId: BlockId): BlockId[];
/**
 * Get the next sibling.
 */
export declare function getNextSibling(doc: Document, blockId: BlockId): BlockId | undefined;
/**
 * Get the previous sibling.
 */
export declare function getPreviousSibling(doc: Document, blockId: BlockId): BlockId | undefined;
/**
 * Expand selection to include another block.
 */
export declare function expandSelection(doc: Document, selection: SelectionState, targetId: BlockId): SelectionState;
export interface SelectionManagerConfig {
    onSelectionChange?: (selection: SelectionState) => void;
}
/**
 * Selection manager for handling selection state and navigation.
 *
 * @example
 * ```typescript
 * const manager = new SelectionManager(doc, {
 *   onSelectionChange: (selection) => {
 *     console.log('Selection changed:', selection)
 *   }
 * })
 *
 * manager.select('blk_123')
 * manager.moveNext()
 * manager.expandToBlock('blk_456')
 * ```
 */
export declare class SelectionManager {
    private doc;
    private selection;
    private config;
    constructor(doc: Document, config?: SelectionManagerConfig);
    /**
     * Update the document reference.
     */
    setDocument(doc: Document): void;
    /**
     * Get current selection.
     */
    getSelection(): SelectionState;
    /**
     * Update selection and notify.
     */
    private setSelection;
    /**
     * Select a single block.
     */
    select(blockId: BlockId): void;
    /**
     * Select multiple blocks.
     */
    selectMultiple(blockIds: BlockId[]): void;
    /**
     * Clear selection.
     */
    clear(): void;
    /**
     * Select text within a block.
     */
    selectText(blockId: BlockId, start: number, end: number): void;
    /**
     * Expand selection to include another block.
     */
    expandTo(blockId: BlockId): void;
    /**
     * Move selection to next block.
     */
    moveNext(): boolean;
    /**
     * Move selection to previous block.
     */
    movePrevious(): boolean;
    /**
     * Move selection to parent block.
     */
    moveToParent(): boolean;
    /**
     * Move selection to first child.
     */
    moveToFirstChild(): boolean;
    /**
     * Expand selection to next block.
     */
    expandNext(): boolean;
    /**
     * Expand selection to previous block.
     */
    expandPrevious(): boolean;
    /**
     * Select all blocks.
     */
    selectAll(): void;
    /**
     * Check if block is selected.
     */
    isSelected(blockId: BlockId): boolean;
    /**
     * Check if block is focused.
     */
    isFocused(blockId: BlockId): boolean;
}
