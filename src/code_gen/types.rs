use inkwell::AddressSpace;
use crate::types::builtin_type::BuiltinType;
use crate::types::data_type::{DataType, DataTypeId, FunctionDataType};
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType};
use super::CodeGenerator;
use crate::code_gen::types::LLVMDataType::Function;

pub enum LLVMDataType<'llvm_ctx> {
    Void,
    BasicType(BasicTypeEnum<'llvm_ctx>),
    Function(FunctionType<'llvm_ctx>),
}

impl<'llvm_ctx> From<LLVMDataType<'llvm_ctx>> for BasicTypeEnum<'llvm_ctx> {
    fn from(data_type: LLVMDataType<'llvm_ctx>) -> Self {
        use LLVMDataType::*;
        
        match data_type {
            Void => unreachable!("Void is not a basic type"),
            BasicType(basic_type) => basic_type,
            Function(func_type) => func_type.get_context().ptr_type(AddressSpace::default()).into(),
        }
    }
}

impl<'llvm_ctx> From<LLVMDataType<'llvm_ctx>> for BasicMetadataTypeEnum<'llvm_ctx> {
    fn from(data_type: LLVMDataType<'llvm_ctx>) -> Self {
        let basic_type: BasicTypeEnum = data_type.into();
        basic_type.into()
    }
}

impl<'ast, 'llvm_ctx> CodeGenerator<'ast, 'llvm_ctx> {
    fn builtin_llvm_type(&self, builtin_type: &BuiltinType) -> LLVMDataType<'llvm_ctx> {
        use BuiltinType::*;
        use LLVMDataType::*;

        BasicType(match builtin_type {
            Unit => return Void,
            Bool => self.llvm_context.bool_type().into(),
            Int => self.llvm_context.i64_type().into(),
            String => unimplemented!("String type")
        })
    }

    fn function_type_helper(&self, function_type: &FunctionDataType) -> FunctionType<'llvm_ctx>  {
        use LLVMDataType::*;
        
        let mut params: Vec<BasicMetadataTypeEnum> = Vec::with_capacity(function_type.param_types.len());

        for &param_type in &function_type.param_types {
            params.push(self.llvm_type(param_type).into());
        }

        let ret_llvm_type = self.llvm_type(function_type.return_type);

        let basic_type = match ret_llvm_type {
            Void => return self.llvm_context.void_type().fn_type(&params, false),
            BasicType(basic_type) => basic_type,
            Function(func_type) => func_type.get_context().ptr_type(AddressSpace::default()).into(),
        };

        basic_type.fn_type(&params, false)
    }

    pub fn function_type(&self, data_type: DataTypeId) -> FunctionType<'llvm_ctx> {
        let data_type = self.compiler_context.type_arena.get_data_type(data_type);

        match data_type {
            DataType::Fn(function_type) => {
                self.function_type_helper(function_type)
            },
            _ => unreachable!("Must be a function type")
        }
    }

    pub fn llvm_type(&self, data_type_id: DataTypeId) -> LLVMDataType<'llvm_ctx> {
        use DataType::*;
        
        let data_type = self.compiler_context.type_arena.get_data_type(data_type_id);

        match data_type {
            Builtin(builtin_type) => self.builtin_llvm_type(builtin_type),
            UserDefined(name) => unimplemented!("UserDefined types not implemented"),
            Fn(function_type) => {
                Function(self.function_type_helper(function_type))
            },
        }
    }
}


