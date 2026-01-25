use crate::ast::typed_variable::TypedVariable;
use crate::error::compiler_error::{CompilerResult, SpannableError};
use crate::lexical_analysis::token::TokenType::Colon;
use crate::syntax_analysis::error::SyntaxError::UnexpectedExpression;
use crate::syntax_analysis::parser::token_stream::TokenStream;
use crate::syntax_analysis::parser::type_annotation::parse_type_annotation;
use crate::phase::types::type_annotation::TypeAnnotation;

pub fn parse_class_name(mut token_stream: TokenStream) -> CompilerResult<TypeAnnotation> {
    let class_type_annotation = parse_type_annotation(&mut token_stream)?;

    if let Some(token) = token_stream.next() {
        Err(UnexpectedExpression.at(token.span))
    } else {
        Ok(class_type_annotation)
    }
}

pub fn parse_field(mut token_stream: TokenStream) -> CompilerResult<TypedVariable> {
    let field_name = token_stream.expect_next_identifier()?;
    let field_symbol = field_name.symbol;
    let field_span = field_name.span;
    
    token_stream.expect_next_token(Colon)?;
    
    let type_annotation = parse_type_annotation(&mut token_stream)?;
    
    Ok(TypedVariable::new(field_symbol, type_annotation, field_span))
}
