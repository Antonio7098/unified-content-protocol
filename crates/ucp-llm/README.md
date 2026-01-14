# UCP LLM

**ucp-llm** contains helpers for turning UCM documents into LLM-friendly prompts and UCL command scaffolds. It focuses on token efficiency, deterministic mappings, and safe prompt composition.

## Features

| Component | Description |
|-----------|-------------|
| [`IdMapper`](#idmapper) | Maps long `BlockId`s (`blk_…`) to short numeric IDs to save tokens, and converts UCL in both directions |
| [`PromptBuilder`](#promptbuilder) | Builds capability-scoped system instructions, task context, and rule sets for LLM agents |
| [`presets`](#presets) | Ready-made prompt configurations (basic editing, structure manipulation, etc.) |

## Installation

```toml
[dependencies]
ucp-llm = { path = "crates/ucp-llm" }
```

## IdMapper

Token budgets collapse quickly when every UCL command references `blk_a1b2…`. `IdMapper` provides a deterministic mapping so prompts can use `1`, `2`, `3`, … instead. Key APIs:

```rust
use ucp_llm::IdMapper;
use ucm_core::{Content, Document};

let mut doc = Document::create();
let root = doc.root.clone();
let heading = doc.add_block(Content::text("Intro"), Some("heading1"), &root)?;

let mapper = IdMapper::from_document(&doc);
assert_eq!(mapper.to_short_id(&root), Some(1));
assert!(mapper.to_short_id(&heading).is_some());

let long = "EDIT blk_ff00000000000000000000 SET text = \"Hello\"";
let short = mapper.shorten_ucl(long);
assert_eq!(short, "EDIT 1 SET text = \"Hello\"");
```

Highlights:
- Deterministic ordering (root first, remaining blocks sorted by ID)
- `shorten_ucl` / `expand_ucl` for round-tripping commands
- `document_to_prompt` helper for hierarchical summaries
- `estimate_token_savings` for quick what-if analysis

## PromptBuilder

`PromptBuilder` assembles the system/task instructions LLMs need to safely emit UCL.

```rust
use ucp_llm::{PromptBuilder, UclCapability};

let builder = PromptBuilder::new()
    .with_capability(UclCapability::Edit)
    .with_capability(UclCapability::Append)
    .with_rule("Never delete blocks unless explicitly asked")
    .with_short_ids(true);

let system_prompt = builder.build_system_prompt();
let doc_context = "[1] heading1 - Intro\n  [2] paragraph - Hello";
let final_prompt = builder.build_prompt(doc_context, "Update block 2 to mention the date");
```

Capabilities gate which command documentation is included (`EDIT`, `APPEND`, `MOVE`, `DELETE`, `LINK`, `SNAPSHOT`, `TRANSACTION`). Short-ID mode automatically updates rule text so the model knows IDs like `1`, `2`, `3` will appear.

## Presets

```rust
use ucp_llm::presets;

let editing = presets::basic_editing();      // EDIT/APPEND/DELETE
let structural = presets::structure_manipulation();
let token_efficient = presets::token_efficient();
```

Use these as starting points for common workflows.

## When to Use `ucp-llm`

- Building an agent that reads a UCM document and must output UCL
- Preparing evaluation prompts for benchmarking
- Any time you need short IDs, consistent command references, or prebuilt rule sets for LLM instructions

## Public API

```rust
pub use id_mapper::IdMapper;
pub use prompt_builder::{PromptBuilder, UclCapability, presets};
```

## See Also

- [Getting Started › Concepts](../../docs/getting-started/concepts.md) – background on BlockIds and deterministic IDs
- [UCL Commands](../../docs/ucl-parser/commands.md) – reference for the actual command syntax
