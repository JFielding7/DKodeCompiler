use crate::ast::ast_node::Item::FunctionDef;
use crate::ast::ast_node::{Expression, ExpressionId, ItemId, Statement, StatementId};
use crate::ast::binary_operator_node::BinaryOperatorNode;
use crate::ast::block::BlockId;
use crate::ast::function_call_node::FunctionCallNode;
use crate::ast::function_def_node::FunctionDefNode;
use crate::ast::if_node::IfNode;
use crate::ast::unary_operator_node::UnaryOperatorNode;
use crate::ast::variable_node::VariableNode;
use crate::code_gen::types::LLVMDataTypeEnum::Function;
use crate::code_gen::types::{LLVMDataType, LLVMDataTypeEnum};
use crate::code_gen::value::Value;
use crate::code_gen::value::Value::{RValue, Void};
use crate::compiler_context::symbol_table::symbol::SymbolId;
use crate::compiler_context::CompilerContext;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::unary_operators::UnaryOperator;
use crate::semantic_analysis::AnnotatedAST;
use crate::types::builtin_type::BuiltinType;
use crate::types::data_type::{DataType, DataTypeId, FunctionDataType};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType};
use inkwell::values::{BasicMetadataValueEnum, FunctionValue, PointerValue, ValueKind};
use inkwell::AddressSpace;
use std::collections::HashMap;
use string_interner::DefaultSymbol;
use crate::types::builtin_type::BuiltinType::Str;

pub struct CodeGenerator<'llvm_ctx> {
    llvm_context: &'llvm_ctx Context,
    module: Module<'llvm_ctx>,
    builder: Builder<'llvm_ctx>,
    compiler_context: &'llvm_ctx mut CompilerContext,
    annotated_ast: &'llvm_ctx AnnotatedAST,
    curr_block_id: BlockId,
    curr_function: FunctionValue<'llvm_ctx>,
    pointer_map: HashMap<SymbolId, PointerValue<'llvm_ctx>>,
    data_type_map: HashMap<DataTypeId, LLVMDataType<'llvm_ctx>>,
}

impl<'llvm_ctx> CodeGenerator<'llvm_ctx> {
    const STRING_CONSTRUCTOR: &'llvm_ctx str = "str_new";

    pub fn new(
        annotated_ast: &'llvm_ctx AnnotatedAST,
        llvm_context: &'llvm_ctx Context,
        module: Module<'llvm_ctx>,
        curr_function: FunctionValue<'llvm_ctx>,
        compiler_context: &'llvm_ctx mut CompilerContext
    ) -> Self {
        let builder = llvm_context.create_builder();

        Self {
            llvm_context,
            module,
            builder,
            compiler_context,
            annotated_ast,
            curr_block_id: annotated_ast.ast.global_block_id,
            curr_function,
            pointer_map: HashMap::new(),
            data_type_map: HashMap::new(),
        }
    }

    fn builtin_llvm_type(&self, builtin_type: &BuiltinType) -> LLVMDataTypeEnum<'llvm_ctx> {
        use BuiltinType::*;
        use LLVMDataTypeEnum::*;

        BasicType(match builtin_type {
            BuiltinType::Unit => return LLVMDataTypeEnum::Unit,
            Bool => self.llvm_context.bool_type().into(),
            Int => self.llvm_context.i64_type().into(),
            Str => {
                let i8_ptr = self.llvm_context.ptr_type(AddressSpace::default());
                let i64_type = self.llvm_context.i64_type();

                let string_ty = self.llvm_context.opaque_struct_type("str");
                string_ty.set_body(&[i8_ptr.into(), i64_type.into()], false);

                return StructType(string_ty)
            }
        })
    }

    fn get_function_llvm_type(&self, function_type: &FunctionDataType) -> FunctionType<'llvm_ctx>  {
        use LLVMDataTypeEnum::*;

        let mut params: Vec<BasicMetadataTypeEnum> = Vec::with_capacity(function_type.param_types.len());

        for param_type in &function_type.param_types {
            let data_type = &self.data_type_map.get(param_type).unwrap().data_type;
            params.push(data_type.into());
        }

        let ret_llvm_type = self.data_type_map.get(&function_type.return_type).unwrap();

        let basic_type: BasicTypeEnum = match &ret_llvm_type.data_type {
            Unit => return self.llvm_context.void_type().fn_type(&params, false),
            llvm_data_type => llvm_data_type.into()
        };

        basic_type.fn_type(&params, false)
    }

    fn function_type(&self, data_type: DataTypeId) -> FunctionType<'llvm_ctx> {
        let data_type = self.compiler_context.type_arena.get_data_type(data_type);

        match data_type {
            DataType::Fn(function_type) => {
                self.get_function_llvm_type(function_type)
            },
            _ => unreachable!("Must be a function type")
        }
    }

    fn llvm_data_type_enum(&self, data_type_id: DataTypeId) -> LLVMDataTypeEnum<'llvm_ctx> {
        use DataType::*;

        let data_type = self.compiler_context.type_arena.get_data_type(data_type_id);

        match data_type {
            Builtin(builtin_type) => self.builtin_llvm_type(builtin_type),
            UserDefined(name) => unimplemented!("UserDefined types not implemented"),
            Fn(function_type) => {
                Function(self.get_function_llvm_type(function_type))
            },
        }
    }

    fn emit_int_literal(&self, literal: DefaultSymbol) -> Value<'llvm_ctx> {
        let literal_str = self.compiler_context.string_interner.get_str(literal);
        let int_val = literal_str.parse::<i64>().expect("i64 should be a number");

        RValue(self.llvm_context.i64_type().const_int(int_val as u64, int_val < 0).into())
    }

    fn emit_string_literal(&mut self, literal: DefaultSymbol) -> Value<'llvm_ctx> {
        let str_data_type_id = self.compiler_context.type_arena.builtin_type_id(Str);

        let str_new_symbol = self.compiler_context.string_interner
            .get_intern_symbol(Self::STRING_CONSTRUCTOR);
        let str_new_method = self.data_type_map
            .get(&str_data_type_id)
            .unwrap().get_method(str_new_symbol);

        let literal_str_with_quotes = self.compiler_context.string_interner.get_str(literal);
        let literal_str = &literal_str_with_quotes[1..literal_str_with_quotes.len() - 1].replace("\\\"", "\"");
        let literal_str_length = literal_str.len();

        let length = self.llvm_context.i64_type().const_int(literal_str_length as u64, false);

        let global_str = self.module.add_global(
            self.llvm_context.i8_type().array_type(literal_str_length as u32),
            None,
            "global_str",
        );
        let str_const = self.llvm_context.const_string(literal_str[..literal_str.len()-1].as_bytes(), true);
        global_str.set_initializer(&str_const);
        global_str.set_constant(true);

        let args = [global_str.as_pointer_value().into(), length.into()];

        let call = self.builder.build_call(str_new_method, &args, "").unwrap();

        match call.try_as_basic_value() {
            ValueKind::Basic(value) => RValue(value),
            _ => Void,
        }
    }

    fn emit_variable(&mut self, var_node: &VariableNode, expr_id: ExpressionId) -> Value<'llvm_ctx> {
        let symbol = self.compiler_context.symbol_table.lookup(
            var_node.name, self.curr_block_id
        ).unwrap();

        let data_type_id = self.annotated_ast.expr_data_type_id(expr_id);
        let data_type = &self.data_type_map.get(&data_type_id).unwrap().data_type;
        let llvm_type: BasicTypeEnum = data_type.into();

        let ptr_val = *self.pointer_map.entry(symbol.id).or_insert_with(|| {
            match symbol.func_param_index {
                Some(index) => {
                    let param = self.curr_function
                        .get_nth_param(index as u32)
                        .unwrap();

                    let ptr_val = self.builder.build_alloca(
                        llvm_type,
                        self.compiler_context.string_interner.get_str(var_node.name)
                    ).unwrap();

                    self.builder.build_store(ptr_val, param).unwrap();

                    ptr_val
                }
                None => {
                    self.builder.build_alloca(
                        llvm_type,
                        self.compiler_context.string_interner.get_str(var_node.name)
                    ).unwrap()
                }
            }
        });

        Value::LValue {
            pointee_type: llvm_type,
            ptr: ptr_val
        }
    }

    fn emit_unary_operator(&mut self, op_node: &UnaryOperatorNode) -> Value<'llvm_ctx> {
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

    fn emit_binary_operator(&mut self, op_node: &BinaryOperatorNode) -> Value<'llvm_ctx> {
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
                    unreachable!("assignment LHS of must be LValue")  // TODO: semantic_analysis analysis allows RValue here
                }
            }
            _ => unimplemented!("Emit Binary Operator {}", op_node.op_type),
        }
    }

    fn emit_function_call(&mut self, func_call_node: &FunctionCallNode) -> Value<'llvm_ctx> {
        let mut args: Vec<BasicMetadataValueEnum> = Vec::with_capacity(func_call_node.args.len());

        for &expr_id in &func_call_node.args {
            args.push(self.emit_expression(expr_id).to_rvalue(&self.builder).into());
        }

        let func_type_id = self.annotated_ast.expr_data_type_id(func_call_node.function);
        let func = self.emit_expression(func_call_node.function).to_rvalue(&self.builder);

        let call = self.builder.build_indirect_call(
            self.function_type(func_type_id),
            func.into_pointer_value(),
            &args,
            "fn_call",
        ).unwrap();

        match call.try_as_basic_value() {
            ValueKind::Basic(value) => RValue(value),
            _ => Void,
        }
    }

    fn emit_expression(&mut self, expr_id: ExpressionId) -> Value<'llvm_ctx> {
        use Expression::*;

        let expr_node = self.annotated_ast.ast.lookup_expression(expr_id);

        match &expr_node.node_type {
            IntLiteral(literal) => {
                self.emit_int_literal(*literal)
            }
            StringLiteral(literal) => {
                self.emit_string_literal(*literal)
            }
            Variable(var_node) => {
                self.emit_variable(var_node, expr_id)
            }
            UnaryOperator(op_node) => {
                self.emit_unary_operator(op_node)
            }
            BinaryOperator(op_node) => {
                self.emit_binary_operator(op_node)
            }
            FunctionCall(func_call_node) => {
                self.emit_function_call(func_call_node)
            }
            _ => unimplemented!("Emit Expression {:?}", expr_node.node_type),
        }
    }

    fn emit_if_chain(&mut self, if_node: &IfNode) {
        let function = self.curr_function;
        let merge_block = self.llvm_context.append_basic_block(function, "merge");
        
        for (i, block) in if_node.condition_blocks.iter().enumerate() {
            let cond = self
                .emit_expression(block.condition)
                .to_rvalue(&self.builder)
                .into_int_value();

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
        use Statement::*;

        let statement_node = self.annotated_ast.ast.lookup_statement(stmt_id);

        match &statement_node.node_type {
            ExpressionStatement(expression_id) => {
                self.emit_expression(*expression_id);
            }
            ReturnStatement(expression_id) => {
                self.emit_return_statement(expression_id)
            }
            If(if_node) => {
                self.emit_if_chain(if_node)
            }
            While(_) => {}
            For(_) => {}
        }
    }
    fn emit_function(&mut self, func_def_node: &FunctionDefNode) {
        let func_symbol = self.compiler_context.symbol_table.lookup_expect_exist(
            func_def_node.name, self.curr_block_id
        );
        let ret_type = &self.data_type_map.get(&func_symbol.data_type_id()).unwrap().data_type;
        let return_type: BasicTypeEnum = ret_type.into();

        let mut param_types = Vec::new();

        for param in func_def_node.params.iter() {
            let symbol = self.compiler_context.symbol_table
                .lookup_expect_exist(param.name, func_def_node.body_id);

            let param_type = &self.data_type_map.get(&symbol.data_type_id()).unwrap().data_type;
            param_types.push(param_type.into());
        }

        let fn_type = return_type.fn_type(
            &param_types,
            false,
        );

        let function_name = self.compiler_context.string_interner.get_str(func_def_node.name);
        let new_function = self.module.add_function(function_name, fn_type, None);
        let symbol_id = self.compiler_context.symbol_table
            .lookup_expect_exist(func_def_node.name, self.curr_block_id)
            .id;

        self.pointer_map.insert(symbol_id, new_function.as_global_value().as_pointer_value());

        for (i, param) in func_def_node.params.iter().enumerate() {
            let param_name = self.compiler_context.string_interner.get_str(param.name);
            new_function.get_nth_param(i as u32).unwrap().set_name(param_name);
        }

        let parent_function = self.curr_function;
        self.curr_function = new_function;
        let curr_block = self.builder.get_insert_block().unwrap();

        self.emit_block(func_def_node.body_id);

        self.builder.position_at_end(curr_block);
        self.curr_function = parent_function;
    }

    fn emit_items(&mut self, items: &Vec<ItemId>) {
        for &item_id in items {
            let item = self.annotated_ast.ast.lookup_item(item_id);

            match &item.node_type {
                FunctionDef(func_def_node) => {
                    self.emit_function(func_def_node);
                }
            }
        }
    }

    // Builder cursor at end of the last child block, or this one if no children when finished
    fn emit_block(&mut self, block_id: BlockId) -> BasicBlock<'llvm_ctx> {
        let parent_block_id = self.curr_block_id;
        self.curr_block_id = block_id;

        let llvm_block = self.llvm_context.append_basic_block(self.curr_function, "block");
        self.builder.position_at_end(llvm_block);

        let block = self.annotated_ast.ast.lookup_block(block_id);

        self.emit_items(&block.items);

        for &statement_id in &block.statements {
            self.emit_statement(statement_id);
        }

        self.curr_block_id = parent_block_id;

        llvm_block
    }

    fn add_builtin_method(&mut self, data_type_id: DataTypeId, method_name: &str, func_type: FunctionType<'llvm_ctx>) {
        let llvm_data_type = self.data_type_map.get_mut(&data_type_id).unwrap();

        let fn_value = self.module.add_function(method_name, func_type, None);
        let str_new = self.compiler_context.string_interner
            .get_intern_symbol(method_name);

        llvm_data_type.add_method(str_new, fn_value);
    }

    fn add_builtin_functions(&mut self) {
        let str_type_id = self.compiler_context.type_arena.builtin_type_id(Str);

        let str_llvm_type = &self.data_type_map.get_mut(&str_type_id).unwrap().data_type;
        let str_llvm_basic_type: BasicTypeEnum = str_llvm_type.into();

        let str_new_fn_type = str_llvm_basic_type.fn_type(
            &[self.llvm_context.ptr_type(AddressSpace::default()).into(), self.llvm_context.i64_type().into()],
            false,
        );

        let str_print_fn_type = self.llvm_context.void_type().fn_type(
            &[str_llvm_type.into()],
            false,
        );

        self.add_builtin_method(str_type_id, Self::STRING_CONSTRUCTOR, str_new_fn_type);
        self.add_builtin_method(str_type_id, "str_print", str_print_fn_type);
    }

    fn create_llvm_types(&mut self) {
        for data_type_id in &self.compiler_context.type_arena {
            let llvm_data_type = LLVMDataType::new(self.llvm_data_type_enum(data_type_id));
            self.data_type_map.insert(data_type_id, llvm_data_type);
        }
    }

    pub fn generate_llvm(
        annotated_ast: &'llvm_ctx AnnotatedAST,
        llvm_context: &'llvm_ctx Context,
        compiler_context: &'llvm_ctx mut CompilerContext
    ) -> String {
        let module = llvm_context.create_module("DKode");
        let i32_type = llvm_context.i32_type();
        let main_fn_type = i32_type.fn_type(&[], false);
        let main_fn = module.add_function("main", main_fn_type, None);

        let mut generator = Self::new(annotated_ast, llvm_context, module, main_fn, compiler_context);
        generator.create_llvm_types();
        generator.add_builtin_functions();
        generator.emit_block(generator.annotated_ast.ast.global_block_id);
        generator.builder.build_return(Some(&generator.llvm_context.i32_type().const_zero())).unwrap();

        generator.module.print_to_string().to_string()
    }
}
