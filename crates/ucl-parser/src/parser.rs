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
}
