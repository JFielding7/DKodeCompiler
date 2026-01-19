use crate::ast::ast_node::ExpressionId;
use crate::ast::block::BlockId;

#[derive(Debug)]
pub struct IfNode {
    pub condition_blocks: Vec<ConditionBlock>,
    pub else_body_id: Option<BlockId>,
}

impl IfNode {
    pub fn new(condition_blocks: Vec<ConditionBlock>, else_body: Option<BlockId>) -> Self {
        Self {
            condition_blocks,
            else_body_id: else_body
        }
    }

    pub fn if_condition(&self) -> ExpressionId {
        self.condition_blocks[0].condition
    }
}

#[derive(Debug)]
pub struct ConditionBlock {
    pub condition: ExpressionId,
    pub body_id: BlockId,
}

impl ConditionBlock {
    pub fn new(condition: ExpressionId, body: BlockId) -> Self {
        Self {
            condition,
            body_id: body
        }
    }
}
