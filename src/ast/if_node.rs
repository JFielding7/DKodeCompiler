use crate::ast::arena_ast::ASTNodeId;
use crate::ast::ast_node::ASTNode;

#[derive(Debug)]
pub struct IfNode {
    condition_blocks: Vec<ConditionBlock>,
    else_body: Option<Vec<ASTNodeId>>,
}

impl IfNode {
    pub fn new(condition_blocks: Vec<ConditionBlock>, else_body: Option<Vec<ASTNodeId>>) -> Self {
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
    body: Vec<ASTNodeId>,
}

impl ConditionBlock {
    pub fn new(condition: ASTNodeId, body: Vec<ASTNodeId>) -> Self {
        Self {
            condition,
            body
        }
    }
}
