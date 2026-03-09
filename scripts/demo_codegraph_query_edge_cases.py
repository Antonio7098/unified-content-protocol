#!/usr/bin/env python3
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PYTHON_SRC = ROOT / "crates" / "ucp-python" / "python"
ARTIFACTS = ROOT / "artifacts"
TRANSCRIPT = Path(sys.argv[1]) if len(sys.argv) > 1 else ARTIFACTS / "codegraph-query-edge-cases-transcript.md"

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
    raw_session = raw_graph.session()

    runs = {
        "Indented triple-quoted snippet": ucp.run_python_query(
            graph,
            """
                hits = graph.find(node_class="symbol", name_regex=r"^context_show$", limit=3)
                result = next(node["logical_key"] for node in hits)
            """,
        ).as_dict(),
        "Common builtins like type()": ucp.run_python_query(
            graph,
            """
                hit = graph.find(node_class="symbol", name_regex=r"^context_show$", limit=1)[0]
                result = {"python_type": type(hit).__name__, "has_logical_key": "logical_key" in hit}
            """,
        ).as_dict(),
        "Parameterized regex via bindings": ucp.run_python_query(
            graph,
            """
                hits = graph.find(node_class="symbol", path_regex=path_rx, name_regex=name_rx, limit=4)
                session.add(hits[0], detail="summary")
                result = {"count": len(hits), "first": hits[0]["logical_key"]}
            """,
            bindings={
                "path_rx": r"crates/ucp-cli/src/commands/(agent|codegraph)\.rs",
                "name_rx": r"context_show|get_session_mut",
            },
        ).as_dict(),
        "Raw session compatibility": ucp.run_python_query(
            raw_graph,
            """
                hits = graph.find(node_class="symbol", name_regex=r"^context_show$", limit=2)
                session.add(hits[0], detail="summary")
                result = session.summary()["selected"]
            """,
            session=raw_session,
        ).as_dict(),
    }

    with TRANSCRIPT.open("w", encoding="utf-8") as handle:
        handle.write("## CodeGraph query runner edge-case transcript\n\n")
        handle.write(
            "This transcript exercises common ergonomic edge cases for model-authored Python queries against the UCP repo CodeGraph.\n"
        )
        for title, payload in runs.items():
            record(handle, title, payload)


if __name__ == "__main__":
    main()