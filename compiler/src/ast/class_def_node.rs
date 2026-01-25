use crate::ast::block::BlockId;
use crate::ast::typed_variable::TypedVariable;
use crate::phase::types::type_annotation::TypeAnnotation;

#[derive(Debug)]
pub struct ClassDefNode {
    pub class_type: TypeAnnotation,
    pub fields: Vec<TypedVariable>,
    pub body_id: BlockId,
}

impl ClassDefNode {
    pub fn new(
        class_type: TypeAnnotation,
        fields: Vec<TypedVariable>,
        body_id: BlockId
    ) -> Self {
        Self {
            class_type, 
            fields, 
            body_id 
        }
    }
}
