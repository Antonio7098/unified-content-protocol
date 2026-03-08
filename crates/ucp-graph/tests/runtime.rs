use tempfile::tempdir;
use ucm_core::{Block, Content, Document, EdgeType};
use ucp_graph::{GraphDetailLevel, GraphFindQuery, GraphNavigator, GraphNeighborMode};

#[derive(Clone)]
struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0
    }

    fn next_usize(&mut self, upper: usize) -> usize {
        (self.next_u64() as usize) % upper.max(1)
    }
}

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

fn generated_document(seed: u64) -> Document {
    let mut rng = Lcg::new(seed);
    let mut doc = Document::create();
    let root = doc.root;
    let mut block_ids = vec![root];

    for index in 0..12 {
        let parent = block_ids[rng.next_usize(block_ids.len())];
        let label = format!("node-{seed}-{index}");
        let block = if index % 3 == 0 {
            Block::new(
                Content::code("rust", format!("fn {label}() -> usize {{ {index} }}")),
                Some("source"),
            )
        } else {
            Block::new(
                Content::text(format!("Generated node {seed}-{index}")),
                Some("section"),
            )
        }
        .with_label(label)
        .with_tag(if index % 2 == 0 { "even" } else { "odd" });
        let block_id = doc.add_block(block, &parent).unwrap();
        block_ids.push(block_id);
    }

    for relation_index in 0..6 {
        let source_index = 1 + rng.next_usize(block_ids.len() - 1);
        let mut target_index = 1 + rng.next_usize(block_ids.len() - 1);
        if source_index == target_index {
            target_index = (target_index % (block_ids.len() - 1)) + 1;
        }
        let edge_type = if relation_index % 2 == 0 {
            EdgeType::References
        } else {
            EdgeType::Custom(format!("rel-{seed}-{relation_index}"))
        };
        doc.add_edge(&block_ids[source_index], edge_type, block_ids[target_index]);
    }

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
    let note = reopened.resolve_selector("note").unwrap();
    let helper = reopened.resolve_selector("helper").unwrap();
    let path = reopened.path_between(note, helper, 2).unwrap();
    assert_eq!(path.hops[0].relation, "references");
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

#[test]
fn randomized_memory_and_sqlite_graphs_preserve_traversal_invariants() {
    let dir = tempdir().unwrap();

    for seed in 0..5u64 {
        let doc = generated_document(seed);
        let graph = GraphNavigator::from_document(doc.clone());
        let sqlite = graph
            .persist_sqlite(
                dir.path().join(format!("seed-{seed}.db")),
                format!("seed-{seed}"),
            )
            .unwrap();

        assert_eq!(graph.store_stats().node_count, doc.blocks.len());
        assert_eq!(
            graph.store_stats().node_count,
            sqlite.store_stats().node_count
        );

        for (block_id, block) in &doc.blocks {
            let memory = graph.describe_node(*block_id).unwrap();
            let persisted = sqlite.describe_node(*block_id).unwrap();
            assert_eq!(memory.block_id, persisted.block_id);
            assert_eq!(memory.label, persisted.label);
            assert_eq!(
                memory.parent, persisted.parent,
                "seed={seed} block={block_id}"
            );
            assert_eq!(
                memory.children, persisted.children,
                "seed={seed} block={block_id}"
            );
            assert_eq!(
                memory.outgoing_edges, persisted.outgoing_edges,
                "seed={seed} block={block_id}"
            );
            assert_eq!(
                memory.incoming_edges, persisted.incoming_edges,
                "seed={seed} block={block_id}"
            );

            if let Some(label) = &block.metadata.label {
                assert_eq!(graph.resolve_selector(label), Some(*block_id));
                assert_eq!(sqlite.resolve_selector(label), Some(*block_id));
            }

            for child in doc.children(block_id) {
                let direct = graph.path_between(*block_id, *child, 1).unwrap();
                assert_eq!(direct.hops.len(), 1);
                assert_eq!(direct.hops[0].to, *child);
            }
        }

        let labels = doc
            .blocks
            .values()
            .filter_map(|block| block.metadata.label.clone())
            .collect::<Vec<_>>();
        let mut rng = Lcg::new(seed ^ 0xA5A5_A5A5_A5A5_A5A5);
        let mut memory_session = graph.session();
        let mut sqlite_session = sqlite.session();

        for _ in 0..16 {
            let selector = if rng.next_usize(5) == 0 {
                "root".to_string()
            } else {
                labels[rng.next_usize(labels.len())].clone()
            };
            match rng.next_usize(5) {
                0 => {
                    let detail_level = match rng.next_usize(3) {
                        0 => GraphDetailLevel::Stub,
                        1 => GraphDetailLevel::Summary,
                        _ => GraphDetailLevel::Full,
                    };
                    memory_session.select(&selector, detail_level).unwrap();
                    sqlite_session.select(&selector, detail_level).unwrap();
                }
                1 => {
                    let mode = match rng.next_usize(5) {
                        0 => GraphNeighborMode::Children,
                        1 => GraphNeighborMode::Parents,
                        2 => GraphNeighborMode::Outgoing,
                        3 => GraphNeighborMode::Incoming,
                        _ => GraphNeighborMode::Neighborhood,
                    };
                    let depth = 1 + rng.next_usize(2);
                    let max_add = if rng.next_usize(3) == 0 {
                        Some(1 + rng.next_usize(4))
                    } else {
                        None
                    };
                    let left = memory_session
                        .expand(&selector, mode, depth, max_add)
                        .unwrap();
                    let right = sqlite_session
                        .expand(&selector, mode, depth, max_add)
                        .unwrap();
                    assert_eq!(left.added.len(), right.added.len());
                    assert_eq!(left.removed.len(), right.removed.len());
                }
                2 => {
                    memory_session.focus(Some(&selector)).unwrap();
                    sqlite_session.focus(Some(&selector)).unwrap();
                }
                3 => {
                    memory_session.pin(&selector, true).unwrap();
                    sqlite_session.pin(&selector, true).unwrap();
                }
                _ => {
                    memory_session.prune(Some(6));
                    sqlite_session.prune(Some(6));
                }
            }

            let mut left_ids = memory_session.selected_block_ids();
            let mut right_ids = sqlite_session.selected_block_ids();
            left_ids.sort_by_key(|id| id.to_string());
            right_ids.sort_by_key(|id| id.to_string());
            assert_eq!(left_ids, right_ids);

            let left_export = memory_session.export();
            let right_export = sqlite_session.export();
            assert_eq!(left_export.nodes.len(), right_export.nodes.len());
            assert_eq!(left_export.edges.len(), right_export.edges.len());

            let selected = left_export
                .nodes
                .iter()
                .map(|node| node.block_id)
                .collect::<std::collections::HashSet<_>>();
            assert!(left_export
                .edges
                .iter()
                .all(|edge| selected.contains(&edge.source) && selected.contains(&edge.target)));
        }
    }
}
