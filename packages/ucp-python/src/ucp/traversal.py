"""
Graph traversal operations for UCM documents.

This module provides utilities for navigating the document's block structure,
including BFS, DFS, and path-finding operations.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum
from typing import Optional, List, Dict, Set
from collections import deque

from .document import Document


class NavigateDirection(Enum):
    """Direction for navigation operations."""
    DOWN = "down"
    UP = "up"
    BOTH = "both"
    SIBLINGS = "siblings"
    BREADTH_FIRST = "breadth_first"
    DEPTH_FIRST = "depth_first"


class TraversalOutput(Enum):
    """Output format for traversal results."""
    STRUCTURE_ONLY = "structure_only"
    STRUCTURE_AND_BLOCKS = "structure_and_blocks"
    STRUCTURE_WITH_PREVIEWS = "structure_with_previews"


@dataclass
class TraversalFilter:
    """Filter criteria for traversal."""
    include_roles: List[str] = field(default_factory=list)
    exclude_roles: List[str] = field(default_factory=list)
    include_tags: List[str] = field(default_factory=list)
    exclude_tags: List[str] = field(default_factory=list)
    content_pattern: Optional[str] = None


@dataclass
class TraversalNode:
    """A node in the traversal result."""
    id: str
    depth: int
    parent_id: Optional[str]
    content_preview: Optional[str]
    semantic_role: Optional[str]
    child_count: int
    edge_count: int


@dataclass
class TraversalSummary:
    """Summary statistics for a traversal."""
    total_nodes: int
    total_edges: int
    max_depth: int
    nodes_by_role: Dict[str, int] = field(default_factory=dict)
    truncated: bool = False
    truncation_reason: Optional[str] = None


@dataclass
class TraversalResult:
    """Complete traversal result."""
    nodes: List[TraversalNode]
    paths: List[List[str]]
    summary: TraversalSummary
    
    @classmethod
    def empty(cls) -> "TraversalResult":
        return cls(
            nodes=[],
            paths=[],
            summary=TraversalSummary(total_nodes=0, total_edges=0, max_depth=0)
        )


@dataclass
class TraversalConfig:
    """Configuration for the traversal engine."""
    max_depth: int = 100
    max_nodes: int = 10000
    default_preview_length: int = 100


class TraversalEngine:
    """Graph traversal engine for UCM documents."""
    
    def __init__(self, config: Optional[TraversalConfig] = None):
        self.config = config or TraversalConfig()
    
    def navigate(
        self,
        doc: Document,
        start_id: Optional[str] = None,
        direction: NavigateDirection = NavigateDirection.BREADTH_FIRST,
        depth: Optional[int] = None,
        filter: Optional[TraversalFilter] = None,
        output: TraversalOutput = TraversalOutput.STRUCTURE_AND_BLOCKS,
    ) -> TraversalResult:
        """
        Navigate from a starting point in a specific direction.
        
        Args:
            doc: The document to traverse
            start_id: Starting block ID (defaults to root)
            direction: Navigation direction
            depth: Maximum depth (defaults to config max_depth)
            filter: Optional filter criteria
            output: Output format
            
        Returns:
            TraversalResult with nodes and summary
        """
        start = start_id or doc.root
        max_depth = min(depth or self.config.max_depth, self.config.max_depth)
        filter = filter or TraversalFilter()
        
        if direction == NavigateDirection.BREADTH_FIRST:
            return self._traverse_bfs(doc, start, max_depth, filter, output)
        elif direction == NavigateDirection.DEPTH_FIRST:
            return self._traverse_dfs(doc, start, max_depth, filter, output)
        elif direction == NavigateDirection.DOWN:
            return self._traverse_bfs(doc, start, max_depth, filter, output)
        elif direction == NavigateDirection.UP:
            return self._traverse_up(doc, start, max_depth, filter, output)
        elif direction == NavigateDirection.SIBLINGS:
            return self._traverse_siblings(doc, start, filter, output)
        elif direction == NavigateDirection.BOTH:
            return self._traverse_both(doc, start, max_depth, filter, output)
        
        return TraversalResult.empty()
    
    def expand(
        self,
        doc: Document,
        node_id: str,
        output: TraversalOutput = TraversalOutput.STRUCTURE_AND_BLOCKS,
    ) -> TraversalResult:
        """Expand a node to get its immediate children."""
        return self.navigate(doc, node_id, NavigateDirection.DOWN, depth=1, output=output)
    
    def path_to_root(self, doc: Document, node_id: str) -> List[str]:
        """Get the path from a node to the root."""
        path = [node_id]
        current = node_id
        
        while True:
            parent = doc.parent(current)
            if parent is None:
                break
            path.append(parent)
            if parent == doc.root:
                break
            current = parent
        
        path.reverse()
        return path
    
    def find_paths(
        self,
        doc: Document,
        from_id: str,
        to_id: str,
        max_paths: int = 10,
    ) -> List[List[str]]:
        """Find all paths between two nodes."""
        paths: List[List[str]] = []
        visited: Set[str] = set()
        current_path = [from_id]
        
        self._find_paths_recursive(doc, from_id, to_id, visited, current_path, paths, max_paths)
        
        return paths
    
    def _find_paths_recursive(
        self,
        doc: Document,
        current: str,
        target: str,
        visited: Set[str],
        current_path: List[str],
        paths: List[List[str]],
        max_paths: int,
    ) -> None:
        if len(paths) >= max_paths:
            return
        
        if current == target:
            paths.append(current_path.copy())
            return
        
        visited.add(current)
        
        # Check children
        for child in doc.children(current):
            if child not in visited:
                current_path.append(child)
                self._find_paths_recursive(doc, child, target, visited, current_path, paths, max_paths)
                current_path.pop()
        
        visited.remove(current)
    
    def _traverse_bfs(
        self,
        doc: Document,
        start: str,
        max_depth: int,
        filter: TraversalFilter,
        output: TraversalOutput,
    ) -> TraversalResult:
        nodes = []
        visited: Set[str] = set()
        queue = deque([(start, None, 0)])  # (node_id, parent_id, depth)
        nodes_by_role: Dict[str, int] = {}
        
        while queue and len(nodes) < self.config.max_nodes:
            node_id, parent_id, depth = queue.popleft()
            
            if depth > max_depth or node_id in visited:
                continue
            
            visited.add(node_id)
            block = doc.blocks.get(node_id)
            
            if block and self._matches_filter(block, filter):
                node = self._create_node(doc, node_id, depth, parent_id, output)
                nodes.append(node)
                
                if node.semantic_role:
                    nodes_by_role[node.semantic_role] = nodes_by_role.get(node.semantic_role, 0) + 1
                
                # Add children to queue
                for child in doc.children(node_id):
                    if child not in visited:
                        queue.append((child, node_id, depth + 1))
        
        max_depth_found = max((n.depth for n in nodes), default=0)
        
        return TraversalResult(
            nodes=nodes,
            paths=[],
            summary=TraversalSummary(
                total_nodes=len(nodes),
                total_edges=0,
                max_depth=max_depth_found,
                nodes_by_role=nodes_by_role,
                truncated=len(nodes) >= self.config.max_nodes,
            )
        )
    
    def _traverse_dfs(
        self,
        doc: Document,
        start: str,
        max_depth: int,
        filter: TraversalFilter,
        output: TraversalOutput,
    ) -> TraversalResult:
        nodes: List[TraversalNode] = []
        visited: Set[str] = set()
        nodes_by_role: Dict[str, int] = {}
        
        self._dfs_recursive(doc, start, None, 0, max_depth, filter, output, visited, nodes, nodes_by_role)
        
        max_depth_found = max((n.depth for n in nodes), default=0)
        
        return TraversalResult(
            nodes=nodes,
            paths=[],
            summary=TraversalSummary(
                total_nodes=len(nodes),
                total_edges=0,
                max_depth=max_depth_found,
                nodes_by_role=nodes_by_role,
            )
        )
    
    def _dfs_recursive(
        self,
        doc: Document,
        node_id: str,
        parent_id: Optional[str],
        depth: int,
        max_depth: int,
        filter: TraversalFilter,
        output: TraversalOutput,
        visited: Set[str],
        nodes: List[TraversalNode],
        nodes_by_role: Dict[str, int],
    ) -> None:
        if depth > max_depth or node_id in visited or len(nodes) >= self.config.max_nodes:
            return
        
        visited.add(node_id)
        block = doc.blocks.get(node_id)
        
        if block and self._matches_filter(block, filter):
            node = self._create_node(doc, node_id, depth, parent_id, output)
            nodes.append(node)
            
            if node.semantic_role:
                nodes_by_role[node.semantic_role] = nodes_by_role.get(node.semantic_role, 0) + 1
            
            for child in doc.children(node_id):
                self._dfs_recursive(doc, child, node_id, depth + 1, max_depth, filter, output, visited, nodes, nodes_by_role)
    
    def _traverse_up(
        self,
        doc: Document,
        start: str,
        max_depth: int,
        filter: TraversalFilter,
        output: TraversalOutput,
    ) -> TraversalResult:
        nodes = []
        current = start
        depth = 0
        
        while depth <= max_depth:
            block = doc.blocks.get(current)
            if block and self._matches_filter(block, filter):
                nodes.append(self._create_node(doc, current, depth, None, output))
            
            parent = doc.parent(current)
            if parent is None:
                break
            current = parent
            depth += 1
        
        return TraversalResult(
            nodes=nodes,
            paths=[],
            summary=TraversalSummary(
                total_nodes=len(nodes),
                total_edges=0,
                max_depth=depth,
            )
        )
    
    def _traverse_siblings(
        self,
        doc: Document,
        start: str,
        filter: TraversalFilter,
        output: TraversalOutput,
    ) -> TraversalResult:
        nodes = []
        parent = doc.parent(start)
        
        if parent:
            for sibling in doc.children(parent):
                block = doc.blocks.get(sibling)
                if block and self._matches_filter(block, filter):
                    nodes.append(self._create_node(doc, sibling, 0, parent, output))
        
        return TraversalResult(
            nodes=nodes,
            paths=[],
            summary=TraversalSummary(
                total_nodes=len(nodes),
                total_edges=0,
                max_depth=0,
            )
        )
    
    def _traverse_both(
        self,
        doc: Document,
        start: str,
        max_depth: int,
        filter: TraversalFilter,
        output: TraversalOutput,
    ) -> TraversalResult:
        up_result = self._traverse_up(doc, start, max_depth, filter, output)
        down_result = self._traverse_bfs(doc, start, max_depth, filter, output)
        
        # Merge results
        seen = set()
        nodes = []
        for node in up_result.nodes + down_result.nodes:
            if node.id not in seen:
                seen.add(node.id)
                nodes.append(node)
        
        max_depth_found = max((n.depth for n in nodes), default=0)
        
        return TraversalResult(
            nodes=nodes,
            paths=[],
            summary=TraversalSummary(
                total_nodes=len(nodes),
                total_edges=0,
                max_depth=max_depth_found,
            )
        )
    
    def _matches_filter(self, block, filter: TraversalFilter) -> bool:
        """Check if a block matches the filter criteria."""
        role = ""
        tags: List[str] = []

        if block.metadata:
            if block.metadata.semantic_role:
                role = block.metadata.semantic_role.value
            tags = list(block.metadata.tags)
        
        # Check role inclusion
        if filter.include_roles and role not in filter.include_roles:
            return False
        
        # Check role exclusion
        if filter.exclude_roles and role in filter.exclude_roles:
            return False
        
        # Check tag inclusion
        if filter.include_tags:
            if not any(t in tags for t in filter.include_tags):
                return False
        
        # Check tag exclusion
        if filter.exclude_tags:
            if any(t in tags for t in filter.exclude_tags):
                return False
        
        # Check content pattern
        if filter.content_pattern:
            content = str(block.content) if block.content else ""
            if filter.content_pattern.lower() not in content.lower():
                return False
        
        return True
    
    def _create_node(
        self,
        doc: Document,
        block_id: str,
        depth: int,
        parent_id: Optional[str],
        output: TraversalOutput,
    ) -> TraversalNode:
        block = doc.blocks.get(block_id)
        children = doc.children(block_id)
        
        content_preview = None
        if output != TraversalOutput.STRUCTURE_ONLY and block:
            content = str(block.content) if block.content else ""
            if len(content) > self.config.default_preview_length:
                content_preview = content[:self.config.default_preview_length] + "..."
            else:
                content_preview = content
        
        semantic_role = None
        edge_count = 0
        if block and block.metadata:
            if block.metadata.semantic_role:
                semantic_role = block.metadata.semantic_role.value
            edge_count = len(block.edges)
        
        return TraversalNode(
            id=block_id,
            depth=depth,
            parent_id=parent_id,
            content_preview=content_preview,
            semantic_role=semantic_role,
            child_count=len(children),
            edge_count=edge_count,
        )


# Convenience functions

def traverse(
    doc: Document,
    start_id: Optional[str] = None,
    direction: NavigateDirection = NavigateDirection.BREADTH_FIRST,
    depth: Optional[int] = None,
) -> TraversalResult:
    """
    Convenience function for document traversal.
    
    Example:
        >>> result = traverse(doc, direction=NavigateDirection.DEPTH_FIRST, depth=3)
        >>> for node in result.nodes:
        ...     print(f"{' ' * node.depth}{node.semantic_role}: {node.content_preview}")
    """
    return TraversalEngine().navigate(doc, start_id, direction, depth)


def path_to_root(doc: Document, node_id: str) -> List[str]:
    """Get the path from a node to the root."""
    return TraversalEngine().path_to_root(doc, node_id)


def expand(doc: Document, node_id: str) -> TraversalResult:
    """Expand a node to get its immediate children."""
    return TraversalEngine().expand(doc, node_id)
