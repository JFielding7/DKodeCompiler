use crate::compiler_context::CompilerContext;
use crate::error::spanned_error::{SpannableError, CompilerError};
use crate::lexer::token::{Token, TokenType};

#[derive(thiserror::Error, Debug)]
pub enum SyntaxError {

    #[error("")]
    ExpectedToken(TokenType, Option<Token>),

    #[error("Error: Unexpected Expression")]
    UnexpectedExpression,

    #[error("Error: Unmatched {0}")]
    UnmatchedGroupOpening(TokenType),

    #[error("Error: Invalid Expression")]
    InvalidExpression,

    #[error("Error: Line indented too far in")]
    IndentTooLarge,
}

impl SpannableError for SyntaxError {
    fn format(&self, ctx: CompilerContext) -> String {
        use SyntaxError::*;

        match self {
            ExpectedToken(expected, actual) => {
                match actual {
                    None => format!("Error: {expected} expected"),
                    Some(token) => format!(
                        "Error: {expected} expected but got '{}'", 
                        ctx.string_interner.get_str(token.symbol)
                    ),
                }
            }
            _ => format!("{self}"),
        }
    }
}
