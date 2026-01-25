use string_interner::DefaultSymbol;
use crate::ast::block::BlockId;
use crate::ast::typed_variable::TypedVariable;
use crate::phase::types::type_annotation::TypeAnnotation;

#[derive(Debug)]
pub struct FunctionDefNode {
    pub name: DefaultSymbol,
    pub params: Vec<TypedVariable>,
    pub body_id: BlockId,
    pub return_type: Option<TypeAnnotation>,
}

impl FunctionDefNode {
    pub fn new(
        name: DefaultSymbol,
        params: Vec<TypedVariable>,
        body: BlockId,
        return_type: Option<TypeAnnotation>,
    ) -> Self {
        Self {
            name,
            params,
            body_id: body,
            return_type,
        }
    }
}
