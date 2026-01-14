"""
Conformance tests for SemanticRole enum.

These tests verify that all expected semantic roles are defined and can be
used with blocks and documents.
"""

from ucp import SemanticRole, Block, create


class TestHeadingRoles:
    """Test heading semantic roles."""

    def test_heading1_exists(self):
        """HEADING1 role exists with correct value."""
        assert SemanticRole.HEADING1.value == "heading1"

    def test_heading2_exists(self):
        """HEADING2 role exists with correct value."""
        assert SemanticRole.HEADING2.value == "heading2"

    def test_heading3_exists(self):
        """HEADING3 role exists with correct value."""
        assert SemanticRole.HEADING3.value == "heading3"

    def test_heading4_exists(self):
        """HEADING4 role exists with correct value."""
        assert SemanticRole.HEADING4.value == "heading4"

    def test_heading5_exists(self):
        """HEADING5 role exists with correct value."""
        assert SemanticRole.HEADING5.value == "heading5"

    def test_heading6_exists(self):
        """HEADING6 role exists with correct value."""
        assert SemanticRole.HEADING6.value == "heading6"

    def test_all_heading_levels(self):
        """All six heading levels are available."""
        headings = [
            SemanticRole.HEADING1,
            SemanticRole.HEADING2,
            SemanticRole.HEADING3,
            SemanticRole.HEADING4,
            SemanticRole.HEADING5,
            SemanticRole.HEADING6,
        ]
        assert len(headings) == 6


class TestContentStructureRoles:
    """Test content structure roles."""

    def test_paragraph_exists(self):
        """PARAGRAPH role exists with correct value."""
        assert SemanticRole.PARAGRAPH.value == "paragraph"

    def test_quote_exists(self):
        """QUOTE role exists with correct value."""
        assert SemanticRole.QUOTE.value == "quote"

    def test_list_exists(self):
        """LIST role exists with correct value."""
        assert SemanticRole.LIST.value == "list"


class TestTechnicalContentRoles:
    """Test technical content roles."""

    def test_code_exists(self):
        """CODE role exists with correct value."""
        assert SemanticRole.CODE.value == "code"

    def test_table_exists(self):
        """TABLE role exists with correct value."""
        assert SemanticRole.TABLE.value == "table"

    def test_equation_exists(self):
        """EQUATION role exists with correct value."""
        assert SemanticRole.EQUATION.value == "equation"


class TestDocumentStructureRoles:
    """Test document structure roles."""

    def test_title_exists(self):
        """TITLE role exists with correct value."""
        assert SemanticRole.TITLE.value == "title"

    def test_subtitle_exists(self):
        """SUBTITLE role exists with correct value."""
        assert SemanticRole.SUBTITLE.value == "subtitle"

    def test_abstract_exists(self):
        """ABSTRACT role exists with correct value."""
        assert SemanticRole.ABSTRACT.value == "abstract"

    def test_section_exists(self):
        """SECTION role exists with correct value."""
        assert SemanticRole.SECTION.value == "section"


class TestNarrativeStructureRoles:
    """Test narrative structure roles."""

    def test_intro_exists(self):
        """INTRO role exists with correct value."""
        assert SemanticRole.INTRO.value == "intro"

    def test_body_exists(self):
        """BODY role exists with correct value."""
        assert SemanticRole.BODY.value == "body"

    def test_conclusion_exists(self):
        """CONCLUSION role exists with correct value."""
        assert SemanticRole.CONCLUSION.value == "conclusion"


class TestCalloutRoles:
    """Test callout and special section roles."""

    def test_note_exists(self):
        """NOTE role exists with correct value."""
        assert SemanticRole.NOTE.value == "note"

    def test_warning_exists(self):
        """WARNING role exists with correct value."""
        assert SemanticRole.WARNING.value == "warning"

    def test_tip_exists(self):
        """TIP role exists with correct value."""
        assert SemanticRole.TIP.value == "tip"

    def test_sidebar_exists(self):
        """SIDEBAR role exists with correct value."""
        assert SemanticRole.SIDEBAR.value == "sidebar"

    def test_callout_exists(self):
        """CALLOUT role exists with correct value."""
        assert SemanticRole.CALLOUT.value == "callout"


class TestMetaRoles:
    """Test meta element roles."""

    def test_metadata_exists(self):
        """METADATA role exists with correct value."""
        assert SemanticRole.METADATA.value == "metadata"

    def test_citation_exists(self):
        """CITATION role exists with correct value."""
        assert SemanticRole.CITATION.value == "citation"

    def test_footnote_exists(self):
        """FOOTNOTE role exists with correct value."""
        assert SemanticRole.FOOTNOTE.value == "footnote"


class TestRoleAssignment:
    """Test role assignment to blocks."""

    def test_role_assignment_to_block(self):
        """Roles can be assigned to blocks via constructor."""
        block = Block.text("Important note", role=SemanticRole.NOTE)
        assert block.role == SemanticRole.NOTE

    def test_multiple_roles_on_different_blocks(self):
        """Different roles can be assigned to different blocks."""
        note = Block.text("Note content", role=SemanticRole.NOTE)
        warning = Block.text("Warning content", role=SemanticRole.WARNING)
        tip = Block.text("Tip content", role=SemanticRole.TIP)

        assert note.role == SemanticRole.NOTE
        assert warning.role == SemanticRole.WARNING
        assert tip.role == SemanticRole.TIP

    def test_heading_role_assignment(self):
        """Heading roles can be assigned to blocks."""
        h1 = Block.text("Main Title", role=SemanticRole.HEADING1)
        h2 = Block.text("Subtitle", role=SemanticRole.HEADING2)

        assert h1.role == SemanticRole.HEADING1
        assert h2.role == SemanticRole.HEADING2


class TestRoleInDocument:
    """Test roles work in document context."""

    def test_role_in_document_add_block(self):
        """Roles work when adding blocks to document."""
        doc = create()
        block_id = doc.add_block(
            doc.root_id,
            "Warning content",
            role=SemanticRole.WARNING
        )

        block = doc.blocks[block_id]
        assert block.role == SemanticRole.WARNING

    def test_multiple_roles_in_document(self):
        """Multiple blocks with different roles in same document."""
        doc = create()

        intro_id = doc.add_block(doc.root_id, "Introduction", role=SemanticRole.INTRO)
        body_id = doc.add_block(doc.root_id, "Main content", role=SemanticRole.BODY)
        conclusion_id = doc.add_block(doc.root_id, "Summary", role=SemanticRole.CONCLUSION)

        assert doc.blocks[intro_id].role == SemanticRole.INTRO
        assert doc.blocks[body_id].role == SemanticRole.BODY
        assert doc.blocks[conclusion_id].role == SemanticRole.CONCLUSION

    def test_technical_roles_in_document(self):
        """Technical roles (equation, code, table) work in documents."""
        doc = create()

        eq_id = doc.add_block(doc.root_id, "E = mc^2", role=SemanticRole.EQUATION)
        code_id = doc.add_block(doc.root_id, "print('hello')", role=SemanticRole.CODE)
        table_id = doc.add_block(doc.root_id, "| A | B |", role=SemanticRole.TABLE)

        assert doc.blocks[eq_id].role == SemanticRole.EQUATION
        assert doc.blocks[code_id].role == SemanticRole.CODE
        assert doc.blocks[table_id].role == SemanticRole.TABLE


class TestRoleEnumProperties:
    """Test SemanticRole enum properties."""

    def test_role_is_str_enum(self):
        """SemanticRole is a string enum."""
        role = SemanticRole.PARAGRAPH
        assert isinstance(role.value, str)
        assert str(role.value) == "paragraph"

    def test_role_count(self):
        """Verify total number of defined roles."""
        all_roles = list(SemanticRole)
        # 6 headings + 3 content + 3 technical + 4 document + 3 narrative
        # + 5 callouts + 3 meta = 27 total
        assert len(all_roles) >= 27

    def test_role_values_are_lowercase(self):
        """All role values are lowercase."""
        for role in SemanticRole:
            assert role.value == role.value.lower()

    def test_role_values_are_unique(self):
        """All role values are unique."""
        values = [role.value for role in SemanticRole]
        assert len(values) == len(set(values))
