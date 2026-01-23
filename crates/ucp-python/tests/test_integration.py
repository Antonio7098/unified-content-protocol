"""Integration tests for parse, render, and UCL execution."""

import pytest


class TestMarkdownIntegration:
    """Test markdown parsing and rendering."""

    def test_parse_simple_markdown(self):
        """Test parsing simple markdown."""
        import ucp

        md = """# Hello World

This is a paragraph.

Another paragraph here.
"""
        doc = ucp.parse(md)

        assert doc is not None
        assert doc.block_count > 1

    def test_parse_markdown_with_code(self):
        """Test parsing markdown with code blocks."""
        import ucp

        md = """# Code Example

Here's some code:

```python
def hello():
    print("Hello!")
```
"""
        doc = ucp.parse(md)

        code_blocks = doc.find_by_type("code")
        assert len(code_blocks) >= 1

    def test_render_document(self, doc_with_blocks):
        """Test rendering document to markdown."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        md = ucp.render(doc)

        assert md is not None
        assert len(md) > 0
        assert "First paragraph" in md or "paragraph" in md.lower()

    def test_roundtrip_markdown(self):
        """Test parsing then rendering markdown."""
        import ucp

        original = """# Test Document

This is a test paragraph.

Another paragraph.
"""
        doc = ucp.parse(original)
        rendered = ucp.render(doc)

        # Should preserve structure (exact text may differ)
        assert "Test Document" in rendered
        assert "test paragraph" in rendered.lower()


class TestUclExecution:
    """Test UCL command execution."""

    def test_execute_edit(self, doc_with_blocks):
        """Test executing EDIT command."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        ucl = f'EDIT {block1} SET text = "Updated text"'
        results = ucp.execute_ucl(doc, ucl)

        assert len(results) > 0
        block = doc.get_block(block1)
        assert block.get_text() == "Updated text"

    def test_execute_append(self, doc_with_blocks):
        """Test executing APPEND command."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        initial_count = doc.block_count
        ucl = f'APPEND {root} text :: "New block content"'
        results = ucp.execute_ucl(doc, ucl)

        assert len(results) > 0
        assert doc.block_count == initial_count + 1

    def test_execute_move(self, doc_with_blocks):
        """Test executing MOVE command."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        # Move block3 from block1 to block2
        ucl = f'MOVE {block3} TO {block2}'
        results = ucp.execute_ucl(doc, ucl)

        assert len(results) > 0
        # Verify block3 is now under block2
        children = doc.children(block2)
        assert block3 in children

    def test_execute_delete(self, doc_with_blocks):
        """Test executing DELETE command."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        initial_count = doc.block_count
        ucl = f'DELETE {block2}'
        results = ucp.execute_ucl(doc, ucl)

        assert len(results) > 0
        assert doc.block_count == initial_count - 1

    def test_execute_multiple_commands(self, doc_with_blocks):
        """Test executing multiple UCL commands."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        ucl = f'''
        EDIT {block1} SET text = "Modified"
        APPEND {root} text :: "New block"
        '''
        results = ucp.execute_ucl(doc, ucl)

        assert len(results) >= 2

    def test_execute_link(self, doc_with_blocks):
        """Test executing LINK command."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        ucl = f'LINK {block1} references {block2}'
        results = ucp.execute_ucl(doc, ucl)

        assert len(results) > 0
        edges = doc.outgoing_edges(block1)
        found = any(et == ucp.EdgeType.References and target == block2 for et, target in edges)
        assert found is True


class TestErrors:
    """Test error handling."""

    def test_invalid_block_id(self):
        """Test handling invalid block ID."""
        import ucp

        with pytest.raises(Exception):
            ucp.BlockId("invalid-format")

    def test_block_not_found_error(self, empty_doc):
        """Test handling block not found."""
        import ucp

        fake_id = ucp.BlockId("blk_000000000000000000000000")
        # Should return None, not raise
        block = empty_doc.get_block(fake_id)
        assert block is None

    def test_invalid_ucl_parse_error(self, empty_doc):
        """Test handling invalid UCL."""
        import ucp

        with pytest.raises(Exception):
            ucp.execute_ucl(empty_doc, "INVALID COMMAND SYNTAX HERE!!!")

    def test_snapshot_not_found_error(self):
        """Test handling snapshot not found."""
        import ucp

        mgr = ucp.SnapshotManager()
        with pytest.raises(Exception):
            mgr.restore("nonexistent")


class TestBlockId:
    """Test BlockId operations."""

    def test_create_root_block_id(self):
        """Test creating root block ID."""
        import ucp

        root = ucp.BlockId.root()
        assert root.is_root() is True

    def test_create_block_id_from_string(self):
        """Test creating block ID from string."""
        import ucp

        # Valid format: blk_ followed by 24 hex characters
        block_id = ucp.BlockId("blk_000000000000000000000001")
        assert block_id is not None

    def test_block_id_string_conversion(self):
        """Test converting block ID to string."""
        import ucp

        root = ucp.BlockId.root()
        s = str(root)
        assert s.startswith("blk_")

    def test_block_id_equality(self):
        """Test block ID equality."""
        import ucp

        root1 = ucp.BlockId.root()
        root2 = ucp.BlockId.root()
        assert root1 == root2

    def test_block_id_hash(self):
        """Test block ID hashing (for use in sets/dicts)."""
        import ucp

        root = ucp.BlockId.root()
        s = {root}  # Should be hashable
        assert root in s


class TestBlock:
    """Test Block properties."""

    def test_block_id(self, doc_with_blocks):
        """Test getting block ID."""
        doc, root, block1, block2, block3 = doc_with_blocks

        block = doc.get_block(block1)
        assert block.id == block1

    def test_block_content(self, doc_with_blocks):
        """Test getting block content."""
        doc, root, block1, block2, block3 = doc_with_blocks

        block = doc.get_block(block1)
        content = block.content
        assert content is not None

    def test_block_content_type(self, doc_with_blocks):
        """Test getting block content type."""
        doc, root, block1, block2, block3 = doc_with_blocks

        block = doc.get_block(block1)
        assert block.content_type == "text"

    def test_block_get_text(self, doc_with_blocks):
        """Test getting block text content."""
        doc, root, block1, block2, block3 = doc_with_blocks

        block = doc.get_block(block1)
        text = block.get_text()
        assert text == "First paragraph"

    def test_block_version(self, doc_with_blocks):
        """Test block version tracking."""
        doc, root, block1, block2, block3 = doc_with_blocks

        block = doc.get_block(block1)
        assert block.version >= 0

    def test_block_timestamps(self, doc_with_blocks):
        """Test block timestamps."""
        doc, root, block1, block2, block3 = doc_with_blocks

        block = doc.get_block(block1)
        assert block.created_at is not None
        assert block.modified_at is not None

    def test_block_is_root(self, doc_with_blocks):
        """Test checking if block is root."""
        doc, root, block1, block2, block3 = doc_with_blocks

        root_block = doc.get_block(root)
        assert root_block.is_root() is True

        normal_block = doc.get_block(block1)
        assert normal_block.is_root() is False

    def test_block_has_tag(self, empty_doc):
        """Test checking if block has a tag."""
        root = empty_doc.root_id
        block_id = empty_doc.add_block(root, "Tagged", tags=["important"])

        block = empty_doc.get_block(block_id)
        assert block.has_tag("important") is True
        assert block.has_tag("nonexistent") is False

    def test_block_token_estimate(self, doc_with_blocks):
        """Test block token estimation."""
        doc, root, block1, block2, block3 = doc_with_blocks

        block = doc.get_block(block1)
        tokens = block.token_estimate()
        assert tokens > 0

    def test_block_size_bytes(self, doc_with_blocks):
        """Test block size in bytes."""
        doc, root, block1, block2, block3 = doc_with_blocks

        block = doc.get_block(block1)
        size = block.size_bytes()
        assert size > 0
