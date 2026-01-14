"""
Core types for UCP SDK.

This module defines all fundamental types used throughout the SDK,
following the Single Responsibility Principle.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from datetime import datetime, timezone
from enum import Enum
from typing import Any, Callable, Dict, List, Optional


# =============================================================================
# ENUMS
# =============================================================================


class ContentType(str, Enum):
    """Content types supported by UCM."""
    TEXT = "text"
    CODE = "code"
    TABLE = "table"
    MATH = "math"
    JSON = "json"
    MEDIA = "media"
    BINARY = "binary"
    COMPOSITE = "composite"


class TextFormat(str, Enum):
    """Text format variants."""
    PLAIN = "plain"
    MARKDOWN = "markdown"
    RICH = "rich"


class SemanticRole(str, Enum):
    """Semantic roles for blocks."""
    # Headings
    HEADING1 = "heading1"
    HEADING2 = "heading2"
    HEADING3 = "heading3"
    HEADING4 = "heading4"
    HEADING5 = "heading5"
    HEADING6 = "heading6"

    # Content structure
    PARAGRAPH = "paragraph"
    QUOTE = "quote"
    LIST = "list"

    # Technical content
    CODE = "code"
    TABLE = "table"
    EQUATION = "equation"

    # Document structure
    TITLE = "title"
    SUBTITLE = "subtitle"
    ABSTRACT = "abstract"
    SECTION = "section"

    # Narrative structure
    INTRO = "intro"
    BODY = "body"
    CONCLUSION = "conclusion"

    # Callouts and special sections
    NOTE = "note"
    WARNING = "warning"
    TIP = "tip"
    SIDEBAR = "sidebar"
    CALLOUT = "callout"

    # Meta elements
    METADATA = "metadata"
    CITATION = "citation"
    FOOTNOTE = "footnote"


class EdgeType(str, Enum):
    """Types of relationships between blocks."""
    # Derivation relationships
    DERIVED_FROM = "derived_from"
    SUPERSEDES = "supersedes"
    TRANSFORMED_FROM = "transformed_from"
    
    # Reference relationships
    REFERENCES = "references"
    CITED_BY = "cited_by"
    LINKS_TO = "links_to"
    
    # Semantic relationships
    SUPPORTS = "supports"
    CONTRADICTS = "contradicts"
    ELABORATES = "elaborates"
    SUMMARIZES = "summarizes"
    
    # Structural relationships
    PARENT_OF = "parent_of"
    CHILD_OF = "child_of"
    SIBLING_OF = "sibling_of"
    PREVIOUS_SIBLING = "previous_sibling"
    NEXT_SIBLING = "next_sibling"
    
    # Version relationships
    VERSION_OF = "version_of"
    ALTERNATIVE_OF = "alternative_of"
    TRANSLATION_OF = "translation_of"

    @classmethod
    def from_str(cls, s: str) -> Optional["EdgeType"]:
        """Parse edge type from string."""
        try:
            return cls(s.lower())
        except ValueError:
            return None

    def inverse(self) -> Optional["EdgeType"]:
        """Get the inverse edge type if applicable."""
        inverses = {
            EdgeType.REFERENCES: EdgeType.CITED_BY,
            EdgeType.CITED_BY: EdgeType.REFERENCES,
            EdgeType.PARENT_OF: EdgeType.CHILD_OF,
            EdgeType.CHILD_OF: EdgeType.PARENT_OF,
            EdgeType.PREVIOUS_SIBLING: EdgeType.NEXT_SIBLING,
            EdgeType.NEXT_SIBLING: EdgeType.PREVIOUS_SIBLING,
            EdgeType.CONTRADICTS: EdgeType.CONTRADICTS,
            EdgeType.SIBLING_OF: EdgeType.SIBLING_OF,
        }
        return inverses.get(self)

    def is_symmetric(self) -> bool:
        """Check if this edge type is symmetric."""
        return self in (EdgeType.CONTRADICTS, EdgeType.SIBLING_OF)

    def is_structural(self) -> bool:
        """Check if this is a structural edge."""
        return self in (
            EdgeType.PARENT_OF,
            EdgeType.CHILD_OF,
            EdgeType.SIBLING_OF,
            EdgeType.PREVIOUS_SIBLING,
            EdgeType.NEXT_SIBLING,
        )


class Capability(str, Enum):
    """UCL command capabilities."""
    EDIT = "edit"
    APPEND = "append"
    MOVE = "move"
    DELETE = "delete"
    LINK = "link"
    SNAPSHOT = "snapshot"
    TRANSACTION = "transaction"


class ValidationSeverity(str, Enum):
    """Severity levels for validation issues."""
    ERROR = "error"
    WARNING = "warning"
    INFO = "info"


class TransactionState(str, Enum):
    """Transaction lifecycle states."""
    ACTIVE = "active"
    COMMITTED = "committed"
    ROLLED_BACK = "rolled_back"
    TIMED_OUT = "timed_out"


# =============================================================================
# DATA CLASSES
# =============================================================================


@dataclass
class EdgeMetadata:
    """Metadata for an edge relationship."""
    confidence: Optional[float] = None
    description: Optional[str] = None
    custom: Dict[str, Any] = field(default_factory=dict)

    def is_empty(self) -> bool:
        return self.confidence is None and self.description is None and not self.custom


@dataclass
class Edge:
    """An edge represents a relationship between blocks."""
    edge_type: EdgeType
    target: str  # Block ID
    metadata: EdgeMetadata = field(default_factory=EdgeMetadata)
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))

    @classmethod
    def new(cls, edge_type: EdgeType, target: str) -> "Edge":
        return cls(edge_type=edge_type, target=target)

    def with_confidence(self, confidence: float) -> "Edge":
        self.metadata.confidence = max(0.0, min(1.0, confidence))
        return self

    def with_description(self, description: str) -> "Edge":
        self.metadata.description = description
        return self


@dataclass
class TokenEstimate:
    """Token count estimates for different models."""
    gpt4: int = 0
    claude: int = 0
    
    @classmethod
    def estimate_text(cls, text: str) -> "TokenEstimate":
        """Estimate tokens for text content."""
        # Rough estimation: ~4 chars per token for English
        char_count = len(text)
        return cls(
            gpt4=max(1, char_count // 4),
            claude=max(1, char_count // 4),
        )


@dataclass
class BlockMetadata:
    """Metadata for a content block."""
    semantic_role: Optional[SemanticRole] = None
    label: Optional[str] = None
    tags: List[str] = field(default_factory=list)
    summary: Optional[str] = None
    token_estimate: Optional[TokenEstimate] = None
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    modified_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    custom: Dict[str, Any] = field(default_factory=dict)

    def touch(self) -> None:
        """Update modification timestamp."""
        self.modified_at = datetime.now(timezone.utc)

    def has_tag(self, tag: str) -> bool:
        """Check if metadata has a specific tag."""
        return tag in self.tags

    def add_tag(self, tag: str) -> None:
        """Add a tag if not present."""
        if tag not in self.tags:
            self.tags.append(tag)

    def remove_tag(self, tag: str) -> None:
        """Remove a tag if present."""
        self.tags = [t for t in self.tags if t != tag]


@dataclass
class DocumentMetadata:
    """Metadata for a document."""
    title: Optional[str] = None
    description: Optional[str] = None
    authors: List[str] = field(default_factory=list)
    language: Optional[str] = None
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    modified_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    custom: Dict[str, Any] = field(default_factory=dict)

    def touch(self) -> None:
        """Update modification timestamp."""
        self.modified_at = datetime.now(timezone.utc)


@dataclass
class ValidationIssue:
    """A validation issue found in a document."""
    severity: ValidationSeverity
    code: str
    message: str
    block_id: Optional[str] = None

    @classmethod
    def error(cls, code: str, message: str, block_id: Optional[str] = None) -> "ValidationIssue":
        return cls(severity=ValidationSeverity.ERROR, code=code, message=message, block_id=block_id)

    @classmethod
    def warning(cls, code: str, message: str, block_id: Optional[str] = None) -> "ValidationIssue":
        return cls(severity=ValidationSeverity.WARNING, code=code, message=message, block_id=block_id)

    @classmethod
    def info(cls, code: str, message: str, block_id: Optional[str] = None) -> "ValidationIssue":
        """Create an INFO severity issue."""
        return cls(severity=ValidationSeverity.INFO, code=code, message=message, block_id=block_id)


@dataclass
class ValidationResult:
    """Result of document validation."""
    valid: bool
    issues: List[ValidationIssue] = field(default_factory=list)

    @classmethod
    def success(cls) -> "ValidationResult":
        return cls(valid=True)

    @classmethod
    def failure(cls, issues: List[ValidationIssue]) -> "ValidationResult":
        has_errors = any(i.severity == ValidationSeverity.ERROR for i in issues)
        return cls(valid=not has_errors, issues=issues)

    def errors(self) -> List[ValidationIssue]:
        return [i for i in self.issues if i.severity == ValidationSeverity.ERROR]

    def warnings(self) -> List[ValidationIssue]:
        return [i for i in self.issues if i.severity == ValidationSeverity.WARNING]

    def infos(self) -> List[ValidationIssue]:
        """Get all INFO severity issues."""
        return [i for i in self.issues if i.severity == ValidationSeverity.INFO]

    def merge(self, other: "ValidationResult") -> None:
        self.issues.extend(other.issues)
        self.valid = self.valid and other.valid


@dataclass
class ResourceLimits:
    """Resource limits for validation."""
    max_document_size: int = 50 * 1024 * 1024  # 50MB
    max_block_count: int = 100_000
    max_block_size: int = 5 * 1024 * 1024  # 5MB
    max_nesting_depth: int = 50
    max_edges_per_block: int = 1000


# =============================================================================
# OBSERVER PROTOCOL (for observability)
# =============================================================================


@dataclass
class UcpEvent:
    """Base event for observability."""
    timestamp: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    event_type: str = ""
    data: Dict[str, Any] = field(default_factory=dict)


EventHandler = Callable[[UcpEvent], None]
