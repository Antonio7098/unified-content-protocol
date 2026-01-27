"""Tests for the agent graph traversal system."""

import pytest
import ucp


class TestAgentTraversal:
    """Test the AgentTraversal class and session management."""

    def test_create_traversal(self):
        """Test creating an AgentTraversal from a document."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        assert traversal is not None

    def test_create_session_default(self):
        """Test creating a session with default config."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()
        assert session is not None
        traversal.close_session(session)

    def test_create_session_with_config(self):
        """Test creating a session with custom config."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        config = ucp.SessionConfig(name="test-agent")
        session = traversal.create_session(config)
        assert session is not None
        traversal.close_session(session)

    def test_session_id_repr(self):
        """Test session ID string representation."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()
        session_str = str(session)
        assert len(session_str) > 0
        traversal.close_session(session)


class TestNavigation:
    """Test navigation operations."""

    def test_navigate_to_root(self):
        """Test navigating to the root block."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        result = traversal.navigate_to(session, doc.root_id)
        assert result.position == doc.root_id

        traversal.close_session(session)

    def test_navigation_result_properties(self):
        """Test NavigationResult properties."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        result = traversal.navigate_to(session, doc.root_id)
        assert result.position is not None
        assert isinstance(result.refreshed, bool)

        traversal.close_session(session)

    def test_go_back_empty_history(self):
        """Test going back with empty history raises error."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        with pytest.raises(RuntimeError, match="history is empty"):
            traversal.go_back(session, 1)

        traversal.close_session(session)


class TestExpansion:
    """Test expansion operations."""

    def test_expand_down(self):
        """Test expanding down from root."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        result = traversal.expand(session, doc.root_id, "down", depth=2)
        assert result.root == doc.root_id
        assert result.total_blocks >= 1

        traversal.close_session(session)

    def test_expand_up(self):
        """Test expanding up from root."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        result = traversal.expand(session, doc.root_id, "up", depth=2)
        assert result.root == doc.root_id

        traversal.close_session(session)

    def test_expand_with_view_mode(self):
        """Test expanding with custom view mode."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        result = traversal.expand(
            session, doc.root_id, "down",
            depth=2,
            view_mode=ucp.ViewMode.preview(50)
        )
        assert result.total_blocks >= 1

        traversal.close_session(session)

    def test_expand_invalid_direction(self):
        """Test expanding with invalid direction raises error."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        with pytest.raises(ValueError, match="Invalid direction"):
            traversal.expand(session, doc.root_id, "invalid", depth=2)

        traversal.close_session(session)


class TestViewModes:
    """Test ViewMode class."""

    def test_view_mode_full(self):
        """Test creating full view mode."""
        mode = ucp.ViewMode.full()
        assert "full" in repr(mode)

    def test_view_mode_preview(self):
        """Test creating preview view mode."""
        mode = ucp.ViewMode.preview(100)
        assert "preview" in repr(mode)

    def test_view_mode_ids_only(self):
        """Test creating ids only view mode."""
        mode = ucp.ViewMode.ids_only()
        assert "ids_only" in repr(mode)

    def test_view_mode_metadata(self):
        """Test creating metadata view mode."""
        mode = ucp.ViewMode.metadata()
        assert "metadata" in repr(mode)

    def test_view_mode_adaptive(self):
        """Test creating adaptive view mode."""
        mode = ucp.ViewMode.adaptive(0.7)
        assert "adaptive" in repr(mode)


class TestViewBlock:
    """Test view block operations."""

    def test_view_block_full(self):
        """Test viewing a block with full mode."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        view = traversal.view_block(session, doc.root_id, ucp.ViewMode.full())
        assert view.block_id == doc.root_id
        assert isinstance(view.children_count, int)

        traversal.close_session(session)

    def test_view_block_preview(self):
        """Test viewing a block with preview mode."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        view = traversal.view_block(session, doc.root_id, ucp.ViewMode.preview(50))
        assert view.block_id == doc.root_id

        traversal.close_session(session)

    def test_block_view_properties(self):
        """Test BlockView properties."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        view = traversal.view_block(session, doc.root_id)
        assert view.block_id is not None
        assert isinstance(view.tags, list)
        assert isinstance(view.children_count, int)
        assert isinstance(view.incoming_edges, int)
        assert isinstance(view.outgoing_edges, int)

        traversal.close_session(session)


class TestFind:
    """Test find operations."""

    def test_find_by_role(self):
        """Test finding blocks by role."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        result = traversal.find(session, role="paragraph")
        assert result.matches is not None
        assert isinstance(result.total_searched, int)

        traversal.close_session(session)

    def test_find_by_tag(self):
        """Test finding blocks by tag."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        result = traversal.find(session, tag="important")
        assert result.matches is not None

        traversal.close_session(session)

    def test_find_by_pattern(self):
        """Test finding blocks by content pattern."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        result = traversal.find(session, pattern="test")
        assert result.matches is not None

        traversal.close_session(session)


class TestFindPathExtended:
    """Test path finding operations."""

    def test_find_path_same_node(self):
        """Test finding path between same node."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        path = traversal.find_path(session, doc.root_id, doc.root_id)
        assert len(path) == 1
        assert path[0] == doc.root_id

        traversal.close_session(session)


class TestContext:
    """Test context operations."""

    def test_context_add(self):
        """Test adding a block to context."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        traversal.context_add(session, doc.root_id, reason="test", relevance=0.9)
        # No error means success

        traversal.close_session(session)

    def test_context_add_results_without_search(self):
        """Test adding results without prior search raises error."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        with pytest.raises(RuntimeError, match="No results available"):
            traversal.context_add_results(session)

        traversal.close_session(session)

    def test_context_add_results_after_find(self):
        """Test adding results after find operation via UCL."""
        # Parse markdown to get a document with proper roles
        doc = ucp.parse("# Test\n\nThis is a paragraph.")

        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        # Use UCL FIND then CTX ADD RESULTS
        ucl = """
        FIND ROLE=paragraph
        CTX ADD RESULTS
        """
        results = traversal.execute_ucl(session, ucl)
        assert len(results) == 2

        traversal.close_session(session)

    def test_context_remove(self):
        """Test removing a block from context."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        # Add then remove
        traversal.context_add(session, doc.root_id)
        traversal.context_remove(session, doc.root_id)
        # No error means success

        traversal.close_session(session)

    def test_context_clear(self):
        """Test clearing the context."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        traversal.context_add(session, doc.root_id)
        traversal.context_clear(session)
        # No error means success

        traversal.close_session(session)

    def test_context_focus(self):
        """Test setting focus block."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        traversal.context_focus(session, doc.root_id)
        # No error means success

        traversal.close_session(session)

    def test_context_focus_clear(self):
        """Test clearing focus block."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        traversal.context_focus(session, doc.root_id)
        traversal.context_focus(session, None)
        # No error means success

        traversal.close_session(session)


class TestAgentCapabilities:
    """Test AgentCapabilities class."""

    def test_default_capabilities(self):
        """Test default capabilities."""
        caps = ucp.AgentCapabilities()
        assert caps.can_traverse
        assert caps.can_search
        assert caps.can_modify_context
        assert caps.can_coordinate

    def test_read_only_capabilities(self):
        """Test read-only capabilities."""
        caps = ucp.AgentCapabilities.read_only()
        assert caps.can_traverse
        assert not caps.can_modify_context


class TestSessionConfig:
    """Test SessionConfig class."""

    def test_session_config_default(self):
        """Test creating default session config."""
        config = ucp.SessionConfig()
        assert config is not None

    def test_session_config_with_name(self):
        """Test creating session config with name."""
        config = ucp.SessionConfig(name="my-agent")
        assert "my-agent" in repr(config)

    def test_session_config_with_view_mode(self):
        """Test creating session config with view mode."""
        config = ucp.SessionConfig().with_view_mode(ucp.ViewMode.preview(100))
        assert config is not None

    def test_session_config_with_capabilities(self):
        """Test creating session config with capabilities."""
        caps = ucp.AgentCapabilities.read_only()
        config = ucp.SessionConfig().with_capabilities(caps)
        assert config is not None


class TestSearch:
    """Test search operations."""

    def test_search_without_rag(self):
        """Test search without RAG provider gives clear error."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        with pytest.raises(RuntimeError, match="RAG provider"):
            traversal.search(session, "test query")

        traversal.close_session(session)

    def test_search_parameters(self):
        """Test search accepts limit and min_similarity parameters."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        # Should accept parameters but fail because no RAG
        with pytest.raises(RuntimeError, match="RAG provider"):
            traversal.search(session, "test query", limit=5, min_similarity=0.7)

        traversal.close_session(session)


class TestUpdateDocument:
    """Test document update functionality."""

    def test_update_document_basic(self):
        """Test updating document syncs changes."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        # Add a block after creating traversal
        block_id = doc.add_block(doc.root_id, "New content")

        # Should fail without update
        with pytest.raises(RuntimeError, match="Block not found"):
            traversal.navigate_to(session, block_id)

        # Update document
        traversal.update_document(doc)

        # Now should work
        result = traversal.navigate_to(session, block_id)
        assert result.position == block_id

        traversal.close_session(session)

    def test_update_document_expand(self):
        """Test expand works after document update."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        # Add blocks
        doc.add_block(doc.root_id, "Child 1")
        doc.add_block(doc.root_id, "Child 2")

        # Update document
        traversal.update_document(doc)

        # Expand should now see the new children
        result = traversal.expand(session, doc.root_id, "down", depth=1)
        assert result.total_blocks >= 3  # root + 2 children

        traversal.close_session(session)


class TestFindWithTags:
    """Test find with tags parameter (plural)."""

    def test_find_with_tags_list(self):
        """Test find accepts tags parameter (list)."""
        doc = ucp.parse("# Test\n\nParagraph with content")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        # Should work with tags parameter
        result = traversal.find(session, tags=["important"])
        assert isinstance(result.matches, list)

        traversal.close_session(session)

    def test_find_with_tag_singular(self):
        """Test find with tag (singular) still works."""
        doc = ucp.parse("# Test\n\nParagraph")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        result = traversal.find(session, tag="important")
        assert isinstance(result.matches, list)

        traversal.close_session(session)


class TestExpandDirections:
    """Test expand direction parameter variations."""

    def test_expand_semantic_direction(self):
        """Test expand with direction='semantic'."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        # Should work with direction='semantic'
        result = traversal.expand(session, doc.root_id, direction="semantic", depth=1)
        assert result.root == doc.root_id

        traversal.close_session(session)

    def test_expand_invalid_direction_message(self):
        """Test expand with invalid direction gives clear error."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        with pytest.raises(ValueError) as exc_info:
            traversal.expand(session, doc.root_id, direction="invalid")

        # Should provide helpful message
        assert "semantic" in str(exc_info.value).lower()
        assert "down" in str(exc_info.value).lower()

        traversal.close_session(session)

    def test_expand_all_directions(self):
        """Test expand works with all valid directions."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        for direction in ["down", "up", "both", "semantic"]:
            result = traversal.expand(session, doc.root_id, direction=direction, depth=1)
            assert result.root == doc.root_id

        traversal.close_session(session)


class TestExecuteUCL:
    """Test UCL command execution."""

    def test_execute_ucl_goto(self):
        """Test executing GOTO command."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        results = traversal.execute_ucl(session, f"GOTO {doc.root_id}")
        assert len(results) == 1

        traversal.close_session(session)

    def test_execute_ucl_multiple_commands(self):
        """Test executing multiple UCL commands."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        ucl = f"""
        GOTO {doc.root_id}
        EXPAND {doc.root_id} DOWN DEPTH=2
        FIND ROLE=paragraph
        """
        results = traversal.execute_ucl(session, ucl)
        assert len(results) == 3

        traversal.close_session(session)

    def test_execute_ucl_ctx_commands(self):
        """Test executing CTX commands."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        ucl = f"""
        CTX ADD {doc.root_id}
        CTX FOCUS {doc.root_id}
        CTX CLEAR
        """
        results = traversal.execute_ucl(session, ucl)
        assert len(results) == 3

        traversal.close_session(session)

    def test_execute_ucl_ctx_add_with_params(self):
        """Test CTX ADD with reason and relevance via Python API."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        # Use Python API for advanced options instead of UCL
        traversal.context_add(session, doc.root_id, reason="test", relevance=0.9)

        traversal.close_session(session)


class TestViewNeighborhood:
    """Test view_neighborhood method."""

    def test_view_neighborhood_basic(self):
        """Test basic neighborhood view."""
        doc = ucp.parse("# Test\n\nParagraph 1\n\nParagraph 2")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        # Navigate to root first
        traversal.navigate_to(session, doc.root_id)

        # View neighborhood
        result = traversal.view_neighborhood(session)

        # Should have position
        assert result.position is not None

        # Should have lists for different contexts
        assert isinstance(result.ancestors, list)
        assert isinstance(result.children, list)
        assert isinstance(result.siblings, list)
        assert isinstance(result.connections, list)

        traversal.close_session(session)

    def test_view_neighborhood_with_children(self):
        """Test neighborhood shows children."""
        doc = ucp.create("Test Document")
        doc.add_block(doc.root_id, "Child 1")
        doc.add_block(doc.root_id, "Child 2")

        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        # Navigate to root
        traversal.navigate_to(session, doc.root_id)

        # View neighborhood
        result = traversal.view_neighborhood(session)

        # Should show children
        assert len(result.children) >= 2

        traversal.close_session(session)

    def test_view_neighborhood_representation(self):
        """Test NeighborhoodView repr."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        traversal.navigate_to(session, doc.root_id)
        result = traversal.view_neighborhood(session)

        # Should have a string representation
        assert "NeighborhoodView" in repr(result)

        traversal.close_session(session)


class TestFindPath:
    """Test find_path method (path finding between blocks)."""

    def test_find_path_basic(self):
        """Test finding path between two blocks."""
        doc = ucp.create("Test Document")
        child = doc.add_block(doc.root_id, "Child block")

        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        # Find path from root to child
        path = traversal.find_path(session, doc.root_id, child)

        # Path should include both blocks
        assert len(path) >= 2
        assert doc.root_id in path
        assert child in path

        traversal.close_session(session)

    def test_find_path_same_block(self):
        """Test finding path from block to itself."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        # Find path to self
        path = traversal.find_path(session, doc.root_id, doc.root_id)

        # Should have at least one element
        assert len(path) >= 1
        assert doc.root_id in path

        traversal.close_session(session)

    def test_find_path_with_max_length(self):
        """Test find_path respects max_length parameter."""
        doc = ucp.create("Test Document")
        level1 = doc.add_block(doc.root_id, "Level 1")
        level2 = doc.add_block(level1, "Level 2")
        level3 = doc.add_block(level2, "Level 3")

        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        # Should find path with sufficient length
        path = traversal.find_path(session, doc.root_id, level3, max_length=10)
        assert len(path) >= 2  # At least start and end

        traversal.close_session(session)


class TestContextRelevance:
    """Test context_add with relevance parameter."""

    def test_context_add_with_relevance(self):
        """Test context_add accepts relevance parameter."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        # Should work with relevance parameter
        traversal.context_add(session, doc.root_id, relevance=0.8)

        traversal.close_session(session)

    def test_context_add_with_reason_and_relevance(self):
        """Test context_add accepts both reason and relevance."""
        doc = ucp.create("Test Document")
        traversal = ucp.AgentTraversal(doc)
        session = traversal.create_session()

        # Should work with both parameters
        traversal.context_add(
            session, doc.root_id,
            reason="semantic match",
            relevance=0.95
        )

        traversal.close_session(session)
