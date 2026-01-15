use crate::ast::access_node::{AccessNode, Member};
use crate::ast::AST;
use crate::ast::ast_node::{ASTNode, ASTNodeType, ASTNodeLocation, ASTNodeId};
use crate::ast::binary_operator_node::{BinaryOperatorNode};
use crate::ast::function_call_node::FunctionCallNode;
use crate::ast::index_node::IndexNode;
use crate::ast::unary_operator_node::{UnaryOperatorNode};
use crate::ast::variable_node::VariableNode;
use crate::compiler_context::scope::ScopeId;
use crate::error::compiler_error::CompilerResult;
use crate::error::compiler_error::SpannableError;
use crate::lexer::token::TokenType::*;
use crate::lexer::token::{Token, TokenType};
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::unary_operators::UnaryOperator;
use crate::syntax::error::SyntaxError::{InvalidExpression, UnmatchedGroupOpening};
use crate::operators::precedence::OperatorPrecedenceGroup;
use crate::operators::precedence::OperatorPrecedenceGroup::Prefix;
use crate::syntax::parser::token_stream::TokenStream;
use crate::syntax::parser::type_annotation::parse_type_annotation;

fn operators_with_lhs_binding_power(op: &Token) -> Option<(u8, u8)> {
    use OperatorPrecedenceGroup::*;

    Some(match op.token_type {
        TokenType::Comma => Comma,

        Equals
        | PlusEquals
        | MinusEquals
        | StarEquals
        | SlashEquals
        | PercentEquals
        | DoubleLeftArrowEquals
        | DoubleRightArrowEquals
        | AmpersandEquals
        | CaretEquals
        | PipeEquals
        => Assign,

        DoublePipe => LogicalOr,

        DoubleAmpersand => LogicalAnd,

        Pipe => BitOr,

        Caret => BitXor,

        Ampersand => BitAnd,

        DoubleEquals
        | ExclamationEquals
        => Equality,

        Less
        | LessEquals
        | Greater
        | GreaterEquals
        => Relational,

        DoubleLeftArrow
        | DoubleRightArrow
        => BitShift,

        Plus
        | Minus
        => Add,

        Star
        | Slash
        | Percent
        => Mul,

        PlusPlus
        | MinusMinus
        | OpenParen
        | OpenBracket
        | Dot
        => Postfix,

        _ => return None,
    }.binding_power())
}


fn binary_operator_type(op: &Token) -> Option<BinaryOperator> {
    use BinaryOperator::*;

    Some(match op.token_type {
        Equals => Assign,
        PlusEquals => AddAssign,
        MinusEquals => SubAssign,
        StarEquals => MulAssign,
        SlashEquals => DivAssign,
        PercentEquals => ModAssign,
        DoubleLeftArrowEquals => LeftShiftAssign,
        DoubleRightArrowEquals => RightShiftAssign,
        AmpersandEquals => AndAssign,
        CaretEquals => XorAssign,
        PipeEquals => OrAssign,

        Plus => Add,
        Minus => Sub,
        Star => Mul,
        Slash => Div,
        Percent => Mod,

        Ampersand => BitAnd,
        Pipe => BitOr,
        Caret => BitXor,

        DoubleLeftArrow => LeftShift,
        DoubleRightArrow => RightShift,

        DoubleEquals => Equal,
        ExclamationEquals => NotEquals,
        Less => LessThan,
        LessEquals => LessOrEqual,
        Greater => GreaterThan,
        GreaterEquals => GreaterOrEqual,

        DoubleAmpersand => LogicalAnd,
        DoublePipe => LogicalOr,

        Comma => CommaOperator,

        _ => return None,
    })
}

fn prefix_unary_operator_type(op: &Token) -> Option<UnaryOperator> {
    use UnaryOperator::*;

   Some(match op.token_type {
        Minus => Neg,
        Exclamation => Not,
        Tilde => BitNot,
        PlusPlus => PreInc,
        MinusMinus => PreDec,
        _ => return None,
    })
}

fn postfix_unary_operator_type(op: &Token) -> Option<UnaryOperator> {
    use UnaryOperator::*;

    Some(match op.token_type {
        PlusPlus => PostInc,
        MinusMinus => PostDec,
        _ => return None,
    })
}

fn is_terminal(token: &Token) -> bool {
    matches!(token.token_type, CloseParen | CloseBracket | Colon)
}

fn close_token(open_token: &Token) -> TokenType {
    use TokenType::*;
    
    match open_token.token_type {
        OpenParen => CloseParen,
        OpenBracket => CloseBracket,
        _ => unreachable!("Invalid group opening token"),
    }
}

pub struct ExpressionParser<'a> {
    token_stream: TokenStream<'a>,
    ast: &'a mut AST,
    scope: ScopeId,
}

impl<'a> ExpressionParser<'a> {
    pub fn new(token_stream: TokenStream<'a>, ast: &'a mut AST, scope: ScopeId) -> Self {
        Self {
            token_stream,
            ast,
            scope
        }
    }

    fn parse_token(&mut self, token: &Token) -> CompilerResult<ASTNodeId> {
        use ASTNodeType::*;

        let token_symbol = token.symbol;
        let token_span = token.span;

        let node = match token.token_type {
            TokenType::IntLiteral    => ASTNode::new(IntLiteral(token_symbol), token_span, self.scope),
            TokenType::StringLiteral => ASTNode::new(StringLiteral(token_symbol), token_span, self.scope),
            _ => return Err(InvalidExpression.at(token_span))
        };

        Ok(self.ast.add_node(node))
    }

    fn assert_group_closed(&mut self, open_token: &Token) -> CompilerResult<()> {
        if self.token_stream.peek_matches(close_token(open_token)) {
            self.token_stream.next();
            Ok(())
        } else {
            Err(UnmatchedGroupOpening(open_token.token_type).at(open_token.span))
        }
    }

    fn parse_required_grouped_expression(&mut self, open_token: &Token) -> CompilerResult<ASTNodeId> {
        if self.token_stream.empty() {
            return Err(UnmatchedGroupOpening(open_token.token_type).at(open_token.span));
        }

        let group = self.parse_expression_rec(0)?;

        self.assert_group_closed(open_token)?;
        Ok(group)
    }

    fn parse_optional_grouped_expression(&mut self, open_token: &Token) -> CompilerResult<Option<ASTNodeId>> {
        let group = match self.token_stream.peek() {
            Some(&token) => {
                if *token == CloseParen {
                    None
                } else {
                    Some(self.parse_expression_rec(0)?)
                }
            }
            None => return Err(UnmatchedGroupOpening(open_token.token_type).at(open_token.span))
        };

        self.assert_group_closed(open_token)?;
        Ok(group)
    }

    fn parse_accessed_member(&mut self) -> CompilerResult<Member> {
        let member_name = self.token_stream.expect_next_token(Identifier)?;
        let member_name_symbol = member_name.symbol;

        if self.token_stream.peek_matches(OpenParen) {
            self.token_stream.next();

            let member = if self.token_stream.peek_matches(CloseParen) {
                Ok(Member::method_no_args(member_name_symbol))
            } else {
                let args = self.parse_expression_rec(0)?;
                Ok(Member::method_with_args(member_name_symbol, args))
            };
            self.token_stream.next();
            member

        } else {
            Ok(Member::field(member_name_symbol))
        }
    }

    fn parse_variable(&mut self, token: &Token) -> CompilerResult<ASTNodeId> {
        let type_annotation = if self.token_stream.peek_matches(Colon) {
            self.token_stream.next();
            Some(parse_type_annotation(&mut self.token_stream)?)
        } else {
            None
        };

        let var_node = VariableNode::new(token.symbol, type_annotation).at(token.span, self.scope);
        Ok(self.ast.add_node(var_node))
    }

    fn nud_hook(&mut self) -> CompilerResult<ASTNodeId> {

        match self.token_stream.next() {
            None => Err(InvalidExpression.at(self.token_stream.prev_span())),

            Some(token) => {
                if let Some(unary_op_type) = prefix_unary_operator_type(token) {
                    let unary_node = UnaryOperatorNode::new(
                        unary_op_type,
                        self.parse_expression_rec(Prefix.as_u8())?
                    ).at(token.span, self.scope);
                    Ok(self.ast.add_node(unary_node))

                } else if *token == Identifier {
                    self.parse_variable(token)

                } else if *token == OpenParen {
                    self.parse_required_grouped_expression(token)

                } else {
                    self.parse_token(token)
                }
            }
        }
    }

    fn led_hook(&mut self, token: &Token, left_node: ASTNodeId, right_precedence: u8) -> CompilerResult<ASTNodeId> {
        let token_span = token.span;

        let node = if let Some(op_type) = binary_operator_type(token) {
            let right_node = self.parse_expression_rec(right_precedence)?;
            BinaryOperatorNode::new(op_type, left_node, right_node).at(token_span, self.scope)

        } else if let Some(op_type) = postfix_unary_operator_type(token) {
            UnaryOperatorNode::new(op_type, left_node).at(token_span, self.scope)

        } else if *token == OpenBracket {
            let args = self.parse_required_grouped_expression(token)?;
            IndexNode::new(left_node, args).at(token_span, self.scope)

        } else if *token == OpenParen {
            let args = self.parse_optional_grouped_expression(token)?;
            FunctionCallNode::new(left_node, args).at(token_span, self.scope)

        } else if *token == Dot {
            let member = self.parse_accessed_member()?;
            AccessNode::new(left_node, member).at(token_span, self.scope)

        } else {
            unreachable!("Led hook not implemented");
        };

        Ok(self.ast.add_node(node))
    }

    fn parse_expression_rec(&mut self, curr_precedence: u8) -> CompilerResult<ASTNodeId> {

        if self.token_stream.empty() {
            return Err(InvalidExpression.at(self.token_stream.end_span()))
        }

        let mut left_node_id = self.nud_hook()?;

        while let Some(&token) = self.token_stream.peek() {

            if is_terminal(token) {
                return Ok(left_node_id);
            }

            if let Some((left_precedence, right_precedence)) = operators_with_lhs_binding_power(token) {
                if left_precedence < curr_precedence {
                    return Ok(left_node_id)
                }

                self.token_stream.next();
                left_node_id = self.led_hook(token, left_node_id, right_precedence)?;

            } else {
                return Err(InvalidExpression.at(token.span));
            }
        }

        Ok(left_node_id)
    }

    pub fn parse(token_stream: TokenStream<'a>, ast_arena: &'a mut AST, scope: ScopeId) -> CompilerResult<ASTNodeId> {
        ExpressionParser::new(token_stream, ast_arena, scope).parse_expression_rec(0)
    }
}
