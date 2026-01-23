"""Tests for Edge operations."""



class TestEdgeTypes:
    """Test edge type enumeration."""

    def test_edge_type_values(self):
        """Test that edge types are accessible."""
        import ucp

        assert ucp.EdgeType.References is not None
        assert ucp.EdgeType.DerivedFrom is not None
        assert ucp.EdgeType.Supports is not None
        assert ucp.EdgeType.Contradicts is not None
        assert ucp.EdgeType.Elaborates is not None
        assert ucp.EdgeType.Summarizes is not None

    def test_edge_type_from_string(self):
        """Test parsing edge type from string."""
        import ucp

        et = ucp.EdgeType.from_string("references")
        assert et == ucp.EdgeType.References

    def test_edge_type_as_string(self):
        """Test converting edge type to string."""
        import ucp

        et = ucp.EdgeType.References
        assert et.as_string() == "references"

    def test_edge_type_is_symmetric(self):
        """Test checking if edge type is symmetric."""
        import ucp

        # SiblingOf should be symmetric
        assert ucp.EdgeType.SiblingOf.is_symmetric() is True
        # References is not symmetric
        assert ucp.EdgeType.References.is_symmetric() is False

    def test_edge_type_is_structural(self):
        """Test checking if edge type is structural."""
        import ucp

        assert ucp.EdgeType.ParentOf.is_structural() is True
        assert ucp.EdgeType.References.is_structural() is False


class TestEdgeCreation:
    """Test edge creation."""

    def test_create_edge(self):
        """Test creating an edge."""
        import ucp

        target = ucp.BlockId.root()
        edge = ucp.Edge(ucp.EdgeType.References, target)

        assert edge.edge_type == ucp.EdgeType.References
        assert edge.target == target

    def test_edge_properties(self):
        """Test edge properties."""
        import ucp

        target = ucp.BlockId.root()
        edge = ucp.Edge(ucp.EdgeType.Supports, target)

        assert edge.created_at is not None
        # confidence and description may be None by default
        assert edge.confidence is None or isinstance(edge.confidence, float)


class TestEdgeOperations:
    """Test edge operations on documents."""

    def test_add_edge(self, doc_with_blocks):
        """Test adding an edge between blocks."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        doc.add_edge(block1, ucp.EdgeType.References, block2)

        edges = doc.outgoing_edges(block1)
        assert len(edges) > 0
        # Check if the edge exists
        found = any(et == ucp.EdgeType.References and target == block2 for et, target in edges)
        assert found is True

    def test_remove_edge(self, doc_with_blocks):
        """Test removing an edge."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        # Add then remove
        doc.add_edge(block1, ucp.EdgeType.Supports, block2)
        removed = doc.remove_edge(block1, ucp.EdgeType.Supports, block2)

        assert removed is True

    def test_outgoing_edges(self, doc_with_blocks):
        """Test getting outgoing edges."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        doc.add_edge(block1, ucp.EdgeType.References, block2)
        doc.add_edge(block1, ucp.EdgeType.Elaborates, block3)

        edges = doc.outgoing_edges(block1)
        assert len(edges) >= 2

    def test_incoming_edges(self, doc_with_blocks):
        """Test getting incoming edges."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        doc.add_edge(block1, ucp.EdgeType.References, block2)
        doc.add_edge(block3, ucp.EdgeType.References, block2)

        edges = doc.incoming_edges(block2)
        assert len(edges) >= 2


class TestBlockEdges:
    """Test edge access from blocks."""

    def test_block_edges(self, doc_with_blocks):
        """Test getting edges from a block."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        doc.add_edge(block1, ucp.EdgeType.References, block2)

        block = doc.get_block(block1)
        edges = block.edges
        assert len(edges) > 0

    def test_block_edges_of_type(self, doc_with_blocks):
        """Test getting edges of a specific type from a block."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        doc.add_edge(block1, ucp.EdgeType.References, block2)
        doc.add_edge(block1, ucp.EdgeType.Supports, block3)

        block = doc.get_block(block1)
        ref_edges = block.edges_of_type(ucp.EdgeType.References)
        assert len(ref_edges) >= 1
