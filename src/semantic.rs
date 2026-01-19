use crate::ast::AST;
use crate::compiler_context::CompilerContext;
use crate::error::compiler_error::CompilerResult;
use crate::semantic::name_resolution::NameResolver;
use crate::semantic::type_synthesis::TypeSynthesizer;
use crate::types::data_type::DataTypeId;

mod error;
mod type_synthesis;
mod name_resolution;

#[derive(Debug)]
pub struct AnnotatedAST {
    pub ast: AST,
    ast_expr_data_types: Vec<DataTypeId>,
    
}

impl AnnotatedAST {
    pub fn new(ast: AST, ast_node_data_types: Vec<DataTypeId>) -> Self {
        Self { 
            ast,
            ast_expr_data_types: ast_node_data_types
        }
    }
}

pub fn semantic_analysis(ast: AST, ctx: &mut CompilerContext) -> CompilerResult<AnnotatedAST> {
    NameResolver::resolve(&ast, ctx)?;

    let ast_expr_data_types = TypeSynthesizer::synthesize(&ast, ctx)?;

    Ok(AnnotatedAST::new(ast, ast_expr_data_types))
}
