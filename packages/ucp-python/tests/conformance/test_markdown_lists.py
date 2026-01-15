"""
Regression tests for Markdown list marker preservation.

These tests verify that ordered and unordered list markers are preserved
during parsing and round-trip conversion (fix for Agent 3 report issue).
"""

from ucp import parse, render, SemanticRole


class TestUnorderedLists:
    """Test unordered list marker preservation."""

    def test_parses_unordered_list(self):
        """Parser correctly identifies unordered lists."""
        md = "- Item 1\n- Item 2\n- Item 3\n"
        doc = parse(md)

        list_blocks = [b for b in doc.blocks.values() if b.role == SemanticRole.LIST]
        assert len(list_blocks) >= 1

    def test_preserves_unordered_markers(self):
        """Unordered list markers are preserved in content."""
        md = "- First\n- Second\n- Third\n"
        doc = parse(md)

        list_blocks = [b for b in doc.blocks.values() if b.role == SemanticRole.LIST]
        assert len(list_blocks) >= 1

        content = list_blocks[0].content
        assert "- " in content  # Unordered marker preserved

    def test_stores_unordered_list_type_in_metadata(self):
        """Unordered lists store list_type='unordered' in metadata."""
        md = "- Apple\n- Banana\n"
        doc = parse(md)

        list_blocks = [b for b in doc.blocks.values() if b.role == SemanticRole.LIST]
        assert len(list_blocks) >= 1

        list_type = list_blocks[0].metadata.custom.get("list_type")
        assert list_type == "unordered"


class TestOrderedLists:
    """Test ordered list marker preservation."""

    def test_parses_ordered_list(self):
        """Parser correctly identifies ordered lists."""
        md = "1. First\n2. Second\n3. Third\n"
        doc = parse(md)

        list_blocks = [b for b in doc.blocks.values() if b.role == SemanticRole.LIST]
        assert len(list_blocks) >= 1

    def test_preserves_ordered_markers(self):
        """Ordered list markers are preserved in content."""
        md = "1. Alpha\n2. Beta\n3. Gamma\n"
        doc = parse(md)

        list_blocks = [b for b in doc.blocks.values() if b.role == SemanticRole.LIST]
        assert len(list_blocks) >= 1

        content = list_blocks[0].content
        # Should have numbered markers
        assert ". " in content

    def test_stores_ordered_list_type_in_metadata(self):
        """Ordered lists store list_type='ordered' in metadata."""
        md = "1. One\n2. Two\n"
        doc = parse(md)

        list_blocks = [b for b in doc.blocks.values() if b.role == SemanticRole.LIST]
        assert len(list_blocks) >= 1

        list_type = list_blocks[0].metadata.custom.get("list_type")
        assert list_type == "ordered"


class TestListRoundTrip:
    """Test list round-trip conversion preserves marker type."""

    def test_unordered_list_roundtrip(self):
        """Unordered lists maintain markers through round-trip."""
        original = "# Test\n\n- Item A\n- Item B\n"
        doc = parse(original)
        rendered = render(doc)

        # Should still have unordered markers
        assert "- " in rendered
        assert "Item A" in rendered
        assert "Item B" in rendered

    def test_ordered_list_roundtrip(self):
        """Ordered lists maintain markers through round-trip."""
        original = "# Test\n\n1. Step one\n2. Step two\n"
        doc = parse(original)
        rendered = render(doc)

        # Should have ordered markers (numbered)
        assert "1. " in rendered or "1." in rendered
        assert "Step one" in rendered
        assert "Step two" in rendered

    def test_mixed_document_with_lists(self):
        """Document with both list types preserves each correctly."""
        original = """# Guide

## Unordered Section

- Apple
- Banana

## Ordered Section

1. First step
2. Second step
"""
        doc = parse(original)

        list_blocks = [b for b in doc.blocks.values() if b.role == SemanticRole.LIST]
        assert len(list_blocks) >= 2

        # Check we have both types
        list_types = [b.metadata.custom.get("list_type") for b in list_blocks]
        assert "ordered" in list_types
        assert "unordered" in list_types


class TestListEdgeCases:
    """Test edge cases for list handling."""

    def test_single_item_unordered_list(self):
        """Single item unordered list is parsed correctly."""
        md = "- Only item\n"
        doc = parse(md)

        list_blocks = [b for b in doc.blocks.values() if b.role == SemanticRole.LIST]
        assert len(list_blocks) >= 1
        assert list_blocks[0].metadata.custom.get("list_type") == "unordered"

    def test_single_item_ordered_list(self):
        """Single item ordered list is parsed correctly."""
        md = "1. Only item\n"
        doc = parse(md)

        list_blocks = [b for b in doc.blocks.values() if b.role == SemanticRole.LIST]
        assert len(list_blocks) >= 1
        assert list_blocks[0].metadata.custom.get("list_type") == "ordered"

    def test_list_with_varied_markers(self):
        """Lists with varied unordered markers (-, *, +) are handled."""
        md = "- Dash item\n* Star item\n+ Plus item\n"
        doc = parse(md)

        list_blocks = [b for b in doc.blocks.values() if b.role == SemanticRole.LIST]
        assert len(list_blocks) >= 1
