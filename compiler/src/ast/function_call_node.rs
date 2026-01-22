use crate::ast::ast_node::ExpressionId;

#[derive(Debug)]
pub struct FunctionCallNode {
    pub function: ExpressionId,
    pub args: Vec<ExpressionId>,
}

impl FunctionCallNode {
    pub fn new(function: ExpressionId, args: Vec<ExpressionId>) -> Self {
        Self {
            function,
            args,
        }
    }
}
