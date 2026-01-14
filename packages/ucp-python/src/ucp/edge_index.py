"""
EdgeIndex - Bidirectional edge index for efficient traversal.

This module provides O(1) edge lookups in both directions.
"""

from __future__ import annotations

from collections import defaultdict
from typing import Dict, List, Set, Tuple

from .types import Edge, EdgeType


class EdgeIndex:
    """Bidirectional edge index for efficient traversal.
    
    Maintains both outgoing and incoming edge mappings for fast lookups.
    """

    def __init__(self) -> None:
        # outgoing: source -> [(edge_type, target)]
        self._outgoing: Dict[str, List[Tuple[EdgeType, str]]] = defaultdict(list)
        # incoming: target -> [(edge_type, source)]
        self._incoming: Dict[str, List[Tuple[EdgeType, str]]] = defaultdict(list)

    def add_edge(self, source: str, edge: Edge) -> None:
        """Add an edge to the index."""
        self._outgoing[source].append((edge.edge_type, edge.target))
        
        # Track in incoming index
        inverse = edge.edge_type.inverse()
        if inverse:
            self._incoming[edge.target].append((inverse, source))
        else:
            self._incoming[edge.target].append((edge.edge_type, source))

    def remove_edge(self, source: str, target: str, edge_type: EdgeType) -> None:
        """Remove an edge from the index."""
        if source in self._outgoing:
            self._outgoing[source] = [
                (t, tgt) for t, tgt in self._outgoing[source]
                if not (t == edge_type and tgt == target)
            ]
            if not self._outgoing[source]:
                del self._outgoing[source]

        incoming_type = edge_type.inverse() or edge_type
        if target in self._incoming:
            self._incoming[target] = [
                (t, src) for t, src in self._incoming[target]
                if not (t == incoming_type and src == source)
            ]
            if not self._incoming[target]:
                del self._incoming[target]

    def remove_block(self, block_id: str) -> None:
        """Remove all edges involving a block."""
        # Remove outgoing edges
        if block_id in self._outgoing:
            for edge_type, target in self._outgoing[block_id]:
                if target in self._incoming:
                    self._incoming[target] = [
                        (t, src) for t, src in self._incoming[target]
                        if src != block_id
                    ]
            del self._outgoing[block_id]

        # Remove incoming edges
        if block_id in self._incoming:
            for _, source in self._incoming[block_id]:
                if source in self._outgoing:
                    self._outgoing[source] = [
                        (t, tgt) for t, tgt in self._outgoing[source]
                        if tgt != block_id
                    ]
            del self._incoming[block_id]

    def outgoing_from(self, source: str) -> List[Tuple[EdgeType, str]]:
        """Get all outgoing edges from a block."""
        return list(self._outgoing.get(source, []))

    def incoming_to(self, target: str) -> List[Tuple[EdgeType, str]]:
        """Get all incoming edges to a block."""
        return list(self._incoming.get(target, []))

    def outgoing_of_type(self, source: str, edge_type: EdgeType) -> List[str]:
        """Get all targets of edges of a specific type from source."""
        return [
            tgt for t, tgt in self._outgoing.get(source, [])
            if t == edge_type
        ]

    def incoming_of_type(self, target: str, edge_type: EdgeType) -> List[str]:
        """Get all sources of edges of a specific type to target."""
        return [
            src for t, src in self._incoming.get(target, [])
            if t == edge_type
        ]

    def has_edge(self, source: str, target: str, edge_type: EdgeType) -> bool:
        """Check if an edge exists."""
        return any(
            t == edge_type and tgt == target
            for t, tgt in self._outgoing.get(source, [])
        )

    def edge_count(self) -> int:
        """Get total edge count."""
        return sum(len(edges) for edges in self._outgoing.values())

    def clear(self) -> None:
        """Clear all edges."""
        self._outgoing.clear()
        self._incoming.clear()

    def sources(self) -> Set[str]:
        """Get all blocks that have outgoing edges."""
        return set(self._outgoing.keys())

    def targets(self) -> Set[str]:
        """Get all blocks that have incoming edges."""
        return set(self._incoming.keys())
