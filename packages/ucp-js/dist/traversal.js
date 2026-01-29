/**
 * Graph traversal operations for UCM documents.
 *
 * This module provides utilities for navigating the document's block structure,
 * including BFS, DFS, and path-finding operations.
 */
const DEFAULT_CONFIG = {
    maxDepth: 100,
    maxNodes: 10000,
    defaultPreviewLength: 100,
};
/**
 * Graph traversal engine for UCM documents.
 */
export class TraversalEngine {
    config;
    constructor(config) {
        this.config = { ...DEFAULT_CONFIG, ...config };
    }
    /**
     * Navigate from a starting point in a specific direction.
     */
    navigate(doc, startId, direction = 'breadth_first', depth, filter, output = 'structure_and_blocks') {
        const start = startId ?? doc.root;
        if (!this.blockExists(doc, start)) {
            throw new Error(`Block not found: ${start}`);
        }
        const maxDepth = Math.min(depth ?? this.config.maxDepth, this.config.maxDepth);
        const filterObj = filter ?? {};
        switch (direction) {
            case 'breadth_first':
            case 'down':
                return this.traverseBfs(doc, start, maxDepth, filterObj, output);
            case 'depth_first':
                return this.traverseDfs(doc, start, maxDepth, filterObj, output);
            case 'up':
                return this.traverseUp(doc, start, maxDepth, filterObj, output);
            case 'siblings':
                return this.traverseSiblings(doc, start, filterObj, output);
            case 'both':
                return this.traverseBoth(doc, start, maxDepth, filterObj, output);
            default:
                return this.emptyResult();
        }
    }
    /**
     * Expand a node to get its immediate children.
     */
    expand(doc, nodeId, output = 'structure_and_blocks') {
        return this.navigate(doc, nodeId, 'down', 1, undefined, output);
    }
    /**
     * Get the path from a node to the root.
     */
    pathToRoot(doc, nodeId) {
        const path = [nodeId];
        let current = nodeId;
        while (true) {
            const parent = this.getParent(doc, current);
            if (!parent)
                break;
            path.push(parent);
            if (parent === doc.root)
                break;
            current = parent;
        }
        return path;
    }
    /**
     * Find all paths between two nodes.
     */
    findPaths(doc, fromId, toId, maxPaths = 10) {
        if (!this.blockExists(doc, fromId)) {
            throw new Error(`Block not found: ${fromId}`);
        }
        if (!this.blockExists(doc, toId)) {
            throw new Error(`Block not found: ${toId}`);
        }
        const paths = [];
        const visited = new Set();
        const currentPath = [fromId];
        const parentIndex = this.buildParentIndex(doc);
        this.findPathsRecursive(doc, fromId, toId, parentIndex, visited, currentPath, paths, maxPaths);
        return paths;
    }
    findPathsRecursive(doc, current, target, parentIndex, visited, currentPath, paths, maxPaths) {
        if (paths.length >= maxPaths)
            return;
        if (current === target) {
            paths.push([...currentPath]);
            return;
        }
        visited.add(current);
        const children = this.getChildren(doc, current);
        for (const child of children) {
            if (!visited.has(child)) {
                currentPath.push(child);
                this.findPathsRecursive(doc, child, target, parentIndex, visited, currentPath, paths, maxPaths);
                currentPath.pop();
            }
        }
        const parent = parentIndex.get(current);
        if (parent && !visited.has(parent)) {
            currentPath.push(parent);
            this.findPathsRecursive(doc, parent, target, parentIndex, visited, currentPath, paths, maxPaths);
            currentPath.pop();
        }
        const block = doc.blocks?.get?.(current);
        if (block?.edges?.length) {
            for (const edge of block.edges) {
                if (!visited.has(edge.target)) {
                    currentPath.push(edge.target);
                    this.findPathsRecursive(doc, edge.target, target, parentIndex, visited, currentPath, paths, maxPaths);
                    currentPath.pop();
                }
            }
        }
        visited.delete(current);
    }
    traverseBfs(doc, start, maxDepth, filter, output) {
        const nodes = [];
        const visited = new Set();
        const queue = [
            { id: start, parentId: undefined, depth: 0 },
        ];
        const nodesByRole = {};
        while (queue.length > 0 && nodes.length < this.config.maxNodes) {
            const { id, parentId, depth } = queue.shift();
            if (depth > maxDepth || visited.has(id))
                continue;
            visited.add(id);
            const block = doc.blocks.get(id);
            const children = this.getChildren(doc, id);
            for (const child of children) {
                if (!visited.has(child)) {
                    queue.push({ id: child, parentId: id, depth: depth + 1 });
                }
            }
            if (block && this.matchesFilter(block, filter)) {
                const node = this.createNode(doc, id, depth, parentId, output);
                nodes.push(node);
                if (node.semanticRole) {
                    nodesByRole[node.semanticRole] = (nodesByRole[node.semanticRole] || 0) + 1;
                }
            }
        }
        const maxDepthFound = Math.max(...nodes.map(n => n.depth), 0);
        return {
            nodes,
            paths: [],
            summary: {
                totalNodes: nodes.length,
                totalEdges: 0,
                maxDepth: maxDepthFound,
                nodesByRole,
                truncated: nodes.length >= this.config.maxNodes,
                truncationReason: nodes.length >= this.config.maxNodes ? `Max nodes (${this.config.maxNodes}) reached` : undefined,
            },
        };
    }
    traverseDfs(doc, start, maxDepth, filter, output) {
        const nodes = [];
        const visited = new Set();
        const nodesByRole = {};
        this.dfsRecursive(doc, start, undefined, 0, maxDepth, filter, output, visited, nodes, nodesByRole);
        const maxDepthFound = Math.max(...nodes.map(n => n.depth), 0);
        return {
            nodes,
            paths: [],
            summary: {
                totalNodes: nodes.length,
                totalEdges: 0,
                maxDepth: maxDepthFound,
                nodesByRole,
                truncated: nodes.length >= this.config.maxNodes,
            },
        };
    }
    dfsRecursive(doc, nodeId, parentId, depth, maxDepth, filter, output, visited, nodes, nodesByRole) {
        if (depth > maxDepth || visited.has(nodeId) || nodes.length >= this.config.maxNodes)
            return;
        visited.add(nodeId);
        const block = doc.blocks.get(nodeId);
        const children = this.getChildren(doc, nodeId);
        for (const child of children) {
            this.dfsRecursive(doc, child, nodeId, depth + 1, maxDepth, filter, output, visited, nodes, nodesByRole);
        }
        if (block && this.matchesFilter(block, filter)) {
            const node = this.createNode(doc, nodeId, depth, parentId, output);
            nodes.push(node);
            if (node.semanticRole) {
                nodesByRole[node.semanticRole] = (nodesByRole[node.semanticRole] || 0) + 1;
            }
        }
    }
    traverseUp(doc, start, maxDepth, filter, output) {
        const nodes = [];
        let current = start;
        let depth = 0;
        while (depth <= maxDepth) {
            const block = doc.blocks.get(current);
            if (block && this.matchesFilter(block, filter)) {
                nodes.push(this.createNode(doc, current, depth, undefined, output));
            }
            const parent = this.getParent(doc, current);
            if (!parent)
                break;
            current = parent;
            depth++;
        }
        return {
            nodes,
            paths: [],
            summary: {
                totalNodes: nodes.length,
                totalEdges: 0,
                maxDepth: depth,
                nodesByRole: {},
                truncated: false,
            },
        };
    }
    traverseSiblings(doc, start, filter, output) {
        const nodes = [];
        const parent = this.getParent(doc, start);
        if (parent) {
            const siblings = this.getChildren(doc, parent);
            for (const sibling of siblings) {
                const block = doc.blocks.get(sibling);
                if (block && this.matchesFilter(block, filter)) {
                    nodes.push(this.createNode(doc, sibling, 0, parent, output));
                }
            }
        }
        return {
            nodes,
            paths: [],
            summary: {
                totalNodes: nodes.length,
                totalEdges: 0,
                maxDepth: 0,
                nodesByRole: {},
                truncated: false,
            },
        };
    }
    traverseBoth(doc, start, maxDepth, filter, output) {
        const upResult = this.traverseUp(doc, start, maxDepth, filter, output);
        const downResult = this.traverseBfs(doc, start, maxDepth, filter, output);
        const seen = new Set();
        const nodes = [];
        for (const node of [...upResult.nodes, ...downResult.nodes]) {
            if (!seen.has(node.id)) {
                seen.add(node.id);
                nodes.push(node);
            }
        }
        const maxDepthFound = Math.max(...nodes.map(n => n.depth), 0);
        return {
            nodes,
            paths: [],
            summary: {
                totalNodes: nodes.length,
                totalEdges: 0,
                maxDepth: maxDepthFound,
                nodesByRole: {},
                truncated: false,
            },
        };
    }
    matchesFilter(block, filter) {
        const role = block.metadata?.semanticRole || '';
        const tags = [...(block.tags || []), ...(block.metadata?.tags || [])];
        if (filter.includeRoles?.length && !filter.includeRoles.includes(role)) {
            return false;
        }
        if (filter.excludeRoles?.length && filter.excludeRoles.includes(role)) {
            return false;
        }
        if (filter.includeTags?.length && !filter.includeTags.some(t => tags.includes(t))) {
            return false;
        }
        if (filter.excludeTags?.length && filter.excludeTags.some(t => tags.includes(t))) {
            return false;
        }
        if (filter.contentPattern) {
            const content = typeof block.content === 'string' ? block.content : '';
            if (!content.toLowerCase().includes(filter.contentPattern.toLowerCase())) {
                return false;
            }
        }
        return true;
    }
    createNode(doc, blockId, depth, parentId, output) {
        const block = doc.blocks.get(blockId);
        const children = this.getChildren(doc, blockId);
        let contentPreview;
        if (output !== 'structure_only' && block) {
            const content = typeof block.content === 'string' ? block.content : '';
            contentPreview = content.length > this.config.defaultPreviewLength
                ? content.substring(0, this.config.defaultPreviewLength) + '...'
                : content;
        }
        return {
            id: blockId,
            depth,
            parentId,
            contentPreview,
            semanticRole: block?.metadata?.semanticRole,
            childCount: children.length,
            edgeCount: block?.edges?.length || 0,
        };
    }
    buildParentIndex(doc) {
        const parents = new Map();
        if (!doc?.blocks)
            return parents;
        for (const [parentId, block] of doc.blocks) {
            if (!block?.children)
                continue;
            for (const childId of block.children) {
                parents.set(childId, parentId);
            }
        }
        return parents;
    }
    blockExists(doc, blockId) {
        if (!doc?.blocks)
            return false;
        if (typeof doc.blocks.has === 'function') {
            return doc.blocks.has(blockId);
        }
        if (typeof doc.blocks.get === 'function') {
            return doc.blocks.get(blockId) !== undefined;
        }
        return Object.prototype.hasOwnProperty.call(doc.blocks, blockId);
    }
    getParent(doc, blockId) {
        for (const [id, block] of doc.blocks) {
            if (block.children?.includes(blockId)) {
                return id;
            }
        }
        return undefined;
    }
    getChildren(doc, blockId) {
        return doc.blocks.get(blockId)?.children || [];
    }
    emptyResult() {
        return {
            nodes: [],
            paths: [],
            summary: {
                totalNodes: 0,
                totalEdges: 0,
                maxDepth: 0,
                nodesByRole: {},
                truncated: false,
            },
        };
    }
}
// Convenience functions
/**
 * Convenience function for document traversal.
 */
export function traverse(doc, startId, direction = 'breadth_first', depth) {
    return new TraversalEngine().navigate(doc, startId, direction, depth);
}
/**
 * Get the path from a node to the root.
 */
export function pathToRoot(doc, nodeId) {
    return new TraversalEngine().pathToRoot(doc, nodeId);
}
/**
 * Expand a node to get its immediate children.
 */
export function expand(doc, nodeId) {
    return new TraversalEngine().expand(doc, nodeId);
}
//# sourceMappingURL=traversal.js.map