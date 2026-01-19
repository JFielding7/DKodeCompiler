use inkwell::builder::Builder;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, PointerValue};

pub enum Value<'llvm_ctx> {
    RValue(BasicValueEnum<'llvm_ctx>),
    LValue {
        ptr: PointerValue<'llvm_ctx>,
        pointee_type: BasicTypeEnum<'llvm_ctx>
    },
}

pub struct LValueType<'llvm_ctx> {
    pub ptr: PointerValue<'llvm_ctx>,
    pointee_type: BasicTypeEnum<'llvm_ctx>
}

impl<'ctx> LValueType<'ctx> {
    pub fn new(ptr: PointerValue<'static>, pointee_type: BasicTypeEnum<'ctx>) -> Self {
        Self {
            ptr,
            pointee_type,
        }
    }
}

impl<'ctx> Value<'ctx> {
    pub fn to_rvalue(&self, builder: &Builder<'ctx>) -> BasicValueEnum<'ctx> {
        match self {
            Value::RValue(val) => *val,
            Value::LValue { ptr, pointee_type } => {
                builder.build_load(*pointee_type, *ptr, "").unwrap()
            },
        }
    }
}
