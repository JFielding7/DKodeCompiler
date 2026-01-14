use crate::source::source_file::SourceFile;
use crate::source::source_span::SourceSpan;
use thiserror::Error;
use crate::compiler_context::CompilerContext;

#[derive(Debug, Error)]
#[error("{error_type}")]
pub struct CompilerError {
    pub error_type: Box<dyn SpannableError>,
    pub span: SourceSpan,
}

impl CompilerError {
    fn new(error_type: Box<dyn SpannableError>, span: SourceSpan) -> Self {
        Self {
            error_type, 
            span
        }
    }

    pub fn format(&self, source_file: SourceFile, ctx: CompilerContext) -> String {
        format!("{}\n{}", self.error_type.format(ctx), self.span.format_source_span(source_file))
    }
}

pub trait SpannableError: std::error::Error where Self: 'static {
    fn at(self, span: SourceSpan) -> CompilerError
    where Self: Sized {
        CompilerError::new(Box::new(self), span)
    }

    fn format(&self, ctx: CompilerContext) -> String;
}
