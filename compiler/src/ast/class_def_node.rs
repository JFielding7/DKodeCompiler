use crate::ast::ast_node::ItemId;
use string_interner::DefaultSymbol;
use crate::ast::block::BlockId;
use crate::ast::function_def_node::Parameter;
use crate::types::type_annotation::TypeAnnotation;

#[derive(Debug)]
pub struct ClassDefNode {
    pub class_type: TypeAnnotation,
    pub fields: Vec<Parameter>,
    pub body_id: BlockId,
}

impl ClassDefNode {
    pub fn new(
        class_type: TypeAnnotation, 
        fields: Vec<Parameter>, 
        body_id: BlockId
    ) -> Self {
        Self {
            class_type, 
            fields, 
            body_id 
        }
    }
}
