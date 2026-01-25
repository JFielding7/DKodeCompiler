use string_interner::DefaultSymbol;
use crate::ast::block::BlockId;
use crate::phase::symbol_table::scope::Scope;
use crate::phase::symbol_table::symbol::{Symbol, SymbolType};
use crate::phase::symbol_table::SymbolTable;
use crate::semantic_analysis::name_resolution::NameResolution;
use crate::source::source_span::SourceSpan;
use crate::syntax_analysis::SyntaxAnalysisOutput;

impl SymbolTable<NameResolution> {
    pub fn name_resolution_table(syntax_output: &SyntaxAnalysisOutput) -> Self {
        let scopes: Vec<Scope<NameResolution>> = syntax_output.scopes
            .iter()
            .map(|scope| Scope::new(scope.parent, scope.function))
            .collect();

        Self {
            scopes,
            unary_op_impl: (),
            binary_op_impl: (),
        }
    }

    pub fn insert_symbol(
        &mut self,
        name: DefaultSymbol,
        symbol_type: SymbolType,
        def_span: SourceSpan,
        block_id: BlockId,
    ) -> bool {
        let symbol = Symbol::new(name, symbol_type, def_span, (), ());
        self.scopes[block_id.as_usize()].insert(symbol)
    }

    pub fn insert_variable(
        &mut self,
        name: DefaultSymbol,
        def_span: SourceSpan,
        block_id: BlockId
    ) -> bool {
        self.insert_symbol(name, SymbolType::Variable, def_span, block_id)
    }

    pub fn insert_function_param(
        &mut self,
        name: DefaultSymbol,
        param_index: usize,
        def_span: SourceSpan,
        block_id: BlockId
    ) -> bool {
        self.insert_symbol(name, SymbolType::FunctionParam(param_index), def_span, block_id)
    }

    pub fn insert_class_field(
        &mut self,
        name: DefaultSymbol,
        param_index: usize,
        def_span: SourceSpan,
        block_id: BlockId
    ) -> bool {
        self.insert_symbol(name, SymbolType::ClassField(param_index), def_span, block_id)
    }
}
