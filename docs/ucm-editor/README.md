# UCM Editor Documentation

The UCM Editor is a React-based interface for exploring, editing, and reviewing Unified Content Model (UCM) documents. It builds on the headless `ucp-js` SDK and shares the same document/edge semantics, so every operation you perform visually maps to the same APIs agents and backends use.

## Goals

- **Speed up human-in-the-loop workflows.** Inspect blocks, metadata, and edges while collaborating with agents or reviewing AI-authored drafts.
- **Expose graph & diff visualizations.** Jump between document, graph, diff, and split views to understand structure and history quickly.
- **Stay type-safe and testable.** Export the same TypeScript types (`Document`, `EditorConfig`, etc.) so UI extensions and automation share a compile-time contract.

## Feature Highlights

| Capability | Description |
|------------|-------------|
| Block Editing | Notion-style block list with inline editors, drag/drop, metadata tooltips, and selection handling. |
| Graph View | DAG visualization with selectable nodes, layout controls, and synchronized selection with the editor. |
| Diff Viewer | Snapshot picker, unified/split diff modes, and change-by-change review with accept/reject flows. |
| History & Undo | Snapshot-driven history with unlimited undo/redo up to configurable thresholds. |
| Keyboard Shortcuts | Full command coverage for editing, navigation, selection, and saving. |
| Events & Logging | Structured events (`block:added`, `document:saved`, etc.) plus optional logging for analytics. |

## Documentation Map

```
docs/ucm-editor/
├── README.md          # This overview
├── integration.md     # Installing, embedding, persistence, events
├── store.md           # EditorStore, hooks, events, configuration
├── components.md      # UI building blocks & extension points
└── views.md           # Document, Graph, Diff, and Split views
```

Use the pages above to go deeper into specific aspects of the editor.

## Related Packages

- [`ucm-editor`](../../packages/ucm-editor/README.md) – package-level README with API reference, keyboard shortcuts, and development commands.
- [`ucp-js`](../../packages/ucp-js/README.md) – headless SDK for parsing, validating, and transforming documents that you load into the editor.

## Next Steps

1. Start with [integration.md](./integration.md) to wire the editor into your React application.
2. Review [store.md](./store.md) if you plan to subscribe to editor state, emit custom events, or build bespoke panes.
3. Visit [views.md](./views.md) and [components.md](./components.md) to understand the UI surface area and customization options.
