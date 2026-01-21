# Sprint: UCM Editor Implementation

**Sprint Goal:** Build a production-ready, Notion-like editor for UCM documents with graph visualization and diff viewing capabilities.

**Status:** In Progress
**Started:** 2026-01-20

---

## Overview

The UCM Editor is a React/TypeScript application that provides:

1. **Block Editor** - Notion-like vertical editing experience
2. **Graph View** - Visual representation of document structure and edges
3. **Diff Viewer** - Compare snapshots and view agent-proposed changes

## Architecture Principles

- **SOLID Principles** - Single responsibility, Open/closed, Liskov substitution, Interface segregation, Dependency inversion
- **Typed Errors** - Discriminated unions for exhaustive error handling
- **Logging** - Structured logging with levels and context
- **Testability** - Dependency injection, pure functions where possible
- **UX Focus** - Keyboard navigation, accessibility, responsive design
- **DX Focus** - Clear APIs, comprehensive types, helpful error messages

## Package Structure

```
packages/ucm-editor/
├── src/
│   ├── index.ts                    # Public API exports
│   ├── types/
│   │   ├── index.ts                # Re-exports
│   │   ├── editor.ts               # Editor-specific types
│   │   ├── errors.ts               # Typed error system
│   │   └── events.ts               # Event types
│   ├── core/
│   │   ├── EditorStore.ts          # Main state management
│   │   ├── SelectionManager.ts     # Selection/focus state
│   │   ├── HistoryManager.ts       # Undo/redo with snapshots
│   │   ├── DiffEngine.ts           # Document comparison
│   │   └── Logger.ts               # Structured logging
│   ├── components/
│   │   ├── Editor.tsx              # Main editor container
│   │   ├── BlockRenderer.tsx       # Renders block by type
│   │   ├── BlockEditor.tsx         # Inline editing wrapper
│   │   ├── SectionContainer.tsx    # Collapsible sections
│   │   ├── MetadataTooltip.tsx     # Hover metadata display
│   │   ├── TypeSelector.tsx        # Block type dropdown
│   │   ├── EdgeIndicator.tsx       # Shows edge connections
│   │   └── editors/
│   │       ├── TextEditor.tsx      # Rich text editing
│   │       ├── CodeEditor.tsx      # Code with highlighting
│   │       ├── TableEditor.tsx     # Tabular data
│   │       ├── MathEditor.tsx      # LaTeX equations
│   │       └── MediaEditor.tsx     # Images/media
│   ├── graph/
│   │   ├── GraphView.tsx           # Main graph container
│   │   ├── GraphNode.tsx           # Block as node
│   │   ├── GraphEdge.tsx           # Edge visualization
│   │   ├── GraphControls.tsx       # Zoom, filter, layout
│   │   └── layouts/
│   │       ├── hierarchical.ts     # Tree layout
│   │       ├── force.ts            # Force-directed
│   │       └── dagre.ts            # DAG layout
│   ├── diff/
│   │   ├── DiffViewer.tsx          # Main diff container
│   │   ├── SnapshotSelector.tsx    # Choose snapshots
│   │   ├── BlockDiff.tsx           # Single block diff
│   │   └── StructureDiff.tsx       # Tree structure changes
│   ├── hooks/
│   │   ├── useEditor.ts            # Main editor hook
│   │   ├── useBlock.ts             # Block operations
│   │   ├── useSelection.ts         # Selection state
│   │   ├── useDragDrop.ts          # Drag and drop
│   │   └── useKeyboard.ts          # Keyboard shortcuts
│   └── utils/
│       ├── dom.ts                  # DOM utilities
│       ├── keyboard.ts             # Key event handling
│       └── diff.ts                 # Diff algorithms
├── tests/
│   ├── core/
│   │   ├── EditorStore.test.ts
│   │   ├── DiffEngine.test.ts
│   │   └── HistoryManager.test.ts
│   ├── components/
│   │   └── *.test.tsx
│   └── integration/
│       └── editor.test.tsx
├── package.json
├── tsconfig.json
├── vite.config.ts
└── README.md
```

## Task Breakdown

### Phase 1: Foundation (Core Infrastructure)

- [x] Create sprint document
- [ ] Set up package with dependencies
- [ ] Implement typed error system
- [ ] Implement structured logging
- [ ] Implement EditorStore (state management)
- [ ] Implement SelectionManager
- [ ] Implement HistoryManager (undo/redo)

### Phase 2: Block Editor Components

- [ ] Implement BlockRenderer (dispatches to type-specific renderers)
- [ ] Implement BlockEditor (inline editing wrapper)
- [ ] Implement TextEditor (rich text)
- [ ] Implement CodeEditor (syntax highlighting)
- [ ] Implement TableEditor (grid editing)
- [ ] Implement MetadataTooltip
- [ ] Implement TypeSelector (change block type)
- [ ] Implement SectionContainer (collapsible sections)

### Phase 3: Interactions

- [ ] Implement keyboard navigation
- [ ] Implement drag-and-drop reordering
- [ ] Implement edge creation UI
- [ ] Implement block focus/selection

### Phase 4: Graph View

- [ ] Implement GraphView container
- [ ] Implement GraphNode component
- [ ] Implement GraphEdge component
- [ ] Implement layout algorithms
- [ ] Implement graph controls (zoom, pan, filter)
- [ ] Implement synchronized selection with editor

### Phase 5: Diff Viewer

- [ ] Implement DiffEngine (compute diffs)
- [ ] Implement DiffViewer container
- [ ] Implement SnapshotSelector
- [ ] Implement BlockDiff (inline diff display)
- [ ] Implement StructureDiff (tree changes)
- [ ] Implement accept/reject UI

### Phase 6: Polish & Testing

- [ ] Write unit tests for core modules
- [ ] Write component tests
- [ ] Write integration tests
- [ ] Add accessibility attributes
- [ ] Performance optimization
- [ ] Documentation

## Dependencies

```json
{
  "dependencies": {
    "ucp-js": "workspace:*",
    "react": "^18.2.0",
    "react-dom": "^18.2.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "typescript": "^5.0.0",
    "vite": "^5.0.0",
    "vitest": "^1.0.0",
    "@testing-library/react": "^14.0.0"
  }
}
```

## Error Codes

| Code | Category | Description |
|------|----------|-------------|
| UCM_E001 | Block | Block not found |
| UCM_E002 | Block | Invalid block content |
| UCM_E003 | Block | Block type mismatch |
| UCM_E010 | Selection | Invalid selection range |
| UCM_E011 | Selection | Selection target not found |
| UCM_E020 | Edit | Edit operation failed |
| UCM_E021 | Edit | Concurrent edit conflict |
| UCM_E030 | Drag | Invalid drop target |
| UCM_E031 | Drag | Cycle would be created |
| UCM_E040 | History | No undo available |
| UCM_E041 | History | No redo available |
| UCM_E050 | Diff | Snapshot not found |
| UCM_E051 | Diff | Incompatible snapshots |
| UCM_E060 | Graph | Layout computation failed |

## Success Criteria

1. **Functional** - All features work as specified
2. **Reliable** - No crashes, graceful error handling
3. **Performant** - Smooth editing with 1000+ blocks
4. **Accessible** - Keyboard navigable, screen reader friendly
5. **Tested** - >80% code coverage
6. **Documented** - API docs, examples, README

## Notes

- Use React 18 concurrent features for smooth updates
- Leverage ucp-js SDK for all document operations
- Keep components pure and testable
- Use CSS-in-JS or Tailwind for styling
- Consider virtualization for large documents

## Progress Log

- **2026-01-20 @ 12:20 UTC**
  1. Swapped the `ucp-js` dependency from `workspace:*` to a relative `file:../ucp-js` reference so `npm` can install without pnpm.
  2. Installed deps and built `packages/ucp-js`, then installed `packages/ucm-editor` dependencies successfully via `npm install`.
  3. Fixed `EditorStore.editBlockContent` to route thrown errors through `handleError`, ensuring `lastError` captures missing-document/block failures; re-ran `npm test` (Vitest) and all suites pass.
- **2026-01-20 @ 12:39 UTC**
  1. Added ESLint tooling and `.eslintrc.cjs`, resolving all lint warnings and ensuring `npm run lint` is clean (aside from TypeScript version notice).
  2. Fixed TypeScript errors (`BlockEditor` styles, `useEditorEvent` typing, duplicate exports, `EditorError` stack trace helper) so `npm run typecheck` succeeds.
  3. Verified `npm test -- --run`, `npm run lint`, and `npm run typecheck` all pass; logged expected EditorStore error events remain from error-handling tests.
