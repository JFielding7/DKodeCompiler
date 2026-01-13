use string_interner::DefaultSymbol;
use thiserror::Error;
use crate::compiler_context::type_arena::DataTypeId;
use crate::error::spanned_error::SpannedError;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::unary_operators::UnaryOperator;
use crate::types::data_type::DataType;

#[derive(Error, Debug)]
pub enum SemanticError {
    #[error("Error: Operator {0} not defined on {1:?}")]
    MismatchedUnaryOperatorTypes(UnaryOperator, DataTypeId),

    #[error("Error: Operator {0} not defined for {1:?} and {2:?}")]
    MismatchedBinaryOperatorTypes(BinaryOperator, DataTypeId, DataTypeId),

    #[error("Error: Cannot infer type")]
    TypeInference,

    #[error("Error: Undefined variable")]
    UndefinedVariable,
}

pub type SemanticResult<T> = Result<T, SpannedError>;
