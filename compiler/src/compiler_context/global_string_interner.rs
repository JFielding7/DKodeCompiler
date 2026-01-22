use string_interner::{DefaultSymbol, DefaultBackend, StringInterner};

#[derive(Debug)]
pub struct GlobalStringInterner {
    pub string_interner: StringInterner<DefaultBackend>,
}

impl GlobalStringInterner {
    pub fn new() -> Self {
        Self {
            string_interner: StringInterner::new(),
        }
    }
    
    pub fn get_intern_symbol(&mut self, string: &str) -> DefaultSymbol {
        self.string_interner.get_or_intern(string)
    }

    pub fn get_str(&self, symbol: DefaultSymbol) -> &str {
        self.string_interner.resolve(symbol).expect("Expected str to be interned")
    }
}
