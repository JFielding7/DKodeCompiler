mod error;
mod type_checking;
mod name_resolution;

use crate::ast::AST;
use crate::ast::ast_node::ExpressionId;
use crate::compiler_context::CompilerContext;
use crate::phase::symbol_table::SymbolTable;
use crate::phase::types::type_arena::TypeArena;
use crate::error::compiler_error::CompilerResult;
use crate::phase::types::data_type::DataTypeId;
use crate::syntax_analysis::SyntaxAnalysisOutput;
use crate::phase::MultiPhase;
use crate::semantic_analysis::name_resolution::name_resolver::NameResolver;
use crate::semantic_analysis::type_checking::type_checker::{TypeChecker, TypeCheckingOutput};
use crate::semantic_analysis::type_checking::TypeChecking;

pub struct SemanticAnalysis;

impl MultiPhase for SemanticAnalysis {
    type LastPhase = TypeChecking;
}

#[derive(Debug)]
pub struct SemanticAnalysisOutput {
    pub ast: AST,
    pub symbol_table: SymbolTable<TypeChecking>,
    pub type_arena: TypeArena<TypeChecking>,
    ast_expr_data_types: Vec<DataTypeId>,
}

impl SemanticAnalysisOutput {
    pub fn new(ast: AST, out: TypeCheckingOutput) -> Self {
        Self {
            ast,
            symbol_table: out.symbol_table,
            type_arena: out.type_arena,
            ast_expr_data_types: out.ast_expr_data_types,
        }
    }

    pub fn expr_data_type_id(&self, expr_id: ExpressionId) -> DataTypeId {
        self.ast_expr_data_types[expr_id.as_usize()]
    }
}

pub fn semantic_analysis(syntax_output: SyntaxAnalysisOutput, ctx: &mut CompilerContext) -> CompilerResult<SemanticAnalysisOutput> {
    let symbol_table = NameResolver::resolve(&syntax_output)?;

    let type_checking_output = TypeChecker::check_types(&syntax_output.ast, symbol_table, ctx)?;

    println!("{:?}", type_checking_output.type_arena);

    Ok(SemanticAnalysisOutput::new(syntax_output.ast, type_checking_output))
}
