use string_interner::DefaultSymbol;
use crate::phase::types::type_annotation::TypeAnnotation;
use crate::source::source_span::SourceSpan;

#[derive(Debug)]
pub struct TypedVariable {
    pub name: DefaultSymbol,
    pub type_annotation: TypeAnnotation,
    pub span: SourceSpan,
}

impl TypedVariable {
    pub fn new(name: DefaultSymbol, type_annotation: TypeAnnotation, span: SourceSpan) -> Self {
        Self {
            name,
            type_annotation,
            span,
        }
    }
}
