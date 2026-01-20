# Editor Store & Hooks

The UCM Editor is powered by a lightweight observable store that controls document state, selection, history, views, and events. This guide explains how to work with the store directly, wire up hooks, and subscribe to events.

## Creating a Store

```ts
import { createEditorStore, DEFAULT_EDITOR_CONFIG } from 'ucm-editor'

const store = createEditorStore({
  ...DEFAULT_EDITOR_CONFIG,
  autoSaveDelay: 1500,
  showBlockIds: true,
})
```

Stores can be created imperatively or through the `useEditorStore` hook. Pass a store to `<Editor store={store} />` to share state across multiple components.

## Configuration Options

| Key | Default | Description |
|-----|---------|-------------|
| `virtualizationThreshold` | `1000` | Number of blocks before list virtualization kicks in. |
| `autoSaveDelay` | `1000` ms | Debounce before auto-saving dirty documents. |
| `maxHistoryEntries` | `100` | Snapshot limit for undo/redo. |
| `enableKeyboardShortcuts` | `true` | Register default keyboard handlers. |
| `enableDragDrop` | `true` | Toggle drag/drop reordering. |
| `showBlockIds` | `false` | Surface block IDs in the UI. |
| `defaultGraphLayout` | `'hierarchical'` | Initial layout in Graph view. |
| `logLevel` | `'info'` | Logger verbosity for store logs. |

## Core APIs

```ts
store.loadDocument(document)
store.createDocument('Untitled')
store.addBlock(parentId, { content: ... })
store.editBlock(blockId, updater)
store.deleteBlock(blockId)
store.moveBlock(blockId, newParentId, position)
store.undo()
store.redo()
store.setView('graph')
store.saveDocument()
```

Each mutating method records a history entry, updates `isDirty`, and emits events so subscribers can react.

## Hooks

| Hook | Purpose |
|------|---------|
| `useEditorStore(config?)` | Creates or retrieves a store instance tied to the component lifecycle. |
| `useEditorState(store, selector)` | Subscribe to any slice of store state with `useSyncExternalStore`. |
| `useDocument(store)` | Convenience wrapper for the active document. |
| `useSelection(store)` | Access selection state and helpers (`isBlockSelected`, `isBlockFocused`). |
| `useHistory(store)` | Inspect undo/redo availability and call `undo`/`redo`. |
| `useBlockActions(store)` | High-level helpers for block CRUD and type changes. |
| `useEditActions(store)` | Track the currently edited block, pending content, and editing lifecycle. |
| `useView(store)` | Read/modify the active view and editor mode. |
| `useDrag(store)` | Monitor drag state, source/target IDs, and gestures. |
| `useKeyboardShortcuts(store, enabled?)` | Register default keyboard handlers conditionally. |
| `useEditorEvent(store, type, handler)` | Subscribe to a store event and auto-unsubscribe on unmount. |

## Events

Stores expose a typed event emitter. Common events include:

- `block:added`, `block:updated`, `block:deleted`
- `selection:changed`
- `history:undo`, `history:redo`
- `document:saved`, `document:loaded`
- `view:changed`

```ts
const unsubscribe = store.events.on('document:saved', (event) => {
  console.log('Saved document', event.data.documentId)
})

// Later
unsubscribe()
```

Use events to hook into analytics, trigger autosave flows, mirror selections, or broadcast changes to collaborative layers.

## Error Handling

All store operations throw `EditorError` instances with `code`, `category`, `severity`, and helper methods like `toUserMessage()`. Use the exported `Errors` factory helpers to compare or construct errors when extending the editor.

```ts
try {
  store.editBlock('missing', () => {})
} catch (error) {
  if (error instanceof EditorError) {
    console.error(error.code, error.toUserMessage())
  }
}
```

## Example: Custom Panel

```tsx
function BlockList({ store }) {
  const document = useDocument(store)
  const { selection, isBlockSelected } = useSelection(store)

  if (!document) return null

  return (
    <ul>
      {[...document.blocks.values()].map((block) => (
        <li key={block.id} data-selected={isBlockSelected(block.id)}>
          {block.content.text}
        </li>
      ))}
    </ul>
  )
}
```

This pattern lets you embed the stock editor next to bespoke panels that share the exact same store instance.
