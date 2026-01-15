use crate::compiler_context::scope::{Scope, ScopeId};
use crate::compiler_context::symbol::Symbol;
use string_interner::DefaultSymbol;
use crate::source::source_span::SourceSpan;
use crate::types::data_type::DataTypeId;

#[derive(Debug)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::global()],
        }
    }

    fn add_scope(&mut self, scope: Scope) -> ScopeId {
        let id = self.scopes.len();
        self.scopes.push(scope);
        ScopeId::new(id)
    }

    pub fn add_function_scope(&mut self, function: DefaultSymbol, parent_scope_id: ScopeId) -> ScopeId {
        let scope = Scope::new(parent_scope_id, Some(function));

        self.add_scope(scope)
    }
    
    pub fn add_block_scope(&mut self, parent_scope_id: ScopeId) -> ScopeId {
        let parent = &self.scopes[parent_scope_id.as_usize()];
        let scope = Scope::new(parent_scope_id, parent.function);

        self.add_scope(scope)
    }

    pub fn lookup(&self, name: DefaultSymbol, scope_id: ScopeId) -> Option<&Symbol> {
        let mut curr_scope = Some(scope_id);
        
        while let Some(id) = curr_scope {
            let scope = &self.scopes[id.as_usize()];
            
            if let Some(symbol) = scope.lookup(name) {
                return Some(symbol);
            }
            
            curr_scope = scope.parent;
        }
        
        None
    }
    
    pub fn contains(&self, name: DefaultSymbol, scope_id: ScopeId) -> bool {
        let mut curr_scope = Some(scope_id);

        while let Some(id) = curr_scope {
            let scope = &self.scopes[id.as_usize()];

            if scope.contains(name) {
                return true;
            }

            curr_scope = scope.parent;
        }

        false
    }
    
    pub fn scope_function_name(&self, scope_id: ScopeId) -> Option<DefaultSymbol> {
        self.scopes[scope_id.as_usize()].function
    }

    pub fn insert(&mut self, name: DefaultSymbol, def_span: SourceSpan, scope_id: ScopeId) -> bool {
        let symbol = Symbol::new(name, def_span);
        self.scopes[scope_id.as_usize()].insert(symbol)
    }
    
    pub fn assign_type(&mut self, data_type_id: DataTypeId, name: DefaultSymbol, scope_id: ScopeId) -> bool {
        let mut curr_scope = Some(scope_id);

        while let Some(id) = curr_scope {
            let scope = &mut self.scopes[id.as_usize()];

            if let Some(symbol) = scope.lookup_mut(name) {
                symbol.data_type = Some(data_type_id);
                return true;
            }

            curr_scope = scope.parent;
        }

        false
    }
}


