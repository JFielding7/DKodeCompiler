use std::collections::HashMap;
use std::hash::Hash;
use crate::phase::Phase;
use crate::phase::symbol_table::builtin_operator_registry::BuiltinOperatorRegistry;
use crate::phase::types::type_arena::TypeArena;
use crate::phase::types::data_type::DataTypeId;

// TODO: operator overloading
#[derive(Debug)]
pub struct OperatorRegistry<T: Phase, OpType: Eq + Hash + BuiltinOperatorRegistry<T>> {
    implementations: HashMap<OpType, HashMap<OpType::Operands, DataTypeId>>
}

impl<T: Phase, OpType: Eq + Hash + BuiltinOperatorRegistry<T>> OperatorRegistry<T, OpType> {
    pub fn new() -> Self {
        Self {
            implementations: HashMap::new()
        }
    }

    pub fn operation_type(&self, op_type: OpType, operands: &OpType::Operands, type_arena: &TypeArena<T>) -> Option<DataTypeId> {
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
