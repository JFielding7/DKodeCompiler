use crate::ast::ast_node::ExpressionId;

#[derive(Debug)]
pub struct IndexNode {
    pub operand: ExpressionId,
    pub arg: ExpressionId,
}

impl IndexNode {
    pub fn new(operand: ExpressionId, arg: ExpressionId) -> Self {
        Self {
            operand,
            arg
        }
    }
}
