# JavaScript API Reference

This page explains how to generate and consume the TypeScript API docs for `@ucp-core/core`.

## Generating TypeDoc Output

We keep generated files out of git to avoid noise. To build docs locally:

```bash
cd packages/ucp-js
npm install
npx typedoc src/index.ts \
  --out ../../docs/reference/js \
  --name "@ucp-core/core API" \
  --tsconfig tsconfig.json
```

The command outputs HTML into `docs/reference/js`. MkDocs automatically treats that folder as static assets, so links like `/reference/js/index.html` will be served on GitHub Pages.

## CI Integration

Add this snippet to the docs workflow (already scaffolded) so the API docs stay fresh:

```bash
cd packages/ucp-js
npm ci
npx typedoc src/index.ts --out ../../docs/reference/js --tsconfig tsconfig.json
```

Do this *before* running `mkdocs gh-deploy`.

## Important Types

| Type | Description |
| --- | --- |
| `Document` | Block graph produced by `ucp.parse()` |
| `Block` | Node containing content, semantic role, child IDs |
| `PromptBuilder` | Fluent builder for capability-scoped prompts |
| `IdMapper` | Converts long block IDs to short tokens and back |
| `UclBuilder` | Ensures generated commands are syntactically valid |

For full signatures, open the generated HTML in `docs/reference/js/index.html` after running TypeDoc.
