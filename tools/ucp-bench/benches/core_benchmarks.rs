//! System performance benchmarks for UCP core operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use ucm_core::{Block, Content, Document};
use ucm_engine::Engine;

fn bench_id_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("id_generation");
    group.throughput(Throughput::Elements(1));

    group.bench_function("generate_block_id", |b| {
        let content = Content::text("Hello, world!");
        b.iter(|| ucm_core::id::generate_block_id(black_box(&content), Some("intro"), None));
    });

    group.bench_function("generate_1000_ids", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let content = Content::text(format!("Content block {}", i));
                ucm_core::id::generate_block_id(black_box(&content), Some("body"), None);
            }
        });
    });

    group.finish();
}

fn bench_content_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("content");

    group.bench_function("create_text_content", |b| {
        b.iter(|| Content::text(black_box("This is a test paragraph with some content.")));
    });

    group.bench_function("create_code_content", |b| {
        b.iter(|| {
            Content::code(
                black_box("python"),
                black_box("def hello():\n    print('Hello, world!')"),
            )
        });
    });

    group.bench_function("create_json_content", |b| {
        let json = serde_json::json!({
            "name": "test",
            "value": 42,
            "nested": {"a": 1, "b": 2}
        });
        b.iter(|| Content::json(black_box(json.clone())));
    });

    group.bench_function("create_table_content", |b| {
        let rows = vec![
            vec!["A".into(), "B".into(), "C".into()],
            vec!["1".into(), "2".into(), "3".into()],
            vec!["4".into(), "5".into(), "6".into()],
        ];
        b.iter(|| Content::table(black_box(rows.clone())));
    });

    group.finish();
}

fn bench_document_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("document");

    group.bench_function("create_document", |b| {
        b.iter(|| Document::create());
    });

    group.bench_function("add_100_blocks", |b| {
        b.iter(|| {
            let mut doc = Document::create();
            let root = doc.root.clone();
            for i in 0..100 {
                let block = Block::new(Content::text(format!("Block {}", i)), None);
                let _ = doc.add_block(block, &root);
            }
            doc
        });
    });

    group.bench_function("add_1000_blocks", |b| {
        b.iter(|| {
            let mut doc = Document::create();
            let root = doc.root.clone();
            for i in 0..1000 {
                let block = Block::new(Content::text(format!("Block {}", i)), None);
                let _ = doc.add_block(block, &root);
            }
            doc
        });
    });

    // Create a document with 1000 blocks for lookup benchmarks
    let mut large_doc = Document::create();
    let root = large_doc.root.clone();
    let mut block_ids = vec![root.clone()];
    for i in 0..1000 {
        let block = Block::new(Content::text(format!("Block {}", i)), None);
        if let Ok(id) = large_doc.add_block(block, &root) {
            block_ids.push(id);
        }
    }

    group.bench_function("get_block_from_1000", |b| {
        let target_id = &block_ids[500];
        b.iter(|| large_doc.get_block(black_box(target_id)));
    });

    group.finish();
}

fn bench_engine_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine");

    group.bench_function("engine_create", |b| {
        let doc = Document::create();
        b.iter(|| Engine::new(black_box(doc.clone())));
    });

    group.finish();
}

fn bench_ucl_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("ucl_parsing");

    let simple_cmd = r#"EDIT blk_000000000001 SET text = "hello""#;
    group.bench_function("parse_simple_command", |b| {
        b.iter(|| ucl_parser::Parser::new(black_box(simple_cmd)).parse_commands_only());
    });

    let complex_cmd = r#"
ATOMIC {
    EDIT blk_000000000001 SET text = "updated"
    APPEND blk_000000000002 text :: "new content"
    MOVE blk_000000000003 TO blk_000000000004
    DELETE blk_000000000005 CASCADE
}
"#;
    group.bench_function("parse_atomic_block", |b| {
        b.iter(|| ucl_parser::Parser::new(black_box(complex_cmd)).parse_commands_only());
    });

    // Generate 100 commands
    let many_commands: String = (0..100)
        .map(|i| format!(r#"EDIT blk_{:012x} SET text = "value {}""#, i, i))
        .collect::<Vec<_>>()
        .join("\n");

    group.bench_function("parse_100_commands", |b| {
        b.iter(|| ucl_parser::Parser::new(black_box(&many_commands)).parse_commands_only());
    });

    group.finish();
}

fn bench_normalization(c: &mut Criterion) {
    let mut group = c.benchmark_group("normalization");

    let text_with_unicode = "Héllo wörld! Ça va? 日本語テスト";
    group.bench_function("normalize_unicode_text", |b| {
        b.iter(|| ucm_core::normalize::normalize_text(black_box(text_with_unicode)));
    });

    let text_with_whitespace = "  Hello   world  \n\n  test  ";
    group.bench_function("normalize_whitespace", |b| {
        b.iter(|| ucm_core::normalize::normalize_whitespace(black_box(text_with_whitespace)));
    });

    // Large text (1KB)
    let large_text: String = (0..100)
        .map(|_| "This is a test sentence with some content. ")
        .collect();
    group.bench_function("normalize_1kb_text", |b| {
        b.iter(|| ucm_core::normalize::normalize_text(black_box(&large_text)));
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_id_generation,
    bench_content_operations,
    bench_document_operations,
    bench_engine_operations,
    bench_ucl_parsing,
    bench_normalization,
);

criterion_main!(benches);
