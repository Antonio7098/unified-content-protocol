/**
 * DiffViewer - Displays differences between document snapshots.
 *
 * Shows block-level and content-level diffs with accept/reject actions.
 */
import React from 'react';
import type { EditorStoreInstance } from '../core/EditorStore.js';
export interface DiffViewerProps {
    store: EditorStoreInstance;
}
/**
 * Component for viewing and managing document diffs.
 */
export declare function DiffViewer({ store }: DiffViewerProps): React.ReactElement;
export default DiffViewer;
