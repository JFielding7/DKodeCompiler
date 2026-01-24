use crate::phase::Phase;
use crate::phase::types::builtin_type::BuiltinType;
use std::collections::HashMap;
use string_interner::DefaultSymbol;

#[derive(Debug)]
pub struct DataType<T: Phase> {
    pub data_type_kind: DataTypeEnum,
    pub methods: HashMap<DefaultSymbol, Method<T>>,
    pub llvm_type: T::LLVMDataType
}

impl<T: Phase> PartialEq for DataType<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data_type_kind == other.data_type_kind
    }
}

impl<T: Phase> DataType<T> {
    pub fn new(data_type_kind: DataTypeEnum, llvm_type: T::LLVMDataType) -> Self {
        Self {
            data_type_kind,
            methods: HashMap::new(),
            llvm_type
        }
    }

    pub fn add_method(&mut self, symbol: DefaultSymbol, function: Method<T>) {
        self.methods.insert(symbol, function);
    }

    pub fn get_method(&self, symbol: DefaultSymbol) -> &Method<T> {
        self.methods.get(&symbol).unwrap()
    }

    pub fn get_method_mut(&mut self, symbol: DefaultSymbol) -> &mut Method<T> {
        self.methods.get_mut(&symbol).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct Method<T: Phase> {
    pub data_type_id: FunctionDataTypeId,
    pub llvm_value: T::LLVMFunction,
}

impl<T: Phase> Method<T> {
    pub fn new(data_type_id: FunctionDataTypeId, llvm_value: T::LLVMFunction) -> Self {
        Self {
            data_type_id,
            llvm_value,
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
