use std::collections::HashMap;
use string_interner::DefaultSymbol;
use crate::compiler_context::symbol::Symbol;

#[derive(Debug)]
pub struct Scope {
    symbols: HashMap<DefaultSymbol, Symbol>,
    pub parent: Option<ScopeId>,
}

impl Scope {
    pub fn global() -> Self {
        Self {
            parent: None,
            symbols: HashMap::new(),
        }
    }

    pub fn with_parent(parent: ScopeId) -> Self {
        Self {
            parent: Some(parent),
            symbols: HashMap::new()
        }
    }

    pub fn lookup(&self, name: DefaultSymbol) -> Option<&Symbol> {
        match self.symbols.get(&name) {
            Some(symbol) => Some(symbol),
            None => None
        }
    }

    pub fn lookup_mut(&mut self, name: DefaultSymbol) -> Option<&mut Symbol> {
        match self.symbols.get_mut(&name) {
            Some(symbol) => Some(symbol),
            None => None
        }
    }

    pub fn contains(&self, name: DefaultSymbol) -> bool {
        self.symbols.contains_key(&name)
    }

    pub fn insert(&mut self, symbol: Symbol) {
        if !self.symbols.contains_key(&symbol.name) {
            self.symbols.insert(symbol.name, symbol);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ScopeId(usize);

impl ScopeId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
    
    pub fn global() -> Self {
        Self::new(0)
    }
}