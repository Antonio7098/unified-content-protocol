"""
UCP - Unified Content Protocol SDK

A developer-friendly SDK for building LLM-powered content manipulation.

Example:
    >>> import ucp
    >>>
    >>> # Parse markdown into a document
    >>> doc = ucp.parse('# Hello\\n\\nWorld')
    >>>
    >>> # Get a prompt builder for your LLM
    >>> prompt = ucp.prompt().edit().append().with_short_ids().build()
    >>>
    >>> # Map IDs for token efficiency
    >>> mapper = ucp.map_ids(doc)
    >>> short_prompt = mapper.shorten(doc_description)
    >>> expanded_ucl = mapper.expand(llm_response)
"""

from __future__ import annotations

import re
from dataclasses import dataclass, field
from enum import Enum
from typing import Dict, List, Optional, Set, Tuple

__version__ = "0.1.0"
__all__ = [
    "Document",
    "Block",
    "ContentType",
    "SemanticRole",
    "Capability",
    "PromptBuilder",
    "IdMapper",
    "UclBuilder",
    "parse",
    "render",
    "create",
    "prompt",
    "map_ids",
    "ucl",
]


# =============================================================================
# TYPES
# =============================================================================


class ContentType(str, Enum):
    """Content types supported by UCM."""
    TEXT = "text"
    CODE = "code"
    TABLE = "table"
    MATH = "math"
    JSON = "json"
    MEDIA = "media"


class SemanticRole(str, Enum):
    """Semantic roles for blocks."""
    HEADING1 = "heading1"
    HEADING2 = "heading2"
    HEADING3 = "heading3"
    HEADING4 = "heading4"
    HEADING5 = "heading5"
    HEADING6 = "heading6"
    PARAGRAPH = "paragraph"
    QUOTE = "quote"
    LIST = "list"
    CODE = "code"
    TABLE = "table"
    TITLE = "title"
    SUBTITLE = "subtitle"
    ABSTRACT = "abstract"


class Capability(str, Enum):
    """UCL command capabilities."""
    EDIT = "edit"
    APPEND = "append"
    MOVE = "move"
    DELETE = "delete"
    LINK = "link"
    SNAPSHOT = "snapshot"
    TRANSACTION = "transaction"


# =============================================================================
# CORE TYPES
# =============================================================================

_block_counter = 0


def _generate_id() -> str:
    """Generate a unique block ID."""
    global _block_counter
    _block_counter += 1
    return f"blk_{_block_counter:012x}"


@dataclass
class Block:
    """A content block in the document."""
    id: str
    content: str
    type: ContentType = ContentType.TEXT
    role: Optional[SemanticRole] = None
    label: Optional[str] = None
    children: List[str] = field(default_factory=list)


@dataclass
class Document:
    """A UCM document."""
    id: str
    root: str
    blocks: Dict[str, Block] = field(default_factory=dict)

    def get_block(self, block_id: str) -> Optional[Block]:
        """Get a block by ID."""
        return self.blocks.get(block_id)

    def add_block(
        self,
        parent_id: str,
        content: str,
        *,
        type: ContentType = ContentType.TEXT,
        role: Optional[SemanticRole] = None,
        label: Optional[str] = None,
    ) -> str:
        """Add a block to the document."""
        parent = self.blocks.get(parent_id)
        if parent is None:
            raise ValueError(f"Parent block not found: {parent_id}")

        block_id = _generate_id()
        block = Block(
            id=block_id,
            content=content,
            type=type,
            role=role,
            label=label,
        )

        self.blocks[block_id] = block
        parent.children.append(block_id)

        return block_id


# =============================================================================
# DOCUMENT OPERATIONS
# =============================================================================


def create() -> Document:
    """Create a new empty document."""
    root_id = _generate_id()
    root = Block(id=root_id, content="")

    return Document(
        id=f"doc_{int(__import__('time').time() * 1000):x}",
        root=root_id,
        blocks={root_id: root},
    )


def parse(markdown: str) -> Document:
    """Parse markdown into a UCM document."""
    doc = create()
    lines = markdown.split("\n")

    current_parent = doc.root
    heading_stack: List[Tuple[int, str]] = [(0, doc.root)]

    i = 0
    while i < len(lines):
        line = lines[i]

        # Skip empty lines
        if line.strip() == "":
            i += 1
            continue

        # Heading
        heading_match = re.match(r"^(#{1,6})\s+(.+)$", line)
        if heading_match:
            level = len(heading_match.group(1))
            text = heading_match.group(2)

            # Find parent (closest heading with lower level)
            while len(heading_stack) > 0 and heading_stack[-1][0] >= level:
                heading_stack.pop()
            current_parent = heading_stack[-1][1] if heading_stack else doc.root

            role = SemanticRole(f"heading{level}")
            block_id = doc.add_block(current_parent, text, role=role)

            heading_stack.append((level, block_id))
            current_parent = block_id
            i += 1
            continue

        # Code block
        if line.startswith("```"):
            code_lines: List[str] = []
            i += 1
            while i < len(lines) and not lines[i].startswith("```"):
                code_lines.append(lines[i])
                i += 1
            doc.add_block(
                current_parent,
                "\n".join(code_lines),
                type=ContentType.CODE,
                role=SemanticRole.CODE,
            )
            i += 1  # skip closing ```
            continue

        # Quote
        if line.startswith("> "):
            quote_lines: List[str] = []
            while i < len(lines) and lines[i].startswith("> "):
                quote_lines.append(lines[i][2:])
                i += 1
            doc.add_block(current_parent, "\n".join(quote_lines), role=SemanticRole.QUOTE)
            continue

        # Paragraph
        para_lines: List[str] = []
        while (
            i < len(lines)
            and lines[i].strip() != ""
            and not lines[i].startswith("#")
            and not lines[i].startswith("```")
        ):
            para_lines.append(lines[i])
            i += 1

        if para_lines:
            doc.add_block(current_parent, "\n".join(para_lines), role=SemanticRole.PARAGRAPH)

    return doc


def render(doc: Document) -> str:
    """Render a document to markdown."""
    lines: List[str] = []

    def render_block(block_id: str) -> None:
        block = doc.blocks.get(block_id)
        if block is None:
            return

        # Skip root block content
        if block_id != doc.root:
            if block.role and block.role.value.startswith("heading"):
                level = int(block.role.value[7:])
                lines.append("#" * level + " " + block.content)
                lines.append("")
            elif block.role == SemanticRole.CODE:
                lines.append("```")
                lines.append(block.content)
                lines.append("```")
                lines.append("")
            elif block.role == SemanticRole.QUOTE:
                for line in block.content.split("\n"):
                    lines.append("> " + line)
                lines.append("")
            else:
                lines.append(block.content)
                lines.append("")

        # Render children
        for child_id in block.children:
            render_block(child_id)

    render_block(doc.root)
    return "\n".join(lines).strip() + "\n"


# =============================================================================
# PROMPT BUILDER
# =============================================================================

_CAPABILITY_DOCS: Dict[Capability, str] = {
    Capability.EDIT: """### EDIT - Modify block content
```
EDIT <block_id> SET text = "<new_content>"
```""",
    Capability.APPEND: """### APPEND - Add new blocks
```
APPEND <parent_id> text :: <content>
APPEND <parent_id> code WITH label = "name" :: <content>
```""",
    Capability.MOVE: """### MOVE - Relocate blocks
```
MOVE <block_id> TO <new_parent_id>
MOVE <block_id> BEFORE <sibling_id>
MOVE <block_id> AFTER <sibling_id>
```""",
    Capability.DELETE: """### DELETE - Remove blocks
```
DELETE <block_id>
DELETE <block_id> CASCADE
```""",
    Capability.LINK: """### LINK - Manage relationships
```
LINK <source_id> references <target_id>
UNLINK <source_id> references <target_id>
```""",
    Capability.SNAPSHOT: """### SNAPSHOT - Version control
```
SNAPSHOT CREATE "name"
SNAPSHOT RESTORE "name"
```""",
    Capability.TRANSACTION: """### TRANSACTION - Atomic operations
```
ATOMIC { <commands> }
```""",
}


class PromptBuilder:
    """Fluent prompt builder for LLM agents."""

    def __init__(self) -> None:
        self._capabilities: Set[Capability] = set()
        self._short_ids = False
        self._custom_rules: List[str] = []
        self._context: Optional[str] = None

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

    def all(self) -> "PromptBuilder":
        """Enable all capabilities."""
        for cap in Capability:
            self._capabilities.add(cap)
        return self

    def with_short_ids(self) -> "PromptBuilder":
        """Use short numeric IDs (1, 2, 3) instead of full block IDs."""
        self._short_ids = True
        return self

    def with_rule(self, rule: str) -> "PromptBuilder":
        """Add a custom rule."""
        self._custom_rules.append(rule)
        return self

    def with_context(self, ctx: str) -> "PromptBuilder":
        """Add context to the prompt."""
        self._context = ctx
        return self

    def build(self) -> str:
        """Build the system prompt."""
        if not self._capabilities:
            raise ValueError("At least one capability must be enabled")

        parts: List[str] = []

        # Header
        parts.append("You are a UCL (Unified Content Language) command generator.")
        parts.append("Generate valid UCL commands to manipulate documents.")
        parts.append("")

        # Context
        if self._context:
            parts.append(self._context)
            parts.append("")

        # Command reference
        parts.append("## UCL Commands")
        parts.append("")
        for cap in self._capabilities:
            parts.append(_CAPABILITY_DOCS[cap])
            parts.append("")

        # Rules
        parts.append("## Rules")
        parts.append("1. Output ONLY UCL commands, no explanations")
        parts.append("2. Use exact block IDs as provided")
        parts.append("3. String values must be quoted")

        if self._short_ids:
            parts.append("4. Block IDs are short numbers (1, 2, 3, etc.)")
        else:
            parts.append("4. Block IDs have format: blk_XXXXXXXXXXXX")

        # Custom rules
        for i, rule in enumerate(self._custom_rules):
            parts.append(f"{5 + i}. {rule}")

        return "\n".join(parts)

    def build_prompt(self, document_description: str, task: str) -> str:
        """Build a complete prompt with document and task."""
        return f"""{document_description}

## Task
{task}

Generate the UCL command:"""


def prompt() -> PromptBuilder:
    """Create a new prompt builder."""
    return PromptBuilder()


# =============================================================================
# ID MAPPER
# =============================================================================


class IdMapper:
    """Maps long block IDs to short numbers for token efficiency."""

    def __init__(self) -> None:
        self._to_short: Dict[str, int] = {}
        self._to_long: Dict[int, str] = {}
        self._next_id = 1

    @classmethod
    def from_document(cls, doc: Document) -> "IdMapper":
        """Create a mapper from a document."""
        mapper = cls()

        # Add root first
        mapper.register(doc.root)

        # Add all blocks in sorted order for determinism
        for block_id in sorted(doc.blocks.keys()):
            if block_id != doc.root:
                mapper.register(block_id)

        return mapper

    def register(self, block_id: str) -> int:
        """Register a block ID."""
        if block_id in self._to_short:
            return self._to_short[block_id]

        short_id = self._next_id
        self._next_id += 1
        self._to_short[block_id] = short_id
        self._to_long[short_id] = block_id
        return short_id

    def get_short(self, block_id: str) -> Optional[int]:
        """Get short ID for a block."""
        return self._to_short.get(block_id)

    def get_long(self, short_id: int) -> Optional[str]:
        """Get long ID from short."""
        return self._to_long.get(short_id)

    def shorten(self, text: str) -> str:
        """Shorten all block IDs in text."""
        result = text
        # Process longer IDs first to avoid partial matches
        for long_id, short_id in sorted(
            self._to_short.items(), key=lambda x: len(x[0]), reverse=True
        ):
            result = result.replace(long_id, str(short_id))
        return result

    def expand(self, ucl: str) -> str:
        """Expand short IDs back to long IDs in UCL commands."""
        result = ucl

        # Match UCL command patterns with numbers
        patterns = [
            r"\b(EDIT|APPEND|MOVE|DELETE|LINK|UNLINK|TO|BEFORE|AFTER)\s+(\d+)",
            r"\b(references|elaborates|summarizes|supports|requires)\s+(\d+)",
        ]

        for pattern in patterns:
            def replacer(match: re.Match[str]) -> str:
                prefix = match.group(1)
                num = int(match.group(2))
                long_id = self._to_long.get(num)
                return f"{prefix} {long_id}" if long_id else match.group(0)

            result = re.sub(pattern, replacer, result)

        return result

    def describe(self, doc: Document) -> str:
        """Generate a compact document description."""
        lines: List[str] = ["Document Structure:"]

        def describe_block(block_id: str, depth: int) -> None:
            block = doc.blocks.get(block_id)
            if block is None:
                return

            indent = "  " * depth
            short_id = self._to_short.get(block_id)
            role = block.role.value if block.role else "block"
            preview = block.content[:40] + ("..." if len(block.content) > 40 else "")

            if block_id != doc.root or block.content:
                lines.append(f"{indent}[{short_id}] {role} - {preview}")

            for child_id in block.children:
                describe_block(child_id, depth + 1)

        describe_block(doc.root, 0)
        return "\n".join(lines)

    def get_mappings(self) -> List[Dict[str, object]]:
        """Get the mapping table (for debugging)."""
        return [
            {"short": short, "long": long}
            for short, long in sorted(self._to_long.items())
        ]


def map_ids(doc: Document) -> IdMapper:
    """Create an ID mapper from a document."""
    return IdMapper.from_document(doc)


# =============================================================================
# UCL BUILDER
# =============================================================================


class UclBuilder:
    """Fluent builder for UCL commands."""

    def __init__(self) -> None:
        self._commands: List[str] = []

    def edit(self, block_id: object, content: str) -> "UclBuilder":
        """Add an EDIT command."""
        escaped = content.replace('"', '\\"').replace("\n", "\\n")
        self._commands.append(f'EDIT {block_id} SET text = "{escaped}"')
        return self

    def append(
        self,
        parent_id: object,
        content: str,
        *,
        type: ContentType = ContentType.TEXT,
        label: Optional[str] = None,
    ) -> "UclBuilder":
        """Add an APPEND command."""
        label_part = f' WITH label = "{label}"' if label else ""
        self._commands.append(f"APPEND {parent_id} {type.value}{label_part} :: {content}")
        return self

    def move_to(self, block_id: object, new_parent: object) -> "UclBuilder":
        """Add a MOVE command."""
        self._commands.append(f"MOVE {block_id} TO {new_parent}")
        return self

    def move_before(self, block_id: object, sibling: object) -> "UclBuilder":
        """Add a MOVE BEFORE command."""
        self._commands.append(f"MOVE {block_id} BEFORE {sibling}")
        return self

    def move_after(self, block_id: object, sibling: object) -> "UclBuilder":
        """Add a MOVE AFTER command."""
        self._commands.append(f"MOVE {block_id} AFTER {sibling}")
        return self

    def delete(self, block_id: object, cascade: bool = False) -> "UclBuilder":
        """Add a DELETE command."""
        self._commands.append(f"DELETE {block_id}{' CASCADE' if cascade else ''}")
        return self

    def link(self, source: object, edge_type: str, target: object) -> "UclBuilder":
        """Add a LINK command."""
        self._commands.append(f"LINK {source} {edge_type} {target}")
        return self

    def atomic(self) -> "UclBuilder":
        """Wrap all commands in ATOMIC block."""
        if self._commands:
            inner = "\n".join("  " + c for c in self._commands)
            self._commands = [f"ATOMIC {{\n{inner}\n}}"]
        return self

    def build(self) -> str:
        """Build the final UCL string."""
        return "\n".join(self._commands)

    def to_list(self) -> List[str]:
        """Get commands as list."""
        return list(self._commands)


def ucl() -> UclBuilder:
    """Create a new UCL builder."""
    return UclBuilder()
