use inkwell::values::PointerValue;
use crate::ast::block::BlockId;
use crate::code_generation::CodeGeneration;
use crate::phase::MultiPhase;
use crate::phase::symbol_table::operator_registry::OperatorRegistry;
use crate::phase::symbol_table::scope::Scope;
use crate::phase::symbol_table::symbol::Symbol;
use crate::phase::symbol_table::SymbolTable;
use crate::semantic_analysis::SemanticAnalysis;

impl<'llvm_ctx> SymbolTable<CodeGeneration<'llvm_ctx>> {
    pub fn code_generation_symbol_table(
        semantic_symbol_table: &SymbolTable<<SemanticAnalysis as MultiPhase>::LastPhase>
    ) -> Self {
        let scopes: Vec<Scope<CodeGeneration>> = semantic_symbol_table.scopes
            .iter()
            .map(|scope| Scope::new(scope.parent, scope.function))
            .collect();

        Self {
            scopes,
            unary_op_impl: OperatorRegistry::new(),
            binary_op_impl: OperatorRegistry::new(),
        }
    }

    pub fn lower_semantic_symbol(
        &mut self,
        semantic_symbol: &Symbol<<SemanticAnalysis as MultiPhase>::LastPhase>,
        llvm_var: PointerValue<'llvm_ctx>,
        block_id: BlockId,
    ) -> &Symbol<CodeGeneration<'llvm_ctx>> {
        let symbol = Symbol::new(
            semantic_symbol.name, 
            semantic_symbol.symbol_type, 
            semantic_symbol.def_span, 
            semantic_symbol.data_type_id, 
            llvm_var
        );
        self.scopes[block_id.as_usize()].get_or_insert(symbol)
    }
}