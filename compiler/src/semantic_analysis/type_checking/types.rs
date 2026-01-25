use std::collections::HashMap;
use string_interner::DefaultSymbol;
use strum::IntoEnumIterator;
use crate::phase::types::builtin_type::BuiltinType;
use crate::phase::types::data_type::{DataType, DataTypeEnum, DataTypeId, FunctionDataType, FunctionDataTypeId};
use crate::phase::types::data_type::DataTypeEnum::Fn;
use crate::phase::types::type_arena::{FunctionDataTypeBinding, TypeArena};
use crate::semantic_analysis::type_checking::TypeChecking;

impl TypeArena<TypeChecking> {
    pub fn type_checking_type_arena() -> Self {
        Self {
            data_types: BuiltinType::iter()
                .map(|t| DataTypeEnum::Builtin(t).into())
                .collect(),
            user_defined_ids: HashMap::new(),
            function_types: Vec::new(),
            function_type_ids: HashMap::new(),
        }
    }

    pub fn insert_new_type(&mut self, type_name: DefaultSymbol, data_type: DataType<TypeChecking>) -> Option<DataTypeId> {
        if self.user_defined_ids.contains_key(&type_name) {
            None
        } else {
            let data_type_id = self.add_new_type(data_type);
            self.user_defined_ids.insert(type_name, data_type_id);
            Some(data_type_id)
        }
    }

    pub fn get_or_insert_function_type(&mut self, function_data_type: FunctionDataType) -> FunctionDataTypeId {
        if let Some(&data_type_id) = self.function_type_ids.get(&function_data_type) {
            data_type_id
        } else {
            let function_type_id = FunctionDataTypeId::new(self.function_types.len());
            let data_type_id = self.add_new_type(Fn(function_type_id).into());

            self.function_type_ids.insert(function_data_type.clone(), function_type_id);
            self.function_types.push(FunctionDataTypeBinding::new(data_type_id, function_data_type));

            function_type_id
        }
    }
}

impl From<DataTypeEnum> for DataType<TypeChecking> {
    fn from(data_type_kind: DataTypeEnum) -> Self {
        Self {
            data_type_kind,
            fields: HashMap::new(),
            methods: HashMap::new(),
            data_type_repr: ()
        }
    }
}
