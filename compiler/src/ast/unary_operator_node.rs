use crate::ast::ast_node::ExpressionId;
use crate::operators::unary_operators::UnaryOperator;

#[derive(Debug)]
pub struct UnaryOperatorNode {
    pub op_type: UnaryOperator,
    pub operand_id: ExpressionId,
}

impl UnaryOperatorNode {
    pub fn new(
        op_type: UnaryOperator,
        operand_id: ExpressionId,
    ) -> Self {
        Self {
            op_type,
            operand_id,
        }
    }
}
