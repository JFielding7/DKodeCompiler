use crate::code_gen::generator::CodeGenerator;
use crate::compiler_context::CompilerContext;
use crate::semantic::AnnotatedAST;
use inkwell::context::Context;

pub mod generator;
mod value;

pub fn generate_code(ast: AnnotatedAST, ctx: &mut CompilerContext) {
    let llvm_context = Context::create();
    CodeGenerator::generate_llvm(&ast, &llvm_context, ctx);
}