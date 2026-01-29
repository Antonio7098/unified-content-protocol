/* tslint:disable */
/* eslint-disable */

export class Content {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Create code content.
   */
  static code(language: string, source: string): Content;
  /**
   * Create JSON content.
   */
  static json(value: any): Content;
  /**
   * Create math content (LaTeX by default).
   */
  static math(expression: string, display_mode?: boolean | null, format?: string | null): Content;
  /**
   * Create plain text content.
   */
  static text(text: string): Content;
  /**
   * Create media content (image, audio, video, document).
   */
  static media(media_type: string, url: string, alt_text?: string | null, width?: number | null, height?: number | null): Content;
  /**
   * Create table content from rows.
   */
  static table(rows: any): Content;
  /**
   * Create binary content.
   */
  static binary(mime_type: string, data: Uint8Array, encoding?: string | null): Content;
  /**
   * Get code content if this is a code block (returns object {language, source}).
   */
  asCode(): any;
  /**
   * Get JSON content if this is a JSON block.
   */
  asJson(): any;
  /**
   * Get math content if this is a math block (returns object {expression, displayMode, format}).
   */
  asMath(): any;
  /**
   * Get text content if this is a text block.
   */
  asText(): string | undefined;
  /**
   * Get media content if this is a media block (returns object {mediaType, url, altText}).
   */
  asMedia(): any;
  /**
   * Get table content if this is a table block (returns object {columns, rows}).
   */
  asTable(): any;
  /**
   * Create markdown text content.
   */
  static markdown(text: string): Content;
  /**
   * Get binary content if this is a binary block (returns object {mimeType, data}).
   */
  asBinary(): any;
  /**
   * Create composite content (container for other blocks).
   */
  static composite(layout?: string | null, children?: string[] | null): Content;
  /**
   * Get size in bytes.
   */
  readonly sizeBytes: number;
  /**
   * Get the content type.
   */
  readonly contentType: ContentType;
  /**
   * Check if empty.
   */
  readonly isEmpty: boolean;
  /**
   * Get the type tag string.
   */
  readonly typeTag: string;
}

/**
 * Content type enumeration.
 */
export enum ContentType {
  Text = 0,
  Code = 1,
  Table = 2,
  Math = 3,
  Media = 4,
  Json = 5,
  Binary = 6,
  Composite = 7,
}

export class Document {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Edit a block's content.
   */
  editBlock(id: string, content: string, role?: string | null): void;
  /**
   * Move a block to a new parent.
   */
  moveBlock(id: string, new_parent_id: string, index?: number | null): void;
  /**
   * Remove a tag from a block.
   */
  removeTag(id: string, tag: string): boolean;
  /**
   * Get the total block count.
   */
  blockCount(): number;
  /**
   * Get descendants of a block.
   */
  descendants(id: string): Array<any>;
  /**
   * Find blocks by tag.
   */
  findByTag(tag: string): Array<any>;
  /**
   * Check if one block is an ancestor of another.
   */
  isAncestor(potential_ancestor: string, block: string): boolean;
  /**
   * Remove an edge from one block to another.
   */
  removeEdge(source_id: string, edge_type: EdgeType, target_id: string): boolean;
  /**
   * Add a block at a specific index.
   */
  addBlockAt(parent_id: string, content: string, index: number, role?: string | null, label?: string | null): string;
  /**
   * Delete a block.
   */
  deleteBlock(id: string, cascade?: boolean | null): Array<any>;
  /**
   * Find blocks by semantic role.
   */
  findByRole(role: string): Array<any>;
  /**
   * Find blocks by content type.
   */
  findByType(content_type: string): Array<any>;
  /**
   * Check if a block is reachable from root.
   */
  isReachable(id: string): boolean;
  /**
   * Find a block by its label.
   */
  findByLabel(label: string): string | undefined;
  /**
   * Get the index of a block among its siblings.
   */
  siblingIndex(id: string): number | undefined;
  /**
   * Write markdown content into a section by block ID.
   */
  writeSection(section_id: string, markdown: string, base_heading_level?: number | null): WasmWriteSectionResult;
  /**
   * Get incoming edges to a block.
   */
  incomingEdges(id: string): any;
  /**
   * Get outgoing edges from a block.
   */
  outgoingEdges(id: string): any;
  /**
   * Get the path from root to a block (list of block IDs).
   */
  pathFromRoot(id: string): Array<any>;
  /**
   * Edit a block with specific content type.
   */
  editBlockContent(id: string, content: Content, role?: string | null): void;
  /**
   * Add a block with specific content.
   */
  addBlockWithContent(parent_id: string, content: Content, role?: string | null, label?: string | null): string;
  /**
   * Get the depth of a block from the root (root has depth 0).
   */
  depth(id: string): number;
  /**
   * Get all blocks in the document.
   */
  blocks(): any;
  /**
   * Create a new empty document.
   */
  constructor(title?: string | null);
  /**
   * Get parent of a block.
   */
  parent(child_id: string): string | undefined;
  /**
   * Add a tag to a block.
   */
  addTag(id: string, tag: string): void;
  /**
   * Serialize to JSON object.
   */
  toJson(): any;
  /**
   * Add a code block.
   */
  addCode(parent_id: string, language: string, source: string, label?: string | null): string;
  /**
   * Add an edge from one block to another.
   */
  addEdge(source_id: string, edge_type: EdgeType, target_id: string): void;
  /**
   * Get children of a block.
   */
  children(parent_id: string): Array<any>;
  /**
   * Get the siblings of a block (children of same parent, excluding self).
   */
  siblings(id: string): Array<any>;
  /**
   * Validate the document.
   */
  validate(): any;
  /**
   * Add a new text block.
   */
  addBlock(parent_id: string, content: string, role?: string | null, label?: string | null): string;
  /**
   * Get all ancestors of a block (from parent to root).
   */
  ancestors(id: string): Array<any>;
  /**
   * Get all block IDs in the document.
   */
  blockIds(): Array<any>;
  /**
   * Get a block by ID (returns JSON representation).
   */
  getBlock(id: string): any;
  /**
   * Set a block's label.
   */
  setLabel(id: string, label?: string | null): void;
  /**
   * Get created timestamp as ISO 8601 string.
   */
  readonly createdAt: string;
  /**
   * Get the document description.
   */
  get description(): string | undefined;
  /**
   * Set the document description.
   */
  set description(value: string | null | undefined);
  /**
   * Get modified timestamp as ISO 8601 string.
   */
  readonly modifiedAt: string;
  /**
   * Get the document ID.
   */
  readonly id: string;
  /**
   * Get the document title.
   */
  get title(): string | undefined;
  /**
   * Set the document title.
   */
  set title(value: string | null | undefined);
  /**
   * Get the root block ID.
   */
  readonly rootId: string;
  /**
   * Get document version.
   */
  readonly version: bigint;
}

/**
 * Edge type enumeration.
 */
export enum EdgeType {
  DerivedFrom = 0,
  Supersedes = 1,
  TransformedFrom = 2,
  References = 3,
  CitedBy = 4,
  LinksTo = 5,
  Supports = 6,
  Contradicts = 7,
  Elaborates = 8,
  Summarizes = 9,
  ParentOf = 10,
  ChildOf = 11,
  SiblingOf = 12,
  PreviousSibling = 13,
  NextSibling = 14,
  VersionOf = 15,
  AlternativeOf = 16,
  TranslationOf = 17,
}

export class IdMapper {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Convert UCL commands from short numeric IDs back to full BlockIds.
   */
  expandUcl(ucl: string): string;
  /**
   * Convert a string containing short IDs back to block IDs.
   */
  expandText(text: string): string;
  /**
   * Convert UCL commands from long BlockIds to short numeric IDs.
   */
  shortenUcl(ucl: string): string;
  /**
   * Get BlockId for a short ID.
   */
  toBlockId(short_id: number): string | undefined;
  /**
   * Get short ID for a BlockId.
   */
  toShortId(block_id: string): number | undefined;
  /**
   * Convert a string containing block IDs to use short IDs.
   */
  shortenText(text: string): string;
  /**
   * Create a mapper from a document, assigning sequential IDs to all blocks.
   */
  static fromDocument(doc: Document): IdMapper;
  /**
   * Get the mapping table as a string (useful for debugging).
   */
  mappingTable(): string;
  /**
   * Generate a normalized document representation for LLM prompts.
   */
  documentToPrompt(doc: Document): string;
  /**
   * Estimate token savings from using short IDs.
   * Returns { originalTokens, shortenedTokens, savings }.
   */
  estimateTokenSavings(text: string): any;
  /**
   * Create a new empty IdMapper.
   */
  constructor();
  /**
   * Register a BlockId and get its short ID.
   */
  register(block_id: string): number;
  /**
   * Total number of mappings.
   */
  readonly length: number;
}

export class PromptBuilder {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Build a complete prompt with document context.
   */
  buildPrompt(document_description: string, task: string): string;
  /**
   * Check if a capability is enabled.
   */
  hasCapability(cap: WasmUclCapability): boolean;
  /**
   * Enable short ID mode (for token efficiency).
   */
  withShortIds(enabled: boolean): PromptBuilder;
  /**
   * Add a single capability.
   */
  withCapability(cap: WasmUclCapability): PromptBuilder;
  /**
   * Set task-specific context.
   */
  withTaskContext(context: string): PromptBuilder;
  /**
   * Remove a capability.
   */
  withoutCapability(cap: WasmUclCapability): PromptBuilder;
  /**
   * Build the system prompt.
   */
  buildSystemPrompt(): string;
  /**
   * Set custom system context (prepended to prompt).
   */
  withSystemContext(context: string): PromptBuilder;
  /**
   * Create a builder with all capabilities enabled.
   */
  static withAllCapabilities(): PromptBuilder;
  /**
   * Create a new prompt builder with no capabilities.
   */
  constructor();
  /**
   * Add a custom rule.
   */
  withRule(rule: string): PromptBuilder;
}

export class PromptPresets {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Full document editing (all except transactions).
   */
  static fullEditing(): PromptBuilder;
  /**
   * Basic editing only (EDIT, APPEND, DELETE).
   */
  static basicEditing(): PromptBuilder;
  /**
   * Token-efficient mode with short IDs.
   */
  static tokenEfficient(): PromptBuilder;
  /**
   * Version control focused.
   */
  static versionControl(): PromptBuilder;
  /**
   * Structure manipulation (MOVE, LINK).
   */
  static structureManipulation(): PromptBuilder;
}

export class SnapshotManager {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Get information about a snapshot.
   */
  get(name: string): any;
  /**
   * Create a new snapshot manager.
   */
  constructor(max_snapshots?: number | null);
  /**
   * List all snapshots (most recent first).
   */
  list(): any;
  /**
   * Create a snapshot of a document.
   */
  create(name: string, doc: Document, description?: string | null): string;
  /**
   * Delete a snapshot.
   */
  delete(name: string): boolean;
  /**
   * Check if a snapshot exists.
   */
  exists(name: string): boolean;
  /**
   * Restore a document from a snapshot.
   */
  restore(name: string): Document;
  /**
   * Get snapshot count.
   */
  readonly length: number;
}

export class WasmAuditEntry {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Set the duration in milliseconds.
   */
  withDuration(duration_ms: bigint): WasmAuditEntry;
  /**
   * Create a new audit entry.
   */
  constructor(operation: string, document_id: string);
  /**
   * Mark as failed.
   */
  failed(): WasmAuditEntry;
  /**
   * Convert to JSON object.
   */
  toJson(): any;
  /**
   * Set the user ID.
   */
  withUser(user_id: string): WasmAuditEntry;
  /**
   * Get the document ID.
   */
  readonly documentId: string;
  /**
   * Get the duration in milliseconds.
   */
  readonly durationMs: bigint;
  /**
   * Check if the operation was successful.
   */
  readonly success: boolean;
  /**
   * Get the user ID if present.
   */
  readonly userId: string | undefined;
  /**
   * Get the operation name.
   */
  readonly operation: string;
  /**
   * Get the timestamp as ISO 8601 string.
   */
  readonly timestamp: string;
}

export class WasmClearResult {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Get the IDs of removed blocks.
   */
  readonly removedIds: Array<any>;
  /**
   * Get the deleted content for potential restoration.
   */
  readonly deletedContent: WasmDeletedContent;
  /**
   * Get the number of removed blocks.
   */
  readonly length: number;
}

export class WasmDeletedContent {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Serialize to JSON string for persistence.
   */
  toJson(): string;
  /**
   * Get all block IDs in the deleted content.
   */
  blockIds(): Array<any>;
  /**
   * Deserialize from JSON string.
   */
  static fromJson(json_str: string): WasmDeletedContent;
  /**
   * Get the deletion timestamp as ISO 8601 string.
   */
  readonly deletedAt: string;
  /**
   * Get the number of deleted blocks.
   */
  readonly blockCount: number;
  /**
   * Check if there is any deleted content.
   */
  readonly isEmpty: boolean;
  /**
   * Get the parent block ID where this content was attached.
   */
  readonly parentId: string;
}

export class WasmEngine {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * List all snapshots.
   */
  listSnapshots(): Array<any>;
  /**
   * Create a snapshot.
   */
  createSnapshot(name: string, doc: Document, description?: string | null): void;
  /**
   * Delete a snapshot.
   */
  deleteSnapshot(name: string): boolean;
  /**
   * Restore from a snapshot.
   */
  restoreSnapshot(name: string): Document;
  /**
   * Begin a new transaction.
   */
  beginTransaction(): string;
  /**
   * Rollback a transaction.
   */
  rollbackTransaction(txn_id: string): void;
  /**
   * Begin a named transaction.
   */
  beginNamedTransaction(name: string): string;
  constructor(config?: WasmEngineConfig | null);
  /**
   * Validate a document.
   */
  validate(doc: Document): WasmValidationResult;
}

export class WasmEngineConfig {
  free(): void;
  [Symbol.dispose](): void;
  constructor(validate_on_operation?: boolean | null, max_batch_size?: number | null, enable_transactions?: boolean | null, enable_snapshots?: boolean | null);
  readonly maxBatchSize: number;
  readonly enableSnapshots: boolean;
  readonly enableTransactions: boolean;
  readonly validateOnOperation: boolean;
}

export class WasmMetricsRecorder {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Record a snapshot creation.
   */
  recordSnapshot(): void;
  /**
   * Record an operation.
   */
  recordOperation(success: boolean): void;
  /**
   * Record a block creation.
   */
  recordBlockCreated(): void;
  /**
   * Record a block deletion.
   */
  recordBlockDeleted(): void;
  /**
   * Create a new metrics recorder.
   */
  constructor();
  /**
   * Convert to JSON object.
   */
  toJson(): any;
  /**
   * Get blocks created count.
   */
  readonly blocksCreated: bigint;
  /**
   * Get blocks deleted count.
   */
  readonly blocksDeleted: bigint;
  /**
   * Get total operations count.
   */
  readonly operationsTotal: bigint;
  /**
   * Get failed operations count.
   */
  readonly operationsFailed: bigint;
  /**
   * Get snapshots created count.
   */
  readonly snapshotsCreated: bigint;
}

export class WasmResourceLimits {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Create default resource limits.
   */
  static defaultLimits(): WasmResourceLimits;
  constructor(max_document_size?: number | null, max_block_count?: number | null, max_block_size?: number | null, max_nesting_depth?: number | null, max_edges_per_block?: number | null);
  /**
   * Convert to JSON object.
   */
  toJson(): any;
  readonly maxBlockSize: number;
  readonly maxBlockCount: number;
  readonly maxDocumentSize: number;
  readonly maxNestingDepth: number;
  readonly maxEdgesPerBlock: number;
}

export class WasmTraversalConfig {
  free(): void;
  [Symbol.dispose](): void;
  constructor(max_depth?: number | null, max_nodes?: number | null, include_orphans?: boolean | null);
  readonly maxDepth: number;
  readonly maxNodes: number;
}

export class WasmTraversalEngine {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Find all paths between two nodes.
   */
  findPaths(doc: Document, from_id: string, to_id: string, max_paths?: number | null): Array<any>;
  /**
   * Get the path from a node to the root.
   */
  pathToRoot(doc: Document, node_id: string): Array<any>;
  constructor(config?: WasmTraversalConfig | null);
  /**
   * Expand a node to get its immediate children.
   */
  expand(doc: Document, node_id: string): any;
  /**
   * Navigate from a starting point in a specific direction.
   */
  navigate(doc: Document, direction: string, start_id?: string | null, depth?: number | null, filter?: WasmTraversalFilter | null): any;
}

export class WasmTraversalFilter {
  free(): void;
  [Symbol.dispose](): void;
  constructor(include_roles?: string[] | null, exclude_roles?: string[] | null, include_tags?: string[] | null, exclude_tags?: string[] | null, content_pattern?: string | null);
}

/**
 * UCL command capability enumeration.
 */
export enum WasmUclCapability {
  Edit = 0,
  Append = 1,
  Move = 2,
  Delete = 3,
  Link = 4,
  Snapshot = 5,
  Transaction = 6,
}

export class WasmUcpEvent {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Create a block added event.
   */
  static blockAdded(document_id: string, block_id: string, parent_id: string, content_type: string): WasmUcpEvent;
  /**
   * Create a block deleted event.
   */
  static blockDeleted(document_id: string, block_id: string, cascade: boolean): WasmUcpEvent;
  /**
   * Create a document created event.
   */
  static documentCreated(document_id: string): WasmUcpEvent;
  /**
   * Create a snapshot created event.
   */
  static snapshotCreated(document_id: string, snapshot_name: string): WasmUcpEvent;
  /**
   * Convert to JSON object.
   */
  toJson(): any;
  /**
   * Get the event type.
   */
  readonly eventType: string;
  /**
   * Get the document ID if present.
   */
  readonly documentId: string | undefined;
  /**
   * Get event details as JSON string.
   */
  readonly details: string;
  /**
   * Get the timestamp as ISO 8601 string.
   */
  readonly timestamp: string;
}

export class WasmValidationIssue {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Convert to JSON object.
   */
  toJson(): any;
  readonly code: string;
  readonly message: string;
  readonly severity: string;
}

export class WasmValidationPipeline {
  free(): void;
  [Symbol.dispose](): void;
  constructor(limits?: WasmResourceLimits | null);
  /**
   * Validate a document.
   */
  validate(doc: Document): WasmValidationResult;
}

export class WasmValidationResult {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Convert to JSON object.
   */
  toJson(): any;
  /**
   * Get error count.
   */
  readonly errorCount: number;
  /**
   * Get warning count.
   */
  readonly warningCount: number;
  readonly valid: boolean;
  readonly issues: Array<any>;
}

export class WasmWriteSectionResult {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  readonly sectionId: string;
  readonly blocksAdded: Array<any>;
  readonly blocksRemoved: Array<any>;
  readonly success: boolean;
}

/**
 * Clear a section's content with undo support.
 */
export function clearSectionWithUndo(doc: Document, section_id: string): WasmClearResult;

/**
 * Create a new empty document.
 */
export function createDocument(title?: string | null): Document;

/**
 * Execute UCL commands on a document.
 */
export function executeUcl(doc: Document, ucl: string): Array<any>;

/**
 * Find a section by path (e.g., "Introduction > Getting Started").
 */
export function findSectionByPath(doc: Document, path: string): string | undefined;

/**
 * Get all sections (heading blocks) in the document.
 */
export function getAllSections(doc: Document): any;

/**
 * Get the depth of a section in the document hierarchy.
 */
export function getSectionDepth(doc: Document, section_id: string): number | undefined;

/**
 * Initialize panic hook for better error messages in WASM.
 */
export function init(): void;

/**
 * Parse HTML into a Document.
 */
export function parseHtml(html: string): Document;

/**
 * Parse markdown into a Document.
 */
export function parseMarkdown(markdown: string): Document;

/**
 * Render a Document to markdown.
 */
export function renderMarkdown(doc: Document): string;

/**
 * Restore previously deleted section content.
 */
export function restoreDeletedSection(doc: Document, deleted: WasmDeletedContent): Array<any>;

/**
 * Get the library version.
 */
export function version(): string;

/**
 * Write markdown content into a section, replacing its children.
 */
export function writeSection(doc: Document, section_id: string, markdown: string, base_heading_level?: number | null): WasmWriteSectionResult;
