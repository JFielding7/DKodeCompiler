use crate::source::source_span::SourceSpan;
use string_interner::DefaultSymbol;

#[derive(Debug)]
pub struct TypeAnnotation {
    pub type_name: DefaultSymbol,
    pub inner_types: Vec<TypeAnnotation>,
    pub span: SourceSpan,
}

impl TypeAnnotation {
    pub fn new(type_name: DefaultSymbol, span: SourceSpan) -> Self {
        Self {
            type_name,
            inner_types: Vec::new(),
            span,
        }
    }

    pub fn with_params(type_name: DefaultSymbol, inner_types: Vec<TypeAnnotation>, span: SourceSpan) -> Self {
        Self {
            type_name,
            inner_types,
            span,
        }
    }
}
