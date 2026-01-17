use crate::ast::ast_node::{ASTNode, Expression, ExpressionId, Item, ItemId, Statement, StatementId};
use crate::ast::block::{Block, BlockId};
use crate::source::source_span::SourceSpan;

pub mod function_def_node;
pub mod binary_operator_node;
pub mod index_node;
pub mod ast_node;
pub mod unary_operator_node;
pub mod if_node;
pub mod access_node;
pub mod function_call_node;
pub mod while_node;
pub mod for_node;
pub mod variable_node;
pub mod block;

#[derive(Debug)]
pub struct AST {
    item_arena: Vec<ASTNode<Item>>,
    block_arena: Vec<Block>,
    statement_arena: Vec<ASTNode<Statement>>,
    expr_arena: Vec<ASTNode<Expression>>,
    pub global_block_id: BlockId,
}

impl AST {
    pub fn new() -> Self {
        Self {
            item_arena: Vec::new(),
            block_arena: Vec::new(),
            statement_arena: Vec::new(),
            expr_arena: Vec::new(),
            global_block_id: BlockId::new(0),
        }
    }

    pub fn add_item(&mut self, item: Item, span: SourceSpan) -> ItemId {
        let ast_node = ASTNode::new(item, span);

        let id = self.item_arena.len();
        self.item_arena.push(ast_node);
        ItemId::new(id)
    }

    pub fn create_block(&mut self) -> BlockId {
        let id = self.block_arena.len();
        self.block_arena.push(Block::new());
        BlockId::new(id)
    }

    pub fn add_statement(&mut self, statement: Statement, span: SourceSpan) -> StatementId {
        let ast_node = ASTNode::new(statement, span);

        let id = self.statement_arena.len();
        self.statement_arena.push(ast_node);
        StatementId::new(id)
    }

    pub fn add_expression(&mut self, expr: Expression, span: SourceSpan) -> ExpressionId {
        let ast_node = ASTNode::new(expr, span);

        let id = self.expr_arena.len();
        self.expr_arena.push(ast_node);
        ExpressionId::new(id)
    }
    
    pub fn items(&self) -> &Vec<ASTNode<Item>> {
        &self.item_arena
    }

    pub fn lookup_item(&self, id: ItemId) -> &ASTNode<Item> {
        &self.item_arena[id.as_usize()]
    }

    pub fn lookup_statement(&self, id: StatementId) -> &ASTNode<Statement> {
        &self.statement_arena[id.as_usize()]
    }

    pub fn lookup_expression(&self, id: ExpressionId) -> &ASTNode<Expression> {
        &self.expr_arena[id.as_usize()]
    }

    pub fn lookup_block(&self, id: BlockId) -> &Block {
        &self.block_arena[id.as_usize()]
    }

    pub fn lookup_block_mut(&mut self, id: BlockId) -> &mut Block {
        &mut self.block_arena[id.as_usize()]
    }

    pub fn block_count(&self) -> usize {
        self.block_arena.len()
    }

    pub fn expression_count(&self) -> usize {
        self.expr_arena.len()
    }
}


