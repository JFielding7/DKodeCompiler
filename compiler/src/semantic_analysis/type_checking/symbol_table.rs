use crate::ast::block::BlockId;
use crate::phase::symbol_table::SymbolTable;
use crate::phase::symbol_table::operator_registry::OperatorRegistry;
use crate::phase::symbol_table::scope::Scope;
use crate::phase::symbol_table::symbol::Symbol;
use crate::phase::types::data_type::DataTypeId;
use crate::semantic_analysis::name_resolution::NameResolution;
use crate::semantic_analysis::type_checking::TypeChecking;

impl SymbolTable<TypeChecking> {
    pub fn type_checking_symbol_table(name_resolution_symbol_table: &SymbolTable<NameResolution>) -> Self {
        let scopes: Vec<Scope<TypeChecking>> = name_resolution_symbol_table.scopes
            .iter()
            .map(|scope| Scope::new(scope.parent, scope.function))
            .collect();

        Self {
            scopes,
            unary_op_impl: OperatorRegistry::new(),
            binary_op_impl: OperatorRegistry::new(),
        }
    }

    pub fn assign_symbol_type(
        &mut self,
        symbol: &Symbol<NameResolution>,
        data_type_id: DataTypeId,
        block_id: BlockId,
    ) -> &Symbol<TypeChecking> {
        let symbol = Symbol::new(symbol.name, symbol.symbol_type, symbol.def_span, data_type_id, ());
        self.scopes[block_id.as_usize()].get_or_insert(symbol)
    }
}
