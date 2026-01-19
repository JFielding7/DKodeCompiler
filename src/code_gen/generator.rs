use std::collections::HashMap;
use crate::ast::ast_node::{Expression, ExpressionId, Statement, StatementId};
use crate::ast::binary_operator_node::BinaryOperatorNode;
use crate::ast::block::BlockId;
use crate::ast::if_node::IfNode;
use crate::ast::unary_operator_node::UnaryOperatorNode;
use crate::ast::variable_node::VariableNode;
use crate::code_gen::value::{LValueType, Value};
use crate::code_gen::value::Value::RValue;
use crate::compiler_context::CompilerContext;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::unary_operators::UnaryOperator;
use crate::semantic::AnnotatedAST;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{FunctionValue, PointerValue};
use string_interner::DefaultSymbol;
use crate::compiler_context::symbol_table::symbol::SymbolId;

pub struct CodeGenerator<'ctx> {
    llvm_context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    compiler_context: &'ctx CompilerContext,
    annotated_ast: &'ctx AnnotatedAST,
    curr_block_id: BlockId,
    curr_function: FunctionValue<'ctx>,
    pointer_map: HashMap<SymbolId, PointerValue<'ctx>>,
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(annotated_ast: &'ctx AnnotatedAST, llvm_context: &'ctx Context, compiler_context: &'ctx CompilerContext) -> Self {
        let module = llvm_context.create_module("code");
        let builder = llvm_context.create_builder();
        let i32_type = llvm_context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let main_fn = module.add_function("main", fn_type, None);

        Self {
            llvm_context,
            module,
            builder,
            compiler_context,
            annotated_ast,
            curr_block_id: annotated_ast.ast.global_block_id,
            curr_function: main_fn,
            pointer_map: HashMap::new(),
        }
    }

    fn emit_int_literal(&self, literal: DefaultSymbol) -> Value<'ctx> {
        let literal_str = self.compiler_context.string_interner.get_str(literal);
        let int_val = literal_str.parse::<i64>().expect("i64 should be a number");

        RValue(self.llvm_context.i64_type().const_int(int_val as u64, int_val < 0).into())
    }

    fn emit_variable(&mut self, var_node: &VariableNode) -> Value<'ctx> {
        let symbol = self.compiler_context.symbol_table.lookup(
            var_node.name, self.curr_block_id
        ).unwrap();

        let alloc = *self.pointer_map.entry(symbol.id).or_insert_with(|| {
            self.builder.build_alloca(
                self.llvm_context.i64_type(),
                self.compiler_context.string_interner.get_str(var_node.name)
            ).unwrap()
        });

        Value::LValue {
            pointee_type: BasicTypeEnum::from(self.llvm_context.i64_type()),
            ptr: alloc
        }
    }

    fn emit_unary_operator(&mut self, op_node: &UnaryOperatorNode) -> Value<'ctx> {
        use UnaryOperator::*;

        let value = self
            .emit_expression(op_node.operand_id).to_rvalue(&self.builder)
            .into_int_value();

        RValue(match op_node.op_type {
            Neg => {
                self.builder.build_int_neg(value, "neg").unwrap().into()
            }
            Not | BitNot => {
                self.builder.build_not(value, "bit_not").unwrap().into()
            }
            _ => unimplemented!("Pre/Post Inc/Dec"),
        })
    }

    fn emit_binary_operator(&mut self, op_node: &BinaryOperatorNode) -> Value<'ctx> {
        use BinaryOperator::*;

        let lhs_val = self.emit_expression(op_node.left);
        let rhs_val = self.emit_expression(op_node.right).to_rvalue(&self.builder);
        let builder = &self.builder;

        match op_node.op_type {
            Add => {
                RValue(builder.build_int_add(
                    lhs_val.to_rvalue(builder).into_int_value(),
                    rhs_val.into_int_value(),
                    ""
                ).unwrap().into())
            },
            Sub => {
                RValue(builder.build_int_sub(
                    lhs_val.to_rvalue(builder).into_int_value(),
                    rhs_val.into_int_value(),
                    ""
                ).unwrap().into())
            },
            Mul => {
                RValue(builder.build_int_mul(
                    lhs_val.to_rvalue(builder).into_int_value(),
                    rhs_val.into_int_value(),
                    ""
                ).unwrap().into())
            },
            Div => {
                RValue(builder.build_int_signed_div(
                    lhs_val.to_rvalue(builder).into_int_value(),
                    rhs_val.into_int_value(),
                    ""
                ).unwrap().into())
            },
            Mod => {
                RValue(builder.build_int_signed_rem(
                    lhs_val.to_rvalue(builder).into_int_value(),
                    rhs_val.into_int_value(),
                    ""
                ).unwrap().into())
            },
            Assign => {
                if let Value::LValue { ptr, .. } = &lhs_val {
                    self.builder.build_store(*ptr, rhs_val).unwrap();
                    lhs_val
                } else {
                    unreachable!("assignment LHS of must be LValue")
                }
            }
            _ => unimplemented!("Emit Binary Operator {}", op_node.op_type),
        }
    }

    fn emit_expression(&mut self, expr_id: ExpressionId) -> Value<'ctx> {
        use Expression::*;

        let expr_node = self.annotated_ast.ast.lookup_expression(expr_id);

        match &expr_node.node_type {
            IntLiteral(literal) => {
                self.emit_int_literal(*literal)
            }

            Variable(var_node) => {
                self.emit_variable(var_node)
            }

            UnaryOperator(op_node) => {
                self.emit_unary_operator(op_node)
            }

            BinaryOperator(op_node) => {
                self.emit_binary_operator(op_node)
            },
            _ => unimplemented!("Emit Expression {:?}", expr_node.node_type),
        }
    }

    fn emit_if_chain(&mut self, if_node: &IfNode) {
        let function = self.curr_function;
        let merge_block = self.llvm_context.append_basic_block(function, "merge");
        
        for (i, block) in if_node.condition_blocks.iter().enumerate() {
            let cond = self.emit_expression(block.condition).to_rvalue(&self.builder).into_int_value();

            let current_block = self.builder.get_insert_block().unwrap();

            let true_block = self.emit_block(block.body_id);
            self.builder.build_unconditional_branch(merge_block).unwrap();

            if i < if_node.condition_blocks.len() - 1 {
                let false_block = self.llvm_context.append_basic_block(function, "false");

                self.builder.position_at_end(current_block);
                self.builder.build_conditional_branch(cond, true_block, false_block).unwrap();

                self.builder.position_at_end(false_block);
            } else {
                let else_block = if let Some(block_id) = if_node.else_body_id {
                    let else_block = self.emit_block(block_id);
                    self.builder.build_unconditional_branch(merge_block).unwrap();
                    else_block
                } else {
                    merge_block
                };

                self.builder.position_at_end(current_block);
                self.builder.build_conditional_branch(cond, true_block, else_block).unwrap();
            }
        }

        self.builder.position_at_end(merge_block);
    }

    fn emit_return_statement(&mut self, expression_id: &Option<ExpressionId>) {
        match expression_id {
            Some(expr_id) => {
                let val = self.emit_expression(*expr_id).to_rvalue(&self.builder);
                self.builder.build_return(Some(&val)).unwrap();
            }
            None => {
                self.builder.build_return(None).unwrap();
            }
        }
    }

    fn emit_statement(&mut self, stmt_id: StatementId) {
        let statement_node = self.annotated_ast.ast.lookup_statement(stmt_id);

        match &statement_node.node_type {
            Statement::ExpressionStatement(expression_id) => {
                self.emit_expression(*expression_id);
            }
            Statement::ReturnStatement(expression_id) => {
                self.emit_return_statement(expression_id)
            }
            Statement::If(if_node) => {
                self.emit_if_chain(if_node)
            }
            Statement::While(_) => {}
            Statement::For(_) => {}
        }
    }
    
    fn emit_block(&mut self, block_id: BlockId) -> BasicBlock<'ctx> {
        let parent_block_id = self.curr_block_id;
        self.curr_block_id = block_id;

        let llvm_block = self.llvm_context.append_basic_block(self.curr_function, "block");
        self.builder.position_at_end(llvm_block);

        let block = self.annotated_ast.ast.lookup_block(block_id);

        for &statement_id in &block.statements {
            self.emit_statement(statement_id);
        }

        self.curr_block_id = parent_block_id;

        llvm_block
    }

    pub fn generate_llvm(
        annotated_ast: &'ctx AnnotatedAST,
        llvm_context: &'ctx Context,
        compiler_context: &'ctx CompilerContext
    ) {
        let mut generator = Self::new(annotated_ast, llvm_context, compiler_context);

        generator.emit_block(generator.annotated_ast.ast.global_block_id);

        println!("{}", generator.module.print_to_string().to_string());
    }
}
