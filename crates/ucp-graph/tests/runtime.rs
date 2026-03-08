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

#[test]
fn neighborhood_expansion_records_truthful_origin_and_anchor() {
    let graph = GraphNavigator::from_document(fixture_document());
    let mut session = graph.session();

    session.select("root", GraphDetailLevel::Summary).unwrap();
    let expanded = session
        .expand("root", GraphNeighborMode::Neighborhood, 2, Some(8))
        .unwrap();
    assert!(!expanded.added.is_empty());

    let note = graph.resolve_selector("note").unwrap();

    let note_why = session.why_selected("note").unwrap();
    assert_eq!(
        note_why.origin.as_ref().and_then(|origin| origin.anchor),
        Some(graph.resolve_selector("section").unwrap())
    );
    assert_eq!(
        note_why.origin.as_ref().map(|origin| origin.kind),
        Some(ucp_graph::GraphSelectionOriginKind::Children)
    );

    let mut outgoing_session = graph.session();
    outgoing_session
        .select("note", GraphDetailLevel::Summary)
        .unwrap();
    outgoing_session
        .expand("note", GraphNeighborMode::Neighborhood, 1, Some(8))
        .unwrap();

    let helper_why = outgoing_session.why_selected("helper").unwrap();
    assert_eq!(
        helper_why.origin.as_ref().and_then(|origin| origin.anchor),
        Some(note)
    );
    assert_eq!(
        helper_why.origin.as_ref().map(|origin| origin.kind),
        Some(ucp_graph::GraphSelectionOriginKind::Outgoing)
    );
}

#[test]
fn prune_and_collapse_handle_focus_and_pins() {
    let graph = GraphNavigator::from_document(fixture_document());
    let mut session = graph.session();

    session.seed_overview(Some(1));
    session.select("helper", GraphDetailLevel::Full).unwrap();
    session.pin("section", true).unwrap();
    session.focus(Some("helper")).unwrap();

    let pruned = session.prune(Some(1));
    assert!(pruned
        .removed
        .iter()
        .all(|id| *id != graph.resolve_selector("section").unwrap()));
    let summary = session.summary();
    assert_eq!(summary.selected, 2);
    assert!(summary.focused);
    assert!(session.why_selected("helper").unwrap().selected);

    let collapsed = session.collapse("helper", false).unwrap();
    assert_eq!(collapsed.focus, None);
    let why = session.why_selected("helper").unwrap();
    assert!(!why.selected);
}

#[test]
fn graph_search_and_paths_handle_case_and_hop_limits() {
    let graph = GraphNavigator::from_document(fixture_document());

    let matches = graph
        .find_nodes(&GraphFindQuery {
            label_regex: Some("SECTION|HELPER".to_string()),
            case_sensitive: false,
            ..GraphFindQuery::default()
        })
        .unwrap();
    assert_eq!(matches.len(), 2);

    let root = graph.resolve_selector("root").unwrap();
    let note = graph.resolve_selector("note").unwrap();
    assert!(graph.path_between(root, note, 1).is_none());
    let path = graph.path_between(root, note, 2).unwrap();
    assert_eq!(path.hops.len(), 2);
    assert_eq!(path.hops[0].relation, "contains");
}
