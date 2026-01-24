use string_interner::DefaultSymbol;
use crate::ast::block::BlockId;
use crate::phase::SyntaxAnalysis;
use crate::phase::symbol_table::scope::Scope;

impl Scope<SyntaxAnalysis> {
   pub fn syntax_analysis_scope(parent: Option<BlockId>, function: Option<DefaultSymbol>) -> Self {
        Self {
            parent,
            function,
            symbols: ()
        }
   }
}
