"""
Context management infrastructure for UCM documents.

This module provides APIs for intelligent context window management,
allowing external orchestration layers to load documents, traverse
the knowledge graph, and curate context windows.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum
from typing import Optional, List, Dict, Set
from collections import deque
import time

from .document import Document


class InclusionReason(Enum):
    """Reason why a block was included in context."""

    DIRECT_REFERENCE = "direct_reference"
    NAVIGATION_PATH = "navigation_path"
    STRUCTURAL_CONTEXT = "structural_context"
    SEMANTIC_RELEVANCE = "semantic_relevance"
    EXTERNAL_DECISION = "external_decision"
    REQUIRED_CONTEXT = "required_context"


class ExpandDirection(Enum):
    """Direction for context expansion."""

    UP = "up"
    DOWN = "down"
    BOTH = "both"
    SEMANTIC = "semantic"


class ExpansionPolicy(Enum):
    """Policy for context expansion."""

    CONSERVATIVE = "conservative"
    BALANCED = "balanced"
    AGGRESSIVE = "aggressive"


class PruningPolicy(Enum):
    """Policy for context pruning."""

    RELEVANCE_FIRST = "relevance_first"
    RECENCY_FIRST = "recency_first"
    REDUNDANCY_FIRST = "redundancy_first"


class CompressionMethod(Enum):
    """Method for content compression."""

    TRUNCATE = "truncate"
    SUMMARIZE = "summarize"
    STRUCTURE_ONLY = "structure_only"


@dataclass
class ContextBlock:
    """A block in the context window with metadata."""

    block_id: str
    inclusion_reason: InclusionReason
    relevance_score: float
    token_estimate: int
    access_count: int = 1
    last_accessed: float = field(default_factory=time.time)
    compressed: bool = False
    original_content: Optional[str] = None


@dataclass
class ContextConstraints:
    """Constraints for the context window."""

    max_tokens: int = 4000
    max_blocks: int = 100
    max_depth: int = 10
    min_relevance: float = 0.0
    required_roles: List[str] = field(default_factory=list)
    excluded_tags: List[str] = field(default_factory=list)
    preserve_structure: bool = True
    allow_compression: bool = True


@dataclass
class ContextUpdateResult:
    """Result of a context operation."""

    blocks_added: List[str] = field(default_factory=list)
    blocks_removed: List[str] = field(default_factory=list)
    blocks_compressed: List[str] = field(default_factory=list)
    total_tokens: int = 0
    total_blocks: int = 0
    warnings: List[str] = field(default_factory=list)


@dataclass
class ContextStatistics:
    """Statistics about the context window."""

    total_tokens: int
    total_blocks: int
    blocks_by_reason: Dict[str, int]
    average_relevance: float
    compressed_count: int


@dataclass
class ContextMetadata:
    """Context window metadata."""

    focus_area: Optional[str] = None
    task_description: Optional[str] = None
    created_at: float = field(default_factory=time.time)
    last_modified: float = field(default_factory=time.time)


class ContextWindow:
    """Context window with intelligent management."""

    def __init__(self, id: str, constraints: Optional[ContextConstraints] = None):
        self.id = id
        self.blocks: Dict[str, ContextBlock] = {}
        self.constraints = constraints or ContextConstraints()
        self.metadata = ContextMetadata()

    @property
    def block_count(self) -> int:
        return len(self.blocks)

    @property
    def total_tokens(self) -> int:
        return sum(b.token_estimate for b in self.blocks.values())

    def has_capacity(self) -> bool:
        return (
            self.block_count < self.constraints.max_blocks
            and self.total_tokens < self.constraints.max_tokens
        )

    def contains(self, block_id: str) -> bool:
        return block_id in self.blocks

    def get(self, block_id: str) -> Optional[ContextBlock]:
        return self.blocks.get(block_id)

    def block_ids(self) -> List[str]:
        return list(self.blocks.keys())


class ContextManager:
    """
    Context Management Infrastructure.

    Provides APIs for external orchestration layers to manage context windows.

    Example:
        >>> manager = ContextManager("my-context")
        >>> manager.initialize_focus(doc, focus_block_id, "Summarize this section")
        >>> manager.expand_context(doc, ExpandDirection.DOWN, depth=2)
        >>> prompt = manager.render_for_prompt(doc)
    """

    def __init__(
        self,
        id: str,
        constraints: Optional[ContextConstraints] = None,
        expansion_policy: ExpansionPolicy = ExpansionPolicy.BALANCED,
        pruning_policy: PruningPolicy = PruningPolicy.RELEVANCE_FIRST,
    ):
        self.window = ContextWindow(id, constraints)
        self.expansion_policy = expansion_policy
        self.pruning_policy = pruning_policy

    def initialize_focus(
        self,
        doc: Document,
        focus_id: str,
        task_description: str,
    ) -> ContextUpdateResult:
        """
        Initialize context with a focus block.

        Args:
            doc: The document to work with
            focus_id: The block ID to focus on
            task_description: Description of the task

        Returns:
            ContextUpdateResult with operation details
        """
        self.window.metadata.focus_area = focus_id
        self.window.metadata.task_description = task_description
        self.window.metadata.last_modified = time.time()

        result = ContextUpdateResult()

        # Add focus block
        if focus_id in doc.blocks:
            self._add_block_internal(doc, focus_id, InclusionReason.DIRECT_REFERENCE, 1.0)
            result.blocks_added.append(focus_id)

        # Add structural context (ancestors)
        current = focus_id
        depth = 0
        while True:
            parent = doc.parent(current)
            if parent is None or parent == doc.root or depth >= 3:
                break
            self._add_block_internal(
                doc, parent, InclusionReason.STRUCTURAL_CONTEXT, 0.8 - depth * 0.1
            )
            result.blocks_added.append(parent)
            current = parent
            depth += 1

        result.total_tokens = self.window.total_tokens
        result.total_blocks = self.window.block_count
        return result

    def navigate_to(
        self,
        doc: Document,
        target_id: str,
        task_description: str,
    ) -> ContextUpdateResult:
        """Navigate to a new focus area."""
        self.window.metadata.focus_area = target_id
        self.window.metadata.task_description = task_description
        self.window.metadata.last_modified = time.time()

        result = ContextUpdateResult()

        if target_id in doc.blocks:
            self._add_block_internal(doc, target_id, InclusionReason.NAVIGATION_PATH, 1.0)
            result.blocks_added.append(target_id)

        result.blocks_removed = self._prune_if_needed()
        result.total_tokens = self.window.total_tokens
        result.total_blocks = self.window.block_count
        return result

    def add_block(
        self,
        doc: Document,
        block_id: str,
        reason: InclusionReason = InclusionReason.EXTERNAL_DECISION,
    ) -> ContextUpdateResult:
        """Add a block to the context."""
        result = ContextUpdateResult()

        if block_id in doc.blocks:
            self._add_block_internal(doc, block_id, reason, 0.7)
            result.blocks_added.append(block_id)

        result.blocks_removed = self._prune_if_needed()
        result.total_tokens = self.window.total_tokens
        result.total_blocks = self.window.block_count
        return result

    def remove_block(self, block_id: str) -> ContextUpdateResult:
        """Remove a block from the context."""
        result = ContextUpdateResult()

        if block_id in self.window.blocks:
            del self.window.blocks[block_id]
            result.blocks_removed.append(block_id)

        self.window.metadata.last_modified = time.time()
        result.total_tokens = self.window.total_tokens
        result.total_blocks = self.window.block_count
        return result

    def expand_context(
        self,
        doc: Document,
        direction: ExpandDirection,
        depth: int = 2,
    ) -> ContextUpdateResult:
        """Expand context in a direction."""
        result = ContextUpdateResult()

        focus_id = self.window.metadata.focus_area
        if not focus_id:
            return result

        if direction == ExpandDirection.DOWN:
            result.blocks_added = self._expand_downward(doc, focus_id, depth)
        elif direction == ExpandDirection.UP:
            result.blocks_added = self._expand_upward(doc, focus_id, depth)
        elif direction == ExpandDirection.BOTH:
            result.blocks_added = self._expand_downward(doc, focus_id, depth) + self._expand_upward(
                doc, focus_id, depth
            )
        elif direction == ExpandDirection.SEMANTIC:
            result.blocks_added = self._expand_semantic(doc, focus_id, depth)

        result.blocks_removed = self._prune_if_needed()
        self.window.metadata.last_modified = time.time()
        result.total_tokens = self.window.total_tokens
        result.total_blocks = self.window.block_count
        return result

    def compress(
        self,
        doc: Document,
        method: CompressionMethod = CompressionMethod.TRUNCATE,
    ) -> ContextUpdateResult:
        """Compress blocks to fit within constraints."""
        result = ContextUpdateResult()

        if not self.window.constraints.allow_compression:
            result.warnings.append("Compression not allowed by constraints")
            return result

        # Find blocks to compress (lowest relevance first)
        blocks_to_compress = sorted(
            [(bid, cb) for bid, cb in self.window.blocks.items() if not cb.compressed],
            key=lambda x: x[1].relevance_score,
        )

        for block_id, context_block in blocks_to_compress[:10]:
            block = doc.blocks.get(block_id)
            if block:
                context_block.original_content = str(block.content) if block.content else ""

                if method == CompressionMethod.TRUNCATE:
                    context_block.token_estimate = context_block.token_estimate // 2
                elif method == CompressionMethod.STRUCTURE_ONLY:
                    context_block.token_estimate = 10
                elif method == CompressionMethod.SUMMARIZE:
                    context_block.token_estimate = context_block.token_estimate // 3

                context_block.compressed = True
                result.blocks_compressed.append(block_id)

            if self.window.total_tokens <= self.window.constraints.max_tokens:
                break

        result.total_tokens = self.window.total_tokens
        result.total_blocks = self.window.block_count
        return result

    def get_statistics(self) -> ContextStatistics:
        """Get statistics about the context."""
        blocks_by_reason: Dict[str, int] = {}
        total_relevance = 0.0
        compressed_count = 0

        for cb in self.window.blocks.values():
            reason = cb.inclusion_reason.value
            blocks_by_reason[reason] = blocks_by_reason.get(reason, 0) + 1
            total_relevance += cb.relevance_score
            if cb.compressed:
                compressed_count += 1

        average_relevance = total_relevance / len(self.window.blocks) if self.window.blocks else 0.0

        return ContextStatistics(
            total_tokens=self.window.total_tokens,
            total_blocks=self.window.block_count,
            blocks_by_reason=blocks_by_reason,
            average_relevance=average_relevance,
            compressed_count=compressed_count,
        )

    def render_for_prompt(self, doc: Document) -> str:
        """Render context to a format suitable for LLM prompts."""
        output = []

        # Sort by relevance
        sorted_blocks = sorted(
            self.window.blocks.items(), key=lambda x: x[1].relevance_score, reverse=True
        )

        for block_id, context_block in sorted_blocks:
            block = doc.blocks.get(block_id)
            if block:
                if context_block.compressed and context_block.original_content:
                    content = f"[compressed] {context_block.original_content[:50]}..."
                else:
                    content = str(block.content) if block.content else ""

                role = "block"
                if block.metadata and block.metadata.semantic_role:
                    role = block.metadata.semantic_role.value

                output.append(f"[{block_id}] {role}: {content}")

        return "\n".join(output)

    # Internal methods

    def _add_block_internal(
        self,
        doc: Document,
        block_id: str,
        reason: InclusionReason,
        relevance: float,
    ) -> None:
        if block_id in self.window.blocks:
            cb = self.window.blocks[block_id]
            cb.access_count += 1
            cb.last_accessed = time.time()
            return

        block = doc.blocks.get(block_id)
        if block:
            content = str(block.content) if block.content else ""
            token_estimate = max(1, len(content) // 4)

            self.window.blocks[block_id] = ContextBlock(
                block_id=block_id,
                inclusion_reason=reason,
                relevance_score=relevance,
                token_estimate=token_estimate,
            )

    def _expand_downward(self, doc: Document, start: str, max_depth: int) -> List[str]:
        added = []
        queue = deque([(start, 0)])

        while queue and self.window.has_capacity():
            node_id, depth = queue.popleft()
            if depth > max_depth:
                break

            for child in doc.children(node_id):
                if not self.window.contains(child):
                    relevance = max(0.1, 0.6 - depth * 0.1)
                    self._add_block_internal(
                        doc, child, InclusionReason.STRUCTURAL_CONTEXT, relevance
                    )
                    added.append(child)
                    queue.append((child, depth + 1))

        return added

    def _expand_upward(self, doc: Document, start: str, max_depth: int) -> List[str]:
        added = []
        current = start
        depth = 0

        while depth < max_depth and self.window.has_capacity():
            parent = doc.parent(current)
            if parent is None or parent == doc.root:
                break

            if not self.window.contains(parent):
                relevance = max(0.1, 0.7 - depth * 0.1)
                self._add_block_internal(doc, parent, InclusionReason.STRUCTURAL_CONTEXT, relevance)
                added.append(parent)

            current = parent
            depth += 1

        return added

    def _expand_semantic(self, doc: Document, start: str, max_depth: int) -> List[str]:
        added = []
        visited: Set[str] = set()
        queue = deque([(start, 0)])

        while queue and self.window.has_capacity():
            node_id, depth = queue.popleft()
            if depth > max_depth or node_id in visited:
                continue

            visited.add(node_id)
            block = doc.blocks.get(node_id)

            if block and block.metadata:
                edges = block.metadata.get("edges", [])
                for edge in edges:
                    target = edge.get("target") if isinstance(edge, dict) else None
                    if target and not self.window.contains(target) and target not in visited:
                        relevance = max(0.1, 0.5 - depth * 0.1)
                        self._add_block_internal(
                            doc, target, InclusionReason.SEMANTIC_RELEVANCE, relevance
                        )
                        added.append(target)
                        queue.append((target, depth + 1))

        return added

    def _prune_if_needed(self) -> List[str]:
        removed = []

        while (
            self.window.block_count > self.window.constraints.max_blocks
            or self.window.total_tokens > self.window.constraints.max_tokens
        ):
            to_remove = self._find_block_to_remove()
            if to_remove:
                del self.window.blocks[to_remove]
                removed.append(to_remove)
            else:
                break

        return removed

    def _find_block_to_remove(self) -> Optional[str]:
        focus = self.window.metadata.focus_area

        if self.pruning_policy == PruningPolicy.RELEVANCE_FIRST:
            candidates = [(bid, cb) for bid, cb in self.window.blocks.items() if bid != focus]
            if candidates:
                return min(candidates, key=lambda x: x[1].relevance_score)[0]

        elif self.pruning_policy == PruningPolicy.RECENCY_FIRST:
            candidates = [(bid, cb) for bid, cb in self.window.blocks.items() if bid != focus]
            if candidates:
                return min(candidates, key=lambda x: x[1].last_accessed)[0]

        return None


# Convenience functions


def create_context(
    id: str,
    max_tokens: int = 4000,
    max_blocks: int = 100,
) -> ContextManager:
    """
    Create a new context manager with specified constraints.

    Example:
        >>> ctx = create_context("my-context", max_tokens=8000)
        >>> ctx.initialize_focus(doc, block_id, "Analyze this")
    """
    constraints = ContextConstraints(max_tokens=max_tokens, max_blocks=max_blocks)
    return ContextManager(id, constraints)
