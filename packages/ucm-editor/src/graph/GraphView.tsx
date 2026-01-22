/**
 * GraphView - Visual graph representation of the document structure.
 *
 * Displays blocks as nodes and relationships as edges in an interactive
 * DAG (Directed Acyclic Graph) visualization.
 */

import React, { useCallback, useMemo, useRef, useState, useEffect } from 'react'
import type { Document, ContentType, EdgeType } from 'ucp-content'

// Type alias for BlockId since it's not exported
type BlockId = string
import type { EditorStoreInstance } from '../core/EditorStore.js'
import type { GraphLayout, GraphNode, GraphEdge } from '../types/editor.js'
import { useEditorState } from '../hooks/useEditor.js'

// =============================================================================
// STYLES
// =============================================================================

const styles = {
  container: {
    position: 'relative' as const,
    width: '100%',
    height: '100%',
    overflow: 'hidden',
    backgroundColor: '#fafafa',
  },
  canvas: {
    position: 'absolute' as const,
    top: 0,
    left: 0,
    width: '100%',
    height: '100%',
  },
  controls: {
    position: 'absolute' as const,
    top: '16px',
    right: '16px',
    display: 'flex',
    flexDirection: 'column' as const,
    gap: '8px',
    zIndex: 10,
  },
  controlGroup: {
    display: 'flex',
    gap: '4px',
    backgroundColor: '#fff',
    padding: '4px',
    borderRadius: '4px',
    boxShadow: '0 2px 4px rgba(0, 0, 0, 0.1)',
  },
  controlButton: {
    padding: '6px 12px',
    border: '1px solid #ddd',
    borderRadius: '4px',
    backgroundColor: '#fff',
    cursor: 'pointer',
    fontSize: '12px',
  },
  controlButtonActive: {
    backgroundColor: '#e3f2fd',
    borderColor: '#2196f3',
  },
  legend: {
    position: 'absolute' as const,
    bottom: '16px',
    left: '16px',
    backgroundColor: '#fff',
    padding: '12px',
    borderRadius: '4px',
    boxShadow: '0 2px 4px rgba(0, 0, 0, 0.1)',
    fontSize: '11px',
  },
  legendTitle: {
    fontWeight: 600,
    marginBottom: '8px',
  },
  legendItem: {
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    marginBottom: '4px',
  },
  legendColor: {
    width: '16px',
    height: '3px',
    borderRadius: '1px',
  },
  minimap: {
    position: 'absolute' as const,
    bottom: '16px',
    right: '16px',
    width: '150px',
    height: '100px',
    backgroundColor: '#fff',
    border: '1px solid #ddd',
    borderRadius: '4px',
    overflow: 'hidden',
  },
  nodeTooltip: {
    position: 'absolute' as const,
    backgroundColor: '#333',
    color: '#fff',
    padding: '6px 10px',
    borderRadius: '4px',
    fontSize: '11px',
    maxWidth: '200px',
    zIndex: 100,
    pointerEvents: 'none' as const,
  },
}

// =============================================================================
// EDGE COLORS
// =============================================================================

const EDGE_COLORS: Record<string, string> = {
  // Structural (default)
  parent_child: '#9e9e9e',
  // Derivation
  derived_from: '#2196f3',
  supersedes: '#1976d2',
  transformed_from: '#1565c0',
  // Reference
  references: '#4caf50',
  cited_by: '#388e3c',
  links_to: '#2e7d32',
  // Semantic
  supports: '#9c27b0',
  contradicts: '#f44336',
  elaborates: '#7b1fa2',
  summarizes: '#6a1b9a',
  // Version
  version_of: '#ff9800',
  alternative_of: '#f57c00',
  translation_of: '#ef6c00',
}

// =============================================================================
// GRAPH VIEW PROPS
// =============================================================================

export interface GraphViewProps {
  store: EditorStoreInstance
}

// =============================================================================
// GRAPH VIEW COMPONENT
// =============================================================================

/**
 * Interactive graph visualization of the document structure.
 */
export function GraphView({ store }: GraphViewProps): React.ReactElement {
  const containerRef = useRef<HTMLDivElement>(null)
  const canvasRef = useRef<HTMLCanvasElement>(null)

  const document = useEditorState(store, (s) => s.document)
  const graphState = useEditorState(store, (s) => s.graph)
  const selection = useEditorState(store, (s) => s.selection)

  const [viewport, setViewport] = useState({ x: 0, y: 0, zoom: 1 })
  const [isDragging, setIsDragging] = useState(false)
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 })
  const [hoveredNodeId, setHoveredNodeId] = useState<BlockId | null>(null)
  const [tooltipPos, setTooltipPos] = useState({ x: 0, y: 0 })

  // Compute graph layout
  const { nodes, edges } = useMemo(() => {
    if (!document) {
      return { nodes: new Map<BlockId, GraphNode>(), edges: [] }
    }

    return computeLayout(document, graphState.layout, selection.focusedBlockId)
  }, [document, graphState.layout, selection.focusedBlockId])

  // Render canvas
  useEffect(() => {
    const canvas = canvasRef.current
    const container = containerRef.current
    if (!canvas || !container) return

    const ctx = canvas.getContext('2d')
    if (!ctx) return

    // Set canvas size
    const rect = container.getBoundingClientRect()
    canvas.width = rect.width * window.devicePixelRatio
    canvas.height = rect.height * window.devicePixelRatio
    canvas.style.width = `${rect.width}px`
    canvas.style.height = `${rect.height}px`
    ctx.scale(window.devicePixelRatio, window.devicePixelRatio)

    // Clear
    ctx.clearRect(0, 0, rect.width, rect.height)

    // Apply viewport transform
    ctx.save()
    ctx.translate(viewport.x + rect.width / 2, viewport.y + rect.height / 2)
    ctx.scale(viewport.zoom, viewport.zoom)

    // Draw edges
    for (const edge of edges) {
      drawEdge(ctx, edge, nodes, hoveredNodeId)
    }

    // Draw nodes
    for (const node of nodes.values()) {
      drawNode(ctx, node, selection.focusedBlockId === node.id, hoveredNodeId === node.id)
    }

    ctx.restore()
  }, [nodes, edges, viewport, selection.focusedBlockId, hoveredNodeId])

  // Mouse handlers
  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    setIsDragging(true)
    setDragStart({ x: e.clientX - viewport.x, y: e.clientY - viewport.y })
  }, [viewport])

  const handleMouseMove = useCallback(
    (e: React.MouseEvent) => {
      if (isDragging) {
        setViewport({
          ...viewport,
          x: e.clientX - dragStart.x,
          y: e.clientY - dragStart.y,
        })
      } else {
        // Check for node hover
        const container = containerRef.current
        if (!container) return

        const rect = container.getBoundingClientRect()
        const mouseX = (e.clientX - rect.left - rect.width / 2 - viewport.x) / viewport.zoom
        const mouseY = (e.clientY - rect.top - rect.height / 2 - viewport.y) / viewport.zoom

        let found: BlockId | null = null
        for (const node of nodes.values()) {
          if (
            mouseX >= node.x - node.width / 2 &&
            mouseX <= node.x + node.width / 2 &&
            mouseY >= node.y - node.height / 2 &&
            mouseY <= node.y + node.height / 2
          ) {
            found = node.id
            setTooltipPos({ x: e.clientX, y: e.clientY })
            break
          }
        }
        setHoveredNodeId(found)
      }
    },
    [isDragging, dragStart, viewport, nodes]
  )

  const handleMouseUp = useCallback(() => {
    setIsDragging(false)
  }, [])

  const handleClick = useCallback(() => {
    if (hoveredNodeId) {
      store.select(hoveredNodeId)
    }
  }, [store, hoveredNodeId])

  const handleWheel = useCallback(
    (e: React.WheelEvent) => {
      e.preventDefault()
      const delta = e.deltaY > 0 ? 0.9 : 1.1
      setViewport((v) => ({
        ...v,
        zoom: Math.max(0.1, Math.min(3, v.zoom * delta)),
      }))
    },
    []
  )

  // Control handlers
  const handleLayoutChange = useCallback(
    (layout: GraphLayout) => {
      store.setGraphLayout(layout)
    },
    [store]
  )

  const handleZoomIn = useCallback(() => {
    setViewport((v) => ({ ...v, zoom: Math.min(3, v.zoom * 1.2) }))
  }, [])

  const handleZoomOut = useCallback(() => {
    setViewport((v) => ({ ...v, zoom: Math.max(0.1, v.zoom / 1.2) }))
  }, [])

  const handleResetView = useCallback(() => {
    setViewport({ x: 0, y: 0, zoom: 1 })
  }, [])

  // Get hovered node for tooltip
  const hoveredNode = hoveredNodeId ? nodes.get(hoveredNodeId) : null

  return (
    <div
      ref={containerRef}
      style={styles.container}
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      onMouseLeave={handleMouseUp}
      onClick={handleClick}
      onWheel={handleWheel}
      data-testid="graph-view"
    >
      <canvas ref={canvasRef} style={styles.canvas} />

      {/* Controls */}
      <div style={styles.controls}>
        {/* Layout selector */}
        <div style={styles.controlGroup}>
          <LayoutButton
            layout="hierarchical"
            currentLayout={graphState.layout}
            onClick={handleLayoutChange}
          />
          <LayoutButton
            layout="force"
            currentLayout={graphState.layout}
            onClick={handleLayoutChange}
          />
          <LayoutButton
            layout="radial"
            currentLayout={graphState.layout}
            onClick={handleLayoutChange}
          />
        </div>

        {/* Zoom controls */}
        <div style={styles.controlGroup}>
          <button onClick={handleZoomIn} style={styles.controlButton}>
            +
          </button>
          <button onClick={handleZoomOut} style={styles.controlButton}>
            -
          </button>
          <button onClick={handleResetView} style={styles.controlButton}>
            Reset
          </button>
        </div>
      </div>

      {/* Legend */}
      <div style={styles.legend}>
        <div style={styles.legendTitle}>Edge Types</div>
        <div style={styles.legendItem}>
          <div style={{ ...styles.legendColor, backgroundColor: EDGE_COLORS.parent_child }} />
          <span>Parent-Child</span>
        </div>
        <div style={styles.legendItem}>
          <div style={{ ...styles.legendColor, backgroundColor: EDGE_COLORS.references }} />
          <span>References</span>
        </div>
        <div style={styles.legendItem}>
          <div style={{ ...styles.legendColor, backgroundColor: EDGE_COLORS.supports }} />
          <span>Semantic</span>
        </div>
      </div>

      {/* Node tooltip */}
      {hoveredNode && (
        <div
          style={{
            ...styles.nodeTooltip,
            left: tooltipPos.x + 10,
            top: tooltipPos.y + 10,
          }}
        >
          <div><strong>{hoveredNode.block.type}</strong></div>
          <div>
            {hoveredNode.block.content.slice(0, 50)}
            {hoveredNode.block.content.length > 50 ? '...' : ''}
          </div>
          <div style={{ color: '#aaa', fontSize: '9px' }}>
            {hoveredNode.block.children.length} children, {hoveredNode.block.edges.length} edges
          </div>
        </div>
      )}
    </div>
  )
}

// =============================================================================
// LAYOUT BUTTON
// =============================================================================

interface LayoutButtonProps {
  layout: GraphLayout
  currentLayout: GraphLayout
  onClick: (layout: GraphLayout) => void
}

function LayoutButton({ layout, currentLayout, onClick }: LayoutButtonProps) {
  const labels: Record<GraphLayout, string> = {
    hierarchical: 'Tree',
    force: 'Force',
    dagre: 'DAG',
    radial: 'Radial',
  }

  return (
    <button
      onClick={() => onClick(layout)}
      style={{
        ...styles.controlButton,
        ...(layout === currentLayout ? styles.controlButtonActive : {}),
      }}
    >
      {labels[layout]}
    </button>
  )
}

// =============================================================================
// LAYOUT COMPUTATION
// =============================================================================

function computeLayout(
  doc: Document,
  layout: GraphLayout,
  selectedId?: BlockId
): { nodes: Map<BlockId, GraphNode>; edges: GraphEdge[] } {
  const nodes = new Map<BlockId, GraphNode>()
  const edges: GraphEdge[] = []

  // Node dimensions
  const NODE_WIDTH = 120
  const NODE_HEIGHT = 40
  const H_SPACING = 40
  const V_SPACING = 60

  // Compute node positions based on layout
  switch (layout) {
    case 'hierarchical':
      computeHierarchicalLayout(doc, nodes, NODE_WIDTH, NODE_HEIGHT, H_SPACING, V_SPACING)
      break
    case 'force':
      computeForceLayout(doc, nodes, NODE_WIDTH, NODE_HEIGHT)
      break
    case 'radial':
      computeRadialLayout(doc, nodes, NODE_WIDTH, NODE_HEIGHT)
      break
    default:
      computeHierarchicalLayout(doc, nodes, NODE_WIDTH, NODE_HEIGHT, H_SPACING, V_SPACING)
  }

  // Mark selected node
  if (selectedId && nodes.has(selectedId)) {
    const node = nodes.get(selectedId)!
    nodes.set(selectedId, { ...node, isSelected: true })
  }

  // Create edges for parent-child relationships
  for (const [blockId, block] of doc.blocks) {
    for (const childId of block.children) {
      const sourceNode = nodes.get(blockId)
      const targetNode = nodes.get(childId)
      if (sourceNode && targetNode) {
        edges.push({
          id: `${blockId}->${childId}`,
          sourceId: blockId,
          targetId: childId,
          edgeType: 'parent_child',
          points: [
            { x: sourceNode.x, y: sourceNode.y + sourceNode.height / 2 },
            { x: targetNode.x, y: targetNode.y - targetNode.height / 2 },
          ],
          isHighlighted: false,
        })
      }
    }

    // Create edges for explicit relationships
    for (const edge of block.edges) {
      const sourceNode = nodes.get(blockId)
      const targetNode = nodes.get(edge.target)
      if (sourceNode && targetNode) {
        edges.push({
          id: `${blockId}-${edge.edgeType}->${edge.target}`,
          sourceId: blockId,
          targetId: edge.target,
          edgeType: edge.edgeType,
          points: [
            { x: sourceNode.x + sourceNode.width / 2, y: sourceNode.y },
            { x: targetNode.x - targetNode.width / 2, y: targetNode.y },
          ],
          isHighlighted: false,
        })
      }
    }
  }

  return { nodes, edges }
}

function computeHierarchicalLayout(
  doc: Document,
  nodes: Map<BlockId, GraphNode>,
  nodeWidth: number,
  nodeHeight: number,
  hSpacing: number,
  vSpacing: number
): void {
  // Compute depth of each node
  const depths = new Map<BlockId, number>()
  const childrenCounts = new Map<number, number>()

  function computeDepth(blockId: BlockId, depth: number): void {
    depths.set(blockId, depth)
    childrenCounts.set(depth, (childrenCounts.get(depth) ?? 0) + 1)

    const block = doc.blocks.get(blockId)
    if (block) {
      for (const childId of block.children) {
        computeDepth(childId, depth + 1)
      }
    }
  }

  computeDepth(doc.root, 0)

  // Position nodes
  const xPositions = new Map<number, number>()

  for (const [blockId, depth] of depths) {
    const block = doc.blocks.get(blockId)
    if (!block) continue

    const xPos = xPositions.get(depth) ?? 0
    const totalAtDepth = childrenCounts.get(depth) ?? 1
    const x = xPos - ((totalAtDepth - 1) * (nodeWidth + hSpacing)) / 2

    nodes.set(blockId, {
      id: blockId,
      x,
      y: depth * (nodeHeight + vSpacing),
      width: nodeWidth,
      height: nodeHeight,
      block,
      depth,
      isExpanded: true,
      isSelected: false,
      isHighlighted: false,
    })

    xPositions.set(depth, xPos + nodeWidth + hSpacing)
  }
}

function computeForceLayout(
  doc: Document,
  nodes: Map<BlockId, GraphNode>,
  nodeWidth: number,
  nodeHeight: number
): void {
  // Simple force-directed layout simulation
  const positions = new Map<BlockId, { x: number; y: number; vx: number; vy: number }>()

  // Initialize with random positions
  let i = 0
  for (const blockId of doc.blocks.keys()) {
    const angle = (i / doc.blocks.size) * 2 * Math.PI
    const radius = 150
    positions.set(blockId, {
      x: Math.cos(angle) * radius,
      y: Math.sin(angle) * radius,
      vx: 0,
      vy: 0,
    })
    i++
  }

  // Simple force simulation (reduced iterations for performance)
  for (let iter = 0; iter < 50; iter++) {
    // Repulsion between all nodes
    for (const [id1, pos1] of positions) {
      for (const [id2, pos2] of positions) {
        if (id1 >= id2) continue
        const dx = pos2.x - pos1.x
        const dy = pos2.y - pos1.y
        const dist = Math.max(1, Math.sqrt(dx * dx + dy * dy))
        const force = 1000 / (dist * dist)
        const fx = (dx / dist) * force
        const fy = (dy / dist) * force
        pos1.vx -= fx
        pos1.vy -= fy
        pos2.vx += fx
        pos2.vy += fy
      }
    }

    // Attraction along edges (parent-child)
    for (const [blockId, block] of doc.blocks) {
      const pos1 = positions.get(blockId)
      if (!pos1) continue
      for (const childId of block.children) {
        const pos2 = positions.get(childId)
        if (!pos2) continue
        const dx = pos2.x - pos1.x
        const dy = pos2.y - pos1.y
        const dist = Math.max(1, Math.sqrt(dx * dx + dy * dy))
        const force = dist * 0.01
        const fx = (dx / dist) * force
        const fy = (dy / dist) * force
        pos1.vx += fx
        pos1.vy += fy
        pos2.vx -= fx
        pos2.vy -= fy
      }
    }

    // Apply velocities with damping
    for (const pos of positions.values()) {
      pos.x += pos.vx * 0.1
      pos.y += pos.vy * 0.1
      pos.vx *= 0.9
      pos.vy *= 0.9
    }
  }

  // Create nodes
  for (const [blockId, pos] of positions) {
    const block = doc.blocks.get(blockId)
    if (!block) continue

    nodes.set(blockId, {
      id: blockId,
      x: pos.x,
      y: pos.y,
      width: nodeWidth,
      height: nodeHeight,
      block,
      depth: 0,
      isExpanded: true,
      isSelected: false,
      isHighlighted: false,
    })
  }
}

function computeRadialLayout(
  doc: Document,
  nodes: Map<BlockId, GraphNode>,
  nodeWidth: number,
  nodeHeight: number
): void {
  // Radial layout with root at center
  const depths = new Map<BlockId, number>()

  function computeDepth(blockId: BlockId, depth: number): void {
    depths.set(blockId, depth)
    const block = doc.blocks.get(blockId)
    if (block) {
      for (const childId of block.children) {
        computeDepth(childId, depth + 1)
      }
    }
  }

  computeDepth(doc.root, 0)

  // Group by depth
  const byDepth = new Map<number, BlockId[]>()
  for (const [blockId, depth] of depths) {
    if (!byDepth.has(depth)) {
      byDepth.set(depth, [])
    }
    byDepth.get(depth)!.push(blockId)
  }

  // Position nodes in concentric circles
  const radiusStep = 100

  for (const [depth, blockIds] of byDepth) {
    const radius = depth * radiusStep
    const angleStep = (2 * Math.PI) / blockIds.length

    blockIds.forEach((blockId, index) => {
      const block = doc.blocks.get(blockId)
      if (!block) return

      const angle = index * angleStep - Math.PI / 2
      nodes.set(blockId, {
        id: blockId,
        x: Math.cos(angle) * radius,
        y: Math.sin(angle) * radius,
        width: nodeWidth,
        height: nodeHeight,
        block,
        depth,
        isExpanded: true,
        isSelected: false,
        isHighlighted: false,
      })
    })
  }
}

// =============================================================================
// DRAWING FUNCTIONS
// =============================================================================

function drawNode(
  ctx: CanvasRenderingContext2D,
  node: GraphNode,
  isSelected: boolean,
  isHovered: boolean
): void {
  const { x, y, width, height, block } = node

  // Background
  ctx.fillStyle = isSelected ? '#e3f2fd' : isHovered ? '#f5f5f5' : '#fff'
  ctx.strokeStyle = isSelected ? '#2196f3' : '#ddd'
  ctx.lineWidth = isSelected ? 2 : 1

  // Rounded rectangle
  const radius = 6
  ctx.beginPath()
  ctx.moveTo(x - width / 2 + radius, y - height / 2)
  ctx.lineTo(x + width / 2 - radius, y - height / 2)
  ctx.quadraticCurveTo(x + width / 2, y - height / 2, x + width / 2, y - height / 2 + radius)
  ctx.lineTo(x + width / 2, y + height / 2 - radius)
  ctx.quadraticCurveTo(x + width / 2, y + height / 2, x + width / 2 - radius, y + height / 2)
  ctx.lineTo(x - width / 2 + radius, y + height / 2)
  ctx.quadraticCurveTo(x - width / 2, y + height / 2, x - width / 2, y + height / 2 - radius)
  ctx.lineTo(x - width / 2, y - height / 2 + radius)
  ctx.quadraticCurveTo(x - width / 2, y - height / 2, x - width / 2 + radius, y - height / 2)
  ctx.closePath()
  ctx.fill()
  ctx.stroke()

  // Type indicator
  const typeColor = getTypeColor(block.type)
  ctx.fillStyle = typeColor
  ctx.fillRect(x - width / 2, y - height / 2, 4, height)

  // Label
  ctx.fillStyle = '#333'
  ctx.font = '11px -apple-system, BlinkMacSystemFont, sans-serif'
  ctx.textAlign = 'center'
  ctx.textBaseline = 'middle'

  const label = block.content.slice(0, 15) || block.type
  ctx.fillText(label + (block.content.length > 15 ? '...' : ''), x, y)
}

function drawEdge(
  ctx: CanvasRenderingContext2D,
  edge: GraphEdge,
  nodes: Map<BlockId, GraphNode>,
  hoveredNodeId: BlockId | null
): void {
  const source = nodes.get(edge.sourceId)
  const target = nodes.get(edge.targetId)
  if (!source || !target) return

  const isHighlighted = edge.sourceId === hoveredNodeId || edge.targetId === hoveredNodeId
  const color = getEdgeColor(edge.edgeType)

  ctx.strokeStyle = isHighlighted ? color : `${color}80`
  ctx.lineWidth = isHighlighted ? 2 : 1

  // Draw curved line
  ctx.beginPath()
  ctx.moveTo(edge.points[0]!.x, edge.points[0]!.y)

  if (edge.points.length === 2) {
    // Simple bezier curve
    const midY = (edge.points[0]!.y + edge.points[1]!.y) / 2
    ctx.bezierCurveTo(
      edge.points[0]!.x,
      midY,
      edge.points[1]!.x,
      midY,
      edge.points[1]!.x,
      edge.points[1]!.y
    )
  } else {
    for (let i = 1; i < edge.points.length; i++) {
      ctx.lineTo(edge.points[i]!.x, edge.points[i]!.y)
    }
  }

  ctx.stroke()

  // Draw arrow
  const lastPoint = edge.points[edge.points.length - 1]!
  const prevPoint = edge.points[edge.points.length - 2] ?? edge.points[0]!
  const angle = Math.atan2(lastPoint.y - prevPoint.y, lastPoint.x - prevPoint.x)
  const arrowSize = 6

  ctx.fillStyle = isHighlighted ? color : `${color}80`
  ctx.beginPath()
  ctx.moveTo(lastPoint.x, lastPoint.y)
  ctx.lineTo(
    lastPoint.x - arrowSize * Math.cos(angle - Math.PI / 6),
    lastPoint.y - arrowSize * Math.sin(angle - Math.PI / 6)
  )
  ctx.lineTo(
    lastPoint.x - arrowSize * Math.cos(angle + Math.PI / 6),
    lastPoint.y - arrowSize * Math.sin(angle + Math.PI / 6)
  )
  ctx.closePath()
  ctx.fill()
}

function getTypeColor(type: string): string {
  const colors: Record<string, string> = {
    text: '#4caf50',
    code: '#ff9800',
    table: '#2196f3',
    math: '#9c27b0',
    json: '#795548',
    media: '#e91e63',
    binary: '#607d8b',
    composite: '#00bcd4',
  }
  return colors[type] ?? '#9e9e9e'
}

function getEdgeColor(type: string): string {
  return EDGE_COLORS[type] ?? '#9e9e9e'
}

export default GraphView
