"""Pytest configuration and fixtures for UCP tests."""

import pytest
import sys
import os

# Add the built module to path for testing
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'python'))


@pytest.fixture
def empty_doc():
    """Create an empty document."""
    import ucp
    return ucp.create()


@pytest.fixture
def doc_with_title():
    """Create a document with a title."""
    import ucp
    return ucp.create("Test Document")


@pytest.fixture
def doc_with_blocks():
    """Create a document with several blocks."""
    import ucp
    doc = ucp.create("Test Document")
    root = doc.root_id

    # Add some blocks
    block1 = doc.add_block(root, "First paragraph", role="paragraph")
    block2 = doc.add_block(root, "Second paragraph", role="paragraph")
    block3 = doc.add_block(block1, "Nested block", role="note")

    return doc, root, block1, block2, block3
