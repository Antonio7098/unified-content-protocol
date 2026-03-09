from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Iterable, Mapping, Optional

from .query import BaseQueryGraph, BaseQuerySession, QueryLimits, QueryRunResult, _merge_limits, run_python_query


@dataclass(frozen=True)
class QueryBenchmarkCase:
    name: str
    description: str
    code: str
    bindings: Optional[Mapping[str, Any]] = None
    include_export: bool = False
    export_kwargs: Optional[Mapping[str, Any]] = None
    limits: Optional[QueryLimits | Mapping[str, Any]] = None


@dataclass
class QueryBenchmarkResult:
    case: QueryBenchmarkCase
    run: QueryRunResult

    @property
    def ok(self) -> bool:
        return self.run.ok

    def as_dict(self) -> dict[str, Any]:
        return {
            "name": self.case.name,
            "description": self.case.description,
            **self.run.as_dict(),
        }


def run_query_benchmark(
    graph: BaseQueryGraph | Any,
    case: QueryBenchmarkCase,
    *,
    session: Optional[BaseQuerySession | Any] = None,
    default_limits: Optional[QueryLimits | Mapping[str, Any]] = None,
) -> QueryBenchmarkResult:
    run = run_python_query(
        graph,
        case.code,
        session=session,
        bindings=case.bindings,
        include_export=case.include_export,
        export_kwargs=case.export_kwargs,
        limits=_merge_limits(default_limits, case.limits),
    )
    return QueryBenchmarkResult(case=case, run=run)


def run_query_benchmark_suite(
    graph: BaseQueryGraph | Any,
    cases: Iterable[QueryBenchmarkCase],
    *,
    session: Optional[BaseQuerySession | Any] = None,
    default_limits: Optional[QueryLimits | Mapping[str, Any]] = None,
    stop_on_error: bool = False,
) -> list[QueryBenchmarkResult]:
    results: list[QueryBenchmarkResult] = []
    for case in cases:
        result = run_query_benchmark(
            graph,
            case,
            session=session,
            default_limits=default_limits,
        )
        results.append(result)
        if stop_on_error and not result.ok:
            break
    return results


def summarize_query_benchmark_suite(
    results: Iterable[QueryBenchmarkResult],
) -> dict[str, Any]:
    materialized = list(results)
    return {
        "cases": len(materialized),
        "ok": sum(1 for result in materialized if result.ok),
        "failed": sum(1 for result in materialized if not result.ok),
        "total_operations": sum(result.run.usage.operation_count for result in materialized),
        "total_trace_events": sum(result.run.usage.trace_events for result in materialized),
        "total_elapsed_seconds": round(
            sum(result.run.usage.elapsed_seconds for result in materialized), 6
        ),
    }