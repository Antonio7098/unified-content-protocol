"""
LLM Utilities - PromptBuilder, IdMapper, and UclBuilder.

This module provides utilities for building LLM prompts and handling
token-efficient ID mapping.
"""

from __future__ import annotations

import re
from typing import Dict, List, Optional, Set

from .document import Document
from .types import Capability


# =============================================================================
# PROMPT BUILDER
# =============================================================================


class PromptBuilder:
    """Fluent builder for LLM prompts.
    
    Example:
        prompt = (PromptBuilder()
            .edit()
            .append()
            .with_short_ids()
            .with_constraints(["max 100 words"])
            .build())
    """

    def __init__(self) -> None:
        self._capabilities: Set[Capability] = set()
        self._short_ids = False
        self._include_examples = False
        self._constraints: List[str] = []
        self._context: Optional[str] = None
        self._format: str = "markdown"

    def edit(self) -> "PromptBuilder":
        """Enable EDIT capability."""
        self._capabilities.add(Capability.EDIT)
        return self

    def append(self) -> "PromptBuilder":
        """Enable APPEND capability."""
        self._capabilities.add(Capability.APPEND)
        return self

    def move(self) -> "PromptBuilder":
        """Enable MOVE capability."""
        self._capabilities.add(Capability.MOVE)
        return self

    def delete(self) -> "PromptBuilder":
        """Enable DELETE capability."""
        self._capabilities.add(Capability.DELETE)
        return self

    def link(self) -> "PromptBuilder":
        """Enable LINK capability."""
        self._capabilities.add(Capability.LINK)
        return self

    def snapshot(self) -> "PromptBuilder":
        """Enable SNAPSHOT capability."""
        self._capabilities.add(Capability.SNAPSHOT)
        return self

    def transaction(self) -> "PromptBuilder":
        """Enable TRANSACTION capability."""
        self._capabilities.add(Capability.TRANSACTION)
        return self

    def all_capabilities(self) -> "PromptBuilder":
        """Enable all capabilities."""
        for cap in Capability:
            self._capabilities.add(cap)
        return self

    def all(self) -> "PromptBuilder":
        """Enable all capabilities (alias)."""
        return self.all_capabilities()

    def with_short_ids(self) -> "PromptBuilder":
        """Use short numeric IDs for token efficiency."""
        self._short_ids = True
        return self

    def with_examples(self) -> "PromptBuilder":
        """Include usage examples in prompt."""
        self._include_examples = True
        return self

    def with_constraints(self, constraints: List[str]) -> "PromptBuilder":
        """Add output constraints."""
        self._constraints.extend(constraints)
        return self

    def with_rule(self, rule: str) -> "PromptBuilder":
        """Add a custom rule (alias for with_constraints)."""
        self._constraints.append(rule)
        return self

    def with_context(self, context: str) -> "PromptBuilder":
        """Add additional context."""
        self._context = context
        return self

    def format(self, fmt: str) -> "PromptBuilder":
        """Set output format (markdown, json, ucl)."""
        self._format = fmt
        return self

    def build(self) -> str:
        """Build the prompt string.
        
        Raises:
            ValueError: If no capabilities are enabled
        """
        if not self._capabilities:
            raise ValueError("At least one capability must be enabled")
        
        parts: List[str] = []
        
        # Header
        parts.append("# UCL Command Reference")
        parts.append("")
        
        # ID format note
        if self._short_ids:
            parts.append("Block IDs are shown as short numbers (e.g., `1`, `2`). Use these in commands.")
            parts.append("")
        
        # Capabilities
        parts.append("## Available Commands")
        parts.append("")
        
        if Capability.EDIT in self._capabilities:
            parts.append("### EDIT")
            parts.append("Modify block content:")
            parts.append('```')
            parts.append('EDIT <block_id> SET text = "new content"')
            parts.append('```')
            if self._include_examples:
                parts.append("Example:")
                parts.append('```')
                parts.append('EDIT 1 SET text = "Updated paragraph"')
                parts.append('```')
            parts.append("")
        
        if Capability.APPEND in self._capabilities:
            parts.append("### APPEND")
            parts.append("Add new block under parent:")
            parts.append('```')
            parts.append('APPEND <parent_id> text :: content')
            parts.append('APPEND <parent_id> code WITH language="python" :: source code')
            parts.append('```')
            if self._include_examples:
                parts.append("Example:")
                parts.append('```')
                parts.append('APPEND 1 text :: This is a new paragraph.')
                parts.append('```')
            parts.append("")
        
        if Capability.MOVE in self._capabilities:
            parts.append("### MOVE")
            parts.append("Reposition a block:")
            parts.append('```')
            parts.append('MOVE <block_id> TO <parent_id>')
            parts.append('MOVE <block_id> BEFORE <sibling_id>')
            parts.append('MOVE <block_id> AFTER <sibling_id>')
            parts.append('```')
            if self._include_examples:
                parts.append("Example:")
                parts.append('```')
                parts.append('MOVE 3 AFTER 1')
                parts.append('```')
            parts.append("")
        
        if Capability.DELETE in self._capabilities:
            parts.append("### DELETE")
            parts.append("Remove a block:")
            parts.append('```')
            parts.append('DELETE <block_id>')
            parts.append('DELETE <block_id> CASCADE')
            parts.append('```')
            if self._include_examples:
                parts.append("Example:")
                parts.append('```')
                parts.append('DELETE 5')
                parts.append('```')
            parts.append("")
        
        if Capability.LINK in self._capabilities:
            parts.append("### LINK / UNLINK")
            parts.append("Create/remove relationships:")
            parts.append('```')
            parts.append('LINK <source_id> references <target_id>')
            parts.append('UNLINK <source_id> references <target_id>')
            parts.append('```')
            parts.append("Edge types: references, supports, contradicts, elaborates, summarizes")
            if self._include_examples:
                parts.append("Example:")
                parts.append('```')
                parts.append('LINK 2 supports 1')
                parts.append('```')
            parts.append("")
        
        if Capability.TRANSACTION in self._capabilities:
            parts.append("### ATOMIC")
            parts.append("Group commands atomically:")
            parts.append('```')
            parts.append('ATOMIC {')
            parts.append('  EDIT 1 SET text = "updated"')
            parts.append('  APPEND 1 text :: new child')
            parts.append('}')
            parts.append('```')
            parts.append("")
        
        # Constraints
        if self._constraints:
            parts.append("## Constraints")
            for constraint in self._constraints:
                parts.append(f"- {constraint}")
            parts.append("")
        
        # Context
        if self._context:
            parts.append("## Context")
            parts.append(self._context)
            parts.append("")
        
        # Output format
        parts.append("## Response Format")
        if self._format == "ucl":
            parts.append("Respond with UCL commands only, one per line.")
        elif self._format == "json":
            parts.append("Respond with JSON containing a `commands` array.")
        else:
            parts.append("Respond with UCL commands in a code block.")
        
        return "\n".join(parts)


# =============================================================================
# ID MAPPER
# =============================================================================


class IdMapper:
    """Maps between full block IDs and short numeric IDs.
    
    This significantly reduces token usage when working with LLMs.
    
    Example:
        mapper = IdMapper(doc)
        short_text = mapper.shorten(long_text)  # blk_123abc -> 1
        expanded_text = mapper.expand(short_text)  # 1 -> blk_123abc
    """

    def __init__(self, doc: Optional[Document] = None) -> None:
        self._doc = doc
        self._id_to_short: Dict[str, int] = {}
        self._short_to_id: Dict[int, str] = {}
        self._next_short = 1
        if doc is not None:
            self._build_mapping()

    @classmethod
    def from_document(cls, doc: Document) -> "IdMapper":
        """Create a mapper from a document."""
        return cls(doc)

    def _build_mapping(self) -> None:
        """Build ID mappings from document."""
        # Start with root
        self._add_mapping(self._doc.root)
        
        # BFS through structure
        queue = [self._doc.root]
        while queue:
            current = queue.pop(0)
            for child in self._doc.children(current):
                if child not in self._id_to_short:
                    self._add_mapping(child)
                    queue.append(child)
        
        # Add any remaining blocks not in structure
        for block_id in self._doc.blocks:
            if block_id not in self._id_to_short:
                self._add_mapping(block_id)

    def _add_mapping(self, block_id: str) -> int:
        """Add a new ID mapping."""
        short = self._next_short
        self._next_short += 1
        self._id_to_short[block_id] = short
        self._short_to_id[short] = block_id
        return short

    def get_short(self, block_id: str) -> Optional[int]:
        """Get short ID for a full block ID."""
        return self._id_to_short.get(block_id)

    def get_full(self, short_id: int) -> Optional[str]:
        """Get full block ID for a short ID."""
        return self._short_to_id.get(short_id)

    def get_long(self, short_id: int) -> Optional[str]:
        """Get full block ID for a short ID (alias for get_full)."""
        return self.get_full(short_id)

    def shorten(self, text: str) -> str:
        """Replace all full block IDs with short IDs in text."""
        result = text
        # Sort by length descending to avoid partial replacements
        for block_id in sorted(self._id_to_short.keys(), key=len, reverse=True):
            short = self._id_to_short[block_id]
            result = result.replace(block_id, str(short))
        return result

    def expand(self, text: str) -> str:
        """Replace all short IDs with full block IDs in text."""
        # Use word boundaries to avoid false matches
        result = text
        for short, block_id in sorted(self._short_to_id.items(), reverse=True):
            # Match short ID as whole word
            pattern = rf'\b{short}\b'
            result = re.sub(pattern, block_id, result)
        return result

    def get_mapping_table(self) -> str:
        """Get a formatted mapping table."""
        lines = ["| Short | Full ID |", "|-------|---------|"]
        for short in sorted(self._short_to_id.keys()):
            block_id = self._short_to_id[short]
            lines.append(f"| {short} | {block_id} |")
        return "\n".join(lines)

    def block_count(self) -> int:
        """Get number of mapped blocks."""
        return len(self._id_to_short)

    def describe(self, doc: Document) -> str:
        """Generate a normalized document description with structure and blocks."""
        lines: List[str] = ["Document structure:"]

        # Collect all block IDs in order (BFS traversal)
        all_blocks: List[str] = []
        queue = [doc.root]
        while queue:
            block_id = queue.pop(0)
            all_blocks.append(block_id)
            queue.extend(doc.children(block_id))

        # Document structure section: parent: child1 child2 ...
        for block_id in all_blocks:
            short_id = self._id_to_short.get(block_id)
            children = doc.children(block_id)
            if children:
                child_ids = " ".join(str(self._id_to_short.get(c)) for c in children)
                lines.append(f"{short_id}: {child_ids}")
            else:
                lines.append(f"{short_id}:")

        # Blocks section
        lines.append("")
        lines.append("Blocks:")
        for block_id in all_blocks:
            block = doc.get_block(block_id)
            if block is None:
                continue
            short_id = self._id_to_short.get(block_id)
            content_type = block.content_type.value if block.content_type else "text"
            # Escape content for display
            escaped_content = block.content.replace('\\', '\\\\').replace('"', '\\"').replace('\n', '\\n')
            lines.append(f'{short_id} type={content_type} content="{escaped_content}"')

        return "\n".join(lines)

    def get_mappings(self) -> List[Dict[str, object]]:
        """Get the mapping table (for debugging)."""
        return [
            {"short": short, "long": long}
            for short, long in sorted(self._short_to_id.items())
        ]


# =============================================================================
# UCL BUILDER
# =============================================================================


class UclBuilder:
    """Programmatic builder for UCL commands.
    
    Example:
        ucl = (UclBuilder()
            .edit("blk_1", "text", "new content")
            .append("blk_1", "text", "child content")
            .build())
    """

    def __init__(self) -> None:
        self._commands: List[str] = []
        self._atomic = False

    def edit(self, block_id: str, content: str, path: str = "text") -> "UclBuilder":
        """Add EDIT command.
        
        Args:
            block_id: Block to edit
            content: New content value
            path: Property path (default: "text")
        """
        escaped = content.replace('\\', '\\\\').replace('"', '\\"').replace('\n', '\\n')
        self._commands.append(f'EDIT {block_id} SET {path} = "{escaped}"')
        return self

    def append(
        self,
        parent_id: str,
        content: str,
        content_type: str = "text",
        **properties: str,
    ) -> "UclBuilder":
        """Add APPEND command.
        
        Args:
            parent_id: Parent block ID
            content: Content to append
            content_type: Type of content (default: "text")
            **properties: Additional properties like label
        """
        props = ""
        if properties:
            prop_parts = [f'{k}="{v}"' for k, v in properties.items()]
            props = " WITH " + " ".join(prop_parts)
        self._commands.append(f'APPEND {parent_id} {content_type}{props} :: {content}')
        return self

    def move_to(self, block_id: str, parent_id: str, index: Optional[int] = None) -> "UclBuilder":
        """Add MOVE TO command."""
        idx = f" INDEX {index}" if index is not None else ""
        self._commands.append(f'MOVE {block_id} TO {parent_id}{idx}')
        return self

    def move_before(self, block_id: str, sibling_id: str) -> "UclBuilder":
        """Add MOVE BEFORE command."""
        self._commands.append(f'MOVE {block_id} BEFORE {sibling_id}')
        return self

    def move_after(self, block_id: str, sibling_id: str) -> "UclBuilder":
        """Add MOVE AFTER command."""
        self._commands.append(f'MOVE {block_id} AFTER {sibling_id}')
        return self

    def delete(self, block_id: str, cascade: bool = False) -> "UclBuilder":
        """Add DELETE command."""
        casc = " CASCADE" if cascade else ""
        self._commands.append(f'DELETE {block_id}{casc}')
        return self

    def link(self, source_id: str, edge_type: str, target_id: str) -> "UclBuilder":
        """Add LINK command."""
        self._commands.append(f'LINK {source_id} {edge_type} {target_id}')
        return self

    def unlink(self, source_id: str, edge_type: str, target_id: str) -> "UclBuilder":
        """Add UNLINK command."""
        self._commands.append(f'UNLINK {source_id} {edge_type} {target_id}')
        return self

    def prune(self, condition: str = "unreachable") -> "UclBuilder":
        """Add PRUNE command."""
        self._commands.append(f'PRUNE {condition}')
        return self

    def atomic(self) -> "UclBuilder":
        """Wrap commands in ATOMIC block."""
        self._atomic = True
        return self

    def clear(self) -> "UclBuilder":
        """Clear all commands."""
        self._commands.clear()
        self._atomic = False
        return self

    def build(self) -> str:
        """Build UCL command string."""
        if not self._commands:
            return ""
        
        if self._atomic:
            lines = ["ATOMIC {"]
            for cmd in self._commands:
                lines.append(f"  {cmd}")
            lines.append("}")
            return "\n".join(lines)
        
        return "\n".join(self._commands)

    def command_count(self) -> int:
        """Get number of commands."""
        return len(self._commands)

    def to_list(self) -> List[str]:
        """Get commands as a list."""
        return list(self._commands)


# =============================================================================
# CONVENIENCE FUNCTIONS
# =============================================================================


def prompt() -> PromptBuilder:
    """Create a new PromptBuilder."""
    return PromptBuilder()


def map_ids(doc: Document) -> IdMapper:
    """Create an IdMapper for a document."""
    return IdMapper(doc)


def ucl() -> UclBuilder:
    """Create a new UclBuilder."""
    return UclBuilder()
