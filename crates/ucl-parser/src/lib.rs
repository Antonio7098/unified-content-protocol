//! # UCL Parser
//!
//! Parser for the Unified Content Language (UCL).
//!
//! UCL is a token-efficient command language for manipulating UCM documents.
//!
//! ## Document Structure
//!
//! ```text
//! STRUCTURE
//! <adjacency declarations>
//!
//! BLOCKS
//! <block definitions>
//!
//! COMMANDS
//! <transformation commands>
//! ```

pub mod ast;
pub mod lexer;
pub mod parser;

pub use ast::*;
pub use lexer::{Token, TokenKind};
pub use parser::{ParseError, ParseResult, Parser};

/// Parse a UCL document string
pub fn parse(input: &str) -> ParseResult<UclDocument> {
    let mut parser = Parser::new(input);
    parser.parse_document()
}

/// Parse UCL commands only (without STRUCTURE/BLOCKS sections)
pub fn parse_commands(input: &str) -> ParseResult<Vec<Command>> {
    let mut parser = Parser::new(input);
    parser.parse_commands_only()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_document() {
        let input = r#"
STRUCTURE
blk_000000000000: [blk_111111111111]

BLOCKS
text #blk_111111111111 label="Introduction" :: "Hello, world!"

COMMANDS
EDIT blk_111111111111 SET content.text = "Updated text"
"#;

        let doc = parse(input).unwrap();
        assert!(!doc.structure.is_empty());
        assert!(!doc.blocks.is_empty());
        assert!(!doc.commands.is_empty());
    }
}
