# Integrating the UCM Editor

This guide covers installation, embedding, persistence, and telemetry patterns for shipping the UCM Editor inside your React application.

## Installation

```bash
npm install ucm-editor ucp-js react react-dom
# or
yarn add ucm-editor ucp-js react react-dom
# or
pnpm add ucm-editor ucp-js react react-dom
```

> Requires React 18+, Node.js 18+, and a modern bundler (Vite, Next.js, etc.).

## Minimal Embed

```tsx
import { Editor } from 'ucm-editor'
import { parseMarkdown } from 'ucp-js'

const doc = parseMarkdown(`# Hello World\n\nWelcome to UCM Editor!`)

export function App() {
  return <Editor document={doc} />
}
```

## Managed Store Pattern

Use `useEditorStore` when you need fine-grained control over persistence, events, or cross-component coordination:

```tsx
import { Editor, useEditorStore } from 'ucm-editor'
import { parseMarkdown, serializeDocument } from 'ucp-js'

export function ManagedEditor() {
  const store = useEditorStore({ autoSaveDelay: 1500 })

  useEffect(() => {
    store.loadDocument(parseMarkdown('# Roadmap'))
  }, [store])

  const handleSave = async (doc) => {
    await fetch('/api/documents/roadmap', {
      method: 'PUT',
      body: JSON.stringify(serializeDocument(doc)),
    })
  }

  return <Editor store={store} onSave={handleSave} />
}
```

## Persistence Hooks

- `onChange(document)` – fires on every successful mutation; mirror into your own state or audit logs.
- `onSave(document)` – async callback for explicit saves (toolbar, `Cmd/Ctrl + S`). Use it to store serialized documents, snapshot IDs, or diff artifacts.
- `store.saveDocument()` – convenience helper that records a snapshot in history and resets `isDirty` after your custom logic runs.

## Loading Data

Source documents from whichever pipeline suits your workflow:

1. **Markdown/HTML ingestion** – convert with `ucp-js` translators, then pass the resulting `Document` to the editor.
2. **Snapshots** – deserialize a stored snapshot via `ucp-js` helpers before loading.
3. **LLM responses** – execute UCL with `ucp-js`, then reflect the updated document in the editor for human review.

## Emitting Telemetry & Automations

The editor exposes an event emitter on the store:

```ts
const unsubscribe = store.events.on('block:added', (event) => {
  analytics.track('block_added', event.data)
})
```

Available event types include `block:*`, `document:*`, `history:*`, and `view:*`. Use them to:

- Trigger autosave flows after specific operations.
- Emit audit events whenever reviewers accept or reject diffs.
- Synchronize selections between the editor and custom sidebars.

## Styling & Layout

The default bundle ships with lightweight inline styles. To fully customize:

- Wrap `<Editor />` in your own container and pass `className`/`style` props.
- Build bespoke panes/components with the same store via hooks (see `store.md`).
- Override typography/colors globally via your design system while reusing the editor’s behaviors.

## Deployment Notes

- Bundle-size: keep `ucm-editor` in a separate chunk if your primary app doesn’t always need the UI.
- SSR: the editor expects browser APIs; lazy-load in frameworks like Next.js using dynamic import with `ssr: false`.
- Versioning: keep `ucm-editor` and `ucp-js` on compatible workspace versions to share types without casting.
