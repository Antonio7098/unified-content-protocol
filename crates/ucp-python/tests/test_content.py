"""Tests for Content types."""



class TestContentCreation:
    """Test content type creation."""

    def test_text_content(self):
        """Test creating text content."""
        import ucp
        content = ucp.Content.text("Hello, World!")

        assert content.type_tag == "text"
        assert content.is_empty is False
        assert content.as_text() == "Hello, World!"

    def test_markdown_content(self):
        """Test creating markdown content."""
        import ucp
        content = ucp.Content.markdown("# Heading")

        assert content.type_tag == "text"
        assert content.as_text() == "# Heading"

    def test_code_content(self):
        """Test creating code content."""
        import ucp
        content = ucp.Content.code("python", "print('hello')")

        assert content.type_tag == "code"
        lang, source = content.as_code()
        assert lang == "python"
        assert source == "print('hello')"

    def test_table_content(self):
        """Test creating table content."""
        import ucp
        rows = [["A", "B"], ["1", "2"], ["3", "4"]]
        content = ucp.Content.table(rows)

        assert content.type_tag == "table"

    def test_json_content(self):
        """Test creating JSON content."""
        import ucp
        data = {"key": "value", "count": 42}
        content = ucp.Content.json(data)

        assert content.type_tag == "json"
        result = content.as_json()
        assert result["key"] == "value"
        assert result["count"] == 42

    def test_math_content(self):
        """Test creating math content."""
        import ucp
        content = ucp.Content.math(r"E = mc^2", display_mode=True)

        assert content.type_tag == "math"
        expr, display, fmt = content.as_math()
        assert expr == r"E = mc^2"
        assert display is True
        assert fmt == "latex"

    def test_math_content_formats(self):
        """Test math content with different formats."""
        import ucp
        # LaTeX (default)
        content = ucp.Content.math(r"\frac{a}{b}")
        _, _, fmt = content.as_math()
        assert fmt == "latex"

        # MathML
        content = ucp.Content.math("<math><mi>x</mi></math>", format="mathml")
        _, _, fmt = content.as_math()
        assert fmt == "mathml"

        # AsciiMath
        content = ucp.Content.math("sum_(i=1)^n i^3", format="asciimath")
        _, _, fmt = content.as_math()
        assert fmt == "asciimath"

    def test_media_content(self):
        """Test creating media content."""
        import ucp
        content = ucp.Content.media(
            "image",
            "https://example.com/image.png",
            alt_text="Example image",
            width=800,
            height=600
        )

        assert content.type_tag == "media"
        media_type, url, alt = content.as_media()
        assert media_type == "image"
        assert url == "https://example.com/image.png"
        assert alt == "Example image"

    def test_media_types(self):
        """Test different media types."""
        import ucp
        for media_type in ["image", "audio", "video", "document"]:
            content = ucp.Content.media(media_type, "https://example.com/file")
            result_type, _, _ = content.as_media()
            assert result_type == media_type

    def test_binary_content(self):
        """Test creating binary content."""
        import ucp
        data = b"\x00\x01\x02\x03\x04"
        content = ucp.Content.binary("application/octet-stream", data)

        assert content.type_tag == "binary"
        mime, result_data = content.as_binary()
        assert mime == "application/octet-stream"
        assert result_data == data

    def test_composite_content(self):
        """Test creating composite content."""
        import ucp
        content = ucp.Content.composite("horizontal")

        assert content.type_tag == "composite"

    def test_composite_layouts(self):
        """Test different composite layouts."""
        import ucp
        for layout in ["vertical", "horizontal", "tabs", "grid:3"]:
            content = ucp.Content.composite(layout)
            assert content.type_tag == "composite"

    def test_table_as_table(self):
        """Test getting table data from table content."""
        import ucp
        rows = [["Name", "Age"], ["Alice", "30"], ["Bob", "25"]]
        content = ucp.Content.table(rows)

        columns, data = content.as_table()
        assert len(columns) == 2
        assert len(data) == 3

    def test_empty_content(self):
        """Test checking empty content."""
        import ucp
        content = ucp.Content.text("")
        assert content.is_empty is True

        content = ucp.Content.text("something")
        assert content.is_empty is False

    def test_content_size(self):
        """Test content size in bytes."""
        import ucp
        content = ucp.Content.text("Hello")
        assert content.size_bytes > 0

    def test_content_to_dict(self):
        """Test converting content to dictionary."""
        import ucp
        content = ucp.Content.text("Test")
        d = content.to_dict()

        assert "type" in d
        assert "text" in d
        assert d["text"] == "Test"


class TestContentInDocument:
    """Test content within documents."""

    def test_add_text_content(self, empty_doc):
        """Test adding text content to document."""
        import ucp
        root = empty_doc.root_id
        content = ucp.Content.text("Plain text")
        block_id = empty_doc.add_block_with_content(root, content)

        block = empty_doc.get_block(block_id)
        assert block.content_type == "text"

    def test_add_code_content(self, empty_doc):
        """Test adding code content to document."""
        import ucp
        root = empty_doc.root_id
        content = ucp.Content.code("rust", "fn main() {}")
        block_id = empty_doc.add_block_with_content(root, content)

        block = empty_doc.get_block(block_id)
        assert block.content_type == "code"

    def test_edit_with_content(self, empty_doc):
        """Test editing block with new content type."""
        import ucp
        root = empty_doc.root_id
        block_id = empty_doc.add_block(root, "Original text")

        new_content = ucp.Content.markdown("**Bold text**")
        empty_doc.edit_block_content(block_id, new_content)

        block = empty_doc.get_block(block_id)
        assert "Bold text" in block.get_text()
