/**
 * ucp-js - Unified Content Protocol JavaScript SDK
 *
 * A developer-friendly SDK for building LLM-powered content manipulation.
 *
 * @example
 * ```typescript
 * import { ucp } from 'ucp-js'
 *
 * // Parse markdown into a document
 * const doc = ucp.parse('# Hello\n\nWorld')
 *
 * // Get a prompt builder for your LLM
 * const prompt = ucp.prompt()
 *   .edit()
 *   .append()
 *   .withShortIds()
 *   .build()
 *
 * // Map IDs for token efficiency
 * const mapper = ucp.mapIds(doc)
 * const shortPrompt = mapper.shorten(docDescription)
 * const expandedUcl = mapper.expand(llmResponse)
 * ```
 */
/** Block ID - unique identifier for content blocks */
export type BlockId = string;
/** Content types supported by UCM */
export type ContentType = 'text' | 'code' | 'table' | 'math' | 'json' | 'media' | 'binary' | 'composite';
/** Semantic roles for blocks */
export type SemanticRole = 'heading1' | 'heading2' | 'heading3' | 'heading4' | 'heading5' | 'heading6' | 'paragraph' | 'quote' | 'list' | 'code' | 'table' | 'equation' | 'title' | 'subtitle' | 'abstract' | 'section' | 'intro' | 'body' | 'conclusion' | 'note' | 'warning' | 'tip' | 'sidebar' | 'callout' | 'metadata' | 'citation' | 'footnote';
/** Edge types for block relationships */
export type EdgeType = 'derived_from' | 'supersedes' | 'transformed_from' | 'references' | 'cited_by' | 'links_to' | 'supports' | 'contradicts' | 'elaborates' | 'summarizes' | 'parent_of' | 'child_of' | 'sibling_of' | 'previous_sibling' | 'next_sibling' | 'version_of' | 'alternative_of' | 'translation_of';
/** Edge metadata */
export interface EdgeMetadata {
    confidence?: number;
    description?: string;
    custom?: Record<string, unknown>;
}
/** An edge represents a relationship between blocks */
export interface Edge {
    edgeType: EdgeType;
    target: BlockId;
    metadata: EdgeMetadata;
    createdAt: Date;
}
/** Validation severity levels */
export type ValidationSeverity = 'error' | 'warning' | 'info';
/** A validation issue */
export interface ValidationIssue {
    severity: ValidationSeverity;
    code: string;
    message: string;
    blockId?: BlockId;
}
/** Validation result */
export interface ValidationResult {
    valid: boolean;
    issues: ValidationIssue[];
}
/** Resource limits for validation */
export interface ResourceLimits {
    maxDocumentSize: number;
    maxBlockCount: number;
    maxBlockSize: number;
    maxNestingDepth: number;
    maxEdgesPerBlock: number;
}
/** Transaction states */
export type TransactionState = 'active' | 'committed' | 'rolled_back' | 'timed_out';
/** Block metadata */
export interface BlockMetadata {
    semanticRole?: SemanticRole;
    label?: string;
    tags: string[];
    summary?: string;
    createdAt: Date;
    modifiedAt: Date;
    custom: Record<string, unknown>;
}
/** A content block in the document */
export interface Block {
    id: BlockId;
    content: string;
    type: ContentType;
    role?: SemanticRole;
    label?: string;
    tags: string[];
    children: BlockId[];
    edges: Edge[];
    metadata?: BlockMetadata;
}
/** Document metadata */
export interface DocumentMetadata {
    title?: string;
    description?: string;
    authors: string[];
    language?: string;
    createdAt: Date;
    modifiedAt: Date;
    custom: Record<string, unknown>;
}
/** A UCM document */
export interface Document {
    id: string;
    root: BlockId;
    blocks: Map<BlockId, Block>;
    metadata?: DocumentMetadata;
    version: number;
}
/** UCL command capabilities */
export type Capability = 'edit' | 'append' | 'move' | 'delete' | 'link' | 'snapshot' | 'transaction';
/** Create a new empty document */
export declare function createDocument(title?: string): Document;
/** Add a block to a document */
export declare function addBlock(doc: Document, parentId: BlockId, content: string, options?: {
    type?: ContentType;
    role?: SemanticRole;
    label?: string;
}): BlockId;
/** Get a block by ID */
export declare function getBlock(doc: Document, id: BlockId): Block | undefined;
/** Edit a block's textual content */
export declare function editBlock(doc: Document, id: BlockId, content: string): void;
/** Move a block (and its subtree) to a new parent */
export declare function moveBlock(doc: Document, id: BlockId, newParentId: BlockId, index?: number): void;
/** Delete a block (with optional cascade to children) */
export declare function deleteBlock(doc: Document, id: BlockId, options?: {
    cascade?: boolean;
}): void;
/** Add a tag to a block */
export declare function addTag(doc: Document, id: BlockId, tag: string): void;
/** Remove a tag from a block */
export declare function removeTag(doc: Document, id: BlockId, tag: string): void;
/** Check if a block has a tag */
export declare function blockHasTag(doc: Document, id: BlockId, tag: string): boolean;
/** Find block IDs with a tag */
export declare function findBlocksByTag(doc: Document, tag: string): BlockId[];
/** Get children of a block */
export declare function getChildren(doc: Document, parentId: BlockId): BlockId[];
/** Get parent of a block */
export declare function getParent(doc: Document, childId: BlockId): BlockId | undefined;
/** Get all ancestors of a block (from immediate parent to root) */
export declare function getAncestors(doc: Document, blockId: BlockId): BlockId[];
/** Get all descendants of a block (breadth-first) */
export declare function getDescendants(doc: Document, blockId: BlockId): BlockId[];
/** Get siblings of a block (excluding itself) */
export declare function getSiblings(doc: Document, blockId: BlockId): BlockId[];
/** Get depth of a block (root is 0) */
export declare function getDepth(doc: Document, blockId: BlockId): number;
/** Find blocks by content type */
export declare function findByType(doc: Document, contentType: ContentType): BlockId[];
/** Find blocks by semantic role */
export declare function findByRole(doc: Document, role: SemanticRole): BlockId[];
/** Find a block by label */
export declare function findByLabel(doc: Document, label: string): BlockId | undefined;
/** Get block count */
export declare function getBlockCount(doc: Document): number;
/** Create a new edge */
export declare function createEdge(edgeType: EdgeType, target: BlockId, metadata?: EdgeMetadata): Edge;
/** Add an edge to a block */
export declare function addEdge(doc: Document, sourceId: BlockId, edgeType: EdgeType, targetId: BlockId, metadata?: EdgeMetadata): void;
/** Remove an edge from a block */
export declare function removeEdge(doc: Document, sourceId: BlockId, edgeType: EdgeType, targetId: BlockId): boolean;
/** Check if an edge exists */
export declare function hasEdge(doc: Document, sourceId: BlockId, targetId: BlockId, edgeType?: EdgeType): boolean;
/** Get outgoing edges from a block */
export declare function getOutgoingEdges(doc: Document, sourceId: BlockId): Edge[];
/** Get incoming edges to a block */
export declare function getIncomingEdges(doc: Document, targetId: BlockId): Array<{
    source: BlockId;
    edge: Edge;
}>;
/** Default resource limits */
export declare const DEFAULT_LIMITS: ResourceLimits;
/** Validate a document */
export declare function validateDocument(doc: Document, limits?: ResourceLimits): ValidationResult;
/** Find orphaned blocks (unreachable from root) */
export declare function findOrphans(doc: Document): BlockId[];
/** Remove all orphaned blocks */
export declare function pruneOrphans(doc: Document): BlockId[];
/** Parse markdown into a UCM document */
export declare function parseMarkdown(markdown: string): Document;
/** Render a document to markdown */
export declare function renderMarkdown(doc: Document): string;
/** Fluent prompt builder for LLM agents */
export declare class PromptBuilder {
    private capabilities;
    private shortIds;
    private customRules;
    private context?;
    /** Enable EDIT capability */
    edit(): this;
    /** Enable APPEND capability */
    append(): this;
    /** Enable MOVE capability */
    move(): this;
    /** Enable DELETE capability */
    delete(): this;
    /** Enable LINK capability */
    link(): this;
    /** Enable SNAPSHOT capability */
    snapshot(): this;
    /** Enable TRANSACTION capability */
    transaction(): this;
    /** Enable all capabilities */
    all(): this;
    /** Use short numeric IDs (1, 2, 3) instead of full block IDs */
    withShortIds(): this;
    /** Add a custom rule */
    withRule(rule: string): this;
    /** Add context to the prompt */
    withContext(ctx: string): this;
    /** Build the system prompt */
    build(): string;
    /** Build a complete prompt with document and task */
    buildPrompt(documentDescription: string, task: string): string;
}
/** Create a new prompt builder */
export declare function prompt(): PromptBuilder;
/** Maps long block IDs to short numbers for token efficiency */
export declare class IdMapper {
    private toShort;
    private toLong;
    private nextId;
    /** Create a mapper from a document */
    static fromDocument(doc: Document): IdMapper;
    /** Register a block ID */
    register(id: BlockId): number;
    /** Get short ID for a block */
    getShort(id: BlockId): number | undefined;
    /** Get long ID from short */
    getLong(shortId: number): BlockId | undefined;
    /** Shorten all block IDs in text */
    shorten(text: string): string;
    /** Expand short IDs back to long IDs in UCL commands */
    expand(ucl: string): string;
    /** Generate a normalized document description with structure and blocks */
    describe(doc: Document): string;
    /** Get the mapping table (for debugging) */
    getMappings(): Array<{
        short: number;
        long: BlockId;
    }>;
}
/** Create an ID mapper from a document */
export declare function mapIds(doc: Document): IdMapper;
/** Fluent builder for UCL commands */
export declare class UclBuilder {
    private commands;
    /** Add an EDIT command */
    edit(blockId: string | number, content: string): this;
    /** Add an APPEND command */
    append(parentId: string | number, content: string, options?: {
        type?: ContentType;
        label?: string;
    }): this;
    /** Add a MOVE command */
    moveTo(blockId: string | number, newParent: string | number): this;
    /** Add a MOVE BEFORE command */
    moveBefore(blockId: string | number, sibling: string | number): this;
    /** Add a MOVE AFTER command */
    moveAfter(blockId: string | number, sibling: string | number): this;
    /** Add a DELETE command */
    delete(blockId: string | number, cascade?: boolean): this;
    /** Add a LINK command */
    link(source: string | number, edgeType: string, target: string | number): this;
    /** Wrap all commands in ATOMIC block */
    atomic(): this;
    /** Build the final UCL string */
    build(): string;
    /** Get commands as array */
    toArray(): string[];
    private escape;
}
/** Create a new UCL builder */
export declare function ucl(): UclBuilder;
/** Snapshot data */
export interface Snapshot {
    id: string;
    description?: string;
    createdAt: Date;
    documentVersion: number;
    data: string;
}
/** Serialize a document to JSON string */
export declare function serializeDocument(doc: Document): string;
/** Deserialize a document from JSON string */
export declare function deserializeDocument(data: string): Document;
/** Snapshot manager for document versioning */
export declare class SnapshotManager {
    private snapshots;
    private maxSnapshots;
    constructor(maxSnapshots?: number);
    /** Create a snapshot of the document */
    create(name: string, doc: Document, description?: string): string;
    /** Restore a document from a snapshot */
    restore(name: string): Document;
    /** Get a snapshot by name */
    get(name: string): Snapshot | undefined;
    /** Get snapshot info without loading full data */
    getInfo(name: string): {
        id: string;
        description?: string;
        createdAt: Date;
        documentVersion: number;
        blockCount: number;
    } | undefined;
    /** List all snapshots (newest first) */
    list(): Snapshot[];
    /** Delete a snapshot */
    delete(name: string): boolean;
    /** Check if a snapshot exists */
    exists(name: string): boolean;
    /** Get snapshot count */
    count(): number;
    private evictOldest;
}
/** Transaction for atomic operations */
export interface Savepoint {
    name: string;
    operationIndex: number;
    documentState: string;
    createdAt: Date;
}
/** Transaction for atomic operations */
export declare class Transaction {
    readonly id: string;
    readonly name?: string;
    private _state;
    private _startTime;
    private _timeout;
    private _initialState;
    private _doc;
    private _savepoints;
    private _operationCount;
    constructor(doc: Document, timeoutMs?: number, name?: string);
    get state(): TransactionState;
    isActive(): boolean;
    isTimedOut(): boolean;
    elapsedMs(): number;
    operationCount(): number;
    /** Create a savepoint for partial rollback */
    savepoint(name: string): void;
    /** Rollback to a named savepoint */
    rollbackToSavepoint(name: string): void;
    /** Get a savepoint by name */
    getSavepoint(name: string): Savepoint | undefined;
    commit(): void;
    rollback(): void;
}
/** Transaction manager */
export declare class TransactionManager {
    private transactions;
    private defaultTimeout;
    constructor(defaultTimeoutMs?: number);
    begin(doc: Document, name?: string, timeoutMs?: number): Transaction;
    get(id: string): Transaction | undefined;
    commit(id: string): void;
    rollback(id: string): void;
    activeCount(): number;
    cleanup(): number;
}
/** UCL execution error */
export declare class UclExecutionError extends Error {
    command?: string | undefined;
    constructor(message: string, command?: string | undefined);
}
/** UCL parse error */
export declare class UclParseError extends Error {
    line?: number | undefined;
    constructor(message: string, line?: number | undefined);
}
/** Execute UCL commands on a document */
export declare function executeUcl(doc: Document, uclText: string): BlockId[];
/** Event types for observability */
export type EventType = 'document.created' | 'document.modified' | 'block.added' | 'block.edited' | 'block.moved' | 'block.deleted' | 'edge.added' | 'edge.removed' | 'tag.added' | 'tag.removed' | 'ucl.parsed' | 'ucl.executed' | 'ucl.error' | 'validation.started' | 'validation.completed' | 'transaction.started' | 'transaction.committed' | 'transaction.rolled_back' | 'snapshot.created' | 'snapshot.restored';
/** UCP event */
export interface UcpEvent {
    type: EventType;
    timestamp: Date;
    data: Record<string, unknown>;
}
/** Event handler type */
export type EventHandler = (event: UcpEvent) => void;
/** Simple event bus for observability */
export declare class EventBus {
    private static instance;
    private handlers;
    private globalHandlers;
    static getInstance(): EventBus;
    subscribe(eventType: EventType, handler: EventHandler): void;
    subscribeAll(handler: EventHandler): void;
    unsubscribe(eventType: EventType, handler: EventHandler): void;
    unsubscribeAll(handler: EventHandler): void;
    publish(event: UcpEvent): void;
    clear(): void;
}
/** Emit an event to the global event bus */
export declare function emitEvent(type: EventType, data?: Record<string, unknown>): void;
/** Simple metrics collector */
export declare class Metrics {
    private static instance;
    private counters;
    private gauges;
    private histograms;
    static getInstance(): Metrics;
    increment(name: string, value?: number): void;
    setGauge(name: string, value: number): void;
    recordHistogram(name: string, value: number): void;
    getCounter(name: string): number;
    getGauge(name: string): number | undefined;
    getHistogram(name: string): number[];
    getAll(): {
        counters: Record<string, number>;
        gauges: Record<string, number>;
        histograms: Record<string, number[]>;
    };
    reset(): void;
}
/** Main UCP API - simple, unified interface */
export declare const ucp: {
    /** Parse markdown into a document */
    parse: typeof parseMarkdown;
    /** Render document to markdown */
    render: typeof renderMarkdown;
    /** Execute UCL commands */
    execute: typeof executeUcl;
    /** Validate a document */
    validate: typeof validateDocument;
    /** Create an empty document */
    create: typeof createDocument;
    /** Create a prompt builder */
    prompt: typeof prompt;
    /** Create an ID mapper from a document */
    mapIds: typeof mapIds;
    /** Create a UCL command builder */
    ucl: typeof ucl;
};
export default ucp;
export { writeSection, findSectionByPath, getAllSections, clearSectionWithUndo, restoreDeletedSection, type SectionWriteResult, type DeletedSectionContent, type ClearSectionResult, } from './section.js';
export { TraversalEngine, traverse, pathToRoot, expand, type NavigateDirection, type TraversalOutput, type TraversalFilter, type TraversalNode, type TraversalSummary, type TraversalResult, type TraversalConfig, } from './traversal.js';
export { ContextWindow, ContextManager, createContext, type InclusionReason, type ExpandDirection, type ExpansionPolicy, type PruningPolicy, type CompressionMethod, type ContextBlock, type ContextConstraints, type ContextUpdateResult, type ContextStatistics, type ContextMetadata, } from './context.js';
export { EdgeIndex } from './edge_index.js';
//# sourceMappingURL=index.d.ts.map