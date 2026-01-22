use std::fs;
use crate::compiler_context::CompilerContext;
use crate::semantic_analysis::AnnotatedAST;
use inkwell::context::Context;
use generator::CodeGenerator;

pub mod generator;
mod value;
mod types;

pub fn generate_code(ast: AnnotatedAST, ctx: &mut CompilerContext) {
    const OUTPUT_FILE: &str = "dkwon.ll";

    let llvm_context = Context::create();
    let llvm_code = CodeGenerator::generate_llvm(&ast, &llvm_context, ctx);

    fs::write(OUTPUT_FILE, llvm_code).unwrap();
}