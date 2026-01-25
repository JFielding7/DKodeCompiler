use crate::phase::Phase;
use crate::phase::types::builtin_type::BuiltinType;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use string_interner::DefaultSymbol;

#[derive(Debug)]
pub struct DataType<T: Phase> {
    pub data_type_kind: DataTypeEnum,
    pub fields: HashMap<DefaultSymbol, Field<T>>,
    pub methods: HashMap<DefaultSymbol, Method<T>>,
    pub data_type_repr: T::DataTypeRepr
}

impl<T: Phase> PartialEq for DataType<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data_type_kind == other.data_type_kind
    }
}

impl<T: Phase> DataType<T> {
    pub fn new(data_type_kind: DataTypeEnum, data_type_repr: T::DataTypeRepr) -> Self {
        Self {
            data_type_kind,
            fields: HashMap::new(),
            methods: HashMap::new(),
            data_type_repr,
        }
    }

    pub fn add_field(&mut self, symbol: DefaultSymbol, field: Field<T>) {
        self.fields.insert(symbol, field);
    }

    pub fn get_field(&self, symbol: DefaultSymbol) -> &Field<T> {
        self.fields.get(&symbol).unwrap()
    }

    pub fn add_method(&mut self, symbol: DefaultSymbol, function: Method<T>) {
        self.methods.insert(symbol, function);
    }

    pub fn get_method(&self, symbol: DefaultSymbol) -> &Method<T> {
        self.methods.get(&symbol).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum DataTypeEnum {
    Builtin(BuiltinType),
    UserDefined(DefaultSymbol),
    Fn(FunctionDataTypeId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionDataType {
    pub param_types: Vec<DataTypeId>,
    pub return_type: DataTypeId,
}

impl FunctionDataType {
    pub fn new(param_types: Vec<DataTypeId>, return_type: DataTypeId) -> Self {
        Self {
            param_types,
            return_type,
        }
    }
}

#[derive(Debug)]
pub struct Field<T: Phase> {
    pub data_type_id: DataTypeId,
    pub field_repr: T::VariableRepr
}

impl<T: Phase> Field<T> {
    pub fn new(data_type_id: DataTypeId, field_repr: T::VariableRepr) -> Self {
        Self {
            data_type_id,
            field_repr,
        }
    }
}

#[derive(Debug)]
pub struct Method<T: Phase> {
    pub data_type_id: FunctionDataTypeId,
    pub function_repr: T::FunctionRepr,
}

impl<T: Phase> Method<T> {
    pub fn new(data_type_id: FunctionDataTypeId, function_repr: T::FunctionRepr) -> Self {
        Self {
            data_type_id,
            function_repr,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DataTypeId(usize);

impl DataTypeId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
    
    pub fn as_usize(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FunctionDataTypeId(usize);

impl FunctionDataTypeId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}
