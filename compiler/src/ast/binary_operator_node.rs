use crate::ast::ast_node::ExpressionId;
use crate::operators::binary_operators::BinaryOperator;

#[derive(Debug)]
pub struct BinaryOperatorNode {
    pub op_type: BinaryOperator,
    pub left: ExpressionId,
    pub right: ExpressionId,
}

impl BinaryOperatorNode {
    pub fn new(
        op_type: BinaryOperator,
        left: ExpressionId,
        right: ExpressionId,
    ) -> Self {
        Self {
            op_type,
            left,
            right,
        }
    }
}
