use crate::ast::AST;
use crate::compiler_context::CompilerContext;
use crate::error::compiler_error::CompilerResult;
use crate::lexer::TokenizedLines;
use crate::syntax::parser::ast_parser::ASTParser;
use crate::syntax::parser::source_statements::SourceStatements;

mod parser;
mod error;

pub fn syntax_analysis(source_lines: TokenizedLines, ctx: &mut CompilerContext) -> CompilerResult<AST> {

    let statements: SourceStatements = source_lines.into();
    let mut parser = ASTParser::new(statements, ctx);
    parser.parse_global_nodes()?;

    Ok(parser.ast)
}
