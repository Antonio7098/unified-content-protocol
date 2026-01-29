/**
 * Edge Index - Bidirectional edge index for efficient traversal.
 *
 * This module provides O(1) edge lookups in both directions.
 */
/**
 * Bidirectional edge index for efficient traversal.
 *
 * Maintains both outgoing and incoming edge mappings for fast lookups.
 */
export class EdgeIndex {
    outgoing = new Map();
    incoming = new Map();
    /**
     * Add an edge to the index.
     */
    addEdge(source, edge) {
        const outgoing = this.outgoing.get(source) ?? [];
        outgoing.push({ edgeType: edge.edgeType, target: edge.target });
        this.outgoing.set(source, outgoing);
        const inverse = this.getInverseEdgeType(edge.edgeType);
        const incoming = this.incoming.get(edge.target) ?? [];
        incoming.push({ edgeType: inverse, source });
        this.incoming.set(edge.target, incoming);
    }
    /**
     * Remove an edge from the index.
     */
    removeEdge(source, target, edgeType) {
        const outgoingEdges = this.outgoing.get(source);
        if (outgoingEdges) {
            this.outgoing.set(source, outgoingEdges.filter(e => !(e.edgeType === edgeType && e.target === target)));
        }
        const inverse = this.getInverseEdgeType(edgeType);
        const incomingEdges = this.incoming.get(target);
        if (incomingEdges) {
            this.incoming.set(target, incomingEdges.filter(e => !(e.edgeType === inverse && e.source === source)));
        }
    }
    /**
     * Remove all edges involving a block.
     */
    removeBlock(blockId) {
        const outgoingEdges = this.outgoing.get(blockId);
        if (outgoingEdges) {
            for (const { target } of outgoingEdges) {
                const incoming = this.incoming.get(target);
                if (incoming) {
                    this.incoming.set(target, incoming.filter(e => e.source !== blockId));
                }
            }
            this.outgoing.delete(blockId);
        }
        const incomingEdges = this.incoming.get(blockId);
        if (incomingEdges) {
            for (const { source } of incomingEdges) {
                const outgoing = this.outgoing.get(source);
                if (outgoing) {
                    this.outgoing.set(source, outgoing.filter(e => e.target !== blockId));
                }
            }
            this.incoming.delete(blockId);
        }
    }
    /**
     * Get all outgoing edges from a block.
     */
    outgoingFrom(source) {
        return [...(this.outgoing.get(source) ?? [])];
    }
    /**
     * Get all incoming edges to a block.
     */
    incomingTo(target) {
        return [...(this.incoming.get(target) ?? [])];
    }
    /**
     * Get all targets of edges of a specific type from source.
     */
    outgoingOfType(source, edgeType) {
        return (this.outgoing.get(source) ?? [])
            .filter(e => e.edgeType === edgeType)
            .map(e => e.target);
    }
    /**
     * Get all sources of edges of a specific type to target.
     */
    incomingOfType(target, edgeType) {
        return (this.incoming.get(target) ?? [])
            .filter(e => e.edgeType === edgeType)
            .map(e => e.source);
    }
    /**
     * Check if an edge exists.
     */
    hasEdge(source, target, edgeType) {
        return (this.outgoing.get(source) ?? [])
            .some(e => e.edgeType === edgeType && e.target === target);
    }
    /**
     * Get total edge count.
     */
    edgeCount() {
        let count = 0;
        for (const edges of this.outgoing.values()) {
            count += edges.length;
        }
        return count;
    }
    /**
     * Clear all edges.
     */
    clear() {
        this.outgoing.clear();
        this.incoming.clear();
    }
    /**
     * Get all blocks that have outgoing edges.
     */
    sources() {
        return new Set(this.outgoing.keys());
    }
    /**
     * Get all blocks that have incoming edges.
     */
    targets() {
        return new Set(this.incoming.keys());
    }
    getInverseEdgeType(edgeType) {
        const inverses = {
            'references': 'cited_by',
            'cited_by': 'references',
            'parent_of': 'child_of',
            'child_of': 'parent_of',
            'previous_sibling': 'next_sibling',
            'next_sibling': 'previous_sibling',
        };
        return inverses[edgeType] ?? edgeType;
    }
}
//# sourceMappingURL=edge_index.js.map