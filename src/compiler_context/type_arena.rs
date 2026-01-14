use std::collections::HashMap;
use string_interner::{DefaultBackend, DefaultSymbol, StringInterner};
use strum::IntoEnumIterator;
use crate::types::builtin_type::BuiltinType;
use crate::types::data_type::{DataType, DataTypeId};
use crate::types::type_annotation::TypeAnnotation;

pub struct TypeArena {
    data_types: Vec<DataType>,
    user_defined_type_ids: HashMap<DefaultSymbol, DataTypeId>,
}

impl TypeArena {
    pub fn new() -> Self {
        Self {
            data_types: BuiltinType::iter()
                .map(|t| DataType::Builtin(t))
                .collect(),
            user_defined_type_ids: HashMap::new(),
        }
    }

    pub fn add_type(&mut self, data_type: DataType) -> DataTypeId {
        let id = self.data_types.len();
        self.data_types.push(data_type);
        DataTypeId(id)
    }

    pub fn get(&self, id: DataTypeId) -> &DataType {
        &self.data_types[id.as_usize()]
    }

    pub fn builtin_type_id(&self, builtin_type: BuiltinType) -> DataTypeId {
        DataTypeId(builtin_type.as_usize())
    }

    pub fn get_type_id(&self, name: DefaultSymbol, string_interner: &StringInterner<DefaultBackend>) -> Option<DataTypeId> {
        let name_str = string_interner.resolve(name).expect("String must be interned");

        if let Some(builtin_type) = BuiltinType::from_string(name_str) {
            Some(self.builtin_type_id(builtin_type))
        } else {
            self.user_defined_type_ids.get(&name).copied()
        }
    }
}
