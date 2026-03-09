#!/usr/bin/env python3
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PYTHON_SRC = ROOT / "crates" / "ucp-python" / "python"
ARTIFACTS = ROOT / "artifacts"
TRANSCRIPT = Path(sys.argv[1]) if len(sys.argv) > 1 else ARTIFACTS / "codegraph-query-benchmarks-transcript.md"

sys.path.insert(0, str(PYTHON_SRC))

try:
    import ucp
except ImportError as exc:  # pragma: no cover
    raise SystemExit("Build the local ucp Python extension first.") from exc


def record(handle, title, payload):
    rendered = payload if isinstance(payload, str) else json.dumps(payload, indent=2, sort_keys=True)
    handle.write(f"\n## {title}\n\n```json\n{rendered}\n```\n")


def main():
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    if TRANSCRIPT.exists():
        TRANSCRIPT.unlink()

    graph = ucp.query(ucp.CodeGraph.build(str(ROOT), continue_on_parse_error=True))
    cases = [
        ucp.QueryBenchmarkCase(
            name="rank-tests-for-run_python_query",
            description="Rank the most likely Python tests for run_python_query via lightweight name/path heuristics.",
            code=r"""
                target = graph.find(node_class='symbol', path_regex=r'crates/ucp-python/python/ucp/query\.py', name_regex=r'^run_python_query$', limit=1)[0]
                tests = graph.find(node_class='symbol', path_regex=r'crates/ucp-python/tests/.*\.py', name_regex=r'test_.*query.*|test_.*python.*|test_.*codegraph.*', limit=80)
                target_words = set(re.findall(r'[A-Za-z]+', target['logical_key'].lower()))
                ranked = []
                for node in tests:
                    key = node.get('logical_key') or ''
                    words = set(re.findall(r'[A-Za-z]+', key.lower()))
                    score = len((target_words & words) - {'symbol', 'py', 'python'})
                    if 'query_api' in key or 'query_tools' in key:
                        score += 2
                    if score:
                        ranked.append({'test': key, 'path': node.get('path'), 'score': score})
                result = {'target': target['logical_key'], 'ranked': sorted(ranked, key=lambda item: (-item['score'], item['test']))[:10]}
            """,
        ),
        ucp.QueryBenchmarkCase(
            name="trace-context-show-to-render-config",
            description="Connect CLI context_show handlers to render/export helpers using path-based explanation.",
            code=r"""
                starts = graph.find(node_class='symbol', path_regex=r'crates/ucp-cli/src/commands/(agent|codegraph)\.rs', name_regex=r'^context_show$', limit=4)
                targets = graph.find(node_class='symbol', path_regex=r'crates/ucp-cli/src/commands/codegraph\.rs|crates/ucp-codegraph/src/context\.rs', name_regex=r'make_export_config|CodeGraphRenderConfig|export_codegraph_context_with_config', limit=8)
                paths = []
                for start in starts:
                    for target in targets:
                        path = graph.path(start, target, max_hops=6)
                        if path:
                            paths.append({'start': start['logical_key'], 'target': target['logical_key'], 'hops': len(path['hops'])})
                result = sorted(paths, key=lambda item: (item['hops'], item['start'], item['target']))[:10]
            """,
        ),
        ucp.QueryBenchmarkCase(
            name="rank-before-hydrate-context-symbols",
            description="Score candidate context/render symbols, then only hydrate the top branch.",
            code=r"""
                candidates = graph.find(node_class='symbol', path_regex=r'crates/ucp-cli/src/commands/(agent|codegraph)\.rs|crates/ucp-codegraph/src/context\.rs', name_regex=r'context_show|context_export|make_export_config|export_codegraph_context_with_config|render_context_show_text', limit=20)
                ranked = []
                for node in candidates:
                    branch = session.fork()
                    branch.add(node, detail='summary')
                    branch.walk(node, mode='dependencies', depth=1, limit=12)
                    exported = branch.export(compact=True, max_frontier_actions=6)
                    path_bonus = 2 if (node.get('path') or '').startswith('crates/ucp-cli/') else 0
                    frontier_bonus = len(exported.get('frontier') or [])
                    score = exported['summary']['selected'] + len(exported['edges']) + frontier_bonus + path_bonus
                    ranked.append({'target': node['logical_key'], 'score': score})
                best = sorted(ranked, key=lambda item: (-item['score'], item['target']))[0]
                session.add(best['target'], detail='summary')
                session.walk(best['target'], mode='dependencies', depth=1, limit=12)
                session.hydrate(best['target'], padding=2)
                result = {'best': best, 'export': session.export(compact=True, max_frontier_actions=6)}
            """,
            include_export=True,
            export_kwargs={"compact": True, "max_frontier_actions": 6},
        ),
        ucp.QueryBenchmarkCase(
            name="find-public-wrappers-around-run_python_query",
            description="Expand one hop to dependents and find lightweight public wrappers before hydrating deeper helpers.",
            code=r"""
                target = graph.find(node_class='symbol', path_regex=r'crates/ucp-python/python/ucp/query\.py', name_regex=r'^run_python_query$', limit=1)[0]
                branch = session.fork()
                branch.add(target, detail='summary')
                branch.walk(target, mode='dependents', depth=1, limit=20)
                exported = branch.export(compact=True, max_frontier_actions=8)
                wrappers = []
                for node in exported['nodes']:
                    path = node.get('path') or ''
                    if path == 'crates/ucp-python/python/ucp/query.py' and node.get('logical_key') != target['logical_key']:
                        wrappers.append({'logical_key': node.get('logical_key'), 'symbol_name': node.get('symbol_name'), 'path': path})
                result = {'target': target['logical_key'], 'wrappers': wrappers}
            """,
        ),
    ]
    results = ucp.run_query_benchmark_suite(
        graph,
        cases,
        default_limits=ucp.QueryLimits(max_seconds=8.0, max_operations=120, max_trace_events=8000),
    )
    summary = ucp.summarize_query_benchmark_suite(results)

    with TRANSCRIPT.open("w", encoding="utf-8") as handle:
        handle.write("## CodeGraph agent benchmark suite demo\n\n")
        handle.write(
            "This transcript runs a small benchmark/evaluation suite of canonical agent workflows against the UCP codebase graph.\n"
        )
        record(handle, "Suite summary", summary)
        for result in results:
            record(handle, f"Benchmark: {result.case.name}", result.as_dict())


if __name__ == "__main__":
    main()