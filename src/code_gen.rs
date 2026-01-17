use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use crate::compiler_context::CompilerContext;
use crate::semantic::AnnotatedAST;

struct CodeGenerator<'llvm_ctx, 'compiler_ctx> {
    llvm_context: &'llvm_ctx Context,
    module: Module<'llvm_ctx>,
    builder: Builder<'llvm_ctx>,
    compiler_context: &'compiler_ctx CompilerContext,
    annotated_ast: AnnotatedAST
}

impl<'llvm_ctx, 'compiler_ctx> CodeGenerator<'llvm_ctx, 'compiler_ctx> {
    fn new(annotated_ast: AnnotatedAST, llvm_context: &'llvm_ctx Context, compiler_context: &'compiler_ctx CompilerContext) -> Self {
        let module = llvm_context.create_module("code");
        let builder = llvm_context.create_builder();

        Self {
            llvm_context,
            module,
            builder,
            compiler_context,
            annotated_ast
        }
    }
}
