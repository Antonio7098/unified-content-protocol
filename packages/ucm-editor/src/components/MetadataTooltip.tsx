/**
 * MetadataTooltip - Displays block metadata on hover.
 *
 * Shows ID, type, role, tags, timestamps, and custom metadata.
 */

import React from 'react'
import type { Block } from 'ucp-content'

// =============================================================================
// STYLES
// =============================================================================

const styles = {
  overlay: {
    position: 'absolute' as const,
    top: '100%',
    right: 0,
    zIndex: 1000,
    marginTop: '4px',
    minWidth: '280px',
    maxWidth: '400px',
    backgroundColor: '#fff',
    border: '1px solid #e5e5e5',
    borderRadius: '8px',
    boxShadow: '0 4px 12px rgba(0, 0, 0, 0.15)',
    overflow: 'hidden',
  },
  header: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    padding: '8px 12px',
    backgroundColor: '#fafafa',
    borderBottom: '1px solid #e5e5e5',
  },
  headerTitle: {
    fontSize: '12px',
    fontWeight: 600,
    color: '#333',
  },
  closeButton: {
    padding: '2px 6px',
    border: 'none',
    backgroundColor: 'transparent',
    cursor: 'pointer',
    fontSize: '14px',
    color: '#999',
    lineHeight: 1,
  },
  content: {
    padding: '8px 12px',
    maxHeight: '300px',
    overflow: 'auto',
  },
  section: {
    marginBottom: '12px',
  },
  sectionLast: {
    marginBottom: 0,
  },
  sectionTitle: {
    fontSize: '10px',
    fontWeight: 600,
    color: '#999',
    textTransform: 'uppercase' as const,
    letterSpacing: '0.5px',
    marginBottom: '4px',
  },
  row: {
    display: 'flex',
    alignItems: 'flex-start',
    marginBottom: '4px',
  },
  label: {
    flex: '0 0 80px',
    fontSize: '11px',
    color: '#666',
  },
  value: {
    flex: 1,
    fontSize: '11px',
    color: '#333',
    wordBreak: 'break-all' as const,
  },
  valueCode: {
    fontFamily: 'Monaco, Consolas, monospace',
    fontSize: '10px',
    backgroundColor: '#f5f5f5',
    padding: '1px 4px',
    borderRadius: '2px',
  },
  tags: {
    display: 'flex',
    flexWrap: 'wrap' as const,
    gap: '4px',
  },
  tag: {
    display: 'inline-block',
    padding: '2px 6px',
    fontSize: '10px',
    backgroundColor: '#e3f2fd',
    color: '#1976d2',
    borderRadius: '10px',
  },
  edge: {
    display: 'flex',
    alignItems: 'center',
    gap: '4px',
    marginBottom: '4px',
    fontSize: '10px',
  },
  edgeType: {
    padding: '1px 4px',
    backgroundColor: '#f5f5f5',
    borderRadius: '2px',
    color: '#666',
  },
  edgeTarget: {
    fontFamily: 'Monaco, Consolas, monospace',
    fontSize: '9px',
    color: '#999',
  },
  noData: {
    fontSize: '11px',
    color: '#999',
    fontStyle: 'italic',
  },
  customValue: {
    fontSize: '10px',
    fontFamily: 'Monaco, Consolas, monospace',
    backgroundColor: '#f9f9f9',
    padding: '4px 6px',
    borderRadius: '2px',
    overflow: 'auto',
    maxHeight: '80px',
  },
}

// =============================================================================
// METADATA TOOLTIP PROPS
// =============================================================================

export interface MetadataTooltipProps {
  block: Block
  onClose: () => void
}

// =============================================================================
// METADATA TOOLTIP COMPONENT
// =============================================================================

/**
 * Displays detailed metadata for a block.
 */
export function MetadataTooltip({ block, onClose }: MetadataTooltipProps): React.ReactElement {
  const metadata = block.metadata

  const formatDate = (date: Date | undefined): string => {
    if (!date) return 'N/A'
    if (typeof date === 'string') {
      return new Date(date).toLocaleString()
    }
    return date.toLocaleString()
  }

  const truncateId = (id: string): string => {
    if (id.length <= 20) return id
    return `${id.slice(0, 10)}...${id.slice(-6)}`
  }

  return (
    <div
      style={styles.overlay}
      onClick={(e) => e.stopPropagation()}
      data-testid="metadata-tooltip"
    >
      {/* Header */}
      <div style={styles.header}>
        <span style={styles.headerTitle}>Block Metadata</span>
        <button style={styles.closeButton} onClick={onClose} title="Close">
          ×
        </button>
      </div>

      {/* Content */}
      <div style={styles.content}>
        {/* Identity Section */}
        <div style={styles.section}>
          <div style={styles.sectionTitle}>Identity</div>

          <div style={styles.row}>
            <span style={styles.label}>ID</span>
            <span style={styles.value}>
              <code style={styles.valueCode} title={block.id}>
                {truncateId(block.id)}
              </code>
            </span>
          </div>

          <div style={styles.row}>
            <span style={styles.label}>Type</span>
            <span style={styles.value}>{block.type}</span>
          </div>

          {(block.role || metadata?.semanticRole) && (
            <div style={styles.row}>
              <span style={styles.label}>Role</span>
              <span style={styles.value}>
                {block.role || metadata?.semanticRole}
              </span>
            </div>
          )}

          {block.label && (
            <div style={styles.row}>
              <span style={styles.label}>Label</span>
              <span style={styles.value}>{block.label}</span>
            </div>
          )}
        </div>

        {/* Tags Section */}
        <div style={styles.section}>
          <div style={styles.sectionTitle}>Tags</div>
          {block.tags.length > 0 ? (
            <div style={styles.tags}>
              {block.tags.map((tag, index) => (
                <span key={index} style={styles.tag}>
                  {tag}
                </span>
              ))}
            </div>
          ) : (
            <span style={styles.noData}>No tags</span>
          )}
        </div>

        {/* Timestamps Section */}
        <div style={styles.section}>
          <div style={styles.sectionTitle}>Timestamps</div>

          <div style={styles.row}>
            <span style={styles.label}>Created</span>
            <span style={styles.value}>{formatDate(metadata?.createdAt)}</span>
          </div>

          <div style={styles.row}>
            <span style={styles.label}>Modified</span>
            <span style={styles.value}>{formatDate(metadata?.modifiedAt)}</span>
          </div>
        </div>

        {/* Structure Section */}
        <div style={styles.section}>
          <div style={styles.sectionTitle}>Structure</div>

          <div style={styles.row}>
            <span style={styles.label}>Children</span>
            <span style={styles.value}>{block.children.length}</span>
          </div>

          <div style={styles.row}>
            <span style={styles.label}>Content</span>
            <span style={styles.value}>
              {block.content.length} chars
            </span>
          </div>
        </div>

        {/* Edges Section */}
        <div style={styles.section}>
          <div style={styles.sectionTitle}>Edges ({block.edges.length})</div>
          {block.edges.length > 0 ? (
            <div>
              {block.edges.slice(0, 5).map((edge, index) => (
                <div key={index} style={styles.edge}>
                  <span style={styles.edgeType}>{edge.edgeType}</span>
                  <span>→</span>
                  <span style={styles.edgeTarget} title={edge.target}>
                    {truncateId(edge.target)}
                  </span>
                </div>
              ))}
              {block.edges.length > 5 && (
                <span style={styles.noData}>
                  +{block.edges.length - 5} more edges
                </span>
              )}
            </div>
          ) : (
            <span style={styles.noData}>No edges</span>
          )}
        </div>

        {/* Custom Metadata Section */}
        {metadata?.custom && Object.keys(metadata.custom).length > 0 && (
          <div style={{ ...styles.section, ...styles.sectionLast }}>
            <div style={styles.sectionTitle}>Custom Metadata</div>
            <pre style={styles.customValue}>
              {JSON.stringify(metadata.custom, null, 2)}
            </pre>
          </div>
        )}
      </div>
    </div>
  )
}

export default MetadataTooltip
