"""
Regression tests for all issues identified in the Beta Test Final Report.

These tests ensure that all fixes remain working and prevent regressions.
"""

import pytest
from ucp import (
    Block,
    SemanticRole,
    ValidationResult,
    ValidationIssue,
    ValidationSeverity,
    Edge,
    EdgeType,
    SnapshotManager,
    create,
    parse,
    render,
)


class TestBlockHashability:
    """Regression tests for Block hashability (Agent 4 critical issue)."""

    def test_block_can_be_used_in_set(self):
        """Block objects can be added to sets."""
        block1 = Block.text("Content 1")
        block2 = Block.text("Content 2")

        block_set = {block1, block2}
        
        assert len(block_set) == 2
        assert block1 in block_set

    def test_block_can_be_dict_key(self):
        """Block objects can be used as dictionary keys."""
        block = Block.text("Key content")
        mapping = {block: "value"}

        assert mapping[block] == "value"

    def test_block_hash_based_on_id(self):
        """Blocks with same ID have same hash."""
        block1 = Block.text("Content")
        block2 = Block(id=block1.id, content="Different content")

        assert hash(block1) == hash(block2)

    def test_block_equality_based_on_id(self):
        """Block equality is determined by ID, not content."""
        block1 = Block.text("Original")
        block2 = Block(id=block1.id, content="Modified")

        assert block1 == block2

    def test_id_mapper_works_with_blocks(self):
        """IdMapper can use blocks in internal data structures."""
        from ucp import map_ids
        
        doc = create()
        doc.add_block(doc.root_id, "First")
        doc.add_block(doc.root_id, "Second")
        
        mapper = map_ids(doc)
        
        # Should be able to get mappings without error
        mappings = mapper.get_mappings()
        assert len(mappings) >= 2


class TestSemanticRoles:
    """Regression tests for SemanticRole completeness (Agent 1 issue)."""

    def test_equation_role_exists(self):
        """EQUATION semantic role exists."""
        assert hasattr(SemanticRole, 'EQUATION')
        assert SemanticRole.EQUATION.value == "equation"

    def test_metadata_role_exists(self):
        """METADATA semantic role exists."""
        assert hasattr(SemanticRole, 'METADATA')
        assert SemanticRole.METADATA.value == "metadata"

    def test_section_role_exists(self):
        """SECTION semantic role exists."""
        assert hasattr(SemanticRole, 'SECTION')
        assert SemanticRole.SECTION.value == "section"

    def test_note_role_exists(self):
        """NOTE semantic role exists."""
        assert hasattr(SemanticRole, 'NOTE')
        assert SemanticRole.NOTE.value == "note"

    def test_warning_role_exists(self):
        """WARNING semantic role exists."""
        assert hasattr(SemanticRole, 'WARNING')
        assert SemanticRole.WARNING.value == "warning"

    def test_tip_role_exists(self):
        """TIP semantic role exists."""
        assert hasattr(SemanticRole, 'TIP')
        assert SemanticRole.TIP.value == "tip"

    def test_can_create_block_with_new_roles(self):
        """Blocks can be created with all new semantic roles."""
        roles = [
            SemanticRole.EQUATION,
            SemanticRole.METADATA,
            SemanticRole.SECTION,
            SemanticRole.NOTE,
            SemanticRole.WARNING,
            SemanticRole.TIP,
        ]
        
        for role in roles:
            block = Block.text("Content", role=role)
            assert block.role == role


class TestValidationResultAPI:
    """Regression tests for ValidationResult API consistency (Agent 1, 5 issue)."""

    def test_errors_is_method(self):
        """errors() is a callable method."""
        result = ValidationResult.success()
        assert callable(result.errors)
        assert result.errors() == []

    def test_warnings_is_method(self):
        """warnings() is a callable method."""
        result = ValidationResult.success()
        assert callable(result.warnings)
        assert result.warnings() == []

    def test_infos_is_method(self):
        """infos() method exists and is callable."""
        result = ValidationResult.success()
        assert callable(result.infos)
        assert result.infos() == []

    def test_infos_returns_info_issues(self):
        """infos() returns only INFO severity issues."""
        issues = [
            ValidationIssue.error("E001", "Error message"),
            ValidationIssue.warning("W001", "Warning message"),
            ValidationIssue.info("I001", "Info message"),
        ]
        result = ValidationResult.failure(issues)

        infos = result.infos()
        assert len(infos) == 1
        assert infos[0].severity == ValidationSeverity.INFO

    def test_validation_issue_info_factory(self):
        """ValidationIssue.info() factory method exists."""
        issue = ValidationIssue.info("I001", "Informational note", block_id="blk_test")
        
        assert issue.severity == ValidationSeverity.INFO
        assert issue.code == "I001"
        assert issue.message == "Informational note"
        assert issue.block_id == "blk_test"


class TestEdgeAPI:
    """Regression tests for Edge API (Agent 5 issue)."""

    def test_edge_new_factory(self):
        """Edge.new() creates edge with type and target."""
        edge = Edge.new(EdgeType.REFERENCES, "blk_target")

        assert edge.edge_type == EdgeType.REFERENCES
        assert edge.target == "blk_target"

    def test_edge_with_confidence(self):
        """Edge supports with_confidence() fluent method."""
        edge = Edge.new(EdgeType.SUPPORTS, "blk_target").with_confidence(0.95)

        assert edge.metadata.confidence == 0.95

    def test_edge_with_description(self):
        """Edge supports with_description() fluent method."""
        edge = Edge.new(EdgeType.REFERENCES, "blk_target").with_description("See also")

        assert edge.metadata.description == "See also"

    def test_edge_fluent_chaining(self):
        """Edge fluent methods can be chained."""
        edge = (Edge.new(EdgeType.ELABORATES, "blk_target")
            .with_confidence(0.9)
            .with_description("Details"))

        assert edge.metadata.confidence == 0.9
        assert edge.metadata.description == "Details"


class TestSnapshotManager:
    """Regression tests for SnapshotManager (Agent 5 issue)."""

    def test_max_snapshots_parameter(self):
        """SnapshotManager accepts max_snapshots in constructor."""
        manager = SnapshotManager(max_snapshots=5)
        
        # Should not raise
        assert manager is not None

    def test_automatic_eviction(self):
        """SnapshotManager automatically evicts oldest when at capacity."""
        manager = SnapshotManager(max_snapshots=3)
        doc = create()
        
        # Create 4 snapshots (exceeds max of 3)
        manager.create("snap1", doc)
        manager.create("snap2", doc)
        manager.create("snap3", doc)
        manager.create("snap4", doc)  # Should trigger eviction
        
        # Should only have 3 snapshots
        assert manager.count() == 3
        
        # Oldest (snap1) should have been evicted
        assert not manager.exists("snap1")
        assert manager.exists("snap4")


class TestMarkdownRoundTrip:
    """Regression tests for markdown conversion (Agent 3 issues)."""

    def test_heading_hierarchy_preserved(self):
        """Heading hierarchy is preserved through round-trip."""
        original = "# H1\n\n## H2\n\n### H3\n"
        doc = parse(original)
        rendered = render(doc)

        assert "# " in rendered  # H1
        assert "## " in rendered  # H2
        assert "### " in rendered  # H3

    def test_code_block_language_preserved(self):
        """Code block language hints are preserved."""
        original = "# Code\n\n```python\nprint('hello')\n```\n"
        doc = parse(original)
        rendered = render(doc)

        assert "```python" in rendered
        assert "print" in rendered

    def test_blockquote_preserved(self):
        """Blockquotes are preserved through round-trip."""
        original = "# Quote\n\n> This is quoted\n> Multiple lines\n"
        doc = parse(original)
        rendered = render(doc)

        assert "> " in rendered
        assert "quoted" in rendered


class TestUclBuilder:
    """Regression tests for UclBuilder (Agent 2, 4 issues)."""

    def test_edit_method_signature(self):
        """UclBuilder.edit() accepts block_id, content, and optional path."""
        from ucp import ucl
        
        builder = ucl()
        # Should work with just block_id and content
        builder.edit("blk_1", "new content")
        
        # Should work with explicit path
        builder.edit("blk_2", "other content", path="text")
        
        result = builder.build()
        assert "EDIT blk_1" in result
        assert "EDIT blk_2" in result

    def test_edit_does_not_accept_label(self):
        """UclBuilder.edit() does not accept label parameter."""
        from ucp import ucl
        import inspect
        
        sig = inspect.signature(ucl().edit)
        params = list(sig.parameters.keys())
        
        # Should only have block_id, content, path
        assert "label" not in params

    def test_append_accepts_label(self):
        """UclBuilder.append() accepts label via properties kwargs."""
        from ucp import ucl
        
        builder = ucl()
        builder.append("blk_1", "content", label="my-label")
        
        result = builder.build()
        assert 'label="my-label"' in result
