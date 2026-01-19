use inkwell::context::Context;
use crate::compiler_context::global_string_interner::GlobalStringInterner;
use crate::compiler_context::symbol_table::SymbolTable;
use crate::compiler_context::type_arena::TypeArena;

pub mod symbol_table;
pub mod type_arena;
mod global_string_interner;

#[derive(Debug)]
pub struct CompilerContext {
    pub string_interner: GlobalStringInterner,
    pub type_arena: TypeArena,
    pub symbol_table: SymbolTable,
}

impl CompilerContext {
    pub fn new() -> Self {
        Self {
            string_interner: GlobalStringInterner::new(),
            type_arena: TypeArena::new(),
            symbol_table: SymbolTable::new(),
        }
    }
}
