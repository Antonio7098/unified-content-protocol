"""
Block - The fundamental unit of content in UCM.

This module implements the Block class following Single Responsibility Principle.
"""

from __future__ import annotations

import hashlib
import unicodedata
from dataclasses import dataclass, field
from typing import List, Optional

from .types import (
    BlockMetadata,
    ContentType,
    Edge,
    EdgeType,
    SemanticRole,
    TokenEstimate,
)


_id_counter = 0
ROOT_BLOCK_ID = "blk_000000000000"


def _next_namespace() -> str:
    """Generate a monotonically increasing namespace suffix."""
    global _id_counter
    _id_counter += 1
    return f"{_id_counter:012x}"


def generate_block_id(
    content: Optional[str] = None,
    role: Optional[str] = None,
    namespace: Optional[str] = None,
) -> str:
    """Generate a deterministic block ID following the UCP spec."""
    if (content is None or content == "") and role is None and namespace is None:
        return ROOT_BLOCK_ID

    normalized = unicodedata.normalize("NFC", content or "")
    payload = normalized.encode("utf-8")

    if role:
        payload += role.encode("utf-8")

    if namespace:
        payload += namespace.encode("utf-8")

    digest = hashlib.sha256(payload).hexdigest()[:12]
    return f"blk_{digest}"


def reset_block_counter() -> None:
    """Reset block counter (useful for testing)."""
    global _id_counter
    _id_counter = 0


@dataclass
class Block:
    """A content block in the document.
    
    Blocks are immutable content units with metadata and relationships.
    """
    id: str
    content: str
    content_type: ContentType = ContentType.TEXT
    metadata: BlockMetadata = field(default_factory=BlockMetadata)
    edges: List[Edge] = field(default_factory=list)
    children: List[str] = field(default_factory=list)

    @classmethod
    def new(
        cls,
        content: str,
        *,
        content_type: ContentType = ContentType.TEXT,
        role: Optional[SemanticRole] = None,
        label: Optional[str] = None,
    ) -> "Block":
        """Create a new block with generated ID."""
        metadata = BlockMetadata(semantic_role=role, label=label)
        return cls(
            id=generate_block_id(content, role.value if role else None, _next_namespace()),
            content=content,
            content_type=content_type,
            metadata=metadata,
        )

    @classmethod
    def root(cls) -> "Block":
        """Create a root block."""
        return cls(
            id=generate_block_id(),
            content="",
            content_type=ContentType.TEXT,
        )

    @classmethod
    def text(cls, text: str, role: Optional[SemanticRole] = None) -> "Block":
        """Create a text block."""
        return cls.new(text, content_type=ContentType.TEXT, role=role)

    @classmethod
    def code(cls, source: str, language: str = "") -> "Block":
        """Create a code block."""
        block = cls.new(source, content_type=ContentType.CODE, role=SemanticRole.CODE)
        block.metadata.custom["language"] = language
        return block

    # -------------------------------------------------------------------------
    # Properties
    # -------------------------------------------------------------------------

    @property
    def type(self) -> ContentType:
        """Get content type (alias for content_type)."""
        return self.content_type

    @property
    def role(self) -> Optional[SemanticRole]:
        """Get semantic role."""
        return self.metadata.semantic_role

    @property
    def semantic_role(self) -> Optional[SemanticRole]:
        """Alias for semantic role to align with spec."""
        return self.metadata.semantic_role

    @property
    def label(self) -> Optional[str]:
        """Get label."""
        return self.metadata.label

    @property
    def tags(self) -> List[str]:
        """Get tags."""
        return self.metadata.tags

    @property
    def type_tag(self) -> str:
        """Get content type as string."""
        return self.content_type.value

    # -------------------------------------------------------------------------
    # Methods
    # -------------------------------------------------------------------------

    def size_bytes(self) -> int:
        """Get approximate size in bytes."""
        return len(self.content.encode("utf-8"))

    def token_estimate(self) -> TokenEstimate:
        """Get or compute token estimate."""
        if self.metadata.token_estimate:
            return self.metadata.token_estimate
        return TokenEstimate.estimate_text(self.content)

    def has_tag(self, tag: str) -> bool:
        """Check if block has a specific tag."""
        return self.metadata.has_tag(tag)

    def add_tag(self, tag: str) -> None:
        """Add a tag to the block."""
        self.metadata.add_tag(tag)
        self.metadata.touch()

    def remove_tag(self, tag: str) -> None:
        """Remove a tag from the block."""
        self.metadata.remove_tag(tag)
        self.metadata.touch()

    def update_content(self, content: str) -> None:
        """Update block content."""
        self.content = content
        self.metadata.touch()

    # -------------------------------------------------------------------------
    # Edge Management
    # -------------------------------------------------------------------------

    def add_edge(self, edge: Edge) -> None:
        """Add an edge to this block."""
        self.edges.append(edge)
        self.metadata.touch()

    def remove_edge(self, target: str, edge_type: EdgeType) -> bool:
        """Remove an edge by target and type. Returns True if removed."""
        original_len = len(self.edges)
        self.edges = [
            e for e in self.edges
            if not (e.target == target and e.edge_type == edge_type)
        ]
        removed = len(self.edges) < original_len
        if removed:
            self.metadata.touch()
        return removed

    def edges_of_type(self, edge_type: EdgeType) -> List[Edge]:
        """Get all edges of a specific type."""
        return [e for e in self.edges if e.edge_type == edge_type]

    def has_edge_to(self, target: str, edge_type: Optional[EdgeType] = None) -> bool:
        """Check if block has an edge to target."""
        for edge in self.edges:
            if edge.target == target:
                if edge_type is None or edge.edge_type == edge_type:
                    return True
        return False

    # -------------------------------------------------------------------------
    # Hashability (for use in sets and as dict keys)
    # -------------------------------------------------------------------------

    def __hash__(self) -> int:
        """Hash based on block ID for use in sets and as dict keys."""
        return hash(self.id)

    def __eq__(self, other: object) -> bool:
        """Equality based on block ID."""
        if not isinstance(other, Block):
            return NotImplemented
        return self.id == other.id
