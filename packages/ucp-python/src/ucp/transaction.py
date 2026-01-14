"""
Transaction - Atomic operations with rollback support.

This module provides transaction management for atomic document operations.
"""

from __future__ import annotations

import time
from dataclasses import dataclass, field
from datetime import datetime, timezone
from typing import Any, Dict, List, Optional

from .document import Document
from .observability import EventType, UcpEvent, emit_event
from .snapshot import deserialize_document, serialize_document
from .types import TransactionState


# =============================================================================
# TYPES
# =============================================================================


@dataclass
class Savepoint:
    """A savepoint within a transaction."""
    name: str
    operation_index: int
    document_state: str  # Serialized document
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))


@dataclass
class TransactionOperation:
    """A recorded operation in a transaction."""
    operation_type: str
    args: Dict[str, Any] = field(default_factory=dict)
    timestamp: datetime = field(default_factory=lambda: datetime.now(timezone.utc))


def generate_transaction_id() -> str:
    """Generate a unique transaction ID."""
    return f"txn_{int(time.time() * 1000000):x}"


# =============================================================================
# TRANSACTION
# =============================================================================


class Transaction:
    """A transaction groups operations for atomic execution.
    
    Transactions provide:
    - Atomic commit/rollback of multiple operations
    - Savepoints for partial rollback
    - Timeout protection
    """

    def __init__(
        self,
        doc: Document,
        timeout_seconds: float = 30.0,
        name: Optional[str] = None,
    ) -> None:
        self.id = name or generate_transaction_id()
        self.name = name
        self._doc = doc
        self._state = TransactionState.ACTIVE
        self._start_time = time.time()
        self._timeout = timeout_seconds
        self._created_at = datetime.now(timezone.utc)
        
        # Store initial state for rollback
        self._initial_state = serialize_document(doc)
        
        # Operations and savepoints
        self._operations: List[TransactionOperation] = []
        self._savepoints: List[Savepoint] = []

    @property
    def state(self) -> TransactionState:
        """Get current transaction state."""
        if self._state == TransactionState.ACTIVE and self.is_timed_out():
            self._state = TransactionState.TIMED_OUT
        return self._state

    def is_active(self) -> bool:
        """Check if transaction is active."""
        return self.state == TransactionState.ACTIVE

    def is_timed_out(self) -> bool:
        """Check if transaction has timed out."""
        return time.time() - self._start_time > self._timeout

    def elapsed_seconds(self) -> float:
        """Get elapsed time in seconds."""
        return time.time() - self._start_time

    def operation_count(self) -> int:
        """Get number of operations."""
        return len(self._operations)

    def _check_active(self) -> None:
        """Raise if transaction is not active."""
        if not self.is_active():
            raise RuntimeError(f"Transaction is {self.state.value}, not active")

    def record_operation(self, op_type: str, **args: Any) -> None:
        """Record an operation in the transaction."""
        self._check_active()
        self._operations.append(TransactionOperation(
            operation_type=op_type,
            args=args,
        ))

    def savepoint(self, name: str) -> None:
        """Create a savepoint for partial rollback."""
        self._check_active()
        self._savepoints.append(Savepoint(
            name=name,
            operation_index=len(self._operations),
            document_state=serialize_document(self._doc),
        ))

    def rollback_to_savepoint(self, name: str) -> None:
        """Rollback to a named savepoint."""
        self._check_active()
        
        # Find savepoint
        savepoint = None
        savepoint_index = -1
        for i, sp in enumerate(self._savepoints):
            if sp.name == name:
                savepoint = sp
                savepoint_index = i
                break
        
        if savepoint is None:
            raise ValueError(f"Savepoint not found: {name}")
        
        # Restore document state
        restored = deserialize_document(savepoint.document_state)
        self._doc.blocks = restored.blocks
        self._doc.structure = restored.structure
        self._doc.metadata = restored.metadata
        self._doc._version = restored._version
        self._doc.rebuild_indices()
        
        # Trim operations and savepoints
        self._operations = self._operations[:savepoint.operation_index]
        self._savepoints = self._savepoints[:savepoint_index + 1]

    def commit(self) -> None:
        """Commit the transaction."""
        self._check_active()
        self._state = TransactionState.COMMITTED
        
        emit_event(UcpEvent(
            event_type=EventType.TRANSACTION_COMMITTED.value,
            data={
                "transaction_id": self.id,
                "operation_count": len(self._operations),
                "elapsed_seconds": self.elapsed_seconds(),
            },
        ))

    def rollback(self) -> None:
        """Rollback all changes in the transaction."""
        if self._state == TransactionState.COMMITTED:
            raise RuntimeError("Cannot rollback a committed transaction")
        
        # Restore initial document state
        restored = deserialize_document(self._initial_state)
        self._doc.blocks = restored.blocks
        self._doc.structure = restored.structure
        self._doc.metadata = restored.metadata
        self._doc._version = restored._version
        self._doc.rebuild_indices()
        
        self._state = TransactionState.ROLLED_BACK
        
        emit_event(UcpEvent(
            event_type=EventType.TRANSACTION_ROLLED_BACK.value,
            data={
                "transaction_id": self.id,
                "operation_count": len(self._operations),
            },
        ))


# =============================================================================
# TRANSACTION MANAGER
# =============================================================================


class TransactionManager:
    """Manages active transactions."""

    def __init__(self, default_timeout: float = 30.0) -> None:
        self._transactions: Dict[str, Transaction] = {}
        self._default_timeout = default_timeout

    def begin(
        self,
        doc: Document,
        name: Optional[str] = None,
        timeout: Optional[float] = None,
    ) -> Transaction:
        """Begin a new transaction.
        
        Args:
            doc: Document to operate on
            name: Optional transaction name
            timeout: Timeout in seconds (default: 30)
            
        Returns:
            New Transaction instance
        """
        txn = Transaction(
            doc,
            timeout_seconds=timeout or self._default_timeout,
            name=name,
        )
        self._transactions[txn.id] = txn
        
        emit_event(UcpEvent(
            event_type=EventType.TRANSACTION_STARTED.value,
            data={"transaction_id": txn.id, "document_id": doc.id},
        ))
        
        return txn

    def get(self, txn_id: str) -> Optional[Transaction]:
        """Get a transaction by ID."""
        return self._transactions.get(txn_id)

    def commit(self, txn_id: str) -> None:
        """Commit a transaction by ID."""
        txn = self._transactions.get(txn_id)
        if txn is None:
            raise KeyError(f"Transaction not found: {txn_id}")
        txn.commit()

    def rollback(self, txn_id: str) -> None:
        """Rollback a transaction by ID."""
        txn = self._transactions.get(txn_id)
        if txn is None:
            raise KeyError(f"Transaction not found: {txn_id}")
        txn.rollback()

    def active_count(self) -> int:
        """Get count of active transactions."""
        return sum(1 for t in self._transactions.values() if t.is_active())

    def cleanup(self) -> int:
        """Remove completed and timed-out transactions. Returns count removed."""
        to_remove = [
            tid for tid, txn in self._transactions.items()
            if not txn.is_active()
        ]
        for tid in to_remove:
            del self._transactions[tid]
        return len(to_remove)


# =============================================================================
# CONTEXT MANAGER
# =============================================================================


class TransactionContext:
    """Context manager for transactions."""

    def __init__(
        self,
        doc: Document,
        manager: Optional[TransactionManager] = None,
        timeout: float = 30.0,
    ) -> None:
        self._doc = doc
        self._manager = manager or TransactionManager()
        self._timeout = timeout
        self._txn: Optional[Transaction] = None

    def __enter__(self) -> Transaction:
        self._txn = self._manager.begin(self._doc, timeout=self._timeout)
        return self._txn

    def __exit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> bool:
        if self._txn is None:
            return False
        
        if exc_type is not None:
            # Exception occurred - rollback
            if self._txn.is_active():
                self._txn.rollback()
            return False
        
        # No exception - commit if still active
        if self._txn.is_active():
            self._txn.commit()
        
        return False


def transaction(doc: Document, timeout: float = 30.0) -> TransactionContext:
    """Create a transaction context manager.
    
    Usage:
        with transaction(doc) as txn:
            doc.edit_block(block_id, "new content")
            txn.savepoint("after_edit")
            doc.add_block(root, "new block")
        # Auto-commits on exit, auto-rollbacks on exception
    """
    return TransactionContext(doc, timeout=timeout)
