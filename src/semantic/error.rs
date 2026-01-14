use string_interner::DefaultSymbol;
use crate::compiler_context::CompilerContext;
use crate::error::spanned_error::{SpannableError, CompilerError};
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::unary_operators::UnaryOperator;
use crate::types::data_type::DataTypeId;
use SemanticError::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SemanticError {
    #[error("")]
    MismatchedUnaryOperatorTypes(UnaryOperator, DataTypeId),

    #[error("")]
    MismatchedBinaryOperatorTypes(BinaryOperator, DataTypeId, DataTypeId),

    #[error("")]
    DuplicateParameterName(DefaultSymbol),

    #[error("")]
    UndefinedVariable(DefaultSymbol),

    #[error("Error: Undefined type")]
    UndefinedType,

    #[error("Error: Cannot infer type")]
    TypeInference,
}

impl SpannableError for SemanticError {
    fn format(&self, ctx: CompilerContext) -> String {
        match self {
            MismatchedUnaryOperatorTypes(op, id) => {
                format!("Error: Operator {op} not defined on {}",
                        ctx.type_arena.format_type(*id, &ctx.string_interner)
                )
            }
            MismatchedBinaryOperatorTypes(op, left, right) => {
                format!("Error: Operator {op} not defined for {} and {}", ctx.type_arena.format_type(*left, &ctx.string_interner), ctx.type_arena.format_type(*right, &ctx.string_interner))
            }
            UndefinedVariable(var_name) => {
                format!("Error: Undefined variable: {}", ctx.string_interner.get_str(*var_name))
            }
            DuplicateParameterName(param_name) => {
                format!("Error: Duplicate parameter name: {}", ctx.string_interner.get_str(*param_name))
            }
            _ => format!("{self}"),
        }
    }
}
