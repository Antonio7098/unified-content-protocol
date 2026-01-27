//! Lexer for UCL using Logos.

use logos::Logos;

/// Token kinds
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t]+")]
pub enum TokenKind {
    // Section headers (case-insensitive)
    #[regex("(?i)STRUCTURE")]
    Structure,
    #[regex("(?i)BLOCKS")]
    Blocks,
    #[regex("(?i)COMMANDS")]
    Commands,

    // Commands (case-insensitive)
    #[regex("(?i)EDIT")]
    Edit,
    #[regex("(?i)SET")]
    Set,
    #[regex("(?i)MOVE")]
    Move,
    #[regex("(?i)TO")]
    To,
    #[regex("(?i)AT")]
    At,
    #[regex("(?i)BEFORE")]
    Before,
    #[regex("(?i)AFTER")]
    After,
    #[regex("(?i)SWAP")]
    Swap,
    #[regex("(?i)APPEND")]
    Append,
    #[regex("(?i)WITH")]
    With,
    #[regex("(?i)DELETE")]
    Delete,
    #[regex("(?i)CASCADE")]
    Cascade,
    #[regex("(?i)PRESERVE_CHILDREN")]
    PreserveChildren,
    #[regex("(?i)PRUNE")]
    Prune,
    #[regex("(?i)UNREACHABLE")]
    Unreachable,
    #[regex("(?i)WHERE")]
    Where,
    #[regex("(?i)DRY_RUN")]
    DryRun,
    #[regex("(?i)FOLD")]
    Fold,
    #[regex("(?i)DEPTH")]
    Depth,
    #[regex("(?i)MAX_TOKENS")]
    MaxTokens,
    #[regex("(?i)PRESERVE_TAGS")]
    PreserveTags,
    #[regex("(?i)LINK")]
    Link,
    #[regex("(?i)UNLINK")]
    Unlink,
    #[regex("(?i)SNAPSHOT")]
    Snapshot,
    #[regex("(?i)CREATE")]
    Create,
    #[regex("(?i)RESTORE")]
    Restore,
    #[regex("(?i)LIST")]
    List,
    #[regex("(?i)DIFF")]
    Diff,
    #[regex("(?i)BEGIN")]
    Begin,
    #[regex("(?i)TRANSACTION")]
    Transaction,
    #[regex("(?i)COMMIT")]
    Commit,
    #[regex("(?i)ROLLBACK")]
    Rollback,
    #[regex("(?i)ATOMIC")]
    Atomic,
    #[regex("(?i)VIEW")]
    View,
    #[regex("(?i)FOLDED")]
    Folded,
    #[regex("(?i)FROM")]
    From,
    #[regex("(?i)TEMPLATE")]
    Template,
    #[regex("(?i)FIRST")]
    First,
    #[regex("(?i)LAST")]
    Last,
    #[regex("(?i)WRITE_SECTION")]
    WriteSection,
    #[regex("(?i)BASE_LEVEL")]
    BaseLevel,

    // Agent traversal commands (case-insensitive)
    #[regex("(?i)GOTO")]
    Goto,
    #[regex("(?i)BACK")]
    Back,
    #[regex("(?i)EXPAND")]
    Expand,
    #[regex("(?i)FOLLOW")]
    Follow,
    #[regex("(?i)PATH")]
    Path,
    #[regex("(?i)SEARCH")]
    Search,
    #[regex("(?i)FIND")]
    Find,
    #[regex("(?i)CTX")]
    Ctx,

    // Traversal directions (case-insensitive)
    #[regex("(?i)DOWN")]
    Down,
    #[regex("(?i)UP")]
    Up,
    #[regex("(?i)SEMANTIC")]
    Semantic,

    // Traversal options (case-insensitive)
    #[regex("(?i)MODE")]
    Mode,
    #[regex("(?i)LIMIT")]
    Limit,
    #[regex("(?i)MIN_SIMILARITY")]
    MinSimilarity,
    #[regex("(?i)ROLES")]
    Roles,
    #[regex("(?i)TAGS")]
    Tags,
    #[regex("(?i)ROLE")]
    Role,
    #[regex("(?i)TAG")]
    Tag,
    #[regex("(?i)LABEL")]
    Label,
    #[regex("(?i)PATTERN")]
    Pattern,
    #[regex("(?i)MAX")]
    Max,
    #[regex("(?i)NEIGHBORHOOD")]
    Neighborhood,

    // Context commands (case-insensitive)
    #[regex("(?i)ADD")]
    Add,
    #[regex("(?i)REMOVE")]
    Remove,
    #[regex("(?i)CLEAR")]
    Clear,
    #[regex("(?i)COMPRESS")]
    Compress,
    #[regex("(?i)RENDER")]
    Render,
    #[regex("(?i)STATS")]
    Stats,
    #[regex("(?i)FOCUS")]
    Focus,
    #[regex("(?i)RESULTS")]
    Results,
    #[regex("(?i)CHILDREN")]
    Children,
    #[regex("(?i)AUTO")]
    Auto,
    #[regex("(?i)TOKENS")]
    Tokens,
    #[regex("(?i)MAX_AGE")]
    MaxAge,
    #[regex("(?i)RELEVANCE")]
    Relevance,
    #[regex("(?i)REASON")]
    Reason,
    #[regex("(?i)METHOD")]
    Method,
    #[regex("(?i)FORMAT")]
    Format,
    #[regex("(?i)TRUNCATE")]
    Truncate,
    #[regex("(?i)SUMMARIZE")]
    Summarize,
    #[regex("(?i)STRUCTURE_ONLY")]
    StructureOnly,
    #[regex("(?i)SHORT_IDS")]
    ShortIds,
    #[regex("(?i)MARKDOWN")]
    Markdown,
    #[regex("(?i)FULL")]
    Full,
    #[regex("(?i)PREVIEW")]
    Preview,
    #[regex("(?i)METADATA")]
    MetadataToken,
    #[regex("(?i)IDS")]
    Ids,
    #[regex("(?i)BOTH")]
    Both,

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

    // Logic (case-insensitive)
    #[regex("(?i)AND")]
    And,
    #[regex("(?i)OR")]
    Or,
    #[regex("(?i)NOT")]
    Not,
    #[regex("(?i)CONTAINS")]
    Contains,
    #[regex("(?i)STARTS_WITH")]
    StartsWith,
    #[regex("(?i)ENDS_WITH")]
    EndsWith,
    #[regex("(?i)MATCHES")]
    Matches,
    #[regex("(?i)EXISTS")]
    Exists,
    #[regex("(?i)IS_NULL")]
    IsNull,
    #[regex("(?i)IS_NOT_NULL")]
    IsNotNull,
    #[regex("(?i)IS_EMPTY")]
    IsEmpty,
    #[regex("(?i)LENGTH")]
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
