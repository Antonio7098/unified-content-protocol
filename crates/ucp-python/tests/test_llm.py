"""Tests for LLM utilities (IdMapper, PromptBuilder)."""



class TestIdMapper:
    """Test IdMapper for token-efficient LLM prompts."""

    def test_create_id_mapper(self):
        """Test creating an empty IdMapper."""
        import ucp

        mapper = ucp.IdMapper()
        assert len(mapper) == 0

    def test_from_document(self, doc_with_blocks):
        """Test creating IdMapper from a document."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mapper = ucp.IdMapper.from_document(doc)
        assert len(mapper) == 4  # root + 3 blocks

    def test_register(self):
        """Test registering a block ID."""
        import ucp

        mapper = ucp.IdMapper()
        block_id = ucp.BlockId.root()

        short_id = mapper.register(block_id)
        assert short_id == 1  # First registered ID

    def test_to_short_id(self, doc_with_blocks):
        """Test converting block ID to short ID."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mapper = ucp.IdMapper.from_document(doc)
        short_id = mapper.to_short_id(root)

        assert short_id is not None
        assert isinstance(short_id, int)

    def test_to_block_id(self, doc_with_blocks):
        """Test converting short ID back to block ID."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mapper = ucp.IdMapper.from_document(doc)
        short_id = mapper.to_short_id(root)
        block_id = mapper.to_block_id(short_id)

        assert block_id == root

    def test_shorten_text(self, doc_with_blocks):
        """Test shortening text with block IDs."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mapper = ucp.IdMapper.from_document(doc)
        text = f"Edit block {root}"
        shortened = mapper.shorten_text(text)

        # Should replace long ID with short ID
        assert str(root) not in shortened
        assert "Edit block" in shortened

    def test_expand_text(self, doc_with_blocks):
        """Test expanding text with short IDs."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mapper = ucp.IdMapper.from_document(doc)
        short_id = mapper.to_short_id(root)
        text = f"block {short_id}"
        expanded = mapper.expand_text(text)

        # Should expand short ID back to long ID
        assert str(root) in expanded

    def test_shorten_ucl(self, doc_with_blocks):
        """Test shortening UCL commands."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mapper = ucp.IdMapper.from_document(doc)
        ucl = f'EDIT {block1} SET text = "hello"'
        shortened = mapper.shorten_ucl(ucl)

        # Long ID should be replaced
        assert str(block1) not in shortened
        assert "EDIT" in shortened

    def test_expand_ucl(self, doc_with_blocks):
        """Test expanding UCL commands."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mapper = ucp.IdMapper.from_document(doc)
        short_id = mapper.to_short_id(block1)
        ucl = f'EDIT {short_id} SET text = "hello"'
        expanded = mapper.expand_ucl(ucl)

        # Short ID should be expanded
        assert str(block1) in expanded

    def test_estimate_token_savings(self, doc_with_blocks):
        """Test estimating token savings."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mapper = ucp.IdMapper.from_document(doc)
        text = f"Block {root} references {block1} which elaborates {block2}"

        original, shortened, savings = mapper.estimate_token_savings(text)

        assert original > shortened
        assert savings > 0

    def test_document_to_prompt(self, doc_with_blocks):
        """Test generating document prompt."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mapper = ucp.IdMapper.from_document(doc)
        prompt = mapper.document_to_prompt(doc)

        assert "Document structure:" in prompt
        assert "Blocks:" in prompt

    def test_mapping_table(self, doc_with_blocks):
        """Test getting mapping table."""
        import ucp
        doc, root, block1, block2, block3 = doc_with_blocks

        mapper = ucp.IdMapper.from_document(doc)
        table = mapper.mapping_table()

        assert "ID Mapping:" in table


class TestUclCapability:
    """Test UCL capability enumeration."""

    def test_all_capabilities(self):
        """Test getting all capabilities."""
        import ucp

        caps = ucp.UclCapability.all()
        assert len(caps) == 7

    def test_capability_values(self):
        """Test capability enum values."""
        import ucp

        assert ucp.UclCapability.Edit is not None
        assert ucp.UclCapability.Append is not None
        assert ucp.UclCapability.Move is not None
        assert ucp.UclCapability.Delete is not None
        assert ucp.UclCapability.Link is not None
        assert ucp.UclCapability.Snapshot is not None
        assert ucp.UclCapability.Transaction is not None

    def test_capability_command_names(self):
        """Test getting command names for a capability."""
        import ucp

        names = ucp.UclCapability.Edit.command_names()
        assert "EDIT" in names

    def test_capability_documentation(self):
        """Test getting documentation for a capability."""
        import ucp

        doc = ucp.UclCapability.Edit.documentation()
        assert "EDIT" in doc
        assert "SET" in doc


class TestPromptBuilder:
    """Test PromptBuilder for UCL prompt generation."""

    def test_create_prompt_builder(self):
        """Test creating an empty PromptBuilder."""
        import ucp

        builder = ucp.PromptBuilder()
        caps = builder.capabilities()
        assert len(caps) == 0

    def test_with_all_capabilities(self):
        """Test creating builder with all capabilities."""
        import ucp

        builder = ucp.PromptBuilder.with_all_capabilities()
        caps = builder.capabilities()
        assert len(caps) == 7

    def test_with_capability(self):
        """Test adding a capability."""
        import ucp

        builder = ucp.PromptBuilder().with_capability(ucp.UclCapability.Edit)
        assert builder.has_capability(ucp.UclCapability.Edit) is True
        assert builder.has_capability(ucp.UclCapability.Delete) is False

    def test_without_capability(self):
        """Test removing a capability."""
        import ucp

        builder = ucp.PromptBuilder.with_all_capabilities()
        builder = builder.without_capability(ucp.UclCapability.Delete)

        assert builder.has_capability(ucp.UclCapability.Delete) is False
        assert builder.has_capability(ucp.UclCapability.Edit) is True

    def test_with_system_context(self):
        """Test setting system context."""
        import ucp

        builder = ucp.PromptBuilder().with_capability(ucp.UclCapability.Edit)
        builder = builder.with_system_context("You are a helpful assistant.")
        prompt = builder.build_system_prompt()

        assert "You are a helpful assistant." in prompt

    def test_with_task_context(self):
        """Test setting task context."""
        import ucp

        builder = ucp.PromptBuilder().with_capability(ucp.UclCapability.Edit)
        builder = builder.with_task_context("Focus on clarity.")
        prompt = builder.build_prompt("Doc structure here", "Edit block 1")

        assert "Focus on clarity." in prompt

    def test_with_rule(self):
        """Test adding custom rule."""
        import ucp

        builder = ucp.PromptBuilder().with_capability(ucp.UclCapability.Edit)
        builder = builder.with_rule("Always use lowercase for labels")
        prompt = builder.build_system_prompt()

        assert "Always use lowercase for labels" in prompt

    def test_with_short_ids(self):
        """Test enabling short ID mode."""
        import ucp

        builder = ucp.PromptBuilder().with_capability(ucp.UclCapability.Edit)
        builder = builder.with_short_ids(True)
        prompt = builder.build_system_prompt()

        assert "short numeric IDs" in prompt

    def test_build_system_prompt(self):
        """Test building system prompt."""
        import ucp

        builder = ucp.PromptBuilder.with_all_capabilities()
        prompt = builder.build_system_prompt()

        assert "UCL Command Reference" in prompt
        assert "EDIT" in prompt
        assert "APPEND" in prompt
        assert "Rules" in prompt

    def test_build_prompt(self):
        """Test building complete prompt."""
        import ucp

        builder = ucp.PromptBuilder().with_capability(ucp.UclCapability.Edit)
        prompt = builder.build_prompt(
            "Document with blocks: [1] Title, [2] Paragraph",
            "Edit block 2 to say 'Hello World'"
        )

        assert "Document Structure" in prompt
        assert "Task" in prompt
        assert "Edit block 2" in prompt


class TestPromptPresets:
    """Test preset prompt configurations."""

    def test_basic_editing(self):
        """Test basic editing preset."""
        import ucp

        builder = ucp.PromptPresets.basic_editing()

        assert builder.has_capability(ucp.UclCapability.Edit) is True
        assert builder.has_capability(ucp.UclCapability.Append) is True
        assert builder.has_capability(ucp.UclCapability.Delete) is True
        assert builder.has_capability(ucp.UclCapability.Move) is False

    def test_structure_manipulation(self):
        """Test structure manipulation preset."""
        import ucp

        builder = ucp.PromptPresets.structure_manipulation()

        assert builder.has_capability(ucp.UclCapability.Move) is True
        assert builder.has_capability(ucp.UclCapability.Link) is True
        assert builder.has_capability(ucp.UclCapability.Edit) is False

    def test_full_editing(self):
        """Test full editing preset."""
        import ucp

        builder = ucp.PromptPresets.full_editing()

        assert builder.has_capability(ucp.UclCapability.Edit) is True
        assert builder.has_capability(ucp.UclCapability.Append) is True
        assert builder.has_capability(ucp.UclCapability.Move) is True
        assert builder.has_capability(ucp.UclCapability.Delete) is True
        assert builder.has_capability(ucp.UclCapability.Link) is True
        assert builder.has_capability(ucp.UclCapability.Transaction) is False

    def test_version_control(self):
        """Test version control preset."""
        import ucp

        builder = ucp.PromptPresets.version_control()

        assert builder.has_capability(ucp.UclCapability.Snapshot) is True
        assert builder.has_capability(ucp.UclCapability.Transaction) is True
        assert builder.has_capability(ucp.UclCapability.Edit) is False

    def test_token_efficient(self):
        """Test token efficient preset."""
        import ucp

        builder = ucp.PromptPresets.token_efficient()
        prompt = builder.build_system_prompt()

        # Should mention short IDs
        assert "short numeric IDs" in prompt
