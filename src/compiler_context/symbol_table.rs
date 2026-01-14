use crate::compiler_context::scope::{Scope, ScopeId};
use crate::compiler_context::symbol::Symbol;
use string_interner::DefaultSymbol;
use crate::compiler_context::type_arena::DataTypeId;
use crate::source::source_span::SourceSpan;

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
    
    pub fn add_scope_with_parent(&mut self, parent_scope_id: ScopeId) -> ScopeId {
        let id = self.scopes.len();
        self.scopes.push(Scope::with_parent(parent_scope_id));
        ScopeId::new(id)
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
        self.scopes[scope_id.as_usize()].contains(name)
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


