use string_interner::DefaultSymbol;
use crate::ast::block::BlockId;
use crate::source::source_span::SourceSpan;
use crate::types::type_annotation::TypeAnnotation;

#[derive(Debug)]
pub struct FunctionDefNode {
    pub name: DefaultSymbol,
    pub params: Vec<Parameter>,
    pub body: BlockId,
    pub return_type: Option<TypeAnnotation>,
}

impl FunctionDefNode {
    pub fn new(
        name: DefaultSymbol,
        params: Vec<Parameter>,
        body: BlockId,
        return_type: Option<TypeAnnotation>,
    ) -> Self {
        Self {
            name,
            params,
            body,
            return_type,
        }
    }
}

#[derive(Debug)]
pub struct Parameter {
    pub name: DefaultSymbol,
    pub type_annotation: TypeAnnotation,
    pub span: SourceSpan,
}

impl Parameter {
    pub fn new(name: DefaultSymbol, type_annotation: TypeAnnotation, span: SourceSpan) -> Self {
        Self { 
            name, 
            type_annotation,
            span,
        }
    }
}
