use std::collections::HashMap;
use inkwell::types::StructType;
use inkwell::types::{BasicMetadataTypeEnum, BasicTypeEnum, FunctionType};
use inkwell::AddressSpace;
use crate::code_generation::CodeGeneration;
use crate::phase::types::type_arena::TypeArena;

impl TypeArena<CodeGeneration<'_>> {
    pub fn code_generation_type_arena() -> Self {
        Self {
            data_types: Vec::new(),
            user_defined_ids: HashMap::new(),
            function_types: Vec::new(),
            function_type_ids: HashMap::new(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum LLVMDataType<'llvm_ctx> {
    Unit,
    BasicType(BasicTypeEnum<'llvm_ctx>),
    StructType(StructType<'llvm_ctx>),
    Function(FunctionType<'llvm_ctx>),
}

impl<'llvm_ctx> LLVMDataType<'llvm_ctx> {
    pub fn function_type(self) -> FunctionType<'llvm_ctx> {
        use LLVMDataType::*;

        if let Function(function) = self {
            function
        } else {
            unreachable!("LLVMDataType must be a function")
        }
    }
}

impl<'llvm_ctx> From<LLVMDataType<'llvm_ctx>> for BasicTypeEnum<'llvm_ctx> {
    fn from(data_type: LLVMDataType<'llvm_ctx>) -> Self {
        use LLVMDataType::*;

        match data_type {
            Unit => unreachable!("Void is not a basic type"),
            BasicType(basic_type) => basic_type.into(),
            StructType(struct_type) => {
                struct_type.get_context().ptr_type(AddressSpace::default()).into()
            },
            Function(func_type) => {
                func_type.get_context().ptr_type(AddressSpace::default()).into()
            },
        }
    }
}

impl<'llvm_ctx> From<LLVMDataType<'llvm_ctx>> for BasicMetadataTypeEnum<'llvm_ctx> {
    fn from(data_type: LLVMDataType<'llvm_ctx>) -> Self {
        let basic_type: BasicTypeEnum = data_type.into();
        basic_type.into()
    }
}
