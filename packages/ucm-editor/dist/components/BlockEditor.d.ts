/**
 * BlockEditor - Inline editing component for blocks.
 *
 * Provides a Notion-like editing experience with auto-growing textarea
 * and keyboard shortcuts.
 */
import React from 'react';
import type { Block } from 'ucp-content';
import type { EditorStoreInstance } from '../core/EditorStore.js';
export interface BlockEditorProps {
    block: Block;
    store: EditorStoreInstance;
}
/**
 * Inline editor for a single block.
 */
export declare function BlockEditor({ block, store }: BlockEditorProps): React.ReactElement;
export default BlockEditor;
