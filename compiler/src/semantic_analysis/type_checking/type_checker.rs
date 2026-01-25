use crate::ast::ast_node::Expression::Variable;
use crate::ast::ast_node::Item::{ClassDef, FunctionDef};
use crate::ast::ast_node::{Expression, ExpressionId, ItemId, Statement, StatementId};
use crate::ast::binary_operator_node::BinaryOperatorNode;
use crate::ast::block::BlockId;
use crate::ast::class_def_node::ClassDefNode;
use crate::ast::for_node::ForNode;
use crate::ast::function_call_node::FunctionCallNode;
use crate::ast::function_def_node::FunctionDefNode;
use crate::ast::if_node::IfNode;
use crate::ast::while_node::WhileNode;
use crate::ast::AST;
use crate::compiler_context::CompilerContext;
use crate::error::compiler_error::CompilerResult;
use crate::error::compiler_error::SpannableError;
use crate::operators::binary_operators::BinaryOperator::Assign;
use crate::operators::unary_operators::UnaryOperator;
use crate::phase::symbol_table::SymbolTable;
use crate::phase::types::builtin_type::BuiltinType::{Int, Str, Unit};
use crate::phase::types::data_type::DataTypeEnum::{Fn, UserDefined};
use crate::phase::types::data_type::{DataType, DataTypeId, Field, FunctionDataType, FunctionDataTypeId, Method};
use crate::phase::types::type_arena::TypeArena;
use crate::semantic_analysis::error::SemanticError::*;
use crate::source::source_span::SourceSpan;
use crate::semantic_analysis::name_resolution::NameResolution;
use crate::semantic_analysis::type_checking::TypeChecking;
use string_interner::DefaultSymbol;
use crate::ast::access_node::{AccessNode, MemberType};
use crate::ast::typed_variable::TypedVariable;

pub struct TypeChecker<'ctx> {
    ast: &'ctx AST,
    name_resolution_symbol_table: SymbolTable<NameResolution>,
    symbol_table: SymbolTable<TypeChecking>,
    type_arena: TypeArena<TypeChecking>,
    ctx: &'ctx mut CompilerContext,
    ast_expr_data_types: Vec<DataTypeId>,
    curr_block_id: BlockId,
    curr_class: Option<DataTypeId>
}

impl<'ast> TypeChecker<'ast> {
    fn new(
        ast: &'ast AST,
        name_resolution_symbol_table: SymbolTable<NameResolution>,
        ctx: &'ast mut CompilerContext
    ) -> Self {
        Self {
            ast,
            symbol_table: SymbolTable::type_checking_symbol_table(&name_resolution_symbol_table),
            name_resolution_symbol_table,
            type_arena: TypeArena::type_checking_type_arena(),
            ctx,
            ast_expr_data_types: vec![DataTypeId::new(0); ast.expression_count()],
            curr_block_id: ast.global_block_id,
            curr_class: None,
        }
    }

    fn assign_expr_data_type(
        &mut self,
        expr_id: ExpressionId,
        data_type_id: DataTypeId
    ) {
        self.ast_expr_data_types[expr_id.as_usize()] = data_type_id;
    }

    fn compute_variable_type(
        &self,
        var_name: DefaultSymbol,
        var_span: SourceSpan
    ) -> CompilerResult<DataTypeId> {
        match self.symbol_table.lookup(var_name, self.curr_block_id) {
            None => Err(TypeInference.at(var_span)),
            Some(symbol) => Ok(symbol.data_type_id)
        }
    }

    fn compute_unary_operation_type(
        &mut self,
        operator_type: UnaryOperator,
        operand_id: ExpressionId
    ) -> CompilerResult<DataTypeId> {
        let operand_node = self.ast.lookup_expression(operand_id);
        let operand_type_id =self.compute_expression_type(operand_id)?;

        match self.symbol_table.unary_op_impl.operation_type(operator_type, &operand_type_id, &self.type_arena) {
            Some(data_type) => Ok(data_type),
            None => {
                let operand_type = self.type_arena.format_type(operand_type_id, self.ctx);

                Err(MismatchedUnaryOperatorTypes { operator_type, operand_type }
                    .at(operand_node.span)
                )
            },
        }
    }

    fn compute_binary_operation_type(
        &mut self,
        operator_node: &BinaryOperatorNode,
        operator_span: SourceSpan,
    ) -> CompilerResult<DataTypeId> {

        let rhs_type_id = self.compute_expression_type(operator_node.right)?;

        if operator_node.op_type == Assign {
            let left_node_id = operator_node.left;
            let left_node = self.ast.lookup_expression(left_node_id);

            if let Variable(var) = &left_node.node_type {
                let symbol = self.name_resolution_symbol_table.lookup_expect_exist(
                    var.name, self.curr_block_id
                );

                let var_type_id = self.symbol_table.assign_symbol_type(
                    symbol, rhs_type_id, self.curr_block_id
                ).data_type_id;

                if var_type_id != rhs_type_id {
                    let var_type = self.type_arena.format_type(var_type_id, self.ctx);
                    let rhs_data_type = self.type_arena.format_type(rhs_type_id, self.ctx);

                    return Err(InvalidAssignment {
                            lhs_data_type: var_type,
                            rhs_data_type,
                        }.at(operator_span)
                    )
                }

                self.assign_expr_data_type(left_node_id, rhs_type_id);

                return Ok(rhs_type_id)
            }
        }

        let lhs_type_id = self.compute_expression_type(operator_node.left)?;

        match self.symbol_table.binary_op_impl.operation_type(
            operator_node.op_type,
            &(lhs_type_id, rhs_type_id),
            &self.type_arena
        ) {
            Some(data_type) => {
                Ok(data_type)
            },
            None => {
                let lhs_data_type = self.type_arena.format_type(lhs_type_id, self.ctx);
                let rhs_data_type = self.type_arena.format_type(rhs_type_id, self.ctx);

                Err(MismatchedBinaryOperatorTypes {
                        op: operator_node.op_type,
                        lhs_data_type,
                        rhs_data_type
                    }.at(operator_span)
                )
            },
        }
    }

    fn check_param_types(
        &mut self,
        function_data_type_id: FunctionDataTypeId,
        arg_ids: &Vec<ExpressionId>
    ) -> CompilerResult<()> {

        let function_type = self.type_arena.get_function_data_type(function_data_type_id);
        let param_types = function_type.param_types.clone(); // TODO: avoid cloning

        for (&formal_param_type_id, &param_node_id) in param_types.iter().zip(arg_ids) {
            let param_node = self.ast.lookup_expression(param_node_id);

            let actual_param_type_id = self.compute_expression_type(param_node_id)?;

            if actual_param_type_id != formal_param_type_id {
                let expected = self.type_arena.format_type(formal_param_type_id, self.ctx);
                let actual = self.type_arena.format_type(actual_param_type_id, self.ctx);

                return Err(MismatchedTypes {
                    expected,
                    actual
                }.at(param_node.span))
            }
        }

        Ok(())
    }

    fn compute_function_call_type(
        &mut self,
        func_call_node: &FunctionCallNode,
        span: SourceSpan,
    ) -> CompilerResult<DataTypeId> {

        let func_node_id = func_call_node.function;
        let func_node = self.ast.lookup_expression(func_node_id);
        let func_type = self.compute_expression_type(func_node_id)?;

        match self.type_arena.get_data_type(func_type).data_type_kind {
            Fn(function_type_id) => {
                let arg_expressions = &func_call_node.args;
                let actual_args_count = arg_expressions.len();
                let expected_args_count = self.type_arena
                    .get_function_data_type(function_type_id)
                    .param_types
                    .len();

                if actual_args_count != expected_args_count {
                    return Err(IncorrectArgumentCount {
                        expected: expected_args_count,
                        actual: actual_args_count
                    }.at(span))
                }

                self.check_param_types(function_type_id, &arg_expressions)?;

                Ok(self.type_arena.get_function_data_type(function_type_id).return_type)
            },
            _ => Err(FunctionExpected.at(func_node.span))
        }
    }

    fn compute_return_statement_type(
        &mut self,
        expr_id_opt: Option<ExpressionId>
    ) -> CompilerResult<DataTypeId> {

        let expr_id = match expr_id_opt {
            Some(expr_id) => expr_id,
            None => return Ok(self.type_arena.get_builtin_type_id(Unit))
        };

        let node = self.ast.lookup_expression(expr_id);
        let span = node.span;
        let scope_id = self.curr_block_id;

        let func_name = match self.symbol_table.scope_function_name(scope_id) {
            Some(func_name) => func_name,
            None => return Err(ReturnStatementOutsideFunction.at(span)),
        };

        let func_type = self.symbol_table
            .lookup(func_name, scope_id)
            .expect("Function must be defined")
            .data_type_id;

        let expected_return_type = match &self.type_arena.get_data_type(func_type).data_type_kind {
            Fn(function_type_id) => self.type_arena.get_function_data_type(*function_type_id).return_type,
            _ => unreachable!("Function must have function type")
        };

        let actual_return_type = self.compute_expression_type(expr_id)?;

        if actual_return_type == expected_return_type {
            Ok(actual_return_type)
        } else {
            let expected = self.type_arena.format_type(expected_return_type, self.ctx);
            let actual = self.type_arena.format_type(actual_return_type, self.ctx);

            Err(IncorrectReturnType {
                expected,
                actual
            }.at(span))
        }
    }

    fn compute_access_type(&mut self, access_node: &AccessNode) -> CompilerResult<DataTypeId> {
        use MemberType;

        let receiver_type_id = self.compute_expression_type(access_node.receiver)?;
        let receiver_type = self.type_arena.get_data_type(receiver_type_id);

        let member = &access_node.member;

        match &member.member_type {
            MemberType::Field => {
                let field = receiver_type.get_field(member.name);
                Ok(field.data_type_id)
            }
            MemberType::Method(arg_ids) => {
                let method = receiver_type.get_method(member.name);
                let method_data_type_id = method.data_type_id;

                self.check_param_types(method.data_type_id, &arg_ids)?;
                Ok(self.type_arena.get_function_data_type(method_data_type_id).return_type)
            },
        }
    }

    fn compute_expression_type(
        &mut self,
        expr_id: ExpressionId
    ) -> CompilerResult<DataTypeId> {
        use Expression::*;

        let node = self.ast.lookup_expression(expr_id);

        let data_type_id = match &node.node_type {
            IntLiteral(_) => {
                self.type_arena.get_builtin_type_id(Int)
            }

            StringLiteral(_) => {
                self.type_arena.get_builtin_type_id(Str)
            }

            Variable(var) => {
                self.compute_variable_type(var.name, node.span)?
            }

            UnaryOperator(op) => {
                self.compute_unary_operation_type(op.op_type, op.operand_id)?
            }

            BinaryOperator(op) => {
                self.compute_binary_operation_type(op, node.span)?
            }

            FunctionCall(func_call_node) => {
                self.compute_function_call_type(func_call_node, node.span)?
            }

            Access(access_node) => {
                self.compute_access_type(access_node)?
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
            self.compute_block_types(cond_block.body_id, None)?;
        }

        if let Some(else_block_id) = if_node.else_body_id {
            self.compute_block_types(else_block_id, None)?;
        }

        Ok(())
    }

    fn compute_while_types(&mut self, _: &WhileNode) -> CompilerResult<()> {
        unimplemented!("While loop types");
        // self.compute_expression_type(while_node.condition)?;

        // Ok(())
    }

    fn compute_for_types(&mut self, _: &ForNode) -> CompilerResult<()> {
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

            match &node.node_type {
                FunctionDef(func_def_node) => {
                    self.compute_block_types(func_def_node.body_id, None)?;
                }
                ClassDef(class_def_node) => {
                    let class_type_id = self.type_arena
                        .get_type_id(class_def_node.class_type.type_name, self.ctx).unwrap();
                    self.compute_block_types(class_def_node.body_id, Some(class_type_id))?;
                }
            }
        }

        Ok(())
    }

    fn compute_var_list_types(&mut self, vars: &Vec<TypedVariable>, block_id: BlockId) -> CompilerResult<Vec<DataTypeId>> {
        vars
            .iter()
            .map(|var| {
                // TODO: generics
                let data_type_id = self.type_arena
                    .get_type_id(var.type_annotation.type_name, self.ctx)
                    .ok_or_else(|| UndefinedType.at(var.span))?;

                let symbol = self.name_resolution_symbol_table.lookup_expect_exist(var.name, block_id);

                self.symbol_table.assign_symbol_type(symbol, data_type_id, block_id);

                Ok(data_type_id)
            })
            .collect()
    }

    fn compute_function_param_types(
        &mut self,
        func_def_node: &FunctionDefNode,
    ) -> CompilerResult<Vec<DataTypeId>> {

        self.compute_var_list_types(&func_def_node.params, func_def_node.body_id)
    }

    fn compute_function_return_type(
        &self,
        func_def_node: &FunctionDefNode
    ) -> CompilerResult<DataTypeId> {
        Ok(match &func_def_node.return_type {
            None => self.type_arena.get_builtin_type_id(Unit),
            Some(ret) => {
                if ret.inner_types.len() > 0 {
                    unimplemented!("Generic type annotations")
                }

                // TODO: generics
                let return_type_symbol = ret.type_name;
                self.type_arena
                    .get_type_id(return_type_symbol, self.ctx)
                    .ok_or_else(|| UndefinedType.at(ret.span))?
            }
        })
    }

    fn compute_function_type(
        &mut self,
        func_def_node: &FunctionDefNode,
    ) -> CompilerResult<FunctionDataTypeId> {
        let param_types = self.compute_function_param_types(func_def_node)?;
        let return_type = self.compute_function_return_type(func_def_node)?;

        let function_type = FunctionDataType::new(param_types, return_type);
        let function_type_id = self.type_arena.get_or_insert_function_type(function_type);
        let data_type_id = self.type_arena.function_to_data_type_id(function_type_id);

        let symbol = self.name_resolution_symbol_table.lookup_expect_exist(
            func_def_node.name, self.curr_block_id
        );
        self.symbol_table.assign_symbol_type(symbol, data_type_id, self.curr_block_id);

        Ok(function_type_id)
    }

    fn compute_class_def_types(&mut self, class_def_node: &ClassDefNode) -> CompilerResult<()> {
        // TODO: Generics
        let type_name = class_def_node.class_type.type_name;
        let mut class_type: DataType<TypeChecking> = UserDefined(type_name).into();

        let field_data_type_ids = self.compute_var_list_types(
            &class_def_node.fields, class_def_node.body_id
        )?;

        let data_type_ids_and_fields = field_data_type_ids
            .iter()
            .zip(class_def_node.fields.iter());

        for (&data_type_id, field) in data_type_ids_and_fields {
            class_type.add_field(field.name, Field::new(data_type_id, ()))
        }

        if self.type_arena.insert_new_type(type_name, class_type).is_none() {
            return Err(DuplicateType(type_name).at(class_def_node.class_type.span));
        }

        Ok(())
    }

    fn compute_item_types(&mut self, items: &Vec<ItemId>) -> CompilerResult<()> {
        for &node_id in items {
            let node = self.ast.lookup_item(node_id);

            match &node.node_type {
                FunctionDef(func_def_node) => {
                    self.compute_function_type(func_def_node)?;
                }
                ClassDef(class_def_node) => {
                    self.compute_class_def_types(class_def_node)?;
                }
            }
        }

        Ok(())
    }

    fn compute_class_item_types(&mut self, items: &Vec<ItemId>, class_type_id: DataTypeId) -> CompilerResult<()> {

        for &node_id in items {
            let node = self.ast.lookup_item(node_id);

            match &node.node_type {
                FunctionDef(func_def_node) => {
                    let func_type = self.compute_function_type(func_def_node)?;
                    let method = Method::new(func_type, ());

                    let class_type = self.type_arena.get_data_type_mut(class_type_id);
                    class_type.add_method(func_def_node.name, method)
                }
                ClassDef(class_def_node) => {
                    self.compute_class_def_types(class_def_node)?;
                }
            }
        }

        Ok(())
    }

    fn compute_block_types(&mut self, block_id: BlockId, class_type_id: Option<DataTypeId>) -> CompilerResult<()> {
        let parent_scope_id = self.curr_block_id;
        self.curr_block_id = block_id;

        let block = self.ast.lookup_block(block_id);

        match class_type_id {
            None => self.compute_item_types(&block.items)?,
            Some(class_type) => self.compute_class_item_types(&block.items, class_type)?,
        }

        self.compute_item_block_types(&block.items)?;
        self.compute_statement_types(&block.statements)?;

        self.curr_block_id = parent_scope_id;

        Ok(())
    }

    pub fn check_types(
        ast: &'ast AST,
        name_resolution_symbol_table: SymbolTable<NameResolution>,
        ctx: &'ast mut CompilerContext
    ) -> CompilerResult<TypeCheckingOutput> {
        let mut type_checker = TypeChecker::new(&ast, name_resolution_symbol_table, ctx);
        type_checker.compute_block_types(ast.global_block_id, None)?;

        Ok(TypeCheckingOutput::new(type_checker.ast_expr_data_types, type_checker.symbol_table, type_checker.type_arena))
    }
}

#[derive(Debug)]
pub struct TypeCheckingOutput {
    pub ast_expr_data_types: Vec<DataTypeId>,
    pub symbol_table: SymbolTable<TypeChecking>,
    pub type_arena: TypeArena<TypeChecking>

}

impl TypeCheckingOutput {
    pub fn new(
        ast_expr_data_types: Vec<DataTypeId>,
        symbol_table: SymbolTable<TypeChecking>,
        type_arena: TypeArena<TypeChecking>
    ) -> Self {
        Self {
            ast_expr_data_types,
            symbol_table,
            type_arena
        }
    }
}
