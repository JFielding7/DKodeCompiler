mod operator_registry;
mod builtin_operator_registry;
pub mod symbol;
pub mod scope;

use scope::Scope;
use symbol::Symbol;
use string_interner::DefaultSymbol;
use crate::ast::block::BlockId;
use crate::compiler_context::symbol_table::operator_registry::OperatorRegistry;
use crate::compiler_context::symbol_table::symbol::SymbolId;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::unary_operators::UnaryOperator;
use crate::source::source_span::SourceSpan;
use crate::types::data_type::DataTypeId;

#[derive(Debug)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
    pub unary_op_impl: OperatorRegistry<UnaryOperator>,
    pub binary_op_impl: OperatorRegistry<BinaryOperator>,
    id_counter: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
            unary_op_impl: OperatorRegistry::new(),
            binary_op_impl: OperatorRegistry::new(),
            id_counter: 0,
        }
    }

    pub fn add_scope(&mut self, scope: Scope) -> BlockId {
        let id = self.scopes.len();
        self.scopes.push(scope);
        BlockId::new(id)
    }

    pub fn lookup(&self, name: DefaultSymbol, block_id: BlockId) -> Option<&Symbol> {
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

    pub fn insert(&mut self, name: DefaultSymbol, def_span: SourceSpan, block_id: BlockId) -> bool {
        let symbol = Symbol::new(SymbolId::new(self.id_counter), name, def_span);
        self.id_counter += 1;
        self.scopes[block_id.as_usize()].insert(symbol)
    }
    
    pub fn assign_type(&mut self, data_type_id: DataTypeId, name: DefaultSymbol, block_id: BlockId) {
        let mut curr_scope = Some(block_id);

        while let Some(id) = curr_scope {
            let scope = &mut self.scopes[id.as_usize()];

            if let Some(symbol) = scope.lookup_mut(name) {
                symbol.data_type = Some(data_type_id);
                return;
            }

            curr_scope = scope.parent;
        }

        unreachable!("Symbol {:?} not found in scope", name);
    }
}
