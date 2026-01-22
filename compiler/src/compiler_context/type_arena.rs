use crate::compiler_context::global_string_interner::GlobalStringInterner;
use crate::types::builtin_type::BuiltinType;
use crate::types::data_type::{DataType, DataTypeId};
use std::collections::HashMap;
use std::iter::Map;
use std::ops::Range;
use std::vec::IntoIter;
use string_interner::DefaultSymbol;
use strum::IntoEnumIterator;

#[derive(Debug)]
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

    fn add_type(&mut self, data_type: DataType) -> DataTypeId {
        let id = self.data_types.len();
        self.data_types.push(data_type);
        DataTypeId(id)
    }

    pub fn add_if_new_type(&mut self, data_type: DataType) -> DataTypeId {
        for (i, t) in self.data_types.iter().enumerate() {
            if *t == data_type {
                return DataTypeId(i)
            }
        }

        self.add_type(data_type)
    }

    pub fn get_data_type(&self, id: DataTypeId) -> &DataType {
        &self.data_types[id.as_usize()]
    }

    pub fn builtin_type_id(&self, builtin_type: BuiltinType) -> DataTypeId {
        DataTypeId(builtin_type.as_usize())
    }

    pub fn get_type_id(&self, name: DefaultSymbol, string_interner: &GlobalStringInterner) -> Option<DataTypeId> {
        let name_str = string_interner.get_str(name);

        if let Some(builtin_type) = BuiltinType::from_str(name_str) {
            Some(self.builtin_type_id(builtin_type))
        } else {
            self.user_defined_type_ids.get(&name).copied()
        }
    }

    pub fn format_type(&self, id: DataTypeId, string_interner: &GlobalStringInterner) -> String {
        use DataType::*;
        
        match self.get_data_type(id) {
            Builtin(builtin_type) => format!("{builtin_type}"),
            UserDefined(data_type) => string_interner.get_str(*data_type).to_string(),
            Fn(function_type) => {
                format!("fn({}): {}",
                        function_type.param_types
                            .iter()
                            .map(|t| self.format_type(*t, string_interner))
                            .collect::<Vec<String>>().join(", "), 
                        self.format_type(function_type.return_type, string_interner)
                )
            }
        }
    }
}

impl IntoIterator for &TypeArena {
    type Item = DataTypeId;
    type IntoIter = Map<Range<usize>, fn(usize) -> DataTypeId>;

    fn into_iter(self) -> Self::IntoIter {
        (0..self.data_types.len()).map(DataTypeId)
    }
}
