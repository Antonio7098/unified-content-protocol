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

    diff = session.diff(branch)
    assert any(node["logical_key"] == "symbol:src/util.rs::util" for node in diff["added"])

    path = branch.path_between("symbol:src/lib.rs::add", "symbol:src/util.rs::util")
    assert path is not None
    assert path["hops"]

    recommended = branch.apply_recommended(top=1, padding=1)
    assert recommended["applied_actions"]

    exported = branch.export(compact=True, max_frontier_actions=4)
    assert exported["nodes"]
    assert exported["frontier"] is not None