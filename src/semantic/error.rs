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

    DuplicateParameterName(DefaultSymbol),

    DuplicateFunctionName(DefaultSymbol),

    UndefinedVariable(DefaultSymbol),

    UndefinedType,

    TypeInference,
}

impl SpannableError for SemanticError {
    fn format(&self, ctx: &CompilerContext) -> String {
        match self {
            MismatchedUnaryOperatorTypes(op, id) => {
                format!("Error: Operator {op} not defined on {}",
                        ctx.type_arena.format_type(*id, &ctx.string_interner)
                )
            }
            MismatchedBinaryOperatorTypes(op, left, right) => {
                format!("Error: Operator {op} not defined for {} and {}", 
                        ctx.type_arena.format_type(*left, &ctx.string_interner), 
                        ctx.type_arena.format_type(*right, &ctx.string_interner)
                )
            }
            UndefinedVariable(var_name) => {
                format!("Error: Undefined variable: {}", 
                        ctx.string_interner.get_str(*var_name)
                )
            }
            DuplicateParameterName(param_name) => {
                format!("Error: Duplicate parameter definition: {}", 
                        ctx.string_interner.get_str(*param_name)
                )
            }
            DuplicateFunctionName(func_name) => {
                format!("Error: Duplicate function definition: {}", 
                        ctx.string_interner.get_str(*func_name)
                )
            }
            UndefinedType => "Error: Undefined type".to_string(),
            TypeInference => "Error: Cannot infer type".to_string(),
        }
    }
}
