"""
Conformance tests for Edge API.

These tests verify Edge creation, manipulation, and EdgeType functionality.
"""

from ucp import (
    Edge,
    EdgeType,
    EdgeMetadata,
    create,
)


class TestEdgeCreation:
    """Test Edge creation and factory methods."""

    def test_edge_new_factory(self):
        """Edge.new() creates edge with type and target."""
        edge = Edge.new(EdgeType.REFERENCES, "blk_target")

        assert edge.edge_type == EdgeType.REFERENCES
        assert edge.target == "blk_target"

    def test_edge_metadata_default(self):
        """Edge has default empty metadata."""
        edge = Edge.new(EdgeType.REFERENCES, "blk_target")

        assert edge.metadata is not None
        assert edge.metadata.confidence is None
        assert edge.metadata.description is None

    def test_edge_has_created_at(self):
        """Edge has created_at timestamp."""
        edge = Edge.new(EdgeType.REFERENCES, "blk_target")

        assert edge.created_at is not None


class TestEdgeFluent:
    """Test Edge fluent builder methods."""

    def test_with_confidence(self):
        """Edge can have confidence metadata."""
        edge = Edge.new(EdgeType.SUPPORTS, "blk_target").with_confidence(0.95)

        assert edge.metadata.confidence == 0.95

    def test_with_confidence_returns_self(self):
        """with_confidence returns self for chaining."""
        edge = Edge.new(EdgeType.SUPPORTS, "blk_target")
        result = edge.with_confidence(0.8)

        assert result is edge

    def test_with_description(self):
        """Edge can have description metadata."""
        edge = Edge.new(EdgeType.REFERENCES, "blk_target").with_description("See also")

        assert edge.metadata.description == "See also"

    def test_with_description_returns_self(self):
        """with_description returns self for chaining."""
        edge = Edge.new(EdgeType.REFERENCES, "blk_target")
        result = edge.with_description("Note")

        assert result is edge

    def test_confidence_clamped_max(self):
        """Confidence above 1.0 is clamped to 1.0."""
        edge = Edge.new(EdgeType.SUPPORTS, "blk_1").with_confidence(1.5)

        assert edge.metadata.confidence == 1.0

    def test_confidence_clamped_min(self):
        """Confidence below 0.0 is clamped to 0.0."""
        edge = Edge.new(EdgeType.SUPPORTS, "blk_2").with_confidence(-0.5)

        assert edge.metadata.confidence == 0.0

    def test_chained_fluent_methods(self):
        """Fluent methods can be chained."""
        edge = (Edge.new(EdgeType.ELABORATES, "blk_target")
            .with_confidence(0.9)
            .with_description("Details"))

        assert edge.metadata.confidence == 0.9
        assert edge.metadata.description == "Details"


class TestEdgeTypeFromStr:
    """Test EdgeType.from_str parsing."""

    def test_from_str_valid_lowercase(self):
        """EdgeType.from_str parses lowercase types."""
        assert EdgeType.from_str("references") == EdgeType.REFERENCES
        assert EdgeType.from_str("supports") == EdgeType.SUPPORTS
        assert EdgeType.from_str("contradicts") == EdgeType.CONTRADICTS

    def test_from_str_case_insensitive(self):
        """EdgeType.from_str is case insensitive."""
        assert EdgeType.from_str("REFERENCES") == EdgeType.REFERENCES
        assert EdgeType.from_str("References") == EdgeType.REFERENCES
        assert EdgeType.from_str("rEfErEnCeS") == EdgeType.REFERENCES

    def test_from_str_invalid_returns_none(self):
        """EdgeType.from_str returns None for invalid types."""
        assert EdgeType.from_str("invalid_type") is None
        assert EdgeType.from_str("") is None
        assert EdgeType.from_str("not_an_edge") is None


class TestEdgeTypeInverse:
    """Test EdgeType inverse relationships."""

    def test_references_inverse(self):
        """REFERENCES inverse is CITED_BY."""
        assert EdgeType.REFERENCES.inverse() == EdgeType.CITED_BY

    def test_cited_by_inverse(self):
        """CITED_BY inverse is REFERENCES."""
        assert EdgeType.CITED_BY.inverse() == EdgeType.REFERENCES

    def test_parent_of_inverse(self):
        """PARENT_OF inverse is CHILD_OF."""
        assert EdgeType.PARENT_OF.inverse() == EdgeType.CHILD_OF

    def test_child_of_inverse(self):
        """CHILD_OF inverse is PARENT_OF."""
        assert EdgeType.CHILD_OF.inverse() == EdgeType.PARENT_OF

    def test_previous_sibling_inverse(self):
        """PREVIOUS_SIBLING inverse is NEXT_SIBLING."""
        assert EdgeType.PREVIOUS_SIBLING.inverse() == EdgeType.NEXT_SIBLING

    def test_next_sibling_inverse(self):
        """NEXT_SIBLING inverse is PREVIOUS_SIBLING."""
        assert EdgeType.NEXT_SIBLING.inverse() == EdgeType.PREVIOUS_SIBLING

    def test_contradicts_is_self_inverse(self):
        """CONTRADICTS is its own inverse (symmetric)."""
        assert EdgeType.CONTRADICTS.inverse() == EdgeType.CONTRADICTS

    def test_sibling_of_is_self_inverse(self):
        """SIBLING_OF is its own inverse (symmetric)."""
        assert EdgeType.SIBLING_OF.inverse() == EdgeType.SIBLING_OF


class TestEdgeTypeSymmetric:
    """Test EdgeType symmetric detection."""

    def test_contradicts_is_symmetric(self):
        """CONTRADICTS is symmetric."""
        assert EdgeType.CONTRADICTS.is_symmetric() is True

    def test_sibling_of_is_symmetric(self):
        """SIBLING_OF is symmetric."""
        assert EdgeType.SIBLING_OF.is_symmetric() is True

    def test_references_not_symmetric(self):
        """REFERENCES is not symmetric."""
        assert EdgeType.REFERENCES.is_symmetric() is False

    def test_parent_of_not_symmetric(self):
        """PARENT_OF is not symmetric."""
        assert EdgeType.PARENT_OF.is_symmetric() is False


class TestEdgeTypeStructural:
    """Test EdgeType structural detection."""

    def test_parent_of_is_structural(self):
        """PARENT_OF is structural."""
        assert EdgeType.PARENT_OF.is_structural() is True

    def test_child_of_is_structural(self):
        """CHILD_OF is structural."""
        assert EdgeType.CHILD_OF.is_structural() is True

    def test_sibling_of_is_structural(self):
        """SIBLING_OF is structural."""
        assert EdgeType.SIBLING_OF.is_structural() is True

    def test_previous_sibling_is_structural(self):
        """PREVIOUS_SIBLING is structural."""
        assert EdgeType.PREVIOUS_SIBLING.is_structural() is True

    def test_next_sibling_is_structural(self):
        """NEXT_SIBLING is structural."""
        assert EdgeType.NEXT_SIBLING.is_structural() is True

    def test_references_not_structural(self):
        """REFERENCES is not structural."""
        assert EdgeType.REFERENCES.is_structural() is False

    def test_supports_not_structural(self):
        """SUPPORTS is not structural."""
        assert EdgeType.SUPPORTS.is_structural() is False


class TestEdgeTypeValues:
    """Test EdgeType enum values."""

    def test_derivation_types(self):
        """Derivation edge types exist."""
        assert EdgeType.DERIVED_FROM.value == "derived_from"
        assert EdgeType.SUPERSEDES.value == "supersedes"
        assert EdgeType.TRANSFORMED_FROM.value == "transformed_from"

    def test_reference_types(self):
        """Reference edge types exist."""
        assert EdgeType.REFERENCES.value == "references"
        assert EdgeType.CITED_BY.value == "cited_by"
        assert EdgeType.LINKS_TO.value == "links_to"

    def test_semantic_types(self):
        """Semantic edge types exist."""
        assert EdgeType.SUPPORTS.value == "supports"
        assert EdgeType.CONTRADICTS.value == "contradicts"
        assert EdgeType.ELABORATES.value == "elaborates"
        assert EdgeType.SUMMARIZES.value == "summarizes"

    def test_structural_types(self):
        """Structural edge types exist."""
        assert EdgeType.PARENT_OF.value == "parent_of"
        assert EdgeType.CHILD_OF.value == "child_of"
        assert EdgeType.SIBLING_OF.value == "sibling_of"

    def test_version_types(self):
        """Version edge types exist."""
        assert EdgeType.VERSION_OF.value == "version_of"
        assert EdgeType.ALTERNATIVE_OF.value == "alternative_of"
        assert EdgeType.TRANSLATION_OF.value == "translation_of"


class TestEdgeMetadata:
    """Test EdgeMetadata functionality."""

    def test_default_metadata_empty(self):
        """Default EdgeMetadata is empty."""
        meta = EdgeMetadata()

        assert meta.is_empty() is True

    def test_metadata_with_confidence_not_empty(self):
        """Metadata with confidence is not empty."""
        meta = EdgeMetadata(confidence=0.9)

        assert meta.is_empty() is False

    def test_metadata_with_description_not_empty(self):
        """Metadata with description is not empty."""
        meta = EdgeMetadata(description="A note")

        assert meta.is_empty() is False

    def test_metadata_with_custom_not_empty(self):
        """Metadata with custom data is not empty."""
        meta = EdgeMetadata(custom={"key": "value"})

        assert meta.is_empty() is False


class TestDocumentEdges:
    """Test edge operations on Document."""

    def test_add_edge_between_blocks(self):
        """Can add edge between blocks."""
        doc = create()
        source = doc.add_block(doc.root_id, "Source")
        target = doc.add_block(doc.root_id, "Target")

        doc.add_edge(source, EdgeType.REFERENCES, target)

        edges = doc.get_edges(source)
        assert len(edges) == 1
        assert edges[0].target == target

    def test_remove_edge_between_blocks(self):
        """Can remove edge between blocks."""
        doc = create()
        source = doc.add_block(doc.root_id, "Source")
        target = doc.add_block(doc.root_id, "Target")

        doc.add_edge(source, EdgeType.REFERENCES, target)
        removed = doc.remove_edge(source, EdgeType.REFERENCES, target)

        assert removed is True
        assert len(doc.get_edges(source)) == 0

    def test_remove_nonexistent_edge(self):
        """Removing nonexistent edge returns False."""
        doc = create()
        source = doc.add_block(doc.root_id, "Source")
        target = doc.add_block(doc.root_id, "Target")

        removed = doc.remove_edge(source, EdgeType.REFERENCES, target)

        assert removed is False

    def test_has_edge(self):
        """has_edge correctly detects edges."""
        doc = create()
        source = doc.add_block(doc.root_id, "Source")
        target = doc.add_block(doc.root_id, "Target")

        assert doc.has_edge(source, target, EdgeType.REFERENCES) is False

        doc.add_edge(source, EdgeType.REFERENCES, target)

        assert doc.has_edge(source, target, EdgeType.REFERENCES) is True

    def test_multiple_edges_from_same_source(self):
        """Can add multiple edges from same source."""
        doc = create()
        source = doc.add_block(doc.root_id, "Source")
        target1 = doc.add_block(doc.root_id, "Target 1")
        target2 = doc.add_block(doc.root_id, "Target 2")

        doc.add_edge(source, EdgeType.REFERENCES, target1)
        doc.add_edge(source, EdgeType.SUPPORTS, target2)

        edges = doc.get_edges(source)
        assert len(edges) == 2

    def test_different_edge_types_to_same_target(self):
        """Can add different edge types to same target."""
        doc = create()
        source = doc.add_block(doc.root_id, "Source")
        target = doc.add_block(doc.root_id, "Target")

        doc.add_edge(source, EdgeType.REFERENCES, target)
        doc.add_edge(source, EdgeType.ELABORATES, target)

        edges = doc.get_edges(source)
        assert len(edges) == 2
