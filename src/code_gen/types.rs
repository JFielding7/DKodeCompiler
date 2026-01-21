use std::collections::HashMap;
use inkwell::types::{BasicMetadataTypeEnum, BasicTypeEnum, FunctionType, StructType};
use inkwell::AddressSpace;
use inkwell::values::FunctionValue;
use string_interner::DefaultSymbol;

pub struct LLVMDataType<'llvm_ctx> {
    methods: HashMap<DefaultSymbol, FunctionValue<'llvm_ctx>>,
    pub data_type: LLVMDataTypeEnum<'llvm_ctx>,
}

impl<'llvm_ctx> LLVMDataType<'llvm_ctx> {
    pub fn new(data_type: LLVMDataTypeEnum<'llvm_ctx>) -> Self {
        Self {
            methods: HashMap::new(),
            data_type,
        }
    }
    
    pub fn add_method(&mut self, symbol: DefaultSymbol, function: FunctionValue<'llvm_ctx>) {
        self.methods.insert(symbol, function);
    }

    pub fn get_method(&self, symbol: DefaultSymbol) -> FunctionValue<'llvm_ctx> {
        *self.methods.get(&symbol).unwrap()
    }
}

pub enum LLVMDataTypeEnum<'llvm_ctx> {
    Unit,
    BasicType(BasicTypeEnum<'llvm_ctx>),
    StructType(StructType<'llvm_ctx>),
    Function(FunctionType<'llvm_ctx>),
}

impl<'llvm_ctx> From<&LLVMDataTypeEnum<'llvm_ctx>> for BasicTypeEnum<'llvm_ctx> {
    fn from(data_type: &LLVMDataTypeEnum<'llvm_ctx>) -> Self {
        use LLVMDataTypeEnum::*;
        
        match data_type {
            Unit => unreachable!("Void is not a basic type"),
            BasicType(basic_type) => *basic_type,
            StructType(struct_type) => {
                struct_type.get_context().ptr_type(AddressSpace::default()).into()
            },
            Function(func_type) => {
                func_type.get_context().ptr_type(AddressSpace::default()).into()
            },
        }
    }
}

impl<'llvm_ctx> From<&LLVMDataTypeEnum<'llvm_ctx>> for BasicMetadataTypeEnum<'llvm_ctx> {
    fn from(data_type: &LLVMDataTypeEnum<'llvm_ctx>) -> Self {
        let basic_type: BasicTypeEnum = data_type.into();
        basic_type.into()
    }
}
