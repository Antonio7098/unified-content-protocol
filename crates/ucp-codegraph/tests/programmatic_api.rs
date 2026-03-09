use std::fs;

use tempfile::tempdir;
use ucp_codegraph::{
    CodeGraphBuildInput, CodeGraphExpandMode, CodeGraphExportConfig, CodeGraphFindQuery,
    CodeGraphNavigator, CodeGraphRenderConfig, CodeGraphTraversalConfig,
};

fn build_graph() -> CodeGraphNavigator {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(
        dir.path().join("src/util.rs"),
        "pub fn util() -> i32 { 1 }\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("src/lib.rs"),
        "mod util;\npub fn add(a: i32, b: i32) -> i32 { util::util() + a + b }\npub fn sub(a: i32, b: i32) -> i32 { util::util() + a - b }\n",
    )
    .unwrap();

    let repository_path = dir.path().to_path_buf();
    std::mem::forget(dir);

    CodeGraphNavigator::build(&CodeGraphBuildInput {
        repository_path,
        commit_hash: "HEAD".to_string(),
        config: Default::default(),
    })
    .unwrap()
}

#[test]
fn find_nodes_supports_regex_filters() {
    let graph = build_graph();
    let matches = graph
        .find_nodes(&CodeGraphFindQuery {
            node_class: Some("symbol".to_string()),
            name_regex: Some("^a.*".to_string()),
            ..CodeGraphFindQuery::default()
        })
        .unwrap();

    assert!(matches.iter().any(|node| node.label.contains("add")));
    assert!(matches.iter().all(|node| node.node_class == "symbol"));
}

#[test]
fn path_between_symbols_finds_dependency_chain() {
    let graph = build_graph();
    let add = graph.resolve_selector("symbol:src/lib.rs::add").unwrap();
    let util = graph.resolve_selector("symbol:src/util.rs::util").unwrap();

    let path = graph.path_between(add, util, 4).unwrap();
    assert!(!path.hops.is_empty());
    assert_eq!(
        path.start.logical_key.as_deref(),
        Some("symbol:src/lib.rs::add")
    );
    assert_eq!(
        path.end.logical_key.as_deref(),
        Some("symbol:src/util.rs::util")
    );
}

#[test]
fn sessions_explain_selection_and_diff_forks() {
    let graph = build_graph();
    let mut base = graph.session();
    base.seed_overview(Some(3));
    base.expand(
        "src/lib.rs",
        CodeGraphExpandMode::File,
        &CodeGraphTraversalConfig::default(),
    )
    .unwrap();

    let mut branch = base.fork();
    branch
        .expand(
            "symbol:src/lib.rs::add",
            CodeGraphExpandMode::Dependencies,
            &CodeGraphTraversalConfig::default(),
        )
        .unwrap();

    let explanation = branch.why_selected("symbol:src/util.rs::util").unwrap();
    assert!(explanation.selected);
    assert!(explanation.explanation.contains("dependency"));

    let diff = base.diff(&branch);
    assert!(diff.added.iter().any(|node| node.label.contains("util")));
    assert!(diff.removed.is_empty());
}

#[test]
fn apply_recommended_actions_hydrates_or_expands_frontier() {
    let graph = build_graph();
    let mut session = graph.session();
    session.seed_overview(Some(3));
    session
        .expand(
            "src/lib.rs",
            CodeGraphExpandMode::File,
            &CodeGraphTraversalConfig::default(),
        )
        .unwrap();
    session.focus(Some("symbol:src/lib.rs::add")).unwrap();

    let result = session
        .apply_recommended_actions(2, 2, Some(1), None, None)
        .unwrap();

    assert!(!result.applied_actions.is_empty());
    let export = session.export(
        &CodeGraphRenderConfig::default(),
        &CodeGraphExportConfig::compact(),
    );
    assert!(export.nodes.len() >= 3);
}
