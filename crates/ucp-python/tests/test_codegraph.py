"""Integration tests for the Python CodeGraph API."""

from pathlib import Path


def _write_repo(root: Path) -> None:
    (root / "src").mkdir(parents=True, exist_ok=True)
    (root / "src" / "util.rs").write_text("pub fn util() -> i32 { 1 }\n")
    (root / "src" / "lib.rs").write_text(
        "mod util;\n"
        "pub fn add(a: i32, b: i32) -> i32 { util::util() + a + b }\n"
        "pub fn sub(a: i32, b: i32) -> i32 { util::util() + a - b }\n"
    )


def test_codegraph_build_find_and_roundtrip(tmp_path):
    import ucp

    _write_repo(tmp_path)
    graph = ucp.CodeGraph.build(str(tmp_path))

    matches = graph.find_nodes(node_class="symbol", name_regex="^a")
    assert any(node["logical_key"] == "symbol:src/lib.rs::add" for node in matches)

    payload = graph.to_json()
    restored = ucp.CodeGraph.from_json(payload)
    assert str(restored.resolve("src/lib.rs")) == str(graph.resolve("src/lib.rs"))


def test_codegraph_sessions_support_agentic_workflows(tmp_path):
    import ucp

    _write_repo(tmp_path)
    graph = ucp.CodeGraph.build(str(tmp_path))
    session = graph.session()

    session.seed_overview(max_depth=3)
    session.expand("src/lib.rs", mode="file")
    session.focus("symbol:src/lib.rs::add")

    branch = session.fork()
    branch.expand("symbol:src/lib.rs::add", mode="dependencies")
    why = branch.why_selected("symbol:src/util.rs::util")
    assert why["selected"] is True
    assert "dependency" in why["explanation"].lower()
    assert why["block_id"] == str(graph.resolve("symbol:src/util.rs::util"))

    diff = session.diff(branch)
    assert any(
        node["logical_key"] == "symbol:src/util.rs::util" for node in diff["added"]
    )

    path = branch.path_between("symbol:src/lib.rs::add", "symbol:src/util.rs::util")
    assert path is not None
    assert path["hops"]

    recommended = branch.apply_recommended(top=1, padding=1)
    assert recommended["applied_actions"]
    assert "focus" in recommended["update"]

    exported = branch.export(compact=True, max_frontier_actions=4)
    assert exported["nodes"]
    assert exported["frontier"] is not None
    add_node = next(
        node
        for node in exported["nodes"]
        if node["logical_key"] == "symbol:src/lib.rs::add"
    )
    assert add_node["symbol_name"] == "add"
    assert add_node["path"] == "src/lib.rs"

    branch.focus("symbol:src/util.rs::util")
    collapsed = branch.collapse("symbol:src/util.rs::util")
    assert "focus" in collapsed
    cleared_focus = branch.focus(None)
    assert "focus" in cleared_focus
    assert cleared_focus["focus"] is None


def test_codegraph_session_observability_persistence_and_estimates(tmp_path):
    import ucp

    _write_repo(tmp_path)
    graph = ucp.CodeGraph.build(str(tmp_path))
    session = graph.session()

    session.seed_overview(max_depth=3)
    expanded = session.expand("src/lib.rs", mode="file", max_nodes_visited=8)
    assert expanded["telemetry"][0]["kind"] == "expand_file"
    assert session.mutation_log()
    assert session.event_log()

    selector = graph.explain_selector("src/lib.rs")
    assert selector["match_kind"] == "path"

    estimate = session.estimate_expand(
        "symbol:src/lib.rs::add", mode="dependencies", depth=1
    )
    assert estimate["estimated_nodes_added"] >= 1

    recommendations = session.recommendations(top=2)
    assert recommendations
    assert recommendations[0]["explanation"]

    session.expand("symbol:src/lib.rs::add", mode="dependencies")
    omission = session.explain_export_omission(
        "symbol:src/util.rs::util",
        compact=True,
        visible_levels=0,
    )
    assert omission["omitted"] is True

    session.prune(max_selected=2)
    why_pruned = session.why_pruned("symbol:src/util.rs::util")
    assert why_pruned["pruned"] is True

    session_path = tmp_path / "session.json"
    session.save(str(session_path))
    restored = graph.load_session(str(session_path))
    assert restored.session_id() == session.session_id()
    assert restored.selected_block_ids() == session.selected_block_ids()
