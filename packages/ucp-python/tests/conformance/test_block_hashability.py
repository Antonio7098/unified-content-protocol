"""
Conformance tests for Block hashability.

These tests verify that Block objects can be used in sets and as dictionary keys,
which is essential for IdMapper and other utilities.
"""

from ucp import Block, create, SemanticRole


class TestBlockHashability:
    """Test that Block objects can be used in sets and as dict keys."""

    def test_block_in_set(self):
        """Block can be added to a set."""
        block1 = Block.text("Hello", role=SemanticRole.PARAGRAPH)
        block2 = Block.text("World", role=SemanticRole.PARAGRAPH)

        block_set = {block1, block2}
        assert len(block_set) == 2
        assert block1 in block_set
        assert block2 in block_set

    def test_block_as_dict_key(self):
        """Block can be used as dictionary key."""
        block = Block.text("Content")
        data = {block: "metadata"}

        assert data[block] == "metadata"

    def test_same_id_same_hash(self):
        """Blocks with same ID have same hash."""
        block1 = Block.text("Content")
        # Create another Block with same ID
        block2 = Block(
            id=block1.id,
            content="Different content",
        )

        assert hash(block1) == hash(block2)

    def test_block_equality_by_id(self):
        """Block equality is determined by ID."""
        block1 = Block.text("Content")
        block2 = Block(id=block1.id, content="Other content")

        assert block1 == block2

    def test_different_id_different_block(self):
        """Different IDs mean different blocks."""
        block1 = Block.text("Same content")
        block2 = Block.text("Same content")  # Gets different ID due to namespace

        assert block1 != block2

    def test_hash_consistency(self):
        """Hash should be consistent across calls."""
        block = Block.text("Test content")

        hash1 = hash(block)
        hash2 = hash(block)

        assert hash1 == hash2

    def test_multiple_blocks_in_dict(self):
        """Multiple blocks can be used as dict keys."""
        doc = create()
        b1 = doc.add_block(doc.root_id, "First")
        b2 = doc.add_block(doc.root_id, "Second")
        b3 = doc.add_block(doc.root_id, "Third")

        block1 = doc.blocks[b1]
        block2 = doc.blocks[b2]
        block3 = doc.blocks[b3]

        mapping = {
            block1: 1,
            block2: 2,
            block3: 3,
        }

        assert len(mapping) == 3
        assert mapping[block1] == 1
        assert mapping[block2] == 2
        assert mapping[block3] == 3

    def test_set_operations(self):
        """Set operations work correctly with blocks."""
        block1 = Block.text("A")
        block2 = Block.text("B")
        block3 = Block.text("C")

        set1 = {block1, block2}
        set2 = {block2, block3}

        # Union
        union = set1 | set2
        assert len(union) == 3

        # Intersection
        intersection = set1 & set2
        assert len(intersection) == 1
        assert block2 in intersection

    def test_equality_not_implemented_for_non_blocks(self):
        """Equality with non-Block types returns NotImplemented."""
        block = Block.text("Content")

        # Should not raise, but should return False for incompatible types
        assert block != "not a block"
        assert block != 42
        assert block is not None
