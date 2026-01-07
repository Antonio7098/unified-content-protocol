# Python API Reference

Use `pdoc` to generate API docs for the `ucp` module. The generated HTML can be served by MkDocs just like the TypeScript docs.

## Generate Docs Locally

```bash
cd packages/ucp-python
python -m pip install -e .[dev]
pdoc src/ucp --output-directory ../../docs/reference/python
```

This creates HTML files under `docs/reference/python`. MkDocs copies them verbatim during `mkdocs build` and `mkdocs gh-deploy`.

## CI Snippet

Run this before `mkdocs gh-deploy` in your docs workflow:

```bash
cd packages/ucp-python
python -m pip install .[dev]
pdoc src/ucp --output-directory ../../docs/reference/python
```

## Key Objects

| Object | Description |
| --- | --- |
| `ucp.Document` | Block graph returned by `ucp.parse()` |
| `ucp.Block` | Node storing content, role, child IDs |
| `PromptBuilder` | Fluent builder mirroring the JS API |
| `IdMapper` | Short ID mapping + describe/shorten/expand |
| `UclBuilder` | Programmatic UCL command generator |

For full signatures, open `docs/reference/python/index.html` after running `pdoc`.
