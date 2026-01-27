//! Parser for UCL documents.

use crate::ast::*;
use crate::lexer::{Lexer, Token, TokenKind};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token at line {line}: expected {expected}, found {found}")]
    UnexpectedToken {
        expected: String,
        found: String,
        line: usize,
        column: usize,
    },
    #[error("Unexpected end of input")]
    UnexpectedEof,
    #[error("Invalid syntax at line {line}: {message}")]
    InvalidSyntax { message: String, line: usize },
    #[error("Lexer error at position {position}")]
    LexerError { position: usize },
}

pub type ParseResult<T> = Result<T, ParseError>;

pub struct Parser<'a> {
    tokens: Vec<Token>,
    pos: usize,
    source: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer
            .filter_map(|r| r.ok())
            .filter(|t| !matches!(t.kind, TokenKind::Newline))
            .collect();
        Self {
            tokens,
            pos: 0,
            source: input,
        }
    }

    pub fn parse_document(&mut self) -> ParseResult<UclDocument> {
        let mut doc = UclDocument::new();
        while !self.is_at_end() {
            match self.peek_kind() {
                Some(TokenKind::Structure) => {
                    self.advance();
                    doc.structure = self.parse_structure()?;
                }
                Some(TokenKind::Blocks) => {
                    self.advance();
                    doc.blocks = self.parse_blocks()?;
                }
                Some(TokenKind::Commands) => {
                    self.advance();
                    doc.commands = self.parse_commands()?;
                }
                Some(_) => {
                    if let Ok(cmd) = self.parse_command() {
                        doc.commands.push(cmd);
                    } else {
                        self.advance();
                    }
                }
                None => break,
            }
        }
        Ok(doc)
    }

    pub fn parse_commands_only(&mut self) -> ParseResult<Vec<Command>> {
        let mut cmds = Vec::new();
        while !self.is_at_end() {
            cmds.push(self.parse_command()?);
        }
        Ok(cmds)
    }

    fn parse_structure(&mut self) -> ParseResult<HashMap<String, Vec<String>>> {
        let mut structure = HashMap::new();
        while !self.is_at_end() && !self.is_section_header() {
            if matches!(self.peek_kind(), Some(TokenKind::BlockId)) {
                let parent = self.expect_block_id()?;
                self.expect(TokenKind::Colon)?;
                self.expect(TokenKind::LBracket)?;
                let mut children = Vec::new();
                while !self.check(TokenKind::RBracket) {
                    children.push(self.expect_block_id()?);
                    if !self.check(TokenKind::RBracket) {
                        let _ = self.expect(TokenKind::Comma);
                    }
                }
                self.expect(TokenKind::RBracket)?;
                structure.insert(parent, children);
            } else {
                break;
            }
        }
        Ok(structure)
    }

    fn parse_blocks(&mut self) -> ParseResult<Vec<BlockDef>> {
        let mut blocks = Vec::new();
        while !self.is_at_end() && !self.is_section_header() {
            if let Some(ct) = self.try_content_type() {
                blocks.push(self.parse_block_def(ct)?);
            } else {
                break;
            }
        }
        Ok(blocks)
    }

    fn parse_block_def(&mut self, content_type: ContentType) -> ParseResult<BlockDef> {
        self.expect(TokenKind::Hash)?;
        let id = self.expect_block_id()?;
        let mut props = HashMap::new();
        while !self.check(TokenKind::DoubleColon) && !self.is_at_end() {
            let k = self.expect_ident_or_keyword()?;
            self.expect(TokenKind::Eq)?;
            props.insert(k, self.parse_value()?);
        }
        self.expect(TokenKind::DoubleColon)?;
        let content = self.parse_content_literal()?;
        Ok(BlockDef {
            content_type,
            id,
            properties: props,
            content,
        })
    }

    fn parse_commands(&mut self) -> ParseResult<Vec<Command>> {
        let mut cmds = Vec::new();
        while !self.is_at_end() && !self.is_section_header() {
            if let Ok(cmd) = self.parse_command() {
                cmds.push(cmd);
            } else {
                break;
            }
        }
        Ok(cmds)
    }

    fn parse_command(&mut self) -> ParseResult<Command> {
        match self.peek_kind() {
            // Document modification commands
            Some(TokenKind::Edit) => self.parse_edit(),
            Some(TokenKind::Move) => self.parse_move(),
            Some(TokenKind::Append) => self.parse_append(),
            Some(TokenKind::Delete) => self.parse_delete(),
            Some(TokenKind::Prune) => self.parse_prune(),
            Some(TokenKind::Link) => self.parse_link(),
            Some(TokenKind::Unlink) => self.parse_unlink(),
            Some(TokenKind::Fold) => self.parse_fold(),
            Some(TokenKind::Snapshot) => self.parse_snapshot(),
            Some(TokenKind::Begin) => self.parse_begin(),
            Some(TokenKind::Commit) => self.parse_commit(),
            Some(TokenKind::Rollback) => self.parse_rollback(),
            Some(TokenKind::Atomic) => self.parse_atomic(),
            Some(TokenKind::WriteSection) => self.parse_write_section(),

            // Agent traversal commands
            Some(TokenKind::Goto) => self.parse_goto(),
            Some(TokenKind::Back) => self.parse_back(),
            Some(TokenKind::Expand) => self.parse_expand(),
            Some(TokenKind::Follow) => self.parse_follow(),
            Some(TokenKind::Path) => self.parse_path_find(),
            Some(TokenKind::Search) => self.parse_search(),
            Some(TokenKind::Find) => self.parse_find(),
            Some(TokenKind::View) => self.parse_view(),

            // Context commands
            Some(TokenKind::Ctx) => self.parse_ctx(),

            _ => Err(self.error("command")),
        }
    }

    fn parse_edit(&mut self) -> ParseResult<Command> {
        self.advance();
        let id = self.expect_block_id()?;
        self.expect(TokenKind::Set)?;
        let path = self.parse_path()?;
        let op = self.parse_op()?;
        let val = self.parse_value()?;
        let cond = if self.check(TokenKind::Where) {
            self.advance();
            Some(self.parse_cond()?)
        } else {
            None
        };
        Ok(Command::Edit(EditCommand {
            block_id: id,
            path,
            operator: op,
            value: val,
            condition: cond,
        }))
    }

    fn parse_move(&mut self) -> ParseResult<Command> {
        self.advance();
        let id = self.expect_block_id()?;
        let target = if self.check(TokenKind::To) {
            self.advance();
            let pid = self.expect_block_id()?;
            let idx = if self.check(TokenKind::At) {
                self.advance();
                Some(self.expect_int()? as usize)
            } else {
                None
            };
            MoveTarget::ToParent {
                parent_id: pid,
                index: idx,
            }
        } else if self.check(TokenKind::Before) {
            self.advance();
            MoveTarget::Before {
                sibling_id: self.expect_block_id()?,
            }
        } else if self.check(TokenKind::After) {
            self.advance();
            MoveTarget::After {
                sibling_id: self.expect_block_id()?,
            }
        } else {
            return Err(self.error("TO/BEFORE/AFTER"));
        };
        Ok(Command::Move(MoveCommand {
            block_id: id,
            target,
        }))
    }

    fn parse_append(&mut self) -> ParseResult<Command> {
        self.advance();
        let pid = self.expect_block_id()?;
        let ct = self.parse_content_type()?;
        let mut props = HashMap::new();
        let mut idx = None;
        if self.check(TokenKind::At) {
            self.advance();
            idx = Some(self.expect_int()? as usize);
        }
        if self.check(TokenKind::With) {
            self.advance();
            while !self.check(TokenKind::DoubleColon) && !self.is_at_end() {
                let k = self.expect_ident()?;
                self.expect(TokenKind::Eq)?;
                props.insert(k, self.parse_value()?);
            }
        }
        self.expect(TokenKind::DoubleColon)?;
        let content = self.parse_content_literal()?;
        Ok(Command::Append(AppendCommand {
            parent_id: pid,
            content_type: ct,
            properties: props,
            content,
            index: idx,
        }))
    }

    fn parse_delete(&mut self) -> ParseResult<Command> {
        self.advance();
        let (bid, cond) = if self.check(TokenKind::Where) {
            self.advance();
            (None, Some(self.parse_cond()?))
        } else {
            (Some(self.expect_block_id()?), None)
        };
        let casc = if self.check(TokenKind::Cascade) {
            {
                self.advance();
                true
            }
        } else {
            false
        };
        let pres = if self.check(TokenKind::PreserveChildren) {
            {
                self.advance();
                true
            }
        } else {
            false
        };
        Ok(Command::Delete(DeleteCommand {
            block_id: bid,
            cascade: casc,
            preserve_children: pres,
            condition: cond,
        }))
    }

    fn parse_prune(&mut self) -> ParseResult<Command> {
        self.advance();
        let tgt = if self.check(TokenKind::Unreachable) {
            self.advance();
            PruneTarget::Unreachable
        } else if self.check(TokenKind::Where) {
            self.advance();
            PruneTarget::Where(self.parse_cond()?)
        } else {
            PruneTarget::Unreachable
        };
        let dry = if self.check(TokenKind::DryRun) {
            {
                self.advance();
                true
            }
        } else {
            false
        };
        Ok(Command::Prune(PruneCommand {
            target: tgt,
            dry_run: dry,
        }))
    }

    fn parse_fold(&mut self) -> ParseResult<Command> {
        self.advance();
        let id = self.expect_block_id()?;
        let (mut d, mut t, mut tags) = (None, None, Vec::new());
        while !self.is_at_end() && !self.is_cmd_start() {
            if self.check(TokenKind::Depth) {
                self.advance();
                d = Some(self.expect_int()? as usize);
            } else if self.check(TokenKind::MaxTokens) {
                self.advance();
                t = Some(self.expect_int()? as usize);
            } else if self.check(TokenKind::PreserveTags) {
                self.advance();
                self.expect(TokenKind::LBracket)?;
                while !self.check(TokenKind::RBracket) {
                    tags.push(self.expect_str()?);
                    if !self.check(TokenKind::RBracket) {
                        let _ = self.expect(TokenKind::Comma);
                    }
                }
                self.expect(TokenKind::RBracket)?;
            } else {
                break;
            }
        }
        Ok(Command::Fold(FoldCommand {
            block_id: id,
            depth: d,
            max_tokens: t,
            preserve_tags: tags,
        }))
    }

    fn parse_link(&mut self) -> ParseResult<Command> {
        self.advance();
        let s = self.expect_block_id()?;
        let e = self.expect_ident()?;
        let t = self.expect_block_id()?;
        let mut m = HashMap::new();
        if self.check(TokenKind::With) {
            self.advance();
            while !self.is_at_end() && !self.is_cmd_start() {
                let k = self.expect_ident()?;
                self.expect(TokenKind::Eq)?;
                m.insert(k, self.parse_value()?);
            }
        }
        Ok(Command::Link(LinkCommand {
            source_id: s,
            edge_type: e,
            target_id: t,
            metadata: m,
        }))
    }

    fn parse_unlink(&mut self) -> ParseResult<Command> {
        self.advance();
        Ok(Command::Unlink(UnlinkCommand {
            source_id: self.expect_block_id()?,
            edge_type: self.expect_ident()?,
            target_id: self.expect_block_id()?,
        }))
    }

    fn parse_write_section(&mut self) -> ParseResult<Command> {
        self.advance(); // consume WRITE_SECTION

        let section_id = self.expect_block_id()?;

        // Expect :: separator for markdown content
        self.expect(TokenKind::DoubleColon)?;

        // Parse markdown content (string literal)
        let markdown = self.expect_str()?;

        // Parse optional BASE_LEVEL
        let base_heading_level = if self.check(TokenKind::BaseLevel) {
            self.advance();
            Some(self.expect_int()? as usize)
        } else {
            None
        };

        Ok(Command::WriteSection(WriteSectionCommand {
            section_id,
            markdown,
            base_heading_level,
        }))
    }

    fn parse_snapshot(&mut self) -> ParseResult<Command> {
        self.advance();
        let cmd = if self.check(TokenKind::Create) {
            self.advance();
            let n = self.expect_str()?;
            let d = if self.check(TokenKind::With) {
                self.advance();
                self.expect_ident()?;
                self.expect(TokenKind::Eq)?;
                Some(self.expect_str()?)
            } else {
                None
            };
            SnapshotCommand::Create {
                name: n,
                description: d,
            }
        } else if self.check(TokenKind::Restore) {
            self.advance();
            SnapshotCommand::Restore {
                name: self.expect_str()?,
            }
        } else if self.check(TokenKind::List) {
            self.advance();
            SnapshotCommand::List
        } else if self.check(TokenKind::Delete) {
            self.advance();
            SnapshotCommand::Delete {
                name: self.expect_str()?,
            }
        } else if self.check(TokenKind::Diff) {
            self.advance();
            SnapshotCommand::Diff {
                name1: self.expect_str()?,
                name2: self.expect_str()?,
            }
        } else {
            return Err(self.error("snapshot action"));
        };
        Ok(Command::Snapshot(cmd))
    }

    fn parse_begin(&mut self) -> ParseResult<Command> {
        self.advance();
        self.expect(TokenKind::Transaction)?;
        let n = self.try_str();
        Ok(Command::Transaction(TransactionCommand::Begin { name: n }))
    }
    fn parse_commit(&mut self) -> ParseResult<Command> {
        self.advance();
        Ok(Command::Transaction(TransactionCommand::Commit {
            name: self.try_str(),
        }))
    }
    fn parse_rollback(&mut self) -> ParseResult<Command> {
        self.advance();
        Ok(Command::Transaction(TransactionCommand::Rollback {
            name: self.try_str(),
        }))
    }

    fn parse_atomic(&mut self) -> ParseResult<Command> {
        self.advance();
        self.expect(TokenKind::LBrace)?;
        let mut cmds = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            cmds.push(self.parse_command()?);
        }
        self.expect(TokenKind::RBrace)?;
        Ok(Command::Atomic(cmds))
    }

    // ========================================================================
    // Agent Traversal Command Parsers
    // ========================================================================

    /// Parse GOTO blk_xxx
    fn parse_goto(&mut self) -> ParseResult<Command> {
        self.advance(); // consume GOTO
        let block_id = self.expect_block_id()?;
        Ok(Command::Goto(GotoCommand { block_id }))
    }

    /// Parse BACK [n]
    fn parse_back(&mut self) -> ParseResult<Command> {
        self.advance(); // consume BACK
        let steps = if let Some(TokenKind::Integer(n)) = self.peek_kind() {
            self.advance();
            n as usize
        } else {
            1
        };
        Ok(Command::Back(BackCommand { steps }))
    }

    /// Parse EXPAND blk_xxx DOWN|UP|BOTH|SEMANTIC [depth=N] [mode=MODE] [roles=...] [tags=...]
    fn parse_expand(&mut self) -> ParseResult<Command> {
        self.advance(); // consume EXPAND
        if matches!(
            self.peek_kind(),
            Some(TokenKind::Down | TokenKind::Up | TokenKind::Both | TokenKind::Semantic)
        ) {
            return Err(self.error_with_hint(
                "EXPAND syntax: provide the block ID before the direction (e.g., EXPAND blk_root DOWN)",
            ));
        }
        let block_id = self.expect_block_id()?;

        // Parse direction
        let direction = match self.peek_kind() {
            Some(TokenKind::Down) => {
                self.advance();
                ExpandDirection::Down
            }
            Some(TokenKind::Up) => {
                self.advance();
                ExpandDirection::Up
            }
            Some(TokenKind::Both) => {
                self.advance();
                ExpandDirection::Both
            }
            Some(TokenKind::Semantic) => {
                self.advance();
                ExpandDirection::Semantic
            }
            _ => ExpandDirection::Down, // Default
        };

        // Parse options
        let mut depth = 1usize;
        let mut mode = None;
        let mut filter = TraversalFilterCriteria::default();

        while !self.is_at_end() && !self.is_cmd_start() {
            match self.peek_kind() {
                Some(TokenKind::Depth) => {
                    self.advance();
                    self.expect(TokenKind::Eq)?;
                    depth = self.expect_int()? as usize;
                }
                Some(TokenKind::Mode) => {
                    self.advance();
                    self.expect(TokenKind::Eq)?;
                    mode = Some(self.parse_view_mode()?);
                }
                Some(TokenKind::Roles) => {
                    self.advance();
                    self.expect(TokenKind::Eq)?;
                    filter.include_roles = self.parse_comma_list()?;
                }
                Some(TokenKind::Tags) => {
                    self.advance();
                    self.expect(TokenKind::Eq)?;
                    filter.include_tags = self.parse_comma_list()?;
                }
                _ => break,
            }
        }

        Ok(Command::Expand(ExpandCommand {
            block_id,
            direction,
            depth,
            mode,
            filter: if filter.include_roles.is_empty()
                && filter.include_tags.is_empty()
                && filter.exclude_roles.is_empty()
                && filter.exclude_tags.is_empty()
            {
                None
            } else {
                Some(filter)
            },
        }))
    }

    /// Parse FOLLOW blk_xxx edge_type[,edge_type...] [blk_yyy]
    fn parse_follow(&mut self) -> ParseResult<Command> {
        self.advance(); // consume FOLLOW
        let source_id = self.expect_block_id()?;

        // Parse edge types (comma-separated identifiers)
        let edge_types = self.parse_comma_list()?;

        // Optional target block
        let target_id = if matches!(self.peek_kind(), Some(TokenKind::BlockId)) {
            Some(self.expect_block_id()?)
        } else {
            None
        };

        Ok(Command::Follow(FollowCommand {
            source_id,
            edge_types,
            target_id,
        }))
    }

    /// Parse PATH blk_xxx TO blk_yyy [max=N]
    fn parse_path_find(&mut self) -> ParseResult<Command> {
        self.advance(); // consume PATH
        let from_id = self.expect_block_id()?;
        if self.check(TokenKind::To) {
            self.advance();
        } else if matches!(self.peek_kind(), Some(TokenKind::BlockId)) {
            return Err(self.error_with_hint(
                "PATH syntax: include the TO keyword between the two block IDs (e.g., PATH blk_a TO blk_b)",
            ));
        } else {
            self.expect(TokenKind::To)?;
        }
        let to_id = self.expect_block_id()?;

        let max_length = if self.check(TokenKind::Max) {
            self.advance();
            self.expect(TokenKind::Eq)?;
            Some(self.expect_int()? as usize)
        } else {
            None
        };

        Ok(Command::Path(PathFindCommand {
            from_id,
            to_id,
            max_length,
        }))
    }

    /// Parse SEARCH "query" [limit=N] [min_similarity=F] [roles=...]
    fn parse_search(&mut self) -> ParseResult<Command> {
        self.advance(); // consume SEARCH
        let query = self.expect_str()?;

        let mut limit = None;
        let mut min_similarity = None;
        let mut filter = TraversalFilterCriteria::default();

        while !self.is_at_end() && !self.is_cmd_start() {
            match self.peek_kind() {
                Some(TokenKind::Limit) => {
                    self.advance();
                    self.expect(TokenKind::Eq)?;
                    limit = Some(self.expect_int()? as usize);
                }
                Some(TokenKind::MinSimilarity) => {
                    self.advance();
                    self.expect(TokenKind::Eq)?;
                    min_similarity = Some(self.expect_float()?);
                }
                Some(TokenKind::Roles) => {
                    self.advance();
                    self.expect(TokenKind::Eq)?;
                    filter.include_roles = self.parse_comma_list()?;
                }
                Some(TokenKind::Tags) => {
                    self.advance();
                    self.expect(TokenKind::Eq)?;
                    filter.include_tags = self.parse_comma_list()?;
                }
                _ => break,
            }
        }

        Ok(Command::Search(SearchCommand {
            query,
            limit,
            min_similarity,
            filter: if filter.include_roles.is_empty() && filter.include_tags.is_empty() {
                None
            } else {
                Some(filter)
            },
        }))
    }

    /// Parse FIND [role=...] [tag=...] [label=...] [pattern=...]
    fn parse_find(&mut self) -> ParseResult<Command> {
        self.advance(); // consume FIND
        let mut cmd = FindCommand::default();

        while !self.is_at_end() && !self.is_cmd_start() {
            match self.peek_kind() {
                Some(TokenKind::Role) => {
                    self.advance();
                    self.expect_eq_with_hint("FIND ROLE must use '=' (e.g., ROLE=heading1)")?;
                    cmd.role = Some(self.expect_ident_or_str()?);
                }
                Some(TokenKind::Tag) => {
                    self.advance();
                    self.expect_eq_with_hint("FIND TAG must use '=' (e.g., TAG=\"important\")")?;
                    cmd.tag = Some(self.expect_str()?);
                }
                Some(TokenKind::Label) => {
                    self.advance();
                    self.expect_eq_with_hint("FIND LABEL must use '=' (e.g., LABEL=\"summary\")")?;
                    cmd.label = Some(self.expect_str()?);
                }
                Some(TokenKind::Pattern) => {
                    self.advance();
                    self.expect_eq_with_hint("FIND PATTERN must use '=' (e.g., PATTERN=\".*\")")?;
                    cmd.pattern = Some(self.expect_str()?);
                }
                _ => break,
            }
        }

        Ok(Command::Find(cmd))
    }

    /// Parse VIEW blk_xxx|NEIGHBORHOOD [mode=...] [depth=N]
    fn parse_view(&mut self) -> ParseResult<Command> {
        self.advance(); // consume VIEW

        let target = if self.check(TokenKind::Neighborhood) {
            self.advance();
            ViewTarget::Neighborhood
        } else if matches!(self.peek_kind(), Some(TokenKind::BlockId)) {
            ViewTarget::Block(self.expect_block_id()?)
        } else {
            ViewTarget::Neighborhood
        };

        let mut mode = ViewMode::Full;
        let mut depth = None;

        while !self.is_at_end() && !self.is_cmd_start() {
            match self.peek_kind() {
                Some(TokenKind::Mode) => {
                    self.advance();
                    self.expect(TokenKind::Eq)?;
                    mode = self.parse_view_mode()?;
                }
                Some(TokenKind::Depth) => {
                    self.advance();
                    self.expect(TokenKind::Eq)?;
                    depth = Some(self.expect_int()? as usize);
                }
                _ => break,
            }
        }

        Ok(Command::View(ViewCommand {
            target,
            mode,
            depth,
        }))
    }

    // ========================================================================
    // Context Command Parsers
    // ========================================================================

    /// Parse CTX ADD|REMOVE|CLEAR|EXPAND|COMPRESS|PRUNE|RENDER|STATS|FOCUS ...
    fn parse_ctx(&mut self) -> ParseResult<Command> {
        self.advance(); // consume CTX

        match self.peek_kind() {
            Some(TokenKind::Add) => self.parse_ctx_add(),
            Some(TokenKind::Remove) => self.parse_ctx_remove(),
            Some(TokenKind::Clear) => {
                self.advance();
                Ok(Command::Context(ContextCommand::Clear))
            }
            Some(TokenKind::Expand) => self.parse_ctx_expand(),
            Some(TokenKind::Compress) => self.parse_ctx_compress(),
            Some(TokenKind::Prune) => self.parse_ctx_prune(),
            Some(TokenKind::Render) => self.parse_ctx_render(),
            Some(TokenKind::Stats) => {
                self.advance();
                Ok(Command::Context(ContextCommand::Stats))
            }
            Some(TokenKind::Focus) => self.parse_ctx_focus(),
            _ => Err(self.error(
                "CTX subcommand (ADD/REMOVE/CLEAR/EXPAND/COMPRESS/PRUNE/RENDER/STATS/FOCUS)",
            )),
        }
    }

    /// Parse CTX ADD blk_xxx|RESULTS|CHILDREN blk_xxx|PATH blk_xxx TO blk_yyy [reason=...] [relevance=F]
    fn parse_ctx_add(&mut self) -> ParseResult<Command> {
        self.advance(); // consume ADD

        let target = if self.check(TokenKind::Results) {
            self.advance();
            ContextAddTarget::Results
        } else if self.check(TokenKind::Children) {
            self.advance();
            let parent_id = self.expect_block_id()?;
            ContextAddTarget::Children { parent_id }
        } else if self.check(TokenKind::Path) {
            self.advance();
            let from_id = self.expect_block_id()?;
            if self.check(TokenKind::To) {
                self.advance();
            } else if matches!(self.peek_kind(), Some(TokenKind::BlockId)) {
                return Err(self.error_with_hint(
                    "CTX ADD PATH syntax: include the TO keyword between the two block IDs",
                ));
            } else {
                self.expect(TokenKind::To)?;
            }
            let to_id = self.expect_block_id()?;
            ContextAddTarget::Path { from_id, to_id }
        } else {
            let block_id = self.expect_block_id()?;
            ContextAddTarget::Block(block_id)
        };

        let mut reason = None;
        let mut relevance = None;

        while !self.is_at_end() && !self.is_cmd_start() {
            match self.peek_kind() {
                Some(TokenKind::Reason) => {
                    self.advance();
                    self.expect_eq_with_hint(
                        "CTX ADD REASON must use '=' (e.g., REASON=\"for_llm\")",
                    )?;
                    reason = Some(self.expect_str()?);
                }
                Some(TokenKind::Relevance) => {
                    self.advance();
                    self.expect_eq_with_hint(
                        "CTX ADD RELEVANCE must use '=' (e.g., RELEVANCE=0.8)",
                    )?;
                    relevance = Some(self.expect_float()?);
                }
                _ => break,
            }
        }

        Ok(Command::Context(ContextCommand::Add(ContextAddCommand {
            target,
            reason,
            relevance,
        })))
    }

    /// Parse CTX REMOVE blk_xxx
    fn parse_ctx_remove(&mut self) -> ParseResult<Command> {
        self.advance(); // consume REMOVE
        let block_id = self.expect_block_id()?;
        Ok(Command::Context(ContextCommand::Remove { block_id }))
    }

    /// Parse CTX EXPAND DOWN|UP|SEMANTIC|AUTO [depth=N] [tokens=N]
    fn parse_ctx_expand(&mut self) -> ParseResult<Command> {
        self.advance(); // consume EXPAND

        let direction = match self.peek_kind() {
            Some(TokenKind::Down) => {
                self.advance();
                ExpandDirection::Down
            }
            Some(TokenKind::Up) => {
                self.advance();
                ExpandDirection::Up
            }
            Some(TokenKind::Semantic) => {
                self.advance();
                ExpandDirection::Semantic
            }
            Some(TokenKind::Both) => {
                self.advance();
                ExpandDirection::Both
            }
            _ => ExpandDirection::Down,
        };

        let mut depth = None;
        let mut token_budget = None;

        while !self.is_at_end() && !self.is_cmd_start() {
            match self.peek_kind() {
                Some(TokenKind::Depth) => {
                    self.advance();
                    self.expect(TokenKind::Eq)?;
                    depth = Some(self.expect_int()? as usize);
                }
                Some(TokenKind::Tokens) => {
                    self.advance();
                    self.expect(TokenKind::Eq)?;
                    token_budget = Some(self.expect_int()? as usize);
                }
                _ => break,
            }
        }

        Ok(Command::Context(ContextCommand::Expand(
            ContextExpandCommand {
                direction,
                depth,
                token_budget,
            },
        )))
    }

    /// Parse CTX COMPRESS method=TRUNCATE|SUMMARIZE|STRUCTURE_ONLY
    fn parse_ctx_compress(&mut self) -> ParseResult<Command> {
        self.advance(); // consume COMPRESS

        let method = if self.check(TokenKind::Method) {
            self.advance();
            self.expect(TokenKind::Eq)?;
            self.parse_compression_method()?
        } else {
            CompressionMethod::Truncate
        };

        Ok(Command::Context(ContextCommand::Compress { method }))
    }

    /// Parse CTX PRUNE [min_relevance=F] [max_age=N]
    fn parse_ctx_prune(&mut self) -> ParseResult<Command> {
        self.advance(); // consume PRUNE

        let mut min_relevance = None;
        let mut max_age_secs = None;

        while !self.is_at_end() && !self.is_cmd_start() {
            match self.peek_kind() {
                Some(TokenKind::MinSimilarity) | Some(TokenKind::Relevance) => {
                    self.advance();
                    self.expect(TokenKind::Eq)?;
                    min_relevance = Some(self.expect_float()?);
                }
                Some(TokenKind::MaxAge) => {
                    self.advance();
                    self.expect(TokenKind::Eq)?;
                    max_age_secs = Some(self.expect_int()? as u64);
                }
                _ => break,
            }
        }

        Ok(Command::Context(ContextCommand::Prune(
            ContextPruneCommand {
                min_relevance,
                max_age_secs,
            },
        )))
    }

    /// Parse CTX RENDER [format=DEFAULT|SHORT_IDS|MARKDOWN]
    fn parse_ctx_render(&mut self) -> ParseResult<Command> {
        self.advance(); // consume RENDER

        let format = if self.check(TokenKind::Format) {
            self.advance();
            self.expect(TokenKind::Eq)?;
            Some(self.parse_render_format()?)
        } else {
            None
        };

        Ok(Command::Context(ContextCommand::Render { format }))
    }

    /// Parse CTX FOCUS blk_xxx|CLEAR
    fn parse_ctx_focus(&mut self) -> ParseResult<Command> {
        self.advance(); // consume FOCUS

        let block_id = if self.check(TokenKind::Clear) {
            self.advance();
            None
        } else if matches!(self.peek_kind(), Some(TokenKind::BlockId)) {
            Some(self.expect_block_id()?)
        } else {
            None
        };

        Ok(Command::Context(ContextCommand::Focus { block_id }))
    }

    // ========================================================================
    // Helper Parsers
    // ========================================================================

    fn parse_view_mode(&mut self) -> ParseResult<ViewMode> {
        match self.peek_kind() {
            Some(TokenKind::Full) => {
                self.advance();
                Ok(ViewMode::Full)
            }
            Some(TokenKind::Preview) => {
                self.advance();
                Ok(ViewMode::Preview { length: 100 })
            }
            Some(TokenKind::MetadataToken) => {
                self.advance();
                Ok(ViewMode::Metadata)
            }
            Some(TokenKind::Ids) => {
                self.advance();
                Ok(ViewMode::IdsOnly)
            }
            Some(TokenKind::Identifier) => {
                let span = self.tokens[self.pos].span.clone();
                let s = self.source[span].to_string();
                self.advance();
                ViewMode::parse(&s).ok_or_else(|| self.error("view mode"))
            }
            _ => Err(self.error("view mode (FULL/PREVIEW/METADATA/IDS)")),
        }
    }

    fn parse_compression_method(&mut self) -> ParseResult<CompressionMethod> {
        match self.peek_kind() {
            Some(TokenKind::Truncate) => {
                self.advance();
                Ok(CompressionMethod::Truncate)
            }
            Some(TokenKind::Summarize) => {
                self.advance();
                Ok(CompressionMethod::Summarize)
            }
            Some(TokenKind::StructureOnly) => {
                self.advance();
                Ok(CompressionMethod::StructureOnly)
            }
            Some(TokenKind::Identifier) => {
                let span = self.tokens[self.pos].span.clone();
                let s = self.source[span].to_string();
                self.advance();
                CompressionMethod::parse(&s).ok_or_else(|| self.error("compression method"))
            }
            _ => Err(self.error("compression method (TRUNCATE/SUMMARIZE/STRUCTURE_ONLY)")),
        }
    }

    fn parse_render_format(&mut self) -> ParseResult<RenderFormat> {
        match self.peek_kind() {
            Some(TokenKind::ShortIds) => {
                self.advance();
                Ok(RenderFormat::ShortIds)
            }
            Some(TokenKind::Markdown) => {
                self.advance();
                Ok(RenderFormat::Markdown)
            }
            Some(TokenKind::Identifier) => {
                let span = self.tokens[self.pos].span.clone();
                let s = self.source[span].to_string();
                self.advance();
                RenderFormat::parse(&s).ok_or_else(|| self.error("render format"))
            }
            _ => Ok(RenderFormat::Default),
        }
    }

    /// Parse comma-separated list of identifiers
    fn parse_comma_list(&mut self) -> ParseResult<Vec<String>> {
        let mut items = Vec::new();
        items.push(self.expect_ident_or_str()?);

        while self.check(TokenKind::Comma) {
            self.advance();
            items.push(self.expect_ident_or_str()?);
        }

        Ok(items)
    }

    fn expect_ident_or_str(&mut self) -> ParseResult<String> {
        match self.peek_kind() {
            Some(TokenKind::DoubleString(s)) | Some(TokenKind::SingleString(s)) => {
                self.advance();
                Ok(s)
            }
            Some(TokenKind::Identifier) => {
                let span = self.tokens[self.pos].span.clone();
                self.advance();
                Ok(self.source[span].to_string())
            }
            _ => self.expect_ident_or_keyword(),
        }
    }

    fn expect_float(&mut self) -> ParseResult<f32> {
        match self.peek_kind() {
            Some(TokenKind::Float(n)) => {
                self.advance();
                Ok(n as f32)
            }
            Some(TokenKind::Integer(n)) => {
                self.advance();
                Ok(n as f32)
            }
            _ => Err(self.error("float")),
        }
    }

    fn parse_path(&mut self) -> ParseResult<Path> {
        let mut segs = Vec::new();
        if self.check(TokenKind::Dollar) {
            self.advance();
            segs.push(PathSegment::JsonPath(self.expect_ident_or_keyword()?));
            return Ok(Path::new(segs));
        }
        loop {
            if self.is_path_property_start() {
                segs.push(PathSegment::Property(self.expect_path_property()?));
            } else {
                break;
            }
            if self.check(TokenKind::LBracket) {
                self.advance();
                let s = if matches!(self.peek_kind(), Some(TokenKind::Integer(_))) {
                    let n = self.expect_int()?;
                    Some(n)
                } else {
                    None
                };
                if self.check(TokenKind::Colon) {
                    self.advance();
                    let e = if matches!(self.peek_kind(), Some(TokenKind::Integer(_))) {
                        Some(self.expect_int()?)
                    } else {
                        None
                    };
                    segs.push(PathSegment::Slice { start: s, end: e });
                } else if let Some(i) = s {
                    segs.push(PathSegment::Index(i));
                }
                self.expect(TokenKind::RBracket)?;
            }
            if self.check(TokenKind::Dot) {
                self.advance();
            } else {
                break;
            }
        }
        Ok(Path::new(segs))
    }

    fn parse_op(&mut self) -> ParseResult<Operator> {
        match self.peek_kind() {
            Some(TokenKind::Eq) => {
                self.advance();
                Ok(Operator::Set)
            }
            Some(TokenKind::PlusEq) => {
                self.advance();
                Ok(Operator::Append)
            }
            Some(TokenKind::MinusEq) => {
                self.advance();
                Ok(Operator::Remove)
            }
            Some(TokenKind::PlusPlus) => {
                self.advance();
                Ok(Operator::Increment)
            }
            Some(TokenKind::MinusMinus) => {
                self.advance();
                Ok(Operator::Decrement)
            }
            _ => Err(self.error("operator")),
        }
    }

    fn parse_value(&mut self) -> ParseResult<Value> {
        match self.peek_kind() {
            Some(TokenKind::Null) => {
                self.advance();
                Ok(Value::Null)
            }
            Some(TokenKind::True) => {
                self.advance();
                Ok(Value::Bool(true))
            }
            Some(TokenKind::False) => {
                self.advance();
                Ok(Value::Bool(false))
            }
            Some(TokenKind::Integer(n)) => {
                self.advance();
                Ok(Value::Number(n as f64))
            }
            Some(TokenKind::Float(n)) => {
                self.advance();
                Ok(Value::Number(n))
            }
            Some(TokenKind::DoubleString(s))
            | Some(TokenKind::SingleString(s))
            | Some(TokenKind::TripleString(s)) => {
                self.advance();
                Ok(Value::String(s))
            }
            Some(TokenKind::At_) => {
                self.advance();
                Ok(Value::BlockRef(self.expect_block_id()?))
            }
            Some(TokenKind::LBracket) => self.parse_array(),
            Some(TokenKind::LBrace) => self.parse_object(),
            _ => Err(self.error("value")),
        }
    }

    fn parse_array(&mut self) -> ParseResult<Value> {
        self.expect(TokenKind::LBracket)?;
        let mut arr = Vec::new();
        while !self.check(TokenKind::RBracket) && !self.is_at_end() {
            arr.push(self.parse_value()?);
            if !self.check(TokenKind::RBracket) {
                let _ = self.expect(TokenKind::Comma);
            }
        }
        self.expect(TokenKind::RBracket)?;
        Ok(Value::Array(arr))
    }

    fn parse_object(&mut self) -> ParseResult<Value> {
        self.expect(TokenKind::LBrace)?;
        let mut m = HashMap::new();
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            let k = self.expect_str()?;
            self.expect(TokenKind::Colon)?;
            m.insert(k, self.parse_value()?);
            if !self.check(TokenKind::RBrace) {
                let _ = self.expect(TokenKind::Comma);
            }
        }
        self.expect(TokenKind::RBrace)?;
        Ok(Value::Object(m))
    }

    fn parse_cond(&mut self) -> ParseResult<Condition> {
        self.parse_or()
    }
    fn parse_or(&mut self) -> ParseResult<Condition> {
        let mut l = self.parse_and()?;
        while self.check(TokenKind::Or) {
            self.advance();
            l = Condition::Or(Box::new(l), Box::new(self.parse_and()?));
        }
        Ok(l)
    }
    fn parse_and(&mut self) -> ParseResult<Condition> {
        let mut l = self.parse_unary()?;
        while self.check(TokenKind::And) {
            self.advance();
            l = Condition::And(Box::new(l), Box::new(self.parse_unary()?));
        }
        Ok(l)
    }
    fn parse_unary(&mut self) -> ParseResult<Condition> {
        if self.check(TokenKind::Not) {
            self.advance();
            return Ok(Condition::Not(Box::new(self.parse_unary()?)));
        }
        if self.check(TokenKind::LParen) {
            self.advance();
            let c = self.parse_cond()?;
            self.expect(TokenKind::RParen)?;
            return Ok(c);
        }
        self.parse_comp()
    }
    fn parse_comp(&mut self) -> ParseResult<Condition> {
        let p = self.parse_path()?;
        match self.peek_kind() {
            Some(TokenKind::Eq) => {
                self.advance();
                Ok(Condition::Comparison {
                    path: p,
                    op: ComparisonOp::Eq,
                    value: self.parse_value()?,
                })
            }
            Some(TokenKind::Ne) => {
                self.advance();
                Ok(Condition::Comparison {
                    path: p,
                    op: ComparisonOp::Ne,
                    value: self.parse_value()?,
                })
            }
            Some(TokenKind::Gt) => {
                self.advance();
                Ok(Condition::Comparison {
                    path: p,
                    op: ComparisonOp::Gt,
                    value: self.parse_value()?,
                })
            }
            Some(TokenKind::Ge) => {
                self.advance();
                Ok(Condition::Comparison {
                    path: p,
                    op: ComparisonOp::Ge,
                    value: self.parse_value()?,
                })
            }
            Some(TokenKind::Lt) => {
                self.advance();
                Ok(Condition::Comparison {
                    path: p,
                    op: ComparisonOp::Lt,
                    value: self.parse_value()?,
                })
            }
            Some(TokenKind::Le) => {
                self.advance();
                Ok(Condition::Comparison {
                    path: p,
                    op: ComparisonOp::Le,
                    value: self.parse_value()?,
                })
            }
            Some(TokenKind::Contains) => {
                self.advance();
                Ok(Condition::Contains {
                    path: p,
                    value: self.parse_value()?,
                })
            }
            Some(TokenKind::StartsWith) => {
                self.advance();
                Ok(Condition::StartsWith {
                    path: p,
                    prefix: self.expect_str()?,
                })
            }
            Some(TokenKind::EndsWith) => {
                self.advance();
                Ok(Condition::EndsWith {
                    path: p,
                    suffix: self.expect_str()?,
                })
            }
            Some(TokenKind::Matches) => {
                self.advance();
                Ok(Condition::Matches {
                    path: p,
                    regex: self.expect_str()?,
                })
            }
            Some(TokenKind::Exists) => {
                self.advance();
                Ok(Condition::Exists { path: p })
            }
            Some(TokenKind::IsNull) => {
                self.advance();
                Ok(Condition::IsNull { path: p })
            }
            _ => Err(self.error("comparison")),
        }
    }

    fn parse_content_literal(&mut self) -> ParseResult<String> {
        match self.peek_kind() {
            Some(TokenKind::DoubleString(s))
            | Some(TokenKind::SingleString(s))
            | Some(TokenKind::TripleString(s)) => {
                self.advance();
                Ok(s)
            }
            Some(TokenKind::CodeBlock(s)) | Some(TokenKind::TableLiteral(s)) => {
                self.advance();
                Ok(s)
            }
            Some(TokenKind::LBrace) => {
                let o = self.parse_object()?;
                Ok(serde_json::to_string(&o.to_json()).unwrap_or_default())
            }
            _ => Err(self.error("content")),
        }
    }

    fn parse_content_type(&mut self) -> ParseResult<ContentType> {
        self.try_content_type()
            .ok_or_else(|| self.error("content type"))
    }
    fn try_content_type(&mut self) -> Option<ContentType> {
        match self.peek_kind() {
            Some(TokenKind::TextType) => {
                self.advance();
                Some(ContentType::Text)
            }
            Some(TokenKind::TableType) => {
                self.advance();
                Some(ContentType::Table)
            }
            Some(TokenKind::CodeType) => {
                self.advance();
                Some(ContentType::Code)
            }
            Some(TokenKind::MathType) => {
                self.advance();
                Some(ContentType::Math)
            }
            Some(TokenKind::MediaType) => {
                self.advance();
                Some(ContentType::Media)
            }
            Some(TokenKind::JsonType) => {
                self.advance();
                Some(ContentType::Json)
            }
            Some(TokenKind::BinaryType) => {
                self.advance();
                Some(ContentType::Binary)
            }
            Some(TokenKind::CompositeType) => {
                self.advance();
                Some(ContentType::Composite)
            }
            _ => None,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    fn peek_kind(&self) -> Option<TokenKind> {
        self.peek().map(|t| t.kind.clone())
    }
    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.pos += 1;
        }
        self.tokens.get(self.pos - 1)
    }
    fn check(&self, k: TokenKind) -> bool {
        self.peek_kind() == Some(k)
    }
    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }
    fn expect(&mut self, k: TokenKind) -> ParseResult<&Token> {
        if self.check(k.clone()) {
            Ok(self.advance().unwrap())
        } else {
            Err(self.error(&format!("{:?}", k)))
        }
    }
    fn expect_block_id(&mut self) -> ParseResult<String> {
        if matches!(self.peek_kind(), Some(TokenKind::BlockId)) {
            let span = self.tokens[self.pos].span.clone();
            self.advance();
            Ok(self.source[span].to_string())
        } else {
            Err(self.error("block ID"))
        }
    }
    fn expect_ident(&mut self) -> ParseResult<String> {
        if matches!(self.peek_kind(), Some(TokenKind::Identifier)) {
            let span = self.tokens[self.pos].span.clone();
            self.advance();
            Ok(self.source[span].to_string())
        } else {
            Err(self.error("identifier"))
        }
    }
    fn is_ident_or_keyword(&self) -> bool {
        matches!(
            self.peek_kind(),
            Some(TokenKind::Identifier)
                | Some(TokenKind::TextType)
                | Some(TokenKind::TableType)
                | Some(TokenKind::CodeType)
                | Some(TokenKind::MathType)
                | Some(TokenKind::MediaType)
                | Some(TokenKind::JsonType)
                | Some(TokenKind::BinaryType)
                | Some(TokenKind::CompositeType)
                | Some(TokenKind::True)
                | Some(TokenKind::False)
                | Some(TokenKind::Null)
                // Allow keywords to be used as property names
                | Some(TokenKind::Label)
                | Some(TokenKind::Role)
                | Some(TokenKind::Tag)
                | Some(TokenKind::Tags)
                | Some(TokenKind::Mode)
                | Some(TokenKind::Depth)
                | Some(TokenKind::Limit)
                | Some(TokenKind::Max)
                | Some(TokenKind::Format)
                | Some(TokenKind::Method)
                | Some(TokenKind::Reason)
                | Some(TokenKind::Relevance)
                | Some(TokenKind::Pattern)
                | Some(TokenKind::Full)
                | Some(TokenKind::Preview)
                | Some(TokenKind::MetadataToken)
        )
    }
    fn expect_ident_or_keyword(&mut self) -> ParseResult<String> {
        if self.is_ident_or_keyword() {
            let span = self.tokens[self.pos].span.clone();
            self.advance();
            Ok(self.source[span].to_string())
        } else {
            Err(self.error("identifier"))
        }
    }
    fn is_path_property_start(&self) -> bool {
        self.is_ident_or_keyword()
            || matches!(
                self.peek_kind(),
                Some(TokenKind::DoubleString(_)) | Some(TokenKind::SingleString(_))
            )
    }
    fn expect_path_property(&mut self) -> ParseResult<String> {
        match self.peek_kind() {
            Some(TokenKind::DoubleString(s)) | Some(TokenKind::SingleString(s)) => {
                self.advance();
                Ok(s)
            }
            _ => self.expect_ident_or_keyword(),
        }
    }
    fn expect_str(&mut self) -> ParseResult<String> {
        match self.peek_kind() {
            Some(TokenKind::DoubleString(s))
            | Some(TokenKind::SingleString(s))
            | Some(TokenKind::TripleString(s)) => {
                self.advance();
                Ok(s)
            }
            _ => Err(self.error("string")),
        }
    }
    fn expect_int(&mut self) -> ParseResult<i64> {
        if let Some(TokenKind::Integer(n)) = self.peek_kind() {
            self.advance();
            Ok(n)
        } else {
            Err(self.error("integer"))
        }
    }

    fn expect_eq_with_hint(&mut self, hint: &str) -> ParseResult<()> {
        if self.check(TokenKind::Eq) {
            self.advance();
            Ok(())
        } else {
            Err(self.error_with_hint(hint))
        }
    }
    fn try_str(&mut self) -> Option<String> {
        match self.peek_kind() {
            Some(TokenKind::DoubleString(s)) | Some(TokenKind::SingleString(s)) => {
                self.advance();
                Some(s)
            }
            _ => None,
        }
    }
    fn is_section_header(&self) -> bool {
        matches!(
            self.peek_kind(),
            Some(TokenKind::Structure) | Some(TokenKind::Blocks) | Some(TokenKind::Commands)
        )
    }
    fn is_cmd_start(&self) -> bool {
        matches!(
            self.peek_kind(),
            // Document modification commands
            Some(TokenKind::Edit)
                | Some(TokenKind::Move)
                | Some(TokenKind::Append)
                | Some(TokenKind::Delete)
                | Some(TokenKind::Prune)
                | Some(TokenKind::Fold)
                | Some(TokenKind::Link)
                | Some(TokenKind::Unlink)
                | Some(TokenKind::Snapshot)
                | Some(TokenKind::Begin)
                | Some(TokenKind::Commit)
                | Some(TokenKind::Rollback)
                | Some(TokenKind::Atomic)
                | Some(TokenKind::WriteSection)
                // Agent traversal commands
                | Some(TokenKind::Goto)
                | Some(TokenKind::Back)
                | Some(TokenKind::Expand)
                | Some(TokenKind::Follow)
                | Some(TokenKind::Path)
                | Some(TokenKind::Search)
                | Some(TokenKind::Find)
                | Some(TokenKind::View)
                // Context commands
                | Some(TokenKind::Ctx)
        )
    }
    fn error(&self, exp: &str) -> ParseError {
        let (l, c, f) = self
            .peek()
            .map(|t| (t.line, t.column, format!("{:?}", t.kind)))
            .unwrap_or((0, 0, "EOF".into()));
        ParseError::UnexpectedToken {
            expected: exp.into(),
            found: f,
            line: l,
            column: c,
        }
    }
    fn error_with_hint(&self, message: &str) -> ParseError {
        let line = self.peek().map(|t| t.line).unwrap_or(0);
        ParseError::InvalidSyntax {
            message: message.into(),
            line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_edit() {
        let r = Parser::new(r#"EDIT blk_abc123def456 SET name = "hello""#).parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
    }

    #[test]
    fn test_parse_edit_with_keyword_path() {
        // Test that keywords like 'text' work as identifiers in paths
        let r = Parser::new(r#"EDIT blk_abc123def456 SET content.text = "hello""#)
            .parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
    }

    // ========================================================================
    // Agent Traversal Command Tests
    // ========================================================================

    #[test]
    fn test_parse_goto() {
        let r = Parser::new("GOTO blk_abc123def456").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        let cmds = r.unwrap();
        assert_eq!(cmds.len(), 1);
        match &cmds[0] {
            Command::Goto(cmd) => assert_eq!(cmd.block_id, "blk_abc123def456"),
            _ => panic!("Expected Goto command"),
        }
    }

    #[test]
    fn test_parse_back() {
        // Default steps
        let r = Parser::new("BACK").parse_commands_only();
        assert!(r.is_ok());
        match &r.unwrap()[0] {
            Command::Back(cmd) => assert_eq!(cmd.steps, 1),
            _ => panic!("Expected Back command"),
        }

        // Custom steps
        let r = Parser::new("BACK 3").parse_commands_only();
        assert!(r.is_ok());
        match &r.unwrap()[0] {
            Command::Back(cmd) => assert_eq!(cmd.steps, 3),
            _ => panic!("Expected Back command"),
        }
    }

    #[test]
    fn test_parse_expand() {
        let r =
            Parser::new("EXPAND blk_abc123def456 DOWN DEPTH=3 MODE=PREVIEW").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::Expand(cmd) => {
                assert_eq!(cmd.block_id, "blk_abc123def456");
                assert_eq!(cmd.direction, ExpandDirection::Down);
                assert_eq!(cmd.depth, 3);
                assert!(matches!(cmd.mode, Some(ViewMode::Preview { .. })));
            }
            _ => panic!("Expected Expand command"),
        }
    }

    #[test]
    fn test_parse_expand_semantic() {
        let r = Parser::new("EXPAND blk_abc123def456 SEMANTIC").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::Expand(cmd) => {
                assert_eq!(cmd.direction, ExpandDirection::Semantic);
            }
            _ => panic!("Expected Expand command"),
        }
    }

    #[test]
    fn test_parse_follow() {
        let r = Parser::new("FOLLOW blk_abc123def456 references").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::Follow(cmd) => {
                assert_eq!(cmd.source_id, "blk_abc123def456");
                assert_eq!(cmd.edge_types, vec!["references"]);
                assert!(cmd.target_id.is_none());
            }
            _ => panic!("Expected Follow command"),
        }
    }

    #[test]
    fn test_parse_path_find() {
        let r = Parser::new("PATH blk_abc123def456 TO blk_111222333444").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::Path(cmd) => {
                assert_eq!(cmd.from_id, "blk_abc123def456");
                assert_eq!(cmd.to_id, "blk_111222333444");
                assert!(cmd.max_length.is_none());
            }
            _ => panic!("Expected Path command"),
        }
    }

    #[test]
    fn test_parse_search() {
        let r = Parser::new(r#"SEARCH "authentication flow" LIMIT=10"#).parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::Search(cmd) => {
                assert_eq!(cmd.query, "authentication flow");
                assert_eq!(cmd.limit, Some(10));
            }
            _ => panic!("Expected Search command"),
        }
    }

    #[test]
    fn test_parse_find() {
        let r = Parser::new(r#"FIND ROLE=heading1 TAG="important""#).parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::Find(cmd) => {
                assert_eq!(cmd.role, Some("heading1".to_string()));
                assert_eq!(cmd.tag, Some("important".to_string()));
            }
            _ => panic!("Expected Find command"),
        }
    }

    #[test]
    fn test_parse_view_block() {
        let r = Parser::new("VIEW blk_abc123def456 MODE=METADATA").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::View(cmd) => {
                assert!(
                    matches!(cmd.target, ViewTarget::Block(ref id) if id == "blk_abc123def456")
                );
                assert!(matches!(cmd.mode, ViewMode::Metadata));
            }
            _ => panic!("Expected View command"),
        }
    }

    #[test]
    fn test_parse_view_neighborhood() {
        let r = Parser::new("VIEW NEIGHBORHOOD DEPTH=2").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::View(cmd) => {
                assert!(matches!(cmd.target, ViewTarget::Neighborhood));
                assert_eq!(cmd.depth, Some(2));
            }
            _ => panic!("Expected View command"),
        }
    }

    // ========================================================================
    // Context Command Tests
    // ========================================================================

    #[test]
    fn test_parse_ctx_add_block() {
        let r = Parser::new(r#"CTX ADD blk_abc123def456 REASON="semantic_relevance""#)
            .parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::Context(ContextCommand::Add(cmd)) => {
                assert!(
                    matches!(cmd.target, ContextAddTarget::Block(ref id) if id == "blk_abc123def456")
                );
                assert_eq!(cmd.reason, Some("semantic_relevance".to_string()));
            }
            _ => panic!("Expected CTX ADD command"),
        }
    }

    #[test]
    fn test_parse_ctx_add_results() {
        let r = Parser::new("CTX ADD RESULTS").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::Context(ContextCommand::Add(cmd)) => {
                assert!(matches!(cmd.target, ContextAddTarget::Results));
            }
            _ => panic!("Expected CTX ADD RESULTS command"),
        }
    }

    #[test]
    fn test_parse_ctx_add_children() {
        let r = Parser::new("CTX ADD CHILDREN blk_abc123def456").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::Context(ContextCommand::Add(cmd)) => {
                assert!(
                    matches!(cmd.target, ContextAddTarget::Children { ref parent_id } if parent_id == "blk_abc123def456")
                );
            }
            _ => panic!("Expected CTX ADD CHILDREN command"),
        }
    }

    #[test]
    fn test_parse_ctx_remove() {
        let r = Parser::new("CTX REMOVE blk_abc123def456").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::Context(ContextCommand::Remove { block_id }) => {
                assert_eq!(block_id, "blk_abc123def456");
            }
            _ => panic!("Expected CTX REMOVE command"),
        }
    }

    #[test]
    fn test_parse_ctx_clear() {
        let r = Parser::new("CTX CLEAR").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        assert!(matches!(
            r.unwrap()[0],
            Command::Context(ContextCommand::Clear)
        ));
    }

    #[test]
    fn test_parse_ctx_expand() {
        let r = Parser::new("CTX EXPAND SEMANTIC DEPTH=2").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::Context(ContextCommand::Expand(cmd)) => {
                assert_eq!(cmd.direction, ExpandDirection::Semantic);
                assert_eq!(cmd.depth, Some(2));
            }
            _ => panic!("Expected CTX EXPAND command"),
        }
    }

    #[test]
    fn test_parse_ctx_compress() {
        let r = Parser::new("CTX COMPRESS METHOD=TRUNCATE").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::Context(ContextCommand::Compress { method }) => {
                assert_eq!(*method, CompressionMethod::Truncate);
            }
            _ => panic!("Expected CTX COMPRESS command"),
        }
    }

    #[test]
    fn test_parse_ctx_prune() {
        let r = Parser::new("CTX PRUNE RELEVANCE=0.3 MAX_AGE=300").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::Context(ContextCommand::Prune(cmd)) => {
                assert_eq!(cmd.min_relevance, Some(0.3));
                assert_eq!(cmd.max_age_secs, Some(300));
            }
            _ => panic!("Expected CTX PRUNE command"),
        }
    }

    #[test]
    fn test_parse_ctx_render() {
        let r = Parser::new("CTX RENDER FORMAT=SHORT_IDS").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::Context(ContextCommand::Render { format }) => {
                assert_eq!(*format, Some(RenderFormat::ShortIds));
            }
            _ => panic!("Expected CTX RENDER command"),
        }
    }

    #[test]
    fn test_parse_ctx_stats() {
        let r = Parser::new("CTX STATS").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        assert!(matches!(
            r.unwrap()[0],
            Command::Context(ContextCommand::Stats)
        ));
    }

    #[test]
    fn test_parse_ctx_focus() {
        let r = Parser::new("CTX FOCUS blk_abc123def456").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::Context(ContextCommand::Focus { block_id }) => {
                assert_eq!(*block_id, Some("blk_abc123def456".to_string()));
            }
            _ => panic!("Expected CTX FOCUS command"),
        }
    }

    #[test]
    fn test_parse_ctx_focus_clear() {
        let r = Parser::new("CTX FOCUS CLEAR").parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        match &r.unwrap()[0] {
            Command::Context(ContextCommand::Focus { block_id }) => {
                assert!(block_id.is_none());
            }
            _ => panic!("Expected CTX FOCUS CLEAR command"),
        }
    }

    #[test]
    fn test_parse_multiple_commands() {
        let input = r#"
            GOTO blk_abc123def456
            EXPAND blk_abc123def456 DOWN DEPTH=2
            CTX ADD RESULTS
            CTX RENDER FORMAT=SHORT_IDS
        "#;
        let r = Parser::new(input).parse_commands_only();
        assert!(r.is_ok(), "Parse error: {:?}", r.err());
        assert_eq!(r.unwrap().len(), 4);
    }
}
