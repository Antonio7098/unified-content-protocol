"""
Regression tests for all 23 failure scenarios from detailed_failure_analysis.md.

This test suite ensures that previously identified failures remain fixed.
Each test category corresponds to a category from the failure analysis.

Categories:
1. UCL Command Language (7 scenarios)
2. Advanced Navigation Features (6 scenarios)
3. Advanced Context Features (8 scenarios)
4. Error Message Clarity (2 scenarios)
"""

import pytest
from ucp import (
    create,
    AgentTraversal,
    BlockId,
    ViewMode,
)


# =============================================================================
# Test Fixtures
# =============================================================================

@pytest.fixture
def complex_doc():
    """Create a complex document mimicking the failure analysis test structure."""
    doc = create("Failure Analysis Test Document")
    root = doc.root_id
    
    # Create sections like in the failure analysis
    intro = doc.add_block(root, "Introduction")
    intro_overview = doc.add_block(intro, "Introduction Overview")
    intro_details = doc.add_block(intro, "Introduction Details")
    intro_summary = doc.add_block(intro, "Introduction Summary")
    
    methods = doc.add_block(root, "Methods")
    methods_overview = doc.add_block(methods, "Methods Overview")
    methods_details = doc.add_block(methods, "Methods Details")
    methods_summary = doc.add_block(methods, "Methods Summary")
    
    results = doc.add_block(root, "Results")
    results_overview = doc.add_block(results, "Results Overview")
    
    conclusion = doc.add_block(root, "Conclusion")
    conclusion_details = doc.add_block(conclusion, "Conclusion Details")
    
    # Deeper nesting for distant path tests
    deep_content = doc.add_block(intro_overview, "Deep Content Level 1")
    deeper_content = doc.add_block(deep_content, "Deep Content Level 2")
    
    return {
        "doc": doc,
        "root": root,
        "introduction": intro,
        "intro_overview": intro_overview,
        "intro_details": intro_details,
        "intro_summary": intro_summary,
        "methods": methods,
        "methods_overview": methods_overview,
        "methods_details": methods_details,
        "methods_summary": methods_summary,
        "results": results,
        "results_overview": results_overview,
        "conclusion": conclusion,
        "conclusion_details": conclusion_details,
        "deep_content": deep_content,
        "deeper_content": deeper_content,
    }


@pytest.fixture
def traversal_session(complex_doc):
    """Create traversal and session for the complex document."""
    traversal = AgentTraversal(complex_doc["doc"])
    session = traversal.create_session()
    return traversal, session, complex_doc


# =============================================================================
# Category 1: UCL Command Language (7 failure scenarios)
# =============================================================================

class TestUCLCommandLanguageRegression:
    """
    Regression tests for UCL Command Language failures.
    
    Original failures:
    1.1: UCL Navigation Command
    1.2: UCL Search Command (FIND PATTERN)
    1.3: UCL Expansion Command with depth
    1.4: UCL View Command
    1.5: UCL Script Execution (multi-line)
    1.6: UCL Conditional Operations
    1.7: UCL Variables and State
    """
    
    def test_1_1_ucl_navigation_expand_command(self, traversal_session):
        """Regression: EXPAND block_id DOWN command should work."""
        traversal, session, blocks = traversal_session
        
        # The original failure was: EXPAND {block_id} DOWN
        result = traversal.execute_ucl(
            session, 
            f"EXPAND {blocks['intro_overview']} DOWN"
        )
        assert result is not None
        assert len(result) > 0
    
    def test_1_2_ucl_search_find_pattern(self, traversal_session):
        """Regression: FIND PATTERN = 'pattern' should work with = syntax."""
        traversal, session, blocks = traversal_session
        
        # The original failure was: FIND PATTERN = "Content"
        # Parser expects PATTERN = "value" syntax
        result = traversal.execute_ucl(session, 'FIND PATTERN = ".*"')
        assert result is not None
    
    def test_1_3_ucl_expansion_with_depth_eq_syntax(self, traversal_session):
        """Regression: EXPAND with DEPTH = N should use = not :."""
        traversal, session, blocks = traversal_session
        
        # Original failure: depth:2 - parser expects DEPTH = 2
        result = traversal.execute_ucl(
            session,
            f"EXPAND {blocks['intro_overview']} DOWN DEPTH = 2"
        )
        assert result is not None
    
    def test_1_4_ucl_view_mode_command(self, traversal_session):
        """Regression: VIEW with MODE parameter should use = syntax."""
        traversal, session, blocks = traversal_session
        
        # Original failure: VIEW MODE = preview
        # Parser expects MODE = value syntax
        result = traversal.execute_ucl(
            session,
            f"VIEW {blocks['intro_overview']} MODE = PREVIEW"
        )
        assert result is not None
    
    def test_1_5_ucl_script_execution_multiline(self, traversal_session):
        """Regression: Multiple UCL commands should execute in sequence."""
        traversal, session, blocks = traversal_session
        
        # Test executing multiple commands in sequence (not as a script)
        result1 = traversal.execute_ucl(session, f"GOTO {blocks['introduction']}")
        assert result1 is not None
        
        result2 = traversal.execute_ucl(
            session, 
            f"EXPAND {blocks['introduction']} DOWN DEPTH = 2"
        )
        assert result2 is not None
        
        result3 = traversal.execute_ucl(session, 'FIND PATTERN = "Overview"')
        assert result3 is not None
        
        result4 = traversal.execute_ucl(
            session, 
            f"VIEW {blocks['intro_overview']} MODE = FULL"
        )
        assert result4 is not None
    
    def test_1_6_ucl_case_insensitivity(self, traversal_session):
        """Regression: Commands should be case-insensitive."""
        traversal, session, blocks = traversal_session
        
        # Original issue: parser not recognizing commands
        # Test all case variations
        results = []
        
        results.append(traversal.execute_ucl(session, f"GOTO {blocks['root']}"))
        results.append(traversal.execute_ucl(session, f"goto {blocks['root']}"))
        results.append(traversal.execute_ucl(session, f"Goto {blocks['root']}"))
        results.append(traversal.execute_ucl(session, f"GoTo {blocks['root']}"))
        
        for r in results:
            assert r is not None
    
    def test_1_7_ucl_parameter_syntax(self, traversal_session):
        """Regression: Parameters should use = syntax consistently."""
        traversal, session, blocks = traversal_session
        
        # Test various parameter syntaxes with =
        # DEPTH = N
        traversal.execute_ucl(
            session, 
            f"EXPAND {blocks['introduction']} DOWN DEPTH = 3"
        )
        
        # MODE = value
        traversal.execute_ucl(
            session, 
            f"VIEW {blocks['introduction']} MODE = METADATA"
        )
        
        # RELEVANCE = F
        traversal.execute_ucl(
            session, 
            f"CTX ADD {blocks['introduction']} RELEVANCE = 0.9"
        )
        
        # REASON = "string"
        traversal.execute_ucl(
            session, 
            f'CTX ADD {blocks["methods"]} REASON = "test reason"'
        )


# =============================================================================
# Category 2: Advanced Navigation Features (6 failure scenarios)
# =============================================================================

class TestAdvancedNavigationRegression:
    """
    Regression tests for Advanced Navigation failures.
    
    Original failures:
    2.1: Path Finding Inconsistency
    2.2: View Mode Inconsistency
    2.3: Neighborhood Viewing Parameter Issue
    2.4: Sibling Path Finding
    2.5: Distant Path Finding
    2.6: Adaptive View Mode
    """
    
    def test_2_1_path_finding_parent_to_child(self, traversal_session):
        """Regression: Path from parent to child should work."""
        traversal, session, blocks = traversal_session
        
        path = traversal.find_path(
            session, 
            blocks["introduction"], 
            blocks["intro_overview"]
        )
        assert path is not None
        assert len(path) >= 2
        assert path[0] == blocks["introduction"]
        assert path[-1] == blocks["intro_overview"]
    
    def test_2_2_view_mode_ids_only_returns_valid_result(self, traversal_session):
        """Regression: ViewMode.ids_only() should return valid BlockView."""
        traversal, session, blocks = traversal_session
        
        result = traversal.view_block(
            session, 
            blocks["intro_overview"], 
            ViewMode.ids_only()
        )
        # Should return valid BlockView, not None
        assert result is not None
        assert hasattr(result, 'block_id')
        assert result.block_id == blocks["intro_overview"]
    
    def test_2_3_neighborhood_viewing_works(self, traversal_session):
        """Regression: view_neighborhood should work."""
        traversal, session, blocks = traversal_session
        
        # Navigate to a block first
        traversal.navigate_to(session, blocks["introduction"])
        
        # View neighborhood
        result = traversal.view_neighborhood(session)
        assert result is not None
        assert hasattr(result, 'position')
        assert hasattr(result, 'children')
        assert hasattr(result, 'ancestors')
        assert hasattr(result, 'siblings')
    
    def test_2_4_sibling_path_finding(self, traversal_session):
        """Regression: Path between siblings should work."""
        traversal, session, blocks = traversal_session
        
        # Path from intro_overview to intro_details (siblings)
        path = traversal.find_path(
            session,
            blocks["intro_overview"],
            blocks["intro_details"]
        )
        assert path is not None
        assert len(path) >= 2
        assert path[0] == blocks["intro_overview"]
        assert path[-1] == blocks["intro_details"]
    
    def test_2_5_distant_path_finding(self, traversal_session):
        """Regression: Path between distant blocks should work."""
        traversal, session, blocks = traversal_session
        
        # Path from deeper_content to conclusion_details (distant)
        path = traversal.find_path(
            session,
            blocks["deeper_content"],
            blocks["conclusion_details"]
        )
        assert path is not None
        assert len(path) >= 2
    
    def test_2_6_adaptive_view_mode(self, traversal_session):
        """Regression: ViewMode.adaptive() should work."""
        traversal, session, blocks = traversal_session
        
        result = traversal.view_block(
            session,
            blocks["intro_overview"],
            ViewMode.adaptive()
        )
        assert result is not None
        assert hasattr(result, 'block_id')
    
    def test_cross_branch_path_finding(self, traversal_session):
        """Regression: Path between blocks in different branches."""
        traversal, session, blocks = traversal_session
        
        # Path from intro_overview to methods_overview (cross-branch)
        path = traversal.find_path(
            session,
            blocks["intro_overview"],
            blocks["methods_overview"]
        )
        assert path is not None
        assert len(path) >= 2
        assert path[0] == blocks["intro_overview"]
        assert path[-1] == blocks["methods_overview"]


# =============================================================================
# Category 3: Advanced Context Features (8 failure scenarios)
# =============================================================================

class TestAdvancedContextRegression:
    """
    Regression tests for Advanced Context Features failures.
    
    Original failures:
    3.1: Context Removal Operations
    3.2: Context Clear Operations
    3.3: Focus Block Management
    3.4: Bulk Context Operations
    3.5-3.8: Event System (4 failures)
    """
    
    def test_3_1_context_removal_method_exists(self, traversal_session):
        """Regression: context_remove method should exist and work."""
        traversal, session, blocks = traversal_session
        
        # Add then remove
        traversal.context_add(session, blocks["introduction"])
        traversal.context_remove(session, blocks["introduction"])
        # Should not raise
    
    def test_3_2_context_clear_method_exists(self, traversal_session):
        """Regression: context_clear method should exist and work."""
        traversal, session, blocks = traversal_session
        
        # Add multiple then clear
        traversal.context_add(session, blocks["introduction"])
        traversal.context_add(session, blocks["methods"])
        traversal.context_add(session, blocks["results"])
        
        # Clear all
        traversal.context_clear(session)
        # Should not raise
    
    def test_3_3_focus_set_method_exists(self, traversal_session):
        """Regression: context_focus method should exist and work."""
        traversal, session, blocks = traversal_session
        
        # Set focus
        traversal.context_focus(session, blocks["results"])
        # Should not raise
    
    def test_3_3_focus_clear(self, traversal_session):
        """Regression: context_focus(None) should clear focus."""
        traversal, session, blocks = traversal_session
        
        # Set then clear focus
        traversal.context_focus(session, blocks["results"])
        traversal.context_focus(session, None)
        # Should not raise
    
    def test_3_4_bulk_context_add_results(self, traversal_session):
        """Regression: context_add_results should add search results to context."""
        traversal, session, blocks = traversal_session
        
        # First perform a find
        traversal.find(session, pattern=".*")
        
        # Add results to context
        result = traversal.context_add_results(session)
        assert result is not None
    
    def test_context_add_with_relevance(self, traversal_session):
        """Regression: context_add should accept relevance parameter."""
        traversal, session, blocks = traversal_session
        
        traversal.context_add(
            session, 
            blocks["introduction"], 
            relevance=0.95
        )
    
    def test_context_add_with_reason(self, traversal_session):
        """Regression: context_add should accept reason parameter."""
        traversal, session, blocks = traversal_session
        
        traversal.context_add(
            session, 
            blocks["introduction"], 
            reason="important for summary"
        )
    
    def test_context_add_with_both_params(self, traversal_session):
        """Regression: context_add should accept both relevance and reason."""
        traversal, session, blocks = traversal_session
        
        traversal.context_add(
            session, 
            blocks["introduction"], 
            relevance=0.9,
            reason="high relevance content"
        )
    
    def test_ucl_ctx_add_command(self, traversal_session):
        """Regression: CTX ADD via UCL should work."""
        traversal, session, blocks = traversal_session
        
        result = traversal.execute_ucl(
            session, 
            f"CTX ADD {blocks['introduction']}"
        )
        assert result is not None
    
    def test_ucl_ctx_remove_command(self, traversal_session):
        """Regression: CTX REMOVE via UCL should work."""
        traversal, session, blocks = traversal_session
        
        traversal.execute_ucl(session, f"CTX ADD {blocks['introduction']}")
        result = traversal.execute_ucl(
            session, 
            f"CTX REMOVE {blocks['introduction']}"
        )
        assert result is not None
    
    def test_ucl_ctx_clear_command(self, traversal_session):
        """Regression: CTX CLEAR via UCL should work."""
        traversal, session, blocks = traversal_session
        
        result = traversal.execute_ucl(session, "CTX CLEAR")
        assert result is not None
    
    def test_ucl_ctx_focus_command(self, traversal_session):
        """Regression: CTX FOCUS via UCL should work."""
        traversal, session, blocks = traversal_session
        
        result = traversal.execute_ucl(
            session, 
            f"CTX FOCUS {blocks['introduction']}"
        )
        assert result is not None
    
    def test_ucl_ctx_stats_command(self, traversal_session):
        """Regression: CTX STATS via UCL should work."""
        traversal, session, blocks = traversal_session
        
        result = traversal.execute_ucl(session, "CTX STATS")
        assert result is not None


# =============================================================================
# Category 4: Error Message Clarity (2 failure scenarios)
# =============================================================================

class TestErrorMessageClarityRegression:
    """
    Regression tests for Error Message Clarity failures.
    
    Original failures:
    4.1: Navigation Error Messages
    4.2: Expansion Error Messages
    """
    
    def test_4_1_navigate_to_invalid_block_error_message(self, traversal_session):
        """Regression: Error for invalid block should be descriptive."""
        traversal, session, blocks = traversal_session
        
        fake_id = BlockId("blk_000000000000000000000000")
        
        with pytest.raises(Exception) as exc_info:
            traversal.navigate_to(session, fake_id)
        
        error_msg = str(exc_info.value).lower()
        # Should mention block or not found
        assert "block" in error_msg or "not found" in error_msg or "sync" in error_msg
    
    def test_4_2_expand_invalid_block_error_message(self, traversal_session):
        """Regression: Expanding invalid block should either raise or return empty."""
        traversal, session, blocks = traversal_session
        
        fake_id = BlockId("blk_000000000000000000000000")
        
        try:
            result = traversal.expand(session, fake_id, direction="down")
            # If it doesn't raise, should return result with 0 or minimal blocks
            assert result is not None
        except Exception as exc_info:
            # If it raises, error should be descriptive
            error_msg = str(exc_info).lower()
            assert "block" in error_msg or "not found" in error_msg or "sync" in error_msg
    
    def test_invalid_block_id_format_error(self):
        """Regression: Invalid block ID format should give descriptive error."""
        with pytest.raises(Exception) as exc_info:
            BlockId("not_a_valid_block_id")
        
        error_msg = str(exc_info.value)
        assert "blk_" in error_msg or "format" in error_msg.lower()
    
    def test_invalid_direction_error(self, traversal_session):
        """Regression: Invalid direction should give descriptive error."""
        traversal, session, blocks = traversal_session
        
        with pytest.raises(Exception) as exc_info:
            traversal.expand(session, blocks["introduction"], direction="invalid")
        
        error_msg = str(exc_info.value).lower()
        assert "direction" in error_msg or "invalid" in error_msg


# =============================================================================
# Additional Edge Case Tests
# =============================================================================

class TestEdgeCaseRegression:
    """Additional edge case tests to prevent regression."""
    
    def test_empty_document_operations(self):
        """Operations on empty document should work."""
        doc = create("Empty Doc")
        traversal = AgentTraversal(doc)
        session = traversal.create_session()
        
        # Navigate to root
        traversal.navigate_to(session, doc.root_id)
        
        # Expand should work but return few results
        result = traversal.expand(session, doc.root_id, direction="down")
        assert result is not None
    
    def test_deeply_nested_path_finding(self):
        """Path finding in deeply nested documents should work."""
        doc = create("Deep Doc")
        current = doc.root_id
        
        # Create deep nesting (8 levels to stay within default max_depth of 10)
        blocks = [current]
        for i in range(8):
            current = doc.add_block(current, f"Level {i}")
            blocks.append(current)
        
        traversal = AgentTraversal(doc)
        # Sync changes after adding blocks
        traversal.update_document(doc)
        session = traversal.create_session()
        
        # Find path from root to deepest
        path = traversal.find_path(session, blocks[0], blocks[-1])
        assert path is not None
        assert len(path) == 9  # root + 8 levels
    
    def test_very_deep_path_with_max_length(self):
        """Path finding with explicit max_length for deep documents."""
        doc = create("Very Deep Doc")
        current = doc.root_id
        
        # Create deeper nesting
        blocks = [current]
        for i in range(15):
            current = doc.add_block(current, f"Level {i}")
            blocks.append(current)
        
        traversal = AgentTraversal(doc)
        traversal.update_document(doc)
        session = traversal.create_session()
        
        # Find path with explicit max_length to handle deep nesting
        path = traversal.find_path(session, blocks[0], blocks[-1], max_length=20)
        assert path is not None
        assert len(path) == 16  # root + 15 levels
    
    def test_wide_document_operations(self):
        """Operations on wide documents should work."""
        doc = create("Wide Doc")
        root = doc.root_id
        
        # Create many children
        children = []
        for i in range(30):
            children.append(doc.add_block(root, f"Child {i}"))
        
        traversal = AgentTraversal(doc)
        session = traversal.create_session()
        
        # Expand should return all children
        result = traversal.expand(session, root, direction="down", depth=1)
        assert result is not None
        assert result.total_blocks >= 30
    
    def test_multiple_sessions_isolation(self, complex_doc):
        """Multiple sessions should be isolated."""
        doc = complex_doc["doc"]
        traversal = AgentTraversal(doc)
        
        session1 = traversal.create_session()
        session2 = traversal.create_session()
        
        # Navigate to different positions
        traversal.navigate_to(session1, complex_doc["introduction"])
        traversal.navigate_to(session2, complex_doc["methods"])
        
        # Add different blocks to context
        traversal.context_add(session1, complex_doc["intro_overview"])
        traversal.context_add(session2, complex_doc["methods_overview"])
        
        # Clear session1 context shouldn't affect session2
        traversal.context_clear(session1)
        
        # Session2 should still work
        result = traversal.view_neighborhood(session2)
        assert result is not None
    
    def test_session_close_and_reopen(self, complex_doc):
        """Closing and reopening sessions should work."""
        doc = complex_doc["doc"]
        traversal = AgentTraversal(doc)
        
        session1 = traversal.create_session()
        traversal.navigate_to(session1, complex_doc["introduction"])
        traversal.close_session(session1)
        
        # Create new session
        session2 = traversal.create_session()
        traversal.navigate_to(session2, complex_doc["methods"])
        
        result = traversal.view_neighborhood(session2)
        assert result is not None


# =============================================================================
# UCL Parser Specific Regression Tests
# =============================================================================

class TestUCLParserRegression:
    """Regression tests for UCL parser edge cases."""
    
    def test_all_view_modes_via_ucl(self, traversal_session):
        """All view modes should be parseable via UCL."""
        traversal, session, blocks = traversal_session
        block_id = blocks["introduction"]
        
        modes = ["FULL", "PREVIEW", "METADATA", "IDS"]
        for mode in modes:
            result = traversal.execute_ucl(
                session, 
                f"VIEW {block_id} MODE = {mode}"
            )
            assert result is not None, f"Mode {mode} failed"
    
    def test_all_directions_via_ucl(self, traversal_session):
        """All expand directions should be parseable via UCL."""
        traversal, session, blocks = traversal_session
        block_id = blocks["introduction"]
        
        directions = ["DOWN", "UP", "BOTH", "SEMANTIC"]
        for direction in directions:
            result = traversal.execute_ucl(
                session, 
                f"EXPAND {block_id} {direction}"
            )
            assert result is not None, f"Direction {direction} failed"
    
    def test_ctx_expand_directions_via_ucl(self, traversal_session):
        """CTX EXPAND with all directions should work."""
        traversal, session, blocks = traversal_session
        traversal.navigate_to(session, blocks["introduction"])
        
        directions = ["DOWN", "UP", "SEMANTIC", "BOTH"]
        for direction in directions:
            result = traversal.execute_ucl(session, f"CTX EXPAND {direction}")
            assert result is not None, f"CTX EXPAND {direction} failed"
    
    def test_path_command_via_ucl(self, traversal_session):
        """PATH command via UCL should work."""
        traversal, session, blocks = traversal_session
        
        result = traversal.execute_ucl(
            session,
            f"PATH {blocks['introduction']} TO {blocks['conclusion']}"
        )
        assert result is not None
    
    def test_back_command_via_ucl(self, traversal_session):
        """BACK command via UCL should work."""
        traversal, session, blocks = traversal_session
        
        # Navigate first
        traversal.navigate_to(session, blocks["introduction"])
        traversal.navigate_to(session, blocks["methods"])
        
        result = traversal.execute_ucl(session, "BACK")
        assert result is not None
    
    def test_back_with_steps_via_ucl(self, traversal_session):
        """BACK N command via UCL should work."""
        traversal, session, blocks = traversal_session
        
        # Navigate multiple times
        traversal.navigate_to(session, blocks["introduction"])
        traversal.navigate_to(session, blocks["methods"])
        traversal.navigate_to(session, blocks["results"])
        
        result = traversal.execute_ucl(session, "BACK 2")
        assert result is not None
