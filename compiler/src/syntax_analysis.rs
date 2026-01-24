use crate::ast::AST;
use crate::error::compiler_error::CompilerResult;
use crate::lexical_analysis::TokenizedLines;
use crate::phase::symbol_table::scope::Scope;
use crate::phase::SyntaxAnalysis;
use crate::syntax_analysis::parser::ast_parser::ASTParser;
use crate::syntax_analysis::parser::source_statements::SourceStatements;

mod parser;
mod error;
mod scope;

pub struct SyntaxAnalysisOutput {
    pub ast: AST,
    pub scopes: Vec<Scope<SyntaxAnalysis>>,
}

impl SyntaxAnalysisOutput {
    fn new(ast: AST, scopes: Vec<Scope<SyntaxAnalysis>>) -> SyntaxAnalysisOutput {
        Self {
            ast,
            scopes
        }
    }
}

pub fn syntax_analysis(source_lines: TokenizedLines) -> CompilerResult<SyntaxAnalysisOutput> {

    let statements: SourceStatements = source_lines.into();
    let mut parser = ASTParser::new(statements);
    parser.parse_global_nodes()?;

    Ok(SyntaxAnalysisOutput::new(parser.ast, parser.scopes))
}
