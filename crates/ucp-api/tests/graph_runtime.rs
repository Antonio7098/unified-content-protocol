use tempfile::tempdir;
use ucm_core::{Block, Content, Document, EdgeType};
use ucp_api::{GraphFindQuery, GraphNavigator};

fn fixture_document() -> Document {
    let mut doc = Document::create();
    let root = doc.root;
    let section = Block::new(Content::text("Section"), Some("section")).with_label("section");
    let note = Block::new(Content::text("Note"), Some("paragraph"))
        .with_label("note")
        .with_tag("important");
    let code =
        Block::new(Content::code("rust", "fn helper() {}"), Some("source")).with_label("helper");
    let section_id = doc.add_block(section, &root).unwrap();
    let note_id = doc.add_block(note, &section_id).unwrap();
    let code_id = doc.add_block(code, &section_id).unwrap();
    doc.add_edge(&note_id, EdgeType::References, code_id);
    doc
}

#[test]
fn graph_runtime_is_reexported_from_ucp_api() {
    let graph = GraphNavigator::from_document(fixture_document());
    let matches = graph
        .find_nodes(&GraphFindQuery {
            label_regex: Some("note|helper".to_string()),
            ..GraphFindQuery::default()
        })
        .unwrap();
    assert_eq!(matches.len(), 2);

    let dir = tempdir().unwrap();
    let db_path = dir.path().join("api-graph.db");
    let sqlite = graph.persist_sqlite(&db_path, "api-fixture").unwrap();
    assert_eq!(sqlite.store_stats().backend, "sqlite");
}
