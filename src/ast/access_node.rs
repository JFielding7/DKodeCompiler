use string_interner::DefaultSymbol;
use crate::ast::access_node::Member::Field;
use crate::ast::ast_node::ASTNodeId;

#[derive(Debug)]
pub struct AccessNode {
    pub receiver: ASTNodeId,
    pub member: Member,
}

impl AccessNode {
    pub fn new(receiver: ASTNodeId, member: Member) -> Self {
        Self {
            receiver,
            member,
        }
    }
}

#[derive(Debug)]
pub enum Member {
    Field {
        name: DefaultSymbol,
    },
    Method {
        name: DefaultSymbol,
        args: Option<ASTNodeId>,
    },
}

impl Member {
    pub fn field(name: DefaultSymbol) -> Self {
        Field {
            name,
        }
    }
    
    pub fn method_no_args(name: DefaultSymbol) -> Self {
        Self::Method {
            name,
            args: None,
        }
    }

    pub fn method_with_args(name: DefaultSymbol, args: ASTNodeId) -> Self {
        Self::Method {
            name,
            args: Some(args),
        }
    }
}
