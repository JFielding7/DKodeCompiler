use crate::ast::ast_node::{ASTNodeType, ASTNodeLocation, ASTNodeId};
use crate::ast::for_node::{ForNode, ForVariable};
use crate::ast::function_def_node::FunctionDefNode;
use crate::ast::if_node::{ConditionBlock, IfNode};
use crate::ast::while_node::WhileNode;
use crate::error::spanned_error::SpannableError;
use crate::lexer::token::TokenType::*;
use crate::lexer::token::TokenType;
use crate::lexer::tokenizer::TokenizedLines;
use crate::syntax::error::SyntaxError::IndentTooLarge;
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::expression::ExpressionParser;
use crate::syntax::parser::function_signature::{parse_function_name, parse_parameters, parse_return_type};
use crate::syntax::parser::source_statements::SourceStatements;
use crate::syntax::parser::statement::Statement;
use std::iter::Peekable;
use std::vec::IntoIter;
use crate::ast::arena_ast::AST;
use crate::ast::ast_node::ASTNodeType::{FunctionDef, Variable};
use crate::ast::block_body::Block;
use crate::ast::variable_node::VariableNode;
use crate::compiler_context::CompilerContext;
use crate::compiler_context::scope::ScopeId;
use crate::compiler_context::symbol::Symbol;

pub struct ASTParser<'a> {
    ast: AST,
    statements_iter: Peekable<IntoIter<Statement>>,
    ctx: &'a mut CompilerContext,
}

impl<'a> ASTParser<'a> {
    pub fn new(statements: SourceStatements, ctx: &'a mut CompilerContext) -> Self {
        Self {
            statements_iter: statements.into_iter(),
            ast: AST::new(),
            ctx
        }
    }

    fn next_starts_with(&mut self, token_type: TokenType) -> bool {
        self.statements_iter
            .peek()
            .is_some_and(|statement| statement.token_after_indent_matches(token_type))
    }

    fn parse_children(&mut self, statement: &Statement, scope_id: ScopeId) -> SyntaxResult<Vec<ASTNodeId>> {
        
        let indent_size = statement.indent_size;
        let mut children = Vec::new();

        while let Some(child) =self.statements_iter.peek() {
            if child.indent_size <= indent_size {
                break;
            }

            if indent_size + 1 < child.indent_size {
                return Err(IndentTooLarge.at(child.indent_token().span))
            }

            if let Some(child) = self.parse_next_statement_ast_node(scope_id)? {
                children.push(child);
            }
        }
        
        Ok(children)
    }

    fn parse_function_def(&mut self, func_def_statement: Statement, scope_id: ScopeId) -> SyntaxResult<ASTNodeId> {
        const TOKENS_BEFORE_NAME: usize = 2;
        let mut token_stream = func_def_statement.suffix_stream(TOKENS_BEFORE_NAME);

        let name = parse_function_name(&mut token_stream)?;
        
        let body_scope_id = self.ctx.symbol_table.add_scope_with_parent(scope_id);
        let params = parse_parameters(&mut token_stream)?;
        
        let return_type = parse_return_type(&mut token_stream)?;
        
        let body_nodes = self.parse_children(&func_def_statement, body_scope_id)?;
        let body = Block::new(body_nodes, body_scope_id);
        
        let span = func_def_statement.full_span();

        let func_def_node = FunctionDefNode::new(
            name, params, body, return_type
        ).at(span, scope_id);

        Ok(self.ast.add_node(func_def_node))
    }

    fn parse_if_statement(&mut self, if_statement: Statement, scope_id: ScopeId) -> SyntaxResult<ASTNodeId> {
        const TOKENS_BEFORE_COND: usize = 2;

        let if_cond = ExpressionParser::parse(
            if_statement.suffix_stream(TOKENS_BEFORE_COND),
            &mut self.ast,
            scope_id,
        )?;
        
        let if_body_scope_id = self.ctx.symbol_table.add_scope_with_parent(scope_id);
        let if_body_nodes = self.parse_children(&if_statement, if_body_scope_id)?;
        let if_body = Block::new(if_body_nodes, if_body_scope_id);

        let mut condition_blocks = vec![ConditionBlock::new(if_cond, if_body)];

        while self.next_starts_with(Elif) {
            let elif_statement = self.statements_iter
                .next()
                .expect("Statement Expected");

            let elif_cond = ExpressionParser::parse(
                elif_statement.suffix_stream(TOKENS_BEFORE_COND),
                &mut self.ast,
                scope_id,
            )?;
            
            let elif_body_scope_id = self.ctx.symbol_table.add_scope_with_parent(scope_id);
            let elif_body_nodes = self.parse_children(&elif_statement, elif_body_scope_id)?;
            let elif_body = Block::new(elif_body_nodes, elif_body_scope_id);

            condition_blocks.push(ConditionBlock::new(elif_cond, elif_body));
        }

        let else_body = if self.next_starts_with(Else) {
            let else_statement = self.statements_iter
                .next()
                .expect("Statement Expected");

            let else_body_scope_id = self.ctx.symbol_table.add_scope_with_parent(scope_id);
            let else_body_nodes = self.parse_children(&else_statement, else_body_scope_id)?;
            Some(Block::new(else_body_nodes, else_body_scope_id))
        } else {
            None
        };

        let if_node = IfNode::new(condition_blocks, else_body)
            .at(if_statement.full_span(), scope_id);

        Ok(self.ast.add_node(if_node))
    }

    fn parse_while_loop(&mut self, while_statement: Statement, scope_id: ScopeId) -> SyntaxResult<ASTNodeId> {
        const TOKENS_BEFORE_COND: usize = 2;
        
        let while_cond = ExpressionParser::parse(
            while_statement.suffix_stream(TOKENS_BEFORE_COND),
            &mut self.ast,
            scope_id
        )?;

        let while_body_scope_id = self.ctx.symbol_table.add_scope_with_parent(scope_id);
        let while_body_nodes = self.parse_children(&while_statement, while_body_scope_id)?;
        let while_body = Block::new(while_body_nodes, while_body_scope_id);

        let while_node = WhileNode::new(while_cond, while_body)
            .at(while_statement.full_span(), scope_id);

        Ok(self.ast.add_node(while_node))
    }

    fn parse_for_loop(&mut self, for_statement: Statement, scope_id: ScopeId) -> SyntaxResult<ASTNodeId> {
        const TOKENS_BEFORE_ITEM_IDENT: usize = 2;
        let mut token_stream = for_statement.suffix_stream(TOKENS_BEFORE_ITEM_IDENT);

        let for_body_scope_id = self.ctx.symbol_table.add_scope_with_parent(scope_id);

        let item_identifier = token_stream.expect_next_identifier()?;
        let item_var = ForVariable::new(item_identifier.symbol, item_identifier.span);

        token_stream.expect_next_token(In)?;

        let iterator = ExpressionParser::parse(
            token_stream,
            &mut self.ast,
            scope_id
        )?;
        
        let for_body_nodes = self.parse_children(&for_statement, for_body_scope_id)?;
        let for_body = Block::new(for_body_nodes, for_body_scope_id);

        let node = ForNode::new(item_var, iterator, for_body)
            .at(for_statement.full_span(), scope_id);

        Ok(self.ast.add_node(node))
    }

    fn parse_next_statement_ast_node(&mut self, scope_id: ScopeId) -> SyntaxResult<Option<ASTNodeId>> {

        if let Some(statement) = self.statements_iter.next() {

            let node_id = match statement.token_after_indent_type() {
                Fn => self.parse_function_def(statement, scope_id)?,
                If => self.parse_if_statement(statement, scope_id)?,
                While => self.parse_while_loop(statement, scope_id)?,
                For => self.parse_for_loop(statement, scope_id)?,
                _ => ExpressionParser::parse(
                    statement.suffix_stream(Statement::INDEX_AFTER_INDENT),
                    &mut self.ast,
                    scope_id
                )?,
            };
            
            self.ast.add_statement_root(node_id);

            Ok(Some(node_id))
        } else {
            Ok(None)
        }
    }
    
    fn parse_global_nodes(&mut self) -> SyntaxResult<()> {

        let global_scope_id = ScopeId::global();
        while let Some(_node_id) = self.parse_next_statement_ast_node(global_scope_id)? {}

        Ok(())
    }

    pub fn generate_ast(source_lines: TokenizedLines, ctx: &'a mut CompilerContext) -> SyntaxResult<AST> {

        let statements: SourceStatements = source_lines.into();
        let mut parser = Self::new(statements, ctx);
        parser.parse_global_nodes()?;
        
        Ok(parser.ast)
    }
}

