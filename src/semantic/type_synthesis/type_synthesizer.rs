use crate::ast::arena_ast::AST;
use crate::ast::ast_node::ASTNodeType::{FunctionDef, Variable};
use crate::ast::ast_node::{ASTNode, ASTNodeId, ASTNodeType};
use crate::ast::binary_operator_node::BinaryOperatorNode;
use crate::ast::function_def_node::FunctionDefNode;
use crate::compiler_context::scope::ScopeId;
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
use crate::ast::function_call_node::FunctionCallNode;

pub struct TypeSynthesizer<'a> {
    ast: &'a AST,
    ast_node_data_types: Vec<Option<DataTypeId>>,
    unary_op_impl: OperatorRegistry<UnaryOperator>,
    binary_op_impl: OperatorRegistry<BinaryOperator>,
    ctx: &'a mut CompilerContext,
}

impl<'a> TypeSynthesizer<'a> {
    fn new(ast: &'a AST, ctx: &'a mut CompilerContext) -> Self {
        let ast_node_data_type = vec![None; ast.node_count()];

        Self {
            ast,
            ast_node_data_types: ast_node_data_type,
            unary_op_impl: OperatorRegistry::new(),
            binary_op_impl: OperatorRegistry::new(),
            ctx,
        }
    }

    fn get_node_data_type(&self, node_id: ASTNodeId) -> Option<DataTypeId> {
        self.ast_node_data_types[node_id.as_usize()]
    }

    fn assign_node_data_type(&mut self, node_id: ASTNodeId, data_type_id: Option<DataTypeId>) {
        self.ast_node_data_types[node_id.as_usize()] = data_type_id;
    }
    
    fn compute_variable_type(
        &self,
        var_name: DefaultSymbol,
        scope_id: ScopeId
    ) -> CompilerResult<Option<DataTypeId>> {
        match self.ctx.symbol_table.lookup(var_name, scope_id) {
            None => Ok(None),
            Some(symbol) => Ok(symbol.data_type)
        }
    }

    fn compute_unary_operation_type(
        &self,
        operator_type: UnaryOperator,
        operand: ASTNodeId
    ) -> CompilerResult<DataTypeId> {
        let operand_node = self.ast.lookup(operand);

        let operand_type_id = match self.ast_node_data_types[operand.as_usize()] {
            Some(e) => e,
            None => return Err(TypeInference.at(operand_node.span)),
        };

        match self.unary_op_impl.operation_type(operator_type, &operand_type_id, &self.ctx.type_arena) {
            Some(data_type) => Ok(data_type),
            None => Err(MismatchedUnaryOperatorTypes(operator_type, operand_type_id)
                .at(operand_node.span)
            ),
        }
    }

    fn assign_variable_type(&mut self, var_id: ASTNodeId, data_type_id: DataTypeId) -> CompilerResult<()> {
        let node = self.ast.lookup(var_id);

        if let Variable(var) = &node.node_type {
            self.ctx.symbol_table.assign_type(data_type_id, var.name, node.scope_id);
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

        let right_node = self.ast.lookup(operator_node.right);
        let rhs_type_opt = self.get_node_data_type(operator_node.right);

        let rhs_type_id = match rhs_type_opt {
            Some(data_type) => data_type,
            None => {
                return Err(TypeInference.at(right_node.span))
            },
        };

        let left_node = self.ast.lookup(operator_node.left);
        let lhs_type_opt = self.get_node_data_type(operator_node.left);

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

    fn compute_function_param_types(
        &mut self,
        func_def_node: &FunctionDefNode
    ) -> CompilerResult<Vec<DataTypeId>> {
        let mut param_types = Vec::new();

        for param in &func_def_node.params {
            let type_id = self.ctx.type_arena
                .get_type_id(param.type_annotation.type_name, &self.ctx.string_interner)
                .ok_or_else(|| UndefinedType.at(param.span))?;
            self.ctx.symbol_table.assign_type(type_id, param.name, func_def_node.body.scope_id);

            param_types.push(type_id);
        }

        Ok(param_types)
    }

    fn compute_function_return_type(&self, func_def_node: &FunctionDefNode) -> CompilerResult<DataTypeId> {
        Ok(match &func_def_node.return_type {
            None => self.ctx.type_arena.builtin_type_id(Unit),
            Some(ret) => {
                if ret.inner_types.len() > 0 {
                    unimplemented!("Generic type annotations")
                }

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
        scope_id: ScopeId
    ) -> CompilerResult<DataTypeId> {
        let param_types = self.compute_function_param_types(func_def_node)?;
        let return_type = self.compute_function_return_type(func_def_node)?;

        let function_type = DataType::Fn {
            param_types,
            return_type
        };

        let function_type_id = self.ctx.type_arena.add_if_new_type(function_type);
        self.ctx.symbol_table.assign_type(function_type_id, func_def_node.name, scope_id);

        Ok(self.ctx.type_arena.builtin_type_id(Unit))
    }

    fn check_param_types(&self, param_types: &Vec<DataTypeId>, arg_ids: &Vec<ASTNodeId>) -> CompilerResult<()> {
        let param_iter = param_types.iter().zip(arg_ids.iter().rev());

        for (&formal_param_type_id, &param_node_id) in param_iter {
            let param_node = self.ast.lookup(param_node_id);

            match self.get_node_data_type(param_node_id) {
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

    fn function_arg_nodes(&self, func_call_node: &FunctionCallNode) -> Vec<ASTNodeId> {
        let mut args_types = Vec::new();

        let mut curr_arg_id = match func_call_node.args {
            Some(args_id) => args_id,
            None => return args_types,
        };

        loop {
            args_types.push(curr_arg_id);

            match &self.ast.lookup(curr_arg_id).node_type {
                ASTNodeType::BinaryOperator(op) => {

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
        &self,
        func_call_node: &FunctionCallNode,
        span: SourceSpan,
    ) -> CompilerResult<DataTypeId> {
        let func_node_id = func_call_node.function;
        let func_node = self.ast.lookup(func_node_id);
        let func_type_opt = self.get_node_data_type(func_node_id);

        Ok(match func_type_opt {
            Some(func_type) => {
                use DataType::*;

                match self.ctx.type_arena.get_data_type(func_type) {
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

                        *return_type
                    },
                    _ => return Err(FunctionExpected.at(func_node.span))
                }
            },
            None => return Err(TypeInference.at(func_node.span))
        })
    }

    fn compute_type(&mut self, ast_node_id: ASTNodeId) -> CompilerResult<Option<DataTypeId>> {
        use ASTNodeType::*;

        let node = self.ast.lookup(ast_node_id);

        if let Some(data_type_id) = self.get_node_data_type(ast_node_id) {
            return Ok(Some(data_type_id));
        }

        let data_type = match &node.node_type {
            IntLiteral(_) => Some(self.ctx.type_arena.builtin_type_id(Int)),
            StringLiteral(_) => Some(self.ctx.type_arena.builtin_type_id(String)),

            Variable(var) => self.compute_variable_type(var.name, node.scope_id)?,

            UnaryOperator(op) => {
                Some(self.compute_unary_operation_type(op.op_type, op.operand)?)
            },

            BinaryOperator(op) => {
                Some(self.compute_binary_operation_type(
                    op, node.span
                )?)
            },

            If(_) => {
                Some(self.ctx.type_arena.builtin_type_id(Unit))
            }

            FunctionDef(_) => {
                unreachable!("function definition should already be synthesized")
            }

            FunctionCall(func_call_node) => {
                Some(self.compute_function_call_type(func_call_node, node.span)?)
            }

            _ => unimplemented!("{:?} type resolution unimplemented", node.node_type),
        };

        Ok(data_type)
    }

    fn compute_ast_types(&mut self) -> CompilerResult<()> {
        for node_id in self.ast.ast_node_id_iter() {
            let data_type_id = self.compute_type(node_id)?;
            self.assign_node_data_type(node_id, data_type_id);
        }

        Ok(())
    }

    fn compute_function_types(&mut self) -> CompilerResult<()> {
        for node_id in self.ast.ast_node_id_iter() {
            let node = self.ast.lookup(node_id);

            if let FunctionDef(func_def_node) = &node.node_type {
                let data_type_id = self.compute_function_signature_types(func_def_node, node.scope_id)?;
                self.assign_node_data_type(node_id, Some(data_type_id));
            }
        }

        Ok(())
    }

    pub fn synthesize(ast: &AST, ctx: &mut CompilerContext) -> CompilerResult<Vec<DataTypeId>> {
        let mut synthesizer = TypeSynthesizer::new(&ast, ctx);
        synthesizer.compute_function_types()?;
        synthesizer.compute_ast_types()?;

        let mut ast_node_types = Vec::with_capacity(synthesizer.ast_node_data_types.len());

        for (i, &node_data_type) in synthesizer.ast_node_data_types.iter().enumerate() {
            match node_data_type {
                None => return Err(TypeInference.at(
                    ast.lookup(ASTNodeId(i)).span
                )),
                Some(node_type) => {
                    ast_node_types.push(node_type);
                }
            }
        }

        Ok(ast_node_types)
    }
}
