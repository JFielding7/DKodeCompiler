use crate::ast::arena_ast::AST;
use crate::ast::ast_node::ASTNodeType::Variable;
use crate::ast::ast_node::{ASTNodeId, ASTNodeType};
use crate::ast::function_def_node::{FunctionDefNode, Parameter};
use crate::compiler_context::scope::ScopeId;
use crate::compiler_context::CompilerContext;
use crate::error::spanned_error::SpannableError;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::binary_operators::BinaryOperator::Assign;
use crate::operators::unary_operators::UnaryOperator;
use crate::semantic::error::SemanticError::*;
use crate::semantic::error::SemanticResult;
use crate::semantic::type_synthesis::operator_registry::OperatorRegistry;
use crate::source::source_span::SourceSpan;
use crate::types::builtin_type::BuiltinType::{Int, String, Unit};
use crate::types::data_type::{DataType, DataTypeId};
use crate::types::type_annotation::TypeAnnotation;
use string_interner::DefaultSymbol;
use crate::ast::binary_operator_node::BinaryOperatorNode;

pub struct TypeSynthesizer<'a> {
    ast: &'a AST,
    ast_node_data_type: Vec<Option<DataTypeId>>,
    unary_op_impl: OperatorRegistry<UnaryOperator>,
    binary_op_impl: OperatorRegistry<BinaryOperator>,
    ctx: &'a mut CompilerContext,
}

impl<'a> TypeSynthesizer<'a> {
    fn new(ast: &'a AST, ctx: &'a mut CompilerContext) -> Self {
        let ast_node_data_type = vec![None; ast.node_count()];

        Self {
            ast,
            ast_node_data_type,
            unary_op_impl: OperatorRegistry::new(),
            binary_op_impl: OperatorRegistry::new(),
            ctx,
        }
    }

    fn get_node_data_type(&self, node_id: ASTNodeId) -> Option<DataTypeId> {
        self.ast_node_data_type[node_id.as_usize()]
    }

    fn assign_node_data_type(&mut self, node_id: ASTNodeId, data_type_id: Option<DataTypeId>) {
        self.ast_node_data_type[node_id.as_usize()] = data_type_id;
    }
    
    fn compute_variable_type(
        &self, 
        var_name: DefaultSymbol, 
        scope_id: ScopeId
    ) -> SemanticResult<Option<DataTypeId>> {
        match self.ctx.symbol_table.lookup(var_name, scope_id) {
            None => Ok(None),
            Some(symbol) => Ok(symbol.data_type)
        }
    }

    fn compute_unary_operation_type(
        &self,
        operator_type: UnaryOperator,
        operand: ASTNodeId
    ) -> SemanticResult<DataTypeId> {
        let operand_node = self.ast.lookup(operand);

        let operand_type_id = match self.ast_node_data_type[operand.as_usize()] {
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

    fn assign_variable_type(&mut self, var_id: ASTNodeId, data_type_id: DataTypeId) -> SemanticResult<()> {
        let node = self.ast.lookup(var_id);

        if let Variable(var) = &node.node_type {
            self.ctx.symbol_table.assign_type(data_type_id, var.name, node.scope_id);
            self.ast_node_data_type[var_id.as_usize()] = Some(data_type_id);
            Ok(())
        } else {
            Err(TypeInference.at(node.span))
        }
    }

    fn compute_binary_operation_type(
        &mut self,
        operator_node: &BinaryOperatorNode,
        operator_span: SourceSpan,
    ) -> SemanticResult<DataTypeId> {

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
            Some(data_type) => Ok(data_type),
            None => Err(MismatchedBinaryOperatorTypes(operator_node.op_type, lhs_type_id, rhs_type_id)
                .at(operator_span)
            ),
        }
    }

    fn compute_function_param_types(
        &mut self, 
        func_def_node: &FunctionDefNode
    ) -> SemanticResult<Vec<DataTypeId>> {
        let mut param_types = Vec::new();

        for param in &func_def_node.params {
            let type_id = self.ctx.type_arena
                .get_type_id(param.name, &self.ctx.string_interner)
                .ok_or_else(|| UndefinedType.at(param.span))?;
            self.ctx.symbol_table.assign_type(type_id, param.name, func_def_node.body.scope_id);

            param_types.push(type_id);
        }

        Ok(param_types)
    }

    fn compute_function_signature_types(
        &mut self, 
        func_def_node: &FunctionDefNode, 
        scope_id: ScopeId
    ) -> SemanticResult<DataTypeId> {
        let param_types = self.compute_function_param_types(func_def_node)?;
        
        let return_type = match &func_def_node.return_type {
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
        };

        let function_type = DataType::Fn {
            param_types,
            return_type
        };

        let function_type_id = self.ctx.type_arena.add_type(function_type);
        self.ctx.symbol_table.assign_type(function_type_id, func_def_node.name, scope_id);

        Ok(self.ctx.type_arena.builtin_type_id(Unit))
    }

    fn compute_type(&mut self, ast_node_id: ASTNodeId) -> SemanticResult<Option<DataTypeId>> {
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

            If(if_node) => {
                unimplemented!("If node")
                // self.compute_type(if_node.if_condition())?
            }

            FunctionDef(func_def_node) => {
               Some(self.compute_function_signature_types(func_def_node, node.scope_id)?)
            }

            _ => unimplemented!("{:?} type resolution unimplemented", node.node_type),
        };

        Ok(data_type)
    }

    fn compute_ast_types(&mut self) -> SemanticResult<()> {
        for node_id in self.ast.ast_node_id_iter() {
            let data_type_id = self.compute_type(node_id)?;
            self.assign_node_data_type(node_id, data_type_id);
        }

        Ok(())
    }

    pub fn synthesize(ast: &mut AST, ctx: &mut CompilerContext) -> SemanticResult<()> {
        let mut synthesizer = TypeSynthesizer::new(ast, ctx);
        synthesizer.compute_ast_types()?;
        
        for data_type_id in synthesizer.ast_node_data_type {
            
        }
        
        Ok(())
    }
}
