use crate::ast::ast_node::ExpressionId;
use crate::ast::block::BlockId;

#[derive(Debug)]
pub struct WhileNode {
    pub condition: ExpressionId,
    pub body: BlockId,
}

impl WhileNode {
    pub fn new(condition: ExpressionId, body: BlockId) -> Self {
        Self {
            condition,
            body
        }
    }
}
