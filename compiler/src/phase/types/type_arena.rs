use crate::phase::Phase;
use crate::phase::types::builtin_type::BuiltinType;
use crate::phase::types::data_type::{DataType, DataTypeEnum, DataTypeId, FunctionDataType, FunctionDataTypeId};
use std::collections::HashMap;
use string_interner::DefaultSymbol;
use crate::compiler_context::CompilerContext;

#[derive(Debug)]
pub struct TypeArena<T: Phase> {
    pub data_types: Vec<DataType<T>>,
    pub function_types: Vec<FunctionDataType>,
    pub function_type_ids: HashMap<FunctionDataType, DataTypeId>,
}

impl<T: Phase> TypeArena<T> {

    pub fn add_new_type(&mut self, data_type: DataType<T>) -> DataTypeId {
        let id = self.data_types.len();
        self.data_types.push(data_type);
        DataTypeId::new(id)
    }

    pub fn get_data_type(&self, id: DataTypeId) -> &DataType<T> {
        &self.data_types[id.as_usize()]
    }

    pub fn get_function_data_type(&self, id: FunctionDataTypeId) -> &FunctionDataType {
        &self.function_types[id.as_usize()]
    }
    
    pub fn get_data_type_mut(&mut self, id: DataTypeId) -> &mut DataType<T> {
        &mut self.data_types[id.as_usize()]
    }

    pub fn get_builtin_type_id(&self, builtin_type: BuiltinType) -> DataTypeId {
        DataTypeId::new(builtin_type.as_usize())
    }

    pub fn get_type_id(&self, name: DefaultSymbol, ctx: &CompilerContext) -> Option<DataTypeId> {
        let name_str = ctx.string_interner.get_str(name);

        if let Some(builtin_type) = BuiltinType::from_str(name_str) {
            Some(self.get_builtin_type_id(builtin_type))
        } else {
            unimplemented!("User defined Types")
        }
    }

    pub fn format_type(&self, id: DataTypeId, ctx: &CompilerContext) -> String {
        use DataTypeEnum::*;
        
        match &self.get_data_type(id).data_type_kind {
            Builtin(builtin_type) => format!("{builtin_type}"),
            UserDefined(data_type) => ctx.string_interner.get_str(*data_type).to_string(),
            Fn(function_type_id) => {
                let function_type = &self.function_types[function_type_id.as_usize()];

                format!("fn({}): {}",
                        function_type.param_types
                            .iter()
                            .map(|t| self.format_type(*t, ctx))
                            .collect::<Vec<String>>().join(", "), 
                        self.format_type(function_type.return_type, ctx)
                )
            }
        }
    }
}
