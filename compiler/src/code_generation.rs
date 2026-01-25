use std::collections::HashMap;
use std::fs;
use std::marker::PhantomData;
use crate::semantic_analysis::SemanticAnalysisOutput;
use inkwell::context::Context;
use inkwell::values::{FunctionValue, PointerValue};
use string_interner::DefaultSymbol;
use generator::CodeGenerator;
use crate::code_generation::types::LLVMDataType;
use crate::compiler_context::CompilerContext;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::unary_operators::UnaryOperator;
use crate::phase::Phase;
use crate::phase::symbol_table::operator_registry::OperatorRegistry;
use crate::phase::symbol_table::symbol::Symbol;
use crate::phase::types::data_type::DataTypeId;

pub mod generator;
mod value;
mod types;
mod symbol_table;

#[derive(Debug)]
pub struct CodeGeneration<'llvm_ctx> {
    _marker: PhantomData<&'llvm_ctx ()>
}

impl<'llvm_ctx> Phase for CodeGeneration<'llvm_ctx> {
    type Symbols = HashMap<DefaultSymbol, Symbol<CodeGeneration<'llvm_ctx>>>;
    type UnaryOpImpl = OperatorRegistry<CodeGeneration<'llvm_ctx>, UnaryOperator>;
    type BinaryOpImpl = OperatorRegistry<CodeGeneration<'llvm_ctx>, BinaryOperator>;
    type SymbolDataTypeId = DataTypeId;
    type DataTypeRepr = LLVMDataType<'llvm_ctx>;
    type VariableRepr = PointerValue<'llvm_ctx>;
    type FunctionRepr = FunctionValue<'llvm_ctx>;
}


pub fn generate_code(ast: SemanticAnalysisOutput, ctx: &mut CompilerContext) {
    const OUTPUT_FILE: &str = "dkwon.ll";

    let llvm_context = Context::create();
    let llvm_code = CodeGenerator::generate_llvm(&ast, &llvm_context, ctx);

    fs::write(OUTPUT_FILE, llvm_code).unwrap();
}
