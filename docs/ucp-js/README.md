# JavaScript & TypeScript SDK (@ucp-core/core)

The JavaScript SDK ships as the `@ucp-core/core` package inside this repository's `/packages/ucp-js` workspace. It exposes the same document, block, edge, snapshot, UCL, and LLM utilities that exist in the Rust and Python SDKs, and publishes full TypeScript definitions from `packages/ucp-js/dist/index.d.ts`.

## Installation

| Environment | Command |
| --- | --- |
| npm | `npm install @ucp-core/core@0.1.9` |
| pnpm | `pnpm add @ucp-core/core@0.1.9` |
| Yarn (Berry) | `yarn add @ucp-core/core@0.1.9` |

The package is distributed as pure ESM with `.d.ts` files and does **not** require native builds. Import paths resolve cleanly in Node.js 18+, Bun, Deno, and modern bundlers.

If you are developing inside this monorepo, run `npm install` in `packages/ucp-js/` to bootstrap local dependencies before building.

## Runtime Targets

The SDK currently ships two bundles from `packages/ucp-js/dist/index.js`:

1. **Node / Bun** — optimized for server runtimes and CommonJS interop (`import { createDocument } from '@ucp-core/core'`).
2. **Browser / Edge** — tree-shakeable ESM build that works with Vite, Next.js (app router), Remix, and Cloudflare Workers.

Because the package is implemented entirely in TypeScript/JavaScript, no WASM initialization step is required.

## Quick Start

```typescript
import { createDocument, addBlock, executeUcl } from '@ucp-core/core';

const doc = createDocument('Docs from JS');
const root = doc.root;

addBlock(doc, root, 'Hello from JavaScript', { role: 'intro' });
executeUcl(doc, `
    APPEND ${root} text WITH label="summary" :: "Summary block"
`);

console.log(JSON.stringify(doc, null, 2));
```

## TypeScript Definitions

Type information is published with the package. Highlights include:

- `ContentType`, `SemanticRole`, `EdgeType`, `ValidationResult`, and `ResourceLimits` enums/interfaces.
- `Document`, `Block`, `Edge`, and `Snapshot` shapes for strongly typed authoring.
- High-level classes such as `PromptBuilder`, `IdMapper`, `Transaction`, and `SnapshotManager` (@packages/ucp-js/dist/index.d.ts#26-318).

Because the `.d.ts` file mirrors the runtime exports exactly, your editor receives autocomplete and inline docs without extra configuration.

## Error Handling

The SDK raises descriptive error classes for UCL parsing/execution so that application code can distinguish recoverable mistakes:

```typescript
import { UclExecutionError, UclParseError, executeUcl } from '@ucp-core/core';

try {
  executeUcl(doc, 'INVALID COMMAND');
} catch (err) {
  if (err instanceof UclParseError) {
    console.error('Syntax problem', err.message);
  } else if (err instanceof UclExecutionError) {
    console.error('Runtime failure', err.command);
  }
}
```

See @packages/ucp-js/dist/index.d.ts#365-375 for the exported error types.

## Prompting & ID Mapping

`PromptBuilder` and `IdMapper` offer parity with the Rust `ucp-llm` crate:

- `prompt().edit().append().withShortIds().build()` emits a policy string for LLM orchestration.
- `mapIds(doc).shorten(text)` converts full block IDs into numeric aliases; `expand()` reverses them.

Refer to @packages/ucp-js/dist/index.d.ts#186-248 for the relevant APIs.

## Snapshot & Transaction Helpers

The SDK includes batteries-included state management for browser or desktop tooling:

- `SnapshotManager` keeps a rolling window of document versions and exposes `create`, `restore`, and `list` methods.
- `Transaction` / `TransactionManager` offer atomic editing with savepoints and timeouts.@packages/ucp-js/dist/index.d.ts#279-364

## Import Patterns

| Scenario | Example |
| --- | --- |
| ESM (Node 18+) | `import { createDocument } from '@ucp-core/core';` |
| Next.js / Vite | No config needed; the package is published as side-effect-free ESM. |
| CommonJS bridge | `const { createDocument } = await import('@ucp-core/core');` |
| TypeScript | `"moduleResolution": "bundler"` or `"nodeNext"` works out of the box. |

## Where to Go Next

1. Follow the [Getting Started quick start](../getting-started/quick-start.md) for end-to-end flows.
2. Use the [CLI guide](../ucp-cli/README.md) for scripted automation.
3. Check the [LLM utilities](../ucp-llm/README.md) for cross-SDK prompt consistency.
