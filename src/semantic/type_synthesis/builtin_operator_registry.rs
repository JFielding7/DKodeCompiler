use std::hash::Hash;
use crate::compiler_context::type_arena::{DataTypeId, TypeArena};
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::unary_operators::UnaryOperator;
use crate::types::data_type::DataType::{Builtin, UserDefined};
use crate::types::data_type::BuiltinType;

pub trait BuiltinOperatorRegistry {
    type Operands: Hash + Eq;

    fn builtin_operations(&self, operand: &Self::Operands, type_arena: &TypeArena) -> Option<DataTypeId>;
}

impl BuiltinOperatorRegistry for UnaryOperator {
    type Operands = DataTypeId;

    fn builtin_operations(&self, operand_type_id: &DataTypeId, type_arena: &TypeArena) -> Option<DataTypeId> {
        use UnaryOperator::*;
        use BuiltinType::*;

        let operand_type = match type_arena.get(*operand_type_id) {
            Builtin(builtin_type) => builtin_type,
            UserDefined(_) => return None,
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

        Some(type_arena.builtin_type_id(builtin_type))
    }
}

impl BuiltinOperatorRegistry for BinaryOperator {
    type Operands = (DataTypeId, DataTypeId);

    fn builtin_operations(&self, operand_ids: &(DataTypeId, DataTypeId), type_arena: &TypeArena) -> Option<DataTypeId> {
        use BinaryOperator::*;
        use BuiltinType::*;

        let (lhs_type_id, rhs_type_id) = *operand_ids;

        if *self == Assign && lhs_type_id == rhs_type_id {
            return Some(rhs_type_id)
        } else if *self == CommaOperator {
            return Some(rhs_type_id)
        }

        println!("{:?} {:?} {}", lhs_type_id, rhs_type_id, lhs_type_id == rhs_type_id);

        let lhs_type = match type_arena.get(lhs_type_id) {
            Builtin(builtin_type) => builtin_type,
            _ => return None,
        };

        let rhs_type = match type_arena.get(rhs_type_id) {
            Builtin(builtin_type) => builtin_type,
            _ => return None
        };

        let builtin_type_res = match self {

            AddAssign | SubAssign | MulAssign | DivAssign | ModAssign => match (lhs_type, rhs_type) {
                (Int, Int) => Int,
                _ => return None,
            },

            LeftShiftAssign | RightShiftAssign => match (lhs_type, lhs_type) {
                (Int, Int) => Int,
                _ => return None,
            },

            AndAssign | XorAssign | OrAssign => match (lhs_type, lhs_type) {
                (Int, Int) => Int,
                _ => return None,
            },

            Add => match (lhs_type, lhs_type) {
                (Int, Int) => Int,
                (String, String) => String,
                _ => return None,
            },

            Sub | Mul | Div | Mod => match (lhs_type, lhs_type) {
                (Int, Int) => Int,
                _ => return None,
            },

            BitAnd | BitOr | BitXor | LeftShift | RightShift => match (lhs_type, lhs_type) {
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

            LessThan | LessOrEqual | GreaterThan | GreaterOrEqual => match (lhs_type, lhs_type) {
                (Int, Int) | (String, String) => Bool,
                _ => return None,
            },

            LogicalAnd | LogicalOr => match (lhs_type, lhs_type) {
                (Bool, Bool) => Bool,
                _ => return None,
            },

            _ => return None,
        };

        Some(type_arena.builtin_type_id(builtin_type_res))
    }
}
