use crate::ast::arena_ast::AST;
use crate::compiler_context::CompilerContext;
use crate::lexer::tokenizer::lex_source_file;
use crate::syntax::parser::ast_parser::ASTParser;
use error::compiler_error::CompilerError::NoInputFiles;
use error::compiler_error::CompilerResult;
use crate::error::compiler_error::CompilerError;
use crate::error::compiler_error::CompilerError::FileRead;
use crate::error::spanned_error::SpannedError;
use crate::semantic::name_resolution::NameResolver;
use crate::semantic::semantic_analysis;
use crate::semantic::type_synthesis::type_synthesizer::TypeSynthesizer;
use crate::source::source_file::SourceFile;

mod lexer;
mod syntax;
mod error;
mod source;
mod semantic;
mod ast;
mod compiler_context;
mod types;
mod operators;

fn compile_source_file(source_file: &SourceFile, compiler_context: &mut CompilerContext) -> Result<(), SpannedError> {

    let source_lines = lex_source_file(source_file, compiler_context)?;

    let ast: AST = ASTParser::generate_ast(source_lines, compiler_context)?;
    
    semantic_analysis(ast, compiler_context)?;

    Ok(())
}

fn compile_program(args: Vec<String>, compiler_context: &mut CompilerContext) -> CompilerResult {
    const MIN_ARG_COUNT: usize = 2;

    if args.len() < MIN_ARG_COUNT {
        return Err(NoInputFiles)
    }

    for source_file_name in args.into_iter().skip(1) {
        let source_file = SourceFile::read(source_file_name.clone())
            .map_err(|err| FileRead { file_name: source_file_name, error: err })?;

        compile_source_file(&source_file, compiler_context)
            .map_err(|spanned_error| CompilerError::Spanned(source_file, spanned_error))?;
    }

    Ok(())
}

fn main()  {
    let args = std::env::args().collect::<Vec<_>>();
    let mut compiler_context = CompilerContext::new();

    // compile_program(args, &mut compiler_context).unwrap();

    if let Err(err) = compile_program(args, &mut compiler_context) {
        println!("{err}");
    }
}
