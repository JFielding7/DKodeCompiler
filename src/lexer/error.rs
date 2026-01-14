use crate::lexer::token::INDENT_SIZE;
use crate::compiler_context::CompilerContext;
use crate::error::spanned_error::{SpannableError, CompilerError};


#[derive(thiserror::Error, Debug)]
pub enum LexerError {
    #[error("Error: Unrecognized token: {0}")]
    InvalidToken(String),

    #[error("Error: Unaligned Indent: Indent size {0} is not a multiple of {INDENT_SIZE}")]
    UnalignedIndent(usize),
}

impl SpannableError for LexerError {
    fn format(&self, _: CompilerContext) -> String {
        format!("{self}")
    }
}
