"""
Document - A collection of blocks with hierarchical structure.

This module implements the Document class with full feature parity
to the Rust implementation.
"""

from __future__ import annotations

import time
from collections import defaultdict
from typing import (
    Dict,
    Iterator,
    List,
    Optional,
    Set,
    Tuple,
)

from .block import Block
from .edge_index import EdgeIndex
from .types import (
    ContentType,
    DocumentMetadata,
    Edge,
    EdgeType,
    ResourceLimits,
    SemanticRole,
    ValidationIssue,
    ValidationResult,
)


# =============================================================================
# DOCUMENT INDICES
# =============================================================================


class DocumentIndices:
    """Secondary indices for fast lookup."""

    def __init__(self) -> None:
        self.by_tag: Dict[str, Set[str]] = defaultdict(set)
        self.by_role: Dict[str, Set[str]] = defaultdict(set)
        self.by_content_type: Dict[str, Set[str]] = defaultdict(set)
        self.by_label: Dict[str, str] = {}

    def index_block(self, block: Block) -> None:
        """Index a block."""
        block_id = block.id

        # Index by tags
        for tag in block.tags:
            self.by_tag[tag].add(block_id)

        # Index by semantic role
        if block.role:
            self.by_role[block.role.value].add(block_id)

        # Index by content type
        self.by_content_type[block.type_tag].add(block_id)

        # Index by label
        if block.label:
            self.by_label[block.label] = block_id

    def remove_block(self, block: Block) -> None:
        """Remove a block from indices."""
        block_id = block.id

        for tag in block.tags:
            self.by_tag[tag].discard(block_id)

        if block.role:
            self.by_role[block.role.value].discard(block_id)

        self.by_content_type[block.type_tag].discard(block_id)

        if block.label and self.by_label.get(block.label) == block_id:
            del self.by_label[block.label]

    def rebuild(self, blocks: Dict[str, Block]) -> None:
        """Rebuild all indices from blocks."""
        self.by_tag.clear()
        self.by_role.clear()
        self.by_content_type.clear()
        self.by_label.clear()

        for block in blocks.values():
            self.index_block(block)

    def find_by_tag(self, tag: str) -> Set[str]:
        """Find blocks by tag."""
        return set(self.by_tag.get(tag, set()))

    def find_by_type(self, content_type: str) -> Set[str]:
        """Find blocks by content type."""
        return set(self.by_content_type.get(content_type, set()))

    def find_by_role(self, role: str) -> Set[str]:
        """Find blocks by semantic role."""
        return set(self.by_role.get(role, set()))

    def find_by_label(self, label: str) -> Optional[str]:
        """Find block by label."""
        return self.by_label.get(label)


# =============================================================================
# DOCUMENT
# =============================================================================


def generate_document_id() -> str:
    """Generate a unique document ID."""
    return f"doc_{int(time.time() * 1000):x}"


class Document:
    """A UCM document is a collection of blocks with hierarchical structure.
    
    Documents maintain:
    - A tree structure of blocks (parent-child relationships)
    - Secondary indices for fast lookup
    - An edge index for relationship traversal
    - Version tracking for concurrency control
    """

    def __init__(
        self,
        doc_id: Optional[str] = None,
        metadata: Optional[DocumentMetadata] = None,
    ) -> None:
        self.id = doc_id or generate_document_id()
        self.metadata = metadata or DocumentMetadata()
        
        # Create root block
        root = Block.root()
        self.root = root.id
        
        # Block storage
        self.blocks: Dict[str, Block] = {root.id: root}
        
        # Parent -> children structure (keep alias for compatibility)
        self._children: Dict[str, List[str]] = {root.id: []}
        self.structure = self._children
        
        # Indices
        self.indices = DocumentIndices()
        self.edge_index = EdgeIndex()
        self.indices.index_block(root)
        
        # Version for optimistic concurrency
        self._version = 1

    @classmethod
    def create(cls, title: Optional[str] = None) -> "Document":
        """Create a new document with optional title."""
        metadata = DocumentMetadata(title=title) if title else None
        return cls(metadata=metadata)

    # -------------------------------------------------------------------------
    # Block Access
    # -------------------------------------------------------------------------

    def get_block(self, block_id: str) -> Optional[Block]:
        """Get a block by ID."""
        return self.blocks.get(block_id)

    def get_block_mut(self, block_id: str) -> Optional[Block]:
        """Get a mutable reference to a block."""
        return self.blocks.get(block_id)

    def block_count(self) -> int:
        """Get total number of blocks."""
        return len(self.blocks)

    @property
    def root_id(self) -> str:
        """Alias for root block ID."""
        return self.root

    def __contains__(self, block_id: str) -> bool:
        """Check if document contains a block."""
        return block_id in self.blocks

    def __iter__(self) -> Iterator[Block]:
        """Iterate over all blocks."""
        return iter(self.blocks.values())

    # -------------------------------------------------------------------------
    # Structure Access
    # -------------------------------------------------------------------------

    def children(self, parent_id: str) -> List[str]:
        """Get children of a block."""
        return list(self.structure.get(parent_id, []))

    def parent(self, child_id: str) -> Optional[str]:
        """Get parent of a block."""
        for parent_id, children in self.structure.items():
            if child_id in children:
                return parent_id
        return None

    def ancestors(self, block_id: str) -> List[str]:
        """Get all ancestors of a block (from immediate parent to root)."""
        result = []
        current = self.parent(block_id)
        while current:
            result.append(current)
            current = self.parent(current)
        return result

    def descendants(self, block_id: str) -> List[str]:
        """Get all descendants of a block (breadth-first)."""
        result = []
        queue = list(self.children(block_id))
        while queue:
            current = queue.pop(0)
            result.append(current)
            queue.extend(self.children(current))
        return result

    def siblings(self, block_id: str) -> List[str]:
        """Get siblings of a block (excluding itself)."""
        parent_id = self.parent(block_id)
        if not parent_id:
            return []
        return [c for c in self.children(parent_id) if c != block_id]

    def depth(self, block_id: str) -> int:
        """Get depth of a block (root is 0)."""
        return len(self.ancestors(block_id))

    # -------------------------------------------------------------------------
    # Block Manipulation
    # -------------------------------------------------------------------------

    def add_block(
        self,
        parent_id: str,
        content: str,
        *,
        content_type: ContentType = ContentType.TEXT,
        role: Optional[SemanticRole] = None,
        label: Optional[str] = None,
        index: Optional[int] = None,
    ) -> str:
        """Add a new block to the document.
        
        Args:
            parent_id: ID of the parent block
            content: Block content
            content_type: Type of content
            role: Semantic role
            label: Optional label
            index: Position in parent's children (None = append)
            
        Returns:
            The new block's ID
            
        Raises:
            ValueError: If parent not found
        """
        if parent_id not in self.blocks:
            raise ValueError(f"Parent block not found: {parent_id}")

        block = Block.new(content, content_type=content_type, role=role, label=label)
        block_id = block.id

        # Index edges
        for edge in block.edges:
            self.edge_index.add_edge(block_id, edge)

        # Index block
        self.indices.index_block(block)

        # Add to blocks and initialize child list
        self.blocks[block_id] = block
        if block_id not in self.structure:
            self.structure[block_id] = []

        # Add to structure
        if parent_id not in self.structure:
            self.structure[parent_id] = []
        
        if index is None or index < 0 or index >= len(self.structure[parent_id]):
            self.structure[parent_id].append(block_id)
        else:
            self.structure[parent_id].insert(index, block_id)

        # Keep parent block's children list in sync
        parent_block = self.blocks.get(parent_id)
        if parent_block:
            parent_block.children = list(self.structure[parent_id])

        self._touch()
        return block_id

    def edit_block(self, block_id: str, content: str) -> None:
        """Update the content of a block.
        
        Raises:
            ValueError: If block not found
        """
        block = self.blocks.get(block_id)
        if block is None:
            raise ValueError(f"Block not found: {block_id}")
        
        block.update_content(content)
        self._touch()

    def move_block(
        self,
        block_id: str,
        new_parent_id: str,
        index: Optional[int] = None,
    ) -> None:
        """Move a block to a new parent.
        
        Args:
            block_id: ID of block to move
            new_parent_id: ID of new parent
            index: Position in new parent's children
            
        Raises:
            ValueError: If block/parent not found or invalid move
        """
        if block_id == self.root:
            raise ValueError("Cannot move the root block")

        if block_id not in self.blocks:
            raise ValueError(f"Block not found: {block_id}")

        if new_parent_id not in self.blocks:
            raise ValueError(f"Parent block not found: {new_parent_id}")

        # Check for cycle
        if new_parent_id == block_id or self._is_descendant(block_id, new_parent_id):
            raise ValueError("Cannot move a block into itself or its descendants")

        # Remove from old parent
        old_parent_id = self.parent(block_id)
        if old_parent_id and old_parent_id in self.structure:
            self.structure[old_parent_id] = [
                c for c in self.structure[old_parent_id] if c != block_id
            ]
            # Sync old parent's children list
            old_parent_block = self.blocks.get(old_parent_id)
            if old_parent_block:
                old_parent_block.children = list(self.structure[old_parent_id])

        # Add to new parent
        if new_parent_id not in self.structure:
            self.structure[new_parent_id] = []

        children = self.structure[new_parent_id]
        if index is None or index < 0 or index > len(children):
            children.append(block_id)
        else:
            children.insert(index, block_id)

        # Sync new parent's children list
        new_parent_block = self.blocks.get(new_parent_id)
        if new_parent_block:
            new_parent_block.children = list(self.structure[new_parent_id])

        self._touch()

    def delete_block(self, block_id: str, *, cascade: bool = True) -> None:
        """Delete a block.
        
        Args:
            block_id: ID of block to delete
            cascade: If True, delete all descendants. If False, reparent children.
            
        Raises:
            ValueError: If block not found or is root
        """
        if block_id == self.root:
            raise ValueError("Cannot delete the root block")

        block = self.blocks.get(block_id)
        if block is None:
            raise ValueError(f"Block not found: {block_id}")

        parent_id = self.parent(block_id)
        if parent_id is None:
            raise ValueError(f"Parent not found for block: {block_id}")

        # Get children before removal
        block_children = self.children(block_id)

        # Remove from parent's children
        if parent_id in self.structure:
            idx = self.structure[parent_id].index(block_id)
            self.structure[parent_id].pop(idx)

            if not cascade:
                # Reparent children to deleted block's parent
                for child_id in block_children:
                    self.structure[parent_id].insert(idx, child_id)
                    idx += 1

        # Remove from structure
        if block_id in self.structure:
            del self.structure[block_id]

        # Remove block and indices
        self.indices.remove_block(block)
        self.edge_index.remove_block(block_id)
        del self.blocks[block_id]

        # Cascade delete children
        if cascade:
            for child_id in block_children:
                self._delete_subtree(child_id)

        self._touch()

    def _delete_subtree(self, block_id: str) -> None:
        """Recursively delete a subtree."""
        block = self.blocks.get(block_id)
        if block is None:
            return

        # Delete children first
        for child_id in self.children(block_id):
            self._delete_subtree(child_id)

        # Remove from structure
        if block_id in self.structure:
            del self.structure[block_id]

        # Remove block
        self.indices.remove_block(block)
        self.edge_index.remove_block(block_id)
        del self.blocks[block_id]

    def _is_descendant(self, ancestor_id: str, candidate_id: str) -> bool:
        """Check if candidate is a descendant of ancestor."""
        return candidate_id in self.descendants(ancestor_id)

    # -------------------------------------------------------------------------
    # Tag Management
    # -------------------------------------------------------------------------

    def add_tag(self, block_id: str, tag: str) -> None:
        """Add a tag to a block."""
        block = self.blocks.get(block_id)
        if block is None:
            raise ValueError(f"Block not found: {block_id}")
        
        if tag not in block.tags:
            block.add_tag(tag)
            self.indices.by_tag[tag].add(block_id)
            self._touch()

    def remove_tag(self, block_id: str, tag: str) -> None:
        """Remove a tag from a block."""
        block = self.blocks.get(block_id)
        if block is None:
            raise ValueError(f"Block not found: {block_id}")
        
        block.remove_tag(tag)
        self.indices.by_tag[tag].discard(block_id)
        self._touch()

    def block_has_tag(self, block_id: str, tag: str) -> bool:
        """Check if a block has a tag."""
        block = self.blocks.get(block_id)
        if block is None:
            raise ValueError(f"Block not found: {block_id}")
        return block.has_tag(tag)

    def find_blocks_by_tag(self, tag: str) -> List[str]:
        """Find all blocks with a tag."""
        return list(self.indices.find_by_tag(tag))

    def get_edges(self, block_id: str) -> List[Edge]:
        """Get all edges originating from a block."""
        block = self.blocks.get(block_id)
        if block is None:
            raise ValueError(f"Block not found: {block_id}")
        return list(block.edges)

    # -------------------------------------------------------------------------
    # Edge Management
    # -------------------------------------------------------------------------

    def add_edge(
        self,
        source_id: str,
        edge_type: EdgeType,
        target_id: str,
        *,
        confidence: Optional[float] = None,
        description: Optional[str] = None,
    ) -> None:
        """Add an edge between blocks."""
        source = self.blocks.get(source_id)
        if source is None:
            raise ValueError(f"Source block not found: {source_id}")
        if target_id not in self.blocks:
            raise ValueError(f"Target block not found: {target_id}")

        edge = Edge.new(edge_type, target_id)
        if confidence is not None:
            edge.with_confidence(confidence)
        if description is not None:
            edge.with_description(description)

        source.add_edge(edge)
        self.edge_index.add_edge(source_id, edge)
        self._touch()

    def remove_edge(self, source_id: str, edge_type: EdgeType, target_id: str) -> bool:
        """Remove an edge between blocks."""
        source = self.blocks.get(source_id)
        if source is None:
            raise ValueError(f"Source block not found: {source_id}")

        removed = source.remove_edge(target_id, edge_type)
        if removed:
            self.edge_index.remove_edge(source_id, target_id, edge_type)
            self._touch()
        return removed

    def has_edge(self, source_id: str, target_id: str, edge_type: EdgeType) -> bool:
        """Check if an edge exists."""
        return self.edge_index.has_edge(source_id, target_id, edge_type)

    def outgoing_edges(self, source_id: str) -> List[Tuple[EdgeType, str]]:
        """Get all outgoing edges from a block."""
        return self.edge_index.outgoing_from(source_id)

    def incoming_edges(self, target_id: str) -> List[Tuple[EdgeType, str]]:
        """Get all incoming edges to a block."""
        return self.edge_index.incoming_to(target_id)

    # -------------------------------------------------------------------------
    # Query Methods
    # -------------------------------------------------------------------------

    def find_by_type(self, content_type: ContentType) -> List[str]:
        """Find blocks by content type."""
        return list(self.indices.find_by_type(content_type.value))

    def find_by_role(self, role: SemanticRole) -> List[str]:
        """Find blocks by semantic role."""
        return list(self.indices.find_by_role(role.value))

    def find_by_label(self, label: str) -> Optional[str]:
        """Find a block by label."""
        return self.indices.find_by_label(label)

    def find_orphans(self) -> List[str]:
        """Find blocks that are not reachable from root."""
        reachable = {self.root}
        queue = [self.root]
        
        while queue:
            current = queue.pop(0)
            for child in self.children(current):
                if child not in reachable:
                    reachable.add(child)
                    queue.append(child)

        return [bid for bid in self.blocks if bid not in reachable]

    def prune_orphans(self) -> List[str]:
        """Remove all orphaned blocks. Returns list of pruned block IDs."""
        orphans = self.find_orphans()
        for block_id in orphans:
            block = self.blocks.get(block_id)
            if block:
                self.indices.remove_block(block)
                self.edge_index.remove_block(block_id)
                del self.blocks[block_id]
        
        if orphans:
            self._touch()
        return orphans

    # -------------------------------------------------------------------------
    # Validation
    # -------------------------------------------------------------------------

    def validate(self, limits: Optional[ResourceLimits] = None) -> ValidationResult:
        """Validate the document structure and content."""
        limits = limits or ResourceLimits()
        result = ValidationResult.success()

        # Check block count
        if self.block_count() > limits.max_block_count:
            result.issues.append(ValidationIssue.error(
                "E400",
                f"Document has {self.block_count()} blocks, max is {limits.max_block_count}",
            ))
            result.valid = False

        # Check for cycles
        if self._has_cycles():
            result.issues.append(ValidationIssue.error(
                "E201",
                "Document structure contains a cycle",
            ))
            result.valid = False

        # Check nesting depth
        max_depth = self._max_depth()
        if max_depth > limits.max_nesting_depth:
            result.issues.append(ValidationIssue.error(
                "E403",
                f"Max nesting depth is {limits.max_nesting_depth}, document has {max_depth}",
            ))
            result.valid = False

        # Validate each block
        for block in self.blocks.values():
            block_result = self._validate_block(block, limits)
            result.merge(block_result)

        # Check for orphans (warning)
        orphans = self.find_orphans()
        for orphan in orphans:
            result.issues.append(ValidationIssue.warning(
                "E203",
                f"Block {orphan} is unreachable from root",
                block_id=orphan,
            ))

        return result

    def _validate_block(self, block: Block, limits: ResourceLimits) -> ValidationResult:
        """Validate a single block."""
        issues = []

        # Check block size
        size = block.size_bytes()
        if size > limits.max_block_size:
            issues.append(ValidationIssue.error(
                "E402",
                f"Block {block.id} has size {size}, max is {limits.max_block_size}",
                block_id=block.id,
            ))

        # Check edge count
        if len(block.edges) > limits.max_edges_per_block:
            issues.append(ValidationIssue.error(
                "E404",
                f"Block {block.id} has {len(block.edges)} edges, max is {limits.max_edges_per_block}",
                block_id=block.id,
            ))

        # Check edge targets exist
        for edge in block.edges:
            if edge.target not in self.blocks:
                issues.append(ValidationIssue.error(
                    "E001",
                    f"Block {block.id} has edge to non-existent block {edge.target}",
                    block_id=block.id,
                ))

        return ValidationResult.failure(issues) if issues else ValidationResult.success()

    def _has_cycles(self) -> bool:
        """Check for cycles in document structure."""
        visited: Set[str] = set()
        rec_stack: Set[str] = set()

        def dfs(node: str) -> bool:
            visited.add(node)
            rec_stack.add(node)

            for child in self.children(node):
                if child not in visited:
                    if dfs(child):
                        return True
                elif child in rec_stack:
                    return True

            rec_stack.remove(node)
            return False

        return dfs(self.root)

    def _max_depth(self) -> int:
        """Compute maximum nesting depth."""
        def depth_from(node: str, current: int) -> int:
            children = self.children(node)
            if not children:
                return current
            return max(depth_from(c, current + 1) for c in children)

        return depth_from(self.root, 1)

    # -------------------------------------------------------------------------
    # Index Rebuilding
    # -------------------------------------------------------------------------

    def rebuild_indices(self) -> None:
        """Rebuild all indices from current state."""
        self.indices.rebuild(self.blocks)
        
        self.edge_index.clear()
        for block in self.blocks.values():
            for edge in block.edges:
                self.edge_index.add_edge(block.id, edge)

    # -------------------------------------------------------------------------
    # Version / Touch
    # -------------------------------------------------------------------------

    @property
    def version(self) -> int:
        """Get current version number."""
        return self._version

    def _touch(self) -> None:
        """Update modification timestamp and version."""
        self.metadata.touch()
        self._version += 1
