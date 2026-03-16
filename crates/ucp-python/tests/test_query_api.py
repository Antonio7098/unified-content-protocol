"""Tests for the agent-facing Python query facade and runner."""

import importlib
from pathlib import Path

import pytest


def _build_doc(ucp):
    doc = ucp.create("Agent Query Fixture")
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


def _write_repo(root: Path) -> None:
    (root / "src").mkdir(parents=True, exist_ok=True)
    (root / "src" / "util.rs").write_text("pub fn util() -> i32 { 1 }\n")
    (root / "src" / "lib.rs").write_text(
        "mod util;\n"
        "pub fn add(a: i32, b: i32) -> i32 { util::util() + a + b }\n"
        "pub fn sub(a: i32, b: i32) -> i32 { util::util() + a - b }\n"
    )


def test_generic_query_facade_supports_agent_friendly_aliases():
    import ucp

    graph = ucp.query(ucp.Graph.from_document(_build_doc(ucp)))
    matches = graph.find(label_regex="note|helper", case_sensitive=False)
    assert {node["label"] for node in matches} == {"note", "helper"}

    note = next(node for node in matches if node["label"] == "note")
    helper = next(node for node in matches if node["label"] == "helper")

    session = graph.session()
    selected = session.add(note, detail="full")
    assert selected["added"]

    walked = session.walk(note, mode="outgoing", depth=1, limit=4)
    assert walked["added"]

    why = session.why(helper)
    assert why["selected"] is True
    assert why["origin"]["kind"] == "outgoing"

    path = session.path("root", helper, max_hops=3)
    assert path is not None
    assert len(path["hops"]) == 2


def test_run_python_query_executes_python_control_flow_and_returns_state():
    import ucp

    graph = ucp.query(ucp.Graph.from_document(_build_doc(ucp)))
    run = ucp.run_python_query(
        graph,
        """
hits = graph.find(label_regex="NOTE|HELPER", case_sensitive=False)
for node in hits:
    if re.search("note", node["label"], re.I):
        session.add(node, detail="full")
        session.walk(node, mode="outgoing", depth=1)
        break
print("selected", session.summary()["selected"])
result = {
    "labels": [node["label"] for node in hits],
    "helper_selected": session.why("helper")["selected"],
}
""",
        include_export=True,
    )

    assert run.ok is True
    assert run.result["helper_selected"] is True
    assert "selected" in run.stdout
    assert run.summary["selected"] >= 2
    assert run.export["nodes"]
    assert run.session.why("helper")["selected"] is True
    assert run.as_dict()["ok"] is True


def test_run_python_query_dedents_common_triple_quoted_snippets_and_exposes_common_builtins():
    import ucp

    graph = ucp.query(ucp.Graph.from_document(_build_doc(ucp)))
    run = ucp.run_python_query(
        graph,
        """
            hits = graph.find(label_regex="note|helper")
            chosen = next(node for node in hits if node["label"] == "helper")
            session.add(chosen, detail="summary")
            result = type(chosen).__name__, session.summary()["selected"]
        """,
    )

    assert run.ok is True
    assert run.result == ("dict", 1)


def test_prepare_python_query_reuses_compilation_and_runs_multiple_times(monkeypatch):
    import ucp

    query_module = importlib.import_module("ucp.query")
    query_module._prepare_python_query_cached.cache_clear()

    compile_calls = []
    original = query_module._compile_normalized_query

    def counting_compile(source):
        compile_calls.append(source)
        return original(source)

    monkeypatch.setattr(query_module, "_compile_normalized_query", counting_compile)

    graph = ucp.query(ucp.Graph.from_document(_build_doc(ucp)))
    prepared = ucp.prepare_python_query(
        """
            hits = graph.find(label_regex="note|helper")
            result = len(hits) + bonus
        """
    )

    first = prepared.run(graph, bindings={"bonus": 1})
    second = ucp.run_python_query(graph, prepared, bindings={"bonus": 40})

    assert first.ok is True
    assert second.ok is True
    assert first.result == 3
    assert second.result == 42
    assert prepared.source == 'hits = graph.find(label_regex="note|helper")\nresult = len(hits) + bonus'
    assert len(compile_calls) == 1


def test_run_python_query_reports_errors_and_can_raise():
    import ucp

    graph = ucp.query(ucp.Graph.from_document(_build_doc(ucp)))
    failed = ucp.run_python_query(graph, "import os\nresult = 1")
    assert failed.ok is False
    assert failed.error_type in {"ImportError", "NameError"}
    assert failed.summary["selected"] == 0

    with pytest.raises(ucp.QueryExecutionError):
        ucp.run_python_query(
            graph,
            "raise RuntimeError('boom')",
            raise_on_error=True,
        )


def test_run_python_query_accepts_raw_graph_and_raw_session():
    import ucp

    raw_graph = ucp.Graph.from_document(_build_doc(ucp))
    raw_session = raw_graph.session()
    run = ucp.run_python_query(
        raw_graph,
        "session.add('note', detail='full')\nresult = session.summary()['selected']",
        session=raw_session,
    )

    assert run.ok is True
    assert run.result == 1
    assert run.summary["selected"] == 1


def test_run_python_query_supports_bindings_for_parameterized_queries(tmp_path):
    import ucp

    _write_repo(tmp_path)
    graph = ucp.query(ucp.CodeGraph.build(str(tmp_path)))
    run = ucp.run_python_query(
        graph,
        """
            candidates = graph.find(node_class="symbol", path_regex=path_rx, name_regex=name_rx, limit=4)
            session.add(candidates[0], detail="summary")
            result = candidates[0]["logical_key"]
        """,
        bindings={
            "path_rx": r"src/lib\.rs|src/util\.rs",
            "name_rx": r"add|util",
        },
    )

    assert run.ok is True
    assert run.result == "symbol:src/lib.rs::add"


def test_codegraph_query_facade_supports_minimal_agent_surface(tmp_path):
    import ucp

    _write_repo(tmp_path)
    graph = ucp.query(ucp.CodeGraph.build(str(tmp_path)))
    matches = graph.find(node_class="symbol", name_regex="add|util", limit=4)
    assert any(node["logical_key"] == "symbol:src/lib.rs::add" for node in matches)

    add_symbol = next(
        node for node in matches if node["logical_key"] == "symbol:src/lib.rs::add"
    )
    session = graph.session()
    session.add(add_symbol, detail="summary")
    why_add = session.why(add_symbol)
    assert why_add["detail_level"] == "symbol_card"
    assert (
        session.estimate_expand(add_symbol, mode="dependencies", depth=1)[
            "estimated_nodes_added"
        ]
        >= 1
    )

    walked = session.walk(add_symbol, mode="dependencies", depth=1, limit=6)
    assert walked["added"]
    assert session.why("symbol:src/util.rs::util")["selected"] is True

    hydrated = session.hydrate(add_symbol, padding=1)
    assert "changed" in hydrated
    assert session.recommendations(top=2)
    assert (
        session.explain_export_omission(
            "symbol:src/util.rs::util", compact=True, visible_levels=0
        )["omitted"]
        is True
    )
    path = session.path(add_symbol, "symbol:src/util.rs::util")
    assert path is not None
    assert path["hops"]


def test_codegraph_query_runner_supports_branch_and_compare(tmp_path):
    import ucp

    _write_repo(tmp_path)
    graph = ucp.query(ucp.CodeGraph.build(str(tmp_path)))
    run = graph.run(
        """
candidates = graph.find(node_class="symbol", name_regex="add|sub", limit=2)
branches = []
for node in candidates:
    branch = session.fork()
    branch.add(node, detail="summary")
    branch.walk(node, mode="dependencies", depth=1)
    branches.append({
        "target": node["logical_key"],
        "selected": branch.summary()["selected"],
        "diff": session.diff(branch),
    })
session.add(candidates[0], detail="summary")
session.walk(candidates[0], mode="dependencies", depth=1)
result = {
    "branch_targets": [item["target"] for item in branches],
    "diff_sizes": [len(item["diff"]["added"]) for item in branches],
    "best": candidates[0]["logical_key"],
}
""",
        include_export=True,
        export_kwargs={"compact": True, "max_frontier_actions": 4},
    )

    assert run.ok is True
    assert run.result["best"] == "symbol:src/lib.rs::add"
    assert all(size >= 1 for size in run.result["diff_sizes"])
    assert run.export["nodes"]
    assert any(
        node["logical_key"] == "symbol:src/util.rs::util"
        for node in run.export["nodes"]
    )
