use crate::ast::ast_node::ASTNode;
use std::iter::Map;
use std::ops::Range;

#[derive(Debug)]
pub struct AST {
    node_arena: Vec<ASTNode>,
    statement_root_node_ids: Vec<ASTNodeId>,
}

impl AST {
    pub fn new() -> Self {
        Self {
            node_arena: Vec::new(),
            statement_root_node_ids: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: ASTNode) -> ASTNodeId {
        let id = self.node_arena.len();
        self.node_arena.push(node);
        ASTNodeId(id)
    }

    pub fn add_statement_root(&mut self, root_node_id: ASTNodeId) {
        self.statement_root_node_ids.push(root_node_id);
    }

    pub fn lookup(&self, id: ASTNodeId) -> &ASTNode {
        &self.node_arena[id.0]
    }

    pub fn lookup_mut(&mut self, id: ASTNodeId) -> &mut ASTNode {
        &mut self.node_arena[id.0]
    }

    pub fn ast_node_id_iter(&self) -> NodeIdIter {
        (0..self.node_arena.len()).map(|id| ASTNodeId(id))
    }
    
    pub fn statement_root_node_id_iter(&self) -> &[ASTNodeId] {
        self.statement_root_node_ids.as_slice()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ASTNodeId(usize);
pub type NodeIdIter = Map<Range<usize>, fn(usize) -> ASTNodeId>;
