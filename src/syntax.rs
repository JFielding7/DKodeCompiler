use crate::ast::AST;
use crate::error::compiler_error::CompilerResult;
use crate::lexer::TokenizedLines;
use crate::syntax::parser::ast_parser::ASTParser;
use crate::syntax::parser::source_statements::SourceStatements;

mod parser;
mod error;

pub fn syntax_analysis(source_lines: TokenizedLines) -> CompilerResult<AST> {

    let statements: SourceStatements = source_lines.into();
    let mut parser = ASTParser::new(statements);
    parser.parse_global_nodes()?;

    Ok(parser.ast)
}
