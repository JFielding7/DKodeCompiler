use string_interner::DefaultSymbol;
use crate::types::type_annotation::TypeAnnotation;

#[derive(Debug)]
pub struct VariableNode {
    pub name: DefaultSymbol,
    type_annotation: Option<TypeAnnotation>,
}

impl VariableNode {
    pub fn new(name: DefaultSymbol, type_annotation: Option<TypeAnnotation>) -> Self {
        Self {
            name,
            type_annotation
        }
    }
}
