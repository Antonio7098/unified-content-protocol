/**
 * BlockEditor - Inline editing component for blocks.
 *
 * Provides a Notion-like editing experience with auto-growing textarea
 * and keyboard shortcuts.
 */

import React, { useCallback, useRef, useEffect, useState } from 'react'
import type { Block, ContentType } from 'ucp-content'
import type { EditorStoreInstance } from '../core/EditorStore.js'
import { useEditorState } from '../hooks/useEditor.js'

// =============================================================================
// STYLES
// =============================================================================

const styles = {
  container: {
    position: 'relative' as const,
  },
  textarea: {
    width: '100%',
    minHeight: '24px',
    padding: '0',
    margin: '0',
    border: 'none',
    outline: 'none',
    resize: 'none' as const,
    backgroundColor: 'transparent',
    fontFamily: 'inherit',
    fontSize: 'inherit',
    fontWeight: 'inherit',
    lineHeight: 'inherit',
    color: 'inherit',
    overflow: 'hidden',
  },
  codeTextarea: {
    fontFamily: 'Monaco, Consolas, "Courier New", monospace',
    fontSize: '13px',
    backgroundColor: '#f5f5f5',
    padding: '12px',
    borderRadius: '4px',
    whiteSpace: 'pre' as const,
  },
  toolbar: {
    display: 'flex',
    alignItems: 'center',
    gap: '4px',
    padding: '4px 0',
    marginBottom: '4px',
    borderBottom: '1px solid #e5e5e5',
  },
  toolbarButton: {
    padding: '2px 6px',
    border: '1px solid #ddd',
    borderRadius: '2px',
    backgroundColor: '#fff',
    cursor: 'pointer',
    fontSize: '11px',
    color: '#666',
    transition: 'background-color 0.1s',
  },
  toolbarButtonHover: {
    backgroundColor: '#f5f5f5',
  },
  typeSelector: {
    padding: '2px 4px',
    border: '1px solid #ddd',
    borderRadius: '2px',
    backgroundColor: '#fff',
    fontSize: '11px',
    cursor: 'pointer',
  },
  hint: {
    fontSize: '10px',
    color: '#999',
    marginTop: '4px',
  },
}

// =============================================================================
// BLOCK EDITOR PROPS
// =============================================================================

export interface BlockEditorProps {
  block: Block
  store: EditorStoreInstance
}

// =============================================================================
// BLOCK EDITOR COMPONENT
// =============================================================================

/**
 * Inline editor for a single block.
 */
export function BlockEditor({ block, store }: BlockEditorProps): React.ReactElement {
  const textareaRef = useRef<HTMLTextAreaElement>(null)
  const [showToolbar, setShowToolbar] = useState(false)

  const pendingContent = useEditorState(store, (s) => s.pendingContent)
  const content = pendingContent ?? block.content

  // Focus and select on mount
  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.focus()
      // Place cursor at end
      textareaRef.current.setSelectionRange(
        textareaRef.current.value.length,
        textareaRef.current.value.length
      )
    }
  }, [])

  // Auto-resize textarea
  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = 'auto'
      textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`
    }
  }, [content])

  // Handle content change
  const handleChange = useCallback(
    (e: React.ChangeEvent<HTMLTextAreaElement>) => {
      store.updatePendingContent(e.target.value)
    },
    [store]
  )

  // Handle keyboard shortcuts
  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
      // Save on Cmd/Ctrl+Enter
      if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') {
        e.preventDefault()
        store.stopEditing(true)
        return
      }

      // Cancel on Escape
      if (e.key === 'Escape') {
        e.preventDefault()
        store.stopEditing(false)
        return
      }

      // Add new block on Enter (when at end and not Shift)
      if (e.key === 'Enter' && !e.shiftKey && !e.metaKey && !e.ctrlKey) {
        const textarea = textareaRef.current
        if (textarea && textarea.selectionStart === textarea.value.length) {
          // At end of content, create new sibling block
          e.preventDefault()
          store.stopEditing(true)

          // Find parent and add sibling
          const parent = findParent(store, block.id)
          if (parent) {
            const newId = store.addBlock(parent, '')
            store.startEditing(newId)
          }
          return
        }
      }

      // Tab for indentation in code blocks
      if (e.key === 'Tab' && block.type === 'code') {
        e.preventDefault()
        const textarea = textareaRef.current
        if (textarea) {
          const start = textarea.selectionStart
          const end = textarea.selectionEnd
          const value = textarea.value
          const newValue = value.substring(0, start) + '  ' + value.substring(end)
          store.updatePendingContent(newValue)

          // Restore cursor position
          setTimeout(() => {
            textarea.selectionStart = textarea.selectionEnd = start + 2
          }, 0)
        }
        return
      }

      // Stop propagation for all other keys to prevent global shortcuts
      e.stopPropagation()
    },
    [store, block.id, block.type]
  )

  // Handle blur (save on focus loss)
  const handleBlur = useCallback(() => {
    // Small delay to allow click on toolbar buttons
    setTimeout(() => {
      if (document.activeElement?.closest('[data-editor-toolbar]')) {
        return
      }
      store.stopEditing(true)
    }, 100)
  }, [store])

  // Handle type change
  const handleTypeChange = useCallback(
    (e: React.ChangeEvent<HTMLSelectElement>) => {
      store.changeBlockType(block.id, e.target.value as ContentType)
    },
    [store, block.id]
  )

  // Determine textarea style
  const getTextareaStyle = (): React.CSSProperties => {
    const baseStyle = { ...styles.textarea }

    if (block.type === 'code') {
      Object.assign(baseStyle, styles.codeTextarea)
    }

    // Apply role-based styling
    const role = block.role ?? block.metadata?.semanticRole
    switch (role) {
      case 'heading1':
        baseStyle.fontSize = '28px'
        baseStyle.fontWeight = '700'
        break
      case 'heading2':
        baseStyle.fontSize = '24px'
        baseStyle.fontWeight = '600'
        break
      case 'heading3':
        baseStyle.fontSize = '20px'
        baseStyle.fontWeight = '600'
        break
      case 'heading4':
        baseStyle.fontSize = '18px'
        baseStyle.fontWeight = '600'
        break
      case 'heading5':
        baseStyle.fontSize = '16px'
        baseStyle.fontWeight = '600'
        break
      case 'heading6':
        baseStyle.fontSize = '14px'
        baseStyle.fontWeight = '600'
        break
    }

    return baseStyle
  }

  return (
    <div
      style={styles.container}
      onMouseEnter={() => setShowToolbar(true)}
      onMouseLeave={() => setShowToolbar(false)}
    >
      {/* Toolbar */}
      {showToolbar && (
        <div style={styles.toolbar} data-editor-toolbar>
          <select
            value={block.type}
            onChange={handleTypeChange}
            style={styles.typeSelector}
            onClick={(e) => e.stopPropagation()}
          >
            <option value="text">Text</option>
            <option value="code">Code</option>
            <option value="table">Table</option>
            <option value="math">Math</option>
            <option value="json">JSON</option>
            <option value="media">Media</option>
          </select>

          <button
            style={styles.toolbarButton}
            onClick={(e) => {
              e.stopPropagation()
              store.deleteBlock(block.id)
            }}
            title="Delete block"
          >
            Delete
          </button>
        </div>
      )}

      {/* Textarea */}
      <textarea
        ref={textareaRef}
        value={content}
        onChange={handleChange}
        onKeyDown={handleKeyDown}
        onBlur={handleBlur}
        style={getTextareaStyle()}
        placeholder="Type something..."
        rows={1}
        spellCheck={block.type !== 'code'}
        data-testid={`block-editor-${block.id}`}
      />

      {/* Hint */}
      <div style={styles.hint}>
        Press <kbd>Esc</kbd> to cancel, <kbd>Cmd+Enter</kbd> to save
      </div>
    </div>
  )
}

// =============================================================================
// HELPERS
// =============================================================================

function findParent(store: EditorStoreInstance, blockId: string): string | undefined {
  const document = store.document
  if (!document) return undefined

  for (const [id, block] of document.blocks) {
    if (block.children.includes(blockId)) {
      return id
    }
  }
  return undefined
}

export default BlockEditor
