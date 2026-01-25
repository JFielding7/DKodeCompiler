use crate::ast::ast_node::Item::{ClassDef, FunctionDef};
use crate::ast::ast_node::{Expression, ExpressionId, ItemId, Statement, StatementId};
use crate::ast::binary_operator_node::BinaryOperatorNode;
use crate::ast::block::BlockId;
use crate::ast::function_call_node::FunctionCallNode;
use crate::ast::function_def_node::FunctionDefNode;
use crate::ast::if_node::IfNode;
use crate::ast::unary_operator_node::UnaryOperatorNode;
use crate::ast::variable_node::VariableNode;
use crate::code_generation::types::LLVMDataType;
use crate::code_generation::types::LLVMDataType::Function;
use crate::code_generation::value::Value;
use crate::code_generation::value::Value::{RValue, Void};
use crate::compiler_context::CompilerContext;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::unary_operators::UnaryOperator;
use crate::phase::symbol_table::symbol::SymbolType::FunctionParam;
use crate::phase::symbol_table::symbol::SymbolType;
use crate::phase::symbol_table::SymbolTable;
use crate::phase::types::builtin_type::BuiltinType;
use crate::phase::types::builtin_type::BuiltinType::Str;
use crate::phase::types::data_type::DataTypeEnum::{Builtin, Fn, UserDefined};
use crate::phase::types::data_type::{DataType, DataTypeId, FunctionDataTypeId, Method};
use crate::phase::types::type_arena::TypeArena;
use crate::phase::MultiPhase;
use crate::semantic_analysis::{SemanticAnalysis, SemanticAnalysisOutput};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType};
use inkwell::values::{BasicMetadataValueEnum, FunctionValue, ValueKind};
use inkwell::AddressSpace;
use string_interner::DefaultSymbol;
use crate::code_generation::CodeGeneration;

pub struct CodeGenerator<'llvm_ctx> {
    llvm_context: &'llvm_ctx Context,
    module: Module<'llvm_ctx>,
    builder: Builder<'llvm_ctx>,
    semantic_analysis_output: &'llvm_ctx SemanticAnalysisOutput,
    symbol_table: SymbolTable<CodeGeneration<'llvm_ctx>>,
    type_arena: TypeArena<CodeGeneration<'llvm_ctx>>,
    curr_block_id: BlockId,
    curr_function: FunctionValue<'llvm_ctx>,
    compiler_context: &'llvm_ctx mut CompilerContext,
}

impl<'llvm_ctx> CodeGenerator<'llvm_ctx> {
    const STRING_CONSTRUCTOR: &'llvm_ctx str = "str_new";

    pub fn new(
        semantic_analysis_output: &'llvm_ctx SemanticAnalysisOutput,
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
            symbol_table: SymbolTable::code_generation_symbol_table(&semantic_analysis_output.symbol_table),
            type_arena: TypeArena::code_generation_type_arena(),
            curr_block_id: semantic_analysis_output.ast.global_block_id,
            curr_function,
            semantic_analysis_output,
        }
    }
    
    fn emit_int_literal(&self, literal: DefaultSymbol) -> Value<'llvm_ctx> {
        let literal_str = self.compiler_context.string_interner.get_str(literal);
        let int_val = literal_str.parse::<i64>().expect("i64 should be a number");

        RValue(self.llvm_context.i64_type().const_int(int_val as u64, int_val < 0).into())
    }

    fn emit_string_literal(&mut self, literal: DefaultSymbol) -> Value<'llvm_ctx> {
        let str_data_type_id = self.type_arena.get_builtin_type_id(Str);

        let str_new_symbol = self.compiler_context.string_interner
            .get_intern_symbol(Self::STRING_CONSTRUCTOR);
        let str_new_method = self.type_arena
            .get_data_type(str_data_type_id)
            .get_method(str_new_symbol).function_repr;
        
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
        let semantic_symbol = self.semantic_analysis_output.symbol_table.lookup_expect_exist(var_node.name, self.curr_block_id);

        let symbol = match self.symbol_table.lookup(var_node.name, self.curr_block_id) {
            Some(symbol) => symbol,
            None => {
                let basic_type: BasicTypeEnum = self.type_arena.get_data_type(semantic_symbol.data_type_id).data_type_repr.into();

                let var_ptr = self.builder.build_alloca(
                    basic_type,
                    self.compiler_context.string_interner.get_str(var_node.name)
                ).unwrap();

                self.symbol_table.lower_semantic_symbol(semantic_symbol, var_ptr, self.curr_block_id)
            }
        };

        let data_type_id = self.semantic_analysis_output.expr_data_type_id(expr_id);
        let data_type = self.type_arena.get_data_type(data_type_id).data_type_repr;
        let llvm_type: BasicTypeEnum = data_type.into();

        let var_name = self.compiler_context.string_interner.get_str(var_node.name);

        let ptr_val = match symbol.symbol_type {
            FunctionParam(index) => {
                let param = self.curr_function
                    .get_nth_param(index as u32)
                    .unwrap();

                let ptr_val = self.builder.build_alloca(
                    llvm_type,
                    var_name
                ).unwrap();

                self.builder.build_store(ptr_val, param).unwrap();

                ptr_val
            }
            SymbolType::Variable => {
                symbol.llvm_variable
            }
            SymbolType::ClassField(_) => {
                unimplemented!("ClassField is not implemented")
            }
            SymbolType::Class => {
                unimplemented!("Class is not implemented")
            }
        };

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

        let func_type_id = self.semantic_analysis_output.expr_data_type_id(func_call_node.function);
        let func = self.emit_expression(func_call_node.function).to_rvalue(&self.builder);

        let call = self.builder.build_indirect_call(
            self.type_arena.get_data_type(func_type_id).data_type_repr.function_type(),
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

        let expr_node = self.semantic_analysis_output.ast.lookup_expression(expr_id);

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

        let statement_node = self.semantic_analysis_output.ast.lookup_statement(stmt_id);

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
            While(_) => {
                unimplemented!("Emit While Loop")
            }
            For(_) => {
                unimplemented!("Emit For Loop")
            }
        }
    }

    fn emit_function_body(&mut self, func_body_id: BlockId, lowered_function: FunctionValue<'llvm_ctx>) {
        let parent_function = self.curr_function;
        self.curr_function = lowered_function;

        let curr_block = self.builder.get_insert_block().unwrap();
        self.emit_block(func_body_id);
        self.builder.position_at_end(curr_block);

        self.curr_function = parent_function;
    }

    fn lower_function_symbol(&mut self, func_def_node: &FunctionDefNode) -> FunctionValue<'llvm_ctx> {
        let semantic_func_symbol = self.semantic_analysis_output.symbol_table.lookup_expect_exist(
            func_def_node.name, self.curr_block_id
        );

        let ret_type = self.type_arena.get_data_type(semantic_func_symbol.data_type_id).data_type_repr;
        let return_type: BasicTypeEnum = ret_type.into();

        let mut param_types = Vec::new();

        for param in func_def_node.params.iter() {
            let symbol = self.semantic_analysis_output.symbol_table
                .lookup_expect_exist(param.name, func_def_node.body_id);

            let param_type = self.type_arena.get_data_type(symbol.data_type_id).data_type_repr;
            param_types.push(param_type.into());
        }

        let fn_type = return_type.fn_type(
            &param_types,
            false,
        );

        let function_name = self.compiler_context.string_interner.get_str(func_def_node.name);
        let new_function = self.module.add_function(function_name, fn_type, None);

        for (i, param) in func_def_node.params.iter().enumerate() {
            let param_name = self.compiler_context.string_interner.get_str(param.name);
            new_function.get_nth_param(i as u32).unwrap().set_name(param_name);
        }

        self.symbol_table.lower_semantic_symbol(
            semantic_func_symbol, 
            new_function.as_global_value().as_pointer_value(), 
            self.curr_block_id
        );

        new_function
    }

    fn emit_items(&mut self, items: &Vec<ItemId>) {
        let mut lowered_functions = Vec::new();

        for &item_id in items {
            let item = self.semantic_analysis_output.ast.lookup_item(item_id);

            match &item.node_type {
                FunctionDef(func_def_node) => {
                    lowered_functions.push((func_def_node.body_id, self.lower_function_symbol(func_def_node)));
                }
                ClassDef(_) => {
                    unimplemented!("Classes")
                }
            }
        }

        for (func_body_id, lowered_function) in lowered_functions {
            self.emit_function_body(func_body_id, lowered_function);
        }
    }

    // Builder cursor at end of the last child block, or this one if no children when finished
    fn emit_block(&mut self, block_id: BlockId) -> BasicBlock<'llvm_ctx> {
        let parent_block_id = self.curr_block_id;
        self.curr_block_id = block_id;

        let llvm_block = self.llvm_context.append_basic_block(self.curr_function, "block");
        self.builder.position_at_end(llvm_block);

        let block = self.semantic_analysis_output.ast.lookup_block(block_id);

        self.emit_items(&block.items);

        for &statement_id in &block.statements {
            self.emit_statement(statement_id);
        }

        self.curr_block_id = parent_block_id;

        llvm_block
    }

    fn lower_builtin_data_type(&self, builtin_type: &BuiltinType) -> LLVMDataType<'llvm_ctx> {
        use BuiltinType::*;
        use LLVMDataType::*;

        BasicType(match builtin_type {
            BuiltinType::Unit => return LLVMDataType::Unit,
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

    fn lower_function_data_type(&self, function_type_id: FunctionDataTypeId) -> FunctionType<'llvm_ctx>  {
        use LLVMDataType::*;

        let function_type = self.semantic_analysis_output.type_arena.get_function_data_type(function_type_id);
        let mut params: Vec<BasicMetadataTypeEnum> = Vec::with_capacity(function_type.param_types.len());

        for &param_type_id in &function_type.param_types {
            let data_type = self.type_arena.get_data_type(param_type_id).data_type_repr;
            params.push(data_type.into());
        }

        let ret_llvm_type = self.type_arena.get_data_type(function_type.return_type);

        let basic_type: BasicTypeEnum = match ret_llvm_type.data_type_repr {
            Unit => return self.llvm_context.void_type().fn_type(&params, false),
            llvm_data_type => llvm_data_type.into()
        };

        basic_type.fn_type(&params, false)
    }

    fn lower_method(&self, method_name_symbol: DefaultSymbol, method: &Method<<SemanticAnalysis as MultiPhase>::LastPhase>) -> Method<CodeGeneration<'llvm_ctx>> {
        let func_data_type = self.type_arena.get_function_data_type(method.data_type_id);

        let return_type: BasicTypeEnum = self.type_arena.get_data_type(func_data_type.return_type).data_type_repr.into();

        let mut param_types: Vec<BasicMetadataTypeEnum> = Vec::with_capacity(func_data_type.param_types.len());

        for &param_type_id in &func_data_type.param_types {
            let param_type = self.type_arena.get_data_type(param_type_id);
            param_types.push(param_type.data_type_repr.into());
        }

        let fn_type = return_type.fn_type(
            &param_types,
            false,
        );

        let method_name = self.compiler_context.string_interner.get_str(method_name_symbol);
        let fn_value = self.module.add_function(method_name, fn_type, None);
        
        Method::new(method.data_type_id, fn_value)
    }

    fn lower_data_type_methods(&mut self, data_type_id: DataTypeId) {
        let semantic_data_type = self.semantic_analysis_output.type_arena.get_data_type(data_type_id);

        for (&name, method) in &semantic_data_type.methods {
            let lowered_method = self.lower_method(name, method);
            self.type_arena.get_data_type_mut(data_type_id).add_method(name, lowered_method);
        }
    }

    fn lower_data_type(&mut self, data_type: &DataType<<SemanticAnalysis as MultiPhase>::LastPhase>) {

        let llvm_data_type = match &data_type.data_type_kind {
            Builtin(builtin_type) => self.lower_builtin_data_type(builtin_type),
            UserDefined(_) => unimplemented!("UserDefined types not implemented"),
            Fn(function_type) => {
                Function(self.lower_function_data_type(*function_type))
            },
        };

        self.type_arena.add_new_type(DataType::new(data_type.data_type_kind.clone(), llvm_data_type));
    }

    fn lower_semantic_data_types(&mut self) {

        for semantic_data_type in &self.semantic_analysis_output.type_arena.data_types {
            self.lower_data_type(semantic_data_type);
        }

        for data_type_id in (0..self.semantic_analysis_output.type_arena.data_types.len()).map(|i| DataTypeId::new(i)) {
            self.lower_data_type_methods(data_type_id);
        }
    }

    pub fn generate_llvm(
        annotated_ast: &'llvm_ctx SemanticAnalysisOutput,
        llvm_context: &'llvm_ctx Context,
        compiler_context: &'llvm_ctx mut CompilerContext
    ) -> String {
        let module = llvm_context.create_module("DKode");
        let i32_type = llvm_context.i32_type();
        let main_fn_type = i32_type.fn_type(&[], false);
        let main_fn = module.add_function("main", main_fn_type, None);

        let mut generator = Self::new(annotated_ast, llvm_context, module, main_fn, compiler_context);
        generator.lower_semantic_data_types();
        // generator.add_builtin_functions();
        generator.emit_block(generator.semantic_analysis_output.ast.global_block_id);
        generator.builder.build_return(Some(&generator.llvm_context.i32_type().const_zero())).unwrap();

        generator.module.print_to_string().to_string()
    }
}
