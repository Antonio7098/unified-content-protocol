/**
 * Context management infrastructure for UCM documents.
 *
 * This module provides APIs for intelligent context window management,
 * allowing external orchestration layers to load documents, traverse
 * the knowledge graph, and curate context windows.
 */
import type { BlockId, Document } from './index.js';
/** Reason why a block was included in context */
export type InclusionReason = 'direct_reference' | 'navigation_path' | 'structural_context' | 'semantic_relevance' | 'external_decision' | 'required_context';
/** Direction for context expansion */
export type ExpandDirection = 'up' | 'down' | 'both' | 'semantic';
/** Policy for context expansion */
export type ExpansionPolicy = 'conservative' | 'balanced' | 'aggressive';
/** Policy for context pruning */
export type PruningPolicy = 'relevance_first' | 'recency_first' | 'redundancy_first';
/** Method for content compression */
export type CompressionMethod = 'truncate' | 'summarize' | 'structure_only';
/** A block in the context window with metadata */
export interface ContextBlock {
    blockId: BlockId;
    inclusionReason: InclusionReason;
    relevanceScore: number;
    tokenEstimate: number;
    accessCount: number;
    lastAccessed: number;
    compressed: boolean;
    originalContent?: string;
}
/** Constraints for the context window */
export interface ContextConstraints {
    maxTokens: number;
    maxBlocks: number;
    maxDepth: number;
    minRelevance: number;
    requiredRoles: string[];
    excludedTags: string[];
    preserveStructure: boolean;
    allowCompression: boolean;
}
/** Result of a context operation */
export interface ContextUpdateResult {
    blocksAdded: BlockId[];
    blocksRemoved: BlockId[];
    blocksCompressed: BlockId[];
    totalTokens: number;
    totalBlocks: number;
    warnings: string[];
}
/** Statistics about the context window */
export interface ContextStatistics {
    totalTokens: number;
    totalBlocks: number;
    blocksByReason: Record<string, number>;
    averageRelevance: number;
    compressedCount: number;
}
/** Context window metadata */
export interface ContextMetadata {
    focusArea?: BlockId;
    taskDescription?: string;
    createdAt: number;
    lastModified: number;
}
/**
 * Context window with intelligent management.
 */
export declare class ContextWindow {
    readonly id: string;
    readonly blocks: Map<BlockId, ContextBlock>;
    readonly constraints: ContextConstraints;
    metadata: ContextMetadata;
    constructor(id: string, constraints?: Partial<ContextConstraints>);
    get blockCount(): number;
    get totalTokens(): number;
    hasCapacity(): boolean;
    contains(blockId: BlockId): boolean;
    get(blockId: BlockId): ContextBlock | undefined;
    blockIds(): BlockId[];
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
export declare class ContextManager {
    readonly window: ContextWindow;
    private expansionPolicy;
    private pruningPolicy;
    constructor(id: string, constraints?: Partial<ContextConstraints>, expansionPolicy?: ExpansionPolicy, pruningPolicy?: PruningPolicy);
    /**
     * Initialize context with a focus block.
     */
    initializeFocus(doc: Document, focusId: BlockId, taskDescription: string): ContextUpdateResult;
    /**
     * Navigate to a new focus area.
     */
    navigateTo(doc: Document, targetId: BlockId, taskDescription: string): ContextUpdateResult;
    /**
     * Add a block to the context.
     */
    addBlock(doc: Document, blockId: BlockId, reason?: InclusionReason): ContextUpdateResult;
    /**
     * Remove a block from the context.
     */
    removeBlock(blockId: BlockId): ContextUpdateResult;
    /**
     * Expand context in a direction.
     */
    expandContext(doc: Document, direction: ExpandDirection, depth?: number): ContextUpdateResult;
    /**
     * Compress blocks to fit within constraints.
     */
    compress(doc: Document, method?: CompressionMethod): ContextUpdateResult;
    /**
     * Get statistics about the context.
     */
    getStatistics(): ContextStatistics;
    /**
     * Render context to a format suitable for LLM prompts.
     */
    renderForPrompt(doc: Document): string;
    private addBlockInternal;
    private expandDownward;
    private expandUpward;
    private expandSemantic;
    private pruneIfNeeded;
    private findBlockToRemove;
    private getParent;
    private getChildren;
}
/**
 * Create a new context manager with specified constraints.
 */
export declare function createContext(id: string, maxTokens?: number, maxBlocks?: number): ContextManager;
//# sourceMappingURL=context.d.ts.map