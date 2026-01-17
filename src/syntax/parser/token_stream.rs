use crate::source::source_span::SourceSpan;
use crate::lexer::token::TokenType::Identifier;
use crate::lexer::token::{Token, TokenType};
use std::iter::Peekable;
use std::slice::Iter;
use crate::error::compiler_error::CompilerResult;
use crate::error::compiler_error::SpannableError;
use crate::syntax::error::SyntaxError::ExpectedToken;

pub struct TokenStream<'a> {
    iter: Peekable<Iter<'a, Token>>,
    prev_token: &'a Token,
    curr_token_split: bool,

}

impl<'a> TokenStream<'a> {
    pub fn new(tokens: &'a [Token], start: usize) -> Self {
        Self {
            iter: tokens[start..].iter().peekable(),
            prev_token: &tokens[start - 1],
            curr_token_split: false
        }
    }

    pub fn peek(&mut self) -> Option<&&'a Token> {
        self.iter.peek()
    }

    pub fn empty(&mut self) -> bool{
        self.peek().is_none()
    }
    
    pub fn prev_span(&self) -> SourceSpan {
        self.prev_token.span
    }

    pub fn end_span(&mut self) -> SourceSpan {
        let mut span = self.prev_span();
        span.start = span.end;
        span.end += 1;
        span
    }

    pub fn split_curr_token(&mut self) {
        self.curr_token_split = true;
    }

    pub fn is_curr_token_split(&self) -> bool {
        self.curr_token_split
    }

    pub fn expect_next_token(&mut self, expected: TokenType) -> CompilerResult<&Token> {

        match self.next() {
            None => Err(ExpectedToken {
                expected,
                actual: None
            }.at(self.end_span())),

            Some(token) => {
                if *token == expected {
                    Ok(token)
                } else {
                    Err(ExpectedToken {
                        expected,
                        actual: Some(token.clone())
                    }.at(token.span))
                }
            },
        }
    }
    
    pub fn expect_next_identifier(&mut self) -> CompilerResult<&Token> {
        self.expect_next_token(Identifier)
    }

    pub fn peek_matches(&mut self, token_type: TokenType) -> bool {
        self.peek().is_some_and(|&token| *token == token_type)
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token_opt = self.iter.next();

        if let Some(token) = token_opt {
            self.prev_token = token;
        }

        self.curr_token_split = false;

        token_opt
    }
}
