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
    Document,
    Edge,
    EdgeType,
    # LLM utilities
    IdMapper,
    PromptBuilder,
    PromptPresets,
    UclCapability,
    # Snapshot management
    SnapshotManager,
    SnapshotInfo,
    # Functions
    parse,
    render,
    execute_ucl,
    create,
    # Exceptions
    UcpError,
    BlockNotFoundError,
    InvalidBlockIdError,
    CycleDetectedError,
    ValidationError,
    ParseError,
)

__version__ = "0.1.5"
__all__ = [
    # Classes
    "BlockId",
    "Content",
    "Block",
    "Document",
    "Edge",
    "EdgeType",
    # LLM utilities
    "IdMapper",
    "PromptBuilder",
    "PromptPresets",
    "UclCapability",
    # Snapshot management
    "SnapshotManager",
    "SnapshotInfo",
    # Functions
    "parse",
    "render",
    "execute_ucl",
    "create",
    # Exceptions
    "UcpError",
    "BlockNotFoundError",
    "InvalidBlockIdError",
    "CycleDetectedError",
    "ValidationError",
    "ParseError",
]
