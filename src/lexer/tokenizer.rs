use crate::compiler_context::CompilerContext;
use crate::error::compiler_error::SpannableError;
use crate::lexer::error::LexerError::{InvalidToken, UnalignedIndent};
use crate::lexer::token::TokenType::Indent;
use crate::lexer::token::{Token, TokenType, INDENT_SIZE};
use crate::source::source_file::SourceFile;
use crate::source::source_span::SourceSpan;
use crate::error::compiler_error::CompilerResult;
use logos::Logos;

pub type LineTokens = Vec<Token>;

fn get_indent_token(
    line_index: usize,
    content: &str,
    ctx: &mut CompilerContext
) -> CompilerResult<Token> {
    let indent_chars = content.chars().take_while(|&c| c == ' ').collect::<String>();
    let indent = ctx.string_interner.get_intern_symbol(indent_chars.as_str());

    let indent_spaces = indent_chars.len();
    let span = SourceSpan::new(line_index, 0, indent_spaces);

    if (indent_spaces % INDENT_SIZE) != 0 {
        Err(UnalignedIndent(indent_spaces).at(span))
    } else {
        Ok(Token::new(
            Indent(indent_spaces / INDENT_SIZE),
            indent,
            span
        ))
    }
}

pub fn tokenize_line(
    line_index: usize,
    content: &str,
    ctx: &mut CompilerContext
) -> CompilerResult<LineTokens> {
    let mut tokens = vec![get_indent_token(line_index, content, ctx)?];
    let mut lexer = TokenType::lexer(content);

    while let Some(next_token) = lexer.next() {
        let span = lexer.span();
        let slice = lexer.slice();
        let source_span = SourceSpan::new(line_index, span.start, span.end);

        let token_type = next_token.map_err(|_|
            InvalidToken(
                ctx.string_interner.get_intern_symbol(slice)
            ).at(source_span)
        )?;
        
        tokens.push(Token::new(
            token_type,
            ctx.string_interner.get_intern_symbol(slice),
            source_span
        ));
    }

    Ok(tokens)
}
