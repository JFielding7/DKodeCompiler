use crate::ast::access_node::AccessNode;
use crate::ast::AST;
use crate::ast::ast_node::{Expression, ExpressionId, ItemId, Statement, StatementId};
use crate::ast::ast_node::Expression::Variable;
use crate::ast::ast_node::Item::FunctionDef;
use crate::ast::binary_operator_node::BinaryOperatorNode;
use crate::ast::block::BlockId;
use crate::ast::for_node::ForNode;
use crate::ast::function_call_node::FunctionCallNode;
use crate::ast::function_def_node::FunctionDefNode;
use crate::ast::if_node::IfNode;
use crate::ast::index_node::IndexNode;
use crate::ast::variable_node::VariableNode;
use crate::ast::while_node::WhileNode;
use crate::compiler_context::CompilerContext;
use crate::error::compiler_error::CompilerResult;
use crate::error::compiler_error::SpannableError;
use crate::operators::precedence::OperatorPrecedenceGroup::Assign;
use crate::semantic::error::SemanticError::{DuplicateFunctionName, DuplicateParameterName, UndefinedVariable};
use crate::source::source_span::SourceSpan;

pub struct NameResolver<'ast, 'llvm_ctx> {
    ast: &'ast AST,
    ctx: &'llvm_ctx mut CompilerContext,
    curr_block_id: BlockId,
}

impl<'ast, 'llvm_ctx> NameResolver<'ast, 'llvm_ctx> {
    fn new(ast: &'ast AST, ctx: &'llvm_ctx mut CompilerContext) -> Self {
        Self {
            ast,
            ctx,
            curr_block_id: ast.global_block_id,
        }
    }

    fn resolve_variable(&mut self, var: &VariableNode, span: SourceSpan) -> CompilerResult<()> {
        if self.ctx.symbol_table.contains(var.name, self.curr_block_id) {
            Ok(())
        } else {
            Err(UndefinedVariable(var.name).at(span))
        }
    }

    fn resolve_binary_operator(&mut self, op: &BinaryOperatorNode) -> CompilerResult<()> {
        if op.op_type.precedence_group() == Assign {
            self.resolve_expression(op.right)?;

            let left_node = self.ast.lookup_expression(op.left);
            if let Variable(var) = &left_node.node_type {
                self.ctx.symbol_table.insert_variable(var.name, left_node.span, self.curr_block_id);
            } else {
                self.resolve_expression(op.left)?;
            }
        } else {
            self.resolve_expression(op.left)?;
            self.resolve_expression(op.right)?;
        }

        Ok(())
    }

    fn resolve_function_call(&mut self, call: &FunctionCallNode) -> CompilerResult<()> {
        self.resolve_expression(call.function)?;

        for arg in &call.args {
            self.resolve_expression(*arg)?;
        }

        Ok(())
    }

    fn resolve_index(&mut self, index: &IndexNode) -> CompilerResult<()> {
        self.resolve_expression(index.operand)?;
        self.resolve_expression(index.arg)?;

        Ok(())
    }

    fn resolve_access(&mut self, access: &AccessNode) -> CompilerResult<()> {
        // self.resolve_statement_names(access.receiver)?;

        unimplemented!("property access");
    }

    fn resolve_if(&mut self, if_node: &IfNode) -> CompilerResult<()> {
        for cond_block in &if_node.condition_blocks {
            self.resolve_expression(cond_block.condition)?;
            self.resolve_block(cond_block.body_id)?;
        }

        if let Some(block_id) = if_node.else_body_id {
            self.resolve_block(block_id)?;
        }

        Ok(())
    }

    fn resolve_while(&mut self, while_node: &WhileNode) -> CompilerResult<()> {
        self.resolve_expression(while_node.condition)?;
        self.resolve_block(while_node.body_id)?;

        Ok(())
    }

    fn resolve_for(&mut self, for_node: &ForNode) -> CompilerResult<()> {
        self.resolve_expression(for_node.iterator)?;

        let item_var = &for_node.item_variable;

        self.ctx.symbol_table.insert_variable(item_var.name, item_var.span, for_node.body_id);

        self.resolve_block(for_node.body_id)?;

        Ok(())
    }

    fn resolve_expression(&mut self, expr_id: ExpressionId) -> CompilerResult<()> {
        use Expression::*;

        let expr_node = self.ast.lookup_expression(expr_id);

        Ok(match &expr_node.node_type {
            IntLiteral(_) => {}
            StringLiteral(_) => {}
            Variable(var_node) => {
                self.resolve_variable(var_node, expr_node.span)?;
            }
            UnaryOperator(op) => {
                self.resolve_expression(op.operand_id)?;
            }
            BinaryOperator(op) => {
                self.resolve_binary_operator(op)?;
            }
            FunctionCall(call_node) => {
                self.resolve_function_call(call_node)?;
            }
            Index(index_node) => {
                self.resolve_index(index_node)?;
            }
            Access(_) => {}
        })
    }
    
    fn resolve_return_statement(&mut self, expr_id_opt: Option<ExpressionId>) -> CompilerResult<()> {
        if let Some(expr_id) = expr_id_opt {
            self.resolve_expression(expr_id)?;
        }
        
        Ok(())
    }

    fn resolve_curr_statement(&mut self, statement_root_id: StatementId) -> CompilerResult<()> {
        use Statement::*;

        let node = self.ast.lookup_statement(statement_root_id);

        match &node.node_type {
            ExpressionStatement(expr_id) => {
                self.resolve_expression(*expr_id)?;
            }
            ReturnStatement(expr_id) => {
                self.resolve_return_statement(*expr_id)?;
            }
            If(if_node) => {
                self.resolve_if(if_node)?;
            }
            While(while_node) => {
                self.resolve_while(while_node)?;
            }
            For(for_node) => {
                self.resolve_for(for_node)?;
            }
        };

        Ok(())
    }

    fn resolve_statements(&mut self, statements: &Vec<StatementId>) -> CompilerResult<()> {
        for &statement_root_node_id in statements {
            self.resolve_curr_statement(statement_root_node_id)?;
        }

        Ok(())
    }

    fn resolve_item_blocks(&mut self, items: &Vec<ItemId>) -> CompilerResult<()> {

        for &item_id in items {
            let item = self.ast.lookup_item(item_id);

            match &item.node_type {
                FunctionDef(func_def_node) => {
                    self.resolve_block(func_def_node.body_id)?;
                }
            }
        }

        Ok(())
    }

    fn resolve_function_signature(&mut self, func_def_node: &FunctionDefNode, span: SourceSpan) -> CompilerResult<()> {
        let curr_block_id = self.curr_block_id;

        if !self.ctx.symbol_table.insert_variable(func_def_node.name, span, curr_block_id) {
            return Err(DuplicateFunctionName(func_def_node.name).at(span))
        }

        for (i, param) in func_def_node.params.iter().enumerate() {
            if !self.ctx.symbol_table.insert_function_param(
                param.name, i, param.span, func_def_node.body_id
            ) {
                return Err(DuplicateParameterName(param.name).at(param.span));
            }
        }

        Ok(())
    }

    fn resolve_items(&mut self, items: &Vec<ItemId>) -> CompilerResult<()> {
        for &item_id in items {
            let item = self.ast.lookup_item(item_id);

            match &item.node_type {
                FunctionDef(func_def_node) => {
                    self.resolve_function_signature(func_def_node, item.span)?;
                }
            }
        }

        Ok(())
    }

    fn resolve_block(&mut self, block_id: BlockId) -> CompilerResult<()> {

        let parent_scope_id = self.curr_block_id;
        self.curr_block_id = block_id;

        let block = self.ast.lookup_block(block_id);

        self.resolve_items(&block.items)?;
        self.resolve_item_blocks(&block.items)?;
        self.resolve_statements(&block.statements)?;

        self.curr_block_id = parent_scope_id;

        Ok(())
    }

    pub fn resolve(ast: &'llvm_ctx AST, ctx: &'llvm_ctx mut CompilerContext) -> CompilerResult<()> {

        let mut resolver = NameResolver::new(ast, ctx);
        resolver.resolve_block(ast.global_block_id)?;

        Ok(())
    }
}
