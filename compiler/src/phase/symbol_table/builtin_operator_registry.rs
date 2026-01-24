use crate::phase::types::type_arena::TypeArena;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::unary_operators::UnaryOperator;
use crate::phase::types::data_type::DataTypeId;
use crate::phase::types::builtin_type::BuiltinType;
use std::hash::Hash;
use crate::phase::Phase;
use crate::phase::types::data_type::DataTypeEnum::Builtin;

pub trait BuiltinOperatorRegistry<T: Phase> {
    type Operands: Hash + Eq;

    fn builtin_operations(&self, operand: &Self::Operands, type_arena: &TypeArena<T>) -> Option<DataTypeId>;
}

impl<T: Phase> BuiltinOperatorRegistry<T> for UnaryOperator {
    type Operands = DataTypeId;

    fn builtin_operations(&self, operand_type_id: &DataTypeId, type_arena: &TypeArena<T>) -> Option<DataTypeId> {
        use UnaryOperator::*;
        use BuiltinType::*;

        let operand_type = match type_arena.get_data_type(*operand_type_id).data_type_kind {
            Builtin(builtin_type) => builtin_type,
            _ => return None,
        };

        let builtin_type = match self {
            Neg => match operand_type {
                Int => Int,
                _ => return None,
            },

            Not => match operand_type {
                Bool => Bool,
                _ => return None,
            },

            BitNot | PreInc | PreDec | PostInc | PostDec => match operand_type {
                Int => Int,
                _ => return None,
            },
        };

        Some(type_arena.get_builtin_type_id(builtin_type))
    }
}

impl<T: Phase> BuiltinOperatorRegistry<T> for BinaryOperator {
    type Operands = (DataTypeId, DataTypeId);

    fn builtin_operations(&self, operand_ids: &(DataTypeId, DataTypeId), type_arena: &TypeArena<T>) -> Option<DataTypeId> {
        use BinaryOperator::*;
        use BuiltinType::*;

        let (lhs_type_id, rhs_type_id) = *operand_ids;

        if *self == Assign && lhs_type_id == rhs_type_id {
            return Some(rhs_type_id)
        } else if *self == CommaOperator {
            return Some(rhs_type_id)
        }

        let lhs_type = match type_arena.get_data_type(lhs_type_id).data_type_kind {
            Builtin(builtin_type) => builtin_type,
            _ => return None,
        };

        let rhs_type = match type_arena.get_data_type(rhs_type_id).data_type_kind {
            Builtin(builtin_type) => builtin_type,
            _ => return None
        };

        let operand_types = (lhs_type, rhs_type);

        let builtin_type_res = match self {

            AddAssign | SubAssign | MulAssign | DivAssign | ModAssign => match operand_types {
                (Int, Int) => Int,
                _ => return None,
            },

            LeftShiftAssign | RightShiftAssign => match operand_types {
                (Int, Int) => Int,
                _ => return None,
            },

            AndAssign | XorAssign | OrAssign => match operand_types {
                (Int, Int) => Int,
                _ => return None,
            },

            Add => match operand_types {
                (Int, Int) => Int,
                (Str, Str) => Str,
                _ => return None,
            },

            Sub | Mul | Div | Mod => match operand_types {
                (Int, Int) => Int,
                _ => return None,
            },

            BitAnd | BitOr | BitXor | LeftShift | RightShift => match operand_types {
                (Int, Int) => Int,
                _ => return None,
            },

            Equal | NotEquals => {
                if lhs_type == rhs_type {
                    Bool
                } else {
                    return None
                }
            },

            LessThan | LessOrEqual | GreaterThan | GreaterOrEqual => match operand_types {
                (Int, Int) | (Str, Str) => Bool,
                _ => return None,
            },

            LogicalAnd | LogicalOr => match operand_types {
                (Bool, Bool) => Bool,
                _ => return None,
            },

            _ => return None,
        };

        Some(type_arena.get_builtin_type_id(builtin_type_res))
    }
}
