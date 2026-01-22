use string_interner::DefaultSymbol;
use crate::types::data_type::DataTypeId;
use crate::source::source_span::SourceSpan;

#[derive(Debug)]
pub struct Symbol {
    pub id: SymbolId,
    pub name: DefaultSymbol,
    pub data_type_id: Option<DataTypeId>,
    pub symbol_type: SymbolType,
    def_span: SourceSpan,
}

impl Symbol {
    pub fn new(
        id: SymbolId, 
        name: DefaultSymbol, 
        symbol_type: SymbolType, 
        def_span: SourceSpan
    ) -> Self {
        Self {
            id,
            name,
            def_span,
            symbol_type,
            data_type_id: None,
        }
    }

    pub fn data_type_id(&self) -> DataTypeId {
        self.data_type_id.expect("Symbol must have data type")
    }
}

#[derive(Debug)]
pub enum SymbolType {
    Variable,
    FunctionParam(usize),
    ClassField(usize),
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct SymbolId(usize);

impl SymbolId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}
