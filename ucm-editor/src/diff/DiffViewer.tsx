/**
 * DiffViewer - Displays differences between document snapshots.
 *
 * Shows block-level and content-level diffs with accept/reject actions.
 */

import React, { useCallback, useMemo, useState } from 'react'
import type { Document, ContentType, EdgeType } from 'ucp-content'

// Type alias for BlockId since it's not exported
type BlockId = string
import type { EditorStoreInstance } from '../core/EditorStore.js'
import type { DocumentDiff, BlockDiff, TextDiff, DiffChangeType } from '../types/editor.js'
import { useEditorState } from '../hooks/useEditor.js'

// =============================================================================
// STYLES
// =============================================================================

const styles = {
  container: {
    display: 'flex',
    flexDirection: 'column' as const,
    height: '100%',
    backgroundColor: '#fff',
  },
  header: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    padding: '12px 16px',
    borderBottom: '1px solid #e5e5e5',
    backgroundColor: '#fafafa',
  },
  headerTitle: {
    fontSize: '14px',
    fontWeight: 600,
  },
  headerActions: {
    display: 'flex',
    gap: '8px',
  },
  button: {
    padding: '6px 12px',
    border: '1px solid #ddd',
    borderRadius: '4px',
    backgroundColor: '#fff',
    cursor: 'pointer',
    fontSize: '12px',
  },
  buttonPrimary: {
    backgroundColor: '#4caf50',
    borderColor: '#4caf50',
    color: '#fff',
  },
  buttonDanger: {
    backgroundColor: '#f44336',
    borderColor: '#f44336',
    color: '#fff',
  },
  snapshotSelector: {
    display: 'flex',
    alignItems: 'center',
    gap: '12px',
    padding: '12px 16px',
    borderBottom: '1px solid #e5e5e5',
  },
  snapshotSelect: {
    padding: '6px 8px',
    border: '1px solid #ddd',
    borderRadius: '4px',
    fontSize: '12px',
    minWidth: '150px',
  },
  content: {
    flex: 1,
    overflow: 'auto',
    padding: '16px',
  },
  summary: {
    display: 'flex',
    gap: '16px',
    padding: '12px 16px',
    backgroundColor: '#f5f5f5',
    borderRadius: '4px',
    marginBottom: '16px',
    fontSize: '12px',
  },
  summaryItem: {
    display: 'flex',
    alignItems: 'center',
    gap: '4px',
  },
  summaryBadge: {
    display: 'inline-block',
    padding: '2px 8px',
    borderRadius: '10px',
    fontSize: '11px',
    fontWeight: 500,
  },
  badgeAdded: {
    backgroundColor: '#e8f5e9',
    color: '#2e7d32',
  },
  badgeRemoved: {
    backgroundColor: '#ffebee',
    color: '#c62828',
  },
  badgeModified: {
    backgroundColor: '#fff3e0',
    color: '#ef6c00',
  },
  badgeMoved: {
    backgroundColor: '#e3f2fd',
    color: '#1565c0',
  },
  diffList: {
    display: 'flex',
    flexDirection: 'column' as const,
    gap: '8px',
  },
  diffItem: {
    border: '1px solid #e5e5e5',
    borderRadius: '4px',
    overflow: 'hidden',
  },
  diffItemHeader: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    padding: '8px 12px',
    backgroundColor: '#fafafa',
    borderBottom: '1px solid #e5e5e5',
    cursor: 'pointer',
  },
  diffItemHeaderLeft: {
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
  },
  diffItemId: {
    fontFamily: 'Monaco, Consolas, monospace',
    fontSize: '10px',
    color: '#666',
  },
  diffItemType: {
    fontSize: '11px',
    color: '#333',
  },
  diffItemActions: {
    display: 'flex',
    gap: '4px',
  },
  diffItemContent: {
    padding: '12px',
  },
  diffLine: {
    fontFamily: 'Monaco, Consolas, monospace',
    fontSize: '12px',
    lineHeight: '1.5',
    padding: '2px 4px',
    borderRadius: '2px',
  },
  diffLineAdded: {
    backgroundColor: '#e8f5e9',
    color: '#1b5e20',
  },
  diffLineRemoved: {
    backgroundColor: '#ffebee',
    color: '#b71c1c',
    textDecoration: 'line-through',
  },
  diffLineEqual: {
    color: '#666',
  },
  splitView: {
    display: 'grid',
    gridTemplateColumns: '1fr 1fr',
    gap: '1px',
    backgroundColor: '#e5e5e5',
  },
  splitPane: {
    backgroundColor: '#fff',
    padding: '12px',
  },
  splitPaneTitle: {
    fontSize: '11px',
    fontWeight: 600,
    color: '#666',
    marginBottom: '8px',
    textTransform: 'uppercase' as const,
  },
  emptyState: {
    display: 'flex',
    flexDirection: 'column' as const,
    alignItems: 'center',
    justifyContent: 'center',
    height: '200px',
    color: '#666',
    fontSize: '14px',
  },
  filterBar: {
    display: 'flex',
    alignItems: 'center',
    gap: '12px',
    padding: '8px 16px',
    borderBottom: '1px solid #e5e5e5',
  },
  filterCheckbox: {
    display: 'flex',
    alignItems: 'center',
    gap: '4px',
    fontSize: '12px',
  },
}

// =============================================================================
// DIFF VIEWER PROPS
// =============================================================================

export interface DiffViewerProps {
  store: EditorStoreInstance
}

// =============================================================================
// DIFF VIEWER COMPONENT
// =============================================================================

/**
 * Component for viewing and managing document diffs.
 */
export function DiffViewer({ store }: DiffViewerProps): React.ReactElement {
  const diffState = useEditorState(store, (s) => s.diff)
  const document = useEditorState(store, (s) => s.document)

  const [showUnchanged, setShowUnchanged] = useState(false)
  const [expandedItems, setExpandedItems] = useState<Set<BlockId>>(new Set())
  const [filter, setFilter] = useState<DiffChangeType | 'all'>('all')

  // Mock snapshots for demo (in real implementation, these come from store)
  const snapshots = useMemo(() => {
    return [
      { id: 'current', description: 'Current state' },
      { id: 'initial', description: 'Initial state' },
    ]
  }, [])

  // Compute diff when comparing
  const diff = useMemo((): DocumentDiff | null => {
    if (!diffState.isComparing || !document) {
      return null
    }
    // For demo, create a simple diff with current document
    // In real implementation, compare actual snapshots
    return {
      fromSnapshotId: diffState.leftSnapshotId ?? 'initial',
      toSnapshotId: diffState.rightSnapshotId ?? 'current',
      blockDiffs: new Map(),
      structuralChanges: [],
      summary: { added: 0, removed: 0, modified: 0, moved: 0, unchanged: document.blocks.size },
    }
  }, [diffState.isComparing, diffState.leftSnapshotId, diffState.rightSnapshotId, document])

  // Filter diffs
  const filteredDiffs = useMemo(() => {
    if (!diff) return []

    const diffs = Array.from(diff.blockDiffs.values())

    return diffs.filter((d) => {
      if (!showUnchanged && d.changeType === 'unchanged') return false
      if (filter !== 'all' && d.changeType !== filter) return false
      return true
    })
  }, [diff, showUnchanged, filter])

  // Handlers
  const handleStartCompare = useCallback(() => {
    store.startCompare('initial', 'current')
  }, [store])

  const handleStopCompare = useCallback(() => {
    store.stopCompare()
  }, [store])

  const handleToggleExpand = useCallback((blockId: BlockId) => {
    setExpandedItems((prev) => {
      const next = new Set(prev)
      if (next.has(blockId)) {
        next.delete(blockId)
      } else {
        next.add(blockId)
      }
      return next
    })
  }, [])

  const handleApplyChange = useCallback(
    (blockId: BlockId) => {
      store.applyChange(blockId)
    },
    [store]
  )

  const handleRejectChange = useCallback(
    (blockId: BlockId) => {
      store.rejectChange(blockId)
    },
    [store]
  )

  const handleApplyAll = useCallback(() => {
    filteredDiffs.forEach((d) => {
      if (d.changeType !== 'unchanged') {
        store.applyChange(d.blockId)
      }
    })
  }, [store, filteredDiffs])

  if (!diffState.isComparing) {
    return (
      <div style={styles.container}>
        <div style={styles.header}>
          <span style={styles.headerTitle}>Diff Viewer</span>
        </div>
        <div style={styles.emptyState}>
          <p>Select two snapshots to compare</p>
          <button
            style={{ ...styles.button, marginTop: '12px' }}
            onClick={handleStartCompare}
          >
            Start Comparison
          </button>
        </div>
      </div>
    )
  }

  return (
    <div style={styles.container} data-testid="diff-viewer">
      {/* Header */}
      <div style={styles.header}>
        <span style={styles.headerTitle}>Comparing Changes</span>
        <div style={styles.headerActions}>
          <button
            style={{ ...styles.button, ...styles.buttonPrimary }}
            onClick={handleApplyAll}
          >
            Apply All
          </button>
          <button style={styles.button} onClick={handleStopCompare}>
            Close
          </button>
        </div>
      </div>

      {/* Snapshot selector */}
      <div style={styles.snapshotSelector}>
        <span>From:</span>
        <select style={styles.snapshotSelect} value={diffState.leftSnapshotId ?? ''}>
          {snapshots.map((s) => (
            <option key={s.id} value={s.id}>
              {s.description}
            </option>
          ))}
        </select>
        <span>To:</span>
        <select style={styles.snapshotSelect} value={diffState.rightSnapshotId ?? ''}>
          {snapshots.map((s) => (
            <option key={s.id} value={s.id}>
              {s.description}
            </option>
          ))}
        </select>
      </div>

      {/* Filter bar */}
      <div style={styles.filterBar}>
        <label style={styles.filterCheckbox}>
          <input
            type="checkbox"
            checked={showUnchanged}
            onChange={(e) => setShowUnchanged(e.target.checked)}
          />
          Show unchanged
        </label>
        <select
          value={filter}
          onChange={(e) => setFilter(e.target.value as DiffChangeType | 'all')}
          style={{ ...styles.snapshotSelect, minWidth: '100px' }}
        >
          <option value="all">All changes</option>
          <option value="added">Added</option>
          <option value="removed">Removed</option>
          <option value="modified">Modified</option>
          <option value="moved">Moved</option>
        </select>
      </div>

      {/* Summary */}
      {diff && (
        <div style={styles.content}>
          <div style={styles.summary}>
            <div style={styles.summaryItem}>
              <span style={{ ...styles.summaryBadge, ...styles.badgeAdded }}>
                +{diff.summary.added}
              </span>
              <span>Added</span>
            </div>
            <div style={styles.summaryItem}>
              <span style={{ ...styles.summaryBadge, ...styles.badgeRemoved }}>
                -{diff.summary.removed}
              </span>
              <span>Removed</span>
            </div>
            <div style={styles.summaryItem}>
              <span style={{ ...styles.summaryBadge, ...styles.badgeModified }}>
                ~{diff.summary.modified}
              </span>
              <span>Modified</span>
            </div>
            <div style={styles.summaryItem}>
              <span style={{ ...styles.summaryBadge, ...styles.badgeMoved }}>
                ↔{diff.summary.moved}
              </span>
              <span>Moved</span>
            </div>
          </div>

          {/* Diff list */}
          <div style={styles.diffList}>
            {filteredDiffs.length === 0 ? (
              <div style={styles.emptyState}>
                <p>No changes to display</p>
              </div>
            ) : (
              filteredDiffs.map((blockDiff) => (
                <DiffItem
                  key={blockDiff.blockId}
                  blockDiff={blockDiff}
                  isExpanded={expandedItems.has(blockDiff.blockId)}
                  onToggle={() => handleToggleExpand(blockDiff.blockId)}
                  onApply={() => handleApplyChange(blockDiff.blockId)}
                  onReject={() => handleRejectChange(blockDiff.blockId)}
                />
              ))
            )}
          </div>
        </div>
      )}
    </div>
  )
}

// =============================================================================
// DIFF ITEM COMPONENT
// =============================================================================

interface DiffItemProps {
  blockDiff: BlockDiff
  isExpanded: boolean
  onToggle: () => void
  onApply: () => void
  onReject: () => void
}

function DiffItem({
  blockDiff,
  isExpanded,
  onToggle,
  onApply,
  onReject,
}: DiffItemProps): React.ReactElement {
  const { blockId, changeType, oldBlock, newBlock, contentDiff } = blockDiff

  const getBadgeStyle = () => {
    switch (changeType) {
      case 'added':
        return styles.badgeAdded
      case 'removed':
        return styles.badgeRemoved
      case 'modified':
        return styles.badgeModified
      case 'moved':
        return styles.badgeMoved
      default:
        return {}
    }
  }

  const getChangeLabel = () => {
    switch (changeType) {
      case 'added':
        return 'Added'
      case 'removed':
        return 'Removed'
      case 'modified':
        return 'Modified'
      case 'moved':
        return 'Moved'
      case 'unchanged':
        return 'Unchanged'
      default:
        return changeType
    }
  }

  return (
    <div style={styles.diffItem}>
      <div style={styles.diffItemHeader} onClick={onToggle}>
        <div style={styles.diffItemHeaderLeft}>
          <span style={{ ...styles.summaryBadge, ...getBadgeStyle() }}>
            {getChangeLabel()}
          </span>
          <span style={styles.diffItemId}>{truncateId(blockId)}</span>
          <span style={styles.diffItemType}>
            {(newBlock ?? oldBlock)?.type ?? 'unknown'}
          </span>
        </div>
        <div style={styles.diffItemActions}>
          {changeType !== 'unchanged' && (
            <>
              <button
                style={{ ...styles.button, padding: '2px 8px', fontSize: '11px' }}
                onClick={(e) => {
                  e.stopPropagation()
                  onApply()
                }}
              >
                Apply
              </button>
              <button
                style={{ ...styles.button, padding: '2px 8px', fontSize: '11px' }}
                onClick={(e) => {
                  e.stopPropagation()
                  onReject()
                }}
              >
                Reject
              </button>
            </>
          )}
          <span style={{ fontSize: '12px', color: '#666' }}>
            {isExpanded ? '▼' : '▶'}
          </span>
        </div>
      </div>

      {isExpanded && (
        <div style={styles.diffItemContent}>
          {changeType === 'modified' && contentDiff ? (
            <TextDiffView textDiff={contentDiff} />
          ) : changeType === 'added' && newBlock ? (
            <div style={{ ...styles.diffLine, ...styles.diffLineAdded }}>
              + {newBlock.content}
            </div>
          ) : changeType === 'removed' && oldBlock ? (
            <div style={{ ...styles.diffLine, ...styles.diffLineRemoved }}>
              - {oldBlock.content}
            </div>
          ) : (
            <SplitBlockView oldBlock={oldBlock} newBlock={newBlock} />
          )}
        </div>
      )}
    </div>
  )
}

// =============================================================================
// TEXT DIFF VIEW
// =============================================================================

interface TextDiffViewProps {
  textDiff: TextDiff
}

function TextDiffView({ textDiff }: TextDiffViewProps): React.ReactElement {
  return (
    <div>
      {textDiff.operations.map((op, index) => {
        let style: React.CSSProperties = styles.diffLine

        switch (op.type) {
          case 'insert':
            style = { ...styles.diffLine, ...styles.diffLineAdded }
            break
          case 'delete':
            style = { ...styles.diffLine, ...styles.diffLineRemoved }
            break
          default:
            style = { ...styles.diffLine, ...styles.diffLineEqual }
        }

        return (
          <span key={index} style={style}>
            {op.type === 'insert' && '+'}
            {op.type === 'delete' && '-'}
            {op.text}
          </span>
        )
      })}
    </div>
  )
}

// =============================================================================
// SPLIT BLOCK VIEW
// =============================================================================

interface SplitBlockViewProps {
  oldBlock?: Block
  newBlock?: Block
}

function SplitBlockView({ oldBlock, newBlock }: SplitBlockViewProps): React.ReactElement {
  return (
    <div style={styles.splitView}>
      <div style={styles.splitPane}>
        <div style={styles.splitPaneTitle}>Before</div>
        {oldBlock ? (
          <pre style={{ margin: 0, fontSize: '12px', whiteSpace: 'pre-wrap' }}>
            {oldBlock.content}
          </pre>
        ) : (
          <span style={{ color: '#999', fontStyle: 'italic' }}>Not present</span>
        )}
      </div>
      <div style={styles.splitPane}>
        <div style={styles.splitPaneTitle}>After</div>
        {newBlock ? (
          <pre style={{ margin: 0, fontSize: '12px', whiteSpace: 'pre-wrap' }}>
            {newBlock.content}
          </pre>
        ) : (
          <span style={{ color: '#999', fontStyle: 'italic' }}>Not present</span>
        )}
      </div>
    </div>
  )
}

// =============================================================================
// HELPERS
// =============================================================================

function truncateId(id: string): string {
  if (id.length <= 16) return id
  return `${id.slice(0, 8)}...${id.slice(-4)}`
}

export default DiffViewer
