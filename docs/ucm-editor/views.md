# Views & Workflows

The UCM Editor provides multiple perspectives on the same document state. Each view is backed by the shared `EditorStore`, so switching views never loses context or undo history.

## Document View

- **Purpose:** Primary editing surface with Notion-style blocks.
- **Highlights:** Inline editing, drag-and-drop reordering, metadata tooltips, edge indicators, keyboard shortcuts.
- **When to use:** Authoring, reviewing narrative content, editing metadata, inserting new blocks.

### Tips

- Use `Cmd/Ctrl + Enter` to start editing the focused block.
- Toggle block IDs via `config.showBlockIds` for debugging and referencing in UCL.
- Bind `onChange` to stream block edits to your autosave logic.

## Graph View

- **Purpose:** Visualize the document as a directed acyclic graph (DAG).
- **Highlights:** Multiple layouts (`hierarchical`, `force`, `radial`), edge filters, node hover states, synchronized selection with Document View.
- **When to use:** Debugging relationship structures, spotting orphaned blocks, teaching UCM concepts to non-technical stakeholders.

### Tips

- Listen for `view:changed` events to track when reviewers enter/leave the graph.
- Extend the graph sidebar by reading `store.getState().graph` for layout, viewport, and active node information.

## Diff View

- **Purpose:** Compare two snapshots of a document.
- **Highlights:** Snapshot selector, unified/split diff modes, change pills (added, removed, moved, text edits), accept/reject helpers.
- **When to use:** Reviewing agent-generated revisions, code reviews for content, regression analysis.

### Tips

- Create snapshots programmatically via `store.saveDocument()` before/after invoking agents.
- Combine with `store.events.on('diff:changeSelected', ...)` (custom events) to annotate diffs in your own tooling.

## Split View

- **Purpose:** Display Document and Graph views side-by-side for context-rich editing.
- **Highlights:** Synchronized scrolling/selection, responsive grid layout.
- **When to use:** Structural refactors that require both linear and graph perspectives.

### Tips

- Use CSS grid overrides to resize panes or add third-party panels.
- Persist the preferred view in user settings by calling `store.setView('split')` after loading preferences.

## Switching Views

- Toolbar buttons labelled Document, Graph, Diff, Split.
- Programmatic: `store.setView('graph')` or `useView(store).setView('diff')`.
- Keyboard shortcuts can be added via custom bindings (e.g., `Cmd+1` for Document).

## Extending Views

All views read from the same store slices (`state.view`, `state.graph`, `state.diff`). To extend them:

1. Observe the relevant slice via `useEditorState`.
2. Render your own React components alongside `<Editor />` using the shared store.
3. Dispatch additional actions or events (e.g., custom `store.events.emit('view:analytics', data)` wrappers) if needed.

Because views are decoupled from persistence, you can even replace a built-in view with your own implementation by conditionally rendering `<Editor />` with custom children that consume the store.
