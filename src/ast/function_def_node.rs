use string_interner::DefaultSymbol;
use crate::ast::arena_ast::ASTNodeId;
use crate::types::type_annotation::TypeAnnotation;

#[derive(Debug)]
pub struct FunctionDefNode {
    pub name: DefaultSymbol,
    params: Vec<Parameter>,
    body: Vec<ASTNodeId>,
    return_type: Option<TypeAnnotation>,
}

impl FunctionDefNode {
    pub fn new(
        name: DefaultSymbol,
        params: Vec<Parameter>,
        body: Vec<ASTNodeId>,
        return_type: Option<TypeAnnotation>,
    ) -> Self {
        Self {
            name,
            params,
            body,
            return_type
        }
    }
}

#[derive(Debug)]
pub struct Parameter {
    name: DefaultSymbol,
    type_annotation: TypeAnnotation,
}

impl Parameter {
    pub fn new(name: DefaultSymbol, type_annotation: TypeAnnotation) -> Self {
        Self { 
            name, 
            type_annotation 
        }
    }
}
