use string_interner::DefaultSymbol;
use crate::compiler_context::CompilerContext;
use crate::error::compiler_error::SpannableError;
use crate::lexical_analysis::token::INDENT_SIZE;

#[derive(Debug)]
pub enum LexerError {
    InvalidToken(DefaultSymbol),

    UnalignedIndent(usize),
}

impl SpannableError for LexerError {
    fn format(&self, ctx: &CompilerContext) -> String {
        use LexerError::*;

        match self {
            InvalidToken(token) => {
                format!("Error: Unrecognized token: {}", ctx.string_interner.get_str(*token))
            },
            UnalignedIndent(size) => {
                format!("Error: Unaligned Indent: Indent size {size} is not a multiple of {INDENT_SIZE}")
            },
        }
    }
}
