use crate::compiler_context::global_string_interner::GlobalStringInterner;

mod global_string_interner;

#[derive(Debug)]
pub struct CompilerContext {
    pub string_interner: GlobalStringInterner,
}

impl CompilerContext {
    pub fn new() -> Self {
        Self {
            string_interner: GlobalStringInterner::new(),
        }
    }
}


