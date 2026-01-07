"""Tests for UCP Python SDK."""

import pytest
import ucp


class TestDocumentOperations:
    def test_creates_empty_document(self):
        doc = ucp.create()
        assert len(doc.blocks) == 1  # root block
        assert doc.root in doc.blocks

    def test_adds_blocks_to_document(self):
        doc = ucp.create()
        block_id = doc.add_block(doc.root, "Hello", role=ucp.SemanticRole.PARAGRAPH)

        assert len(doc.blocks) == 2
        assert doc.blocks[block_id].content == "Hello"

    def test_parses_markdown(self):
        doc = ucp.parse("# Hello\n\nWorld")

        assert len(doc.blocks) == 3  # root + heading + paragraph

    def test_renders_markdown(self):
        doc = ucp.parse("# Hello\n\nWorld")
        md = ucp.render(doc)

        assert "# Hello" in md
        assert "World" in md

    def test_handles_code_blocks(self):
        doc = ucp.parse('# Title\n\n```js\nconsole.log("hi")\n```')

        blocks = list(doc.blocks.values())
        code_block = next((b for b in blocks if b.type == ucp.ContentType.CODE), None)

        assert code_block is not None
        assert "console.log" in code_block.content


class TestPromptBuilder:
    def test_builds_prompt_with_capabilities(self):
        prompt = ucp.prompt().edit().append().build()

        assert "EDIT" in prompt
        assert "APPEND" in prompt
        assert "MOVE" not in prompt

    def test_enables_all_capabilities(self):
        prompt = ucp.prompt().all().build()

        assert "EDIT" in prompt
        assert "APPEND" in prompt
        assert "MOVE" in prompt
        assert "DELETE" in prompt

    def test_adds_short_id_instructions(self):
        prompt = ucp.prompt().edit().with_short_ids().build()

        assert "short numbers" in prompt

    def test_adds_custom_rules(self):
        prompt = ucp.prompt().edit().with_rule("Be concise").build()

        assert "Be concise" in prompt

    def test_throws_without_capabilities(self):
        with pytest.raises(ValueError):
            ucp.prompt().build()


class TestIdMapper:
    def test_creates_mappings_from_document(self):
        doc = ucp.parse("# Hello\n\nWorld")
        mapper = ucp.map_ids(doc)

        assert mapper.get_short(doc.root) == 1

    def test_shortens_text_with_block_ids(self):
        doc = ucp.parse("# Hello")
        mapper = ucp.map_ids(doc)

        blocks = list(doc.blocks.values())
        heading = next((b for b in blocks if b.role == ucp.SemanticRole.HEADING1), None)

        text = f"Edit block {heading.id}"
        short = mapper.shorten(text)

        assert "blk_" not in short

    def test_expands_ucl_commands(self):
        doc = ucp.parse("# Hello")
        mapper = ucp.map_ids(doc)

        expanded = mapper.expand('EDIT 2 SET text = "hi"')

        assert "blk_" in expanded

    def test_generates_document_description(self):
        doc = ucp.parse("# Hello\n\nWorld")
        mapper = ucp.map_ids(doc)

        desc = mapper.describe(doc)

        assert "[2]" in desc
        assert "heading1" in desc


class TestUclBuilder:
    def test_builds_edit_commands(self):
        result = ucp.ucl().edit(1, "hello").build()

        assert result == 'EDIT 1 SET text = "hello"'

    def test_builds_append_commands(self):
        result = ucp.ucl().append(1, "content").build()

        assert "APPEND 1 text :: content" in result

    def test_builds_delete_commands(self):
        result = ucp.ucl().delete(1, cascade=True).build()

        assert result == "DELETE 1 CASCADE"

    def test_wraps_in_atomic_block(self):
        result = ucp.ucl().edit(1, "a").edit(2, "b").atomic().build()

        assert "ATOMIC {" in result
        assert "EDIT 1" in result
        assert "EDIT 2" in result
