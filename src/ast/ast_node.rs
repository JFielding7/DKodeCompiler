use crate::ast::access_node::AccessNode;
use crate::ast::ast_node::ASTNodeType::*;
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
use crate::compiler_context::scope::ScopeId;
use crate::compiler_context::type_arena::DataTypeId;

#[derive(Debug)]
pub struct ASTNode {
    pub node_type: ASTNodeType,
    pub span: SourceSpan,
    pub scope_id: ScopeId,
    pub data_type_id: Option<DataTypeId>,
}

impl ASTNode {
    pub fn new(node_data_type: ASTNodeType, span: SourceSpan, scope: ScopeId) -> Self {
        Self {
            node_type: node_data_type, 
            span,
            scope_id: scope,
            data_type_id: None,
        }
    }
}

#[derive(Debug)]
pub enum ASTNodeType {
    IntLiteral(DefaultSymbol),
    StringLiteral(DefaultSymbol),

    Variable(VariableNode),

    UnaryOperator(UnaryOperatorNode),

    BinaryOperator(BinaryOperatorNode),

    FunctionDef(FunctionDefNode),

    FunctionCall(FunctionCallNode),

    Index(IndexNode),

    Access(AccessNode),
    
    If(IfNode),
    
    While(WhileNode),

    For(ForNode),
}

pub trait ASTNodeLocation {
    fn at(self, span: SourceSpan, scope: ScopeId) -> ASTNode
    where Self: Sized, ASTNodeType: From<Self> {
        ASTNode::new(self.into(), span, scope)
    }
}

macro_rules! impl_to_ast_node_type {
    ($($node_type:ident => $variant:ident),*) => {
        $(
            impl From<$node_type> for ASTNodeType {
                fn from(node: $node_type) -> Self {
                    $variant(node)
                }
            }

            impl ASTNodeLocation for $node_type {}
        )*
    };
}

impl_to_ast_node_type! {
    VariableNode => Variable,
    UnaryOperatorNode => UnaryOperator,
    BinaryOperatorNode => BinaryOperator,
    IndexNode => Index,
    AccessNode => Access,
    FunctionCallNode => FunctionCall,
    FunctionDefNode => FunctionDef,
    IfNode => If,
    WhileNode => While,
    ForNode => For
}
