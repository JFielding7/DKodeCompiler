use crate::error::compiler_error::SpannableError;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::unary_operators::UnaryOperator;
use string_interner::DefaultSymbol;
use crate::compiler_context::CompilerContext;

#[derive(Debug)]
pub enum SemanticError {
    MismatchedUnaryOperatorTypes {
        operator_type: UnaryOperator,
        operand_type: String
    },

    MismatchedBinaryOperatorTypes {
        op: BinaryOperator,
        lhs_data_type: String,
        rhs_data_type: String,
    },

    InvalidAssignment {
        lhs_data_type: String,
        rhs_data_type: String,
    },

    MismatchedTypes {
        expected: String,
        actual: String,
    },

    DuplicateParameterName(DefaultSymbol),

    DuplicateFunctionName(DefaultSymbol),
    
    DuplicateClassFieldName(DefaultSymbol),

    UndefinedVariable(DefaultSymbol),

    FunctionExpected,

    IncorrectArgumentCount {
        expected: usize,
        actual: usize,
    },

    ReturnStatementOutsideFunction,

    IncorrectReturnType {
        expected: String,
        actual: String,
    },
    
    InvalidLValue,

    UndefinedType,
    
    DuplicateType(DefaultSymbol),

    TypeInference,
}

impl SpannableError for SemanticError {
    fn format(&self, ctx: &CompilerContext) -> String {
        use SemanticError::*;

        let string_interner = &ctx.string_interner;

        match self {
            MismatchedUnaryOperatorTypes { operator_type, operand_type } => {
                format!("Error: Operator {operator_type} not defined on {operand_type}")
            }
            MismatchedBinaryOperatorTypes { op, lhs_data_type, rhs_data_type } => {
                format!("Error: Operator {op} not defined for {lhs_data_type} and {rhs_data_type}")
            }
            InvalidAssignment { lhs_data_type, rhs_data_type, } => {
                format!("Error: Cannot assign a {rhs_data_type} to a {lhs_data_type}")
            }
            MismatchedTypes { expected, actual } => {
                format!("Error: Mismatched types: Expected {expected}, but got {actual}")
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
            DuplicateClassFieldName(field_name) => {
                format!("Error: Duplicate field definition: {}",
                        string_interner.get_str(*field_name)
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
                format!("Error: Incorrect Return type: Expected {expected}, but got {actual}")
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
