use crate::compiler_context::CompilerContext;
use crate::error::compiler_error::CompilerError;
use crate::source::source_file::SourceFile;

pub mod compiler_error;

#[derive(Debug)]
pub enum Error {
    NoInputFiles,

    FileRead {
        file_name: String,
        error: std::io::Error,
    },

    Compiler {
        source_file: SourceFile,
        error: CompilerError,
    },
}

impl Error {
    pub fn format(self, ctx: &CompilerContext) -> String {
        use Error::*;

        match self {
            NoInputFiles => {
                "Error: No Input Files".to_string()
            },
            FileRead { file_name, error } => {
                format!("Error: {file_name}: {error}")
            },
            Compiler { source_file: file, error } => {
                error.format(file, ctx)
            },
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
