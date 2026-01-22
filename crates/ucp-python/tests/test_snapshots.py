"""Tests for Snapshot management."""



class TestSnapshotManager:
    """Test SnapshotManager for document versioning."""

    def test_create_snapshot_manager(self):
        """Test creating a SnapshotManager."""
        import ucp

        mgr = ucp.SnapshotManager()
        assert len(mgr) == 0

    def test_create_snapshot_manager_with_max(self):
        """Test creating a SnapshotManager with max snapshots."""
        import ucp

        mgr = ucp.SnapshotManager(max_snapshots=5)
        assert len(mgr) == 0

    def test_create_snapshot(self, doc_with_blocks):
        """Test creating a snapshot."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mgr = ucp.SnapshotManager()
        name = mgr.create("v1", doc)

        assert name == "v1"
        assert len(mgr) == 1

    def test_create_snapshot_with_description(self, doc_with_blocks):
        """Test creating a snapshot with description."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mgr = ucp.SnapshotManager()
        mgr.create("v1", doc, description="Initial version")

        info = mgr.get("v1")
        assert info.description == "Initial version"

    def test_restore_snapshot(self, doc_with_blocks):
        """Test restoring from a snapshot."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mgr = ucp.SnapshotManager()
        mgr.create("v1", doc)

        # Make changes to original document
        doc.add_block(root, "New block after snapshot")
        original_count = doc.block_count

        # Restore
        restored = mgr.restore("v1")

        # Restored should have fewer blocks
        assert restored.block_count < original_count

    def test_get_snapshot_info(self, doc_with_blocks):
        """Test getting snapshot information."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mgr = ucp.SnapshotManager()
        mgr.create("v1", doc, description="First version")

        info = mgr.get("v1")
        assert info is not None
        assert info.name == "v1"
        assert info.description == "First version"
        assert info.created_at is not None
        assert info.version >= 0

    def test_get_nonexistent_snapshot(self):
        """Test getting nonexistent snapshot returns None."""
        import ucp

        mgr = ucp.SnapshotManager()
        info = mgr.get("nonexistent")
        assert info is None

    def test_list_snapshots(self, doc_with_blocks):
        """Test listing all snapshots."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mgr = ucp.SnapshotManager()
        mgr.create("v1", doc)
        mgr.create("v2", doc)
        mgr.create("v3", doc)

        snapshots = mgr.list()
        assert len(snapshots) == 3
        # Should be sorted most recent first
        names = [s.name for s in snapshots]
        assert "v1" in names
        assert "v2" in names
        assert "v3" in names

    def test_delete_snapshot(self, doc_with_blocks):
        """Test deleting a snapshot."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mgr = ucp.SnapshotManager()
        mgr.create("v1", doc)

        assert mgr.exists("v1") is True

        deleted = mgr.delete("v1")
        assert deleted is True
        assert mgr.exists("v1") is False

    def test_delete_nonexistent_snapshot(self):
        """Test deleting nonexistent snapshot."""
        import ucp

        mgr = ucp.SnapshotManager()
        deleted = mgr.delete("nonexistent")
        assert deleted is False

    def test_exists(self, doc_with_blocks):
        """Test checking if snapshot exists."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mgr = ucp.SnapshotManager()
        mgr.create("v1", doc)

        assert mgr.exists("v1") is True
        assert mgr.exists("v2") is False

    def test_snapshot_eviction(self, doc_with_blocks):
        """Test that old snapshots are evicted when max is reached."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mgr = ucp.SnapshotManager(max_snapshots=2)
        mgr.create("v1", doc)
        mgr.create("v2", doc)
        mgr.create("v3", doc)  # Should evict v1

        assert len(mgr) == 2
        assert mgr.exists("v1") is False  # Should be evicted
        assert mgr.exists("v2") is True
        assert mgr.exists("v3") is True

    def test_restore_preserves_content(self, doc_with_blocks):
        """Test that restore preserves document content."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        # Get original content
        original_block = doc.get_block(block1)
        original_text = original_block.get_text()

        # Create snapshot
        mgr = ucp.SnapshotManager()
        mgr.create("v1", doc)

        # Modify document
        doc.edit_block(block1, "Modified content")
        modified_block = doc.get_block(block1)
        assert modified_block.get_text() == "Modified content"

        # Restore
        restored = mgr.restore("v1")
        restored_block = restored.get_block(block1)
        assert restored_block.get_text() == original_text

    def test_multiple_snapshots_independent(self, doc_with_blocks):
        """Test that multiple snapshots are independent."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mgr = ucp.SnapshotManager()

        # Snapshot v1
        mgr.create("v1", doc)

        # Add more content
        doc.add_block(root, "New content")

        # Snapshot v2
        mgr.create("v2", doc)

        # Restore v1 - should not have new_block
        restored_v1 = mgr.restore("v1")
        assert restored_v1.block_count == 4

        # Restore v2 - should have new_block
        restored_v2 = mgr.restore("v2")
        assert restored_v2.block_count == 5


class TestSnapshotInfo:
    """Test SnapshotInfo properties."""

    def test_snapshot_info_repr(self, doc_with_blocks):
        """Test SnapshotInfo string representation."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mgr = ucp.SnapshotManager()
        mgr.create("test-snapshot", doc)

        info = mgr.get("test-snapshot")
        repr_str = repr(info)

        assert "SnapshotInfo" in repr_str
        assert "test-snapshot" in repr_str
