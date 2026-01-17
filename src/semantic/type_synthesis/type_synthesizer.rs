use crate::ast::ast_node::{Expression, ExpressionId, ItemId, Statement, StatementId};
use crate::ast::binary_operator_node::BinaryOperatorNode;
use crate::ast::function_call_node::FunctionCallNode;
use crate::ast::function_def_node::FunctionDefNode;
use crate::ast::AST;
use crate::compiler_context::CompilerContext;
use crate::error::compiler_error::CompilerResult;
use crate::error::compiler_error::SpannableError;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::binary_operators::BinaryOperator::Assign;
use crate::operators::unary_operators::UnaryOperator;
use crate::semantic::error::SemanticError::*;
use crate::semantic::type_synthesis::operator_registry::OperatorRegistry;
use crate::source::source_span::SourceSpan;
use crate::types::builtin_type::BuiltinType::{Int, String, Unit};
use crate::types::data_type::{DataType, DataTypeId};
use string_interner::DefaultSymbol;
use crate::ast::ast_node::Expression::Variable;
use crate::ast::ast_node::Item::FunctionDef;
use crate::ast::block::BlockId;
use crate::ast::for_node::ForNode;
use crate::ast::if_node::IfNode;
use crate::ast::while_node::WhileNode;
use crate::compiler_context::scope::ScopeId;

pub struct TypeSynthesizer<'a> {
    ast: &'a AST,
    ctx: &'a mut CompilerContext,
    block_scope_ids: &'a Vec<ScopeId>,
    ast_node_data_types: Vec<Option<DataTypeId>>,
    unary_op_impl: OperatorRegistry<UnaryOperator>,
    binary_op_impl: OperatorRegistry<BinaryOperator>,
    curr_scope_id: ScopeId,
}

impl<'a> TypeSynthesizer<'a> {
    fn new(ast: &'a AST, ctx: &'a mut CompilerContext, block_scope_ids: &'a Vec<ScopeId>) -> Self {
        let ast_node_data_type = vec![None; ast.expression_count()];

        Self {
            ast,
            ctx,
            block_scope_ids,
            ast_node_data_types: ast_node_data_type,
            unary_op_impl: OperatorRegistry::new(),
            binary_op_impl: OperatorRegistry::new(),
            curr_scope_id: ScopeId::global(),
        }
    }

    fn assign_expr_data_type(
        &mut self,
        expr_id: ExpressionId,
        data_type_id: Option<DataTypeId>
    ) {
        self.ast_node_data_types[expr_id.as_usize()] = data_type_id;
    }
    
    fn compute_variable_type(
        &self,
        var_name: DefaultSymbol,
    ) -> CompilerResult<Option<DataTypeId>> {
        match self.ctx.symbol_table.lookup(var_name, self.curr_scope_id) {
            None => Ok(None),
            Some(symbol) => Ok(symbol.data_type)
        }
    }

    fn compute_unary_operation_type(
        &mut self,
        operator_type: UnaryOperator,
        operand_id: ExpressionId
    ) -> CompilerResult<DataTypeId> {
        let operand_node = self.ast.lookup_expression(operand_id);

        let operand_type_id = match self.compute_expression_type(operand_id)? {
            Some(data_type_id) => data_type_id,
            None => return Err(TypeInference.at(operand_node.span)),
        };

        match self.unary_op_impl.operation_type(operator_type, &operand_type_id, &self.ctx.type_arena) {
            Some(data_type) => Ok(data_type),
            None => Err(MismatchedUnaryOperatorTypes(operator_type, operand_type_id)
                .at(operand_node.span)
            ),
        }
    }

    fn assign_variable_type(
        &mut self,
        var_id: ExpressionId,
        data_type_id: DataTypeId
    ) -> CompilerResult<()> {
        let node = self.ast.lookup_expression(var_id);

        if let Variable(var) = &node.node_type {
            println!("Scope: {:?} {}", self.curr_scope_id, self.ctx.string_interner.get_str(var.name));
            self.ctx.symbol_table
                .assign_type(data_type_id, var.name, self.curr_scope_id);

            self.ast_node_data_types[var_id.as_usize()] = Some(data_type_id);
            Ok(())
        } else {
            Err(TypeInference.at(node.span))
        }
    }

    fn compute_binary_operation_type(
        &mut self,
        operator_node: &BinaryOperatorNode,
        operator_span: SourceSpan,
    ) -> CompilerResult<DataTypeId> {

        let right_node = self.ast.lookup_expression(operator_node.right);
        let rhs_type_opt = self.compute_expression_type(operator_node.right)?;

        let rhs_type_id = match rhs_type_opt {
            Some(data_type) => data_type,
            None => {
                return Err(TypeInference.at(right_node.span))
            },
        };

        let left_node = self.ast.lookup_expression(operator_node.left);
        let lhs_type_opt = self.compute_expression_type(operator_node.left)?;

        let lhs_type_id = match lhs_type_opt {
            Some(data_type) => data_type,
            None => {
                return if operator_node.op_type == Assign {
                    self.assign_variable_type(operator_node.left, rhs_type_id)?;
                    Ok(rhs_type_id)
                } else {
                    Err(TypeInference.at(left_node.span))
                }
            }
        };

        match self.binary_op_impl.operation_type(
            operator_node.op_type,
            &(lhs_type_id, rhs_type_id),
            &self.ctx.type_arena
        ) {
            Some(data_type) => {
                Ok(data_type)
            },
            None => Err(MismatchedBinaryOperatorTypes(operator_node.op_type, lhs_type_id, rhs_type_id)
                .at(operator_span)
            ),
        }
    }

    fn check_param_types(
        &mut self,
        param_types: &Vec<DataTypeId>,
        arg_ids: &Vec<ExpressionId>
    ) -> CompilerResult<()> {
        let param_iter = param_types.iter().zip(arg_ids.iter().rev());

        for (&formal_param_type_id, &param_node_id) in param_iter {
            let param_node = self.ast.lookup_expression(param_node_id);

            match self.compute_expression_type(param_node_id)? {
                None => return Err(TypeInference.at(param_node.span)),
                Some(actual_param_type_id) => {
                    if actual_param_type_id != formal_param_type_id {
                        return Err(MismatchedTypes {
                            expected: formal_param_type_id,
                            actual: actual_param_type_id
                        }.at(param_node.span))
                    }
                }
            }
        }

        Ok(())
    }

    fn function_arg_nodes(
        &self,
        func_call_node: &FunctionCallNode
    ) -> Vec<ExpressionId> {
        let mut args_types = Vec::new();

        let mut curr_arg_id = match func_call_node.args {
            Some(args_id) => args_id,
            None => return args_types,
        };

        loop {
            args_types.push(curr_arg_id);

            match &self.ast.lookup_expression(curr_arg_id).node_type {
                Expression::BinaryOperator(op) => {

                    match op.op_type {
                        BinaryOperator::CommaOperator => curr_arg_id = op.left,
                        _ => break
                    }
                }
                _ => break
            }
        }

        args_types
    }

    fn compute_function_call_type(
        &mut self,
        func_call_node: &FunctionCallNode,
        span: SourceSpan,
    ) -> CompilerResult<DataTypeId> {

        let func_node_id = func_call_node.function;
        let func_node = self.ast.lookup_expression(func_node_id);
        let func_type_opt = self.compute_expression_type(func_node_id)?;

        Ok(match func_type_opt {
            Some(func_type) => {
                use DataType::*;

                match self.ctx.type_arena.get_data_type(func_type).clone() {
                    Fn { param_types, return_type } => {
                        let arg_types = self.function_arg_nodes(func_call_node);
                        let actual_args_count = arg_types.len();
                        let expected_args_count = param_types.len();

                        if actual_args_count != expected_args_count {
                            return Err(IncorrectArgumentCount {
                                expected: expected_args_count,
                                actual: actual_args_count
                            }.at(span))
                        }

                        self.check_param_types(&param_types, &arg_types)?;

                        return_type
                    },
                    _ => return Err(FunctionExpected.at(func_node.span))
                }
            },
            None => return Err(TypeInference.at(func_node.span))
        })
    }

    fn compute_return_statement_type(
        &mut self,
        expr_id_opt: Option<ExpressionId>
    ) -> CompilerResult<DataTypeId> {
        use DataType::*;

        let expr_id = match expr_id_opt {
            Some(expr_id) => expr_id,
            None => return Ok(self.ctx.type_arena.builtin_type_id(Unit))
        };

        let node = self.ast.lookup_expression(expr_id);
        let span = node.span;
        let scope_id = self.curr_scope_id;

        let func_name = match self.ctx.symbol_table.scope_function_name(scope_id) {
            Some(func_name) => func_name,
            None => return Err(ReturnStatementOutsideFunction.at(span)),
        };

        let func_type = self.ctx.symbol_table
            .lookup(func_name, scope_id)
            .expect("Function must be defined")
            .data_type
            .expect("Function must have data type");

        let expected_return_type = match self.ctx.type_arena.get_data_type(func_type) {
            Fn { return_type, .. } => *return_type,
            _ => unreachable!("Function must have function type")
        };

        let actual_return_type = match self.compute_expression_type(expr_id)? {
            Some(return_type) => return_type,
            None => return Err(TypeInference.at(span)),
        };

        if actual_return_type == expected_return_type {
            Ok(actual_return_type)
        } else {
            Err(IncorrectReturnType {
                expected: expected_return_type,
                actual: actual_return_type
            }.at(span))
        }
    }

    fn compute_expression_type(
        &mut self,
        expr_id: ExpressionId
    ) -> CompilerResult<Option<DataTypeId>> {
        use Expression::*;

        // if let Some(data_type_id) = self.compute_expression_type(expr_id)? {
        //     return Ok(Some(data_type_id))
        // }

        let node = self.ast.lookup_expression(expr_id);

        let data_type_id = match &node.node_type {
            IntLiteral(_) => {
                Some(self.ctx.type_arena.builtin_type_id(Int))
            },

            StringLiteral(_) => {
                Some(self.ctx.type_arena.builtin_type_id(String))
            },

            Variable(var) => {
                self.compute_variable_type(var.name)?
            },

            UnaryOperator(op) => {
                Some(self.compute_unary_operation_type(op.op_type, op.operand_id)?)
            },

            BinaryOperator(op) => {
                Some(self.compute_binary_operation_type(op, node.span)?)
            },

            FunctionCall(func_call_node) => {
                Some(self.compute_function_call_type(func_call_node, node.span)?)
            }

            _ => unimplemented!("{:?} type resolution unimplemented", node.node_type),
        };

        self.assign_expr_data_type(expr_id, data_type_id);

        Ok(data_type_id)
    }

    fn compute_if_types(
        &mut self,
        if_node: &IfNode
    ) -> CompilerResult<()> {

        for cond_block in &if_node.condition_blocks {
            self.compute_expression_type(cond_block.condition)?;
            self.compute_block_types(cond_block.body)?;
        }

        if let Some(else_block_id) = if_node.else_body {
            self.compute_block_types(else_block_id)?;
        }

        Ok(())
    }

    fn compute_while_types(&mut self, while_node: &WhileNode) -> CompilerResult<()> {
        unimplemented!("While loop types");
        // self.compute_expression_type(while_node.condition)?;

        Ok(())
    }

    fn compute_for_types(&mut self, for_node: &ForNode) -> CompilerResult<()> {
        unimplemented!("For loop types")
        // self.compute_expression_type(for_node.iterator)?;
        //
        // let item_var = &for_node.item_variable;
        //
        // Ok(())
    }

    fn compute_curr_statement_type(&mut self, statement_id: StatementId) -> CompilerResult<()> {
        use Statement::*;

        let node = self.ast.lookup_statement(statement_id);
        println!("{node:?}");

        match &node.node_type {
            ExpressionStatement(expr_id) => {
                self.compute_expression_type(*expr_id)?;
            }
            ReturnStatement(ret_id) => {
                self.compute_return_statement_type(*ret_id)?;
            }
            If(if_node) => {
                self.compute_if_types(if_node)?;
            }
            While(while_node) => {
                self.compute_while_types(while_node)?;
            }
            For(for_node) => {
                self.compute_for_types(for_node)?;
            }
        }

        Ok(())
    }

    fn compute_statement_types(
        &mut self,
        statements: &Vec<StatementId>
    ) -> CompilerResult<()> {
        for &node_id in statements {
            self.compute_curr_statement_type(node_id)?;
        }

        Ok(())
    }

    fn compute_item_block_types(&mut self, items: &Vec<ItemId>) -> CompilerResult<()> {
        for &node_id in items {
            let node = self.ast.lookup_item(node_id);

            let FunctionDef(func_def_node) = &node.node_type;
            self.compute_block_types(func_def_node.body)?;
        }

        Ok(())
    }

    fn compute_function_param_types(
        &mut self,
        func_def_node: &FunctionDefNode,
    ) -> CompilerResult<Vec<DataTypeId>> {
        let body_scope_id = self.block_scope_ids[func_def_node.body.as_usize()];

        let mut param_types = Vec::new();

        for param in &func_def_node.params {
            // TODO: generics
            let type_id = self.ctx.type_arena
                .get_type_id(param.type_annotation.type_name, &self.ctx.string_interner)
                .ok_or_else(|| UndefinedType.at(param.span))?;

            self.ctx.symbol_table.assign_type(type_id, param.name, body_scope_id);

            param_types.push(type_id);
        }

        Ok(param_types)
    }

    fn compute_function_return_type(
        &self,
        func_def_node: &FunctionDefNode
    ) -> CompilerResult<DataTypeId> {
        Ok(match &func_def_node.return_type {
            None => self.ctx.type_arena.builtin_type_id(Unit),
            Some(ret) => {
                if ret.inner_types.len() > 0 {
                    unimplemented!("Generic type annotations")
                }

                // TODO: generics
                let return_type_symbol = ret.type_name;
                self.ctx.type_arena
                    .get_type_id(return_type_symbol, &self.ctx.string_interner)
                    .ok_or_else(|| UndefinedType.at(ret.span))?
            }
        })
    }

    fn compute_function_signature_types(
        &mut self,
        func_def_node: &FunctionDefNode,
    ) -> CompilerResult<()> {
        let param_types = self.compute_function_param_types(func_def_node)?;
        let return_type = self.compute_function_return_type(func_def_node)?;

        let function_type = DataType::Fn {
            param_types,
            return_type
        };

        let function_type_id = self.ctx.type_arena.add_if_new_type(function_type);

        self.ctx.symbol_table.assign_type(
            function_type_id, func_def_node.name, self.curr_scope_id
        );

        Ok(())
    }

    fn compute_item_types(&mut self, items: &Vec<ItemId>) -> CompilerResult<()> {
        for &node_id in items {
            let node = self.ast.lookup_item(node_id);

            let FunctionDef(func_def_node) = &node.node_type;
            self.compute_function_signature_types(func_def_node)?;
        }

        Ok(())
    }

    fn compute_block_types(&mut self, block_id: BlockId) -> CompilerResult<()> {
        let parent_scope_id = self.curr_scope_id;
        self.curr_scope_id = self.block_scope_ids[block_id.as_usize()];

        let block = self.ast.lookup_block(block_id);

        self.compute_item_types(&block.items)?;
        self.compute_item_block_types(&block.items)?;
        self.compute_statement_types(&block.statements)?;

        self.curr_scope_id = parent_scope_id;

        Ok(())
    }

    pub fn synthesize(ast: &AST, ctx: &mut CompilerContext, block_scope_ids: &Vec<ScopeId>) -> CompilerResult<Vec<DataTypeId>> {
        let mut synthesizer = TypeSynthesizer::new(&ast, ctx, block_scope_ids);
        synthesizer.compute_block_types(ast.global_block_id)?;

        let mut ast_node_types = Vec::with_capacity(
            synthesizer.ast_node_data_types.len()
        );

        for (i, &node_data_type) in synthesizer.ast_node_data_types.iter().enumerate() {
            match node_data_type {
                None => return Err(TypeInference.at(
                    ast.lookup_expression(ExpressionId::new(i)).span)
                ),
                Some(node_type) => {
                    ast_node_types.push(node_type);
                }
            }
        }

        Ok(ast_node_types)
    }
}
