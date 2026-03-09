#!/usr/bin/env python3
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PYTHON_SRC = ROOT / "crates" / "ucp-python" / "python"
ARTIFACTS = ROOT / "artifacts"
TRANSCRIPT = Path(sys.argv[1]) if len(sys.argv) > 1 else ARTIFACTS / "codegraph-python-query-demo-transcript.md"
TARGET_REGEX = "context_show|get_session_mut|print_context_update|resolve_codegraph_selector"
PATH_REGEX = r"crates/ucp-cli/src/commands/codegraph\.rs|crates/ucp-cli/src/commands/agent\.rs|crates/ucp-codegraph/src/context\.rs"

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

    raw_graph = ucp.CodeGraph.build(str(ROOT), continue_on_parse_error=True)
    graph = ucp.query(raw_graph)

    run = graph.run(
        f"""
candidates = graph.find(
    node_class="symbol",
    path_regex={PATH_REGEX!r},
    name_regex={TARGET_REGEX!r},
    limit=6,
)
branches = []
for node in candidates[:3]:
    branch = session.fork()
    branch.add(node, detail="summary")
    branch.walk(node, mode="dependencies", depth=1, limit=8)
    branch.hydrate(node, padding=2)
    exported = branch.export(compact=True, max_frontier_actions=4)
    branches.append({{
        "target": node["logical_key"],
        "selected": exported["summary"]["selected"],
        "has_frontier": bool(exported.get("frontier")),
        "diff": session.diff(branch),
    }})

best = max(branches, key=lambda item: item["selected"]) if branches else None
if best:
    session.add(best["target"], detail="summary")
    session.walk(best["target"], mode="dependencies", depth=1, limit=8)
    session.hydrate(best["target"], padding=2)
result = {{
    "candidates": candidates,
    "branches": branches,
    "best": best,
    "final_export": session.export(compact=True, max_frontier_actions=6),
}}
""",
        include_export=True,
        export_kwargs={"compact": True, "max_frontier_actions": 6},
    )

    with TRANSCRIPT.open("w", encoding="utf-8") as handle:
        handle.write("## CodeGraph Python query façade demo\n\n")
        handle.write(
            "This transcript demonstrates agent-style repository querying through the thin Python façade and query runner, "
            "using regex discovery, loops, branch-and-compare, and targeted hydration.\n"
        )
        record(handle, "Raw CodeGraph summary", {"nodes": len(raw_graph.to_document().blocks), "repr": repr(raw_graph)})
        record(handle, "Facade graph.find(...) seed candidates", graph.find(node_class="symbol", name_regex=TARGET_REGEX, limit=6))
        record(handle, "Python query runner result", run.as_dict())
        record(handle, "Final export", run.export)
        handle.write("\n## Final summary\n\n")
        handle.write(f"- ok: {run.ok}\n")
        handle.write(f"- selected nodes: {run.summary['selected']}\n")
        handle.write(f"- transcript: `{TRANSCRIPT}`\n")


if __name__ == "__main__":
    main()