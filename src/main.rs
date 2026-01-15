use crate::ast::arena_ast::AST;
use crate::compiler_context::CompilerContext;
use crate::error::Error;
use crate::semantic::semantic_analysis;
use crate::source::source_file::SourceFile;
use error::Result;
use crate::error::compiler_error::CompilerResult;
use crate::lexer::lexical_analysis;
use crate::syntax::syntax_analysis;

mod lexer;
mod syntax;
mod error;
mod source;
mod semantic;
mod ast;
mod compiler_context;
mod types;
mod operators;

fn compile_source_file(source_file: &SourceFile, compiler_context: &mut CompilerContext) -> CompilerResult<()> {

    let source_lines = lexical_analysis(source_file, compiler_context)?;

    let ast: AST = syntax_analysis(source_lines, compiler_context)?;
    
    let annotated_ast = semantic_analysis(ast, compiler_context)?;

    println!("{:?}", annotated_ast);

    Ok(())
}

fn compile_program(args: Vec<String>, compiler_context: &mut CompilerContext) -> Result<()> {
    use Error::*;
    
    const MIN_ARG_COUNT: usize = 2;

    if args.len() < MIN_ARG_COUNT {
        return Err(NoInputFiles)
    }

    for source_file_name in args.into_iter().skip(1) {
        let source_file = SourceFile::read(source_file_name.clone())
            .map_err(|err| FileRead { file_name: source_file_name, error: err })?;

        compile_source_file(&source_file, compiler_context)
            .map_err(|error| Compiler { source_file, error })?;
    }

    Ok(())
}

fn main()  {
    let args = std::env::args().collect::<Vec<_>>();
    let mut compiler_context = CompilerContext::new();

    if let Err(err) = compile_program(args, &mut compiler_context) {
        println!("{}", err.format(&compiler_context));
    }
}
