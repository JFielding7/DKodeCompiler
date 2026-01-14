use crate::compiler_context::CompilerContext;
use crate::error::compiler_error::Error::{FileRead, NoInputFiles, Compiler};
use crate::error::spanned_error::CompilerError;
use crate::source::source_file::SourceFile;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    NoInputFiles,

    FileRead {
        file_name: String,
        #[source]
        error: std::io::Error,
    },

    Compiler(SourceFile, #[source] CompilerError),
}

impl Error {
    pub fn format(self, ctx: CompilerContext) -> String {
        match self {
            Compiler(source_file, err) => err.format(source_file, ctx),
            _ => format!("{self}"),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoInputFiles => write!(f, "Error: No Input Files"),
            FileRead { file_name, error } => {
                write!(f, "Error: {file_name}: {error}")
            }
            _ => write!(f, ""),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
pub type CompilerResult<T> = core::result::Result<T, CompilerError>;
