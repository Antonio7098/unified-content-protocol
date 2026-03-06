#!/usr/bin/env python3
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TRANSCRIPT = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "artifacts/codegraph-context-demo-transcript.md"
TARGET_FILES = [
    "crates/ucp-cli/src/commands/codegraph.rs",
    "crates/ucp-cli/src/commands/agent.rs",
    "crates/ucp-cli/src/state.rs",
]
TARGET_SUFFIXES = {"::context_show", "::resolve_selector", "::get_session_mut", "::print_context_update"}
QUEUE_FILES = set(TARGET_FILES + ["crates/ucp-codegraph/src/context.rs"])
MAX_OUTPUT_LINES = 80


def write(text: str = "") -> None:
    TRANSCRIPT.parent.mkdir(parents=True, exist_ok=True)
    with TRANSCRIPT.open("a", encoding="utf-8") as handle:
        handle.write(text)


def run_step(title: str, *cmd: str) -> str:
    write(f"\n## {title}\n\n`$ {' '.join(cmd)}`\n\n")
    completed = subprocess.run(cmd, cwd=ROOT, text=True, capture_output=True)
    if completed.stdout:
        write(f"```text\n{clip_output(completed.stdout)}```\n")
    if completed.stderr:
        write(f"```text\n{clip_output(completed.stderr)}```\n")
    if completed.returncode != 0:
        raise SystemExit(completed.returncode)
    return completed.stdout


def cli(*args: str) -> list[str]:
    return ["cargo", "run", "-q", "-p", "ucp-cli", "--", *args]


def parse_json(text: str) -> dict:
    return json.loads(text)


def clip_output(text: str) -> str:
    lines = text.splitlines()
    if len(lines) <= MAX_OUTPUT_LINES:
        return text
    clipped = "\n".join(lines[:MAX_OUTPUT_LINES])
    return f"{clipped}\n... clipped {len(lines) - MAX_OUTPUT_LINES} more lines ...\n"


def read_excerpt(path: Path, start: int | None, end: int | None, padding: int = 2) -> str:
    lines = path.read_text(encoding="utf-8").splitlines()
    first = max(1, (start or 1) - padding)
    last = min(len(lines), (end or start or 1) + padding)
    return "\n".join(f"{idx + 1:>4} {lines[idx]}" for idx in range(first - 1, last))


def seed_symbols(export: dict) -> list[str]:
    out = []
    for node in export.get("nodes", []):
        logical_key = node.get("logical_key")
        if not logical_key or node.get("node_class") != "symbol":
            continue
        if any(logical_key == f"symbol:{path}{suffix}" for path in TARGET_FILES for suffix in TARGET_SUFFIXES):
            out.append(logical_key)
    return sorted(set(out))


def queueable_symbols(export: dict, seen: set[str], queued: set[str]) -> list[str]:
    discovered = []
    for node in export.get("nodes", []):
        logical_key = node.get("logical_key")
        coderef = node.get("coderef") or {}
        origin = (node.get("origin") or {}).get("kind")
        if not logical_key or node.get("node_class") != "symbol":
            continue
        if logical_key in seen or logical_key in queued:
            continue
        if coderef.get("path") not in QUEUE_FILES:
            continue
        if origin not in {"dependencies", "dependents", "file_symbols", "manual"}:
            continue
        discovered.append(logical_key)
    return sorted(discovered)


def main() -> None:
    if TRANSCRIPT.exists():
        TRANSCRIPT.unlink()
    commit = subprocess.run(
        ["git", "rev-parse", "--short", "HEAD"], cwd=ROOT, text=True, capture_output=True
    ).stdout.strip() or "demo"
    write("## Codegraph context demo transcript\n\n")
    write("Chosen refactor candidate: deduplicate codegraph context/session helper logic across `agent.rs` and `codegraph.rs`.\n")
    with tempfile.TemporaryDirectory(prefix="ucp-codegraph-demo-") as tmp:
        doc = Path(tmp) / "ucp-codegraph.json"
        build_out = run_step(
            "Build a codegraph for the current repository",
            *cli("codegraph", "build", str(ROOT), "--commit", commit, "--output", str(doc), "--allow-partial", "--format", "json"),
        )
        _ = parse_json(build_out)
        init_out = run_step(
            "Initialize a stateful codegraph context session from the root overview",
            *cli("codegraph", "context", "init", "--input", str(doc), "--name", "demo_context_walk", "--max-selected", "512", "--format", "json"),
        )
        session_id = parse_json(init_out)["session_id"]
        write(f"\nSession: `{session_id}`\n")
        run_step("Show the initial root working set", *cli("codegraph", "context", "show", "--input", str(doc), "--session", session_id, "--format", "json"))
        for path in TARGET_FILES:
            run_step(
                f"Expand file symbols for {path}",
                *cli("codegraph", "context", "expand", "--input", str(doc), "--session", session_id, path, "--mode", "file", "--format", "json"),
            )
        export = parse_json(
            run_step("Export the structured working set after file expansion", *cli("codegraph", "context", "export", "--input", str(doc), "--session", session_id, "--format", "json"))
        )
        seeds = seed_symbols(export)
        write("\n### Seed symbols\n\n")
        for symbol in seeds:
            write(f"- `{symbol}`\n")

        queue = list(seeds)
        seen: set[str] = set()
        while queue:
            symbol = queue.pop(0)
            if symbol in seen:
                continue
            seen.add(symbol)
            run_step(
                f"Focus {symbol}",
                *cli("codegraph", "context", "focus", "--input", str(doc), "--session", session_id, symbol, "--format", "json"),
            )
            frontier_export = parse_json(
                run_step(f"Export frontier for {symbol}", *cli("codegraph", "context", "export", "--input", str(doc), "--session", session_id, "--format", "json"))
            )
            for action in frontier_export.get("frontier", []):
                if action.get("candidate_count", 0) == 0:
                    continue
                if action["action"] == "expand_dependencies":
                    run_step(
                        f"Expand dependencies for {symbol} via {action['relation']}",
                        *cli("codegraph", "context", "expand", "--input", str(doc), "--session", session_id, symbol, "--mode", "dependencies", "--relation", action["relation"], "--format", "json"),
                    )
                elif action["action"] == "expand_dependents":
                    run_step(
                        f"Expand dependents for {symbol} via {action['relation']}",
                        *cli("codegraph", "context", "expand", "--input", str(doc), "--session", session_id, symbol, "--mode", "dependents", "--relation", action["relation"], "--format", "json"),
                    )
            run_step(
                f"Hydrate source for {symbol}",
                *cli("codegraph", "context", "hydrate", "--input", str(doc), "--session", session_id, symbol, "--padding", "2", "--format", "json"),
            )
            updated = parse_json(
                run_step(f"Export updated working set for {symbol}", *cli("codegraph", "context", "export", "--input", str(doc), "--session", session_id, "--format", "json"))
            )
            queued = set(queue)
            queue.extend(queueable_symbols(updated, seen, queued))

        final_export = parse_json(
            run_step("Export the final structured context", *cli("codegraph", "context", "export", "--input", str(doc), "--session", session_id, "--format", "json"))
        )
        write("\n## Read coderef-backed excerpts from the final working set\n\n")
        seen_refs: set[tuple[str, int | None, int | None, str]] = set()
        for node in final_export.get("nodes", []):
            coderef = node.get("coderef")
            logical_key = node.get("logical_key") or node.get("label") or node["block_id"]
            if not coderef or coderef.get("path") not in QUEUE_FILES:
                continue
            if node.get("node_class") == "symbol" and logical_key not in seen:
                continue
            key = (coderef["path"], coderef.get("start_line"), coderef.get("end_line"), logical_key)
            if key in seen_refs:
                continue
            seen_refs.add(key)
            path = ROOT / coderef["path"]
            if not path.is_file():
                continue
            excerpt = read_excerpt(path, coderef.get("start_line"), coderef.get("end_line"))
            write(f"### {node['short_id']} `{logical_key}`\n\n")
            write(f"- ref: `{coderef['path']}:{coderef.get('start_line')}-{coderef.get('end_line')}`\n\n")
            write(f"```rust\n{excerpt}\n```\n")
        write("\n## Final summary\n\n")
        write(f"- selected nodes: {final_export['summary']['selected']}\n")
        write(f"- frontier actions remaining: {len(final_export.get('frontier', []))}\n")
        write(f"- transcript file: `{TRANSCRIPT}`\n")


if __name__ == "__main__":
    main()