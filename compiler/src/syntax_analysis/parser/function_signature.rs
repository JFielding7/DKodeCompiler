use string_interner::DefaultSymbol;
use crate::ast::function_def_node::Parameter;
use crate::error::compiler_error::CompilerResult;
use crate::error::compiler_error::SpannableError;
use crate::lexical_analysis::token::TokenType::{CloseParen, Colon, Comma, Identifier, OpenParen};
use crate::syntax_analysis::error::SyntaxError::UnexpectedExpression;
use crate::syntax_analysis::parser::token_stream::TokenStream;
use crate::syntax_analysis::parser::type_annotation::parse_type_annotation;
use crate::types::type_annotation::TypeAnnotation;

pub fn parse_function_name(token_stream: &mut TokenStream) -> CompilerResult<DefaultSymbol> {
    Ok(token_stream.expect_next_identifier()?.symbol)
}

fn parse_parameter(token_stream: &mut TokenStream) -> CompilerResult<Parameter> {
    let param_token = token_stream.expect_next_token(Identifier)?;
    let param_name = param_token.symbol;
    let param_span = param_token.span;

    token_stream.expect_next_token(Colon)?;
    let type_annotation = parse_type_annotation(token_stream)?;

    Ok(Parameter::new(param_name, type_annotation, param_span))
}

pub fn parse_parameters(token_stream: &mut TokenStream) -> CompilerResult<Vec<Parameter>> {
    token_stream.expect_next_token(OpenParen)?;

    let mut params = Vec::new();

    if token_stream.peek_matches(CloseParen) {
        token_stream.next();
        return Ok(params);
    }

    params.push(parse_parameter(token_stream)?);

    while token_stream.peek_matches(Comma) {
        token_stream.next();
        params.push(parse_parameter(token_stream)?);
    }

    token_stream.expect_next_token(CloseParen)?;

    Ok(params)
}

fn parse_return_type_annotation(token_stream: &mut TokenStream) -> CompilerResult<TypeAnnotation> {
    token_stream.next();
    let type_annotation = parse_type_annotation(token_stream)?;

    match token_stream.next() {
        None => Ok(type_annotation),
        Some(token) => Err(UnexpectedExpression.at(token.span))
    }
}

pub fn parse_return_type(token_stream: &mut TokenStream) -> CompilerResult<Option<TypeAnnotation>> {
    match token_stream.peek() {
        Some(&token) => {
            if *token == Colon {
                Ok(Some(parse_return_type_annotation(token_stream)?))
            } else {
                Err(UnexpectedExpression.at(token.span))
            }
        },
        None => Ok(None),
    }
}
