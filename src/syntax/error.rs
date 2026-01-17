use crate::compiler_context::CompilerContext;
use crate::error::compiler_error::SpannableError;
use crate::lexer::token::{Token, TokenType};

#[derive(Debug)]
pub enum SyntaxError {
    ExpectedToken {
        expected: TokenType,
        actual: Option<Token>,
    },

    UnmatchedGroupOpening(TokenType),

    IndentTooLarge,

    UnexpectedExpression,

    ExpressionExpected,

    InvalidExpression,
}

impl SpannableError for SyntaxError {
    fn format(&self, ctx: &CompilerContext) -> String {
        use SyntaxError::*;

        match self {
            ExpectedToken { expected, actual } => {
                match actual {
                    None => format!("Error: {expected} expected"),
                    Some(token) => format!(
                        "Error: {expected} expected but got '{}'", 
                        ctx.string_interner.get_str(token.symbol)
                    ),
                }
            }
            UnmatchedGroupOpening(opening) => {
                format!("Error: Unmatched {opening}")
            },
            IndentTooLarge => "Error: Line indented too far in".to_string(),
            UnexpectedExpression => "Error: Unexpected Expression".to_string(),
            ExpressionExpected => "Error: Expression expected".to_string(),
            InvalidExpression => "Error: Invalid Expression".to_string(),

        }
    }
}
