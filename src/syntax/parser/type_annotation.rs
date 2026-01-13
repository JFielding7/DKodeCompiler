use crate::lexer::token::TokenType::{Comma, DoubleRightArrow, Greater, Less};
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::token_stream::TokenStream;
use crate::types::type_annotation::TypeAnnotation;

fn assert_type_params_closed(token_stream: &mut TokenStream) -> SyntaxResult<()> {
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

fn parse_inner_types(token_stream: &mut TokenStream) -> SyntaxResult<Vec<TypeAnnotation>> {

    let mut inner_types = vec![parse_type_annotation(token_stream)?];

    while token_stream.peek_matches(Comma) {
        token_stream.next();

        inner_types.push(parse_type_annotation(token_stream)?);
    }

    Ok(inner_types)
}

pub fn parse_type_annotation(token_stream: &mut TokenStream) -> SyntaxResult<TypeAnnotation> {

    let type_name = token_stream.expect_next_identifier()?.symbol;

    if token_stream.peek_matches(Less) {
        token_stream.next();
        let inner_types = parse_inner_types(token_stream)?;
        assert_type_params_closed(token_stream)?;
        Ok(TypeAnnotation::with_params(type_name, inner_types))
        
    } else {
        Ok(TypeAnnotation::new(type_name))
    }
}
