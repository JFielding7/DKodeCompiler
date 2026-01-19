use string_interner::DefaultSymbol;
use crate::ast::access_node::MemberType::{Field, Method};
use crate::ast::ast_node::ExpressionId;
use crate::source::source_span::SourceSpan;

#[derive(Debug)]
pub struct AccessNode {
    pub receiver: ExpressionId,
    pub member: Member,
}

impl AccessNode {
    pub fn new(receiver: ExpressionId, member: Member) -> Self {
        Self {
            receiver,
            member,
        }
    }
}

#[derive(Debug)]
pub struct Member {
    name: DefaultSymbol,
    member_type: MemberType,
    pub span: SourceSpan,
}

#[derive(Debug)]
pub enum MemberType {
    Field,
    Method(Vec<ExpressionId>),
}

impl Member {
    pub fn field(name: DefaultSymbol, span: SourceSpan) -> Self {
        Self {
            name,
            member_type: Field,
            span
        }
    }
    
    pub fn method(name: DefaultSymbol, args: Vec<ExpressionId>, span: SourceSpan) -> Self {
        Self {
            name,
            member_type: Method(args),
            span
        }
    }
}
