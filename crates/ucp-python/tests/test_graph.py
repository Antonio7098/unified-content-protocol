from pathlib import Path


def _build_doc(ucp):
    doc = ucp.create("Graph Fixture")
    section = doc.add_block(doc.root_id, "Section", role="section", label="section")
    note = doc.add_block(
        section,
        "Important note",
        role="paragraph",
        label="note",
        tags=["important"],
    )
    helper = doc.add_code(section, "rust", "fn helper() {}", label="helper")
    doc.add_edge(note, ucp.EdgeType.References, helper)
    return doc


def test_graph_runtime_supports_json_and_sqlite(tmp_path: Path):
    import ucp

    doc = _build_doc(ucp)
    graph = ucp.Graph.from_document(doc)

    stats = graph.store_stats()
    assert stats["node_count"] >= 4
    assert str(graph.resolve("section"))

    matches = graph.find_nodes(label_regex="helper|note", tag_regex="important")
    assert any(node["label"] == "note" for node in matches)

    path = graph.path_between("note", "helper", max_hops=3)
    assert path is not None
    assert path["hops"][0]["relation"] in {"references", "References"}

    json_path = tmp_path / "graph.json"
    db_path = tmp_path / "graph.db"
    graph.save(str(json_path))
    loaded = ucp.Graph.load(str(json_path))
    sqlite = loaded.persist_sqlite(str(db_path), "fixture")
    reopened = ucp.Graph.from_sqlite(str(db_path), "fixture")

    assert reopened.store_stats()["backend"] == "sqlite"
    assert reopened.store_stats()["root_block_id"] == str(doc.root_id)
    assert str(sqlite.resolve("helper")) == str(reopened.resolve("helper"))
    assert reopened.describe("helper")["block_id"] == str(reopened.resolve("helper"))
    assert str(reopened.to_document().root_id) == str(doc.root_id)


def test_graph_sessions_support_traversal_and_diff(tmp_path: Path):
    import ucp

    graph = ucp.Graph.from_document(_build_doc(ucp))
    session = graph.session()

    seeded = session.seed_overview(max_depth=1)
    assert seeded["added"]

    selected = session.select("note", detail_level="full")
    assert selected["added"]

    expanded = session.expand("note", mode="outgoing", depth=1)
    assert expanded["added"]

    why = session.why_selected("helper")
    assert why["selected"] is True

    branch = session.fork()
    branch.pin("helper", pinned=True)
    diff = session.diff(branch)
    assert diff["added"] == []
    assert diff["removed"] == []

    exported = branch.export()
    assert exported["nodes"]
    assert any(edge["relation"].lower() == "references" for edge in exported["edges"])


def test_graph_sessions_preserve_truthful_origin_and_focus():
    import ucp

    graph = ucp.Graph.from_document(_build_doc(ucp))
    session = graph.session()

    session.select("root", detail_level="summary")
    session.expand("root", mode="neighborhood", depth=2)

    note_why = session.why_selected("note")
    assert note_why["selected"] is True
    assert note_why["origin"]["kind"] == "children"
    assert note_why["anchor"]["block_id"] == str(graph.resolve("section"))

    outgoing = graph.session()
    outgoing.select("note", detail_level="summary")
    outgoing.expand("note", mode="neighborhood", depth=1)

    helper_why = outgoing.why_selected("helper")
    assert helper_why["selected"] is True
    assert helper_why["origin"]["kind"] == "outgoing"
    assert helper_why["anchor"]["block_id"] == str(graph.resolve("note"))

    outgoing.focus("helper")
    collapsed = outgoing.collapse("helper", include_descendants=False)
    assert "focus" in collapsed
    assert collapsed["focus"] is None


def test_graph_search_and_paths_handle_case_and_hop_limits():
    import ucp

    graph = ucp.Graph.from_document(_build_doc(ucp))

    matches = graph.find_nodes(label_regex="SECTION|HELPER", case_sensitive=False)
    assert {node["label"] for node in matches} == {"section", "helper"}

    assert graph.path_between("root", "note", max_hops=1) is None
    path = graph.path_between("root", "note", max_hops=2)
    assert path is not None
    assert len(path["hops"]) == 2