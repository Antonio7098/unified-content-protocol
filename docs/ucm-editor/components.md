# Editor Components & Extension Points

This page catalogs the major UI components that ship with the UCM Editor and explains how to extend or wrap them. Most components are internal, but understanding their roles helps when customizing behavior via slots, events, or shared stores.

## Top-Level Structure

| Component | Purpose |
|-----------|---------|
| `Editor` | Root container; renders toolbar, status bar, and active view. |
| `BlockRenderer` | Chooses a renderer based on block type, role, or metadata. |
| `BlockEditor` | Inline editing wrapper that manages focus, keyboard shortcuts, and auto-resize behavior. |
| `SectionContainer` | Groups related blocks with collapsible affordances. |
| `TypeSelector` | Dropdown for switching block content types. |
| `MetadataTooltip` | Hover card showing tags, semantic roles, timestamps, and edges. |

## View-Specific Components

### Document View

- `DocumentView` – The scrollable, virtualized list of blocks.
- `BlockControls` – Inline toolbar for duplication, deletion, and metadata toggles.
- `EdgeIndicator` – Badges that show incoming/outgoing edges.

Customization ideas:

- Build a custom block palette that calls `store.addBlock` with templated content.
- Use `useSelection` and `useBlockActions` to sync block selection with external panes.

### Graph View

- `GraphView` – Main canvas; renders nodes, edges, and mini-map.
- `GraphNode` – Block nodes with status badges and type icons.
- `GraphEdge` – Relationship arcs with labels and weight indicators.
- `GraphControls` – Layout selector, zoom/pan controls, and edge filters.

You can hook into graph interactions by listening for events such as `view:changed` or by observing graph state on the store (`store.getState().graph`).

### Diff Viewer

- `DiffViewer` – Container orchestrating snapshot pickers and diff panes.
- `SnapshotSelector` – Dropdowns for choosing left/right snapshots.
- `BlockDiff` – Inline diff rendering for content changes.
- `StructureDiff` – Shows added/removed/moved blocks in a tree format.

Use events (`history:*`, `document:saved`) to automate snapshot creation before and after agent edits.

## Styling Strategies

1. **Wrapper Classes** – Pass `className`/`style` to `<Editor />` for high-level theming.
2. **Global Tokens** – Override CSS variables or global styles (typography, color) to match your design system.
3. **Custom Panels** – Render your own React components next to the editor while sharing the same store; this avoids forking the editor internals while letting you present bespoke dashboards.

## Embedding Bespoke Components

Because the editor exposes hooks and store accessors, you can layer new UI on top of the core experience:

```tsx
function AuditTrail({ store }) {
  const history = useHistory(store)

  return (
    <aside>
      <h3>History ({history.entries.length})</h3>
      <button onClick={history.undo} disabled={!history.canUndo}>Undo</button>
      <button onClick={history.redo} disabled={!history.canRedo}>Redo</button>
    </aside>
  )
}
```

Mount custom components within the same React tree to share context, or pass the store through props if you render them elsewhere.

## Keyboard & Accessibility

- `useKeyboardShortcuts` centralizes key bindings; disable it if you need to handle shortcuts globally.
- All focusable elements include ARIA labels and status indicators; extend them via your own components by forwarding props and hooking into the store.

## Testing Components

Use `@testing-library/react` or your preferred framework:

```tsx
import { render, fireEvent } from '@testing-library/react'
import { Editor } from 'ucm-editor'
import { parseMarkdown } from 'ucp-js'

test('renders blocks', () => {
  const doc = parseMarkdown('# Hello')
  const { getByText } = render(<Editor document={doc} />)

  expect(getByText('Hello')).toBeInTheDocument()
})
```

Because components rely on a shared store, keep tests deterministic by seeding documents via `ucp-js` and mocking network calls around `onSave`.
