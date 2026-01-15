"""
UCP - Unified Content Protocol SDK

A developer-friendly SDK for building LLM-powered content manipulation.

This SDK provides:
- Document parsing and rendering (markdown)
- Block manipulation (edit, move, delete)
- Edge/relationship management
- Tag support
- UCL command execution
- LLM prompt building utilities
- Token-efficient ID mapping
- Snapshots and transactions
- Observability (logging, tracing, metrics)

Example:
    >>> import ucp
    >>>
    >>> # Parse markdown into a document
    >>> doc = ucp.parse('# Hello\\n\\nWorld')
    >>>
    >>> # Manipulate blocks
    >>> doc.edit_block(block_id, "New content")
    >>> doc.add_tag(block_id, "important")
    >>>
    >>> # Execute UCL commands
    >>> ucp.execute_ucl(doc, 'EDIT blk_1 SET text = "updated"')
    >>>
    >>> # Get a prompt builder for your LLM
    >>> prompt_text = ucp.prompt().edit().append().with_short_ids().build()
    >>>
    >>> # Map IDs for token efficiency
    >>> mapper = ucp.map_ids(doc)
    >>> short_prompt = mapper.shorten(doc_description)
    >>> expanded_ucl = mapper.expand(llm_response)
"""

from __future__ import annotations

__version__ = "0.1.3"

# =============================================================================
# CORE TYPES (from types module)
# =============================================================================
from .types import (
    ContentType,
    TextFormat,
    SemanticRole,
    EdgeType,
    Capability,
    ValidationSeverity,
    TransactionState,
    EdgeMetadata,
    Edge,
    TokenEstimate,
    BlockMetadata,
    DocumentMetadata,
    ValidationIssue,
    ValidationResult,
    ResourceLimits,
    UcpEvent,
    EventHandler,
)

# =============================================================================
# BLOCK (from block module)
# =============================================================================
from .block import (
    Block,
    generate_block_id,
    reset_block_counter,
)

# =============================================================================
# EDGE INDEX (from edge_index module)
# =============================================================================
from .edge_index import EdgeIndex

# =============================================================================
# DOCUMENT (from document module)
# =============================================================================
from .document import (
    Document,
    DocumentIndices,
    generate_document_id,
)

# =============================================================================
# EXECUTOR (from executor module)
# =============================================================================
from .executor import (
    UclParseError,
    UclExecutionError,
    UclParser,
    UclExecutor,
    ExecutionResult,
    execute_ucl,
)

# =============================================================================
# MARKDOWN (from markdown module)
# =============================================================================
from .markdown import (
    MarkdownParser,
    MarkdownRenderer,
    parse,
    render,
)

# =============================================================================
# LLM UTILITIES (from llm module)
# =============================================================================
from .llm import (
    PromptBuilder,
    IdMapper,
    UclBuilder,
    prompt,
    map_ids,
    ucl,
)

# =============================================================================
# SNAPSHOT (from snapshot module)
# =============================================================================
from .snapshot import (
    Snapshot,
    SnapshotInfo,
    SnapshotManager,
    serialize_document,
    deserialize_document,
)

# =============================================================================
# TRANSACTION (from transaction module)
# =============================================================================
from .transaction import (
    Transaction,
    TransactionManager,
    TransactionContext,
    Savepoint,
    transaction,
)

# =============================================================================
# OBSERVABILITY (from observability module)
# =============================================================================
from .observability import (
    LogLevel,
    EventType,
    EventBus,
    Tracer,
    Metrics,
    SpanContext,
    DocumentEvent,
    UclEvent,
    get_logger,
    emit_event,
    on_event,
    trace,
    record_metric,
)

# =============================================================================
# CONVENIENCE FUNCTIONS
# =============================================================================


def create(title: str | None = None) -> Document:
    """Create a new empty document.
    
    Args:
        title: Optional document title
        
    Returns:
        New Document instance
    """
    return Document.create(title=title)


# =============================================================================
# PUBLIC API
# =============================================================================

__all__ = [
    # Version
    "__version__",
    # Core types
    "ContentType",
    "TextFormat",
    "SemanticRole",
    "EdgeType",
    "Capability",
    "ValidationSeverity",
    "TransactionState",
    "EdgeMetadata",
    "Edge",
    "TokenEstimate",
    "BlockMetadata",
    "DocumentMetadata",
    "ValidationIssue",
    "ValidationResult",
    "ResourceLimits",
    "UcpEvent",
    "EventHandler",
    # Block
    "Block",
    "generate_block_id",
    "reset_block_counter",
    # Edge index
    "EdgeIndex",
    # Document
    "Document",
    "DocumentIndices",
    "generate_document_id",
    # Executor
    "UclParseError",
    "UclExecutionError",
    "UclParser",
    "UclExecutor",
    "ExecutionResult",
    "execute_ucl",
    # Markdown
    "MarkdownParser",
    "MarkdownRenderer",
    "parse",
    "render",
    # LLM utilities
    "PromptBuilder",
    "IdMapper",
    "UclBuilder",
    "prompt",
    "map_ids",
    "ucl",
    # Snapshot
    "Snapshot",
    "SnapshotInfo",
    "SnapshotManager",
    "serialize_document",
    "deserialize_document",
    # Transaction
    "Transaction",
    "TransactionManager",
    "TransactionContext",
    "Savepoint",
    "transaction",
    # Observability
    "LogLevel",
    "EventType",
    "EventBus",
    "Tracer",
    "Metrics",
    "SpanContext",
    "DocumentEvent",
    "UclEvent",
    "get_logger",
    "emit_event",
    "on_event",
    "trace",
    "record_metric",
    # Convenience
    "create",
]
