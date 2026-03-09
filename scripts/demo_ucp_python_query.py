#!/usr/bin/env python3
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PYTHON_SRC = ROOT / "crates" / "ucp-python" / "python"
ARTIFACTS = ROOT / "artifacts"
TRANSCRIPT = Path(sys.argv[1]) if len(sys.argv) > 1 else ARTIFACTS / "ucp-python-query-demo-transcript.md"

sys.path.insert(0, str(PYTHON_SRC))

try:
    import ucp
except ImportError as exc:  # pragma: no cover
    raise SystemExit("Build the local ucp Python extension first.") from exc


def build_document():
    doc = ucp.create("Python query demo")
    section = doc.add_block(doc.root_id, "Section", role="section", label="section")
    note = doc.add_block(section, "Important note", role="paragraph", label="note", tags=["important"])
    helper = doc.add_code(section, "rust", "fn helper() -> i32 { 1 }", label="helper")
    doc.add_edge(note, ucp.EdgeType.References, helper)
    return doc


def record(handle, title, payload):
    rendered = payload if isinstance(payload, str) else json.dumps(payload, indent=2, sort_keys=True)
    handle.write(f"\n## {title}\n\n```json\n{rendered}\n```\n")


def main():
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    if TRANSCRIPT.exists():
        TRANSCRIPT.unlink()

    raw_graph = ucp.Graph.from_document(build_document())
    graph = ucp.query(raw_graph)
    session = graph.session()

    run = session.run(
        """
hits = graph.find(label_regex="NOTE|HELPER", case_sensitive=False)
for node in hits:
    if re.search("note", node["label"], re.I):
        session.add(node, detail="full")
        session.walk(node, mode="outgoing", depth=1)
        break
result = {
    "labels": [node["label"] for node in hits],
    "why_helper": session.why("helper"),
    "path": session.path("root", "helper", max_hops=3),
}
""",
        include_export=True,
    )

    with TRANSCRIPT.open("w", encoding="utf-8") as handle:
        handle.write("## UCP Python query façade demo\n\n")
        handle.write(
            "This transcript demonstrates the thin agent-facing Python façade over the generic graph runtime, "
            "using loops, regex, and conditional traversal without a separate graph DSL.\n"
        )
        record(handle, "Raw graph stats", raw_graph.store_stats())
        record(handle, "Facade graph.find(...) results", graph.find(label_regex="note|helper"))
        record(handle, "Query runner result", run.as_dict())
        record(handle, "Final session export", run.export)
        handle.write("\n## Final summary\n\n")
        handle.write(f"- ok: {run.ok}\n")
        handle.write(f"- selected nodes: {run.summary['selected']}\n")
        handle.write(f"- transcript: `{TRANSCRIPT}`\n")


if __name__ == "__main__":
    main()