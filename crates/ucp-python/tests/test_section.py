"""Tests for section management utilities."""

import pytest


class TestSectionUtilities:
    """Test section management functions."""

    def test_find_section_by_path(self):
        """Test finding a section by path."""
        import ucp

        # Create a document with headings
        doc = ucp.parse("""
# Introduction

Some intro text.

## Getting Started

Getting started content.

## Advanced Topics

Advanced content.
""")

        # Find single level section
        section_id = ucp.find_section_by_path(doc, "Introduction")
        assert section_id is not None

        # Find nested section
        section_id = ucp.find_section_by_path(doc, "Introduction > Getting Started")
        assert section_id is not None

        # Non-existent section
        section_id = ucp.find_section_by_path(doc, "Missing Section")
        assert section_id is None

    def test_get_all_sections(self):
        """Test getting all sections from a document."""
        import ucp

        doc = ucp.parse("""
# Title

## Section 1

### Subsection 1.1

## Section 2
""")

        sections = ucp.get_all_sections(doc)
        assert len(sections) >= 4  # Title, Section 1, Subsection 1.1, Section 2

        # Check that we have different heading levels
        levels = [level for _, level in sections]
        assert 1 in levels
        assert 2 in levels
        assert 3 in levels

    def test_get_section_depth(self):
        """Test getting section depth."""
        import ucp

        doc = ucp.parse("""
# Title

## Section 1

### Subsection 1.1
""")

        # Find the title section
        title_id = ucp.find_section_by_path(doc, "Title")
        assert title_id is not None

        depth = ucp.get_section_depth(doc, title_id)
        assert depth is not None
        assert depth >= 1

    def test_clear_section_with_undo(self):
        """Test clearing a section with undo support."""
        import ucp

        doc = ucp.parse("""
# Title

## Section to Clear

Content to be cleared.

More content.
""")

        # Find the section
        section_id = ucp.find_section_by_path(doc, "Title > Section to Clear")
        assert section_id is not None

        # Get initial block count
        initial_count = doc.block_count

        # Clear the section
        result = ucp.clear_section_with_undo(doc, section_id)

        # Check result
        assert len(result.removed_ids) > 0
        assert result.deleted_content is not None
        assert result.deleted_content.block_count > 0

        # Document should have fewer blocks
        assert doc.block_count < initial_count

    def test_restore_deleted_section(self):
        """Test restoring deleted section content."""
        import ucp

        doc = ucp.parse("""
# Title

## Section to Clear

Content to be cleared.
""")

        section_id = ucp.find_section_by_path(doc, "Title > Section to Clear")
        initial_count = doc.block_count

        # Clear and save deleted content
        result = ucp.clear_section_with_undo(doc, section_id)
        deleted = result.deleted_content

        # Restore
        restored_ids = ucp.restore_deleted_section(doc, deleted)

        # Should have restored blocks
        assert len(restored_ids) > 0
        assert doc.block_count == initial_count

    def test_deleted_content_serialization(self):
        """Test serializing and deserializing deleted content."""
        import ucp

        doc = ucp.parse("""
# Title

## Section

Content here.
""")

        section_id = ucp.find_section_by_path(doc, "Title > Section")
        result = ucp.clear_section_with_undo(doc, section_id)

        # Serialize to JSON
        json_str = result.deleted_content.to_json()
        assert json_str is not None
        assert len(json_str) > 0

        # Deserialize
        restored = ucp.DeletedContent.from_json(json_str)
        assert restored.block_count == result.deleted_content.block_count


class TestHtmlTranslator:
    """Test HTML parsing functionality."""

    def test_parse_simple_html(self):
        """Test parsing simple HTML."""
        import ucp

        html = """
        <html><body>
            <h1>Hello World</h1>
            <p>This is a paragraph.</p>
        </body></html>
        """

        doc = ucp.parse_html(html)
        assert doc is not None
        assert doc.block_count > 1

    def test_parse_nested_html(self):
        """Test parsing nested HTML structure."""
        import ucp

        html = """
        <html><body>
            <h1>Main Title</h1>
            <p>Intro paragraph</p>
            <h2>Section 1</h2>
            <p>Section 1 content</p>
            <h2>Section 2</h2>
            <p>Section 2 content</p>
        </body></html>
        """

        doc = ucp.parse_html(html)
        assert doc.block_count >= 5

    def test_parse_html_with_code(self):
        """Test parsing HTML with code blocks."""
        import ucp

        html = """
        <html><body>
            <pre><code class="language-python">print("Hello")</code></pre>
        </body></html>
        """

        doc = ucp.parse_html(html)
        assert doc.block_count >= 2

    def test_parse_html_with_lists(self):
        """Test parsing HTML with lists."""
        import ucp

        html = """
        <html><body>
            <ul>
                <li>Item 1</li>
                <li>Item 2</li>
                <li>Item 3</li>
            </ul>
        </body></html>
        """

        doc = ucp.parse_html(html)
        assert doc.block_count >= 2

    def test_parse_empty_html(self):
        """Test parsing empty HTML."""
        import ucp

        html = "<html><body></body></html>"
        doc = ucp.parse_html(html)
        assert doc.block_count == 1  # Just root


class TestObservability:
    """Test observability utilities."""

    def test_audit_entry_creation(self):
        """Test creating audit entries."""
        import ucp

        entry = ucp.AuditEntry("EDIT", "doc_123")
        assert entry.operation == "EDIT"
        assert entry.document_id == "doc_123"
        assert entry.success is True

    def test_audit_entry_with_user(self):
        """Test audit entry with user ID."""
        import ucp

        entry = ucp.AuditEntry("CREATE", "doc_456")
        entry = entry.with_user("user_789")
        assert entry.user_id == "user_789"

    def test_audit_entry_failed(self):
        """Test marking audit entry as failed."""
        import ucp

        entry = ucp.AuditEntry("DELETE", "doc_000")
        entry = entry.failed()
        assert entry.success is False

    def test_audit_entry_to_dict(self):
        """Test converting audit entry to dict."""
        import ucp

        entry = ucp.AuditEntry("UPDATE", "doc_111")
        d = entry.to_dict()
        assert "operation" in d
        assert "document_id" in d
        assert "timestamp" in d

    def test_metrics_recorder(self):
        """Test metrics recorder."""
        import ucp

        metrics = ucp.MetricsRecorder()
        assert metrics.operations_total == 0

        metrics.record_operation(True)
        metrics.record_operation(False)
        assert metrics.operations_total == 2
        assert metrics.operations_failed == 1

    def test_metrics_recorder_blocks(self):
        """Test recording block operations."""
        import ucp

        metrics = ucp.MetricsRecorder()
        metrics.record_block_created()
        metrics.record_block_created()
        metrics.record_block_deleted()

        assert metrics.blocks_created == 2
        assert metrics.blocks_deleted == 1

    def test_metrics_to_dict(self):
        """Test converting metrics to dict."""
        import ucp

        metrics = ucp.MetricsRecorder()
        metrics.record_operation(True)
        metrics.record_snapshot()

        d = metrics.to_dict()
        assert "operations_total" in d
        assert "snapshots_created" in d
        assert d["operations_total"] == 1
        assert d["snapshots_created"] == 1

    def test_ucp_event_creation(self):
        """Test creating UCP events."""
        import ucp

        event = ucp.UcpEvent.document_created("doc_123")
        assert event.event_type == "document_created"
        assert event.document_id == "doc_123"
        assert event.timestamp is not None

    def test_ucp_event_block_added(self):
        """Test block added event."""
        import ucp

        event = ucp.UcpEvent.block_added("doc_1", "blk_1", "root", "text")
        assert event.event_type == "block_added"

    def test_ucp_event_block_deleted(self):
        """Test block deleted event."""
        import ucp

        event = ucp.UcpEvent.block_deleted("doc_1", "blk_1", cascade=True)
        assert event.event_type == "block_deleted"

    def test_event_bus(self):
        """Test event bus creation."""
        import ucp

        bus = ucp.EventBus()
        assert bus.subscriber_count == 0
