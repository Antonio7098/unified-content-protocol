"""
Section-based operations for UCM documents.

This module provides utilities for section-based markdown writing,
allowing efficient bulk updates to document sections.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Optional, List, Dict
import copy
import time

from .document import Document
from .block import Block
from .types import SemanticRole


@dataclass
class SectionWriteResult:
    """Result of a section write operation."""

    success: bool
    section_id: str
    blocks_removed: List[str]
    blocks_added: List[str]
    error: Optional[str] = None


@dataclass
class DeletedSectionContent:
    """Preserved section content for restore operations."""

    parent_id: str
    blocks: Dict[str, Block] = field(default_factory=dict)
    structure: Dict[str, List[str]] = field(default_factory=dict)
    deleted_at: float = field(default_factory=lambda: time.time())

    def block_ids(self) -> List[str]:
        return list(self.blocks.keys())


@dataclass
class ClearSectionResult:
    """Result of clearing a section with undo support."""

    removed_ids: List[str]
    deleted_content: DeletedSectionContent


def write_section(
    doc: Document,
    section_id: str,
    markdown: str,
    base_heading_level: Optional[int] = None,
) -> SectionWriteResult:
    """
    Write markdown content to a section, replacing all children.

    This parses the provided markdown and replaces all content under
    the specified section (heading block) with the parsed blocks.

    Args:
        doc: The document to modify
        section_id: The block ID of the section heading
        markdown: Markdown content to write
        base_heading_level: Optional base level for heading adjustment

    Returns:
        SectionWriteResult with details of the operation

    Example:
        >>> result = write_section(doc, "blk_123", "## New Section\\n\\nContent here")
        >>> print(f"Added {len(result.blocks_added)} blocks")
    """
    from .markdown import parse as parse_markdown

    # Verify section exists
    if section_id not in doc.blocks:
        return SectionWriteResult(
            success=False,
            section_id=section_id,
            blocks_removed=[],
            blocks_added=[],
            error=f"Section not found: {section_id}",
        )

    # Clear existing children
    blocks_removed = _clear_section_children(doc, section_id)

    # Parse markdown into temporary structure
    parsed = parse_markdown(markdown)

    # Integrate parsed blocks
    blocks_added = _integrate_blocks(doc, section_id, parsed, base_heading_level)

    return SectionWriteResult(
        success=True,
        section_id=section_id,
        blocks_removed=blocks_removed,
        blocks_added=blocks_added,
    )


def clear_section_with_undo(doc: Document, section_id: str) -> ClearSectionResult:
    """Clear a section while preserving content for restoration."""

    if section_id not in doc.blocks:
        raise ValueError(f"Section not found: {section_id}")

    deleted = _capture_section_content(doc, section_id)
    removed_ids = _clear_section_children(doc, section_id)
    return ClearSectionResult(removed_ids=removed_ids, deleted_content=deleted)


def restore_deleted_section(doc: Document, content: DeletedSectionContent) -> List[str]:
    """Restore previously deleted section content."""

    if content.parent_id not in doc.blocks:
        raise ValueError(f"Parent section not found: {content.parent_id}")

    # Remove current children under the parent
    for child_id in list(doc.structure.get(content.parent_id, [])):
        _remove_subtree(doc, child_id)
    doc.structure[content.parent_id] = []

    restored_ids: List[str] = []

    # Restore blocks
    for block_id, block in content.blocks.items():
        doc.blocks[block_id] = copy.deepcopy(block)
        restored_ids.append(block_id)

    # Restore structure for deleted blocks
    for block_id, children in content.structure.items():
        if block_id != content.parent_id:
            doc.structure[block_id] = list(children)
            block = doc.blocks.get(block_id)
            if block:
                block.children = list(children)

    # Restore parent's child list
    parent_children = content.structure.get(content.parent_id, [])
    doc.structure[content.parent_id] = list(parent_children)
    parent_block = doc.blocks.get(content.parent_id)
    if parent_block:
        parent_block.children = list(parent_children)

    return restored_ids


def _clear_section_children(doc: Document, section_id: str) -> List[str]:
    """Clear all children of a section recursively."""
    removed: List[str] = []
    children = list(doc.structure.get(section_id, []))

    for child_id in children:
        removed.extend(_remove_subtree(doc, child_id))

    doc.structure[section_id] = []
    section_block = doc.blocks.get(section_id)
    if section_block:
        section_block.children = []

    return removed


def _integrate_blocks(
    doc: Document,
    parent_id: str,
    source_doc: Document,
    base_heading_level: Optional[int] = None,
) -> List[str]:
    """Integrate blocks from source document into target parent."""
    added = []

    # Get root children from source
    root_children = source_doc.structure.get(source_doc.root, [])

    for child_id in root_children:
        integrated = _integrate_subtree(doc, parent_id, source_doc, child_id, base_heading_level, 0)
        added.extend(integrated)

    return added


def _capture_section_content(doc: Document, section_id: str) -> DeletedSectionContent:
    content = DeletedSectionContent(parent_id=section_id)
    queue: List[str] = []

    parent_children = list(doc.structure.get(section_id, []))
    content.structure[section_id] = list(parent_children)
    queue.extend(parent_children)

    while queue:
        current = queue.pop(0)
        block = doc.blocks.get(current)
        if block:
            content.blocks[current] = copy.deepcopy(block)

        children = list(doc.structure.get(current, []))
        content.structure[current] = list(children)
        queue.extend(children)

    return content


def _remove_subtree(doc: Document, block_id: str) -> List[str]:
    removed: List[str] = []
    block = doc.blocks.get(block_id)
    if block is None:
        return removed

    children = list(block.children)
    for child_id in children:
        removed.extend(_remove_subtree(doc, child_id))

    parent_id = doc.parent(block_id)
    if parent_id and parent_id in doc.structure:
        doc.structure[parent_id] = [c for c in doc.structure[parent_id] if c != block_id]
        parent_block = doc.blocks.get(parent_id)
        if parent_block:
            parent_block.children = list(doc.structure[parent_id])

    doc.structure.pop(block_id, None)
    del doc.blocks[block_id]
    removed.append(block_id)
    return removed


def _integrate_subtree(
    doc: Document,
    parent_id: str,
    source_doc: Document,
    source_block_id: str,
    base_heading_level: Optional[int],
    depth: int,
) -> List[str]:
    """Recursively integrate a subtree."""
    added = []

    source_block = source_doc.blocks.get(source_block_id)
    if not source_block:
        return added

    # Copy metadata so adjustments don't mutate source
    metadata = copy.deepcopy(source_block.metadata) if source_block.metadata else None

    # Adjust heading level if specified
    if base_heading_level and metadata:
        role = metadata.semantic_role.value if metadata.semantic_role else ""
        if role and role.startswith("heading"):
            try:
                current_level = int(role[7:])
                new_level = min(6, max(1, base_heading_level + current_level - 1))
                metadata.semantic_role = SemanticRole(f"heading{new_level}")
            except ValueError:
                pass

    # Add block to document, preserving semantic role/label for indexing
    role = metadata.semantic_role if metadata and metadata.semantic_role else None
    label = metadata.label if metadata else None
    new_id = doc.add_block(
        parent_id,
        source_block.content,
        content_type=source_block.content_type,
        role=role,
        label=label,
    )
    new_block = doc.blocks[new_id]

    if metadata:
        # Re-index to include tags/metadata beyond role/label
        doc.indices.remove_block(new_block)
        new_block.metadata = metadata
        doc.indices.index_block(new_block)

    added.append(new_id)

    # Process children
    children = source_doc.structure.get(source_block_id, [])
    for child_id in children:
        child_added = _integrate_subtree(
            doc, new_id, source_doc, child_id, base_heading_level, depth + 1
        )
        added.extend(child_added)

    return added


def find_section_by_path(doc: Document, path: str) -> Optional[str]:
    """
    Find a section by its path in the document hierarchy.

    The path uses " > " as a separator between heading names.

    Args:
        doc: The document to search
        path: Path like "Introduction > Getting Started"

    Returns:
        Block ID of the section, or None if not found

    Example:
        >>> section_id = find_section_by_path(doc, "Chapter 1 > Section 1.1")
    """
    parts = [p.strip() for p in path.split(" > ")]
    if not parts:
        return None

    current_id = doc.root

    for part in parts:
        children = doc.structure.get(current_id, [])
        found = None

        for child_id in children:
            block = doc.blocks.get(child_id)
            if not block:
                continue

            # Check if this is a heading with matching text
            role = (
                block.metadata.semantic_role.value
                if block.metadata and block.metadata.semantic_role
                else ""
            )
            if role.startswith("heading"):
                content = block.content
                if isinstance(content, str) and content.strip() == part:
                    found = child_id
                    break

        if not found:
            return None
        current_id = found

    return current_id if current_id != doc.root else None


def get_all_sections(doc: Document) -> List[tuple[str, int]]:
    """
    Get all sections (heading blocks) in the document.

    Returns:
        List of (block_id, heading_level) tuples

    Example:
        >>> sections = get_all_sections(doc)
        >>> for block_id, level in sections:
        ...     print(f"H{level}: {doc.blocks[block_id].content}")
    """
    sections = []

    for block_id, block in doc.blocks.items():
        if block.metadata and block.metadata.semantic_role:
            role = block.metadata.semantic_role.value
            if role.startswith("heading"):
                try:
                    level = int(role[7:])
                    sections.append((block_id, level))
                except ValueError:
                    pass

    return sections
