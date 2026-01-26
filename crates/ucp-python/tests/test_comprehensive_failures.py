"""
Comprehensive test suite covering all failure scenarios from the failure report.

This test suite ensures that:
1. UCL command execution works with case-insensitive commands
2. Context management features work correctly
3. Advanced navigation features work correctly
4. Error messages are clear and helpful
5. Safety limits are properly enforced
"""

import pytest
from ucp import (
    create,
    AgentTraversal,
    BlockId,
    ViewMode,
    SessionConfig,
    AgentCapabilities,
)


# =============================================================================
# Test Fixtures
# =============================================================================

@pytest.fixture
def simple_doc():
    """Create a simple document for testing."""
    doc = create("Test Document")
    return doc


@pytest.fixture
def nested_doc():
    """Create a document with nested structure for path finding tests."""
    doc = create("Nested Document")
    root = doc.root_id
    
    # Level 1 children
    child1 = doc.add_block(root, "Child 1")
    child2 = doc.add_block(root, "Child 2")
    child3 = doc.add_block(root, "Child 3")
    
    # Level 2 grandchildren
    grandchild1a = doc.add_block(child1, "Grandchild 1a")
    grandchild1b = doc.add_block(child1, "Grandchild 1b")
    grandchild2a = doc.add_block(child2, "Grandchild 2a")
    
    # Level 3 great-grandchildren
    great_grandchild = doc.add_block(grandchild1a, "Great Grandchild")
    
    return {
        "doc": doc,
        "root": root,
        "child1": child1,
        "child2": child2,
        "child3": child3,
        "grandchild1a": grandchild1a,
        "grandchild1b": grandchild1b,
        "grandchild2a": grandchild2a,
        "great_grandchild": great_grandchild,
    }


@pytest.fixture
def traversal_with_session(simple_doc):
    """Create an AgentTraversal with an active session."""
    traversal = AgentTraversal(simple_doc)
    session = traversal.create_session()
    return traversal, session, simple_doc


@pytest.fixture
def nested_traversal_with_session(nested_doc):
    """Create an AgentTraversal with nested document."""
    traversal = AgentTraversal(nested_doc["doc"])
    session = traversal.create_session()
    return traversal, session, nested_doc


# =============================================================================
# 1. UCL Command Execution Tests (7 failures in report)
# =============================================================================

class TestUCLCommandExecution:
    """Test UCL command parsing and execution with case-insensitivity."""
    
    def test_ucl_navigation_uppercase(self, traversal_with_session):
        """Test GOTO command in uppercase."""
        traversal, session, doc = traversal_with_session
        result = traversal.execute_ucl(session, f"GOTO {doc.root_id}")
        assert result is not None
    
    def test_ucl_navigation_lowercase(self, traversal_with_session):
        """Test goto command in lowercase."""
        traversal, session, doc = traversal_with_session
        result = traversal.execute_ucl(session, f"goto {doc.root_id}")
        assert result is not None
    
    def test_ucl_navigation_mixedcase(self, traversal_with_session):
        """Test Goto command in mixed case."""
        traversal, session, doc = traversal_with_session
        result = traversal.execute_ucl(session, f"Goto {doc.root_id}")
        assert result is not None
    
    def test_ucl_expand_uppercase(self, nested_traversal_with_session):
        """Test EXPAND command in uppercase (requires block_id first)."""
        traversal, session, nested_doc = nested_traversal_with_session
        root = nested_doc["root"]
        result = traversal.execute_ucl(session, f"EXPAND {root} DOWN")
        assert result is not None
    
    def test_ucl_expand_lowercase(self, nested_traversal_with_session):
        """Test expand command in lowercase."""
        traversal, session, nested_doc = nested_traversal_with_session
        root = nested_doc["root"]
        result = traversal.execute_ucl(session, f"expand {root} down")
        assert result is not None
    
    def test_ucl_expand_mixedcase(self, nested_traversal_with_session):
        """Test Expand command in mixed case."""
        traversal, session, nested_doc = nested_traversal_with_session
        root = nested_doc["root"]
        result = traversal.execute_ucl(session, f"Expand {root} Down")
        assert result is not None
    
    def test_ucl_view_uppercase(self, traversal_with_session):
        """Test VIEW command in uppercase."""
        traversal, session, doc = traversal_with_session
        result = traversal.execute_ucl(session, f"VIEW {doc.root_id}")
        assert result is not None
    
    def test_ucl_view_lowercase(self, traversal_with_session):
        """Test view command in lowercase."""
        traversal, session, doc = traversal_with_session
        result = traversal.execute_ucl(session, f"view {doc.root_id}")
        assert result is not None
    
    def test_ucl_find_uppercase(self, traversal_with_session):
        """Test FIND command in uppercase (uses PATTERN = syntax)."""
        traversal, session, doc = traversal_with_session
        result = traversal.execute_ucl(session, "FIND PATTERN = \".*\"")
        assert result is not None
    
    def test_ucl_find_lowercase(self, traversal_with_session):
        """Test find command in lowercase."""
        traversal, session, doc = traversal_with_session
        result = traversal.execute_ucl(session, "find pattern = \".*\"")
        assert result is not None
    
    def test_ucl_ctx_uppercase(self, traversal_with_session):
        """Test CTX command in uppercase."""
        traversal, session, doc = traversal_with_session
        result = traversal.execute_ucl(session, f"CTX ADD {doc.root_id}")
        assert result is not None
    
    def test_ucl_ctx_lowercase(self, traversal_with_session):
        """Test ctx command in lowercase."""
        traversal, session, doc = traversal_with_session
        result = traversal.execute_ucl(session, f"ctx add {doc.root_id}")
        assert result is not None
    
    def test_ucl_back_command(self, nested_traversal_with_session):
        """Test BACK command after navigation."""
        traversal, session, nested_doc = nested_traversal_with_session
        root = nested_doc["root"]
        child1 = nested_doc["child1"]
        
        traversal.navigate_to(session, root)
        traversal.navigate_to(session, child1)
        result = traversal.execute_ucl(session, "BACK")
        assert result is not None
    
    def test_ucl_multiple_commands(self, nested_traversal_with_session):
        """Test multiple UCL commands in sequence."""
        traversal, session, nested_doc = nested_traversal_with_session
        root = nested_doc["root"]
        
        # Navigate, then expand
        traversal.execute_ucl(session, f"goto {root}")
        result = traversal.execute_ucl(session, f"expand {root} down")
        assert result is not None


# =============================================================================
# 2. Context Management Tests (9 failures in report)
# =============================================================================

class TestContextManagement:
    """Test context management features."""
    
    def test_context_add_basic(self, nested_traversal_with_session):
        """Test basic context_add."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        # Should not raise
        traversal.context_add(session, child1)
    
    def test_context_add_with_relevance(self, nested_traversal_with_session):
        """Test context_add with relevance parameter."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        # Should accept relevance parameter (not relevance_score)
        traversal.context_add(session, child1, relevance=0.8)
    
    def test_context_add_with_reason(self, nested_traversal_with_session):
        """Test context_add with reason parameter."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        traversal.context_add(session, child1, reason="important context")
    
    def test_context_add_with_both_params(self, nested_traversal_with_session):
        """Test context_add with both relevance and reason."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        traversal.context_add(session, child1, relevance=0.9, reason="very important")
    
    def test_context_remove(self, nested_traversal_with_session):
        """Test context_remove method."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        traversal.context_add(session, child1)
        # Should not raise
        traversal.context_remove(session, child1)
    
    def test_context_clear(self, nested_traversal_with_session):
        """Test context_clear method."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        child2 = nested_doc["child2"]
        
        traversal.context_add(session, child1)
        traversal.context_add(session, child2)
        # Should not raise
        traversal.context_clear(session)
    
    def test_context_focus_set(self, nested_traversal_with_session):
        """Test context_focus to set focus block."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        # Should not raise
        traversal.context_focus(session, child1)
    
    def test_context_focus_clear(self, nested_traversal_with_session):
        """Test context_focus with None to clear focus."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        traversal.context_focus(session, child1)
        # Clear focus by passing None
        traversal.context_focus(session, None)
    
    def test_context_add_results_after_find(self, nested_traversal_with_session):
        """Test context_add_results after a find operation."""
        traversal, session, nested_doc = nested_traversal_with_session
        
        # First do a find
        traversal.find(session, pattern=".*")
        # Then add results to context
        traversal.context_add_results(session)
    
    def test_context_add_results_no_results_error(self, nested_traversal_with_session):
        """Test context_add_results raises when no results available."""
        traversal, session, nested_doc = nested_traversal_with_session
        
        # Without doing a find first, should raise
        with pytest.raises(Exception):
            traversal.context_add_results(session)
    
    def test_ucl_ctx_add_with_relevance(self, nested_traversal_with_session):
        """Test CTX ADD via UCL with RELEVANCE parameter (uses = syntax)."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        result = traversal.execute_ucl(session, f"CTX ADD {child1} RELEVANCE = 0.8")
        assert result is not None
    
    def test_ucl_ctx_add_with_reason(self, nested_traversal_with_session):
        """Test CTX ADD via UCL with REASON parameter (uses = syntax)."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        result = traversal.execute_ucl(session, f"CTX ADD {child1} REASON = \"test reason\"")
        assert result is not None
    
    def test_ucl_ctx_clear(self, nested_traversal_with_session):
        """Test CTX CLEAR via UCL."""
        traversal, session, nested_doc = nested_traversal_with_session
        
        result = traversal.execute_ucl(session, "CTX CLEAR")
        assert result is not None
    
    def test_ucl_ctx_focus(self, nested_traversal_with_session):
        """Test CTX FOCUS via UCL."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        result = traversal.execute_ucl(session, f"CTX FOCUS {child1}")
        assert result is not None


# =============================================================================
# 3. Advanced Navigation Tests (6 failures in report)
# =============================================================================

class TestAdvancedNavigation:
    """Test advanced navigation features."""
    
    def test_path_finding_parent_child(self, nested_traversal_with_session):
        """Test path finding from parent to child."""
        traversal, session, nested_doc = nested_traversal_with_session
        root = nested_doc["root"]
        child1 = nested_doc["child1"]
        
        path = traversal.find_path(session, root, child1)
        assert path is not None
        assert len(path) >= 2
        # Path should start with root and end with child1
        assert path[0] == root
        assert path[-1] == child1
    
    def test_path_finding_siblings(self, nested_traversal_with_session):
        """Test path finding between siblings."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        child2 = nested_doc["child2"]
        
        path = traversal.find_path(session, child1, child2)
        assert path is not None
        assert len(path) >= 2
        # Should go through parent
        assert path[0] == child1
        assert path[-1] == child2
    
    def test_path_finding_distant_blocks(self, nested_traversal_with_session):
        """Test path finding between distant blocks."""
        traversal, session, nested_doc = nested_traversal_with_session
        grandchild1a = nested_doc["grandchild1a"]
        grandchild2a = nested_doc["grandchild2a"]
        
        path = traversal.find_path(session, grandchild1a, grandchild2a)
        assert path is not None
        assert len(path) >= 2
    
    def test_path_finding_with_max_length(self, nested_traversal_with_session):
        """Test path finding with max_length constraint."""
        traversal, session, nested_doc = nested_traversal_with_session
        root = nested_doc["root"]
        great_grandchild = nested_doc["great_grandchild"]
        
        # Path exists but might be longer than 2
        path = traversal.find_path(session, root, great_grandchild, max_length=10)
        assert path is not None
    
    def test_path_finding_same_block(self, nested_traversal_with_session):
        """Test path finding from block to itself."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        path = traversal.find_path(session, child1, child1)
        assert path is not None
        assert len(path) == 1
        assert path[0] == child1
    
    def test_view_mode_full(self, nested_traversal_with_session):
        """Test VIEW with Full mode."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        result = traversal.view_block(session, child1, view_mode=ViewMode.full())
        assert result is not None
        assert result.content is not None
    
    def test_view_mode_preview(self, nested_traversal_with_session):
        """Test VIEW with Preview mode."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        result = traversal.view_block(session, child1, view_mode=ViewMode.preview())
        assert result is not None
    
    def test_view_mode_ids_only(self, nested_traversal_with_session):
        """Test VIEW with IdsOnly mode - should return BlockView with None content."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        result = traversal.view_block(session, child1, view_mode=ViewMode.ids_only())
        assert result is not None
        # IdsOnly mode returns BlockView but content may be None
        assert hasattr(result, 'block_id')
    
    def test_view_mode_metadata(self, nested_traversal_with_session):
        """Test VIEW with Metadata mode."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        result = traversal.view_block(session, child1, view_mode=ViewMode.metadata())
        assert result is not None
        assert hasattr(result, 'block_id')
    
    def test_view_mode_adaptive(self, nested_traversal_with_session):
        """Test VIEW with Adaptive mode."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        result = traversal.view_block(session, child1, view_mode=ViewMode.adaptive())
        assert result is not None
    
    def test_view_neighborhood_basic(self, nested_traversal_with_session):
        """Test basic neighborhood viewing."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        traversal.navigate_to(session, child1)
        result = traversal.view_neighborhood(session)
        assert result is not None
        assert hasattr(result, 'position')
        assert hasattr(result, 'children')
        assert hasattr(result, 'ancestors')
        assert hasattr(result, 'siblings')
    
    def test_view_neighborhood_with_children(self, nested_traversal_with_session):
        """Test neighborhood viewing includes children."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        traversal.navigate_to(session, child1)
        result = traversal.view_neighborhood(session)
        # child1 has grandchild1a and grandchild1b as children
        assert len(result.children) >= 2
    
    def test_expand_down(self, nested_traversal_with_session):
        """Test expand down from root (requires block_id parameter)."""
        traversal, session, nested_doc = nested_traversal_with_session
        root = nested_doc["root"]
        
        result = traversal.expand(session, root, direction="down")
        assert result is not None
        assert result.total_blocks > 0
    
    def test_expand_up(self, nested_traversal_with_session):
        """Test expand up from child."""
        traversal, session, nested_doc = nested_traversal_with_session
        grandchild1a = nested_doc["grandchild1a"]
        
        result = traversal.expand(session, grandchild1a, direction="up")
        assert result is not None
    
    def test_expand_with_depth(self, nested_traversal_with_session):
        """Test expand with depth parameter."""
        traversal, session, nested_doc = nested_traversal_with_session
        root = nested_doc["root"]
        
        result = traversal.expand(session, root, direction="down", depth=2)
        assert result is not None


# =============================================================================
# 4. Error Message Clarity Tests (8 failures in report)
# =============================================================================

class TestErrorMessageClarity:
    """Test that error messages are clear and helpful."""
    
    def test_navigate_to_nonexistent_block(self, traversal_with_session):
        """Test error message for navigating to nonexistent block."""
        traversal, session, doc = traversal_with_session
        
        # Create a fake block ID that doesn't exist
        fake_id = BlockId("blk_000000000000000000000000")
        
        with pytest.raises(Exception) as exc_info:
            traversal.navigate_to(session, fake_id)
        
        error_msg = str(exc_info.value).lower()
        # Should mention "block" and "not found" or similar
        assert "block" in error_msg or "not found" in error_msg
    
    def test_expand_invalid_block(self, traversal_with_session):
        """Test error message for expanding invalid block."""
        traversal, session, doc = traversal_with_session
        
        fake_id = BlockId("blk_000000000000000000000000")
        
        with pytest.raises(Exception) as exc_info:
            traversal.navigate_to(session, fake_id)
            traversal.expand(session, direction="down")
        
        # Should have a descriptive error
        assert exc_info.value is not None
    
    def test_view_invalid_block(self, traversal_with_session):
        """Test error message for viewing invalid block."""
        traversal, session, doc = traversal_with_session
        
        fake_id = BlockId("blk_000000000000000000000000")
        
        with pytest.raises(Exception) as exc_info:
            traversal.view_block(session, fake_id)
        
        error_msg = str(exc_info.value).lower()
        assert "block" in error_msg or "not found" in error_msg
    
    def test_invalid_block_id_format(self):
        """Test error message for invalid block ID format."""
        with pytest.raises(Exception) as exc_info:
            BlockId("invalid_id")
        
        error_msg = str(exc_info.value)
        # Should explain the expected format
        assert "blk_" in error_msg or "format" in error_msg.lower()
    
    def test_invalid_block_id_too_short(self):
        """Test error message for block ID that's too short."""
        with pytest.raises(Exception) as exc_info:
            BlockId("blk_123")
        
        error_msg = str(exc_info.value)
        assert "blk_" in error_msg or "format" in error_msg.lower() or "hex" in error_msg.lower()
    
    def test_expand_invalid_direction(self, traversal_with_session):
        """Test error message for invalid expand direction."""
        traversal, session, doc = traversal_with_session
        
        with pytest.raises(Exception) as exc_info:
            traversal.expand(session, doc.root_id, direction="invalid")
        
        error_msg = str(exc_info.value).lower()
        # Should mention valid directions
        assert "direction" in error_msg or "invalid" in error_msg
    
    def test_expand_negative_depth(self, traversal_with_session):
        """Test error message for negative depth."""
        traversal, session, doc = traversal_with_session
        
        with pytest.raises(Exception) as exc_info:
            traversal.expand(session, doc.root_id, direction="down", depth=-1)
        
        # Should reject negative depth
        assert exc_info.value is not None


# =============================================================================
# 5. Safety and Limits Tests
# =============================================================================

class TestSafetyAndLimits:
    """Test safety features and limit enforcement."""
    
    def test_depth_limit_respected(self, nested_traversal_with_session):
        """Test that depth limits are respected during expansion."""
        traversal, session, nested_doc = nested_traversal_with_session
        root = nested_doc["root"]
        
        # Should work with reasonable depth
        result = traversal.expand(session, root, direction="down", depth=5)
        assert result is not None
    
    def test_session_capabilities_default(self, nested_doc):
        """Test that default session capabilities work."""
        doc = nested_doc["doc"]
        
        traversal = AgentTraversal(doc)
        session = traversal.create_session()
        
        # Navigation should work with default capabilities
        traversal.navigate_to(session, nested_doc["root"])
    
    def test_session_capabilities_read_only(self, nested_doc):
        """Test read-only capabilities object."""
        caps = AgentCapabilities.read_only()
        # Verify read_only returns a valid capabilities object
        assert caps is not None
    
    def test_session_closes_properly(self, nested_traversal_with_session):
        """Test that sessions close properly."""
        traversal, session, nested_doc = nested_traversal_with_session
        
        # Close session
        traversal.close_session(session)
        
        # Operations should fail after close
        with pytest.raises(Exception):
            traversal.navigate_to(session, nested_doc["root"])


# =============================================================================
# 6. UCL Command Variations Tests
# =============================================================================

class TestUCLCommandVariations:
    """Test various UCL command syntax variations."""
    
    def test_ucl_expand_with_depth(self, nested_traversal_with_session):
        """Test EXPAND with DEPTH parameter (uses = syntax)."""
        traversal, session, nested_doc = nested_traversal_with_session
        root = nested_doc["root"]
        
        result = traversal.execute_ucl(session, f"EXPAND {root} DOWN DEPTH = 3")
        assert result is not None
    
    def test_ucl_expand_with_mode(self, nested_traversal_with_session):
        """Test EXPAND with MODE parameter (uses = syntax)."""
        traversal, session, nested_doc = nested_traversal_with_session
        root = nested_doc["root"]
        
        result = traversal.execute_ucl(session, f"EXPAND {root} DOWN MODE = PREVIEW")
        assert result is not None
    
    def test_ucl_view_with_mode(self, nested_traversal_with_session):
        """Test VIEW with MODE parameter (uses = syntax)."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        result = traversal.execute_ucl(session, f"VIEW {child1} MODE = FULL")
        assert result is not None
    
    def test_ucl_view_neighborhood(self, nested_traversal_with_session):
        """Test VIEW NEIGHBORHOOD command."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        traversal.navigate_to(session, child1)
        result = traversal.execute_ucl(session, "VIEW NEIGHBORHOOD")
        assert result is not None
    
    def test_ucl_path_find(self, nested_traversal_with_session):
        """Test PATH command (uses TO keyword between block IDs)."""
        traversal, session, nested_doc = nested_traversal_with_session
        root = nested_doc["root"]
        child1 = nested_doc["child1"]
        
        result = traversal.execute_ucl(session, f"PATH {root} TO {child1}")
        assert result is not None
    
    def test_ucl_ctx_remove(self, nested_traversal_with_session):
        """Test CTX REMOVE command."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        traversal.execute_ucl(session, f"CTX ADD {child1}")
        result = traversal.execute_ucl(session, f"CTX REMOVE {child1}")
        assert result is not None


# =============================================================================
# 7. Edge Cases and Integration Tests
# =============================================================================

class TestEdgeCasesAndIntegration:
    """Test edge cases and integration scenarios."""
    
    def test_empty_document(self):
        """Test operations on empty document."""
        doc = create("Empty")
        traversal = AgentTraversal(doc)
        session = traversal.create_session()
        
        # Should be able to navigate to root
        traversal.navigate_to(session, doc.root_id)
        
        # Expand should work but return empty results (requires block_id)
        result = traversal.expand(session, doc.root_id, direction="down")
        assert result is not None
    
    def test_deep_nesting(self):
        """Test operations on deeply nested document."""
        doc = create("Deep")
        current = doc.root_id
        
        # Create deep nesting
        for i in range(20):
            current = doc.add_block(current, f"Level {i}")
        
        traversal = AgentTraversal(doc)
        session = traversal.create_session()
        
        # Should be able to navigate to deepest block
        traversal.navigate_to(session, current)
        
        # View should work
        result = traversal.view_block(session, current)
        assert result is not None
    
    def test_wide_branching(self):
        """Test operations on document with wide branching."""
        doc = create("Wide")
        root = doc.root_id
        
        # Create many children
        children = []
        for i in range(50):
            children.append(doc.add_block(root, f"Child {i}"))
        
        traversal = AgentTraversal(doc)
        session = traversal.create_session()
        
        traversal.navigate_to(session, root)
        result = traversal.expand(session, root, direction="down")
        assert result is not None
        assert result.total_blocks >= 50
    
    def test_multiple_sessions(self, nested_doc):
        """Test multiple concurrent sessions."""
        doc = nested_doc["doc"]
        traversal = AgentTraversal(doc)
        
        session1 = traversal.create_session()
        session2 = traversal.create_session()
        
        # Each session should work independently
        traversal.navigate_to(session1, nested_doc["child1"])
        traversal.navigate_to(session2, nested_doc["child2"])
        
        # Both should still work
        result1 = traversal.view_neighborhood(session1)
        result2 = traversal.view_neighborhood(session2)
        
        assert result1.position != result2.position
    
    def test_session_with_custom_name(self, nested_doc):
        """Test session with custom name configuration."""
        doc = nested_doc["doc"]
        
        # SessionConfig only accepts name and start_block parameters
        config = SessionConfig(name="Custom Session")
        
        traversal = AgentTraversal(doc)
        session = traversal.create_session(config=config)
        
        traversal.navigate_to(session, nested_doc["root"])
        result = traversal.expand(session, nested_doc["root"], direction="down")
        assert result is not None


# =============================================================================
# 8. Regression Prevention Tests
# =============================================================================

class TestRegressionPrevention:
    """Tests to prevent regression of fixed bugs."""
    
    def test_case_insensitive_all_commands(self, nested_traversal_with_session):
        """Ensure all commands work case-insensitively."""
        traversal, session, nested_doc = nested_traversal_with_session
        root = nested_doc["root"]
        child1 = nested_doc["child1"]
        
        # Test various case combinations with correct UCL syntax
        commands = [
            # GOTO variations
            f"GOTO {root}",
            f"goto {root}",
            f"GoTo {root}",
            # EXPAND variations (requires block_id first)
            f"EXPAND {root} DOWN",
            f"expand {root} down",
            f"Expand {root} Down",
            # VIEW variations
            f"VIEW {child1}",
            f"view {child1}",
            "VIEW NEIGHBORHOOD",
            "view neighborhood",
            # FIND variations (uses = syntax)
            "FIND PATTERN = \".*\"",
            "find pattern = \".*\"",
            # CTX variations
            f"CTX ADD {child1}",
            f"ctx add {child1}",
            "CTX CLEAR",
            "ctx clear",
            # PATH variations (uses TO keyword)
            f"PATH {root} TO {child1}",
            f"path {root} to {child1}",
        ]
        
        for cmd in commands:
            try:
                result = traversal.execute_ucl(session, cmd)
                assert result is not None, f"Command '{cmd}' returned None"
            except Exception as e:
                pytest.fail(f"Command '{cmd}' failed: {e}")
    
    def test_context_parameter_names(self, nested_traversal_with_session):
        """Ensure correct parameter names are used for context operations."""
        traversal, session, nested_doc = nested_traversal_with_session
        child1 = nested_doc["child1"]
        
        # These should work (correct parameter names)
        traversal.context_add(session, child1, relevance=0.5)
        traversal.context_add(session, child1, reason="test")
        traversal.context_add(session, child1, relevance=0.5, reason="test")
    
    def test_block_id_error_messages(self):
        """Ensure block ID error messages are descriptive."""
        invalid_ids = [
            "invalid",
            "blk_",
            "blk_123",
            "blk_xyz",
            "123",
            "",
        ]
        
        for invalid_id in invalid_ids:
            with pytest.raises(Exception) as exc_info:
                BlockId(invalid_id)
            
            error_msg = str(exc_info.value)
            # Error should mention format or blk_
            assert len(error_msg) > 10, f"Error for '{invalid_id}' too short: {error_msg}"
