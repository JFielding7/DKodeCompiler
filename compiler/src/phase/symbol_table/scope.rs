use crate::ast::block::BlockId;
use crate::phase::symbol_table::symbol::Symbol;
use crate::phase::Phase;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use string_interner::DefaultSymbol;

#[derive(Debug)]
pub struct Scope<T: Phase> {
    pub parent: Option<BlockId>,
    pub function: Option<DefaultSymbol>,
    pub symbols: T::Symbols,
}

impl<T> Scope<T>
where
    T: Phase + Phase<Symbols = HashMap<DefaultSymbol, Symbol<T>>>,
{
    pub fn new(parent: Option<BlockId>, function: Option<DefaultSymbol>) -> Self {
        Self {
            parent,
            symbols: HashMap::new(),
            function,
        }
    }

    pub fn insert(&mut self, symbol: Symbol<T>) -> bool {

        match self.symbols.entry(symbol.name) {
            Entry::Vacant(entry) => {
                entry.insert(symbol);
                true
            }
            Entry::Occupied(_) => false,
        }
    }
    
    pub fn get_or_insert(&mut self, symbol: Symbol<T>) -> &mut Symbol<T> {
        self.symbols.entry(symbol.name).or_insert(symbol)
    }

    pub fn lookup(&self, name: DefaultSymbol) -> Option<&Symbol<T>> {
        match self.symbols.get(&name) {
            Some(symbol) => Some(symbol),
            None => None
        }
    }

    pub fn lookup_mut(&mut self, name: DefaultSymbol) -> Option<&mut Symbol<T>> {
        match self.symbols.get_mut(&name) {
            Some(symbol) => Some(symbol),
            None => None
        }
    }

    pub fn contains(&self, name: DefaultSymbol) -> bool {
        self.symbols.contains_key(&name)
    }
}
