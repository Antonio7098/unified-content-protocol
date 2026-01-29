/**
 * GraphView - Visual graph representation of the document structure.
 *
 * Displays blocks as nodes and relationships as edges in an interactive
 * DAG (Directed Acyclic Graph) visualization.
 */
import React from 'react';
import type { EditorStoreInstance } from '../core/EditorStore.js';
export interface GraphViewProps {
    store: EditorStoreInstance;
}
/**
 * Interactive graph visualization of the document structure.
 */
export declare function GraphView({ store }: GraphViewProps): React.ReactElement;
export default GraphView;
