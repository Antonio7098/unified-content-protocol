"""Tests for Document operations."""



class TestDocumentCreation:
    """Test document creation and basic properties."""

    def test_create_empty_document(self, empty_doc):
        """Test creating an empty document."""
        assert empty_doc is not None
        assert empty_doc.id is not None
        assert empty_doc.root_id is not None
        assert empty_doc.title is None

    def test_create_document_with_title(self, doc_with_title):
        """Test creating a document with a title."""
        assert doc_with_title.title == "Test Document"

    def test_set_title(self, empty_doc):
        """Test setting document title."""
        empty_doc.title = "New Title"
        assert empty_doc.title == "New Title"

    def test_set_description(self, empty_doc):
        """Test setting document description."""
        empty_doc.description = "A test description"
        assert empty_doc.description == "A test description"

    def test_block_count(self, empty_doc):
        """Test block count starts at 1 (root block)."""
        assert empty_doc.block_count == 1

    def test_document_version(self, empty_doc):
        """Test document version exists."""
        assert empty_doc.version >= 0

    def test_document_timestamps(self, empty_doc):
        """Test document timestamps exist."""
        assert empty_doc.created_at is not None
        assert empty_doc.modified_at is not None


class TestBlockOperations:
    """Test block CRUD operations."""

    def test_add_block(self, empty_doc):
        """Test adding a block."""
        root = empty_doc.root_id
        block_id = empty_doc.add_block(root, "Hello, World!")

        assert block_id is not None
        assert empty_doc.block_count == 2

    def test_add_block_with_role(self, empty_doc):
        """Test adding a block with a role."""
        root = empty_doc.root_id
        block_id = empty_doc.add_block(root, "Introduction", role="intro")

        block = empty_doc.get_block(block_id)
        assert block.role == "intro"

    def test_add_block_with_label(self, empty_doc):
        """Test adding a block with a label."""
        root = empty_doc.root_id
        block_id = empty_doc.add_block(root, "Labeled block", label="my-label")

        block = empty_doc.get_block(block_id)
        assert block.label == "my-label"

    def test_add_block_with_tags(self, empty_doc):
        """Test adding a block with tags."""
        root = empty_doc.root_id
        block_id = empty_doc.add_block(root, "Tagged block", tags=["important", "reviewed"])

        block = empty_doc.get_block(block_id)
        assert "important" in block.tags
        assert "reviewed" in block.tags

    def test_add_code_block(self, empty_doc):
        """Test adding a code block."""
        root = empty_doc.root_id
        block_id = empty_doc.add_code(root, "python", "print('hello')")

        block = empty_doc.get_block(block_id)
        assert block.content_type == "code"

    def test_add_block_with_content(self, empty_doc):
        """Test adding a block with specific content type."""
        import ucp
        root = empty_doc.root_id
        content = ucp.Content.markdown("# Heading")
        block_id = empty_doc.add_block_with_content(root, content)

        block = empty_doc.get_block(block_id)
        assert block.content_type == "text"

    def test_get_block(self, doc_with_blocks):
        """Test getting a block by ID."""
        doc, root, block1, block2, block3 = doc_with_blocks

        block = doc.get_block(block1)
        assert block is not None
        assert block.id == block1

    def test_get_nonexistent_block(self, empty_doc):
        """Test getting a nonexistent block returns None."""
        import ucp
        fake_id = ucp.BlockId("blk_000000000000000000000000")
        block = empty_doc.get_block(fake_id)
        assert block is None

    def test_edit_block(self, doc_with_blocks):
        """Test editing a block."""
        doc, root, block1, block2, block3 = doc_with_blocks

        doc.edit_block(block1, "Updated content")
        block = doc.get_block(block1)
        assert block.get_text() == "Updated content"

    def test_move_block(self, doc_with_blocks):
        """Test moving a block to a new parent."""
        doc, root, block1, block2, block3 = doc_with_blocks

        # Move block3 from block1 to block2
        doc.move_block(block3, block2)

        # Verify new parent
        children = doc.children(block2)
        assert block3 in children

        # Verify old parent
        children = doc.children(block1)
        assert block3 not in children

    def test_delete_block(self, doc_with_blocks):
        """Test deleting a block."""
        doc, root, block1, block2, block3 = doc_with_blocks
        initial_count = doc.block_count

        # Delete block2 (no children)
        doc.delete_block(block2)

        assert doc.block_count == initial_count - 1
        assert doc.get_block(block2) is None

    def test_delete_block_cascade(self, doc_with_blocks):
        """Test deleting a block with cascade."""
        doc, root, block1, block2, block3 = doc_with_blocks
        initial_count = doc.block_count

        # Delete block1 with cascade (should also delete block3)
        doc.delete_block(block1, cascade=True)

        assert doc.block_count == initial_count - 2
        assert doc.get_block(block1) is None
        assert doc.get_block(block3) is None


class TestTraversal:
    """Test document traversal methods."""

    def test_children(self, doc_with_blocks):
        """Test getting children of a block."""
        doc, root, block1, block2, block3 = doc_with_blocks

        children = doc.children(root)
        assert block1 in children
        assert block2 in children

    def test_parent(self, doc_with_blocks):
        """Test getting parent of a block."""
        doc, root, block1, block2, block3 = doc_with_blocks

        parent = doc.parent(block1)
        assert parent == root

    def test_ancestors(self, doc_with_blocks):
        """Test getting ancestors of a block."""
        doc, root, block1, block2, block3 = doc_with_blocks

        ancestors = doc.ancestors(block3)
        assert block1 in ancestors
        assert root in ancestors

    def test_descendants(self, doc_with_blocks):
        """Test getting descendants of a block."""
        doc, root, block1, block2, block3 = doc_with_blocks

        descendants = doc.descendants(root)
        assert block1 in descendants
        assert block2 in descendants
        assert block3 in descendants

    def test_siblings(self, doc_with_blocks):
        """Test getting siblings of a block."""
        doc, root, block1, block2, block3 = doc_with_blocks

        siblings = doc.siblings(block1)
        assert block2 in siblings
        assert block1 not in siblings

    def test_depth(self, doc_with_blocks):
        """Test getting depth of a block."""
        doc, root, block1, block2, block3 = doc_with_blocks

        assert doc.depth(root) == 0
        assert doc.depth(block1) == 1
        assert doc.depth(block3) == 2

    def test_path_from_root(self, doc_with_blocks):
        """Test getting path from root to a block."""
        doc, root, block1, block2, block3 = doc_with_blocks

        path = doc.path_from_root(block3)
        assert path[0] == root
        assert path[1] == block1
        assert path[2] == block3

    def test_sibling_index(self, doc_with_blocks):
        """Test getting sibling index of a block."""
        doc, root, block1, block2, block3 = doc_with_blocks

        assert doc.sibling_index(block1) == 0
        assert doc.sibling_index(block2) == 1

    def test_is_reachable(self, doc_with_blocks):
        """Test checking if a block is reachable."""
        doc, root, block1, block2, block3 = doc_with_blocks

        assert doc.is_reachable(block1) is True
        assert doc.is_reachable(block3) is True

    def test_is_ancestor(self, doc_with_blocks):
        """Test checking if one block is ancestor of another."""
        doc, root, block1, block2, block3 = doc_with_blocks

        assert doc.is_ancestor(root, block3) is True
        assert doc.is_ancestor(block1, block3) is True
        assert doc.is_ancestor(block2, block3) is False


class TestFinding:
    """Test finding blocks by various criteria."""

    def test_find_by_tag(self, empty_doc):
        """Test finding blocks by tag."""
        root = empty_doc.root_id
        block1 = empty_doc.add_block(root, "Block 1", tags=["important"])
        block2 = empty_doc.add_block(root, "Block 2", tags=["important", "reviewed"])
        block3 = empty_doc.add_block(root, "Block 3", tags=["reviewed"])

        important = empty_doc.find_by_tag("important")
        assert block1 in important
        assert block2 in important
        assert block3 not in important

    def test_find_by_label(self, empty_doc):
        """Test finding a block by label."""
        root = empty_doc.root_id
        block_id = empty_doc.add_block(root, "Labeled block", label="unique-label")

        found = empty_doc.find_by_label("unique-label")
        assert found == block_id

    def test_find_by_type(self, empty_doc):
        """Test finding blocks by content type."""
        root = empty_doc.root_id
        text_block = empty_doc.add_block(root, "Text block")
        code_block = empty_doc.add_code(root, "python", "print('hi')")

        code_blocks = empty_doc.find_by_type("code")
        assert code_block in code_blocks
        assert text_block not in code_blocks

    def test_find_by_role(self, doc_with_blocks):
        """Test finding blocks by semantic role."""
        doc, root, block1, block2, block3 = doc_with_blocks

        paragraphs = doc.find_by_role("paragraph")
        assert block1 in paragraphs
        assert block2 in paragraphs
        assert block3 not in paragraphs


class TestTags:
    """Test tag operations."""

    def test_add_tag(self, doc_with_blocks):
        """Test adding a tag to a block."""
        doc, root, block1, block2, block3 = doc_with_blocks

        doc.add_tag(block1, "new-tag")
        block = doc.get_block(block1)
        assert "new-tag" in block.tags

    def test_remove_tag(self, empty_doc):
        """Test removing a tag from a block."""
        root = empty_doc.root_id
        block_id = empty_doc.add_block(root, "Tagged", tags=["tag1", "tag2"])

        removed = empty_doc.remove_tag(block_id, "tag1")
        assert removed is True

        block = empty_doc.get_block(block_id)
        assert "tag1" not in block.tags
        assert "tag2" in block.tags

    def test_set_label(self, doc_with_blocks):
        """Test setting a block's label."""
        doc, root, block1, block2, block3 = doc_with_blocks

        doc.set_label(block1, "new-label")
        block = doc.get_block(block1)
        assert block.label == "new-label"


class TestSerialization:
    """Test document serialization."""

    def test_to_json(self, doc_with_blocks):
        """Test serializing document to JSON."""
        doc, root, block1, block2, block3 = doc_with_blocks

        json_str = doc.to_json()
        assert json_str is not None
        assert "blocks" in json_str
        assert "structure" in json_str

    def test_block_ids(self, doc_with_blocks):
        """Test getting all block IDs."""
        doc, root, block1, block2, block3 = doc_with_blocks

        ids = doc.block_ids()
        assert len(ids) == 4  # root + 3 blocks
        assert root in ids
        assert block1 in ids

    def test_blocks(self, doc_with_blocks):
        """Test getting all blocks."""
        doc, root, block1, block2, block3 = doc_with_blocks

        blocks = doc.blocks
        assert len(blocks) == 4


class TestValidation:
    """Test document validation."""

    def test_validate_valid_document(self, doc_with_blocks):
        """Test validating a valid document."""
        doc, root, block1, block2, block3 = doc_with_blocks

        issues = doc.validate()
        # May have some issues but shouldn't crash
        assert isinstance(issues, list)

    def test_find_orphans(self, doc_with_blocks):
        """Test finding orphaned blocks."""
        doc, root, block1, block2, block3 = doc_with_blocks

        orphans = doc.find_orphans()
        assert isinstance(orphans, list)
        # Should be no orphans in a well-formed document
        assert len(orphans) == 0
