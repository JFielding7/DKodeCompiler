use string_interner::DefaultSymbol;
use crate::types::data_type::DataTypeId;
use crate::source::source_span::SourceSpan;

#[derive(Debug)]
pub struct Symbol {
    pub name: DefaultSymbol,
    pub data_type: Option<DataTypeId>,
    def_span: SourceSpan,
}

impl Symbol {
    pub fn new(name: DefaultSymbol, def_span: SourceSpan) -> Self {
        Self {
            name,
            def_span,
            data_type: None,
        }
    }
}
