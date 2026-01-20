# UCM Editor

A Notion-like editor for UCM (Unified Content Model) documents with graph visualization and diff viewing capabilities.

## Features

- **Block-based Editing** - Notion-like vertical editing experience with inline editors
- **Drag & Drop** - Reorder blocks and sections by dragging
- **Metadata Tooltips** - Hover over blocks to see metadata, tags, timestamps, and edges
- **Graph View** - Visual DAG representation of document structure with multiple layout algorithms
- **Diff Viewer** - Compare snapshots and view/accept/reject changes
- **Keyboard Shortcuts** - Full keyboard navigation and common shortcuts
- **Undo/Redo** - Complete history with snapshot-based undo/redo
- **Type Support** - Text, Code, Table, Math, JSON, Media content types
- **Observability** - Structured logging and event system

## Installation

```bash
npm install ucm-editor
# or
yarn add ucm-editor
# or
pnpm add ucm-editor
```

## Quick Start

```tsx
import { Editor } from 'ucm-editor'
import { parseMarkdown } from 'ucp-js'

// Parse a markdown document
const doc = parseMarkdown(`# Hello World

Welcome to UCM Editor!

## Features

- Block-based editing
- Graph visualization
- Diff viewing
`)

function App() {
  return (
    <Editor
      document={doc}
      onChange={(doc) => console.log('Document changed:', doc)}
    />
  )
}
```

## Architecture

### Core Modules

- **EditorStore** - Central state management with observable pattern
- **SelectionManager** - Block and text selection handling
- **DiffEngine** - Document comparison and diff computation
- **Logger** - Structured logging with levels and context

### Components

- **Editor** - Main container with view switching and toolbar
- **BlockRenderer** - Renders blocks based on type and role
- **BlockEditor** - Inline editing with auto-growing textarea
- **MetadataTooltip** - Displays block metadata on hover
- **GraphView** - Canvas-based graph visualization
- **DiffViewer** - Side-by-side or unified diff display

## API Reference

### Editor Props

```typescript
interface EditorProps {
  // Initial document to load
  document?: Document

  // Editor configuration
  config?: Partial<EditorConfig>

  // External store instance
  store?: EditorStoreInstance

  // Callback when document changes
  onChange?: (document: Document) => void

  // Callback when document is saved
  onSave?: (document: Document) => Promise<void>

  // Additional styling
  className?: string
  style?: React.CSSProperties
}
```

### Editor Configuration

```typescript
interface EditorConfig {
  // Maximum blocks before virtualization
  virtualizationThreshold: number  // default: 1000

  // Auto-save debounce delay (ms)
  autoSaveDelay: number            // default: 1000

  // Maximum history entries
  maxHistoryEntries: number        // default: 100

  // Enable keyboard shortcuts
  enableKeyboardShortcuts: boolean // default: true

  // Enable drag and drop
  enableDragDrop: boolean          // default: true

  // Show block IDs in UI
  showBlockIds: boolean            // default: false

  // Default graph layout
  defaultGraphLayout: GraphLayout  // default: 'hierarchical'

  // Log level
  logLevel: LogLevel               // default: 'info'
}
```

### Using the Store Directly

```typescript
import { createEditorStore } from 'ucm-editor'
import { parseMarkdown } from 'ucp-js'

const store = createEditorStore()

// Load a document
const doc = parseMarkdown('# Hello')
store.loadDocument(doc)

// Subscribe to state changes
const unsubscribe = store.subscribe((state, prevState) => {
  console.log('State changed:', state)
})

// Perform operations
store.select('blk_123')
store.startEditing('blk_123')
store.updatePendingContent('New content')
store.stopEditing(true) // save changes

// Undo/Redo
store.undo()
store.redo()

// Clean up
unsubscribe()
```

### React Hooks

```typescript
import {
  useEditorStore,
  useEditorState,
  useDocument,
  useSelection,
  useHistory,
  useBlockActions,
  useKeyboardShortcuts,
} from 'ucm-editor'

function MyComponent() {
  const store = useEditorStore()
  const document = useDocument(store)
  const { selection, isBlockSelected } = useSelection(store)
  const { canUndo, canRedo, undo, redo } = useHistory(store)
  const { addBlock, editBlock, deleteBlock } = useBlockActions(store)

  useKeyboardShortcuts(store)

  // ...
}
```

### Error Handling

```typescript
import { EditorError, Errors } from 'ucm-editor'

try {
  store.editBlock('invalid_id', 'content')
} catch (error) {
  if (error instanceof EditorError) {
    console.log('Error code:', error.code)
    console.log('Category:', error.category)
    console.log('Suggestion:', error.suggestion)
    console.log('User message:', error.toUserMessage())
  }
}

// Or use Result types
import { ok, err, unwrap } from 'ucm-editor'

function safeOperation(): Result<string, EditorError> {
  if (success) {
    return ok('result')
  }
  return err(Errors.blockNotFound('blk_123'))
}

const result = safeOperation()
if (result.ok) {
  console.log(result.value)
} else {
  console.error(result.error.message)
}
```

### Events

```typescript
import { useEditorEvent } from 'ucm-editor'

function MyComponent({ store }) {
  useEditorEvent(store, 'block:added', (event) => {
    console.log('Block added:', event.data.blockId)
  })

  useEditorEvent(store, 'document:saved', (event) => {
    console.log('Document saved:', event.data.documentId)
  })
}
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd/Ctrl + Z` | Undo |
| `Cmd/Ctrl + Shift + Z` | Redo |
| `Cmd/Ctrl + Y` | Redo |
| `Cmd/Ctrl + S` | Save |
| `Cmd/Ctrl + A` | Select all |
| `Escape` | Cancel edit / Clear selection |
| `Enter` | Start editing selected block |
| `Delete/Backspace` | Delete selected blocks |
| `↑/↓` | Navigate blocks |
| `Shift + ↑/↓` | Extend selection |

## Graph Layouts

- **Hierarchical** - Tree layout with parent-child relationships
- **Force** - Force-directed layout with physics simulation
- **Radial** - Concentric circles with root at center

## Views

- **Document** - Block-based editing view
- **Graph** - Interactive graph visualization
- **Diff** - Snapshot comparison view
- **Split** - Side-by-side document and graph

## Development

```bash
# Install dependencies
npm install

# Run tests
npm test

# Build
npm run build

# Type check
npm run typecheck
```

## License

MIT
