use serde_json::Value;
use std::collections::HashMap;
use ucm_core::{Block, BlockId, Content, Document};

pub mod ids {
    use ucm_core::BlockId;

    pub fn root() -> BlockId { BlockId::root() }
    pub fn metadata() -> BlockId { BlockId::from_hex("100000000001").unwrap() }
    pub fn hero() -> BlockId { BlockId::from_hex("100000000010").unwrap() }
    pub fn hero_title() -> BlockId { BlockId::from_hex("100000000011").unwrap() }
    pub fn hero_subtitle() -> BlockId { BlockId::from_hex("100000000012").unwrap() }
    pub fn steps() -> BlockId { BlockId::from_hex("100000000020").unwrap() }
    pub fn step_intro() -> BlockId { BlockId::from_hex("100000000021").unwrap() }
    pub fn step_install() -> BlockId { BlockId::from_hex("100000000022").unwrap() }
    pub fn step_command() -> BlockId { BlockId::from_hex("100000000023").unwrap() }
    pub fn recap() -> BlockId { BlockId::from_hex("100000000030").unwrap() }
    pub fn recap_list() -> BlockId { BlockId::from_hex("100000000031").unwrap() }
}

pub fn create_document() -> Document {
    let mut doc = Document::create();

    let metadata = Block::with_id(
        ids::metadata(),
        Content::json(serde_json::json!({
            "title": "Quickstart Guide",
            "audience": "Engineers",
            "version": "0.1",
            "tags": ["quickstart", "guide"],
        })),
    );
    doc.add_block(metadata, &ids::root()).ok();

    let hero = Block::with_id(ids::hero(), Content::text("Welcome to Quickstart"));
    doc.add_block(hero, &ids::root()).ok();

    doc.add_block(
        Block::with_id(ids::hero_title(), Content::text("Build Your First Benchmark")),
        &ids::hero(),
    ).ok();
    doc.add_block(
        Block::with_id(ids::hero_subtitle(), Content::text("Follow three easy steps to get productive.")),
        &ids::hero(),
    ).ok();

    let steps = Block::with_id(ids::steps(), Content::text("Steps"));
    doc.add_block(steps, &ids::root()).ok();

    doc.add_block(
        Block::with_id(ids::step_intro(), Content::text("Step 1: Read the sample document")),
        &ids::steps(),
    ).ok();

    doc.add_block(
        Block::with_id(ids::step_install(), Content::text("Step 2: Install dependencies")),
        &ids::steps(),
    ).ok();

    doc.add_block(
        Block::with_id(
            ids::step_command(),
            Content::code("bash", "cargo run -p ucp-bench -- serve"),
        ),
        &ids::steps(),
    ).ok();

    let recap = Block::with_id(ids::recap(), Content::text("Recap"));
    doc.add_block(recap, &ids::root()).ok();

    doc.add_block(
        Block::with_id(
            ids::recap_list(),
            Content::text("1. Choose a document\n2. Select providers\n3. Review results"),
        ),
        &ids::recap(),
    ).ok();

    doc
}

pub fn document_description() -> &'static str {
    r#"Quickstart Guide Document

STRUCTURE
root: [blk_100000000001, blk_100000000010, blk_100000000020, blk_100000000030]
blk_100000000010: [blk_100000000011, blk_100000000012]
blk_100000000020: [blk_100000000021, blk_100000000022, blk_100000000023]
blk_100000000030: [blk_100000000031]

BLOCKS
blk_100000000001 json "metadata": title, audience, version, tags
blk_100000000010 text "hero": Hero section
blk_100000000011 text "hero_title": Headline
blk_100000000012 text "hero_subtitle": Subtitle
blk_100000000020 text "steps": Steps container
blk_100000000021 text "step_intro": Step description
blk_100000000022 text "step_install": Installation step
blk_100000000023 code "step_command": Command snippet
blk_100000000030 text "recap": Recap section
blk_100000000031 text "recap_list": Bullet list recap"#
}

pub fn document_ucm_json(doc: &Document) -> Value {
    let structure = doc
        .structure
        .iter()
        .map(|(parent, children)| {
            (
                parent.to_string(),
                children
                    .iter()
                    .map(|child| child.to_string())
                    .collect::<Vec<String>>(),
            )
        })
        .collect::<HashMap<String, Vec<String>>>();

    let blocks = doc
        .blocks
        .iter()
        .map(|(id, block)| {
            let block_value = serde_json::to_value(block).unwrap_or(Value::Null);
            (id.to_string(), block_value)
        })
        .collect::<HashMap<String, Value>>();

    serde_json::json!({
        "id": doc.id.to_string(),
        "root": doc.root.to_string(),
        "metadata": doc.metadata,
        "structure": structure,
        "blocks": blocks,
    })
}

use crate::documents::DocumentDefinition;

pub fn definition() -> DocumentDefinition {
    DocumentDefinition {
        id: "quickstart_blog",
        name: "Quickstart Blog",
        summary: "Concise onboarding document that walks users through three benchmark steps.",
        tags: &["quickstart", "guide", "blog"],
        builder: create_document,
        llm_description: document_description,
        ucm_serializer: document_ucm_json,
    }
}
