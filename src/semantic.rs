use crate::ast::arena_ast::AST;
use crate::compiler_context::CompilerContext;
use crate::semantic::error::SemanticResult;
use crate::semantic::name_resolution::NameResolver;
use crate::semantic::type_synthesis::type_synthesizer::TypeSynthesizer;

pub mod error;
pub mod type_synthesis;
pub mod name_resolution;

pub fn semantic_analysis(ast: AST, ctx: &mut CompilerContext) -> SemanticResult<AST> {
    NameResolver::resolve(&ast, ctx)?;

    println!("{:?}", ctx.symbol_table);

    let ast = TypeSynthesizer::synthesize(ast, ctx)?;

    println!("{:?}", ast);

    Ok(ast)
}
