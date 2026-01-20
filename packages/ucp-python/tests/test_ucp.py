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

    def test_edits_blocks(self):
        doc = ucp.parse("# Title\n\nParagraph")
        paragraph = next(
            (b for b in doc.blocks.values() if b.role == ucp.SemanticRole.PARAGRAPH), None
        )
        assert paragraph is not None

        doc.edit_block(paragraph.id, "Updated")
        assert doc.blocks[paragraph.id].content == "Updated"

    def test_moves_blocks(self):
        doc = ucp.parse("# Title\n\n## Section\n\nParagraph")
        blocks = list(doc.blocks.values())
        section = next((b for b in blocks if b.role == ucp.SemanticRole.HEADING2), None)
        paragraph = next(
            (b for b in blocks if b.role == ucp.SemanticRole.PARAGRAPH),
            None,
        )

        assert section is not None
        assert paragraph is not None

        doc.move_block(paragraph.id, doc.root)
        root_children = doc.blocks[doc.root].children
        assert paragraph.id in root_children

    def test_deletes_blocks(self):
        doc = ucp.parse("# Title\n\nParagraph")
        paragraph = next(
            (b for b in doc.blocks.values() if b.role == ucp.SemanticRole.PARAGRAPH), None
        )
        assert paragraph is not None

        doc.delete_block(paragraph.id)
        assert paragraph.id not in doc.blocks

    def test_manages_tags(self):
        doc = ucp.parse("# Title\n\nParagraph")
        paragraph = next(
            (b for b in doc.blocks.values() if b.role == ucp.SemanticRole.PARAGRAPH), None
        )
        assert paragraph is not None

        doc.add_tag(paragraph.id, "important")
        doc.add_tag(paragraph.id, "draft")
        assert doc.block_has_tag(paragraph.id, "important")
        assert sorted(doc.find_blocks_by_tag("important")) == [paragraph.id]

        doc.remove_tag(paragraph.id, "important")
        assert not doc.block_has_tag(paragraph.id, "important")

    def test_section_clear_and_restore(self):
        doc = ucp.parse("# Intro\n\n## Getting Started\n\nParagraph")

        h1_id = ucp.find_section_by_path(doc, "Intro")
        assert h1_id is not None

        original_count = doc.block_count()

        snapshot = ucp.clear_section_with_undo(doc, h1_id)
        assert snapshot.removed_ids

        # Add replacement content after clearing
        replacement_id = doc.add_block(
            h1_id,
            "Replacement",
            role=ucp.SemanticRole.PARAGRAPH,
        )
        assert replacement_id in doc.blocks

        restored_ids = ucp.restore_deleted_section(doc, snapshot.deleted_content)
        assert len(restored_ids) == len(snapshot.removed_ids)
        assert replacement_id not in doc.blocks
        assert doc.block_count() == original_count

        # Original subsection should exist again
        assert ucp.find_section_by_path(doc, "Intro > Getting Started") is not None


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

        assert "Document structure:" in desc
        assert "Blocks:" in desc
        assert "type=text" in desc
        assert 'content="Hello"' in desc


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


class TestEdgeCases:
    def test_handles_empty_markdown(self):
        doc = ucp.parse("")
        assert len(doc.blocks) == 1  # just root

    def test_handles_deeply_nested_headings(self):
        doc = ucp.parse("""# H1
## H2
### H3
#### H4
##### H5
###### H6

Paragraph under H6""")

        assert len(doc.blocks) == 8  # root + 6 headings + 1 paragraph

    def test_preserves_content_with_special_characters(self):
        content = "Text with \"quotes\" and 'apostrophes' and `backticks`"
        doc = ucp.parse(f"# Title\n\n{content}")
        blocks = list(doc.blocks.values())
        para = next((b for b in blocks if b.role == ucp.SemanticRole.PARAGRAPH), None)
        assert para is not None
        assert para.content == content

    def test_handles_multiple_code_blocks(self):
        doc = ucp.parse("""# Title

```js
const a = 1
```

```python
x = 2
```""")

        code_blocks = [b for b in doc.blocks.values() if b.type == ucp.ContentType.CODE]
        assert len(code_blocks) == 2

    def test_roundtrips_markdown_correctly(self):
        original = """# Hello

World

## Section

Content here
"""
        doc = ucp.parse(original)
        rendered = ucp.render(doc)

        assert "# Hello" in rendered
        assert "World" in rendered
        assert "## Section" in rendered
        assert "Content here" in rendered


class TestErrorHandling:
    def test_throws_when_adding_to_nonexistent_parent(self):
        doc = ucp.create()
        with pytest.raises(ValueError):
            doc.add_block("invalid_id", "content")

    def test_prompt_builder_requires_capability(self):
        with pytest.raises(ValueError, match="At least one capability"):
            ucp.prompt().build()

    def test_id_mapper_returns_none_for_unknown_ids(self):
        mapper = ucp.IdMapper()
        assert mapper.get_short("unknown") is None
        assert mapper.get_long(999) is None


class TestUclBuilderAdvanced:
    def test_builds_move_commands(self):
        assert ucp.ucl().move_to(1, 2).build() == "MOVE 1 TO 2"
        assert ucp.ucl().move_before(1, 2).build() == "MOVE 1 BEFORE 2"
        assert ucp.ucl().move_after(1, 2).build() == "MOVE 1 AFTER 2"

    def test_builds_link_commands(self):
        assert ucp.ucl().link(1, "references", 2).build() == "LINK 1 references 2"

    def test_chains_multiple_commands(self):
        result = ucp.ucl().edit(1, "a").append(1, "b").delete(2).build()

        assert "EDIT 1" in result
        assert "APPEND 1" in result
        assert "DELETE 2" in result

    def test_escapes_special_characters(self):
        result = ucp.ucl().edit(1, "line1\nline2").build()
        assert "\\n" in result

    def test_returns_commands_as_list(self):
        builder = ucp.ucl().edit(1, "a").edit(2, "b")
        assert len(builder.to_list()) == 2


class TestIdMapperAdvanced:
    def test_handles_ucl_with_multiple_id_references(self):
        doc = ucp.parse("# A\n\n## B\n\n### C")
        mapper = ucp.map_ids(doc)

        ucl_cmd = "MOVE 3 TO 2\nLINK 2 references 4"
        expanded = mapper.expand(ucl_cmd)

        assert "blk_" in expanded

    def test_provides_accurate_mappings_list(self):
        doc = ucp.parse("# Title\n\nPara")
        mapper = ucp.map_ids(doc)
        mappings = mapper.get_mappings()

        assert len(mappings) == 3  # root + heading + para
        assert mappings[0]["short"] == 1


class TestSectionOperations:
    def test_write_section_replaces_children_and_adjusts_headings(self):
        doc = ucp.parse("# Intro\n\n## Getting Started\n\nParagraph")
        section_id = ucp.find_section_by_path(doc, "Intro")
        assert section_id is not None

        original_children = doc.children(section_id)
        assert original_children

        result = ucp.write_section(
            doc,
            section_id,
            "# Replacement Heading\n\nParagraph content",
            base_heading_level=3,
        )

        assert result.success
        assert len(result.blocks_removed) >= len(original_children)

        new_children = doc.children(section_id)
        assert new_children
        new_heading = doc.blocks[new_children[0]]
        assert new_heading.metadata.semantic_role.value == "heading3"

    def test_write_section_requires_existing_section(self):
        doc = ucp.parse("# Intro")
        result = ucp.write_section(doc, "missing", "# Content")

        assert not result.success
        assert result.error.startswith("Section not found")

    def test_write_section_without_base_level_preserves_heading_role(self):
        doc = ucp.parse("# Intro")
        section_id = ucp.find_section_by_path(doc, "Intro")
        assert section_id is not None

        result = ucp.write_section(doc, section_id, "# Replacement")

        assert result.success
        new_children = doc.children(section_id)
        assert new_children
        replacement = doc.blocks[new_children[0]]
        assert replacement.metadata.semantic_role.value == "heading1"

    def test_clear_restore_and_write_section_roundtrip(self):
        doc = ucp.parse("# Intro\n\n## Getting Started\n\nParagraph")
        section_id = ucp.find_section_by_path(doc, "Intro")
        assert section_id is not None

        snapshot = ucp.clear_section_with_undo(doc, section_id)
        assert not doc.children(section_id)

        write_result = ucp.write_section(doc, section_id, "## Temporary\n\nTemp content")
        assert write_result.success
        assert write_result.blocks_added

        restored_ids = ucp.restore_deleted_section(doc, snapshot.deleted_content)
        assert restored_ids
        assert ucp.find_section_by_path(doc, "Intro > Getting Started") is not None

        for block_id in write_result.blocks_added:
            assert block_id not in doc.blocks


class TestSectionIntegrations:
    def test_traversal_and_context_after_section_updates(self):
        doc = ucp.parse(
            """# Intro

## Getting Started

Paragraph

## Details

Paragraph"""
        )

        section_id = ucp.find_section_by_path(doc, "Intro")
        assert section_id is not None

        ucp.write_section(
            doc,
            section_id,
            "# Plan\n\n## Steps\n\nDo things",
            base_heading_level=2,
        )

        engine = ucp.TraversalEngine()
        traversal_filter = ucp.TraversalFilter()
        result = engine.navigate(
            doc,
            doc.root,
            direction=ucp.NavigateDirection.BREADTH_FIRST,
            depth=3,
            filter=traversal_filter,
            output=ucp.TraversalOutput.STRUCTURE_WITH_PREVIEWS,
        )

        assert result.nodes
        assert any(node.semantic_role in ("heading2", "heading3") for node in result.nodes)

        manager = ucp.ContextManager("section-test")
        summary = manager.initialize_focus(doc, section_id, "Summarize intro")
        assert summary.blocks_added

        prompt_view = manager.render_for_prompt(doc)
        assert f"[{section_id}]" in prompt_view
        assert "heading" in prompt_view


class TestTraversalFilters:
    def test_role_inclusion_filter(self):
        doc = ucp.parse("""# Intro

## Getting Started

Paragraph""")

        section_id = ucp.find_section_by_path(doc, "Intro > Getting Started")
        assert section_id is not None

        engine = ucp.TraversalEngine()
        traversal_filter = ucp.TraversalFilter(include_roles=["heading2"])
        result = engine.navigate(
            doc,
            section_id,
            direction=ucp.NavigateDirection.BREADTH_FIRST,
            depth=3,
            filter=traversal_filter,
            output=ucp.TraversalOutput.STRUCTURE_AND_BLOCKS,
        )

        assert any(node.id == section_id for node in result.nodes)

    def test_tag_inclusion_filter(self):
        doc = ucp.parse("# Intro\n\nParagraph")
        paragraph = next(
            (b for b in doc.blocks.values() if b.role == ucp.SemanticRole.PARAGRAPH), None
        )
        assert paragraph is not None

        doc.add_tag(paragraph.id, "important")

        engine = ucp.TraversalEngine()
        traversal_filter = ucp.TraversalFilter(include_tags=["important"])
        result = engine.navigate(
            doc,
            paragraph.id,
            direction=ucp.NavigateDirection.BREADTH_FIRST,
            depth=2,
            filter=traversal_filter,
            output=ucp.TraversalOutput.STRUCTURE_WITH_PREVIEWS,
        )

        assert result.nodes
        assert all("important" in doc.blocks[node.id].metadata.tags for node in result.nodes)


class TestContextRendering:
    def test_render_for_prompt_defaults_to_block_role(self):
        doc = ucp.create()
        orphan_id = doc.add_block(doc.root, "Plain content")

        manager = ucp.ContextManager("ctx-default")
        manager.add_block(doc, orphan_id)

        prompt = manager.render_for_prompt(doc)
        assert "block: Plain content" in prompt

    def test_render_for_prompt_shows_compressed_marker(self):
        doc = ucp.parse("# Title\n\nParagraph")
        paragraph = next(
            (b for b in doc.blocks.values() if b.role == ucp.SemanticRole.PARAGRAPH), None
        )
        assert paragraph is not None

        manager = ucp.ContextManager("ctx-compress")
        manager.add_block(doc, paragraph.id)
        manager.compress(doc)

        prompt = manager.render_for_prompt(doc)
        assert "[compressed]" in prompt
