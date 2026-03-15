#!/usr/bin/env python3
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PYTHON_SRC = ROOT / "crates" / "ucp-python" / "python"
ARTIFACTS = ROOT / "artifacts"
TRANSCRIPT = (
    Path(sys.argv[1])
    if len(sys.argv) > 1
    else ARTIFACTS / "codegraph-session-observability-demo-transcript.md"
)

sys.path.insert(0, str(PYTHON_SRC))

try:
    import ucp
except ImportError as exc:  # pragma: no cover
    raise SystemExit("Build the local ucp Python extension first.") from exc


def record(handle, title, payload):
    rendered = (
        payload
        if isinstance(payload, str)
        else json.dumps(payload, indent=2, sort_keys=True, default=str)
    )
    handle.write(f"\n## {title}\n\n```json\n{rendered}\n```\n")


def main():
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    if TRANSCRIPT.exists():
        TRANSCRIPT.unlink()

    graph = ucp.CodeGraph.build(str(ROOT), continue_on_parse_error=True)
    session = graph.session()
    session.seed_overview(max_depth=3)
    expand_file = session.expand(
        "crates/ucp-python/src/codegraph.rs", mode="file", max_nodes_visited=16
    )
    selector = graph.explain_selector("crates/ucp-python/src/codegraph.rs")
    estimate = session.estimate_expand(
        "symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
        mode="dependencies",
        depth=1,
        max_nodes_visited=8,
    )
    dependency_walk = session.expand(
        "symbol:crates/ucp-python/src/codegraph.rs::build_traversal",
        mode="dependencies",
        depth=1,
    )
    export = session.export(compact=True, visible_levels=0, max_frontier_actions=4)
    omission = session.explain_export_omission(
        "symbol:crates/ucp-python/src/codegraph.rs::render_config",
        compact=True,
        visible_levels=0,
    )
    session.prune(max_selected=8)
    pruned = session.why_pruned(
        "symbol:crates/ucp-python/src/codegraph.rs::render_config"
    )
    recommendations = session.recommendations(top=3)

    session_path = ARTIFACTS / "codegraph-session-observability-demo.json"
    session.save(str(session_path))
    restored = graph.load_session(str(session_path))

    with TRANSCRIPT.open("w", encoding="utf-8") as handle:
        handle.write("## CodeGraph Session Observability Demo\n\n")
        handle.write(
            "This transcript exercises selector explanations, mutation telemetry, recommendations, "
            "pre-mutation estimates, omission reporting, prune explanations, and session persistence.\n"
        )
        record(handle, "Selector Explanation", selector)
        record(handle, "Expand File Update", expand_file)
        record(handle, "Expansion Estimate", estimate)
        record(handle, "Dependency Walk Update", dependency_walk)
        record(handle, "Compact Export", export)
        record(handle, "Export Omission Explanation", omission)
        record(handle, "Prune Explanation", pruned)
        record(handle, "Recommendations", recommendations)
        record(handle, "Mutation Log", session.mutation_log())
        record(handle, "Event Log", session.event_log())
        record(
            handle,
            "Restored Session Summary",
            {
                "session_id": restored.session_id(),
                "selected_block_ids": restored.selected_block_ids(),
                "summary": restored.summary(),
            },
        )
        handle.write("\n## Final summary\n\n")
        handle.write(f"- session id: `{session.session_id()}`\n")
        handle.write(f"- restored session id: `{restored.session_id()}`\n")
        handle.write(f"- transcript: `{TRANSCRIPT}`\n")


if __name__ == "__main__":
    main()
