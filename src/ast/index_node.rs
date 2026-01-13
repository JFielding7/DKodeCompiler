use crate::ast::arena_ast::ASTNodeId;

#[derive(Debug)]
pub struct IndexNode {
    pub operand: ASTNodeId,
    pub arg: ASTNodeId,
}

impl IndexNode {
    pub fn new(operand: ASTNodeId, arg: ASTNodeId) -> Self {
        Self {
            operand,
            arg
        }
    }
}
