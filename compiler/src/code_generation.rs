use std::fs;
use crate::semantic_analysis::SemanticAnalysisOutput;
use inkwell::context::Context;
use generator::CodeGenerator;
use crate::compiler_context::CompilerContext;

pub mod generator;
mod value;

pub fn generate_code(ast: SemanticAnalysisOutput, ctx: &mut CompilerContext) {
    const OUTPUT_FILE: &str = "dkwon.ll";

    let llvm_context = Context::create();
    let llvm_code = CodeGenerator::generate_llvm(&ast, &llvm_context, ctx);

    fs::write(OUTPUT_FILE, llvm_code).unwrap();
}