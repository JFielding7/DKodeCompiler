use crate::source::source_span::SourceSpan;
use crate::lexer::token::TokenType::Identifier;
use crate::lexer::token::{Token, TokenType};
use crate::syntax::error::{SyntaxError, SyntaxResult};
use std::iter::Peekable;
use std::slice::Iter;
use string_interner::DefaultSymbol;
use crate::error::spanned_error::SpannableError;
use crate::syntax::error::SyntaxError::ExpectedToken;

pub struct TokenStream<'a> {
    iter: Peekable<Iter<'a, Token>>,
    prev_token: &'a Token,
    curr_token_split: bool,

}

impl<'a> TokenStream<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            iter: tokens.iter().peekable(),
            prev_token: &tokens[0],
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

    pub fn expect_next_token(&mut self, token_type: TokenType) -> SyntaxResult<&Token> {

        match self.next() {
            None => Err(ExpectedToken(token_type).at(self.end_span())),

            Some(token) => {
                if *token == token_type {
                    Ok(token)
                } else {
                    Err(ExpectedToken(token_type).at(token.span))
                }
            },
        }
    }
    
    pub fn expect_next_identifier(&mut self) -> SyntaxResult<&Token> {
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
