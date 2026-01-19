use inkwell::builder::Builder;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, PointerValue};

pub enum Value<'llvm_ctx> {
    RValue(Option<BasicValueEnum<'llvm_ctx>>),
    LValue {
        ptr: PointerValue<'llvm_ctx>,
        pointee_type: BasicTypeEnum<'llvm_ctx>
    },
}

pub struct LValueType<'llvm_ctx> {
    pub ptr: PointerValue<'llvm_ctx>,
    pointee_type: BasicTypeEnum<'llvm_ctx>
}

impl<'llvm_ctx> LValueType<'llvm_ctx> {
    pub fn new(ptr: PointerValue<'llvm_ctx>, pointee_type: BasicTypeEnum<'llvm_ctx>) -> Self {
        Self {
            ptr,
            pointee_type,
        }
    }
}

impl<'llvm_ctx> Value<'llvm_ctx> {
    pub fn to_rvalue(&self, builder: &Builder<'llvm_ctx>) -> BasicValueEnum<'llvm_ctx> {
        match self {
            Value::RValue(val) => val.unwrap(),
            Value::LValue { ptr, pointee_type } => {
                builder.build_load(*pointee_type, *ptr, "").unwrap()
            },
        }
    }
}
