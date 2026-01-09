"""
Snapshot - Document versioning and restore functionality.

This module provides snapshot management for documents.
"""

from __future__ import annotations

import json
from dataclasses import dataclass, field
from datetime import datetime, timezone
from typing import Any, Dict, List, Optional

from .document import Document
from .observability import EventType, UcpEvent, emit_event


# =============================================================================
# SNAPSHOT TYPES
# =============================================================================


@dataclass
class Snapshot:
    """A snapshot of document state."""
    id: str
    description: Optional[str] = None
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    document_version: int = 0
    data: str = ""  # JSON serialized document


@dataclass
class SnapshotInfo:
    """Summary information about a snapshot."""
    id: str
    description: Optional[str]
    created_at: datetime
    document_version: int
    block_count: int


# =============================================================================
# SERIALIZATION
# =============================================================================


def serialize_document(doc: Document) -> str:
    """Serialize a document to JSON string."""
    from .block import Block
    
    def serialize_block(block: Block) -> Dict[str, Any]:
        return {
            "id": block.id,
            "content": block.content,
            "content_type": block.content_type.value,
            "metadata": {
                "semantic_role": block.metadata.semantic_role.value if block.metadata.semantic_role else None,
                "label": block.metadata.label,
                "tags": block.metadata.tags,
                "summary": block.metadata.summary,
                "created_at": block.metadata.created_at.isoformat(),
                "modified_at": block.metadata.modified_at.isoformat(),
                "custom": block.metadata.custom,
            },
            "edges": [
                {
                    "edge_type": e.edge_type.value,
                    "target": e.target,
                    "metadata": {
                        "confidence": e.metadata.confidence,
                        "description": e.metadata.description,
                        "custom": e.metadata.custom,
                    },
                    "created_at": e.created_at.isoformat(),
                }
                for e in block.edges
            ],
            "children": block.children,
        }
    
    serialized = {
        "id": doc.id,
        "root": doc.root,
        "version": doc._version,
        "metadata": {
            "title": doc.metadata.title,
            "description": doc.metadata.description,
            "authors": doc.metadata.authors,
            "language": doc.metadata.language,
            "created_at": doc.metadata.created_at.isoformat(),
            "modified_at": doc.metadata.modified_at.isoformat(),
            "custom": doc.metadata.custom,
        },
        "structure": doc.structure,
        "blocks": {bid: serialize_block(b) for bid, b in doc.blocks.items()},
    }
    
    return json.dumps(serialized)


def deserialize_document(data: str) -> Document:
    """Deserialize a document from JSON string."""
    from .block import Block
    from .types import (
        BlockMetadata,
        ContentType,
        DocumentMetadata,
        Edge,
        EdgeMetadata,
        EdgeType,
        SemanticRole,
    )
    
    parsed = json.loads(data)
    
    # Create document metadata
    meta_data = parsed.get("metadata", {})
    metadata = DocumentMetadata(
        title=meta_data.get("title"),
        description=meta_data.get("description"),
        authors=meta_data.get("authors", []),
        language=meta_data.get("language"),
        custom=meta_data.get("custom", {}),
    )
    if meta_data.get("created_at"):
        metadata.created_at = datetime.fromisoformat(meta_data["created_at"])
    if meta_data.get("modified_at"):
        metadata.modified_at = datetime.fromisoformat(meta_data["modified_at"])
    
    # Create document
    doc = Document(doc_id=parsed["id"], metadata=metadata)
    doc.root = parsed["root"]
    doc._version = parsed.get("version", 1)
    doc.structure = {k: list(v) for k, v in parsed.get("structure", {}).items()}
    
    # Deserialize blocks
    doc.blocks.clear()
    for bid, block_data in parsed.get("blocks", {}).items():
        block_meta = block_data.get("metadata", {})
        
        # Create block metadata
        role = None
        if block_meta.get("semantic_role"):
            try:
                role = SemanticRole(block_meta["semantic_role"])
            except ValueError:
                pass
        
        bm = BlockMetadata(
            semantic_role=role,
            label=block_meta.get("label"),
            tags=block_meta.get("tags", []),
            summary=block_meta.get("summary"),
            custom=block_meta.get("custom", {}),
        )
        if block_meta.get("created_at"):
            bm.created_at = datetime.fromisoformat(block_meta["created_at"])
        if block_meta.get("modified_at"):
            bm.modified_at = datetime.fromisoformat(block_meta["modified_at"])
        
        # Create edges
        edges = []
        for edge_data in block_data.get("edges", []):
            edge_type = EdgeType.from_str(edge_data["edge_type"])
            if edge_type:
                em = EdgeMetadata(
                    confidence=edge_data.get("metadata", {}).get("confidence"),
                    description=edge_data.get("metadata", {}).get("description"),
                    custom=edge_data.get("metadata", {}).get("custom", {}),
                )
                edge = Edge(
                    edge_type=edge_type,
                    target=edge_data["target"],
                    metadata=em,
                )
                if edge_data.get("created_at"):
                    edge.created_at = datetime.fromisoformat(edge_data["created_at"])
                edges.append(edge)
        
        # Create block
        try:
            content_type = ContentType(block_data.get("content_type", "text"))
        except ValueError:
            content_type = ContentType.TEXT
        
        block = Block(
            id=bid,
            content=block_data.get("content", ""),
            content_type=content_type,
            metadata=bm,
            edges=edges,
            children=block_data.get("children", []),
        )
        doc.blocks[bid] = block
    
    # Rebuild indices
    doc.rebuild_indices()
    
    return doc


# =============================================================================
# SNAPSHOT MANAGER
# =============================================================================


class SnapshotManager:
    """Manages document snapshots."""

    def __init__(self, max_snapshots: int = 100) -> None:
        self._snapshots: Dict[str, Snapshot] = {}
        self._max_snapshots = max_snapshots

    def create(
        self,
        name: str,
        doc: Document,
        description: Optional[str] = None,
    ) -> str:
        """Create a snapshot of the document.
        
        Args:
            name: Snapshot name/ID
            doc: Document to snapshot
            description: Optional description
            
        Returns:
            Snapshot ID
        """
        # Evict oldest if at capacity
        if len(self._snapshots) >= self._max_snapshots:
            self._evict_oldest()
        
        data = serialize_document(doc)
        
        snapshot = Snapshot(
            id=name,
            description=description,
            document_version=doc.version,
            data=data,
        )
        
        self._snapshots[name] = snapshot
        
        emit_event(UcpEvent(
            event_type=EventType.SNAPSHOT_CREATED.value,
            data={"snapshot_id": name, "document_id": doc.id},
        ))
        
        return name

    def restore(self, name: str) -> Document:
        """Restore a document from a snapshot.
        
        Args:
            name: Snapshot name/ID
            
        Returns:
            Restored document
            
        Raises:
            KeyError: If snapshot not found
        """
        if name not in self._snapshots:
            raise KeyError(f"Snapshot not found: {name}")
        
        snapshot = self._snapshots[name]
        doc = deserialize_document(snapshot.data)
        
        emit_event(UcpEvent(
            event_type=EventType.SNAPSHOT_RESTORED.value,
            data={"snapshot_id": name, "document_id": doc.id},
        ))
        
        return doc

    def get(self, name: str) -> Optional[Snapshot]:
        """Get a snapshot by name."""
        return self._snapshots.get(name)

    def get_info(self, name: str) -> Optional[SnapshotInfo]:
        """Get snapshot info without loading full data."""
        snapshot = self._snapshots.get(name)
        if not snapshot:
            return None
        
        # Count blocks from serialized data
        try:
            parsed = json.loads(snapshot.data)
            block_count = len(parsed.get("blocks", {}))
        except Exception:
            block_count = 0
        
        return SnapshotInfo(
            id=snapshot.id,
            description=snapshot.description,
            created_at=snapshot.created_at,
            document_version=snapshot.document_version,
            block_count=block_count,
        )

    def list(self) -> List[SnapshotInfo]:
        """List all snapshots (sorted by creation time, newest first)."""
        infos = []
        for name in self._snapshots:
            info = self.get_info(name)
            if info:
                infos.append(info)
        return sorted(infos, key=lambda s: s.created_at, reverse=True)

    def delete(self, name: str) -> bool:
        """Delete a snapshot. Returns True if deleted."""
        if name in self._snapshots:
            del self._snapshots[name]
            return True
        return False

    def exists(self, name: str) -> bool:
        """Check if a snapshot exists."""
        return name in self._snapshots

    def count(self) -> int:
        """Get number of snapshots."""
        return len(self._snapshots)

    def _evict_oldest(self) -> None:
        """Evict the oldest snapshot."""
        if not self._snapshots:
            return
        
        oldest_name = min(
            self._snapshots.keys(),
            key=lambda k: self._snapshots[k].created_at,
        )
        del self._snapshots[oldest_name]
