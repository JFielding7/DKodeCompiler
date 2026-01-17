use crate::ast::access_node::AccessNode;
use crate::ast::binary_operator_node::BinaryOperatorNode;
use crate::ast::for_node::ForNode;
use crate::ast::function_call_node::FunctionCallNode;
use crate::ast::function_def_node::FunctionDefNode;
use crate::ast::if_node::IfNode;
use crate::ast::index_node::IndexNode;
use crate::ast::unary_operator_node::UnaryOperatorNode;
use crate::ast::variable_node::VariableNode;
use crate::ast::while_node::WhileNode;
use crate::source::source_span::SourceSpan;
use string_interner::DefaultSymbol;
use Expression::*;
use Item::*;
use Statement::*;

#[derive(Debug)]
pub struct ASTNode<T> {
    pub node_type: T,
    pub span: SourceSpan,
}

impl<T> ASTNode<T> {
    pub fn new(node_type: T, span: SourceSpan) -> Self {
        Self {
            node_type, 
            span,
        }
    }
}

#[derive(Debug)]
pub enum Item {
    FunctionDef(FunctionDefNode),
    // TODO: Classes
}

#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(ExpressionId),

    ReturnStatement(Option<ExpressionId>),

    If(IfNode),

    While(WhileNode),

    For(ForNode),
}

#[derive(Debug)]
pub enum Expression {
    IntLiteral(DefaultSymbol),

    StringLiteral(DefaultSymbol),

    Variable(VariableNode),

    UnaryOperator(UnaryOperatorNode),

    BinaryOperator(BinaryOperatorNode),

    FunctionCall(FunctionCallNode),

    Index(IndexNode),

    Access(AccessNode),
}

#[derive(Debug, Copy, Clone)]
pub struct ItemId(usize);

impl ItemId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StatementId(usize);

impl StatementId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ExpressionId(usize);

impl ExpressionId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

macro_rules! impl_to_expression {
    ($($node_type:ident => $variant:ident),*) => {
        $(
            impl From<$node_type> for Expression {
                fn from(node: $node_type) -> Self {
                    $variant(node)
                }
            }
        )*
    };
}

impl_to_expression! {
    VariableNode => Variable,
    UnaryOperatorNode => UnaryOperator,
    BinaryOperatorNode => BinaryOperator,
    IndexNode => Index,
    AccessNode => Access,
    FunctionCallNode => FunctionCall
}

macro_rules! impl_to_statement {
    ($($node_type:ident => $variant:ident),*) => {
        $(
            impl From<$node_type> for Statement {
                fn from(node: $node_type) -> Self {
                    $variant(node)
                }
            }
        )*
    };
}

impl_to_statement! {
    IfNode => If,
    WhileNode => While,
    ForNode => For
}

macro_rules! impl_to_item {
    ($($node_type:ident => $variant:ident),*) => {
        $(
            impl From<$node_type> for Item {
                fn from(node: $node_type) -> Self {
                    $variant(node)
                }
            }
        )*
    };
}

impl_to_item! {
    FunctionDefNode => FunctionDef
}
