pub mod operator_registry;
mod builtin_operator_registry;
pub mod symbol;
pub mod scope;

use crate::ast::block::BlockId;
use crate::phase::Phase;
use scope::Scope;
use std::collections::HashMap;
use string_interner::DefaultSymbol;
use symbol::Symbol;

#[derive(Debug)]
pub struct SymbolTable<T: Phase> {
    pub scopes: Vec<Scope<T>>,
    pub unary_op_impl: T::UnaryOpImpl,
    pub binary_op_impl: T::BinaryOpImpl,
}

impl<T> SymbolTable<T>
where
    T: Phase,
    T: Phase<Symbols = HashMap<DefaultSymbol, Symbol<T>>>,
{
    // pub fn new() -> Self {
    //     Self {
    //         scopes: Vec::new(),
    //         unary_op_impl,
    //         binary_op_impl
    //     }
    // }

    pub fn lookup(&self, name: DefaultSymbol, block_id: BlockId) -> Option<&Symbol<T>> {
        let mut curr_block_id = Some(block_id);
        
        while let Some(id) = curr_block_id {
            let scope = &self.scopes[id.as_usize()];
            
            if let Some(symbol) = scope.lookup(name) {
                return Some(symbol);
            }
            
            curr_block_id = scope.parent;
        }
        
        None
    }
    
    pub fn lookup_expect_exist(&self, name: DefaultSymbol, block_id: BlockId) -> &Symbol<T> {
        self.lookup(name, block_id).expect("Symbol must exist")
    }

    pub fn lookup_mut(&mut self, name: DefaultSymbol, block_id: BlockId) -> Option<&mut Symbol<T>> {
        let mut curr_block_id = Some(block_id);

        while let Some(id) = curr_block_id {
            let parent = self.scopes[id.as_usize()].parent;

            if self.scopes[id.as_usize()].lookup_mut(name).is_some() {
                return self.scopes[id.as_usize()].lookup_mut(name);
            }

            curr_block_id = parent;
        }

        None
    }

    pub fn lookup_expect_exist_mut(&mut self, name: DefaultSymbol, block_id: BlockId) -> &mut Symbol<T> {
        self.lookup_mut(name, block_id).expect("Symbol must exist")
    }
    
    pub fn contains(&self, name: DefaultSymbol, block_id: BlockId) -> bool {
        let mut curr_scope = Some(block_id);
    
        while let Some(id) = curr_scope {
            let scope = &self.scopes[id.as_usize()];
    
            if scope.contains(name) {
                return true;
            }
    
            curr_scope = scope.parent;
        }
    
        false
    }
    
    pub fn scope_function_name(&self, block_id: BlockId) -> Option<DefaultSymbol> {
        self.scopes[block_id.as_usize()].function
    }
}
