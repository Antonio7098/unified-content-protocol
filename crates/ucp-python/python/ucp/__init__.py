"""
UCP (Unified Content Protocol) - Python bindings for the Rust implementation.

This package provides Python bindings to the high-performance Rust UCP library.

Example:
    >>> import ucp
    >>> doc = ucp.create("My Document")
    >>> block_id = doc.add_block(doc.root_id, "Hello, world!")
    >>> print(ucp.render(doc))
"""

from ucp._core import (
    # Classes
    BlockId,
    Content,
    Block,
    Graph,
    GraphSession,
    CodeGraph,
    CodeGraphSession,
    Document,
    Edge,
    EdgeType,
    # Engine and validation
    Engine,
    EngineConfig,
    TransactionId,
    ResourceLimits,
    ValidationPipeline,
    ValidationResult,
    ValidationIssue,
    # Traversal
    TraversalEngine,
    TraversalConfig,
    TraversalFilter,
    TraversalDirection,
    TraversalResult,
    TraversalNode,
    WriteSectionResult,
    # Agent traversal system
    AgentTraversal,
    AgentSessionId,
    SessionConfig,
    AgentCapabilities,
    ViewMode,
    NavigationResult,
    ExpansionResult,
    BlockView,
    SearchResult,
    FindResult,
    # Section utilities
    ClearResult,
    DeletedContent,
    # LLM utilities
    IdMapper,
    PromptBuilder,
    PromptPresets,
    UclCapability,
    # Snapshot management
    SnapshotManager,
    SnapshotInfo,
    # Observability
    UcpEvent,
    EventBus,
    AuditEntry,
    MetricsRecorder,
    # Functions
    parse,
    render,
    parse_html,
    execute_ucl,
    create,
    # Section functions
    clear_section_with_undo,
    restore_deleted_section,
    find_section_by_path,
    get_all_sections,
    get_section_depth,
    write_section,
    # Exceptions
    UcpError,
    BlockNotFoundError,
    InvalidBlockIdError,
    CycleDetectedError,
    ValidationError,
    ParseError,
)
from ucp.query import (
    CodeQueryGraph,
    CodeQuerySession,
    QueryExecutionError,
    QueryGraph,
    QueryLimitExceededError,
    QueryLimits,
    QueryRunResult,
    QuerySession,
    QueryUsage,
    query,
    run_python_query,
)
from ucp.query_benchmarks import (
    QueryBenchmarkCase,
    QueryBenchmarkResult,
    run_query_benchmark,
    run_query_benchmark_suite,
    summarize_query_benchmark_suite,
)
from ucp.query_tools import PythonQueryTool, QueryToolResult

__version__ = "0.1.9"
__all__ = [
    # Classes
    "BlockId",
    "Content",
    "Block",
    "Graph",
    "GraphSession",
    "CodeGraph",
    "CodeGraphSession",
    "Document",
    "Edge",
    "EdgeType",
    # Engine and validation
    "Engine",
    "EngineConfig",
    "TransactionId",
    "ResourceLimits",
    "ValidationPipeline",
    "ValidationResult",
    "ValidationIssue",
    # Traversal
    "TraversalEngine",
    "TraversalConfig",
    "TraversalFilter",
    "TraversalDirection",
    "TraversalResult",
    "TraversalNode",
    "WriteSectionResult",
    # Agent traversal system
    "AgentTraversal",
    "AgentSessionId",
    "SessionConfig",
    "AgentCapabilities",
    "ViewMode",
    "NavigationResult",
    "ExpansionResult",
    "BlockView",
    "SearchResult",
    "FindResult",
    # Section utilities
    "ClearResult",
    "DeletedContent",
    # LLM utilities
    "IdMapper",
    "PromptBuilder",
    "PromptPresets",
    "UclCapability",
    # Snapshot management
    "SnapshotManager",
    "SnapshotInfo",
    # Observability
    "UcpEvent",
    "EventBus",
    "AuditEntry",
    "MetricsRecorder",
    # Functions
    "parse",
    "render",
    "parse_html",
    "execute_ucl",
    "create",
    # Section functions
    "clear_section_with_undo",
    "restore_deleted_section",
    "find_section_by_path",
    "get_all_sections",
    "get_section_depth",
    "write_section",
    # Exceptions
    "UcpError",
    "BlockNotFoundError",
    "InvalidBlockIdError",
    "CycleDetectedError",
    "ValidationError",
    "ParseError",
    "QueryGraph",
    "CodeQueryGraph",
    "QuerySession",
    "CodeQuerySession",
    "QueryRunResult",
    "QueryExecutionError",
    "QueryLimitExceededError",
    "QueryLimits",
    "QueryUsage",
    "PythonQueryTool",
    "QueryToolResult",
    "QueryBenchmarkCase",
    "QueryBenchmarkResult",
    "query",
    "run_python_query",
    "run_query_benchmark",
    "run_query_benchmark_suite",
    "summarize_query_benchmark_suite",
]
