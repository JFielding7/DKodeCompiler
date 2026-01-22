use crate::compiler_context::CompilerContext;
use crate::error::compiler_error::SpannableError;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::unary_operators::UnaryOperator;
use crate::types::data_type::DataTypeId;
use string_interner::DefaultSymbol;
use SemanticError::*;

#[derive(Debug)]
pub enum SemanticError {
    MismatchedUnaryOperatorTypes(UnaryOperator, DataTypeId),

    MismatchedBinaryOperatorTypes(BinaryOperator, DataTypeId, DataTypeId),

    MismatchedTypes {
        expected: DataTypeId,
        actual: DataTypeId,
    },

    DuplicateParameterName(DefaultSymbol),

    DuplicateFunctionName(DefaultSymbol),

    UndefinedVariable(DefaultSymbol),

    FunctionExpected,

    IncorrectArgumentCount {
        expected: usize,
        actual: usize,
    },

    ReturnStatementOutsideFunction,

    IncorrectReturnType {
        expected: DataTypeId,
        actual: DataTypeId,
    },
    
    InvalidLValue,

    UndefinedType,
    
    DuplicateType(DefaultSymbol),

    TypeInference,
}

impl SpannableError for SemanticError {
    fn format(&self, ctx: &CompilerContext) -> String {
        let string_interner = &ctx.string_interner;
        let type_arena = &ctx.type_arena;

        match self {
            MismatchedUnaryOperatorTypes(op, id) => {
                format!("Error: Operator {op} not defined on {}",
                        type_arena.format_type(*id, string_interner)
                )
            }
            MismatchedBinaryOperatorTypes(op, left, right) => {
                format!("Error: Operator {op} not defined for {} and {}", 
                        type_arena.format_type(*left, string_interner),
                        type_arena.format_type(*right, string_interner)
                )
            }
            MismatchedTypes { expected, actual } => {
                format!("Error: Mismatched types: Expected {}, but got {}",
                        type_arena.format_type(*expected, string_interner),
                        type_arena.format_type(*actual, string_interner)
                )
            }
            UndefinedVariable(var_name) => {
                format!("Error: Undefined variable: {}",
                        string_interner.get_str(*var_name)
                )
            }
            DuplicateParameterName(param_name) => {
                format!("Error: Duplicate parameter definition: {}", 
                        string_interner.get_str(*param_name)
                )
            }
            DuplicateFunctionName(func_name) => {
                format!("Error: Duplicate function definition: {}", 
                        string_interner.get_str(*func_name)
                )
            }
            FunctionExpected => {
                "Error: Function expected".to_string()
            }
            IncorrectArgumentCount { expected, actual } => {
                format!("Error: Expected {expected} arguments, but got {actual}")
            }
            ReturnStatementOutsideFunction => {
                "Error: Return statement outside function".to_string()
            }
            IncorrectReturnType { expected, actual } => {
                format!("Error: Incorrect Return type: Expected {}, but got {}",
                        type_arena.format_type(*expected, string_interner),
                        type_arena.format_type(*actual, string_interner)
                )
            }
            InvalidLValue => {
                "Error: Value is not assignable".to_string()
            }
            UndefinedType => {
                "Error: Undefined type".to_string()
            }
            DuplicateType(type_name) => {
                format!("Error: Duplicate function definition: {}", 
                        string_interner.get_str(*type_name)
                )
            }
            TypeInference => {
                "Error: Cannot infer type".to_string()
            },
        }
    }
}
