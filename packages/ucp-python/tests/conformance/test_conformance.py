"""
UCP Conformance Tests for Python SDK.

These tests verify that the Python SDK conforms to the UCP specification
defined in docs/conformance/README.md.
"""

import pytest
from ucp import (
    ContentType,
    EdgeType,
    parse,
    render,
    create,
    execute_ucl,
    generate_block_id,
)


class TestBlockId:
    """Test BlockId generation and format."""

    def test_block_id_format(self):
        """Block IDs should have format blk_XXXXXXXXXXXX."""
        doc = create()
        block_id = doc.add_block(doc.root_id, "Test content")

        assert block_id.startswith("blk_"), "Block ID should start with blk_"
        assert len(block_id) == 16, "Block ID should be 16 chars (blk_ + 12 hex)"

    def test_root_block_id(self):
        """Root block should have special ID."""
        doc = create()
        assert doc.root_id.startswith("blk_"), "Root ID should start with blk_"

    def test_deterministic_ids(self):
        """Same content should produce same ID (content-addressed)."""
        id1 = generate_block_id("Hello", "intro")
        id2 = generate_block_id("Hello", "intro")
        assert id1 == id2, "Same content + role should produce same ID"

    def test_different_role_different_id(self):
        """Different roles should produce different IDs."""
        id1 = generate_block_id("Hello", "intro")
        id2 = generate_block_id("Hello", "conclusion")
        assert id1 != id2, "Different roles should produce different IDs"


class TestDocument:
    """Test Document operations."""

    def test_create_document(self):
        """Creating a document should initialize with root block."""
        doc = create()

        assert doc.root_id is not None, "Document should have root"
        assert len(doc.blocks) >= 1, "Document should have at least root block"

    def test_add_block(self):
        """Adding a block should increase block count."""
        doc = create()
        initial_count = len(doc.blocks)

        block_id = doc.add_block(doc.root_id, "New block")

        assert len(doc.blocks) == initial_count + 1
        assert block_id in doc.blocks

    def test_delete_block(self):
        """Deleting a block should remove it."""
        doc = create()
        block_id = doc.add_block(doc.root_id, "To delete")

        doc.delete_block(block_id)

        assert block_id not in doc.blocks

    def test_cannot_delete_root(self):
        """Should not be able to delete root block."""
        doc = create()

        with pytest.raises(ValueError):
            doc.delete_block(doc.root_id)

    def test_move_block(self):
        """Moving a block should change its parent."""
        doc = create()
        parent1 = doc.add_block(doc.root_id, "Parent 1")
        parent2 = doc.add_block(doc.root_id, "Parent 2")
        child = doc.add_block(parent1, "Child")

        doc.move_block(child, parent2)

        assert child in doc.children(parent2)
        assert child not in doc.children(parent1)

    def test_cannot_move_to_self(self):
        """Should not be able to move block into itself."""
        doc = create()
        block_id = doc.add_block(doc.root_id, "Block")

        with pytest.raises(ValueError):
            doc.move_block(block_id, block_id)

    def test_cannot_move_to_descendant(self):
        """Should not be able to move block into its descendant."""
        doc = create()
        parent = doc.add_block(doc.root_id, "Parent")
        child = doc.add_block(parent, "Child")

        with pytest.raises(ValueError):
            doc.move_block(parent, child)


class TestContentTypes:
    """Test content type support."""

    def test_text_content(self):
        """Should support text content."""
        doc = create()
        block_id = doc.add_block(doc.root_id, "Plain text", content_type=ContentType.TEXT)

        block = doc.blocks[block_id]
        assert block.content_type == ContentType.TEXT

    def test_code_content(self):
        """Should support code content."""
        doc = create()
        block_id = doc.add_block(doc.root_id, "print('hello')", content_type=ContentType.CODE)

        block = doc.blocks[block_id]
        assert block.content_type == ContentType.CODE


class TestEdges:
    """Test edge/relationship operations."""

    def test_add_edge(self):
        """Should be able to add edges between blocks."""
        doc = create()
        source = doc.add_block(doc.root_id, "Source")
        target = doc.add_block(doc.root_id, "Target")

        doc.add_edge(source, EdgeType.REFERENCES, target)

        edges = doc.get_edges(source)
        assert len(edges) == 1
        assert edges[0].target == target

    def test_remove_edge(self):
        """Should be able to remove edges."""
        doc = create()
        source = doc.add_block(doc.root_id, "Source")
        target = doc.add_block(doc.root_id, "Target")

        doc.add_edge(source, EdgeType.REFERENCES, target)
        doc.remove_edge(source, EdgeType.REFERENCES, target)

        edges = doc.get_edges(source)
        assert len(edges) == 0


class TestValidation:
    """Test document validation."""

    def test_valid_document(self):
        """A properly structured document should be valid."""
        doc = create()
        doc.add_block(doc.root_id, "Valid block")

        result = doc.validate()

        assert result.valid

    def test_detect_orphans(self):
        """Should detect orphaned blocks as warnings."""
        doc = create()
        block_id = doc.add_block(doc.root_id, "Block")

        # Manually orphan the block (simulate corruption)
        doc._children[doc.root_id].remove(block_id)

        result = doc.validate()

        orphan_warnings = [i for i in result.issues if i.code == "E203"]
        assert len(orphan_warnings) > 0


class TestMarkdown:
    """Test markdown parsing and rendering."""

    def test_parse_simple(self):
        """Should parse simple markdown."""
        md = "# Hello\n\nWorld\n"
        doc = parse(md)

        assert len(doc.blocks) > 1

    def test_roundtrip(self):
        """Parsing and rendering should preserve content."""
        md = "# Hello\n\nThis is a paragraph.\n"
        doc = parse(md)
        rendered = render(doc)

        assert "Hello" in rendered
        assert "paragraph" in rendered

    def test_heading_hierarchy(self):
        """Should preserve heading hierarchy."""
        md = "# H1\n\n## H2\n\n### H3\n"
        doc = parse(md)

        # Find blocks with heading roles
        headings = [b for b in doc.blocks.values() if b.semantic_role]
        assert len(headings) >= 3


class TestUclExecution:
    """Test UCL command execution."""

    def test_edit_command(self):
        """Should execute EDIT command."""
        doc = create()
        block_id = doc.add_block(doc.root_id, "Original")

        ucl = f'EDIT {block_id} SET text = "Updated"'
        results = execute_ucl(doc, ucl)
        assert results.success
        assert doc.blocks[block_id].content == "Updated"

    def test_append_command(self):
        """Should execute APPEND command."""
        doc = create()
        initial_count = len(doc.blocks)

        ucl = f"APPEND {doc.root_id} text :: New content"
        results = execute_ucl(doc, ucl)
        assert results.success
        assert len(doc.blocks) == initial_count + 1

    def test_delete_command(self):
        """Should execute DELETE command."""
        doc = create()
        block_id = doc.add_block(doc.root_id, "To delete")

        ucl = f"DELETE {block_id}"
        results = execute_ucl(doc, ucl)
        assert results.success
        assert block_id not in doc.blocks

    def test_move_command(self):
        """Should execute MOVE command."""
        doc = create()
        parent1 = doc.add_block(doc.root_id, "Parent 1")
        parent2 = doc.add_block(doc.root_id, "Parent 2")
        child = doc.add_block(parent1, "Child")

        ucl = f"MOVE {child} TO {parent2}"
        result = execute_ucl(doc, ucl)

        assert result.success
        assert child in doc.children(parent2)


class TestSerialization:
    """Test document serialization."""

    def test_serialize_deserialize(self):
        """Should serialize and deserialize documents."""
        from ucp import serialize_document, deserialize_document

        doc = create(title="Test Doc")
        doc.add_block(doc.root_id, "Content")

        serialized = serialize_document(doc)
        restored = deserialize_document(serialized)

        assert restored.metadata.title == "Test Doc"
        assert len(restored.blocks) == len(doc.blocks)


class TestLimits:
    """Test resource limits."""

    def test_block_count_limit(self):
        """Should enforce block count limit."""
        from ucp import ResourceLimits

        doc = create()
        limits = ResourceLimits(max_block_count=5)

        # Add blocks up to limit
        for i in range(4):
            doc.add_block(doc.root_id, f"Block {i}")

        result = doc.validate(limits=limits)
        assert result.valid

        # Add one more to exceed
        doc.add_block(doc.root_id, "Exceeds limit")
        result = doc.validate(limits=limits)

        limit_errors = [i for i in result.issues if i.code == "E400"]
        assert len(limit_errors) > 0
