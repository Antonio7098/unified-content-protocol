"""
Observability - Logging, tracing, and event handling for UCP.

This module provides observability features following the Observer pattern.
"""

from __future__ import annotations

import logging
import time
from contextlib import contextmanager
from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Callable, Dict, Generator, List, Optional

from .types import UcpEvent


# =============================================================================
# LOGGING SETUP
# =============================================================================


class LogLevel(str, Enum):
    """Log levels."""
    DEBUG = "debug"
    INFO = "info"
    WARNING = "warning"
    ERROR = "error"


def get_logger(name: str = "ucp") -> logging.Logger:
    """Get a configured logger."""
    logger = logging.getLogger(name)
    if not logger.handlers:
        handler = logging.StreamHandler()
        formatter = logging.Formatter(
            "%(asctime)s - %(name)s - %(levelname)s - %(message)s"
        )
        handler.setFormatter(formatter)
        logger.addHandler(handler)
        logger.setLevel(logging.INFO)
    return logger


_logger = get_logger()


# =============================================================================
# EVENT TYPES
# =============================================================================


class EventType(str, Enum):
    """Types of UCP events."""
    # Document events
    DOCUMENT_CREATED = "document.created"
    DOCUMENT_MODIFIED = "document.modified"
    
    # Block events
    BLOCK_ADDED = "block.added"
    BLOCK_EDITED = "block.edited"
    BLOCK_MOVED = "block.moved"
    BLOCK_DELETED = "block.deleted"
    
    # Edge events
    EDGE_ADDED = "edge.added"
    EDGE_REMOVED = "edge.removed"
    
    # Tag events
    TAG_ADDED = "tag.added"
    TAG_REMOVED = "tag.removed"
    
    # UCL events
    UCL_PARSED = "ucl.parsed"
    UCL_EXECUTED = "ucl.executed"
    UCL_ERROR = "ucl.error"
    
    # Validation events
    VALIDATION_STARTED = "validation.started"
    VALIDATION_COMPLETED = "validation.completed"
    
    # Transaction events
    TRANSACTION_STARTED = "transaction.started"
    TRANSACTION_COMMITTED = "transaction.committed"
    TRANSACTION_ROLLED_BACK = "transaction.rolled_back"
    
    # Snapshot events
    SNAPSHOT_CREATED = "snapshot.created"
    SNAPSHOT_RESTORED = "snapshot.restored"


@dataclass
class DocumentEvent(UcpEvent):
    """Event related to document operations."""
    document_id: str = ""
    block_id: Optional[str] = None
    operation: str = ""


@dataclass
class UclEvent(UcpEvent):
    """Event related to UCL execution."""
    command: str = ""
    success: bool = True
    error_message: Optional[str] = None
    affected_blocks: List[str] = field(default_factory=list)


@dataclass
class SpanContext:
    """Context for a traced span."""
    trace_id: str
    span_id: str
    parent_span_id: Optional[str] = None
    start_time: float = field(default_factory=time.time)
    end_time: Optional[float] = None
    name: str = ""
    attributes: Dict[str, Any] = field(default_factory=dict)

    def duration_ms(self) -> Optional[float]:
        """Get duration in milliseconds."""
        if self.end_time is None:
            return None
        return (self.end_time - self.start_time) * 1000


# =============================================================================
# EVENT BUS
# =============================================================================


EventHandler = Callable[[UcpEvent], None]


class EventBus:
    """Simple event bus for publishing and subscribing to events."""

    _instance: Optional["EventBus"] = None

    def __init__(self) -> None:
        self._handlers: Dict[str, List[EventHandler]] = {}
        self._global_handlers: List[EventHandler] = []

    @classmethod
    def get_instance(cls) -> "EventBus":
        """Get the singleton event bus instance."""
        if cls._instance is None:
            cls._instance = cls()
        return cls._instance

    def subscribe(self, event_type: str, handler: EventHandler) -> None:
        """Subscribe to a specific event type."""
        if event_type not in self._handlers:
            self._handlers[event_type] = []
        self._handlers[event_type].append(handler)

    def subscribe_all(self, handler: EventHandler) -> None:
        """Subscribe to all events."""
        self._global_handlers.append(handler)

    def unsubscribe(self, event_type: str, handler: EventHandler) -> None:
        """Unsubscribe from an event type."""
        if event_type in self._handlers:
            self._handlers[event_type] = [
                h for h in self._handlers[event_type] if h != handler
            ]

    def unsubscribe_all(self, handler: EventHandler) -> None:
        """Unsubscribe from all events."""
        self._global_handlers = [h for h in self._global_handlers if h != handler]

    def publish(self, event: UcpEvent) -> None:
        """Publish an event to all subscribers."""
        # Call specific handlers
        for handler in self._handlers.get(event.event_type, []):
            try:
                handler(event)
            except Exception as e:
                _logger.error(f"Event handler error: {e}")

        # Call global handlers
        for handler in self._global_handlers:
            try:
                handler(event)
            except Exception as e:
                _logger.error(f"Global event handler error: {e}")

    def clear(self) -> None:
        """Clear all handlers."""
        self._handlers.clear()
        self._global_handlers.clear()


# Global event bus
_event_bus = EventBus.get_instance()


def emit_event(event: UcpEvent) -> None:
    """Emit an event to the global event bus."""
    _event_bus.publish(event)


def on_event(event_type: str) -> Callable[[EventHandler], EventHandler]:
    """Decorator to subscribe a function to an event type."""
    def decorator(handler: EventHandler) -> EventHandler:
        _event_bus.subscribe(event_type, handler)
        return handler
    return decorator


# =============================================================================
# TRACER
# =============================================================================


class Tracer:
    """Simple tracer for performance monitoring."""

    _instance: Optional["Tracer"] = None

    def __init__(self) -> None:
        self._spans: List[SpanContext] = []
        self._active_span: Optional[SpanContext] = None
        self._trace_counter = 0
        self._span_counter = 0
        self._enabled = True

    @classmethod
    def get_instance(cls) -> "Tracer":
        """Get the singleton tracer instance."""
        if cls._instance is None:
            cls._instance = cls()
        return cls._instance

    def enable(self) -> None:
        """Enable tracing."""
        self._enabled = True

    def disable(self) -> None:
        """Disable tracing."""
        self._enabled = False

    def _generate_trace_id(self) -> str:
        self._trace_counter += 1
        return f"trace_{self._trace_counter:08x}"

    def _generate_span_id(self) -> str:
        self._span_counter += 1
        return f"span_{self._span_counter:08x}"

    @contextmanager
    def span(self, name: str, **attributes: Any) -> Generator[SpanContext, None, None]:
        """Create a traced span."""
        if not self._enabled:
            # Return a dummy context
            yield SpanContext(trace_id="", span_id="", name=name)
            return

        trace_id = (
            self._active_span.trace_id if self._active_span
            else self._generate_trace_id()
        )
        parent_span_id = self._active_span.span_id if self._active_span else None

        span = SpanContext(
            trace_id=trace_id,
            span_id=self._generate_span_id(),
            parent_span_id=parent_span_id,
            name=name,
            attributes=attributes,
        )

        old_active = self._active_span
        self._active_span = span

        try:
            yield span
        finally:
            span.end_time = time.time()
            self._spans.append(span)
            self._active_span = old_active

            # Log span completion
            duration = span.duration_ms()
            _logger.debug(
                f"Span '{name}' completed in {duration:.2f}ms"
                if duration else f"Span '{name}' completed"
            )

    def get_spans(self) -> List[SpanContext]:
        """Get all recorded spans."""
        return list(self._spans)

    def clear_spans(self) -> None:
        """Clear recorded spans."""
        self._spans.clear()


# Global tracer
_tracer = Tracer.get_instance()


def trace(name: str, **attributes: Any) -> Callable:
    """Decorator to trace a function."""
    def decorator(func: Callable) -> Callable:
        def wrapper(*args: Any, **kwargs: Any) -> Any:
            with _tracer.span(name, **attributes):
                return func(*args, **kwargs)
        return wrapper
    return decorator


# =============================================================================
# METRICS
# =============================================================================


class Metrics:
    """Simple metrics collector."""

    _instance: Optional["Metrics"] = None

    def __init__(self) -> None:
        self._counters: Dict[str, int] = {}
        self._gauges: Dict[str, float] = {}
        self._histograms: Dict[str, List[float]] = {}

    @classmethod
    def get_instance(cls) -> "Metrics":
        """Get the singleton metrics instance."""
        if cls._instance is None:
            cls._instance = cls()
        return cls._instance

    def increment(self, name: str, value: int = 1) -> None:
        """Increment a counter."""
        self._counters[name] = self._counters.get(name, 0) + value

    def set_gauge(self, name: str, value: float) -> None:
        """Set a gauge value."""
        self._gauges[name] = value

    def record_histogram(self, name: str, value: float) -> None:
        """Record a value in a histogram."""
        if name not in self._histograms:
            self._histograms[name] = []
        self._histograms[name].append(value)

    def get_counter(self, name: str) -> int:
        """Get counter value."""
        return self._counters.get(name, 0)

    def get_gauge(self, name: str) -> Optional[float]:
        """Get gauge value."""
        return self._gauges.get(name)

    def get_histogram(self, name: str) -> List[float]:
        """Get histogram values."""
        return list(self._histograms.get(name, []))

    def get_all(self) -> Dict[str, Any]:
        """Get all metrics."""
        return {
            "counters": dict(self._counters),
            "gauges": dict(self._gauges),
            "histograms": {k: list(v) for k, v in self._histograms.items()},
        }

    def reset(self) -> None:
        """Reset all metrics."""
        self._counters.clear()
        self._gauges.clear()
        self._histograms.clear()


# Global metrics
_metrics = Metrics.get_instance()


def record_metric(name: str, value: float = 1.0, metric_type: str = "counter") -> None:
    """Record a metric value."""
    if metric_type == "counter":
        _metrics.increment(name, int(value))
    elif metric_type == "gauge":
        _metrics.set_gauge(name, value)
    elif metric_type == "histogram":
        _metrics.record_histogram(name, value)
