/**
 * Edge Index - Bidirectional edge index for efficient traversal.
 *
 * This module provides O(1) edge lookups in both directions.
 */
import type { BlockId, EdgeType, Edge } from './index.js';
/**
 * Bidirectional edge index for efficient traversal.
 *
 * Maintains both outgoing and incoming edge mappings for fast lookups.
 */
export declare class EdgeIndex {
    private outgoing;
    private incoming;
    /**
     * Add an edge to the index.
     */
    addEdge(source: BlockId, edge: Edge): void;
    /**
     * Remove an edge from the index.
     */
    removeEdge(source: BlockId, target: BlockId, edgeType: EdgeType): void;
    /**
     * Remove all edges involving a block.
     */
    removeBlock(blockId: BlockId): void;
    /**
     * Get all outgoing edges from a block.
     */
    outgoingFrom(source: BlockId): Array<{
        edgeType: EdgeType;
        target: BlockId;
    }>;
    /**
     * Get all incoming edges to a block.
     */
    incomingTo(target: BlockId): Array<{
        edgeType: EdgeType;
        source: BlockId;
    }>;
    /**
     * Get all targets of edges of a specific type from source.
     */
    outgoingOfType(source: BlockId, edgeType: EdgeType): BlockId[];
    /**
     * Get all sources of edges of a specific type to target.
     */
    incomingOfType(target: BlockId, edgeType: EdgeType): BlockId[];
    /**
     * Check if an edge exists.
     */
    hasEdge(source: BlockId, target: BlockId, edgeType: EdgeType): boolean;
    /**
     * Get total edge count.
     */
    edgeCount(): number;
    /**
     * Clear all edges.
     */
    clear(): void;
    /**
     * Get all blocks that have outgoing edges.
     */
    sources(): Set<BlockId>;
    /**
     * Get all blocks that have incoming edges.
     */
    targets(): Set<BlockId>;
    private getInverseEdgeType;
}
//# sourceMappingURL=edge_index.d.ts.map