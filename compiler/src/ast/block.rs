use crate::ast::ast_node::{ItemId, StatementId};

#[derive(Debug)]
pub struct Block {
    pub items: Vec<ItemId>,
    pub statements: Vec<StatementId>,
}

impl Block {
    pub fn new() -> Self {
        Self { 
            items: Vec::new(),
            statements: Vec::new(), 
        }
    }
    
    pub fn add_item(&mut self, item: ItemId) {
        self.items.push(item);
    }
    
    pub fn add_statement(&mut self, statement: StatementId) {
        self.statements.push(statement);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BlockId(usize);

impl BlockId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
    
    pub fn as_usize(&self) -> usize {
        self.0
    }
}
