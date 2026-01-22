"""Tests for Engine, ValidationPipeline, and TraversalEngine."""



class TestEngine:
    """Test Engine class with transaction support."""

    def test_engine_creation(self):
        """Test creating an engine."""
        import ucp

        engine = ucp.Engine()
        assert engine is not None

    def test_engine_with_config(self):
        """Test creating an engine with custom config."""
        import ucp

        config = ucp.EngineConfig(
            validate_on_operation=False,
            max_batch_size=5000,
            enable_transactions=True,
            enable_snapshots=False
        )
        assert config.validate_on_operation is False
        assert config.max_batch_size == 5000
        assert config.enable_transactions is True
        assert config.enable_snapshots is False

        engine = ucp.Engine(config)
        assert engine is not None

    def test_engine_validate(self):
        """Test validating a document."""
        import ucp

        engine = ucp.Engine()
        doc = ucp.parse("# Hello\n\nWorld")

        result = engine.validate(doc)
        assert result.valid is True
        assert len(result.issues) == 0

    def test_engine_begin_transaction(self):
        """Test beginning a transaction."""
        import ucp

        engine = ucp.Engine()
        txn_id = engine.begin_transaction()
        assert txn_id is not None
        assert str(txn_id).startswith("txn_")

    def test_engine_begin_named_transaction(self):
        """Test beginning a named transaction."""
        import ucp

        engine = ucp.Engine()
        txn_id = engine.begin_named_transaction("my_transaction")
        assert str(txn_id) == "my_transaction"

    def test_engine_rollback_transaction(self):
        """Test rolling back a transaction."""
        import ucp

        engine = ucp.Engine()
        txn_id = engine.begin_transaction()
        engine.rollback_transaction(txn_id)
        # Should not raise

    def test_engine_snapshots(self):
        """Test snapshot creation and restoration."""
        import ucp

        engine = ucp.Engine()
        doc = ucp.parse("# Original\n\nContent")

        # Create snapshot
        engine.create_snapshot("v1", doc, "First version")

        # Modify document
        doc.add_block(doc.root_id, "New content")
        modified_count = doc.block_count

        # Restore snapshot
        restored = engine.restore_snapshot("v1")
        assert restored.block_count < modified_count

    def test_engine_list_snapshots(self):
        """Test listing snapshots."""
        import ucp

        engine = ucp.Engine()
        doc = ucp.parse("# Test")

        engine.create_snapshot("v1", doc)
        engine.create_snapshot("v2", doc)

        snapshots = engine.list_snapshots()
        assert "v1" in snapshots
        assert "v2" in snapshots

    def test_engine_delete_snapshot(self):
        """Test deleting a snapshot."""
        import ucp

        engine = ucp.Engine()
        doc = ucp.parse("# Test")

        engine.create_snapshot("to_delete", doc)
        assert engine.delete_snapshot("to_delete") is True
        assert engine.delete_snapshot("nonexistent") is False


class TestResourceLimits:
    """Test ResourceLimits class."""

    def test_default_limits(self):
        """Test default resource limits."""
        import ucp

        limits = ucp.ResourceLimits.default_limits()
        assert limits.max_block_count == 100_000
        assert limits.max_nesting_depth == 50
        assert limits.max_edges_per_block == 1000

    def test_custom_limits(self):
        """Test custom resource limits."""
        import ucp

        limits = ucp.ResourceLimits(
            max_block_count=1000,
            max_nesting_depth=10,
            max_edges_per_block=50
        )
        assert limits.max_block_count == 1000
        assert limits.max_nesting_depth == 10
        assert limits.max_edges_per_block == 50


class TestValidationPipeline:
    """Test ValidationPipeline class."""

    def test_validation_pipeline_default(self):
        """Test validation pipeline with default limits."""
        import ucp

        pipeline = ucp.ValidationPipeline()
        doc = ucp.parse("# Test\n\nContent")

        result = pipeline.validate(doc)
        assert result.valid is True

    def test_validation_pipeline_with_limits(self):
        """Test validation pipeline with custom limits."""
        import ucp

        limits = ucp.ResourceLimits(max_block_count=2)
        pipeline = ucp.ValidationPipeline(limits)

        # Create a document with more than 2 blocks
        doc = ucp.parse("# Title\n\n## Section 1\n\n## Section 2\n\n## Section 3")

        result = pipeline.validate(doc)
        assert result.valid is False
        assert len(result.errors()) > 0

    def test_validation_result_methods(self):
        """Test ValidationResult methods."""
        import ucp

        pipeline = ucp.ValidationPipeline()
        doc = ucp.parse("# Test")

        result = pipeline.validate(doc)
        assert result.valid is True
        assert len(result.errors()) == 0
        assert isinstance(result.warnings(), list)

        # Test __bool__
        assert bool(result) is True


class TestTraversalEngine:
    """Test TraversalEngine class."""

    def test_traversal_engine_creation(self):
        """Test creating a traversal engine."""
        import ucp

        engine = ucp.TraversalEngine()
        assert engine is not None

    def test_traversal_engine_with_config(self):
        """Test creating a traversal engine with config."""
        import ucp

        config = ucp.TraversalConfig(max_depth=50, max_nodes=5000)
        assert config.max_depth == 50
        assert config.max_nodes == 5000

        engine = ucp.TraversalEngine(config)
        assert engine is not None

    def test_navigate_down(self):
        """Test navigating down from root."""
        import ucp

        doc = ucp.parse("""
# Title

## Section 1

Content 1

## Section 2

Content 2
""")

        engine = ucp.TraversalEngine()
        result = engine.navigate(doc, "down")

        assert result.total_nodes > 0
        assert len(result.nodes) > 0
        assert result.max_depth >= 1

    def test_navigate_up(self):
        """Test navigating up from a node."""
        import ucp

        doc = ucp.parse("# Title\n\n## Section\n\nContent")
        engine = ucp.TraversalEngine()

        # Get a leaf node
        blocks = doc.blocks
        leaf_id = None
        for block in blocks:
            if block.content_type == "text":
                leaf_id = block.id
                break

        if leaf_id:
            result = engine.navigate(doc, "up", start_id=leaf_id)
            assert result.total_nodes > 0

    def test_navigate_with_filter(self):
        """Test navigating with a filter."""
        import ucp

        doc = ucp.parse("""
# Title

## Section 1

Content 1

## Section 2

Content 2
""")

        engine = ucp.TraversalEngine()
        filter = ucp.TraversalFilter(include_roles=["heading1", "heading2"])

        result = engine.navigate(doc, "down", filter=filter)
        # Should only include heading blocks
        for node in result.nodes:
            if node.semantic_role:
                assert "heading" in node.semantic_role

    def test_navigate_with_depth(self):
        """Test navigating with depth limit."""
        import ucp

        doc = ucp.parse("# Title\n\n## Section\n\n### Subsection\n\nContent")
        engine = ucp.TraversalEngine()

        result = engine.navigate(doc, "down", depth=1)
        assert result.max_depth <= 1

    def test_expand(self):
        """Test expanding a node."""
        import ucp

        doc = ucp.parse("# Title\n\nChild 1\n\nChild 2")
        engine = ucp.TraversalEngine()

        result = engine.expand(doc, doc.root_id)
        assert len(result.nodes) > 0

    def test_path_to_root(self):
        """Test getting path to root."""
        import ucp

        doc = ucp.parse("# Title\n\n## Section\n\nContent")
        engine = ucp.TraversalEngine()

        # Find a non-root block
        blocks = doc.blocks
        non_root = None
        for block in blocks:
            if block.id != doc.root_id:
                non_root = block.id
                break

        if non_root:
            path = engine.path_to_root(doc, non_root)
            assert len(path) >= 2
            assert path[0] == doc.root_id

    def test_find_paths(self):
        """Test finding paths between nodes."""
        import ucp

        doc = ucp.parse("# Title\n\n## Section\n\nContent")
        engine = ucp.TraversalEngine()

        # Find paths from root to a descendant
        blocks = doc.blocks
        descendant = None
        for block in blocks:
            if block.id != doc.root_id:
                descendant = block.id
                break

        if descendant:
            paths = engine.find_paths(doc, doc.root_id, descendant)
            assert len(paths) >= 1
            assert paths[0][0] == doc.root_id

    def test_traversal_result_node_ids(self):
        """Test getting node IDs from traversal result."""
        import ucp

        doc = ucp.parse("# Title\n\nContent")
        engine = ucp.TraversalEngine()

        result = engine.navigate(doc, "down")
        ids = result.node_ids()
        assert len(ids) == len(result.nodes)

    def test_traversal_direction_constants(self):
        """Test TraversalDirection constants."""
        import ucp

        assert ucp.TraversalDirection.DOWN == "down"
        assert ucp.TraversalDirection.UP == "up"
        assert ucp.TraversalDirection.BOTH == "both"
        assert ucp.TraversalDirection.SIBLINGS == "siblings"
        assert ucp.TraversalDirection.BREADTH_FIRST == "breadth_first"
        assert ucp.TraversalDirection.DEPTH_FIRST == "depth_first"
