use string_interner::{DefaultBackend, DefaultSymbol, StringInterner};
use crate::compiler_context::symbol_table::SymbolTable;
use crate::compiler_context::type_arena::TypeArena;

pub mod symbol_table;
pub mod type_arena;
pub mod scope;
pub mod symbol;

pub struct CompilerContext {
    pub string_interner: StringInterner<DefaultBackend>,
    pub type_arena: TypeArena,
    pub symbol_table: SymbolTable,
}

impl CompilerContext {
    pub fn new() -> Self {
        Self {
            string_interner: StringInterner::default(),
            type_arena: TypeArena::new(),
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn get_intern_symbol(&mut self, string: &str) -> DefaultSymbol {
        self.string_interner.get_or_intern(string)
    }

    pub fn get_str(&self, symbol: DefaultSymbol) -> Option<&str> {
        self.string_interner.resolve(symbol)
    }
    
    // pub fn get_type_id(&self, symbol: DefaultSymbol) -> Option<DataTypeId> {
    //     self.type_arena.get_type_id(symbol, self.get_str(symbol)?)
    // }
}
