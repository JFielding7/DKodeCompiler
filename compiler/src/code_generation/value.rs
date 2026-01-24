use inkwell::builder::Builder;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, PointerValue};

pub enum Value<'llvm_ctx> {
    Void,
    RValue(BasicValueEnum<'llvm_ctx>),
    LValue {
        ptr: PointerValue<'llvm_ctx>,
        pointee_type: BasicTypeEnum<'llvm_ctx>
    },
}

impl<'llvm_ctx> Value<'llvm_ctx> {
    pub fn to_rvalue(&self, builder: &Builder<'llvm_ctx>) -> BasicValueEnum<'llvm_ctx> {
        use Value::*;

        match self {
            RValue(val) => *val,
            LValue { ptr, pointee_type } => {
                builder.build_load(*pointee_type, *ptr, "").unwrap()
            },
            Void => unreachable!("RValues cannot be void"),
        }
    }
}
