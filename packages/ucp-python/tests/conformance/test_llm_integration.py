"""
Conformance tests for LLM integration utilities.

These tests verify IdMapper, PromptBuilder, and UclBuilder functionality.
"""

import pytest
from ucp import (
    parse,
    create,
)
from ucp.llm import (
    IdMapper,
    PromptBuilder,
    UclBuilder,
    prompt,
    map_ids,
    ucl,
)


class TestIdMapper:
    """Test IdMapper functionality."""

    def test_mapper_from_document(self):
        """IdMapper correctly maps document blocks."""
        doc = parse("# Title\n\nParagraph\n\n## Section")
        mapper = IdMapper(doc)

        assert mapper.block_count() == len(doc.blocks)

    def test_from_document_class_method(self):
        """IdMapper.from_document creates mapper."""
        doc = create()
        doc.add_block(doc.root_id, "Test")

        mapper = IdMapper.from_document(doc)

        assert mapper.block_count() == len(doc.blocks)

    def test_shorten_and_expand_roundtrip(self):
        """Shorten and expand are inverse operations."""
        doc = create()
        block_id = doc.add_block(doc.root_id, "Test content")
        mapper = IdMapper(doc)

        original = f"EDIT {block_id} SET text = 'hello'"
        shortened = mapper.shorten(original)
        expanded = mapper.expand(shortened)

        assert expanded == original

    def test_short_ids_are_integers(self):
        """Short IDs are simple integers."""
        doc = parse("# Test\n\nContent")
        mapper = IdMapper(doc)

        for block_id in doc.blocks:
            short_id = mapper.get_short(block_id)
            assert isinstance(short_id, int)

    def test_get_full_returns_original_id(self):
        """get_full returns the original block ID."""
        doc = create()
        block_id = doc.add_block(doc.root_id, "Test")
        mapper = IdMapper(doc)

        short_id = mapper.get_short(block_id)
        full_id = mapper.get_full(short_id)

        assert full_id == block_id

    def test_get_long_alias(self):
        """get_long is alias for get_full."""
        doc = create()
        block_id = doc.add_block(doc.root_id, "Test")
        mapper = IdMapper(doc)

        short_id = mapper.get_short(block_id)

        assert mapper.get_long(short_id) == mapper.get_full(short_id)

    def test_nonexistent_id_returns_none(self):
        """Getting nonexistent ID returns None."""
        doc = create()
        mapper = IdMapper(doc)

        assert mapper.get_short("blk_nonexistent") is None
        assert mapper.get_full(9999) is None

    def test_get_mapping_table(self):
        """get_mapping_table returns formatted table."""
        doc = create()
        doc.add_block(doc.root_id, "Block 1")
        doc.add_block(doc.root_id, "Block 2")
        mapper = IdMapper(doc)

        table = mapper.get_mapping_table()

        assert "| Short | Full ID |" in table
        assert "|-------|---------|" in table

    def test_get_mappings(self):
        """get_mappings returns list of dicts."""
        doc = create()
        doc.add_block(doc.root_id, "Test")
        mapper = IdMapper(doc)

        mappings = mapper.get_mappings()

        assert isinstance(mappings, list)
        assert all("short" in m and "long" in m for m in mappings)

    def test_describe_document(self):
        """describe generates document description."""
        doc = create()
        doc.add_block(doc.root_id, "First paragraph")
        mapper = IdMapper(doc)

        description = mapper.describe(doc)

        assert "Document Structure:" in description

    def test_map_ids_convenience(self):
        """map_ids convenience function works."""
        doc = create()
        mapper = map_ids(doc)

        assert isinstance(mapper, IdMapper)


class TestPromptBuilder:
    """Test PromptBuilder functionality."""

    def test_requires_at_least_one_capability(self):
        """Building without capabilities raises error."""
        with pytest.raises(ValueError, match="capability"):
            PromptBuilder().build()

    def test_edit_capability(self):
        """edit() enables EDIT capability."""
        prompt = PromptBuilder().edit().build()

        assert "EDIT" in prompt

    def test_append_capability(self):
        """append() enables APPEND capability."""
        prompt = PromptBuilder().append().build()

        assert "APPEND" in prompt

    def test_move_capability(self):
        """move() enables MOVE capability."""
        prompt = PromptBuilder().move().build()

        assert "MOVE" in prompt

    def test_delete_capability(self):
        """delete() enables DELETE capability."""
        prompt = PromptBuilder().delete().build()

        assert "DELETE" in prompt

    def test_link_capability(self):
        """link() enables LINK capability."""
        prompt = PromptBuilder().link().build()

        assert "LINK" in prompt

    def test_all_capabilities(self):
        """all() includes EDIT, APPEND, MOVE, DELETE, LINK."""
        prompt = PromptBuilder().all().build()

        assert "EDIT" in prompt
        assert "APPEND" in prompt
        assert "MOVE" in prompt
        assert "DELETE" in prompt
        assert "LINK" in prompt

    def test_with_short_ids(self):
        """with_short_ids adds instruction about short IDs."""
        prompt = PromptBuilder().edit().with_short_ids().build()

        assert "short" in prompt.lower()

    def test_with_constraints(self):
        """with_constraints adds rules to prompt."""
        prompt = PromptBuilder().edit().with_constraints(["max 100 words"]).build()

        assert "max 100 words" in prompt

    def test_with_rule(self):
        """with_rule adds single constraint."""
        prompt = PromptBuilder().edit().with_rule("be concise").build()

        assert "be concise" in prompt

    def test_with_context(self):
        """with_context adds context section."""
        prompt = PromptBuilder().edit().with_context("Editing a blog post").build()

        assert "Context" in prompt
        assert "blog post" in prompt

    def test_with_examples(self):
        """with_examples adds usage examples."""
        prompt = PromptBuilder().edit().with_examples().build()

        assert "Example:" in prompt

    def test_format_ucl(self):
        """format() can set ucl output."""
        prompt = PromptBuilder().edit().format("ucl").build()

        assert "UCL commands only" in prompt

    def test_format_json(self):
        """format() can set json output."""
        prompt = PromptBuilder().edit().format("json").build()

        assert "JSON" in prompt

    def test_fluent_chaining(self):
        """Methods can be chained fluently."""
        prompt = (PromptBuilder()
            .edit()
            .append()
            .move()
            .with_short_ids()
            .with_constraints(["rule1", "rule2"])
            .build())

        assert "EDIT" in prompt
        assert "APPEND" in prompt
        assert "MOVE" in prompt

    def test_prompt_convenience(self):
        """prompt() convenience function works."""
        builder = prompt()
        assert isinstance(builder, PromptBuilder)


class TestUclBuilder:
    """Test UclBuilder functionality."""

    def test_edit_command_format(self):
        """Edit command has correct format."""
        ucl = UclBuilder().edit("blk_123", "new content").build()

        assert "EDIT blk_123" in ucl
        assert "SET text" in ucl
        assert "new content" in ucl

    def test_edit_with_custom_path(self):
        """Edit command can use custom path."""
        ucl = UclBuilder().edit("blk_123", "value", path="metadata.label").build()

        assert "SET metadata.label" in ucl

    def test_edit_escapes_special_chars(self):
        """Edit command escapes special characters."""
        content = 'Line1\nLine2 "quoted"'
        ucl = UclBuilder().edit("blk_123", content).build()

        assert "\\n" in ucl
        assert '\\"' in ucl

    def test_append_command_format(self):
        """Append command has correct format."""
        ucl = UclBuilder().append("blk_123", "content text").build()

        assert "APPEND blk_123" in ucl
        assert "::" in ucl
        assert "content text" in ucl

    def test_append_with_content_type(self):
        """Append command supports content type."""
        ucl = UclBuilder().append("blk_123", "print(1)", content_type="code").build()

        assert "APPEND blk_123 code" in ucl

    def test_append_with_properties(self):
        """Append command supports properties."""
        ucl = UclBuilder().append("blk_123", "content", language="python").build()

        assert "WITH" in ucl
        assert "language=" in ucl

    def test_move_to_command(self):
        """MOVE TO command format."""
        ucl = UclBuilder().move_to("blk_1", "blk_2").build()

        assert "MOVE blk_1 TO blk_2" == ucl

    def test_move_to_with_index(self):
        """MOVE TO with INDEX."""
        ucl = UclBuilder().move_to("blk_1", "blk_2", index=0).build()

        assert "MOVE blk_1 TO blk_2 INDEX 0" == ucl

    def test_move_before_command(self):
        """MOVE BEFORE command format."""
        ucl = UclBuilder().move_before("blk_1", "blk_2").build()

        assert "MOVE blk_1 BEFORE blk_2" == ucl

    def test_move_after_command(self):
        """MOVE AFTER command format."""
        ucl = UclBuilder().move_after("blk_1", "blk_2").build()

        assert "MOVE blk_1 AFTER blk_2" == ucl

    def test_delete_command(self):
        """DELETE command format."""
        ucl = UclBuilder().delete("blk_123").build()

        assert "DELETE blk_123" == ucl

    def test_delete_with_cascade(self):
        """Delete with cascade includes CASCADE keyword."""
        ucl = UclBuilder().delete("blk_123", cascade=True).build()

        assert "DELETE blk_123 CASCADE" == ucl

    def test_link_command(self):
        """LINK command format."""
        ucl = UclBuilder().link("blk_1", "references", "blk_2").build()

        assert "LINK blk_1 references blk_2" == ucl

    def test_unlink_command(self):
        """UNLINK command format."""
        ucl = UclBuilder().unlink("blk_1", "references", "blk_2").build()

        assert "UNLINK blk_1 references blk_2" == ucl

    def test_prune_command(self):
        """PRUNE command format."""
        ucl = UclBuilder().prune().build()

        assert "PRUNE unreachable" == ucl

    def test_atomic_wraps_commands(self):
        """atomic() wraps commands in ATOMIC block."""
        ucl = UclBuilder().edit("1", "a").edit("2", "b").atomic().build()

        assert "ATOMIC {" in ucl
        assert "}" in ucl

    def test_command_chaining(self):
        """Commands can be chained."""
        ucl = UclBuilder().edit("1", "x").append("1", "y").delete("2").build()

        lines = ucl.strip().split("\n")
        assert len(lines) == 3

    def test_command_count(self):
        """command_count returns number of commands."""
        builder = UclBuilder().edit("1", "a").append("1", "b")

        assert builder.command_count() == 2

    def test_to_list(self):
        """to_list returns commands as list."""
        builder = UclBuilder().edit("1", "a").delete("2")

        commands = builder.to_list()

        assert isinstance(commands, list)
        assert len(commands) == 2

    def test_clear_commands(self):
        """clear() removes all commands."""
        builder = UclBuilder().edit("1", "a").append("1", "b")
        builder.clear()

        assert builder.command_count() == 0
        assert builder.build() == ""

    def test_ucl_convenience(self):
        """ucl() convenience function works."""
        builder = ucl()
        assert isinstance(builder, UclBuilder)
