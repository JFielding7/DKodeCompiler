use crate::ast::ast_node::ASTNodeId;
use crate::compiler_context::scope::ScopeId;

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<ASTNodeId>,
    pub scope_id: ScopeId,
}

impl Block {
    pub fn new(statements: Vec<ASTNodeId>, scope_id: ScopeId) -> Self {
        Self { 
            statements, 
            scope_id 
        }
    }
}