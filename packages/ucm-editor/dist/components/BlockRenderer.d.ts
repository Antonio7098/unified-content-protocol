/**
 * BlockRenderer - Renders a block based on its type and role.
 *
 * Handles selection, editing, drag-and-drop, and metadata display.
 */
import React from 'react';
import type { Block, Document, BlockId } from 'ucp-content';
import type { EditorStoreInstance } from '../core/EditorStore.js';
export interface BlockRendererProps {
    block: Block;
    document: Document;
    store: EditorStoreInstance;
    depth: number;
    path: BlockId[];
}
/**
 * Renders a single block with its children.
 */
export declare function BlockRenderer({ block, document, store, depth, path, }: BlockRendererProps): React.ReactElement;
export default BlockRenderer;
