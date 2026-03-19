from __future__ import annotations

import copy
import json
from dataclasses import dataclass
from typing import Any, Mapping, Optional

from .query import (
    BaseQueryGraph,
    BaseQuerySession,
    QueryLimits,
    _merge_limits,
    _wrap_session,
    query,
    run_python_query,
)

_PYTHON_QUERY_TOOL_DESCRIPTION = (
    "Execute a short Python query against a bound UCP Graph or CodeGraph. "
    "Use the prebound `graph` and `session` objects for traversal, then rely on normal Python "
    "for loops, conditionals, regex, and scoring. Return your final structured answer in a `result` variable."
)

_PYTHON_QUERY_TOOL_INPUT_SCHEMA = {
    "type": "object",
    "additionalProperties": False,
    "properties": {
        "code": {
            "type": "string",
            "description": "Python snippet to execute. Use `graph`, `session`, `re`, `json`, `math`, and `collections`.",
        },
        "bindings": {
            "type": "object",
            "description": "Optional parameters injected into the query environment.",
        },
        "include_export": {
            "type": "boolean",
            "description": "If true (default), include the final session export in the tool result. Defaults to true since agents need to see session mutations.",
            "default": True,
        },
        "export_kwargs": {
            "type": "object",
            "description": "Optional keyword arguments forwarded to session.export(...).",
        },
        "limits": {
            "type": "object",
            "description": "Optional execution guards for model-authored queries.",
            "properties": {
                "max_seconds": {"type": "number"},
                "max_operations": {"type": "integer"},
                "max_trace_events": {"type": "integer"},
                "max_stdout_chars": {"type": "integer"},
            },
        },
    },
    "required": ["code"],
}


@dataclass
class QueryToolResult:
    tool_name: str
    ok: bool
    payload: dict[str, Any]

    def as_text(self) -> str:
        return json.dumps(self.payload, indent=2, sort_keys=True)

    def as_openai_message(self, tool_call_id: str) -> dict[str, Any]:
        return {
            "role": "tool",
            "tool_call_id": tool_call_id,
            "content": self.as_text(),
        }

    def as_anthropic_tool_result(self, tool_use_id: str) -> dict[str, Any]:
        return {
            "type": "tool_result",
            "tool_use_id": tool_use_id,
            "is_error": not self.ok,
            "content": [{"type": "text", "text": self.as_text()}],
        }


class PythonQueryTool:
    def __init__(
        self,
        graph: BaseQueryGraph | Any,
        *,
        session: Optional[BaseQuerySession | Any] = None,
        name: str = "run_python_query",
        description: str = _PYTHON_QUERY_TOOL_DESCRIPTION,
        default_include_export: bool = True,
        default_export_kwargs: Optional[Mapping[str, Any]] = None,
        default_limits: Optional[QueryLimits | Mapping[str, Any]] = None,
    ):
        self.graph = query(graph)
        self.session = None if session is None else _wrap_session(session, self.graph)
        self.name = name
        self.description = description
        self.default_include_export = default_include_export
        self.default_export_kwargs = dict(default_export_kwargs or {})
        self.default_limits = default_limits

    @property
    def input_schema(self) -> dict[str, Any]:
        return copy.deepcopy(_PYTHON_QUERY_TOOL_INPUT_SCHEMA)

    def anthropic_tool(self) -> dict[str, Any]:
        return {
            "name": self.name,
            "description": self.description,
            "input_schema": self.input_schema,
        }

    def openai_tool(self) -> dict[str, Any]:
        return {
            "type": "function",
            "function": {
                "name": self.name,
                "description": self.description,
                "parameters": self.input_schema,
            },
        }

    def execute(
        self,
        arguments: Mapping[str, Any] | str,
        *,
        raise_on_error: bool = False,
    ) -> QueryToolResult:
        payload = _coerce_tool_arguments(arguments)
        run = run_python_query(
            self.graph,
            payload["code"],
            session=self.session,
            bindings=payload.get("bindings"),
            include_export=payload.get("include_export", self.default_include_export),
            export_kwargs={
                **self.default_export_kwargs,
                **dict(payload.get("export_kwargs") or {}),
            },
            limits=_merge_limits(self.default_limits, payload.get("limits")),
            raise_on_error=raise_on_error,
        )
        result_payload = {
            "tool_name": self.name,
            **run.as_dict(),
        }
        return QueryToolResult(tool_name=self.name, ok=run.ok, payload=result_payload)

    def execute_openai_tool_call(
        self,
        tool_call: Mapping[str, Any],
        *,
        raise_on_error: bool = False,
    ) -> dict[str, Any]:
        function = tool_call.get("function") or {}
        result = self.execute(
            function.get("arguments") or {}, raise_on_error=raise_on_error
        )
        return result.as_openai_message(tool_call_id=str(tool_call["id"]))

    def execute_anthropic_tool_use(
        self,
        tool_use: Mapping[str, Any],
        *,
        raise_on_error: bool = False,
    ) -> dict[str, Any]:
        result = self.execute(
            tool_use.get("input") or {}, raise_on_error=raise_on_error
        )
        return result.as_anthropic_tool_result(tool_use_id=str(tool_use["id"]))


def _coerce_tool_arguments(arguments: Mapping[str, Any] | str) -> dict[str, Any]:
    if isinstance(arguments, Mapping):
        return dict(arguments)
    if isinstance(arguments, str):
        stripped = arguments.strip()
        if stripped.startswith("{"):
            return dict(json.loads(stripped))
        return {"code": arguments}
    raise TypeError(f"Unsupported tool arguments: {type(arguments)!r}")
