use inkwell::values::PointerValue;
use string_interner::DefaultSymbol;
use crate::types::data_type::DataTypeId;
use crate::source::source_span::SourceSpan;

#[derive(Debug)]
pub struct Symbol {
    pub id: SymbolId,
    pub name: DefaultSymbol,
    pub data_type: Option<DataTypeId>,
    pub pointer: Option<usize>,
    def_span: SourceSpan,
}

impl Symbol {
    pub fn new(id: SymbolId, name: DefaultSymbol, def_span: SourceSpan) -> Self {
        Self {
            id,
            name,
            def_span,
            pointer: None,
            data_type: None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct SymbolId(usize);

impl SymbolId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}
