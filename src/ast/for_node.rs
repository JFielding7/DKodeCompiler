use string_interner::DefaultSymbol;
use crate::ast::ast_node::ExpressionId;
use crate::ast::block::BlockId;
use crate::source::source_span::SourceSpan;

#[derive(Debug)]
pub struct ForNode {
    pub item_variable: ForVariable,
    pub iterator: ExpressionId,
    pub body_id: BlockId,
}

impl ForNode {
    pub fn new(item_variable: ForVariable, iterator: ExpressionId, body: BlockId) -> Self {
        Self {
            item_variable,
            iterator,
            body_id: body
        }
    }
}

#[derive(Debug)]
pub struct ForVariable {
    pub name: DefaultSymbol,
    pub span: SourceSpan,
}

impl ForVariable {
    pub fn new(name: DefaultSymbol, span: SourceSpan) -> Self {
        Self { 
            name, 
            span 
        }
    }
}
