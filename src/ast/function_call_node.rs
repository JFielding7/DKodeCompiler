use crate::ast::ast_node::ExpressionId;

#[derive(Debug)]
pub struct FunctionCallNode {
    pub function: ExpressionId,
    pub args: Option<ExpressionId>,
}

impl FunctionCallNode {
    pub fn new(function: ExpressionId, args: Option<ExpressionId>) -> Self {
        Self {
            function,
            args,
        }
    }
}
