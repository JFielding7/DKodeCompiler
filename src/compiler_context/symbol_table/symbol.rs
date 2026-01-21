use string_interner::DefaultSymbol;
use crate::types::data_type::DataTypeId;
use crate::source::source_span::SourceSpan;

#[derive(Debug)]
pub struct Symbol {
    pub id: SymbolId,
    pub name: DefaultSymbol,
    pub data_type_id: Option<DataTypeId>,
    pub func_param_index: Option<usize>,
    def_span: SourceSpan,
}

impl Symbol {
    pub fn new(id: SymbolId, name: DefaultSymbol, func_param_index: Option<usize>, def_span: SourceSpan) -> Self {
        Self {
            id,
            name,
            def_span,
            func_param_index,
            data_type_id: None,
        }
    }

    pub fn data_type_id(&self) -> DataTypeId {
        self.data_type_id.expect("Symbol must have data type")
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct SymbolId(usize);

impl SymbolId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}
