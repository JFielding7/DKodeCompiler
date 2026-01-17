use crate::ast::AST;
use crate::compiler_context::CompilerContext;
use crate::compiler_context::scope::ScopeId;
use crate::error::compiler_error::CompilerResult;
use crate::semantic::name_resolution::NameResolver;
use crate::semantic::type_synthesis::type_synthesizer::TypeSynthesizer;
use crate::types::data_type::DataTypeId;

mod error;
mod type_synthesis;
mod name_resolution;

#[derive(Debug)]
pub struct AnnotatedAST {
    ast: AST,
    block_scope_ids: Vec<ScopeId>,
    ast_node_data_types: Vec<DataTypeId>,
    
}

impl AnnotatedAST {
    pub fn new(ast: AST, block_scope_ids: Vec<ScopeId>, ast_node_data_types: Vec<DataTypeId>) -> Self {
        Self { 
            ast,
            block_scope_ids,
            ast_node_data_types 
        }
    }
}

pub fn semantic_analysis(ast: AST, ctx: &mut CompilerContext) -> CompilerResult<AnnotatedAST> {
    let block_scope_ids = NameResolver::resolve(&ast, ctx)?;

    let ast_node_data_types = TypeSynthesizer::synthesize(
        &ast, ctx, &block_scope_ids
    )?;

    println!("{:?}", ctx.symbol_table);

    Ok(AnnotatedAST::new(ast, block_scope_ids, ast_node_data_types))
}
