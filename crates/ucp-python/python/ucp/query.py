from __future__ import annotations

import collections
import contextlib
import io
import json
import math
import re
import textwrap
import traceback as traceback_module
from dataclasses import dataclass, field
from typing import Any, Mapping, Optional

from ._core import CodeGraph, CodeGraphSession, Graph, GraphSession

_SAFE_BUILTINS = {
    "abs": abs,
    "all": all,
    "any": any,
    "AssertionError": AssertionError,
    "bool": bool,
    "callable": callable,
    "dict": dict,
    "enumerate": enumerate,
    "Exception": Exception,
    "filter": filter,
    "float": float,
    "getattr": getattr,
    "hasattr": hasattr,
    "int": int,
    "isinstance": isinstance,
    "iter": iter,
    "len": len,
    "list": list,
    "map": map,
    "max": max,
    "min": min,
    "next": next,
    "print": print,
    "range": range,
    "repr": repr,
    "reversed": reversed,
    "round": round,
    "RuntimeError": RuntimeError,
    "set": set,
    "sorted": sorted,
    "str": str,
    "sum": sum,
    "tuple": tuple,
    "type": type,
    "TypeError": TypeError,
    "ValueError": ValueError,
    "zip": zip,
}

_GRAPH_DETAIL_ALIASES = {
    "stub": "stub",
    "summary": "summary",
    "full": "full",
}

_CODEGRAPH_DETAIL_ALIASES = {
    "stub": "skeleton",
    "skeleton": "skeleton",
    "summary": "symbol_card",
    "card": "symbol_card",
    "symbol-card": "symbol_card",
    "symbol_card": "symbol_card",
    "full": "source",
    "source": "source",
    "neighborhood": "neighborhood",
}


class QueryExecutionError(RuntimeError):
    def __init__(self, error_type: str, message: str, traceback_text: str):
        super().__init__(f"{error_type}: {message}")
        self.error_type = error_type
        self.traceback = traceback_text


@dataclass
class QueryRunResult:
    ok: bool
    result: Any
    stdout: str
    summary: dict[str, Any]
    selected_block_ids: list[str]
    graph: "BaseQueryGraph" = field(repr=False)
    session: "BaseQuerySession" = field(repr=False)
    export: Optional[dict[str, Any]] = None
    error_type: Optional[str] = None
    error_message: Optional[str] = None
    traceback: Optional[str] = None

    def as_dict(self) -> dict[str, Any]:
        return {
            "ok": self.ok,
            "result": self.result,
            "stdout": self.stdout,
            "summary": self.summary,
            "selected_block_ids": list(self.selected_block_ids),
            "export": self.export,
            "error": None
            if self.ok
            else {
                "type": self.error_type,
                "message": self.error_message,
                "traceback": self.traceback,
            },
        }


class BaseQueryGraph:
    def __init__(self, raw: Graph | CodeGraph):
        self.raw = raw

    def __getattr__(self, name: str) -> Any:
        return getattr(self.raw, name)

    def resolve(self, selector: Any) -> Any:
        return self.raw.resolve(_selector_value(selector))

    def describe(self, selector: Any) -> Any:
        return self.raw.describe(_selector_value(selector))

    def find(self, **filters: Any) -> Any:
        return self.raw.find_nodes(**filters)

    def path(self, start: Any, end: Any, max_hops: int = 8) -> Any:
        return self.raw.path_between(
            _selector_value(start), _selector_value(end), max_hops=max_hops
        )

    def run(self, code: str, **kwargs: Any) -> QueryRunResult:
        return run_python_query(self, code, **kwargs)


class QueryGraph(BaseQueryGraph):
    def session(self) -> "QuerySession":
        return QuerySession(self.raw.session(), self)


class CodeQueryGraph(BaseQueryGraph):
    def session(self) -> "CodeQuerySession":
        return CodeQuerySession(self.raw.session(), self)


class BaseQuerySession:
    default_detail = "summary"
    default_walk_mode = "neighborhood"

    def __init__(self, raw: GraphSession | CodeGraphSession, graph: BaseQueryGraph):
        self.raw = raw
        self.graph = graph

    def __getattr__(self, name: str) -> Any:
        return getattr(self.raw, name)

    def summary(self) -> Any:
        return self.raw.summary()

    def selected_block_ids(self) -> Any:
        return self.raw.selected_block_ids()

    def find(self, **filters: Any) -> Any:
        return self.graph.find(**filters)

    def add(self, target: Any, detail: Optional[str] = None) -> Any:
        return self.raw.select(
            _selector_value(target), detail_level=self._normalize_detail(detail)
        )

    def focus(self, target: Optional[Any] = None) -> Any:
        return self.raw.focus(None if target is None else _selector_value(target))

    def why(self, target: Any) -> Any:
        return self.raw.why_selected(_selector_value(target))

    def path(self, start: Any, end: Any, max_hops: int = 8) -> Any:
        return self.graph.path(start, end, max_hops=max_hops)

    def export(self, **kwargs: Any) -> Any:
        return self.raw.export(**kwargs)

    def fork(self) -> "BaseQuerySession":
        return type(self)(self.raw.fork(), self.graph)

    def diff(self, other: "BaseQuerySession") -> Any:
        return self.raw.diff(_wrap_session(other, self.graph).raw)

    def run(self, code: str, **kwargs: Any) -> QueryRunResult:
        return run_python_query(self.graph, code, session=self, **kwargs)

    def _normalize_detail(self, detail: Optional[str]) -> str:
        raise NotImplementedError


class QuerySession(BaseQuerySession):
    def _normalize_detail(self, detail: Optional[str]) -> str:
        value = (detail or self.default_detail).lower().replace("-", "_")
        return _GRAPH_DETAIL_ALIASES.get(value, detail or self.default_detail)

    def walk(
        self,
        target: Any,
        mode: str = "neighborhood",
        depth: int = 1,
        limit: Optional[int] = None,
    ) -> Any:
        return self.raw.expand(
            _selector_value(target),
            mode=mode,
            depth=depth,
            max_add=limit,
        )


class CodeQuerySession(BaseQuerySession):
    default_walk_mode = "dependencies"

    def _normalize_detail(self, detail: Optional[str]) -> str:
        value = (detail or self.default_detail).lower().replace("-", "_")
        return _CODEGRAPH_DETAIL_ALIASES.get(value, detail or self.default_detail)

    def walk(
        self,
        target: Any,
        mode: str = "dependencies",
        depth: int = 1,
        limit: Optional[int] = None,
        relation: Optional[str] = None,
        relations: Optional[list[str]] = None,
        priority_threshold: Optional[int] = None,
    ) -> Any:
        return self.raw.expand(
            _selector_value(target),
            mode=mode,
            relation=relation,
            relations=relations,
            depth=depth,
            max_add=limit,
            priority_threshold=priority_threshold,
        )

    def hydrate(self, target: Any, padding: int = 2) -> Any:
        return self.raw.hydrate(_selector_value(target), padding=padding)


def query(graph: Graph | CodeGraph | BaseQueryGraph) -> BaseQueryGraph:
    if isinstance(graph, BaseQueryGraph):
        return graph
    if isinstance(graph, CodeGraph):
        return CodeQueryGraph(graph)
    if isinstance(graph, Graph):
        return QueryGraph(graph)
    raise TypeError(f"Unsupported graph object: {type(graph)!r}")


def run_python_query(
    graph: Graph | CodeGraph | BaseQueryGraph,
    code: str,
    *,
    session: Optional[BaseQuerySession] = None,
    bindings: Optional[Mapping[str, Any]] = None,
    include_export: bool = False,
    export_kwargs: Optional[Mapping[str, Any]] = None,
    raise_on_error: bool = False,
) -> QueryRunResult:
    wrapped_graph = query(graph)
    wrapped_session = _wrap_session(session, wrapped_graph)
    env = {
        "__builtins__": _SAFE_BUILTINS,
        "__name__": "__ucp_query__",
        "collections": collections,
        "graph": wrapped_graph,
        "json": json,
        "math": math,
        "raw_graph": wrapped_graph.raw,
        "raw_session": wrapped_session.raw,
        "re": re,
        "session": wrapped_session,
    }
    if bindings:
        env.update(dict(bindings))
    stdout = io.StringIO()

    try:
        kind, compiled = _compile_query(code)
        with contextlib.redirect_stdout(stdout):
            if kind == "eval":
                result = eval(compiled, env, env)
            else:
                exec(compiled, env, env)
                result = env.get("result")
        return _query_result(
            wrapped_graph,
            wrapped_session,
            stdout.getvalue(),
            result,
            include_export,
            export_kwargs,
        )
    except Exception as exc:
        tb = traceback_module.format_exc()
        if raise_on_error:
            raise QueryExecutionError(type(exc).__name__, str(exc), tb) from exc
        return _query_result(
            wrapped_graph,
            wrapped_session,
            stdout.getvalue(),
            None,
            include_export,
            export_kwargs,
            error=exc,
            traceback_text=tb,
        )


def _compile_query(code: str) -> tuple[str, Any]:
    source = textwrap.dedent(code).strip()
    try:
        return "eval", compile(source, "<ucp-python-query>", "eval")
    except SyntaxError:
        return "exec", compile(source, "<ucp-python-query>", "exec")


def _wrap_session(
    session: Optional[BaseQuerySession | GraphSession | CodeGraphSession],
    graph: BaseQueryGraph,
) -> BaseQuerySession:
    if session is None:
        return graph.session()
    if isinstance(session, BaseQuerySession):
        return session
    if isinstance(session, CodeGraphSession):
        return CodeQuerySession(session, graph)
    if isinstance(session, GraphSession):
        return QuerySession(session, graph)
    raise TypeError(f"Unsupported session object: {type(session)!r}")


def _query_result(
    graph: BaseQueryGraph,
    session: BaseQuerySession,
    stdout: str,
    result: Any,
    include_export: bool,
    export_kwargs: Optional[Mapping[str, Any]],
    *,
    error: Optional[Exception] = None,
    traceback_text: Optional[str] = None,
) -> QueryRunResult:
    export = session.export(**dict(export_kwargs or {})) if include_export else None
    return QueryRunResult(
        ok=error is None,
        result=result,
        stdout=stdout,
        summary=session.summary(),
        selected_block_ids=[str(block_id) for block_id in session.selected_block_ids()],
        graph=graph,
        session=session,
        export=export,
        error_type=None if error is None else type(error).__name__,
        error_message=None if error is None else str(error),
        traceback=traceback_text,
    )


def _selector_value(value: Any) -> Any:
    if isinstance(value, dict):
        for key in ("logical_key", "block_id", "label", "path"):
            candidate = value.get(key)
            if candidate:
                return candidate
    return value