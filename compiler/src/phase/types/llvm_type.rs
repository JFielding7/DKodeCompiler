use inkwell::types::StructType;
use inkwell::types::{BasicMetadataTypeEnum, BasicTypeEnum, FunctionType};
use inkwell::AddressSpace;
use crate::phase::types::llvm_type::LLVMDataType::Function;

#[derive(Debug, Copy, Clone)]
pub enum LLVMDataType<'llvm_ctx> {
    Unit,
    BasicType(BasicTypeEnum<'llvm_ctx>),
    StructType(StructType<'llvm_ctx>),
    Function(FunctionType<'llvm_ctx>),
}

impl<'llvm_ctx> LLVMDataType<'llvm_ctx> {
    pub fn function_type(self) -> FunctionType<'llvm_ctx> {
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
