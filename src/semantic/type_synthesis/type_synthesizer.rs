use crate::ast::arena_ast::{ASTNodeId, AST};
use crate::ast::ast_node::ASTNodeType;
use crate::compiler_context::scope::ScopeId;
use crate::compiler_context::type_arena::DataTypeId;
use crate::compiler_context::CompilerContext;
use crate::error::spanned_error::SpannableError;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::binary_operators::BinaryOperator::Assign;
use crate::operators::unary_operators::UnaryOperator;
use crate::semantic::error::SemanticError::*;
use crate::semantic::error::SemanticResult;
use crate::semantic::type_synthesis::operator_registry::OperatorRegistry;
use crate::source::source_span::SourceSpan;
use crate::types::data_type::BuiltinType::{Int, String};
use string_interner::DefaultSymbol;
use crate::ast::ast_node::ASTNodeType::Variable;

pub struct TypeSynthesizer<'a> {
    ast: AST,
    unary_op_impl: OperatorRegistry<UnaryOperator>,
    binary_op_impl: OperatorRegistry<BinaryOperator>,
    ctx: &'a mut CompilerContext,
    curr_scope: ScopeId,
}

impl<'a> TypeSynthesizer<'a> {
    fn new(ast: AST, ctx: &'a mut CompilerContext) -> Self {
        Self {
            ast,
            unary_op_impl: OperatorRegistry::new(),
            binary_op_impl: OperatorRegistry::new(),
            ctx,
            curr_scope: ScopeId::new(0),
        }
    }
    
    fn compute_variable_type(&self, var_name: DefaultSymbol) -> SemanticResult<Option<DataTypeId>> {
        println!("looking");

        match self.ctx.symbol_table.lookup(var_name, self.curr_scope) {
            None => Ok(None),
            Some(symbol) => {
                println!("found {:?}", symbol.data_type);
                Ok(symbol.data_type)
            }
        }
    }

    fn compute_unary_operation_type(
        &self,
        operator_type: UnaryOperator,
        operand: ASTNodeId
    ) -> SemanticResult<DataTypeId> {

        let operand_node = self.ast.lookup(operand);

        let operand_type_id = match operand_node.data_type_id {
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

    fn compute_binary_operation_type(
        &mut self,
        operator_type: BinaryOperator,
        left: ASTNodeId,
        right: ASTNodeId,
        operator_span: SourceSpan,
    ) -> SemanticResult<DataTypeId> {

        let right_node = self.ast.lookup(right);
        let rhs_type_opt = right_node.data_type_id;

        let rhs_type_id = match rhs_type_opt {
            Some(data_type) => data_type,
            None => {
                return Err(TypeInference.at(right_node.span))
            },
        };

        let left_node = self.ast.lookup(left);
        let lhs_type_opt = left_node.data_type_id;

        let lhs_type_id = match lhs_type_opt {
            Some(data_type) => data_type,
            None => {
                return if operator_type == Assign {
                    if let Variable(var) = &self.ast.lookup(left).node_type {
                        self.ctx.symbol_table.assign_type(rhs_type_id, var.name, left_node.scope_id);
                    } else {
                        unimplemented!("Assign to non variable node")
                    }

                    self.ast.lookup_mut(left).data_type_id = rhs_type_opt;
                    Ok(rhs_type_id)
                } else {
                    Err(TypeInference.at(left_node.span))
                }
            }
        };

        match self.binary_op_impl.operation_type(
            operator_type,
            &(lhs_type_id, rhs_type_id),
            &self.ctx.type_arena
        ) {
            Some(data_type) => Ok(data_type),
            None => Err(MismatchedBinaryOperatorTypes(operator_type, lhs_type_id, rhs_type_id)
                .at(operator_span)
            ),
        }
    }

    fn compute_type(&mut self, ast_node_id: ASTNodeId) -> SemanticResult<Option<DataTypeId>> {
        use ASTNodeType::*;

        let node = self.ast.lookup(ast_node_id);
        self.curr_scope = node.scope_id;

        if let Some(_) = node.data_type_id {
            return Ok(node.data_type_id);
        }

        let data_type = match &node.node_type {
            IntLiteral(_) => Some(self.ctx.type_arena.builtin_type_id(Int)),
            StringLiteral(_) => Some(self.ctx.type_arena.builtin_type_id(String)),

            Variable(var) => self.compute_variable_type(var.name)?,

            UnaryOperator(op) => {
                Some(self.compute_unary_operation_type(op.op_type, op.operand)?)
            },

            BinaryOperator(op) => {
                Some(self.compute_binary_operation_type(
                    op.op_type, op.left, op.right, node.span
                )?)
            },

            If(if_node) => {
                self.compute_type(if_node.if_condition())?
            }

            _ => unimplemented!("{:?} type resolution unimplemented", node.node_type),
        };

        Ok(data_type)
    }

    fn compute_ast_types(&mut self) -> SemanticResult<()> {
        for node_id in self.ast.ast_node_id_iter() {
            let data_type_id = self.compute_type(node_id)?;
            self.ast.lookup_mut(node_id).data_type_id = data_type_id;
        }

        Ok(())
    }

    pub fn synthesize(ast: AST, ctx: &mut CompilerContext) -> SemanticResult<AST> {
        let mut synthesizer = TypeSynthesizer::new(ast, ctx);
        synthesizer.compute_ast_types()?;
        Ok(synthesizer.ast)
    }
}
