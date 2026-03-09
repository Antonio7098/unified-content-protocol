#!/usr/bin/env python3
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PYTHON_SRC = ROOT / "crates" / "ucp-python" / "python"
ARTIFACTS = ROOT / "artifacts"
TRANSCRIPT = Path(sys.argv[1]) if len(sys.argv) > 1 else ARTIFACTS / "codegraph-query-recipes-transcript.md"

sys.path.insert(0, str(PYTHON_SRC))

try:
    import ucp
except ImportError as exc:  # pragma: no cover
    raise SystemExit("Build the local ucp Python extension first.") from exc


def record(handle, title, payload):
    rendered = payload if isinstance(payload, str) else json.dumps(payload, indent=2, sort_keys=True)
    handle.write(f"\n## {title}\n\n```json\n{rendered}\n```\n")


def run_recipe(graph, title, code, *, bindings=None, include_export=False, export_kwargs=None):
    return title, ucp.run_python_query(
        graph,
        code,
        bindings=bindings,
        include_export=include_export,
        export_kwargs=export_kwargs,
    )


def main():
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    if TRANSCRIPT.exists():
        TRANSCRIPT.unlink()

    graph = ucp.query(ucp.CodeGraph.build(str(ROOT), continue_on_parse_error=True))
    recipes = [
        run_recipe(
            graph,
            "Compare mirrored context_show handlers",
            """
                candidates = graph.find(
                    node_class="symbol",
                    path_regex=path_rx,
                    name_regex=r"^context_show$",
                    limit=4,
                )
                branches = []
                for node in candidates:
                    branch = session.fork()
                    branch.add(node, detail="summary")
                    branch.walk(node, mode="dependencies", depth=1, limit=8)
                    branch.hydrate(node, padding=2)
                    exported = branch.export(compact=True, max_frontier_actions=4)
                    branches.append({
                        "target": node["logical_key"],
                        "selected": exported["summary"]["selected"],
                        "edges": len(exported["edges"]),
                        "frontier": [item["action"] for item in exported.get("frontier") or []],
                    })
                result = branches
            """,
            bindings={"path_rx": r"crates/ucp-cli/src/commands/(agent|codegraph)\.rs"},
        ),
        run_recipe(
            graph,
            "Trace context_show to render configuration symbols",
            """
                starts = graph.find(node_class="symbol", path_regex=cli_rx, name_regex=r"^context_show$", limit=2)
                targets = graph.find(node_class="symbol", path_regex=core_rx, name_regex=r"CodeGraphRenderConfig|make_export_config|export_codegraph_context_with_config", limit=6)
                paths = []
                for start in starts:
                    for target in targets:
                        path = graph.path(start, target, max_hops=6)
                        if path:
                            paths.append({
                                "start": start["logical_key"],
                                "target": target["logical_key"],
                                "hops": len(path["hops"]),
                            })
                result = sorted(paths, key=lambda item: (item["hops"], item["start"], item["target"]))[:8]
            """,
            bindings={
                "cli_rx": r"crates/ucp-cli/src/commands/(agent|codegraph)\.rs",
                "core_rx": r"crates/ucp-cli/src/commands/codegraph\.rs|crates/ucp-codegraph/src/context\.rs",
            },
        ),
        run_recipe(
            graph,
            "Rank session-related symbols by local evidence",
            """
                hits = graph.find(
                    node_class="symbol",
                    path_regex=path_rx,
                    name_regex=r"session|context|render|export",
                    limit=12,
                )
                scored = []
                for node in hits:
                    branch = session.fork()
                    branch.add(node, detail="summary")
                    branch.walk(node, mode="dependents", depth=1, limit=8)
                    exported = branch.export(compact=True, max_frontier_actions=4)
                    score = exported["summary"]["selected"] + len(exported["edges"]) + len(exported.get("frontier") or [])
                    scored.append({
                        "target": node["logical_key"],
                        "score": score,
                        "selected": exported["summary"]["selected"],
                        "edges": len(exported["edges"]),
                    })
                result = sorted(scored, key=lambda item: (-item["score"], item["target"]))[:6]
            """,
            bindings={"path_rx": r"crates/ucp-(cli|codegraph|python)/"},
        ),
    ]

    with TRANSCRIPT.open("w", encoding="utf-8") as handle:
        handle.write("## CodeGraph query recipes on the UCP repo\n\n")
        handle.write(
            "This transcript captures a few higher-level recipe patterns over the UCP codebase graph: "
            "branch-and-compare, explanation paths, and lightweight ranking via Python control flow.\n"
        )
        record(handle, "CodeGraph summary", {"repr": repr(graph.raw), "node_count": len(graph.raw.to_document().blocks)})
        for title, run in recipes:
            record(handle, title, run.as_dict())


if __name__ == "__main__":
    main()