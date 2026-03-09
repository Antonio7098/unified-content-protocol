#!/usr/bin/env python3
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PYTHON_SRC = ROOT / "crates" / "ucp-python" / "python"
ARTIFACTS = ROOT / "artifacts"
TRANSCRIPT = Path(sys.argv[1]) if len(sys.argv) > 1 else ARTIFACTS / "codegraph-query-tool-wrapper-transcript.md"

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
    tool = ucp.PythonQueryTool(
        graph,
        default_include_export=True,
        default_export_kwargs={"compact": True, "max_frontier_actions": 4},
        default_limits=ucp.QueryLimits(
            max_seconds=2.0,
            max_operations=40,
            max_trace_events=2000,
            max_stdout_chars=4000,
        ),
    )

    direct = tool.execute(
        {
            "code": """
                candidates = graph.find(
                    node_class='symbol',
                    path_regex=path_rx,
                    name_regex=r'context_show|get_session_mut',
                    limit=6,
                )
                session.add(candidates[0], detail='summary')
                session.walk(candidates[0], mode='dependencies', depth=1, limit=8)
                result = {
                    'first': candidates[0]['logical_key'],
                    'selected': session.export(compact=True)['summary']['selected'],
                }
            """,
            "bindings": {
                "path_rx": r"crates/ucp-cli/src/commands/(agent|codegraph)\.rs",
            },
        }
    )

    openai = tool.execute_openai_tool_call(
        {
            "id": "call_demo_1",
            "function": {
                "name": tool.name,
                "arguments": json.dumps(
                    {
                        "code": "target = graph.find(node_class='symbol', name_regex='^run_python_query$', limit=1)[0]\nsession.add(target, detail='summary')\nresult = target['logical_key']"
                    }
                ),
            },
        }
    )

    anthropic = tool.execute_anthropic_tool_use(
        {
            "id": "toolu_demo_1",
            "input": {
                "code": "target = graph.find(node_class='symbol', name_regex='^run_python_query$', limit=1)[0]\nsession.add(target, detail='summary')\nprint('hello from tool')\nresult = {'selected': session.summary()['selected']}"
            },
        }
    )

    with TRANSCRIPT.open("w", encoding="utf-8") as handle:
        handle.write("## CodeGraph provider-facing Python query tool demo\n\n")
        handle.write(
            "This transcript demonstrates the provider/tool-facing wrapper around `run_python_query(...)`, including OpenAI-style and Anthropic-style envelopes plus execution limits.\n"
        )
        record(handle, "OpenAI tool definition", tool.openai_tool())
        record(handle, "Anthropic tool definition", tool.anthropic_tool())
        record(handle, "Direct tool execution", direct.payload)
        record(handle, "OpenAI tool result message", openai)
        record(handle, "Anthropic tool result", anthropic)


if __name__ == "__main__":
    main()