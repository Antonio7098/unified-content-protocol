# UCP Command-Line Interface (`ucp-cli`)

`ucp` exposes document operations, UCL execution, LLM helpers, and CodeGraph extraction from one CLI.

## Install

```bash
cargo install ucp-cli
ucp --version
```

## Quick Start

```bash
ucp create --title "My Document" --output doc.json
ucp info --input doc.json
ucp --help
```

## CodeGraph Commands

### Build graph from a repository

```bash
ucp codegraph build \
  --repo /path/to/repo \
  --commit "$(git -C /path/to/repo rev-parse --short HEAD)" \
  --extensions rs,py,ts,tsx,js,jsx \
  --output graph.json \
  --format json
```

Useful flags:
- `--include-hidden`
- `--no-export-edges`
- `--fail-on-parse-error`
- `--max-file-bytes <N>`
- `--allow-partial`

### Inspect profile compliance + fingerprint

```bash
ucp codegraph inspect --input graph.json --format json
```

Returns `valid`, `canonical_fingerprint`, and diagnostics.

### Build LLM projection from graph

```bash
ucp codegraph prompt --input graph.json --output graph-projection.txt
```

## LLM Utilities with CodeGraph

### Generate stable ID mapping

```bash
ucp llm id-map --input graph.json --output graph-ids.json
```

### Build context from graph blocks

```bash
ucp llm context --input graph.json --max-tokens 3200 > graph-context.txt
```

### Prompt builder

```bash
ucp llm prompt --capabilities all > system-prompt.txt
```

### Shorten and expand UCL using mapping

```bash
ucp llm shorten-ucl --ucl plan.ucl --mapping graph-ids.json > plan.short.ucl
ucp llm expand-ucl --ucl plan.short.ucl --mapping graph-ids.json > plan.long.ucl
```

## End-to-End Example (manual validation flow)

```bash
# 1) Build
ucp codegraph build --repo /tmp/ts-large --output /tmp/ts-large-graph.json --format json

# 2) Inspect
ucp codegraph inspect --input /tmp/ts-large-graph.json --format json

# 3) Projection + LLM helpers
ucp codegraph prompt --input /tmp/ts-large-graph.json --output /tmp/ts-large-projection.txt
ucp llm id-map --input /tmp/ts-large-graph.json --output /tmp/ts-large-ids.json
ucp llm context --input /tmp/ts-large-graph.json --max-tokens 3000 > /tmp/ts-large-context.txt
ucp llm prompt --capabilities all > /tmp/system-prompt.txt
```

## Other Command Areas

- Document: `create`, `info`, `validate`
- Block: `add`, `get`, `delete`, `move`, `list`, `update`
- Edge: `add`, `remove`, `list`
- Navigation/Search: `nav`, `find`, `orphans`, `tree`, `prune`
- Transactions/Snapshots: `tx`, `snapshot`
- Translators: `import`, `export`
- UCL: `ucl exec`, `ucl parse`
- Agent traversal: `agent ...`
