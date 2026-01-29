/**
 * Context management infrastructure for UCM documents.
 *
 * This module provides APIs for intelligent context window management,
 * allowing external orchestration layers to load documents, traverse
 * the knowledge graph, and curate context windows.
 */
const DEFAULT_CONSTRAINTS = {
    maxTokens: 4000,
    maxBlocks: 100,
    maxDepth: 10,
    minRelevance: 0.0,
    requiredRoles: [],
    excludedTags: [],
    preserveStructure: true,
    allowCompression: true,
};
/**
 * Context window with intelligent management.
 */
export class ContextWindow {
    id;
    blocks = new Map();
    constraints;
    metadata;
    constructor(id, constraints) {
        this.id = id;
        this.constraints = { ...DEFAULT_CONSTRAINTS, ...constraints };
        this.metadata = {
            createdAt: Date.now(),
            lastModified: Date.now(),
        };
    }
    get blockCount() {
        return this.blocks.size;
    }
    get totalTokens() {
        let total = 0;
        for (const block of this.blocks.values()) {
            total += block.tokenEstimate;
        }
        return total;
    }
    hasCapacity() {
        return (this.blockCount < this.constraints.maxBlocks &&
            this.totalTokens < this.constraints.maxTokens);
    }
    contains(blockId) {
        return this.blocks.has(blockId);
    }
    get(blockId) {
        return this.blocks.get(blockId);
    }
    blockIds() {
        return Array.from(this.blocks.keys());
    }
}
/**
 * Context Management Infrastructure.
 *
 * Provides APIs for external orchestration layers to manage context windows.
 *
 * @example
 * ```typescript
 * const manager = new ContextManager('my-context')
 * manager.initializeFocus(doc, focusBlockId, 'Summarize this section')
 * manager.expandContext(doc, 'down', 2)
 * const prompt = manager.renderForPrompt(doc)
 * ```
 */
export class ContextManager {
    window;
    expansionPolicy;
    pruningPolicy;
    constructor(id, constraints, expansionPolicy = 'balanced', pruningPolicy = 'relevance_first') {
        this.window = new ContextWindow(id, constraints);
        this.expansionPolicy = expansionPolicy;
        this.pruningPolicy = pruningPolicy;
    }
    /**
     * Initialize context with a focus block.
     */
    initializeFocus(doc, focusId, taskDescription) {
        this.window.metadata.focusArea = focusId;
        this.window.metadata.taskDescription = taskDescription;
        this.window.metadata.lastModified = Date.now();
        const result = {
            blocksAdded: [],
            blocksRemoved: [],
            blocksCompressed: [],
            totalTokens: 0,
            totalBlocks: 0,
            warnings: [],
        };
        // Add focus block
        if (doc.blocks.has(focusId)) {
            this.addBlockInternal(doc, focusId, 'direct_reference', 1.0);
            result.blocksAdded.push(focusId);
        }
        // Add structural context (ancestors)
        let current = focusId;
        let depth = 0;
        while (depth < 3) {
            const parent = this.getParent(doc, current);
            if (!parent || parent === doc.root)
                break;
            this.addBlockInternal(doc, parent, 'structural_context', 0.8 - depth * 0.1);
            result.blocksAdded.push(parent);
            current = parent;
            depth++;
        }
        result.totalTokens = this.window.totalTokens;
        result.totalBlocks = this.window.blockCount;
        return result;
    }
    /**
     * Navigate to a new focus area.
     */
    navigateTo(doc, targetId, taskDescription) {
        this.window.metadata.focusArea = targetId;
        this.window.metadata.taskDescription = taskDescription;
        this.window.metadata.lastModified = Date.now();
        const result = {
            blocksAdded: [],
            blocksRemoved: [],
            blocksCompressed: [],
            totalTokens: 0,
            totalBlocks: 0,
            warnings: [],
        };
        if (doc.blocks.has(targetId)) {
            this.addBlockInternal(doc, targetId, 'navigation_path', 1.0);
            result.blocksAdded.push(targetId);
        }
        result.blocksRemoved = this.pruneIfNeeded();
        result.totalTokens = this.window.totalTokens;
        result.totalBlocks = this.window.blockCount;
        return result;
    }
    /**
     * Add a block to the context.
     */
    addBlock(doc, blockId, reason = 'external_decision') {
        const result = {
            blocksAdded: [],
            blocksRemoved: [],
            blocksCompressed: [],
            totalTokens: 0,
            totalBlocks: 0,
            warnings: [],
        };
        if (doc.blocks.has(blockId)) {
            this.addBlockInternal(doc, blockId, reason, 0.7);
            result.blocksAdded.push(blockId);
        }
        result.blocksRemoved = this.pruneIfNeeded();
        result.totalTokens = this.window.totalTokens;
        result.totalBlocks = this.window.blockCount;
        return result;
    }
    /**
     * Remove a block from the context.
     */
    removeBlock(blockId) {
        const result = {
            blocksAdded: [],
            blocksRemoved: [],
            blocksCompressed: [],
            totalTokens: 0,
            totalBlocks: 0,
            warnings: [],
        };
        if (this.window.blocks.delete(blockId)) {
            result.blocksRemoved.push(blockId);
        }
        this.window.metadata.lastModified = Date.now();
        result.totalTokens = this.window.totalTokens;
        result.totalBlocks = this.window.blockCount;
        return result;
    }
    /**
     * Expand context in a direction.
     */
    expandContext(doc, direction, depth = 2) {
        const result = {
            blocksAdded: [],
            blocksRemoved: [],
            blocksCompressed: [],
            totalTokens: 0,
            totalBlocks: 0,
            warnings: [],
        };
        const focusId = this.window.metadata.focusArea;
        if (!focusId)
            return result;
        switch (direction) {
            case 'down':
                result.blocksAdded = this.expandDownward(doc, focusId, depth);
                break;
            case 'up':
                result.blocksAdded = this.expandUpward(doc, focusId, depth);
                break;
            case 'both':
                result.blocksAdded = [
                    ...this.expandDownward(doc, focusId, depth),
                    ...this.expandUpward(doc, focusId, depth),
                ];
                break;
            case 'semantic':
                result.blocksAdded = this.expandSemantic(doc, focusId, depth);
                break;
        }
        result.blocksRemoved = this.pruneIfNeeded();
        this.window.metadata.lastModified = Date.now();
        result.totalTokens = this.window.totalTokens;
        result.totalBlocks = this.window.blockCount;
        return result;
    }
    /**
     * Compress blocks to fit within constraints.
     */
    compress(doc, method = 'truncate') {
        const result = {
            blocksAdded: [],
            blocksRemoved: [],
            blocksCompressed: [],
            totalTokens: 0,
            totalBlocks: 0,
            warnings: [],
        };
        if (!this.window.constraints.allowCompression) {
            result.warnings.push('Compression not allowed by constraints');
            return result;
        }
        // Find blocks to compress (lowest relevance first)
        const blocksToCompress = Array.from(this.window.blocks.entries())
            .filter(([_, cb]) => !cb.compressed)
            .sort((a, b) => a[1].relevanceScore - b[1].relevanceScore)
            .slice(0, 10);
        for (const [blockId, contextBlock] of blocksToCompress) {
            const block = doc.blocks.get(blockId);
            if (block) {
                contextBlock.originalContent = typeof block.content === 'string' ? block.content : '';
                switch (method) {
                    case 'truncate':
                        contextBlock.tokenEstimate = Math.floor(contextBlock.tokenEstimate / 2);
                        break;
                    case 'structure_only':
                        contextBlock.tokenEstimate = 10;
                        break;
                    case 'summarize':
                        contextBlock.tokenEstimate = Math.floor(contextBlock.tokenEstimate / 3);
                        break;
                }
                contextBlock.compressed = true;
                result.blocksCompressed.push(blockId);
            }
            if (this.window.totalTokens <= this.window.constraints.maxTokens) {
                break;
            }
        }
        result.totalTokens = this.window.totalTokens;
        result.totalBlocks = this.window.blockCount;
        return result;
    }
    /**
     * Get statistics about the context.
     */
    getStatistics() {
        const blocksByReason = {};
        let totalRelevance = 0;
        let compressedCount = 0;
        for (const cb of this.window.blocks.values()) {
            blocksByReason[cb.inclusionReason] = (blocksByReason[cb.inclusionReason] || 0) + 1;
            totalRelevance += cb.relevanceScore;
            if (cb.compressed)
                compressedCount++;
        }
        return {
            totalTokens: this.window.totalTokens,
            totalBlocks: this.window.blockCount,
            blocksByReason,
            averageRelevance: this.window.blockCount > 0 ? totalRelevance / this.window.blockCount : 0,
            compressedCount,
        };
    }
    /**
     * Render context to a format suitable for LLM prompts.
     */
    renderForPrompt(doc) {
        const lines = [];
        // Sort by relevance
        const sortedBlocks = Array.from(this.window.blocks.entries())
            .sort((a, b) => b[1].relevanceScore - a[1].relevanceScore);
        for (const [blockId, contextBlock] of sortedBlocks) {
            const block = doc.blocks.get(blockId);
            if (block) {
                let content;
                if (contextBlock.compressed && contextBlock.originalContent) {
                    content = `[compressed] ${contextBlock.originalContent.substring(0, 50)}...`;
                }
                else {
                    content = typeof block.content === 'string' ? block.content : '';
                }
                const role = block.metadata?.semanticRole || 'block';
                lines.push(`[${blockId}] ${role}: ${content}`);
            }
        }
        return lines.join('\n');
    }
    // Internal methods
    addBlockInternal(doc, blockId, reason, relevance) {
        const existing = this.window.blocks.get(blockId);
        if (existing) {
            existing.accessCount++;
            existing.lastAccessed = Date.now();
            return;
        }
        const block = doc.blocks.get(blockId);
        if (block) {
            const content = typeof block.content === 'string' ? block.content : '';
            const tokenEstimate = Math.max(1, Math.floor(content.length / 4));
            this.window.blocks.set(blockId, {
                blockId,
                inclusionReason: reason,
                relevanceScore: relevance,
                tokenEstimate,
                accessCount: 1,
                lastAccessed: Date.now(),
                compressed: false,
            });
        }
    }
    expandDownward(doc, start, maxDepth) {
        const added = [];
        const queue = [{ id: start, depth: 0 }];
        while (queue.length > 0 && this.window.hasCapacity()) {
            const { id, depth } = queue.shift();
            if (depth > maxDepth)
                break;
            const children = this.getChildren(doc, id);
            for (const child of children) {
                if (!this.window.contains(child)) {
                    const relevance = Math.max(0.1, 0.6 - depth * 0.1);
                    this.addBlockInternal(doc, child, 'structural_context', relevance);
                    added.push(child);
                    queue.push({ id: child, depth: depth + 1 });
                }
            }
        }
        return added;
    }
    expandUpward(doc, start, maxDepth) {
        const added = [];
        let current = start;
        let depth = 0;
        while (depth < maxDepth && this.window.hasCapacity()) {
            const parent = this.getParent(doc, current);
            if (!parent || parent === doc.root)
                break;
            if (!this.window.contains(parent)) {
                const relevance = Math.max(0.1, 0.7 - depth * 0.1);
                this.addBlockInternal(doc, parent, 'structural_context', relevance);
                added.push(parent);
            }
            current = parent;
            depth++;
        }
        return added;
    }
    expandSemantic(doc, start, maxDepth) {
        const added = [];
        const visited = new Set();
        const queue = [{ id: start, depth: 0 }];
        while (queue.length > 0 && this.window.hasCapacity()) {
            const { id, depth } = queue.shift();
            if (depth > maxDepth || visited.has(id))
                continue;
            visited.add(id);
            const block = doc.blocks.get(id);
            if (block?.edges) {
                for (const edge of block.edges) {
                    const target = edge.target;
                    if (!this.window.contains(target) && !visited.has(target)) {
                        const relevance = Math.max(0.1, 0.5 - depth * 0.1);
                        this.addBlockInternal(doc, target, 'semantic_relevance', relevance);
                        added.push(target);
                        queue.push({ id: target, depth: depth + 1 });
                    }
                }
            }
        }
        return added;
    }
    pruneIfNeeded() {
        const removed = [];
        while (this.window.blockCount > this.window.constraints.maxBlocks ||
            this.window.totalTokens > this.window.constraints.maxTokens) {
            const toRemove = this.findBlockToRemove();
            if (toRemove) {
                this.window.blocks.delete(toRemove);
                removed.push(toRemove);
            }
            else {
                break;
            }
        }
        return removed;
    }
    findBlockToRemove() {
        const focus = this.window.metadata.focusArea;
        const candidates = Array.from(this.window.blocks.entries())
            .filter(([id]) => id !== focus);
        if (candidates.length === 0)
            return undefined;
        if (this.pruningPolicy === 'relevance_first') {
            return candidates.sort((a, b) => a[1].relevanceScore - b[1].relevanceScore)[0]?.[0];
        }
        else if (this.pruningPolicy === 'recency_first') {
            return candidates.sort((a, b) => a[1].lastAccessed - b[1].lastAccessed)[0]?.[0];
        }
        return candidates[0]?.[0];
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
}
// Convenience function
/**
 * Create a new context manager with specified constraints.
 */
export function createContext(id, maxTokens = 4000, maxBlocks = 100) {
    return new ContextManager(id, { maxTokens, maxBlocks });
}
//# sourceMappingURL=context.js.map