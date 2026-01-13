use crate::ast::arena_ast::ASTNodeId;

#[derive(Debug)]
pub struct ForNode {
    pub item_variable: ASTNodeId,
    pub iterator: ASTNodeId,
    body: Vec<ASTNodeId>,
}

impl ForNode {
    pub fn new(item_identifier: ASTNodeId, iterator: ASTNodeId, body: Vec<ASTNodeId>) -> Self {
        Self {
            item_variable: item_identifier,
            iterator,
            body
        }
    }
}
