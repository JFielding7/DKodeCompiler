use crate::ast::arena_ast::ASTNodeId;
use crate::ast::block_body::Block;

#[derive(Debug)]
pub struct IfNode {
    condition_blocks: Vec<ConditionBlock>,
    else_body: Option<Block>,
}

impl IfNode {
    pub fn new(condition_blocks: Vec<ConditionBlock>, else_body: Option<Block>) -> Self {
        Self {
            condition_blocks,
            else_body
        }
    }

    pub fn if_condition(&self) -> ASTNodeId {
        self.condition_blocks[0].condition
    }
}

#[derive(Debug)]
pub struct ConditionBlock {
    condition: ASTNodeId,
    body: Block,
}

impl ConditionBlock {
    pub fn new(condition: ASTNodeId, body: Block) -> Self {
        Self {
            condition,
            body
        }
    }
}
