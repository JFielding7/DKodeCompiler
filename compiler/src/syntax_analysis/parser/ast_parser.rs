use crate::ast::ast_node::Statement::{ExpressionStatement, ReturnStatement};
use crate::ast::ast_node::{ItemId, StatementId};
use crate::ast::block::BlockId;
use crate::ast::class_def_node::ClassDefNode;
use crate::ast::for_node::{ForNode, ForVariable};
use crate::ast::function_def_node::FunctionDefNode;
use crate::ast::if_node::{ConditionBlock, IfNode};
use crate::ast::while_node::WhileNode;
use crate::ast::AST;
use crate::compiler_context::symbol_table::scope::Scope;
use crate::compiler_context::CompilerContext;
use crate::error::compiler_error::CompilerResult;
use crate::error::compiler_error::SpannableError;
use crate::lexical_analysis::token::TokenType;
use crate::lexical_analysis::token::TokenType::*;
use crate::syntax_analysis::error::SyntaxError::IndentTooLarge;
use crate::syntax_analysis::parser::class_def::{parse_class_name, parse_field};
use crate::syntax_analysis::parser::expression::ExpressionParser;
use crate::syntax_analysis::parser::function_signature::{parse_function_name, parse_parameters, parse_return_type};
use crate::syntax_analysis::parser::source_statements::{SourceStatements, SourceStatementsIter};
use crate::syntax_analysis::parser::statement::Statement;
use crate::syntax_analysis::parser::token_stream::TokenStream;
use string_interner::DefaultSymbol;

pub struct ASTParser<'llvm_ctx> {
    pub ast: AST,
    statements_iter: SourceStatementsIter,
    curr_block_id: Option<BlockId>,
    curr_function_name: Option<DefaultSymbol>,
    ctx: &'llvm_ctx mut CompilerContext,
}

impl<'llvm_ctx> ASTParser<'llvm_ctx> {
    pub fn new(statements: SourceStatements, ctx: &'llvm_ctx mut CompilerContext) -> Self {
        Self {
            statements_iter: statements.into_iter(),
            ast: AST::new(),
            curr_block_id: None,
            curr_function_name: None,
            ctx,
        }
    }

    fn next_child_statement(&mut self, parent_indent_size: isize) -> CompilerResult<Option<Statement>> {
        if let Some(child) = self.statements_iter.peek() {
            let child_indent_size = child.indent_size;

            if child_indent_size <= parent_indent_size {
                Ok(None)
            } else if child_indent_size > parent_indent_size + 1 {
                Err(IndentTooLarge.at(child.indent_token().span))
            } else {
                Ok(Some(self.statements_iter.next().unwrap()))
            }
        } else {
            Ok(None)
        }
    }

    fn next_starts_with(&mut self, token_type: TokenType) -> bool {
        self.statements_iter
            .peek()
            .is_some_and(|statement| statement.token_after_indent_matches(token_type))
    }

    fn parse_function_def(&mut self, func_def_statement: Statement) -> CompilerResult<ItemId> {
        const TOKENS_BEFORE_NAME: usize = 2;

        let mut token_stream = TokenStream::new(&func_def_statement, TOKENS_BEFORE_NAME);

        let function_name = parse_function_name(&mut token_stream)?;
        
        let parent_function_name = self.curr_function_name;
        self.curr_function_name = Some(function_name);
        
        let params = parse_parameters(&mut token_stream)?;
        let return_type = parse_return_type(&mut token_stream)?;

        let body = self.parse_block(func_def_statement.indent_size)?;
        
        let span = func_def_statement.full_span();

        let func_def_node = FunctionDefNode::new(
            function_name, params, body, return_type
        ).into();
        
        self.curr_function_name = parent_function_name;

        Ok(self.ast.add_item(func_def_node, span))
    }

    fn parse_class_def(&mut self, class_def_statement: Statement) -> CompilerResult<ItemId> {
        const TOKENS_BEFORE_NAME: usize = 2;

        let token_stream = TokenStream::new(&class_def_statement, TOKENS_BEFORE_NAME);
        let class_type = parse_class_name(token_stream)?;

        let indent_size = class_def_statement.indent_size;

        let mut fields = Vec::new();

        let block_id = self.ast.create_block();
        let block_scope = Scope::new(self.curr_block_id, self.curr_function_name);

        self.ctx.symbol_table.add_scope(block_scope);

        let parent_block_id = self.curr_block_id;
        self.curr_block_id = Some(block_id);

        while let Some(child) = self.next_child_statement(indent_size)? {
            match child.token_after_indent_type() {
                Fn => {
                    let func_def = self.parse_function_def(child)?;
                    self.ast.lookup_block_mut(block_id).add_item(func_def)
                },
                Class => {
                    let class_def = self.parse_class_def(child)?;
                    self.ast.lookup_block_mut(block_id).add_item(class_def)
                },
                _ => {
                    let token_stream = TokenStream::new(&child, Statement::INDEX_AFTER_INDENT);
                    fields.push(parse_field(token_stream)?)
                }
            }
        }

        self.curr_block_id = parent_block_id;

        let class_node = ClassDefNode::new(class_type, fields, block_id).into();

        Ok(self.ast.add_item(class_node, class_def_statement.full_span()))
    }

    fn parse_if_statement(&mut self, if_statement: Statement) -> CompilerResult<StatementId> {
        const TOKENS_BEFORE_COND: usize = 2;

        let if_cond = ExpressionParser::parse(
            TokenStream::new(&if_statement, TOKENS_BEFORE_COND),
            &mut self.ast,
        )?;
        
        let if_body = self.parse_block(if_statement.indent_size)?;

        let mut condition_blocks = vec![ConditionBlock::new(if_cond, if_body)];

        while self.next_starts_with(Elif) {
            let elif_statement = self.statements_iter
                .next()
                .expect("Statement Expected");

            let elif_cond = ExpressionParser::parse(
                TokenStream::new(&elif_statement, TOKENS_BEFORE_COND),
                &mut self.ast,
            )?;
            
            let elif_body = self.parse_block(elif_statement.indent_size)?;

            condition_blocks.push(ConditionBlock::new(elif_cond, elif_body));
        }

        let else_body = if self.next_starts_with(Else) {
            let else_statement = self.statements_iter
                .next()
                .expect("Statement Expected");

            Some(self.parse_block(else_statement.indent_size)?)
        } else {
            None
        };

        let if_node = IfNode::new(condition_blocks, else_body).into();

        Ok(self.ast.add_statement(if_node, if_statement.full_span()))
    }

    fn parse_while_loop(&mut self, while_statement: Statement) -> CompilerResult<StatementId> {
        const TOKENS_BEFORE_COND: usize = 2;
        
        let while_cond = ExpressionParser::parse(
            TokenStream::new(&while_statement, TOKENS_BEFORE_COND),
            &mut self.ast,
        )?;

        let while_body = self.parse_block(while_statement.indent_size)?;

        let while_node = WhileNode::new(while_cond, while_body).into();

        Ok(self.ast.add_statement(while_node, while_statement.full_span()))
    }

    fn parse_for_loop(&mut self, for_statement: Statement) -> CompilerResult<StatementId> {
        const TOKENS_BEFORE_ITEM_IDENT: usize = 2;
        let mut token_stream = TokenStream::new(&for_statement, TOKENS_BEFORE_ITEM_IDENT);

        let item_identifier = token_stream.expect_next_identifier()?;
        let item_var = ForVariable::new(item_identifier.symbol, item_identifier.span);

        token_stream.expect_next_token(In)?;

        let iterator = ExpressionParser::parse(
            token_stream,
            &mut self.ast,
        )?;
        
        let for_body = self.parse_block(for_statement.indent_size)?;

        let for_node = ForNode::new(item_var, iterator, for_body).into();

        Ok(self.ast.add_statement(for_node, for_statement.full_span()))
    }
    
    fn parse_return_statement(&mut self, return_statement: Statement) -> CompilerResult<StatementId> {
        const TOKENS_BEFORE_EXPRESSION: usize = 2;
        
        let ret_statement = if return_statement.len() > 2 {
            let ret_value = ExpressionParser::parse(
                TokenStream::new(&return_statement, TOKENS_BEFORE_EXPRESSION),
                &mut self.ast,
            )?;

            ReturnStatement(Some(ret_value))
        } else {
            ReturnStatement(None)
        };

        Ok(self.ast.add_statement(ret_statement, return_statement.full_span()))
    }
    
    fn parse_expression(&mut self, expr_statement: Statement) -> CompilerResult<StatementId> {
        let statement = ExpressionStatement(ExpressionParser::parse(
            TokenStream::new(&expr_statement, Statement::INDEX_AFTER_INDENT),
            &mut self.ast,
        )?);

        Ok(self.ast.add_statement(statement, expr_statement.full_span()))
    }

    fn parse_next_statement_ast_node(
        &mut self,
        statement: Statement,
    ) -> CompilerResult<BlockChildNodeId> {
        use BlockChildNodeId::*;

        Ok(match statement.token_after_indent_type() {
            Fn => Item(self.parse_function_def(statement)?),
            Class => Item(self.parse_class_def(statement)?),
            If => Statement(self.parse_if_statement(statement)?),
            While => Statement(self.parse_while_loop(statement)?),
            For => Statement(self.parse_for_loop(statement)?),
            Return => Statement(self.parse_return_statement(statement)?),
            _ => Statement(self.parse_expression(statement)?),
        })
    }

    fn parse_block(&mut self, indent_size: isize) -> CompilerResult<BlockId> {
        use BlockChildNodeId::*;

        let block_id = self.ast.create_block();
        let block_scope = Scope::new(self.curr_block_id, self.curr_function_name);
        
        self.ctx.symbol_table.add_scope(block_scope);
        
        let parent_block_id = self.curr_block_id;
        self.curr_block_id = Some(block_id);

        while let Some(child) =self.next_child_statement(indent_size)? {

            match self.parse_next_statement_ast_node(child)? {
                Item(item_id) => {
                    self.ast.lookup_block_mut(block_id).add_item(item_id)
                },
                Statement(statement_id) => {
                    self.ast.lookup_block_mut(block_id).add_statement(statement_id)
                },
            }
        }
        
        self.curr_block_id = parent_block_id;

        Ok(block_id)
    }
    
    pub fn parse_global_nodes(&mut self) -> CompilerResult<()> {
        const GLOBAL_PARENT_INDENT_SIZE: isize = -1;

        self.ast.global_block_id = self.parse_block(GLOBAL_PARENT_INDENT_SIZE)?;

        Ok(())
    }
}

enum BlockChildNodeId {
    Item(ItemId),
    Statement(StatementId),
}
