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
    else ARTIFACTS / "ucp-graph-runtime-demo-transcript.md"
)
JSON_PATH = ARTIFACTS / "ucp-graph-runtime-demo.json"
SQLITE_PATH = ARTIFACTS / "ucp-graph-runtime-demo.db"

sys.path.insert(0, str(PYTHON_SRC))

try:
    import ucp
except ImportError as exc:  # pragma: no cover
    raise SystemExit("Build the local ucp Python extension first.") from exc


def record(handle, title, payload):
    if isinstance(payload, str):
        rendered = payload
    else:
        rendered = json.dumps(payload, indent=2, sort_keys=True)
    handle.write(f"\n## {title}\n\n```json\n{rendered}\n```\n")


def build_document():
    doc = ucp.create("Generic graph runtime demo")
    section = doc.add_block(doc.root_id, "Section", role="section", label="section")
    note = doc.add_block(
        section,
        "Important note about helper usage",
        role="paragraph",
        label="note",
        tags=["important", "demo"],
    )
    helper = doc.add_code(section, "rust", "fn helper() -> i32 { 1 }", label="helper")
    doc.add_edge(note, ucp.EdgeType.References, helper)
    return doc


def main():
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    if TRANSCRIPT.exists():
        TRANSCRIPT.unlink()
    if SQLITE_PATH.exists():
        SQLITE_PATH.unlink()

    doc = build_document()
    graph = ucp.Graph.from_document(doc)
    graph.save(str(JSON_PATH))
    graph.persist_sqlite(str(SQLITE_PATH), "demo")
    reopened = ucp.Graph.from_sqlite(str(SQLITE_PATH), "demo")

    session = reopened.session()
    seeded = session.seed_overview(max_depth=1)
    selected = session.select("note", detail_level="full")
    expanded = session.expand("note", mode="outgoing", depth=1)
    why = session.why_selected("helper")
    exported = session.export()

    with TRANSCRIPT.open("w", encoding="utf-8") as handle:
        handle.write("## UCP graph runtime demo transcript\n\n")
        handle.write(
            "This transcript demonstrates generic graph traversal over a plain UCP document, "
            "plus JSON-backed and SQLite-backed graph persistence.\n"
        )
        record(handle, "In-memory graph stats", graph.store_stats())
        record(handle, "SQLite graph observability", reopened.observability())
        record(
            handle, "Regex graph search", reopened.find_nodes(label_regex="note|helper")
        )
        record(
            handle,
            "Path between note and helper",
            reopened.path_between("note", "helper", max_hops=3),
        )
        record(handle, "Seed overview", seeded)
        record(handle, "Select note", selected)
        record(handle, "Expand outgoing edges from note", expanded)
        record(handle, "Why helper is selected", why)
        record(handle, "Exported working set", exported)

        handle.write("\n## Final summary\n\n")
        handle.write(f"- JSON artifact: `{JSON_PATH}`\n")
        handle.write(f"- SQLite artifact: `{SQLITE_PATH}`\n")
        handle.write(f"- selected nodes: {exported['summary']['selected']}\n")
        handle.write(f"- transcript: `{TRANSCRIPT}`\n")


if __name__ == "__main__":
    main()
