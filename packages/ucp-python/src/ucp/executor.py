"""
UCL Executor - Parse and execute UCL commands.

This module provides UCL parsing and execution against Document instances.
"""

from __future__ import annotations

import re
from dataclasses import dataclass, field
from typing import Dict, List, Optional

from .document import Document
from .observability import (
    EventType,
    UclEvent,
    _metrics,
    _tracer,
    emit_event,
)
from .types import ContentType, EdgeType, SemanticRole


# =============================================================================
# EXCEPTIONS
# =============================================================================


class UclParseError(Exception):
    """Raised when UCL parsing fails."""
    def __init__(self, message: str, line: Optional[int] = None):
        self.line = line
        super().__init__(f"Line {line}: {message}" if line else message)


class UclExecutionError(Exception):
    """Raised when UCL execution fails."""
    def __init__(self, message: str, command: Optional[str] = None):
        self.command = command
        super().__init__(f"[{command}] {message}" if command else message)


# =============================================================================
# COMMAND TYPES
# =============================================================================


@dataclass
class EditCommand:
    """EDIT command."""
    block_id: str
    path: str
    value: str
    operator: str = "SET"


@dataclass
class AppendCommand:
    """APPEND command."""
    parent_id: str
    content_type: str
    content: str
    properties: Dict[str, str] = field(default_factory=dict)
    index: Optional[int] = None


@dataclass
class MoveCommand:
    """MOVE command."""
    block_id: str
    mode: str  # TO, BEFORE, AFTER
    target_id: str
    index: Optional[int] = None


@dataclass
class DeleteCommand:
    """DELETE command."""
    block_id: str
    cascade: bool = False


@dataclass
class LinkCommand:
    """LINK command."""
    source_id: str
    edge_type: str
    target_id: str


@dataclass
class UnlinkCommand:
    """UNLINK command."""
    source_id: str
    edge_type: str
    target_id: str


@dataclass
class PruneCommand:
    """PRUNE command."""
    condition: str = "unreachable"


UclCommand = EditCommand | AppendCommand | MoveCommand | DeleteCommand | LinkCommand | UnlinkCommand | PruneCommand


# =============================================================================
# PARSER
# =============================================================================


class UclParser:
    """Parser for UCL commands."""

    # Regex patterns
    EDIT_PATTERN = re.compile(
        r'^EDIT\s+(?P<block>\S+)\s+SET\s+(?P<path>\S+)\s*=\s*"(?P<value>(?:\\.|[^"])*)"$',
        re.IGNORECASE,
    )
    APPEND_PATTERN = re.compile(
        r'^APPEND\s+(?P<parent>\S+)\s+(?P<ctype>\w+)(?P<props>\s+WITH\s+[^:]+)?\s*::\s*(?P<content>.+)$',
        re.IGNORECASE,
    )
    MOVE_PATTERN = re.compile(
        r'^MOVE\s+(?P<block>\S+)\s+(?P<mode>TO|BEFORE|AFTER)\s+(?P<target>\S+)(?:\s+INDEX\s+(?P<index>\d+))?$',
        re.IGNORECASE,
    )
    DELETE_PATTERN = re.compile(
        r'^DELETE\s+(?P<block>\S+)(?P<cascade>\s+CASCADE)?$',
        re.IGNORECASE,
    )
    LINK_PATTERN = re.compile(
        r'^LINK\s+(?P<source>\S+)\s+(?P<edge_type>\w+)\s+(?P<target>\S+)$',
        re.IGNORECASE,
    )
    UNLINK_PATTERN = re.compile(
        r'^UNLINK\s+(?P<source>\S+)\s+(?P<edge_type>\w+)\s+(?P<target>\S+)$',
        re.IGNORECASE,
    )
    PRUNE_PATTERN = re.compile(
        r'^PRUNE(?:\s+(?P<condition>\w+))?$',
        re.IGNORECASE,
    )
    PROP_PATTERN = re.compile(r'(\w+)\s*=\s*"([^"]*)"')

    def parse(self, ucl: str) -> List[UclCommand]:
        """Parse UCL string into commands."""
        lines = self._extract_lines(ucl)
        commands: List[UclCommand] = []
        
        for i, line in enumerate(lines, 1):
            try:
                cmd = self._parse_line(line)
                if cmd:
                    commands.append(cmd)
            except Exception as e:
                raise UclParseError(str(e), line=i)
        
        return commands

    def _extract_lines(self, ucl: str) -> List[str]:
        """Extract command lines, handling ATOMIC blocks and comments."""
        lines: List[str] = []
        atomic_depth = 0
        
        for raw in ucl.splitlines():
            stripped = raw.strip()
            
            # Skip empty lines and comments
            if not stripped or stripped.startswith("#"):
                continue
            
            # Handle ATOMIC blocks
            if stripped.upper() == "ATOMIC {":
                atomic_depth += 1
                continue
            if stripped == "}" and atomic_depth > 0:
                atomic_depth -= 1
                continue
            
            lines.append(stripped)
        
        return lines

    def _parse_line(self, line: str) -> Optional[UclCommand]:
        """Parse a single command line."""
        upper = line.upper()
        
        if upper.startswith("EDIT "):
            return self._parse_edit(line)
        elif upper.startswith("APPEND "):
            return self._parse_append(line)
        elif upper.startswith("MOVE "):
            return self._parse_move(line)
        elif upper.startswith("DELETE "):
            return self._parse_delete(line)
        elif upper.startswith("LINK "):
            return self._parse_link(line)
        elif upper.startswith("UNLINK "):
            return self._parse_unlink(line)
        elif upper.startswith("PRUNE"):
            return self._parse_prune(line)
        else:
            raise UclParseError(f"Unknown command: {line}")

    def _parse_edit(self, line: str) -> EditCommand:
        match = self.EDIT_PATTERN.match(line)
        if not match:
            raise UclParseError(f"Malformed EDIT command: {line}")
        return EditCommand(
            block_id=match.group("block"),
            path=match.group("path"),
            value=self._unescape(match.group("value")),
        )

    def _parse_append(self, line: str) -> AppendCommand:
        match = self.APPEND_PATTERN.match(line)
        if not match:
            raise UclParseError(f"Malformed APPEND command: {line}")
        
        props_str = match.group("props") or ""
        properties = dict(self.PROP_PATTERN.findall(props_str))
        
        return AppendCommand(
            parent_id=match.group("parent"),
            content_type=match.group("ctype").lower(),
            content=match.group("content"),
            properties={k.lower(): self._unescape(v) for k, v in properties.items()},
        )

    def _parse_move(self, line: str) -> MoveCommand:
        match = self.MOVE_PATTERN.match(line)
        if not match:
            raise UclParseError(f"Malformed MOVE command: {line}")
        
        index_str = match.group("index")
        return MoveCommand(
            block_id=match.group("block"),
            mode=match.group("mode").upper(),
            target_id=match.group("target"),
            index=int(index_str) if index_str else None,
        )

    def _parse_delete(self, line: str) -> DeleteCommand:
        match = self.DELETE_PATTERN.match(line)
        if not match:
            raise UclParseError(f"Malformed DELETE command: {line}")
        return DeleteCommand(
            block_id=match.group("block"),
            cascade=bool(match.group("cascade")),
        )

    def _parse_link(self, line: str) -> LinkCommand:
        match = self.LINK_PATTERN.match(line)
        if not match:
            raise UclParseError(f"Malformed LINK command: {line}")
        return LinkCommand(
            source_id=match.group("source"),
            edge_type=match.group("edge_type").lower(),
            target_id=match.group("target"),
        )

    def _parse_unlink(self, line: str) -> UnlinkCommand:
        match = self.UNLINK_PATTERN.match(line)
        if not match:
            raise UclParseError(f"Malformed UNLINK command: {line}")
        return UnlinkCommand(
            source_id=match.group("source"),
            edge_type=match.group("edge_type").lower(),
            target_id=match.group("target"),
        )

    def _parse_prune(self, line: str) -> PruneCommand:
        match = self.PRUNE_PATTERN.match(line)
        if not match:
            raise UclParseError(f"Malformed PRUNE command: {line}")
        condition = match.group("condition") or "unreachable"
        return PruneCommand(condition=condition.lower())

    def _unescape(self, value: str) -> str:
        """Unescape string value."""
        return value.encode().decode("unicode_escape")


# =============================================================================
# EXECUTOR
# =============================================================================


@dataclass
class ExecutionResult:
    """Result of executing a UCL command."""
    success: bool
    command_type: str
    affected_blocks: List[str] = field(default_factory=list)
    error: Optional[str] = None


class UclExecutor:
    """Executor for UCL commands against a Document."""

    def __init__(self, doc: Document) -> None:
        self.doc = doc
        self._parser = UclParser()

    def execute(self, ucl: str) -> List[ExecutionResult]:
        """Execute UCL commands and return results."""
        with _tracer.span("ucl.execute", ucl_length=len(ucl)):
            try:
                commands = self._parser.parse(ucl)
                
                emit_event(UclEvent(
                    event_type=EventType.UCL_PARSED.value,
                    command=ucl,
                    data={"command_count": len(commands)},
                ))
                
            except UclParseError as e:
                emit_event(UclEvent(
                    event_type=EventType.UCL_ERROR.value,
                    command=ucl,
                    success=False,
                    error_message=str(e),
                ))
                raise

            results: List[ExecutionResult] = []
            for cmd in commands:
                result = self._execute_command(cmd)
                results.append(result)
                
                if not result.success:
                    break  # Stop on first error

            _metrics.increment("ucl.commands_executed", len(results))
            
            emit_event(UclEvent(
                event_type=EventType.UCL_EXECUTED.value,
                command=ucl,
                success=all(r.success for r in results),
                affected_blocks=[b for r in results for b in r.affected_blocks],
            ))
            
            return results

    def _execute_command(self, cmd: UclCommand) -> ExecutionResult:
        """Execute a single command."""
        try:
            if isinstance(cmd, EditCommand):
                return self._exec_edit(cmd)
            elif isinstance(cmd, AppendCommand):
                return self._exec_append(cmd)
            elif isinstance(cmd, MoveCommand):
                return self._exec_move(cmd)
            elif isinstance(cmd, DeleteCommand):
                return self._exec_delete(cmd)
            elif isinstance(cmd, LinkCommand):
                return self._exec_link(cmd)
            elif isinstance(cmd, UnlinkCommand):
                return self._exec_unlink(cmd)
            elif isinstance(cmd, PruneCommand):
                return self._exec_prune(cmd)
            else:
                return ExecutionResult(
                    success=False,
                    command_type="unknown",
                    error=f"Unknown command type: {type(cmd).__name__}",
                )
        except Exception as e:
            return ExecutionResult(
                success=False,
                command_type=type(cmd).__name__,
                error=str(e),
            )

    def _resolve_id(self, raw: str) -> str:
        """Resolve a block ID (handles short IDs if needed)."""
        if raw in self.doc.blocks:
            return raw
        if raw.isdigit():
            raise UclExecutionError(
                f"Unknown block ID '{raw}'. Did you forget to expand short IDs?",
                command="resolve_id",
            )
        raise UclExecutionError(f"Unknown block ID '{raw}'")

    def _exec_edit(self, cmd: EditCommand) -> ExecutionResult:
        block_id = self._resolve_id(cmd.block_id)
        
        # Currently only support text path
        if cmd.path.lower() == "text":
            self.doc.edit_block(block_id, cmd.value)
        else:
            raise UclExecutionError(f"Unsupported edit path: {cmd.path}")
        
        return ExecutionResult(
            success=True,
            command_type="edit",
            affected_blocks=[block_id],
        )

    def _exec_append(self, cmd: AppendCommand) -> ExecutionResult:
        parent_id = self._resolve_id(cmd.parent_id)
        
        # Map content type
        try:
            content_type = ContentType(cmd.content_type)
        except ValueError:
            content_type = ContentType.TEXT
        
        # Get role from properties
        role = None
        if "role" in cmd.properties:
            try:
                role = SemanticRole(cmd.properties["role"])
            except ValueError:
                pass
        
        new_id = self.doc.add_block(
            parent_id,
            cmd.content,
            content_type=content_type,
            role=role,
            label=cmd.properties.get("label"),
            index=cmd.index,
        )
        
        return ExecutionResult(
            success=True,
            command_type="append",
            affected_blocks=[new_id],
        )

    def _exec_move(self, cmd: MoveCommand) -> ExecutionResult:
        block_id = self._resolve_id(cmd.block_id)
        target_id = self._resolve_id(cmd.target_id)
        
        if cmd.mode == "TO":
            self.doc.move_block(block_id, target_id, index=cmd.index)
        else:
            # BEFORE or AFTER - need to find target's parent
            parent_id = self.doc.parent(target_id)
            if parent_id is None:
                raise UclExecutionError(f"Target {target_id} has no parent")
            
            siblings = self.doc.children(parent_id)
            try:
                sibling_index = siblings.index(target_id)
            except ValueError:
                raise UclExecutionError(f"Target {target_id} not found in parent's children")
            
            if cmd.mode == "AFTER":
                sibling_index += 1
            
            self.doc.move_block(block_id, parent_id, index=sibling_index)
        
        return ExecutionResult(
            success=True,
            command_type="move",
            affected_blocks=[block_id],
        )

    def _exec_delete(self, cmd: DeleteCommand) -> ExecutionResult:
        block_id = self._resolve_id(cmd.block_id)
        self.doc.delete_block(block_id, cascade=cmd.cascade)
        
        return ExecutionResult(
            success=True,
            command_type="delete",
            affected_blocks=[block_id],
        )

    def _exec_link(self, cmd: LinkCommand) -> ExecutionResult:
        source_id = self._resolve_id(cmd.source_id)
        target_id = self._resolve_id(cmd.target_id)
        
        edge_type = EdgeType.from_str(cmd.edge_type)
        if edge_type is None:
            raise UclExecutionError(f"Unknown edge type: {cmd.edge_type}")
        
        self.doc.add_edge(source_id, edge_type, target_id)
        
        return ExecutionResult(
            success=True,
            command_type="link",
            affected_blocks=[source_id, target_id],
        )

    def _exec_unlink(self, cmd: UnlinkCommand) -> ExecutionResult:
        source_id = self._resolve_id(cmd.source_id)
        target_id = self._resolve_id(cmd.target_id)
        
        edge_type = EdgeType.from_str(cmd.edge_type)
        if edge_type is None:
            raise UclExecutionError(f"Unknown edge type: {cmd.edge_type}")
        
        self.doc.remove_edge(source_id, edge_type, target_id)
        
        return ExecutionResult(
            success=True,
            command_type="unlink",
            affected_blocks=[source_id, target_id],
        )

    def _exec_prune(self, cmd: PruneCommand) -> ExecutionResult:
        if cmd.condition == "unreachable":
            pruned = self.doc.prune_orphans()
            return ExecutionResult(
                success=True,
                command_type="prune",
                affected_blocks=pruned,
            )
        else:
            raise UclExecutionError(f"Unknown prune condition: {cmd.condition}")


# =============================================================================
# CONVENIENCE FUNCTION
# =============================================================================


class ExecutionSummary:
    """Aggregated result of executing UCL commands."""

    def __init__(self, results: List[ExecutionResult]):
        self._results = results

    @property
    def success(self) -> bool:
        return all(r.success for r in self._results)

    @property
    def results(self) -> List[ExecutionResult]:
        return self._results

    @property
    def affected_blocks(self) -> List[str]:
        blocks: List[str] = []
        for result in self._results:
            for block_id in result.affected_blocks:
                if block_id not in blocks:
                    blocks.append(block_id)
        return blocks

    def __len__(self) -> int:
        return len(self._results)


def execute_ucl(doc: Document, ucl: str) -> ExecutionSummary:
    """Execute UCL commands on a document.
    
    Args:
        doc: The document to modify
        ucl: UCL command string
        
    Returns:
        List of affected block IDs
        
    Raises:
        UclParseError: If parsing fails
        UclExecutionError: If execution fails
    """
    executor = UclExecutor(doc)
    results = executor.execute(ucl)

    summary = ExecutionSummary(results)
    if not summary.success:
        for result in results:
            if not result.success:
                raise UclExecutionError(
                    result.error or "Unknown error",
                    command=result.command_type,
                )

    return summary
