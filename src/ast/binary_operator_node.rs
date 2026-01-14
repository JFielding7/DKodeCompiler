use crate::ast::ast_node::ASTNodeId;
use crate::operators::binary_operators::BinaryOperator;

#[derive(Debug)]
pub struct BinaryOperatorNode {
    pub op_type: BinaryOperator,
    pub left: ASTNodeId,
    pub right: ASTNodeId,
}

impl BinaryOperatorNode {
    pub fn new(
        op_type: BinaryOperator,
        left: ASTNodeId,
        right: ASTNodeId,
    ) -> Self {
        Self {
            op_type,
            left,
            right,
        }
    }
}
