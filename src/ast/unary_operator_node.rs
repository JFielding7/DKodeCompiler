use crate::ast::ast_node::ASTNodeId;
use crate::operators::unary_operators::UnaryOperator;

#[derive(Debug)]
pub struct UnaryOperatorNode {
    pub op_type: UnaryOperator,
    pub operand: ASTNodeId,
}

impl UnaryOperatorNode {
    pub fn new(
        op_type: UnaryOperator,
        operand: ASTNodeId,
    ) -> Self {
        Self {
            op_type,
            operand
        }
    }
}
