use string_interner::DefaultSymbol;
use crate::types::builtin_type::BuiltinType;

#[derive(Debug, Clone)]
pub enum DataType {
    Builtin(BuiltinType),
    UserDefined(DefaultSymbol),
    Fn { 
        param_types: Vec<DataTypeId>, 
        return_type: DataTypeId
    },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DataTypeId(pub usize);

impl DataTypeId {
    pub fn as_usize(&self) -> usize {
        self.0
    }
}
