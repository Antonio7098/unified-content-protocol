/**
 * BlockRenderer - Renders a block based on its type and role.
 *
 * Handles selection, editing, drag-and-drop, and metadata display.
 */

import React, { useCallback, useState, useRef } from 'react'
import type { Document } from 'ucp-content'

// Type alias for BlockId since it's not exported
type BlockId = string
import type { EditorStoreInstance } from '../core/EditorStore.js'
import { useEditorState } from '../hooks/useEditor.js'
import { MetadataTooltip } from './MetadataTooltip.js'
import { BlockEditor } from './BlockEditor.js'

// =============================================================================
// STYLES
// =============================================================================

const styles = {
  blockWrapper: {
    position: 'relative' as const,
    marginBottom: '4px',
  },
  block: {
    position: 'relative' as const,
    padding: '4px 8px',
    borderRadius: '4px',
    cursor: 'pointer',
    transition: 'background-color 0.1s, box-shadow 0.1s',
    minHeight: '24px',
  },
  blockHover: {
    backgroundColor: '#f5f5f5',
  },
  blockSelected: {
    backgroundColor: '#e3f2fd',
    boxShadow: '0 0 0 2px #2196f3',
  },
  blockEditing: {
    backgroundColor: '#fff',
    boxShadow: '0 0 0 2px #4caf50',
  },
  blockDragTarget: {
    backgroundColor: '#e8f5e9',
  },
  dragHandle: {
    position: 'absolute' as const,
    left: '-24px',
    top: '50%',
    transform: 'translateY(-50%)',
    width: '16px',
    height: '16px',
    cursor: 'grab',
    opacity: 0,
    transition: 'opacity 0.1s',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    color: '#999',
    fontSize: '10px',
  },
  dragHandleVisible: {
    opacity: 1,
  },
  dropIndicator: {
    position: 'absolute' as const,
    left: 0,
    right: 0,
    height: '2px',
    backgroundColor: '#2196f3',
    pointerEvents: 'none' as const,
  },
  dropIndicatorBefore: {
    top: '-2px',
  },
  dropIndicatorAfter: {
    bottom: '-2px',
  },
  dropIndicatorInside: {
    top: 0,
    bottom: 0,
    left: 0,
    right: 0,
    height: 'auto',
    border: '2px dashed #2196f3',
    backgroundColor: 'rgba(33, 150, 243, 0.1)',
  },
  children: {
    paddingLeft: '24px',
    borderLeft: '1px solid #e5e5e5',
    marginLeft: '8px',
  },
  addButton: {
    padding: '2px 8px',
    border: 'none',
    backgroundColor: 'transparent',
    color: '#999',
    cursor: 'pointer',
    fontSize: '12px',
    opacity: 0,
    transition: 'opacity 0.1s',
  },
  addButtonVisible: {
    opacity: 1,
  },
  // Content type styles
  heading1: {
    fontSize: '28px',
    fontWeight: 700,
    marginTop: '16px',
    marginBottom: '8px',
  },
  heading2: {
    fontSize: '24px',
    fontWeight: 600,
    marginTop: '14px',
    marginBottom: '6px',
  },
  heading3: {
    fontSize: '20px',
    fontWeight: 600,
    marginTop: '12px',
    marginBottom: '4px',
  },
  heading4: {
    fontSize: '18px',
    fontWeight: 600,
    marginTop: '10px',
    marginBottom: '4px',
  },
  heading5: {
    fontSize: '16px',
    fontWeight: 600,
    marginTop: '8px',
    marginBottom: '4px',
  },
  heading6: {
    fontSize: '14px',
    fontWeight: 600,
    marginTop: '8px',
    marginBottom: '4px',
  },
  paragraph: {
    fontSize: '14px',
    lineHeight: '1.6',
  },
  code: {
    fontFamily: 'Monaco, Consolas, "Courier New", monospace',
    fontSize: '13px',
    backgroundColor: '#f5f5f5',
    padding: '12px',
    borderRadius: '4px',
    overflow: 'auto',
    whiteSpace: 'pre-wrap' as const,
  },
  quote: {
    borderLeft: '3px solid #ddd',
    paddingLeft: '12px',
    color: '#666',
    fontStyle: 'italic',
  },
  table: {
    borderCollapse: 'collapse' as const,
    width: '100%',
    fontSize: '13px',
  },
}

// =============================================================================
// BLOCK RENDERER PROPS
// =============================================================================

export interface BlockRendererProps {
  block: Block
  document: Document
  store: EditorStoreInstance
  depth: number
  path: BlockId[]
}

// =============================================================================
// BLOCK RENDERER COMPONENT
// =============================================================================

/**
 * Renders a single block with its children.
 */
export function BlockRenderer({
  block,
  document,
  store,
  depth,
  path,
}: BlockRendererProps): React.ReactElement {
  const [isHovered, setIsHovered] = useState(false)
  const [showTooltip, setShowTooltip] = useState(false)
  const blockRef = useRef<HTMLDivElement>(null)

  // Get state from store
  const selection = useEditorState(store, (s) => s.selection)
  const editingBlockId = useEditorState(store, (s) => s.editingBlockId)
  const drag = useEditorState(store, (s) => s.drag)

  const isSelected =
    selection.type === 'block'
      ? selection.blocks?.blockIds.includes(block.id) ?? false
      : selection.focusedBlockId === block.id

  const isEditing = editingBlockId === block.id
  const isDragSource = drag.sourceId === block.id
  const isDragTarget = drag.targetId === block.id

  // Event handlers
  const handleClick = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation()
      if (!isEditing) {
        if (e.shiftKey && selection.focusedBlockId) {
          // Extend selection
          const currentIds = selection.blocks?.blockIds ?? []
          if (!currentIds.includes(block.id)) {
            store.selectMultiple([...currentIds, block.id])
          }
        } else {
          store.select(block.id)
        }
      }
    },
    [store, block.id, isEditing, selection]
  )

  const handleDoubleClick = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation()
      if (!isEditing) {
        store.startEditing(block.id)
      }
    },
    [store, block.id, isEditing]
  )

  const handleMouseEnter = useCallback(() => {
    setIsHovered(true)
  }, [])

  const handleMouseLeave = useCallback(() => {
    setIsHovered(false)
    setShowTooltip(false)
  }, [])

  // Drag handlers
  const handleDragStart = useCallback(
    (e: React.DragEvent) => {
      e.dataTransfer.effectAllowed = 'move'
      e.dataTransfer.setData('text/plain', block.id)
      store.startDrag(block.id)
    },
    [store, block.id]
  )

  const handleDragOver = useCallback(
    (e: React.DragEvent) => {
      if (!drag.isDragging || drag.sourceId === block.id) return

      e.preventDefault()
      e.dataTransfer.dropEffect = 'move'

      // Determine drop position based on mouse position
      const rect = blockRef.current?.getBoundingClientRect()
      if (!rect) return

      const relativeY = e.clientY - rect.top
      const height = rect.height

      let position: 'before' | 'after' | 'inside'
      if (relativeY < height * 0.25) {
        position = 'before'
      } else if (relativeY > height * 0.75) {
        position = 'after'
      } else {
        position = 'inside'
      }

      store.updateDragTarget(block.id, position)
    },
    [store, block.id, drag.isDragging, drag.sourceId]
  )

  const handleDragLeave = useCallback(() => {
    // Only clear if leaving this specific block
  }, [])

  const handleDrop = useCallback(
    (e: React.DragEvent) => {
      e.preventDefault()
      if (drag.isDragging) {
        store.endDrag(true)
      }
    },
    [store, drag.isDragging]
  )

  const handleDragEnd = useCallback(() => {
    store.endDrag(false)
  }, [store])

  // Add child handler
  const handleAddChild = useCallback(() => {
    const newId = store.addBlock(block.id, '')
    store.startEditing(newId)
  }, [store, block.id])

  // Tooltip toggle
  const handleTooltipToggle = useCallback(() => {
    setShowTooltip((prev) => !prev)
  }, [])

  // Get block style based on role
  const getBlockStyle = () => {
    const baseStyle = { ...styles.block }

    if (isHovered && !isEditing) {
      Object.assign(baseStyle, styles.blockHover)
    }
    if (isSelected && !isEditing) {
      Object.assign(baseStyle, styles.blockSelected)
    }
    if (isEditing) {
      Object.assign(baseStyle, styles.blockEditing)
    }
    if (isDragTarget && !isDragSource) {
      Object.assign(baseStyle, styles.blockDragTarget)
    }

    return baseStyle
  }

  // Render content based on type and role
  const renderContent = () => {
    if (isEditing) {
      return <BlockEditor block={block} store={store} />
    }

    return <BlockContent block={block} />
  }

  // Render drop indicator
  const renderDropIndicator = () => {
    if (!isDragTarget || isDragSource) return null

    switch (drag.position) {
      case 'before':
        return (
          <div
            style={{ ...styles.dropIndicator, ...styles.dropIndicatorBefore }}
          />
        )
      case 'after':
        return (
          <div
            style={{ ...styles.dropIndicator, ...styles.dropIndicatorAfter }}
          />
        )
      case 'inside':
        return <div style={{ ...styles.dropIndicator, ...styles.dropIndicatorInside }} />
      default:
        return null
    }
  }

  return (
    <div
      style={styles.blockWrapper}
      data-block-id={block.id}
      data-testid={`block-${block.id}`}
    >
      <div
        ref={blockRef}
        style={getBlockStyle()}
        onClick={handleClick}
        onDoubleClick={handleDoubleClick}
        onMouseEnter={handleMouseEnter}
        onMouseLeave={handleMouseLeave}
        draggable={!isEditing && store.config.enableDragDrop}
        onDragStart={handleDragStart}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
        onDragEnd={handleDragEnd}
      >
        {/* Drag handle */}
        <div
          style={{
            ...styles.dragHandle,
            ...(isHovered ? styles.dragHandleVisible : {}),
          }}
          onMouseDown={(e) => e.stopPropagation()}
        >
          ⋮⋮
        </div>

        {/* Content */}
        {renderContent()}

        {/* Drop indicator */}
        {renderDropIndicator()}

        {/* Metadata tooltip trigger */}
        {isHovered && !isEditing && (
          <button
            style={{
              position: 'absolute',
              right: '4px',
              top: '4px',
              padding: '2px 6px',
              fontSize: '10px',
              border: '1px solid #ddd',
              borderRadius: '2px',
              backgroundColor: '#fff',
              cursor: 'pointer',
            }}
            onClick={(e) => {
              e.stopPropagation()
              handleTooltipToggle()
            }}
            title="Show metadata"
          >
            i
          </button>
        )}

        {/* Metadata tooltip */}
        {showTooltip && (
          <MetadataTooltip block={block} onClose={() => setShowTooltip(false)} />
        )}
      </div>

      {/* Children */}
      {block.children.length > 0 && (
        <div style={styles.children}>
          {block.children.map((childId) => {
            const child = document.blocks.get(childId)
            if (!child) return null
            return (
              <BlockRenderer
                key={childId}
                block={child}
                document={document}
                store={store}
                depth={depth + 1}
                path={[...path, block.id]}
              />
            )
          })}
        </div>
      )}

      {/* Add child button */}
      {isHovered && !isEditing && (
        <button
          style={{
            ...styles.addButton,
            ...(isHovered ? styles.addButtonVisible : {}),
          }}
          onClick={(e) => {
            e.stopPropagation()
            handleAddChild()
          }}
        >
          + Add block
        </button>
      )}
    </div>
  )
}

// =============================================================================
// BLOCK CONTENT
// =============================================================================

interface BlockContentProps {
  block: Block
}

function BlockContent({ block }: BlockContentProps): React.ReactElement {
  const role = block.role ?? block.metadata?.semanticRole
  const type = block.type

  // Get style based on role
  const getContentStyle = (): React.CSSProperties => {
    switch (role) {
      case 'heading1':
        return styles.heading1
      case 'heading2':
        return styles.heading2
      case 'heading3':
        return styles.heading3
      case 'heading4':
        return styles.heading4
      case 'heading5':
        return styles.heading5
      case 'heading6':
        return styles.heading6
      case 'code':
        return styles.code
      case 'quote':
        return styles.quote
      default:
        return styles.paragraph
    }
  }

  // Render based on type
  switch (type) {
    case 'code':
      return (
        <pre style={styles.code}>
          <code>{block.content || '\u00A0'}</code>
        </pre>
      )

    case 'table':
      // Simple table rendering
      return (
        <div style={{ overflow: 'auto' }}>
          <pre style={{ ...styles.code, whiteSpace: 'pre' }}>
            {block.content || 'Empty table'}
          </pre>
        </div>
      )

    case 'math':
      return (
        <div
          style={{
            fontFamily: 'serif',
            fontStyle: 'italic',
            textAlign: 'center',
            padding: '8px',
          }}
        >
          {block.content || 'Empty equation'}
        </div>
      )

    case 'media':
      return (
        <div
          style={{
            padding: '8px',
            backgroundColor: '#f5f5f5',
            borderRadius: '4px',
            textAlign: 'center',
          }}
        >
          [Media: {block.content || 'No source'}]
        </div>
      )

    case 'json':
      return (
        <pre style={styles.code}>
          <code>{formatJson(block.content)}</code>
        </pre>
      )

    default:
      return (
        <div style={getContentStyle()}>
          {block.content || <span style={{ color: '#999' }}>Empty block</span>}
        </div>
      )
  }
}

function formatJson(content: string): string {
  try {
    return JSON.stringify(JSON.parse(content), null, 2)
  } catch {
    return content
  }
}

export default BlockRenderer
