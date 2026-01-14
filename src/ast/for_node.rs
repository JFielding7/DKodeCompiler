use string_interner::DefaultSymbol;
use crate::ast::arena_ast::ASTNodeId;
use crate::ast::block_body::Block;
use crate::source::source_span::SourceSpan;

#[derive(Debug)]
pub struct ForNode {
    pub item_variable: ForVariable,
    pub iterator: ASTNodeId,
    pub body: Block,
}

impl ForNode {
    pub fn new(item_variable: ForVariable, iterator: ASTNodeId, body: Block) -> Self {
        Self {
            item_variable,
            iterator,
            body
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

pub struct ForBody {
    
}
