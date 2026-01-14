use crate::lexer::token::TokenType::Indent;
use crate::lexer::token::{Token, TokenType};
use crate::source::source_span::SourceSpan;
use crate::syntax::parser::token_stream::TokenStream;
use std::ops::Deref;
use std::vec::IntoIter;

#[derive(Debug)]
pub struct Statement {
    pub indent_size: usize,
    pub tokens: Vec<Token>,
}

impl Statement {
    pub const INDEX_AFTER_INDENT: usize = 1;
    
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            indent_size: Self::extract_indent_size(&tokens),
            tokens,
        }
    }

    fn extract_indent_size(tokens: &Vec<Token>) -> usize {
        let first_token = tokens
            .first()
            .expect("Statement must have at least one token");

        match first_token.token_type {
            Indent(size) => size,
            _ => unreachable!("Statement must start with Indent"),
        }
    }

    pub fn indent_token(&self) -> &Token {
        &self[0]
    }

    pub fn token_after_indent(&self) -> &Token {
        &self[Self::INDEX_AFTER_INDENT]
    }

    pub fn token_after_indent_type(&self) -> &TokenType {
        &self.token_after_indent().token_type
    }

    pub fn token_after_indent_matches(&self, token_type: TokenType) -> bool {
        self.token_after_indent_type() == &token_type
    }

    pub fn last_token(&self) -> &Token {
        self.last().expect("Statement must have at least one token")
    }
    
    pub fn full_span(&self) -> SourceSpan {
        let start_span = &self.token_after_indent().span;
        SourceSpan::new(start_span.line_index, start_span.start, self.last_token().span.end)
    }

    pub fn suffix_stream(&self, start: usize) -> TokenStream<'_> {
        TokenStream::new(&self[start..])
    }
}

impl Deref for Statement {
    type Target = [Token];

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl IntoIterator for Statement {
    type Item = Token;
    type IntoIter = IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}
