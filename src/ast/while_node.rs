use crate::ast::arena_ast::ASTNodeId;
use crate::ast::block_body::Block;

#[derive(Debug)]
pub struct WhileNode {
    pub condition: ASTNodeId,
    body: Block,
}

impl WhileNode {
    pub fn new(condition: ASTNodeId, body: Block) -> Self {
        Self {
            condition,
            body
        }
    }
}
