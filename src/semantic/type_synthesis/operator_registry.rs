use crate::semantic::type_synthesis::builtin_operator_registry::BuiltinOperatorRegistry;
use std::collections::HashMap;
use std::hash::Hash;
use crate::compiler_context::type_arena::{DataTypeId, TypeArena};

pub struct OperatorRegistry<OpType: Eq + Hash + BuiltinOperatorRegistry> {
    implementations: HashMap<OpType, HashMap<OpType::Operands, DataTypeId>>
}

impl<OpType: Eq + Hash + BuiltinOperatorRegistry> OperatorRegistry<OpType> {
    pub fn new() -> Self {
        Self {
            implementations: HashMap::new()
        }
    }

    pub fn operation_type(&self, op_type: OpType, operands: &OpType::Operands, type_arena: &TypeArena) -> Option<DataTypeId> {
        if let Some(data_type_id) = op_type.builtin_operations(operands, type_arena) {
            return Some(data_type_id);
        }

        let definitions = match self.implementations.get(&op_type) {
            Some(definitions) => definitions,
            None => return None,
        };

        definitions.get(operands).copied()
    }
}
