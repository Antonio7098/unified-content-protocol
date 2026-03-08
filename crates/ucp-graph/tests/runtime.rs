use tempfile::tempdir;
use ucm_core::{Block, Content, Document, EdgeType};
use ucp_graph::{GraphDetailLevel, GraphFindQuery, GraphNavigator, GraphNeighborMode};

fn fixture_document() -> Document {
    let mut doc = Document::create();
    let root = doc.root;
    let section = Block::new(Content::text("Section"), Some("section")).with_label("section");
    let note = Block::new(Content::text("Note"), Some("paragraph"))
        .with_label("note")
        .with_tag("important");
    let code = Block::new(Content::code("rust", "fn helper() {}"), Some("source"))
        .with_label("helper")
        .with_tag("code");

    let section_id = doc.add_block(section, &root).unwrap();
    let note_id = doc.add_block(note, &section_id).unwrap();
    let code_id = doc.add_block(code, &section_id).unwrap();
    doc.add_edge(
        &note_id,
        EdgeType::Custom("references".to_string()),
        code_id,
    );
    doc
}

#[test]
fn memory_graph_supports_find_path_and_sessions() {
    let graph = GraphNavigator::from_document(fixture_document());
    let found = graph
        .find_nodes(&GraphFindQuery {
            label_regex: Some("helper|note".to_string()),
            tag_regex: Some("code|important".to_string()),
            ..GraphFindQuery::default()
        })
        .unwrap();
    assert_eq!(found.len(), 2);

    let note = graph.resolve_selector("note").unwrap();
    let code = graph.resolve_selector("helper").unwrap();
    let path = graph.path_between(note, code, 3).unwrap();
    assert_eq!(path.hops.len(), 1);
    assert_eq!(path.hops[0].relation, "references");

    let mut session = graph.session();
    let seed = session.seed_overview(Some(1));
    assert!(!seed.added.is_empty());
    let selected = session.select("note", GraphDetailLevel::Full).unwrap();
    assert!(!selected.added.is_empty());
    session.focus(Some("helper")).unwrap();
    let expanded = session
        .expand("note", GraphNeighborMode::Outgoing, 1, Some(8))
        .unwrap();
    assert!(!expanded.added.is_empty());
    let why = session.why_selected("helper").unwrap();
    assert!(why.selected);

    let branch = session.fork();
    let diff = session.diff(&branch);
    assert!(diff.added.is_empty());
    assert!(diff.removed.is_empty());

    let export = session.export();
    assert!(export.nodes.len() >= 3);
    assert!(export
        .edges
        .iter()
        .any(|edge| edge.relation == "references"));
}

#[test]
fn graph_roundtrips_between_json_and_sqlite() {
    let graph = GraphNavigator::from_document(fixture_document());
    let payload = graph.to_json().unwrap();
    let restored = GraphNavigator::from_json(&payload).unwrap();
    assert_eq!(
        restored.store_stats().node_count,
        graph.store_stats().node_count
    );

    let dir = tempdir().unwrap();
    let db_path = dir.path().join("graph.db");
    let sqlite = restored.persist_sqlite(&db_path, "fixture").unwrap();
    let reopened = GraphNavigator::open_sqlite(&db_path, "fixture").unwrap();

    assert_eq!(
        sqlite.store_stats().node_count,
        reopened.store_stats().node_count
    );
    assert_eq!(
        reopened.resolve_selector("helper"),
        restored.resolve_selector("helper")
    );
    let observability = reopened.observability();
    assert_eq!(observability.stats.backend, "sqlite");
    assert!(observability
        .indexed_fields
        .iter()
        .any(|field| field == "block_id"));
}
