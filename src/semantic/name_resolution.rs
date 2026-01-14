use crate::ast::access_node::AccessNode;
use crate::ast::arena_ast::AST;
use crate::ast::ast_node::ASTNodeType::{FunctionDef, Variable};
use crate::ast::ast_node::{ASTNode, ASTNodeId, ASTNodeType};
use crate::ast::binary_operator_node::BinaryOperatorNode;
use crate::ast::for_node::ForNode;
use crate::ast::function_call_node::FunctionCallNode;
use crate::ast::function_def_node::FunctionDefNode;
use crate::ast::if_node::IfNode;
use crate::ast::index_node::IndexNode;
use crate::ast::variable_node::VariableNode;
use crate::ast::while_node::WhileNode;
use crate::compiler_context::CompilerContext;
use crate::error::spanned_error::SpannableError;
use crate::operators::precedence::OperatorPrecedenceGroup::Assign;
use crate::semantic::error::SemanticError::{DuplicateParameterName, UndefinedVariable};
use crate::semantic::error::SemanticResult;

pub struct NameResolver<'a> {
    ast: &'a AST,
    ctx: &'a mut CompilerContext,
}

impl<'a> NameResolver<'a> {
    fn new(ast: &'a AST, ctx: &'a mut CompilerContext) -> Self {
        Self {
            ast,
            ctx,
        }
    }

    fn resolve_variable(&mut self, var: &VariableNode, wrapper_node: &ASTNode) -> SemanticResult<()> {
        if self.ctx.symbol_table.contains(var.name, wrapper_node.scope_id) {
            Ok(())
        } else {
            Err(UndefinedVariable.at(wrapper_node.span))
        }
    }

    fn handle_binary_operator(&mut self, op: &BinaryOperatorNode) -> SemanticResult<()> {
        if op.op_type.precedence_group() == Assign {
            self.resolve_statement_names(op.right)?;

            let left_node = self.ast.lookup(op.left);
            if let Variable(var) = &left_node.node_type {
                self.ctx.symbol_table.insert(var.name, left_node.span, left_node.scope_id);
            } else {
                self.resolve_statement_names(op.left)?;
            }
        } else {
            self.resolve_statement_names(op.left)?;
            self.resolve_statement_names(op.right)?;
        }

        Ok(())
    }

    fn resolve_function_call(&mut self, call: &FunctionCallNode) -> SemanticResult<()> {
        self.resolve_statement_names(call.function)?;

        if let Some(args) = call.args {
            self.resolve_statement_names(args)?;
        }

        Ok(())
    }

    fn resolve_index(&mut self, index: &IndexNode) -> SemanticResult<()> {
        self.resolve_statement_names(index.operand)?;
        self.resolve_statement_names(index.arg)?;

        Ok(())
    }

    fn resolve_access(&mut self, access: &AccessNode) -> SemanticResult<()> {
        // self.resolve_statement_names(access.receiver)?;

        unimplemented!("property access");
    }

    fn resolve_if(&mut self, if_node: &IfNode) -> SemanticResult<()> {
        self.resolve_statement_names(if_node.if_condition())
    }

    fn resolve_while(&mut self, while_node: &WhileNode) -> SemanticResult<()> {
        self.resolve_statement_names(while_node.condition)
    }

    fn resolve_for(&mut self, for_node: &ForNode) -> SemanticResult<()> {
        let item_var = &for_node.item_variable;
        self.ctx.symbol_table.insert(item_var.name, item_var.span, for_node.body.scope_id);

        self.resolve_statement_names(for_node.iterator)
    }

    fn resolve_statement_names(&mut self, statement_root_id: ASTNodeId) -> SemanticResult<()> {
        use ASTNodeType::*;

        let node = self.ast.lookup(statement_root_id);

        match &node.node_type {
            Variable(var_node) => self.resolve_variable(var_node, node)?,
            BinaryOperator(op) => self.handle_binary_operator(op)?,
            UnaryOperator(op) => self.resolve_statement_names(op.operand)?,
            FunctionCall(call) => self.resolve_function_call(call)?,
            Index(index) => self.resolve_index(index)?,
            Access(access) => self.resolve_access(access)?,
            If(if_node) => self.resolve_if(if_node)?,
            While(while_node) => self.resolve_while(while_node)?,
            For(for_node) => self.resolve_for(for_node)?,

            FunctionDef(_) => {}
            IntLiteral(_) => {}
            StringLiteral(_) => {}
        };

        Ok(())
    }

    fn resolve_statements(&mut self) -> SemanticResult<()> {
        for statement_root_node_id in self.ast.statement_root_node_id_iter() {
            self.resolve_statement_names(*statement_root_node_id)?;
        }

        Ok(())
    }

    fn resolve_function_def(&mut self, func_def_node: &FunctionDefNode, wrapper_node: &ASTNode) -> SemanticResult<()> {
        self.ctx.symbol_table.insert(func_def_node.name, wrapper_node.span, wrapper_node.scope_id);

        for param in &func_def_node.params {
            if !self.ctx.symbol_table.insert(
                param.name, param.span, func_def_node.body.scope_id
            ) {
                return Err(DuplicateParameterName.at(param.span));
            }
        }

        Ok(())
    }

    fn resolve_functions(&mut self) -> SemanticResult<()> {
        for node_id in self.ast.ast_node_id_iter() {
            let node = self.ast.lookup(node_id);

            if let FunctionDef(func_def_node) = &node.node_type {
                self.resolve_function_def(func_def_node, node)?;
            }
        }

        Ok(())
    }

    pub fn resolve(ast: &'a AST, ctx: &'a mut CompilerContext) -> SemanticResult<()> {
        let mut resolver = NameResolver::new(ast, ctx);
        resolver.resolve_functions()?;
        resolver.resolve_statements()?;

        Ok(())
    }
}
