/**
 * Graph traversal operations for UCM documents.
 *
 * This module provides utilities for navigating the document's block structure,
 * including BFS, DFS, and path-finding operations.
 */

import type { BlockId, Document, Block } from './index.js'

/** Direction for navigation operations */
export type NavigateDirection = 'down' | 'up' | 'both' | 'siblings' | 'breadth_first' | 'depth_first'

/** Output format for traversal results */
export type TraversalOutput = 'structure_only' | 'structure_and_blocks' | 'structure_with_previews'

/** Filter criteria for traversal */
export interface TraversalFilter {
  includeRoles?: string[]
  excludeRoles?: string[]
  includeTags?: string[]
  excludeTags?: string[]
  contentPattern?: string
}

/** A node in the traversal result */
export interface TraversalNode {
  id: BlockId
  depth: number
  parentId?: BlockId
  contentPreview?: string
  semanticRole?: string
  childCount: number
  edgeCount: number
}

/** Summary statistics for a traversal */
export interface TraversalSummary {
  totalNodes: number
  totalEdges: number
  maxDepth: number
  nodesByRole: Record<string, number>
  truncated: boolean
  truncationReason?: string
}

/** Complete traversal result */
export interface TraversalResult {
  nodes: TraversalNode[]
  paths: BlockId[][]
  summary: TraversalSummary
}

/** Configuration for the traversal engine */
export interface TraversalConfig {
  maxDepth: number
  maxNodes: number
  defaultPreviewLength: number
}

const DEFAULT_CONFIG: TraversalConfig = {
  maxDepth: 100,
  maxNodes: 10000,
  defaultPreviewLength: 100,
}

/**
 * Graph traversal engine for UCM documents.
 */
export class TraversalEngine {
  private config: TraversalConfig

  constructor(config?: Partial<TraversalConfig>) {
    this.config = { ...DEFAULT_CONFIG, ...config }
  }

  /**
   * Navigate from a starting point in a specific direction.
   */
  navigate(
    doc: Document,
    startId?: BlockId,
    direction: NavigateDirection = 'breadth_first',
    depth?: number,
    filter?: TraversalFilter,
    output: TraversalOutput = 'structure_and_blocks'
  ): TraversalResult {
    const start = startId ?? doc.root
    const maxDepth = Math.min(depth ?? this.config.maxDepth, this.config.maxDepth)
    const filterObj = filter ?? {}

    switch (direction) {
      case 'breadth_first':
      case 'down':
        return this.traverseBfs(doc, start, maxDepth, filterObj, output)
      case 'depth_first':
        return this.traverseDfs(doc, start, maxDepth, filterObj, output)
      case 'up':
        return this.traverseUp(doc, start, maxDepth, filterObj, output)
      case 'siblings':
        return this.traverseSiblings(doc, start, filterObj, output)
      case 'both':
        return this.traverseBoth(doc, start, maxDepth, filterObj, output)
      default:
        return this.emptyResult()
    }
  }

  /**
   * Expand a node to get its immediate children.
   */
  expand(doc: Document, nodeId: BlockId, output: TraversalOutput = 'structure_and_blocks'): TraversalResult {
    return this.navigate(doc, nodeId, 'down', 1, undefined, output)
  }

  /**
   * Get the path from a node to the root.
   */
  pathToRoot(doc: Document, nodeId: BlockId): BlockId[] {
    const path: BlockId[] = [nodeId]
    let current = nodeId

    while (true) {
      const parent = this.getParent(doc, current)
      if (!parent) break
      path.push(parent)
      if (parent === doc.root) break
      current = parent
    }

    return path.reverse()
  }

  /**
   * Find all paths between two nodes.
   */
  findPaths(doc: Document, fromId: BlockId, toId: BlockId, maxPaths: number = 10): BlockId[][] {
    const paths: BlockId[][] = []
    const visited = new Set<BlockId>()
    const currentPath: BlockId[] = [fromId]

    this.findPathsRecursive(doc, fromId, toId, visited, currentPath, paths, maxPaths)

    return paths
  }

  private findPathsRecursive(
    doc: Document,
    current: BlockId,
    target: BlockId,
    visited: Set<BlockId>,
    currentPath: BlockId[],
    paths: BlockId[][],
    maxPaths: number
  ): void {
    if (paths.length >= maxPaths) return

    if (current === target) {
      paths.push([...currentPath])
      return
    }

    visited.add(current)

    const children = this.getChildren(doc, current)
    for (const child of children) {
      if (!visited.has(child)) {
        currentPath.push(child)
        this.findPathsRecursive(doc, child, target, visited, currentPath, paths, maxPaths)
        currentPath.pop()
      }
    }

    visited.delete(current)
  }

  private traverseBfs(
    doc: Document,
    start: BlockId,
    maxDepth: number,
    filter: TraversalFilter,
    output: TraversalOutput
  ): TraversalResult {
    const nodes: TraversalNode[] = []
    const visited = new Set<BlockId>()
    const queue: Array<{ id: BlockId; parentId?: BlockId; depth: number }> = [
      { id: start, parentId: undefined, depth: 0 },
    ]
    const nodesByRole: Record<string, number> = {}

    while (queue.length > 0 && nodes.length < this.config.maxNodes) {
      const { id, parentId, depth } = queue.shift()!

      if (depth > maxDepth || visited.has(id)) continue
      visited.add(id)

      const block = doc.blocks.get(id)
      if (block && this.matchesFilter(block, filter)) {
        const node = this.createNode(doc, id, depth, parentId, output)
        nodes.push(node)

        if (node.semanticRole) {
          nodesByRole[node.semanticRole] = (nodesByRole[node.semanticRole] || 0) + 1
        }

        const children = this.getChildren(doc, id)
        for (const child of children) {
          if (!visited.has(child)) {
            queue.push({ id: child, parentId: id, depth: depth + 1 })
          }
        }
      }
    }

    const maxDepthFound = Math.max(...nodes.map(n => n.depth), 0)

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
    }
  }

  private traverseDfs(
    doc: Document,
    start: BlockId,
    maxDepth: number,
    filter: TraversalFilter,
    output: TraversalOutput
  ): TraversalResult {
    const nodes: TraversalNode[] = []
    const visited = new Set<BlockId>()
    const nodesByRole: Record<string, number> = {}

    this.dfsRecursive(doc, start, undefined, 0, maxDepth, filter, output, visited, nodes, nodesByRole)

    const maxDepthFound = Math.max(...nodes.map(n => n.depth), 0)

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
    }
  }

  private dfsRecursive(
    doc: Document,
    nodeId: BlockId,
    parentId: BlockId | undefined,
    depth: number,
    maxDepth: number,
    filter: TraversalFilter,
    output: TraversalOutput,
    visited: Set<BlockId>,
    nodes: TraversalNode[],
    nodesByRole: Record<string, number>
  ): void {
    if (depth > maxDepth || visited.has(nodeId) || nodes.length >= this.config.maxNodes) return

    visited.add(nodeId)

    const block = doc.blocks.get(nodeId)
    if (block && this.matchesFilter(block, filter)) {
      const node = this.createNode(doc, nodeId, depth, parentId, output)
      nodes.push(node)

      if (node.semanticRole) {
        nodesByRole[node.semanticRole] = (nodesByRole[node.semanticRole] || 0) + 1
      }

      const children = this.getChildren(doc, nodeId)
      for (const child of children) {
        this.dfsRecursive(doc, child, nodeId, depth + 1, maxDepth, filter, output, visited, nodes, nodesByRole)
      }
    }
  }

  private traverseUp(
    doc: Document,
    start: BlockId,
    maxDepth: number,
    filter: TraversalFilter,
    output: TraversalOutput
  ): TraversalResult {
    const nodes: TraversalNode[] = []
    let current = start
    let depth = 0

    while (depth <= maxDepth) {
      const block = doc.blocks.get(current)
      if (block && this.matchesFilter(block, filter)) {
        nodes.push(this.createNode(doc, current, depth, undefined, output))
      }

      const parent = this.getParent(doc, current)
      if (!parent) break
      current = parent
      depth++
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
    }
  }

  private traverseSiblings(
    doc: Document,
    start: BlockId,
    filter: TraversalFilter,
    output: TraversalOutput
  ): TraversalResult {
    const nodes: TraversalNode[] = []
    const parent = this.getParent(doc, start)

    if (parent) {
      const siblings = this.getChildren(doc, parent)
      for (const sibling of siblings) {
        const block = doc.blocks.get(sibling)
        if (block && this.matchesFilter(block, filter)) {
          nodes.push(this.createNode(doc, sibling, 0, parent, output))
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
    }
  }

  private traverseBoth(
    doc: Document,
    start: BlockId,
    maxDepth: number,
    filter: TraversalFilter,
    output: TraversalOutput
  ): TraversalResult {
    const upResult = this.traverseUp(doc, start, maxDepth, filter, output)
    const downResult = this.traverseBfs(doc, start, maxDepth, filter, output)

    const seen = new Set<BlockId>()
    const nodes: TraversalNode[] = []

    for (const node of [...upResult.nodes, ...downResult.nodes]) {
      if (!seen.has(node.id)) {
        seen.add(node.id)
        nodes.push(node)
      }
    }

    const maxDepthFound = Math.max(...nodes.map(n => n.depth), 0)

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
    }
  }

  private matchesFilter(block: Block, filter: TraversalFilter): boolean {
    const role = block.metadata?.semanticRole || ''
    const tags = block.metadata?.tags || []

    if (filter.includeRoles?.length && !filter.includeRoles.includes(role)) {
      return false
    }

    if (filter.excludeRoles?.length && filter.excludeRoles.includes(role)) {
      return false
    }

    if (filter.includeTags?.length && !filter.includeTags.some(t => tags.includes(t))) {
      return false
    }

    if (filter.excludeTags?.length && filter.excludeTags.some(t => tags.includes(t))) {
      return false
    }

    if (filter.contentPattern) {
      const content = typeof block.content === 'string' ? block.content : ''
      if (!content.toLowerCase().includes(filter.contentPattern.toLowerCase())) {
        return false
      }
    }

    return true
  }

  private createNode(
    doc: Document,
    blockId: BlockId,
    depth: number,
    parentId: BlockId | undefined,
    output: TraversalOutput
  ): TraversalNode {
    const block = doc.blocks.get(blockId)
    const children = this.getChildren(doc, blockId)

    let contentPreview: string | undefined
    if (output !== 'structure_only' && block) {
      const content = typeof block.content === 'string' ? block.content : ''
      contentPreview = content.length > this.config.defaultPreviewLength
        ? content.substring(0, this.config.defaultPreviewLength) + '...'
        : content
    }

    return {
      id: blockId,
      depth,
      parentId,
      contentPreview,
      semanticRole: block?.metadata?.semanticRole,
      childCount: children.length,
      edgeCount: block?.edges?.length || 0,
    }
  }

  private getParent(doc: Document, blockId: BlockId): BlockId | undefined {
    for (const [id, block] of doc.blocks) {
      if (block.children?.includes(blockId)) {
        return id
      }
    }
    return undefined
  }

  private getChildren(doc: Document, blockId: BlockId): BlockId[] {
    return doc.blocks.get(blockId)?.children || []
  }

  private emptyResult(): TraversalResult {
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
    }
  }
}

// Convenience functions

/**
 * Convenience function for document traversal.
 */
export function traverse(
  doc: Document,
  startId?: BlockId,
  direction: NavigateDirection = 'breadth_first',
  depth?: number
): TraversalResult {
  return new TraversalEngine().navigate(doc, startId, direction, depth)
}

/**
 * Get the path from a node to the root.
 */
export function pathToRoot(doc: Document, nodeId: BlockId): BlockId[] {
  return new TraversalEngine().pathToRoot(doc, nodeId)
}

/**
 * Expand a node to get its immediate children.
 */
export function expand(doc: Document, nodeId: BlockId): TraversalResult {
  return new TraversalEngine().expand(doc, nodeId)
}
