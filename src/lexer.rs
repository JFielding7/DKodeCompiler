use std::vec::IntoIter;
use crate::compiler_context::CompilerContext;
use crate::error::compiler_error::CompilerResult;
use crate::lexer::tokenizer::{tokenize_line, LineTokens};
use crate::source::source_file::SourceFile;

pub mod token;
mod tokenizer;
mod error;

#[derive(Debug)]
pub struct TokenizedLines(Vec<LineTokens>);

impl IntoIterator for TokenizedLines {
    type Item = LineTokens;
    type IntoIter = IntoIter<LineTokens>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub fn lexical_analysis(
    source_file: &SourceFile,
    ctx: &mut CompilerContext
) -> CompilerResult<TokenizedLines> {
    let lines = source_file.into_iter()
        .enumerate()
        .map(|(i, content)| tokenize_line(i, content, ctx))
        .collect::<CompilerResult<Vec<LineTokens>>>()?;

    Ok(TokenizedLines(lines))
}
