/**
 * Editor - Main UCM Editor component.
 *
 * Provides the top-level editor container with view switching
 * and keyboard shortcut handling.
 */

import React, { useEffect, useCallback } from 'react'
import type { Document } from 'ucp-js'
import type { EditorStoreInstance } from '../core/EditorStore.js'
import type { EditorConfig, EditorView } from '../types/editor.js'
import {
  useEditorStore,
  useEditorState,
  useKeyboardShortcuts,
} from '../hooks/useEditor.js'
import { BlockRenderer } from './BlockRenderer.js'
import { GraphView } from '../graph/GraphView.js'
import { DiffViewer } from '../diff/DiffViewer.js'

// =============================================================================
// STYLES
// =============================================================================

const styles = {
  container: {
    display: 'flex',
    flexDirection: 'column' as const,
    height: '100%',
    width: '100%',
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
    fontSize: '14px',
    lineHeight: '1.5',
    color: '#1a1a1a',
    backgroundColor: '#ffffff',
  },
  toolbar: {
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    padding: '8px 16px',
    borderBottom: '1px solid #e5e5e5',
    backgroundColor: '#fafafa',
  },
  toolbarGroup: {
    display: 'flex',
    alignItems: 'center',
    gap: '4px',
  },
  toolbarButton: {
    padding: '4px 8px',
    border: '1px solid #d1d1d1',
    borderRadius: '4px',
    backgroundColor: '#ffffff',
    cursor: 'pointer',
    fontSize: '12px',
    transition: 'background-color 0.1s',
  },
  toolbarButtonActive: {
    backgroundColor: '#e3f2fd',
    borderColor: '#2196f3',
  },
  toolbarButtonDisabled: {
    opacity: 0.5,
    cursor: 'not-allowed',
  },
  toolbarDivider: {
    width: '1px',
    height: '20px',
    backgroundColor: '#e5e5e5',
    margin: '0 8px',
  },
  content: {
    flex: 1,
    overflow: 'auto',
    padding: '24px',
  },
  documentView: {
    maxWidth: '800px',
    margin: '0 auto',
  },
  splitView: {
    display: 'grid',
    gridTemplateColumns: '1fr 1fr',
    gap: '16px',
    height: '100%',
  },
  splitPane: {
    overflow: 'auto',
    border: '1px solid #e5e5e5',
    borderRadius: '4px',
  },
  emptyState: {
    display: 'flex',
    flexDirection: 'column' as const,
    alignItems: 'center',
    justifyContent: 'center',
    height: '200px',
    color: '#666',
  },
  statusBar: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    padding: '4px 16px',
    borderTop: '1px solid #e5e5e5',
    backgroundColor: '#fafafa',
    fontSize: '12px',
    color: '#666',
  },
}

// =============================================================================
// EDITOR PROPS
// =============================================================================

export interface EditorProps {
  /** Initial document to load */
  document?: Document
  /** Editor configuration */
  config?: Partial<EditorConfig>
  /** Custom store instance (for external state management) */
  store?: EditorStoreInstance
  /** Callback when document changes */
  onChange?: (document: Document) => void
  /** Callback when document is saved */
  onSave?: (document: Document) => Promise<void>
  /** Additional class name */
  className?: string
  /** Additional styles */
  style?: React.CSSProperties
}

// =============================================================================
// EDITOR COMPONENT
// =============================================================================

/**
 * Main UCM Editor component.
 *
 * @example
 * ```tsx
 * import { Editor } from 'ucm-editor'
 * import { parseMarkdown } from 'ucp-js'
 *
 * function App() {
 *   const doc = parseMarkdown('# Hello\n\nWorld')
 *
 *   return (
 *     <Editor
 *       document={doc}
 *       onChange={(doc) => console.log('Changed:', doc)}
 *     />
 *   )
 * }
 * ```
 */
export function Editor({
  document: initialDocument,
  config,
  store: externalStore,
  onChange,
  onSave,
  className,
  style,
}: EditorProps): React.ReactElement {
  // Use provided store or create a new one
  const internalStore = useEditorStore(config)
  const store = externalStore ?? internalStore

  // Subscribe to state
  const document = useEditorState(store, (s) => s.document)
  const view = useEditorState(store, (s) => s.view)
  const isDirty = useEditorState(store, (s) => s.isDirty)
  const history = useEditorState(store, (s) => s.history)
  const lastError = useEditorState(store, (s) => s.lastError)

  // Enable keyboard shortcuts
  useKeyboardShortcuts(store, store.config.enableKeyboardShortcuts)

  // Load initial document
  useEffect(() => {
    if (initialDocument && !document) {
      store.loadDocument(initialDocument)
    }
  }, [initialDocument, document, store])

  // Notify on changes
  useEffect(() => {
    if (document && onChange) {
      onChange(document)
    }
  }, [document, onChange])

  // View handlers
  const handleViewChange = useCallback(
    (newView: EditorView) => {
      store.setView(newView)
    },
    [store]
  )

  // History handlers
  const handleUndo = useCallback(() => {
    if (history.canUndo) {
      store.undo()
    }
  }, [store, history.canUndo])

  const handleRedo = useCallback(() => {
    if (history.canRedo) {
      store.redo()
    }
  }, [store, history.canRedo])

  // Save handler
  const handleSave = useCallback(async () => {
    if (document && onSave) {
      await onSave(document)
    }
    await store.saveDocument()
  }, [store, document, onSave])

  // Render content based on view
  const renderContent = () => {
    if (!document) {
      return (
        <div style={styles.emptyState}>
          <p>No document loaded</p>
          <button
            onClick={() => store.createDocument('New Document')}
            style={{ ...styles.toolbarButton, marginTop: '8px' }}
          >
            Create New Document
          </button>
        </div>
      )
    }

    switch (view) {
      case 'document':
        return (
          <div style={styles.documentView}>
            <DocumentView store={store} />
          </div>
        )

      case 'graph':
        return <GraphView store={store} />

      case 'diff':
        return <DiffViewer store={store} />

      case 'split':
        return (
          <div style={styles.splitView}>
            <div style={styles.splitPane}>
              <div style={{ padding: '16px' }}>
                <DocumentView store={store} />
              </div>
            </div>
            <div style={styles.splitPane}>
              <GraphView store={store} />
            </div>
          </div>
        )

      default:
        return <DocumentView store={store} />
    }
  }

  return (
    <div
      className={className}
      style={{ ...styles.container, ...style }}
      data-testid="ucm-editor"
    >
      {/* Toolbar */}
      <div style={styles.toolbar}>
        {/* View switcher */}
        <div style={styles.toolbarGroup}>
          <ViewButton
            label="Document"
            view="document"
            currentView={view}
            onClick={handleViewChange}
          />
          <ViewButton
            label="Graph"
            view="graph"
            currentView={view}
            onClick={handleViewChange}
          />
          <ViewButton
            label="Diff"
            view="diff"
            currentView={view}
            onClick={handleViewChange}
          />
          <ViewButton
            label="Split"
            view="split"
            currentView={view}
            onClick={handleViewChange}
          />
        </div>

        <div style={styles.toolbarDivider} />

        {/* History */}
        <div style={styles.toolbarGroup}>
          <button
            onClick={handleUndo}
            disabled={!history.canUndo}
            style={{
              ...styles.toolbarButton,
              ...(history.canUndo ? {} : styles.toolbarButtonDisabled),
            }}
            title="Undo (Cmd+Z)"
          >
            Undo
          </button>
          <button
            onClick={handleRedo}
            disabled={!history.canRedo}
            style={{
              ...styles.toolbarButton,
              ...(history.canRedo ? {} : styles.toolbarButtonDisabled),
            }}
            title="Redo (Cmd+Shift+Z)"
          >
            Redo
          </button>
        </div>

        <div style={styles.toolbarDivider} />

        {/* Save */}
        <button
          onClick={handleSave}
          disabled={!isDirty}
          style={{
            ...styles.toolbarButton,
            ...(isDirty ? {} : styles.toolbarButtonDisabled),
          }}
          title="Save (Cmd+S)"
        >
          Save
        </button>
      </div>

      {/* Content */}
      <div style={styles.content}>{renderContent()}</div>

      {/* Status bar */}
      <div style={styles.statusBar}>
        <div>
          {document ? `${document.blocks.size} blocks` : 'No document'}
          {isDirty && ' (unsaved changes)'}
        </div>
        <div>
          {lastError && (
            <span style={{ color: '#d32f2f' }}>
              Error: {lastError.message}
            </span>
          )}
        </div>
      </div>
    </div>
  )
}

// =============================================================================
// VIEW BUTTON
// =============================================================================

interface ViewButtonProps {
  label: string
  view: EditorView
  currentView: EditorView
  onClick: (view: EditorView) => void
}

function ViewButton({ label, view, currentView, onClick }: ViewButtonProps) {
  const isActive = view === currentView
  return (
    <button
      onClick={() => onClick(view)}
      style={{
        ...styles.toolbarButton,
        ...(isActive ? styles.toolbarButtonActive : {}),
      }}
    >
      {label}
    </button>
  )
}

// =============================================================================
// DOCUMENT VIEW
// =============================================================================

interface DocumentViewProps {
  store: EditorStoreInstance
}

function DocumentView({ store }: DocumentViewProps): React.ReactElement {
  const document = useEditorState(store, (s) => s.document)

  if (!document) {
    return <div>No document</div>
  }

  const rootBlock = document.blocks.get(document.root)
  if (!rootBlock) {
    return <div>Invalid document structure</div>
  }

  return (
    <div data-testid="document-view">
      {rootBlock.children.map((childId) => {
        const child = document.blocks.get(childId)
        if (!child) return null
        return (
          <BlockRenderer
            key={childId}
            block={child}
            document={document}
            store={store}
            depth={0}
            path={[document.root]}
          />
        )
      })}

      {rootBlock.children.length === 0 && (
        <div style={styles.emptyState}>
          <p>Empty document</p>
          <button
            onClick={() => store.addBlock(document.root, 'Start typing...')}
            style={{ ...styles.toolbarButton, marginTop: '8px' }}
          >
            Add Block
          </button>
        </div>
      )}
    </div>
  )
}

export default Editor
