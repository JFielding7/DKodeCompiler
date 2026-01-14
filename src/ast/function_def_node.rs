use string_interner::DefaultSymbol;
use crate::ast::ast_node::ASTNodeId;
use crate::ast::block_body::Block;
use crate::compiler_context::scope::ScopeId;
use crate::source::source_span::SourceSpan;
use crate::types::type_annotation::TypeAnnotation;

#[derive(Debug)]
pub struct FunctionDefNode {
    pub name: DefaultSymbol,
    pub params: Vec<Parameter>,
    pub body: Block,
    pub(crate) return_type: Option<TypeAnnotation>,
}

impl FunctionDefNode {
    pub fn new(
        name: DefaultSymbol,
        params: Vec<Parameter>,
        body: Block,
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
    type_annotation: TypeAnnotation,
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
