use string_interner::DefaultSymbol;
use crate::types::builtin_type::BuiltinType;

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Builtin(BuiltinType),
    UserDefined(DefaultSymbol),
    Fn(FunctionDataType),
}

impl DataType {
    pub fn function(param_types: Vec<DataTypeId>, return_type: DataTypeId) -> Self {
        DataType::Fn(FunctionDataType {
            param_types,
            return_type,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDataType {
    pub param_types: Vec<DataTypeId>,
    pub return_type: DataTypeId
}



#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DataTypeId(pub usize);

impl DataTypeId {
    pub fn as_usize(&self) -> usize {
        self.0
    }
}
