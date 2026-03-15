"""Tests for guarded query execution, provider wrappers, and benchmark helpers."""

import importlib
from pathlib import Path


def _build_doc(ucp):
    doc = ucp.create("Query Tool Fixture")
    section = doc.add_block(doc.root_id, "Section", role="section", label="section")
    note = doc.add_block(section, "Important note", role="paragraph", label="note")
    helper = doc.add_code(section, "rust", "fn helper() {}", label="helper")
    doc.add_edge(note, ucp.EdgeType.References, helper)
    return doc


def _write_repo(root: Path) -> None:
    (root / "src").mkdir(parents=True, exist_ok=True)
    (root / "tests").mkdir(parents=True, exist_ok=True)
    (root / "src" / "util.rs").write_text("pub fn util() -> i32 { 1 }\n")
    (root / "src" / "lib.rs").write_text(
        "mod util;\n"
        "pub fn add(a: i32, b: i32) -> i32 { util::util() + a + b }\n"
        "pub fn sub(a: i32, b: i32) -> i32 { util::util() + a - b }\n"
    )
    (root / "tests" / "test_queries.py").write_text(
        "def test_add_query():\n    assert True\n"
        "def test_sub_query():\n    assert True\n"
    )


def test_run_python_query_reports_usage_and_limits():
    import ucp

    graph = ucp.query(ucp.Graph.from_document(_build_doc(ucp)))
    run = ucp.run_python_query(
        graph,
        "hits = graph.find(label_regex='note|helper')\nresult = len(hits)",
        limits=ucp.QueryLimits(max_operations=10, max_trace_events=1000),
    )

    assert run.ok is True
    assert run.result == 2
    assert run.usage.operation_count >= 1
    assert run.limits.max_operations == 10


def test_run_python_query_enforces_max_operations():
    import ucp

    graph = ucp.query(ucp.Graph.from_document(_build_doc(ucp)))
    run = ucp.run_python_query(
        graph,
        "hits = graph.find(label_regex='note|helper')\nsession.add(hits[0], detail='summary')\nresult = session.summary()['selected']",
        limits=ucp.QueryLimits(max_operations=1),
    )

    assert run.ok is False
    assert run.error_type == "QueryLimitExceededError"
    assert run.usage.operation_count >= 2


def test_run_python_query_enforces_max_trace_events():
    import ucp

    graph = ucp.query(ucp.Graph.from_document(_build_doc(ucp)))
    run = ucp.run_python_query(
        graph,
        "total = 0\nfor i in range(100):\n    total += i\nresult = total",
        limits=ucp.QueryLimits(max_trace_events=8),
    )

    assert run.ok is False
    assert run.error_type == "QueryLimitExceededError"
    assert run.usage.trace_events > 8


def test_run_python_query_enforces_max_stdout_chars():
    import ucp

    graph = ucp.query(ucp.Graph.from_document(_build_doc(ucp)))
    run = ucp.run_python_query(
        graph,
        "print('hello world')\nresult = 1",
        limits=ucp.QueryLimits(max_stdout_chars=5),
    )

    assert run.ok is False
    assert run.error_type == "QueryLimitExceededError"
    assert run.usage.stdout_chars > 5


def test_run_python_query_enforces_max_seconds_via_monkeypatched_clock(monkeypatch):
    import ucp

    query_module = importlib.import_module("ucp.query")
    clock = {"value": 0.0}

    def fake_time():
        clock["value"] += 0.2
        return clock["value"]

    monkeypatch.setattr(query_module, "_current_time", fake_time)
    graph = ucp.query(ucp.Graph.from_document(_build_doc(ucp)))
    run = ucp.run_python_query(
        graph,
        "result = 1",
        limits=ucp.QueryLimits(max_seconds=0.1),
    )

    assert run.ok is False
    assert run.error_type == "QueryLimitExceededError"


def test_python_query_tool_exposes_provider_schemas_and_executes_calls(tmp_path):
    import ucp

    _write_repo(tmp_path)
    graph = ucp.query(ucp.CodeGraph.build(str(tmp_path)))
    tool = ucp.PythonQueryTool(
        graph,
        default_include_export=True,
        default_export_kwargs={"compact": True, "max_frontier_actions": 4},
        default_limits=ucp.QueryLimits(max_operations=20, max_trace_events=1000),
    )

    assert tool.openai_tool()["function"]["name"] == "run_python_query"
    assert tool.anthropic_tool()["input_schema"]["required"] == ["code"]

    direct = tool.execute(
        {
            "code": "hits = graph.find(node_class='symbol', name_regex='add|util', limit=4)\nsession.add(hits[0], detail='summary')\nresult = hits[0]['logical_key']",
        }
    )
    assert direct.ok is True
    assert direct.payload["result"] == "symbol:src/lib.rs::add"
    assert direct.payload["export"]["nodes"]

    openai_result = tool.execute_openai_tool_call(
        {
            "id": "call_123",
            "function": {
                "name": "run_python_query",
                "arguments": "{\"code\": \"result = graph.find(node_class='symbol', name_regex='sub', limit=1)[0]['logical_key']\"}",
            },
        }
    )
    assert openai_result["role"] == "tool"
    assert openai_result["tool_call_id"] == "call_123"

    anthropic_result = tool.execute_anthropic_tool_use(
        {
            "id": "toolu_123",
            "input": {"code": "result = 'ok'"},
        }
    )
    assert anthropic_result["type"] == "tool_result"
    assert anthropic_result["tool_use_id"] == "toolu_123"
    assert anthropic_result["is_error"] is False


def test_query_benchmark_suite_runs_cases_and_stops_on_error(tmp_path):
    import ucp

    _write_repo(tmp_path)
    graph = ucp.query(ucp.CodeGraph.build(str(tmp_path)))
    cases = [
        ucp.QueryBenchmarkCase(
            name="rank-tests",
            description="Find likely tests for add",
            code="target = graph.find(node_class='symbol', name_regex='^add$', limit=1)[0]\n"
            "tests = graph.find(node_class='symbol', path_regex=r'tests/.*\\.py', name_regex=r'test_.*query', limit=20)\n"
            "result = {'target': target['logical_key'], 'tests': [node['logical_key'] for node in tests]}",
            limits=ucp.QueryLimits(max_operations=20),
        ),
        ucp.QueryBenchmarkCase(
            name="forced-failure",
            description="Hit a tiny trace budget",
            code="total = 0\nfor i in range(100):\n    total += i\nresult = total",
            limits=ucp.QueryLimits(max_trace_events=5),
        ),
        ucp.QueryBenchmarkCase(
            name="skipped-after-failure",
            description="Should not run when stop_on_error=True",
            code="result = 'later'",
        ),
    ]

    results = ucp.run_query_benchmark_suite(graph, cases, stop_on_error=True)
    assert [item.case.name for item in results] == ["rank-tests", "forced-failure"]
    assert results[0].ok is True
    assert results[1].ok is False

    summary = ucp.summarize_query_benchmark_suite(results)
    assert summary["cases"] == 2
    assert summary["ok"] == 1
    assert summary["failed"] == 1
