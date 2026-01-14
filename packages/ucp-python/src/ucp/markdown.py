"""
Markdown - Parser and renderer for markdown documents.

This module provides markdown parsing into Document structure
and rendering Documents back to markdown.
"""

from __future__ import annotations

import re
from typing import List, Tuple

from .block import Block
from .document import Document
from .types import ContentType, SemanticRole


# =============================================================================
# MARKDOWN PARSER
# =============================================================================


class MarkdownParser:
    """Parse markdown text into a Document structure."""

    # Regex patterns
    HEADING_PATTERN = re.compile(r'^(#{1,6})\s+(.+)$')
    CODE_FENCE_PATTERN = re.compile(r'^```(\w*)$')
    QUOTE_PATTERN = re.compile(r'^>\s*(.*)$')
    LIST_ITEM_PATTERN = re.compile(r'^(\s*)[-*+]\s+(.+)$')
    NUMBERED_LIST_PATTERN = re.compile(r'^(\s*)\d+\.\s+(.+)$')

    def parse(self, markdown: str) -> Document:
        """Parse markdown string into a Document."""
        doc = Document.create()
        root = doc.root
        
        lines = markdown.split('\n')
        i = 0
        
        # Stack of (indent_level, parent_id) for hierarchical parsing
        # Headings: (heading_level, block_id)
        heading_stack: List[Tuple[int, str]] = [(0, root)]
        
        while i < len(lines):
            line = lines[i]
            
            # Empty line
            if not line.strip():
                i += 1
                continue
            
            # Heading
            heading_match = self.HEADING_PATTERN.match(line)
            if heading_match:
                level = len(heading_match.group(1))
                text = heading_match.group(2)
                role = self._heading_role(level)
                
                # Find appropriate parent
                while heading_stack and heading_stack[-1][0] >= level:
                    heading_stack.pop()
                
                parent_id = heading_stack[-1][1] if heading_stack else root
                
                block_id = doc.add_block(
                    parent_id,
                    text,
                    content_type=ContentType.TEXT,
                    role=role,
                )
                heading_stack.append((level, block_id))
                i += 1
                continue
            
            # Code fence
            code_match = self.CODE_FENCE_PATTERN.match(line)
            if code_match:
                language = code_match.group(1)
                code_lines: List[str] = []
                i += 1
                
                while i < len(lines) and not self.CODE_FENCE_PATTERN.match(lines[i]):
                    code_lines.append(lines[i])
                    i += 1
                
                i += 1  # Skip closing fence
                
                parent_id = heading_stack[-1][1] if heading_stack else root
                block_id = doc.add_block(
                    parent_id,
                    '\n'.join(code_lines),
                    content_type=ContentType.CODE,
                    role=SemanticRole.CODE,
                )
                # Store language in custom metadata
                block = doc.get_block(block_id)
                if block:
                    block.metadata.custom['language'] = language
                continue
            
            # Blockquote
            quote_match = self.QUOTE_PATTERN.match(line)
            if quote_match:
                quote_lines: List[str] = []
                while i < len(lines):
                    qm = self.QUOTE_PATTERN.match(lines[i])
                    if qm:
                        quote_lines.append(qm.group(1))
                        i += 1
                    else:
                        break
                
                parent_id = heading_stack[-1][1] if heading_stack else root
                doc.add_block(
                    parent_id,
                    '\n'.join(quote_lines),
                    content_type=ContentType.TEXT,
                    role=SemanticRole.QUOTE,
                )
                continue
            
            # List item (unordered or ordered)
            unordered_match = self.LIST_ITEM_PATTERN.match(line)
            ordered_match = self.NUMBERED_LIST_PATTERN.match(line)
            list_match = unordered_match or ordered_match
            if list_match:
                is_ordered = ordered_match is not None
                list_lines: List[str] = []
                while i < len(lines):
                    lm = self.LIST_ITEM_PATTERN.match(lines[i]) or self.NUMBERED_LIST_PATTERN.match(lines[i])
                    if lm:
                        list_lines.append(lm.group(2))
                        i += 1
                    elif lines[i].strip() == '':
                        i += 1
                        break
                    else:
                        break
                
                parent_id = heading_stack[-1][1] if heading_stack else root
                # Preserve original list marker type
                if is_ordered:
                    content = '\n'.join(f'{idx}. {item}' for idx, item in enumerate(list_lines, 1))
                else:
                    content = '\n'.join(f'- {item}' for item in list_lines)
                
                block_id = doc.add_block(
                    parent_id,
                    content,
                    content_type=ContentType.TEXT,
                    role=SemanticRole.LIST,
                )
                # Store list type in metadata for round-trip fidelity
                block = doc.get_block(block_id)
                if block:
                    block.metadata.custom['list_type'] = 'ordered' if is_ordered else 'unordered'
                continue
            
            # Regular paragraph
            para_lines: List[str] = []
            while i < len(lines):
                current = lines[i]
                # Stop at empty line, heading, or special block
                if not current.strip():
                    i += 1
                    break
                if self.HEADING_PATTERN.match(current):
                    break
                if self.CODE_FENCE_PATTERN.match(current):
                    break
                if self.QUOTE_PATTERN.match(current):
                    break
                para_lines.append(current)
                i += 1
            
            if para_lines:
                parent_id = heading_stack[-1][1] if heading_stack else root
                doc.add_block(
                    parent_id,
                    ' '.join(para_lines),
                    content_type=ContentType.TEXT,
                    role=SemanticRole.PARAGRAPH,
                )
        
        return doc

    def _heading_role(self, level: int) -> SemanticRole:
        """Get semantic role for heading level."""
        mapping = {
            1: SemanticRole.HEADING1,
            2: SemanticRole.HEADING2,
            3: SemanticRole.HEADING3,
            4: SemanticRole.HEADING4,
            5: SemanticRole.HEADING5,
            6: SemanticRole.HEADING6,
        }
        return mapping.get(level, SemanticRole.HEADING6)


# =============================================================================
# MARKDOWN RENDERER
# =============================================================================


class MarkdownRenderer:
    """Render a Document to markdown text."""

    def render(self, doc: Document) -> str:
        """Render document to markdown string."""
        lines: List[str] = []
        self._render_children(doc, doc.root, lines, 0)
        return '\n'.join(lines)

    def _render_children(
        self,
        doc: Document,
        parent_id: str,
        lines: List[str],
        depth: int,
    ) -> None:
        """Recursively render children of a block."""
        for child_id in doc.children(parent_id):
            block = doc.get_block(child_id)
            if block is None:
                continue
            
            self._render_block(doc, block, lines, depth)
            self._render_children(doc, child_id, lines, depth + 1)

    def _render_block(
        self,
        doc: Document,
        block: Block,
        lines: List[str],
        depth: int,
    ) -> None:
        """Render a single block to markdown."""
        role = block.role
        content = block.content
        
        # Handle by role
        if role in (
            SemanticRole.HEADING1,
            SemanticRole.HEADING2,
            SemanticRole.HEADING3,
            SemanticRole.HEADING4,
            SemanticRole.HEADING5,
            SemanticRole.HEADING6,
        ):
            level = self._heading_level(role)
            lines.append(f'{"#" * level} {content}')
            lines.append('')
        
        elif role == SemanticRole.CODE or block.content_type == ContentType.CODE:
            language = block.metadata.custom.get('language', '')
            lines.append(f'```{language}')
            lines.append(content)
            lines.append('```')
            lines.append('')
        
        elif role == SemanticRole.QUOTE:
            for line in content.split('\n'):
                lines.append(f'> {line}')
            lines.append('')
        
        elif role == SemanticRole.LIST:
            # Check metadata for list type preference
            list_type = block.metadata.custom.get('list_type', 'unordered')
            
            # Content might already have list markers
            if content.startswith('-') or content.startswith('*') or (content and content[0].isdigit()):
                lines.append(content)
            else:
                # Apply appropriate markers based on list type
                content_lines = content.split('\n')
                if list_type == 'ordered':
                    for idx, line in enumerate(content_lines, 1):
                        lines.append(f'{idx}. {line}')
                else:
                    for line in content_lines:
                        lines.append(f'- {line}')
            lines.append('')
        
        elif role == SemanticRole.PARAGRAPH or role is None:
            if content:
                lines.append(content)
                lines.append('')
        
        else:
            # Default: just output content
            if content:
                lines.append(content)
                lines.append('')

    def _heading_level(self, role: SemanticRole) -> int:
        """Get heading level from semantic role."""
        mapping = {
            SemanticRole.HEADING1: 1,
            SemanticRole.HEADING2: 2,
            SemanticRole.HEADING3: 3,
            SemanticRole.HEADING4: 4,
            SemanticRole.HEADING5: 5,
            SemanticRole.HEADING6: 6,
        }
        return mapping.get(role, 1)


# =============================================================================
# CONVENIENCE FUNCTIONS
# =============================================================================


_parser = MarkdownParser()
_renderer = MarkdownRenderer()


def parse(markdown: str) -> Document:
    """Parse markdown string into a Document.
    
    Args:
        markdown: Markdown text
        
    Returns:
        Document instance
    """
    return _parser.parse(markdown)


def render(doc: Document) -> str:
    """Render a Document to markdown string.
    
    Args:
        doc: Document to render
        
    Returns:
        Markdown text
    """
    return _renderer.render(doc)
