/**
 * Graph traversal operations for UCM documents.
 *
 * This module provides utilities for navigating the document's block structure,
 * including BFS, DFS, and path-finding operations.
 */
import type { BlockId, Document } from './index.js';
/** Direction for navigation operations */
export type NavigateDirection = 'down' | 'up' | 'both' | 'siblings' | 'breadth_first' | 'depth_first';
/** Output format for traversal results */
export type TraversalOutput = 'structure_only' | 'structure_and_blocks' | 'structure_with_previews';
/** Filter criteria for traversal */
export interface TraversalFilter {
    includeRoles?: string[];
    excludeRoles?: string[];
    includeTags?: string[];
    excludeTags?: string[];
    contentPattern?: string;
}
/** A node in the traversal result */
export interface TraversalNode {
    id: BlockId;
    depth: number;
    parentId?: BlockId;
    contentPreview?: string;
    semanticRole?: string;
    childCount: number;
    edgeCount: number;
}
/** Summary statistics for a traversal */
export interface TraversalSummary {
    totalNodes: number;
    totalEdges: number;
    maxDepth: number;
    nodesByRole: Record<string, number>;
    truncated: boolean;
    truncationReason?: string;
}
/** Complete traversal result */
export interface TraversalResult {
    nodes: TraversalNode[];
    paths: BlockId[][];
    summary: TraversalSummary;
}
/** Configuration for the traversal engine */
export interface TraversalConfig {
    maxDepth: number;
    maxNodes: number;
    defaultPreviewLength: number;
}
/**
 * Graph traversal engine for UCM documents.
 */
export declare class TraversalEngine {
    private config;
    constructor(config?: Partial<TraversalConfig>);
    /**
     * Navigate from a starting point in a specific direction.
     */
    navigate(doc: Document, startId?: BlockId, direction?: NavigateDirection, depth?: number, filter?: TraversalFilter, output?: TraversalOutput): TraversalResult;
    /**
     * Expand a node to get its immediate children.
     */
    expand(doc: Document, nodeId: BlockId, output?: TraversalOutput): TraversalResult;
    /**
     * Get the path from a node to the root.
     */
    pathToRoot(doc: Document, nodeId: BlockId): BlockId[];
    /**
     * Find all paths between two nodes.
     */
    findPaths(doc: Document, fromId: BlockId, toId: BlockId, maxPaths?: number): BlockId[][];
    private findPathsRecursive;
    private traverseBfs;
    private traverseDfs;
    private dfsRecursive;
    private traverseUp;
    private traverseSiblings;
    private traverseBoth;
    private matchesFilter;
    private createNode;
    private getParent;
    private getChildren;
    private emptyResult;
}
/**
 * Convenience function for document traversal.
 */
export declare function traverse(doc: Document, startId?: BlockId, direction?: NavigateDirection, depth?: number): TraversalResult;
/**
 * Get the path from a node to the root.
 */
export declare function pathToRoot(doc: Document, nodeId: BlockId): BlockId[];
/**
 * Expand a node to get its immediate children.
 */
export declare function expand(doc: Document, nodeId: BlockId): TraversalResult;
//# sourceMappingURL=traversal.d.ts.map