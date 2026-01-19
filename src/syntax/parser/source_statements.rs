use crate::lexer::TokenizedLines;
use crate::syntax::parser::statement::Statement;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct SourceStatements {
    statements: Vec<Statement>
}

impl SourceStatements {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self {
            statements
        }
    }

    pub fn count(&self) -> usize {
        self.statements.len()
    }
}

impl From<TokenizedLines> for SourceStatements {
    fn from(source_lines: TokenizedLines) -> Self {

        let mut statements = Vec::new();
        let mut curr_statement_tokens = Vec::new();

        let mut legal_statement_end = false;

        for mut line in source_lines {
            if line.len() <= Statement::INDEX_AFTER_INDENT {
                continue;
            }

            if legal_statement_end && line[Statement::INDEX_AFTER_INDENT].is_legal_statement_boundary() {
                statements.push(Statement::new(curr_statement_tokens));

                curr_statement_tokens = line;
            } else {
                let last_token = line.last().expect("Line must have at least one token");

                legal_statement_end = last_token.is_legal_statement_boundary();

                if curr_statement_tokens.len() == 0 {
                    curr_statement_tokens.extend(line.drain(..));
                } else {
                    curr_statement_tokens.extend(line.drain(Statement::INDEX_AFTER_INDENT..));
                }
            }
        }

        if !curr_statement_tokens.is_empty() {
            statements.push(Statement::new(curr_statement_tokens));
        }

        Self::new(statements)
    }
}

impl IntoIterator for SourceStatements {
    type Item = Statement;
    type IntoIter = SourceStatementsIter;
    
    fn into_iter(self) -> Self::IntoIter {
        self.statements.into_iter().peekable()
    }
}

pub type SourceStatementsIter = Peekable<IntoIter<Statement>>;
