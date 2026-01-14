use crate::error::compiler_error::CompilerResult;
use crate::lexer::token::TokenType::{Comma, DoubleRightArrow, Greater, Less};
use crate::source::source_span::SourceSpan;
use crate::syntax::parser::token_stream::TokenStream;
use crate::types::type_annotation::TypeAnnotation;

fn assert_type_params_closed(token_stream: &mut TokenStream) -> CompilerResult<()> {
    if token_stream.peek_matches(DoubleRightArrow) {
        if token_stream.is_curr_token_split() {
            token_stream.next();
        } else {
            token_stream.split_curr_token();
        }
    } else {
        token_stream.expect_next_token(Greater)?;
    }
    
    Ok(())
}

fn parse_inner_types(token_stream: &mut TokenStream) -> CompilerResult<Vec<TypeAnnotation>> {

    let mut inner_types = vec![parse_type_annotation(token_stream)?];

    while token_stream.peek_matches(Comma) {
        token_stream.next();

        inner_types.push(parse_type_annotation(token_stream)?);
    }

    Ok(inner_types)
}

pub fn parse_type_annotation(token_stream: &mut TokenStream) -> CompilerResult<TypeAnnotation> {

    let type_token = token_stream.expect_next_identifier()?;
    let type_name = type_token.symbol;
    
    let type_token_span = type_token.span;
    let type_span_start = type_token_span.start;
    let type_span_line_index = type_token_span.line_index;

    if token_stream.peek_matches(Less) {
        token_stream.next();
        let inner_types = parse_inner_types(token_stream)?;
        assert_type_params_closed(token_stream)?;
        
        let type_span = SourceSpan::new(type_span_line_index, type_span_start, token_stream.prev_span().end);
        Ok(TypeAnnotation::with_params(type_name, inner_types, type_span))
        
    } else {
        let type_span = SourceSpan::new(type_span_line_index, type_span_start, token_stream.prev_span().end);
        Ok(TypeAnnotation::new(type_name, type_span))
    }
}
