#!/usr/bin/env python3
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PYTHON_SRC = ROOT / "crates" / "ucp-python" / "python"
TRANSCRIPT = (
    Path(sys.argv[1])
    if len(sys.argv) > 1
    else ROOT / "artifacts/codegraph-context-demo-transcript.md"
)
TARGET_FILES = {
    "crates/ucp-cli/src/commands/codegraph.rs",
    "crates/ucp-cli/src/commands/agent.rs",
    "crates/ucp-codegraph/src/context.rs",
}
SEED_NAME_REGEX = (
    "context_show|get_session_mut|print_context_update|resolve_codegraph_selector"
)
MAX_OUTPUT_LINES = 80

sys.path.insert(0, str(PYTHON_SRC))

try:
    import ucp
except ImportError as exc:  # pragma: no cover - manual demo helper
    raise SystemExit(
        "Could not import the local `ucp` Python package. Build the extension first, for example:\n"
        "  CARGO_TARGET_DIR=/tmp/ucp-python-ext cargo build -p ucp-python --features pyo3/extension-module\n"
        "and place the resulting shared library in crates/ucp-python/python/ucp/."
    ) from exc


def write(text: str = "") -> None:
    TRANSCRIPT.parent.mkdir(parents=True, exist_ok=True)
    with TRANSCRIPT.open("a", encoding="utf-8") as handle:
        handle.write(text)


def clip(text: str) -> str:
    lines = text.splitlines()
    if len(lines) <= MAX_OUTPUT_LINES:
        return text
    return (
        "\n".join(lines[:MAX_OUTPUT_LINES])
        + f"\n... clipped {len(lines) - MAX_OUTPUT_LINES} more lines ...\n"
    )


def record(title: str, payload) -> None:
    rendered = (
        json.dumps(payload, indent=2, sort_keys=True)
        if not isinstance(payload, str)
        else payload
    )
    write(f"\n## {title}\n\n```json\n{clip(rendered)}\n```\n")


def read_excerpt(
    path: Path, start: int | None, end: int | None, padding: int = 2
) -> str:
    lines = path.read_text(encoding="utf-8").splitlines()
    first = max(1, (start or 1) - padding)
    last = min(len(lines), (end or start or 1) + padding)
    return "\n".join(f"{idx + 1:>4} {lines[idx]}" for idx in range(first - 1, last))


def main() -> None:
    if TRANSCRIPT.exists():
        TRANSCRIPT.unlink()

    write("## Codegraph programmatic API demo transcript\n\n")
    write(
        "This transcript manually exercises the new Python CodeGraph API for agent-style traversal,\n"
        "including regex discovery, stateful expansion, provenance inspection, frontier-driven actions,\n"
        "path finding, and coderef-backed source hydration.\n"
    )

    graph = ucp.CodeGraph.build(str(ROOT), continue_on_parse_error=True)
    record(
        "Build repository graph",
        {"nodes": len(graph.to_document().blocks), "repr": repr(graph)},
    )

    session = graph.session()
    record("Seed overview", session.seed_overview(max_depth=3))

    for path in sorted(TARGET_FILES):
        record(
            f"Expand file symbols for {path}",
            session.expand(path, mode="file", depth=2),
        )

    seeds = graph.find_nodes(
        node_class="symbol",
        path_regex="crates/ucp-cli/src/commands/codegraph\\.rs|crates/ucp-cli/src/commands/agent\\.rs|crates/ucp-codegraph/src/context\\.rs",
        name_regex=SEED_NAME_REGEX,
        limit=6,
    )
    record("Find regex-matched seed symbols", seeds)
    if not seeds:
        raise SystemExit("No seed symbols found for the demo workflow")

    branch = session.fork()
    for node in seeds[:3]:
        logical_key = node["logical_key"]
        record(f"Focus {logical_key}", branch.focus(logical_key))
        record(
            f"Expand dependencies for {logical_key}",
            branch.expand(logical_key, mode="dependencies", depth=1, max_add=8),
        )
        record(f"Why is {logical_key} selected?", branch.why_selected(logical_key))
        record(f"Hydrate {logical_key}", branch.hydrate(logical_key, padding=2))
        try:
            record(
                f"Apply recommended action near {logical_key}",
                branch.apply_recommended(top=1, padding=2),
            )
        except RuntimeError as exc:
            record(
                f"Apply recommended action near {logical_key}", {"warning": str(exc)}
            )

    if len(seeds) >= 2:
        path = branch.path_between(
            seeds[0]["logical_key"], seeds[1]["logical_key"], max_hops=8
        )
        record("Path between the first two seed symbols", path)

    diff = session.diff(branch)
    record("Diff between base session and exploration branch", diff)

    exported = branch.export(
        compact=True, include_rendered=False, max_frontier_actions=8
    )
    record("Compact structured export from the exploration branch", exported)

    write("\n## Read coderef-backed excerpts from the final working set\n")
    emitted = 0
    for node in exported.get("nodes", []):
        coderef = node.get("coderef") or {}
        path = coderef.get("path")
        if path not in TARGET_FILES:
            continue
        excerpt_path = ROOT / path
        if not excerpt_path.is_file():
            continue
        excerpt = read_excerpt(
            excerpt_path, coderef.get("start_line"), coderef.get("end_line")
        )
        write(
            f"\n### {node.get('short_id', node['block_id'])} `{node.get('logical_key') or node.get('label')}`\n\n"
        )
        write(
            f"- ref: `{path}:{coderef.get('start_line')}-{coderef.get('end_line')}`\n\n"
        )
        write(f"```rust\n{excerpt}\n```\n")
        emitted += 1
        if emitted >= 6:
            break

    write("\n## Final summary\n\n")
    write(f"- selected nodes: {exported['summary']['selected']}\n")
    write(f"- frontier actions remaining: {len(exported.get('frontier', []))}\n")
    write(f"- transcript file: `{TRANSCRIPT}`\n")


if __name__ == "__main__":
    main()
