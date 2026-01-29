/**
 * Section-based operations for UCM documents.
 *
 * This module provides utilities for section-based markdown writing,
 * allowing efficient bulk updates to document sections.
 */
import type { BlockId, Document, Block } from './index.js';
/** Result of a section write operation */
export interface SectionWriteResult {
    success: boolean;
    sectionId: BlockId;
    blocksRemoved: BlockId[];
    blocksAdded: BlockId[];
    error?: string;
}
/** Deleted content snapshot for undo operations */
export interface DeletedSectionContent {
    parentId: BlockId;
    blocks: Record<BlockId, Block>;
    structure: Record<BlockId, BlockId[]>;
    deletedAt: number;
}
/** Result of clearing a section with undo support */
export interface ClearSectionResult {
    removedIds: BlockId[];
    deletedContent: DeletedSectionContent;
}
/**
 * Write markdown content to a section, replacing all children.
 *
 * @param doc - The document to modify
 * @param sectionId - The block ID of the section heading
 * @param markdown - Markdown content to write
 * @param baseHeadingLevel - Optional base level for heading adjustment
 * @returns SectionWriteResult with details of the operation
 *
 * @example
 * ```typescript
 * const result = writeSection(doc, 'blk_123', '## New Section\n\nContent here')
 * console.log(`Added ${result.blocksAdded.length} blocks`)
 * ```
 */
export declare function writeSection(doc: Document, sectionId: BlockId, markdown: string, baseHeadingLevel?: number): SectionWriteResult;
/**
 * Clear a section while preserving content for undo.
 */
export declare function clearSectionWithUndo(doc: Document, sectionId: BlockId): ClearSectionResult;
/**
 * Restore section content from a previously captured snapshot.
 */
export declare function restoreDeletedSection(doc: Document, content: DeletedSectionContent): BlockId[];
/**
 * Find a section by its path in the document hierarchy.
 *
 * @param doc - The document to search
 * @param path - Path like "Introduction > Getting Started"
 * @returns Block ID of the section, or undefined if not found
 *
 * @example
 * ```typescript
 * const sectionId = findSectionByPath(doc, 'Chapter 1 > Section 1.1')
 * ```
 */
export declare function findSectionByPath(doc: Document, path: string): BlockId | undefined;
/**
 * Get all sections (heading blocks) in the document.
 *
 * @param doc - The document to search
 * @returns Array of [blockId, headingLevel] tuples
 *
 * @example
 * ```typescript
 * const sections = getAllSections(doc)
 * for (const [blockId, level] of sections) {
 *   console.log(`H${level}: ${doc.blocks.get(blockId)?.content}`)
 * }
 * ```
 */
export declare function getAllSections(doc: Document): Array<[BlockId, number]>;
//# sourceMappingURL=section.d.ts.map