use std::collections::hash_map::Entry;
use std::collections::HashMap;
use string_interner::DefaultSymbol;
use crate::compiler_context::symbol::Symbol;

#[derive(Debug)]
pub struct Scope {
    symbols: HashMap<DefaultSymbol, Symbol>,
    pub parent: Option<ScopeId>,
    pub function: Option<DefaultSymbol>,
}

impl Scope {
    pub fn global() -> Self {
        Self {
            parent: None,
            symbols: HashMap::new(),
            function: None,
        }
    }

    pub fn new(parent: ScopeId, function: Option<DefaultSymbol>) -> Self {
        Self {
            parent: Some(parent),
            symbols: HashMap::new(),
            function,
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

    pub fn insert(&mut self, symbol: Symbol) -> bool {
    
        match self.symbols.entry(symbol.name) {
            Entry::Vacant(entry) => {
                entry.insert(symbol);
                true
            }
            Entry::Occupied(_) => false,
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
