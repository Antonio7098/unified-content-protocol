//! Lexer for UCL using Logos.

use logos::Logos;

/// Token kinds
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t]+")]
pub enum TokenKind {
    // Section headers
    #[token("STRUCTURE")]
    Structure,
    #[token("BLOCKS")]
    Blocks,
    #[token("COMMANDS")]
    Commands,

    // Commands
    #[token("EDIT")]
    Edit,
    #[token("SET")]
    Set,
    #[token("MOVE")]
    Move,
    #[token("TO")]
    To,
    #[token("AT")]
    At,
    #[token("BEFORE")]
    Before,
    #[token("AFTER")]
    After,
    #[token("SWAP")]
    Swap,
    #[token("APPEND")]
    Append,
    #[token("WITH")]
    With,
    #[token("DELETE")]
    Delete,
    #[token("CASCADE")]
    Cascade,
    #[token("PRESERVE_CHILDREN")]
    PreserveChildren,
    #[token("PRUNE")]
    Prune,
    #[token("UNREACHABLE")]
    Unreachable,
    #[token("WHERE")]
    Where,
    #[token("DRY_RUN")]
    DryRun,
    #[token("FOLD")]
    Fold,
    #[token("DEPTH")]
    Depth,
    #[token("MAX_TOKENS")]
    MaxTokens,
    #[token("PRESERVE_TAGS")]
    PreserveTags,
    #[token("LINK")]
    Link,
    #[token("UNLINK")]
    Unlink,
    #[token("SNAPSHOT")]
    Snapshot,
    #[token("CREATE")]
    Create,
    #[token("RESTORE")]
    Restore,
    #[token("LIST")]
    List,
    #[token("DIFF")]
    Diff,
    #[token("BEGIN")]
    Begin,
    #[token("TRANSACTION")]
    Transaction,
    #[token("COMMIT")]
    Commit,
    #[token("ROLLBACK")]
    Rollback,
    #[token("ATOMIC")]
    Atomic,
    #[token("VIEW")]
    View,
    #[token("FOLDED")]
    Folded,
    #[token("FROM")]
    From,
    #[token("TEMPLATE")]
    Template,
    #[token("FIRST")]
    First,
    #[token("LAST")]
    Last,

    // Operators
    #[token("=")]
    Eq,
    #[token("!=")]
    Ne,
    #[token(">")]
    Gt,
    #[token(">=")]
    Ge,
    #[token("<")]
    Lt,
    #[token("<=")]
    Le,
    #[token("+=")]
    PlusEq,
    #[token("-=")]
    MinusEq,
    #[token("++")]
    PlusPlus,
    #[token("--")]
    MinusMinus,

    // Logic
    #[token("AND")]
    And,
    #[token("OR")]
    Or,
    #[token("NOT")]
    Not,
    #[token("CONTAINS")]
    Contains,
    #[token("STARTS_WITH")]
    StartsWith,
    #[token("ENDS_WITH")]
    EndsWith,
    #[token("MATCHES")]
    Matches,
    #[token("EXISTS")]
    Exists,
    #[token("IS_NULL")]
    IsNull,
    #[token("IS_NOT_NULL")]
    IsNotNull,
    #[token("IS_EMPTY")]
    IsEmpty,
    #[token("LENGTH")]
    Length,

    // Punctuation
    #[token("::")]
    DoubleColon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("#")]
    Hash,
    #[token("@")]
    At_,
    #[token("$")]
    Dollar,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    // Content types
    #[token("text")]
    TextType,
    #[token("table")]
    TableType,
    #[token("code")]
    CodeType,
    #[token("math")]
    MathType,
    #[token("media")]
    MediaType,
    #[token("json")]
    JsonType,
    #[token("binary")]
    BinaryType,
    #[token("composite")]
    CompositeType,

    // Literals
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("null")]
    Null,

    // Identifier (block IDs, property names)
    #[regex(r"blk_[a-fA-F0-9]+")]
    BlockId,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // Numbers
    #[regex(r"-?[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    Float(f64),

    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    Integer(i64),

    // Strings
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        Some(s[1..s.len()-1].to_string())
    })]
    DoubleString(String),

    #[regex(r#"'([^'\\]|\\.)*'"#, |lex| {
        let s = lex.slice();
        Some(s[1..s.len()-1].to_string())
    })]
    SingleString(String),

    // Triple-quoted strings handled via callback in parser
    TripleString(String),

    // Code blocks handled via callback in parser
    CodeBlock(String),

    // Table literal
    #[regex(r"\|[^\n]+\|(\n\|[^\n]+\|)+", |lex| {
        Some(lex.slice().to_string())
    })]
    TableLiteral(String),

    // Newline
    #[regex(r"\n")]
    Newline,

    // Comment - use // style to avoid conflict with # delimiter in block definitions
    #[regex(r"//[^\n]*")]
    Comment,
}

/// Token with position information
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: std::ops::Range<usize>,
    pub line: usize,
    pub column: usize,
}

/// Lexer wrapper that tracks position
pub struct Lexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
    line: usize,
    column: usize,
    last_newline_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: TokenKind::lexer(input),
            line: 1,
            column: 1,
            last_newline_pos: 0,
        }
    }

    pub fn source(&self) -> &'a str {
        self.inner.source()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let kind = self.inner.next()?;
            let span = self.inner.span();

            // Update line/column tracking
            let source = self.inner.source();
            for c in source[self.last_newline_pos..span.start].chars() {
                if c == '\n' {
                    self.line += 1;
                    self.column = 1;
                    self.last_newline_pos = span.start;
                } else {
                    self.column += 1;
                }
            }

            match kind {
                Ok(TokenKind::Comment) => continue, // Skip comments
                Ok(TokenKind::Newline) => {
                    self.line += 1;
                    self.column = 1;
                    self.last_newline_pos = span.end;
                    // Return newline token for line-aware parsing
                    return Some(Ok(Token {
                        kind: TokenKind::Newline,
                        span,
                        line: self.line - 1,
                        column: 1,
                    }));
                }
                Ok(kind) => {
                    return Some(Ok(Token {
                        kind,
                        span,
                        line: self.line,
                        column: self.column,
                    }));
                }
                Err(_) => return Some(Err(())),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_structure() {
        let input = "STRUCTURE\nblk_abc123def456: [blk_111222333444]";
        let lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer.filter_map(|r| r.ok()).collect();

        assert!(matches!(tokens[0].kind, TokenKind::Structure));
        assert!(matches!(tokens[2].kind, TokenKind::BlockId));
    }

    #[test]
    fn test_lex_edit_command() {
        let input = r#"EDIT blk_abc123def456 SET content.text = "hello""#;
        let lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer.filter_map(|r| r.ok()).collect();

        assert!(matches!(tokens[0].kind, TokenKind::Edit));
        assert!(matches!(tokens[1].kind, TokenKind::BlockId));
        assert!(matches!(tokens[2].kind, TokenKind::Set));
    }

    #[test]
    fn test_lex_string_types() {
        let input = r#""double" 'single'"#;
        let lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer.filter_map(|r| r.ok()).collect();

        assert!(matches!(tokens[0].kind, TokenKind::DoubleString(_)));
        assert!(matches!(tokens[1].kind, TokenKind::SingleString(_)));
    }

    #[test]
    fn test_lex_operators() {
        let input = "= += -= != >= <=";
        let lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer.filter_map(|r| r.ok()).collect();

        assert!(matches!(tokens[0].kind, TokenKind::Eq));
        assert!(matches!(tokens[1].kind, TokenKind::PlusEq));
        assert!(matches!(tokens[2].kind, TokenKind::MinusEq));
        assert!(matches!(tokens[3].kind, TokenKind::Ne));
    }
}
