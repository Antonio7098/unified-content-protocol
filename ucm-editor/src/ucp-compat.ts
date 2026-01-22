/**
 * Compatibility layer for the new WASM-based UCP API.
 * 
 * This provides adapters to make the new API work with the existing editor code.
 */

import { Document, ContentType, EdgeType } from 'ucp-content'

// Type aliases for types not exported from WASM
export type BlockId = string
export interface Block {
  id: BlockId
  content: string
  type: ContentType
  role?: string
  label?: string
  tags: string[]
  children: BlockId[]
  edges: Array<{ edgeType: EdgeType; target: BlockId; metadata?: Record<string, unknown> }>
}

/**
 * Adapter class that wraps the new Document API to provide the old interface
 */
export class DocumentAdapter {
  constructor(private doc: Document) {}

  // Get the document ID
  get id(): string {
    return this.doc.id || ''
  }

  // Get the root block ID
  get root(): BlockId {
    return this.doc.rootId || ''
  }

  // Get blocks as a Map-like interface
  get blocks(): Map<BlockId, Block> {
    const blockIds = this.doc.blockIds() || []
    const blocks = new Map<BlockId, Block>()
    
    for (const id of blockIds) {
      const block = this.doc.getBlock(id)
      if (block) {
        blocks.set(id, {
          id: block.id,
          content: block.content,
          type: block.type,
          role: block.role,
          label: block.label,
          tags: block.tags || [],
          children: block.children || [],
          edges: block.edges || []
        })
      }
    }
    
    return blocks
  }

  // Get document metadata
  get metadata() {
    return {
      title: this.doc.title,
      description: this.doc.description,
      createdAt: new Date(this.doc.createdAt),
      modifiedAt: new Date(this.doc.modifiedAt),
      custom: {}
    }
  }

  // Get document version
  get version(): number {
    return Number(this.doc.version || 1)
  }

  // Delegate method calls to the wrapped document
  add_block(parentId: BlockId, content: string, type?: ContentType): BlockId {
    return this.doc.addBlock(parentId, content, type)
  }

  edit_block(blockId: BlockId, content: string): void {
    this.doc.editBlock(blockId, content)
  }

  delete_block(blockId: BlockId, cascade?: boolean): void {
    this.doc.deleteBlock(blockId, cascade)
  }

  move_block(blockId: BlockId, newParentId: BlockId, index?: number): void {
    this.doc.moveBlock(blockId, newParentId, index)
  }

  get_block(blockId: BlockId): Block | null {
    const block = this.doc.getBlock(blockId)
    if (!block) return null
    
    return {
      id: block.id,
      content: block.content,
      type: block.type,
      role: block.role,
      label: block.label,
      tags: block.tags || [],
      children: block.children || [],
      edges: block.edges || []
    }
  }

  parent(blockId: BlockId): BlockId | undefined {
    return this.doc.parent(blockId)
  }

  child_index(blockId: BlockId): number {
    return this.doc.childIndex ? this.doc.childIndex(blockId) : 0
  }

  add_edge(sourceId: BlockId, targetId: BlockId, edgeType: EdgeType): void {
    this.doc.addEdge(sourceId, edgeType, targetId)
  }

  remove_edge(sourceId: BlockId, targetId: BlockId, edgeType: EdgeType): void {
    this.doc.removeEdge(sourceId, edgeType, targetId)
  }

  validate() {
    return this.doc.validate()
  }

  to_json() {
    return this.doc.toJson()
  }

  block_count(): number {
    return this.doc.blockCount()
  }

  // Get all block IDs
  block_ids(): BlockId[] {
    return this.doc.blockIds()
  }
}

/**
 * Create a new document with the adapter
 */
export function createDocument(title?: string): DocumentAdapter {
  const doc = new Document(title)
  return new DocumentAdapter(doc)
}
