from __future__ import annotations

import collections
import contextlib
import io
import json
import math
import re
import sys
import textwrap
import time
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


class QueryLimitExceededError(RuntimeError):
    def __init__(
        self,
        limit_name: str,
        limit_value: int | float,
        observed_value: int | float,
    ):
        super().__init__(
            f"Query exceeded {limit_name} limit: observed {observed_value!r}, limit {limit_value!r}"
        )
        self.limit_name = limit_name
        self.limit_value = limit_value
        self.observed_value = observed_value


@dataclass(frozen=True)
class QueryLimits:
    max_seconds: Optional[float] = None
    max_operations: Optional[int] = None
    max_trace_events: Optional[int] = None
    max_stdout_chars: Optional[int] = None

    def __post_init__(self) -> None:
        for name in (
            "max_seconds",
            "max_operations",
            "max_trace_events",
            "max_stdout_chars",
        ):
            value = getattr(self, name)
            if value is not None and value < 0:
                raise ValueError(f"{name} must be >= 0")

    def as_dict(self) -> dict[str, Any]:
        return {
            "max_seconds": self.max_seconds,
            "max_operations": self.max_operations,
            "max_trace_events": self.max_trace_events,
            "max_stdout_chars": self.max_stdout_chars,
        }


@dataclass(frozen=True)
class QueryUsage:
    elapsed_seconds: float
    operation_count: int
    trace_events: int
    stdout_chars: int

    def as_dict(self) -> dict[str, Any]:
        return {
            "elapsed_seconds": round(self.elapsed_seconds, 6),
            "operation_count": self.operation_count,
            "trace_events": self.trace_events,
            "stdout_chars": self.stdout_chars,
        }


@dataclass
class QueryRunResult:
    ok: bool
    result: Any
    stdout: str
    summary: dict[str, Any]
    selected_block_ids: list[str]
    usage: QueryUsage
    limits: QueryLimits
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
            "usage": self.usage.as_dict(),
            "limits": self.limits.as_dict(),
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
    def __init__(
        self,
        raw: Graph | CodeGraph,
        *,
        runtime_state: Optional["_QueryRuntimeState"] = None,
    ):
        self.raw = raw
        self._runtime_state = runtime_state

    def __getattr__(self, name: str) -> Any:
        return getattr(self.raw, name)

    def _with_state(self, runtime_state: Optional["_QueryRuntimeState"]) -> "BaseQueryGraph":
        return type(self)(self.raw, runtime_state=runtime_state)

    def _record_operation(self, name: str) -> None:
        if self._runtime_state is not None:
            self._runtime_state.record_operation(name)

    def resolve(self, selector: Any) -> Any:
        self._record_operation("graph.resolve")
        return self.raw.resolve(_selector_value(selector))

    def describe(self, selector: Any) -> Any:
        self._record_operation("graph.describe")
        return self.raw.describe(_selector_value(selector))

    def find(self, **filters: Any) -> Any:
        self._record_operation("graph.find")
        return self.raw.find_nodes(**filters)

    def path(self, start: Any, end: Any, max_hops: int = 8) -> Any:
        self._record_operation("graph.path")
        return self.raw.path_between(
            _selector_value(start), _selector_value(end), max_hops=max_hops
        )

    def run(self, code: str, **kwargs: Any) -> QueryRunResult:
        return run_python_query(self, code, **kwargs)


class QueryGraph(BaseQueryGraph):
    def session(self) -> "QuerySession":
        self._record_operation("graph.session")
        return QuerySession(self.raw.session(), self, runtime_state=self._runtime_state)


class CodeQueryGraph(BaseQueryGraph):
    def session(self) -> "CodeQuerySession":
        self._record_operation("graph.session")
        return CodeQuerySession(self.raw.session(), self, runtime_state=self._runtime_state)


class BaseQuerySession:
    default_detail = "summary"
    default_walk_mode = "neighborhood"

    def __init__(
        self,
        raw: GraphSession | CodeGraphSession,
        graph: BaseQueryGraph,
        *,
        runtime_state: Optional["_QueryRuntimeState"] = None,
    ):
        self.raw = raw
        self.graph = graph
        self._runtime_state = runtime_state or graph._runtime_state

    def __getattr__(self, name: str) -> Any:
        return getattr(self.raw, name)

    def _with_graph(self, graph: BaseQueryGraph) -> "BaseQuerySession":
        return type(self)(self.raw, graph, runtime_state=graph._runtime_state)

    def _record_operation(self, name: str) -> None:
        if self._runtime_state is not None:
            self._runtime_state.record_operation(name)

    def summary(self) -> Any:
        self._record_operation("session.summary")
        return self.raw.summary()

    def selected_block_ids(self) -> Any:
        self._record_operation("session.selected_block_ids")
        return self.raw.selected_block_ids()

    def find(self, **filters: Any) -> Any:
        return self.graph.find(**filters)

    def add(self, target: Any, detail: Optional[str] = None) -> Any:
        self._record_operation("session.add")
        return self.raw.select(
            _selector_value(target), detail_level=self._normalize_detail(detail)
        )

    def focus(self, target: Optional[Any] = None) -> Any:
        self._record_operation("session.focus")
        return self.raw.focus(None if target is None else _selector_value(target))

    def why(self, target: Any) -> Any:
        self._record_operation("session.why")
        return self.raw.why_selected(_selector_value(target))

    def path(self, start: Any, end: Any, max_hops: int = 8) -> Any:
        return self.graph.path(start, end, max_hops=max_hops)

    def export(self, **kwargs: Any) -> Any:
        self._record_operation("session.export")
        return self.raw.export(**kwargs)

    def fork(self) -> "BaseQuerySession":
        self._record_operation("session.fork")
        return type(self)(self.raw.fork(), self.graph, runtime_state=self._runtime_state)

    def diff(self, other: "BaseQuerySession") -> Any:
        self._record_operation("session.diff")
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
        self._record_operation("session.walk")
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
        self._record_operation("session.walk")
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
        self._record_operation("session.hydrate")
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
    session: Optional[BaseQuerySession | GraphSession | CodeGraphSession] = None,
    bindings: Optional[Mapping[str, Any]] = None,
    include_export: bool = False,
    export_kwargs: Optional[Mapping[str, Any]] = None,
    limits: Optional[QueryLimits | Mapping[str, Any]] = None,
    raise_on_error: bool = False,
) -> QueryRunResult:
    runtime_state = _QueryRuntimeState(_coerce_limits(limits))
    wrapped_graph = query(graph)._with_state(runtime_state)
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

    stdout = _GuardedStdout(runtime_state)
    result: Any = None
    error: Optional[Exception] = None
    traceback_text: Optional[str] = None
    previous_trace = sys.gettrace()

    try:
        kind, compiled = _compile_query(code)
        sys.settrace(runtime_state.trace)
        with contextlib.redirect_stdout(stdout):
            if kind == "eval":
                result = eval(compiled, env, env)
            else:
                exec(compiled, env, env)
                result = env.get("result")
        runtime_state.check_limits()
    except Exception as exc:  # pragma: no cover - exercised through result assertions
        error = exc
        traceback_text = traceback_module.format_exc()
    finally:
        sys.settrace(previous_trace)

    if error is not None and raise_on_error:
        raise QueryExecutionError(type(error).__name__, str(error), traceback_text or "") from error

    return _query_result(
        wrapped_graph,
        wrapped_session,
        stdout.getvalue(),
        result,
        include_export,
        export_kwargs,
        runtime_state=runtime_state,
        error=error,
        traceback_text=traceback_text,
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
        if isinstance(graph, CodeQueryGraph):
            return CodeQuerySession(graph.raw.session(), graph, runtime_state=graph._runtime_state)
        return QuerySession(graph.raw.session(), graph, runtime_state=graph._runtime_state)
    if isinstance(session, BaseQuerySession):
        return session._with_graph(graph)
    if isinstance(session, CodeGraphSession):
        return CodeQuerySession(session, graph, runtime_state=graph._runtime_state)
    if isinstance(session, GraphSession):
        return QuerySession(session, graph, runtime_state=graph._runtime_state)
    raise TypeError(f"Unsupported session object: {type(session)!r}")


def _query_result(
    graph: BaseQueryGraph,
    session: BaseQuerySession,
    stdout: str,
    result: Any,
    include_export: bool,
    export_kwargs: Optional[Mapping[str, Any]],
    runtime_state: "_QueryRuntimeState",
    *,
    error: Optional[Exception] = None,
    traceback_text: Optional[str] = None,
) -> QueryRunResult:
    export = session.raw.export(**dict(export_kwargs or {})) if include_export else None
    return QueryRunResult(
        ok=error is None,
        result=result,
        stdout=stdout,
        summary=session.raw.summary(),
        selected_block_ids=[str(block_id) for block_id in session.raw.selected_block_ids()],
        usage=runtime_state.usage(),
        limits=runtime_state.limits,
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


def _current_time() -> float:
    return time.perf_counter()


def _coerce_limits(limits: Optional[QueryLimits | Mapping[str, Any]]) -> QueryLimits:
    if limits is None:
        return QueryLimits()
    if isinstance(limits, QueryLimits):
        return limits
    if isinstance(limits, Mapping):
        return QueryLimits(
            max_seconds=limits.get("max_seconds"),
            max_operations=limits.get("max_operations"),
            max_trace_events=limits.get("max_trace_events"),
            max_stdout_chars=limits.get("max_stdout_chars"),
        )
    raise TypeError(f"Unsupported query limits: {type(limits)!r}")


def _merge_limits(
    base: Optional[QueryLimits | Mapping[str, Any]],
    override: Optional[QueryLimits | Mapping[str, Any]],
) -> QueryLimits:
    left = _coerce_limits(base)
    right = _coerce_limits(override)
    return QueryLimits(
        max_seconds=right.max_seconds if right.max_seconds is not None else left.max_seconds,
        max_operations=(
            right.max_operations if right.max_operations is not None else left.max_operations
        ),
        max_trace_events=(
            right.max_trace_events
            if right.max_trace_events is not None
            else left.max_trace_events
        ),
        max_stdout_chars=(
            right.max_stdout_chars if right.max_stdout_chars is not None else left.max_stdout_chars
        ),
    )


class _QueryRuntimeState:
    def __init__(self, limits: QueryLimits):
        self.limits = limits
        self.started_at = _current_time()
        self.operation_count = 0
        self.trace_events = 0
        self.stdout_chars = 0

    def elapsed_seconds(self) -> float:
        return _current_time() - self.started_at

    def check_limits(self) -> None:
        elapsed = self.elapsed_seconds()
        if self.limits.max_seconds is not None and elapsed > self.limits.max_seconds:
            raise QueryLimitExceededError(
                "max_seconds", self.limits.max_seconds, round(elapsed, 6)
            )

    def record_operation(self, _name: str) -> None:
        self.operation_count += 1
        if (
            self.limits.max_operations is not None
            and self.operation_count > self.limits.max_operations
        ):
            raise QueryLimitExceededError(
                "max_operations", self.limits.max_operations, self.operation_count
            )
        self.check_limits()

    def record_stdout(self, chunk: str) -> None:
        self.stdout_chars += len(chunk)
        if (
            self.limits.max_stdout_chars is not None
            and self.stdout_chars > self.limits.max_stdout_chars
        ):
            raise QueryLimitExceededError(
                "max_stdout_chars", self.limits.max_stdout_chars, self.stdout_chars
            )

    def trace(self, frame: Any, event: str, arg: Any) -> Any:
        if event in {"call", "line"}:
            self.trace_events += 1
            if (
                self.limits.max_trace_events is not None
                and self.trace_events > self.limits.max_trace_events
            ):
                raise QueryLimitExceededError(
                    "max_trace_events", self.limits.max_trace_events, self.trace_events
                )
            self.check_limits()
        return self.trace

    def usage(self) -> QueryUsage:
        return QueryUsage(
            elapsed_seconds=self.elapsed_seconds(),
            operation_count=self.operation_count,
            trace_events=self.trace_events,
            stdout_chars=self.stdout_chars,
        )


class _GuardedStdout(io.StringIO):
    def __init__(self, runtime_state: _QueryRuntimeState):
        super().__init__()
        self._runtime_state = runtime_state

    def write(self, text: str) -> int:
        self._runtime_state.record_stdout(text)
        return super().write(text)