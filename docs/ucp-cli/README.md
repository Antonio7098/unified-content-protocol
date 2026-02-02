# UCP Command-Line Interface (ucp-cli)

The **ucp-cli** binary exposes the entire Unified Content Protocol surface area from a single, scriptable entry point. It is ideal for automation, CI pipelines, regression testing, and rapid document inspection without writing host-language code.

## Installation

### Install from crates.io (Recommended)

```bash
# Install the latest version
cargo install ucp-cli

# Verify installation
ucp --version
```

### Install from Source

```bash
# Clone the repository
git clone https://github.com/Antonio7098/unified-content-protocol.git
cd unified-content-protocol

# Install from local source
cargo install --path crates/ucp-cli --force

# Or run without installing
cargo run -p ucp-cli -- --help
```

### Build Binary Manually

```bash
cargo build --release -p ucp-cli
./target/release/ucp --help
```

**crates.io:** [https://crates.io/crates/ucp-cli](https://crates.io/crates/ucp-cli)

## Quick Start

```bash
# Create a new document
ucp create --title "My Document" --output doc.json

# Get help
ucp --help

# View a specific command
ucp create --help
ucp block --help
```

## Global Flags

| Flag | Description |
| --- | --- |
| `-v`, `--verbose` | Enables debug-level tracing via `tracing_subscriber` (@crates/ucp-cli/src/main.rs#17-41). |
| `--trace` | Overrides logging to full trace-level detail. |
| `-f`, `--format <text|json>` | Controls rendering for command results (defaults to `text`). JSON output is stable for scripting. |

These flags are **global** thanks to Clap's `#[arg(global = true)]` usage in `Cli` (@crates/ucp-cli/src/cli.rs#17-33). Set them before the subcommand, e.g. `ucp --format json info --input doc.json`.

## Command Surface Overview

| Category | Commands |
| --- | --- |
| Document | `create`, `info`, `validate` |
| Block | `add`, `get`, `delete`, `move`, `list`, `update` |
| Edge | `add`, `remove`, `list` |
| Navigation | `nav children`, `nav parent`, `nav siblings`, `nav descendants` |
| Search | `find`, `orphans` |
| Structure | `tree`, `prune` |
| Transactions | `tx begin`, `tx commit`, `tx rollback`, `tx savepoint` |
| Snapshots | `snapshot create`, `snapshot restore`, `snapshot list`, `snapshot delete`, `snapshot diff` |
| Import/Export | `import markdown|html`, `export markdown|json` |
| UCL | `ucl exec`, `ucl parse` |
| Agent | `agent session create|list|close`, `goto`, `back`, `expand`, `follow`, `search`, `find`, `context add|remove|clear|show`, `view` |
| LLM | `llm id-map`, `shorten-ucl`, `expand-ucl`, `prompt`, `context` |

Table mirrors the command enums defined in `cli.rs` (@crates/ucp-cli/src/cli.rs#41-961) and matches the feature matrix from the completion summary.

## Document Management

### `create`
Creates a new document in-memory. You can optionally set a title and/or write the JSON to disk.

```bash
ucp create --title "CLI Document" --format json > doc.json
ucp create --title "CLI Document" --output doc.json
```

In `text` mode without `--output`, the CLI prints a success banner and a preview plus the JSON payload (@crates/ucp-cli/src/commands/document.rs#10-36).

### `info`
Displays metadata, block counts, token counts, and edge counts for an existing document.

```bash
ucp info --input doc.json --format json | jq
```

### `validate`
Runs the validation pipeline with optional `--max-blocks` and `--max-depth` guards. The resulting diagnostics mirror `ucm_engine::ValidationResult` and are rendered via `print_validation_result` (@crates/ucp-cli/src/output.rs#465-484).

```bash
ucp validate --input doc.json --max-blocks 500 --max-depth 8 --format text
```

## Block Operations

Invoke as `ucp block <subcommand> …`. Highlights:

- `add` — Supports `--parent`, `--content-type`, `--content`/stdin, `--language`, `--label`, `--role`, `--tags`. Content builders map to the `Content` enum (@crates/ucp-cli/src/commands/block.rs#97-188).
- `get` — Fetches a block by `--id` with optional `--metadata`-only output.
- `delete` — Accepts `--cascade` or `--preserve-children` when removing blocks.
- `move` — Allows `--to-parent`, `--before`, `--after`, or absolute `--index` repositioning.
- `list` — Emits all blocks, optionally `--ids-only` for terse listings.
- `update` — Mutates content or metadata (`--content`, `--label`, `--role`, `--summary`, `--add-tag`, `--remove-tag`).

Example:

```bash
ucp block add --input doc.json --output doc.json --parent blk_root \
  --content-type text --content "Hello from ucp-cli" --role intro --tags intro,cli
```

## Edge Operations

`ucp edge add|remove|list` manages relationships using source/target IDs and explicit edge types (`--edge-type references`, etc.). Optional descriptions and confidence scores are supported (@crates/ucp-cli/src/cli.rs#348-417).

## Navigation & Search

- `ucp nav children --input doc.json --id blk_section`
- `ucp nav descendants --input doc.json --depth 3`
- `ucp find --input doc.json --role intro --tag overview --pattern "CLI"`
- `ucp orphans --input doc.json`

The search commands rely on helper routines inside `find.rs` and output arrays or structured JSON depending on `--format`.

## Structural Views

- `tree` prints the hierarchy (`--depth`, `--ids`).
- `prune` can remove orphaned blocks or those tagged with `--tag` while writing to `--output`.

```bash
ucp tree --input doc.json --format text --depth 2 --ids
ucp prune --input doc.json --output cleaned.json --tag temp
```

## Transactions & Snapshots

Transactions (`ucp tx …`) encapsulate multi-step edits with `begin`, `commit`, `rollback`, and `savepoint` commands—all delegated through `ucm_engine::EditOperator` (@crates/ucp-cli/src/commands/block.rs#7-14, @crates/ucp-cli/src/cli.rs#470-522).

Snapshot commands mirror persistent versions: `snapshot create|restore|list|delete|diff`. Diff accepts `--from`/`--to` snapshot names.

## Import / Export

- `ucp import markdown README.md --output doc.json`
- `ucp import html page.html --extract-images --extract-links`
- `ucp export markdown --input doc.json --output doc.md`
- `ucp export json --input doc.json --pretty`

Errors from translators surface through `CliError::ParseError` conversions (@crates/ucp-cli/src/error.rs#55-64).

## UCL Execution

- `ucp ucl exec --input doc.json --output doc.json --commands "APPEND blk_root text :: \"Hello\""`
- `ucp ucl exec --file script.ucl --input doc.json --output doc.json`
- `ucp ucl parse --commands "APPEND blk_root text :: \"Hello\""`

Commands pipe through the same parser/executor stack used by `ucp-api`, making the CLI a convenient automation harness.

## Agent Traversal

Agent subcommands wrap the `ucp-agent` crate and expose full traversal workflows:

```bash
# Create a session
SESSION_ID=$(ucp agent session create --input doc.json --format json | jq -r '.session_id')

# Navigate and expand context
ucp agent goto --input doc.json --session "$SESSION_ID" --target blk_intro
ucp agent expand --input doc.json --session "$SESSION_ID" --direction down --depth 2
ucp agent context add --input doc.json --session "$SESSION_ID" --ids blk_intro,blk_code
ucp agent view --input doc.json --session "$SESSION_ID" --mode preview
```

All session-aware commands require `--session` and many accept optional IDs (`--id`, `--edge-type`, `--query`, etc.) as defined in `AgentCommands` (@crates/ucp-cli/src/cli.rs#696-902).

## LLM Utilities

Leverage `llm` subcommands to prepare token-efficient prompts:

```bash
ucp llm id-map --input doc.json --output ids.json
ucp llm shorten-ucl --ucl my.ucl --mapping ids.json
ucp llm expand-ucl --ucl short.ucl --mapping ids.json
ucp llm prompt --capabilities all
ucp llm context --input doc.json --max-tokens 3200 --blocks blk_intro,blk_code
```

These commands bridge the CLI with `ucp-llm`'s IdMapper and PromptBuilder utilities (@crates/ucp-cli/src/cli.rs#906-960).

## Sample Workflow

```bash
# 1. Create a document and persist it
ucp create --title "Workflow" --format json > doc.json

# 2. Add content
ucp block add --input doc.json --output doc.json --parent blk_root --content-type markdown \
  --content "## Section\nThis content was added from the CLI." --role heading2

# 3. Inspect structure
ucp tree --input doc.json --format text

# 4. Export to Markdown
ucp export markdown --input doc.json --output doc.md
```

## Testing & Quality Gates

Run the dedicated suite with `cargo test -p ucp-cli`.

- **Integration tests (33 cases)** — `crates/ucp-cli/tests/integration_tests.rs` verifies help text, document CRUD, block/edge/nav workflows, import/export, validation, and multi-step scenarios such as create→info→export (@crates/ucp-cli/tests/integration_tests.rs#1-372).
- **Unit tests (17 cases)** — Focused on serialization helpers (`output.rs`) and CLI state management (`state.rs`), ensuring JSON preview rendering and snapshot/transaction bookkeeping remain stable (@crates/ucp-cli/src/output.rs#455-531, @crates/ucp-cli/src/state.rs#277-340).

## Troubleshooting

- **Invalid IDs** — Ensure block IDs follow the `blk_` prefix with 24 hex chars; errors surface as `CliError::InvalidBlockId` (@crates/ucp-cli/src/error.rs#16-27).
- **Missing sessions** — Agent commands validate session identifiers and emit `CliError::SessionNotFound` when necessary (@crates/ucp-cli/src/error.rs#22-24).
- **Translator errors** — Markdown/HTML import failures map to `ParseError`; inspect stderr for line numbers.

For deeper architectural context, see the [docs/index](../index.md) and [ucp-api guide](../ucp-api/README.md).
