# UCP LLM Utilities

`ucp-llm` provides deterministic helpers for using UCM documents (including CodeGraph documents) with LLM workflows.

## Installation

```toml
[dependencies]
ucp-llm = { path = "crates/ucp-llm" }
```

## IdMapper

`IdMapper` compresses long `blk_...` IDs into stable short numeric IDs.

```rust
use ucp_llm::IdMapper;
use ucm_core::{Content, Document};

let mut doc = Document::create();
let root = doc.root;
let _child = doc.add_block(Content::text("Intro"), Some("heading1"), &root)?;

let mapper = IdMapper::from_document(&doc);
let short = mapper.shorten_ucl("LINK blk_aaaaaaaaaaaaaaaaaaaaaaaa references blk_bbbbbbbbbbbbbbbbbbbbbbbb");
let long = mapper.expand_ucl(&short)?;
println!("short={short}\nlong={long}");
# Ok::<(), ucm_core::Error>(())
```

## PromptBuilder

```rust
use ucp_llm::{PromptBuilder, UclCapability};

let builder = PromptBuilder::new()
    .with_capability(UclCapability::Edit)
    .with_capability(UclCapability::Append)
    .with_rule("Do not delete blocks unless explicitly requested")
    .with_short_ids(true);

let system_prompt = builder.build_system_prompt();
let prompt = builder.build_prompt("Document structure:\n1: 2\n\nBlocks:\n1 type=text content=\"\"", "Update block 2");
println!("{}\n---\n{}", system_prompt, prompt);
```

## CodeGraph + LLM Flow

Use this when turning source code into stable, replayable LLM context:

1. Build CodeGraph (`ucp codegraph build`)
2. Verify profile/fingerprint (`ucp codegraph inspect`)
3. Generate prompt projection (`ucp codegraph prompt`)
4. Generate `IdMapper` mapping (`ucp llm id-map`)
5. Build context windows (`ucp llm context`)
6. Build system prompt (`ucp llm prompt --capabilities all`)

Example CLI sequence:

```bash
ucp codegraph build --repo /tmp/rust-large --output /tmp/rust-large-graph.json --format json
ucp codegraph inspect --input /tmp/rust-large-graph.json --format json
ucp codegraph prompt --input /tmp/rust-large-graph.json --output /tmp/rust-large-projection.txt
ucp llm id-map --input /tmp/rust-large-graph.json --output /tmp/rust-large-ids.json
ucp llm context --input /tmp/rust-large-graph.json --max-tokens 3000 > /tmp/rust-large-context.txt
ucp llm prompt --capabilities all > /tmp/system-prompt.txt
```

## Determinism Notes

- `canonical_fingerprint` from CodeGraph verifies graph integrity.
- `logical_key` metadata on nodes provides stable semantic references across block ID churn.
- ID maps are deterministic for a fixed document state.
